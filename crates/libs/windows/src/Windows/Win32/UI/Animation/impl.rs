pub trait IUIAnimationInterpolatorImpl: Sized {
    fn SetInitialValueAndVelocity();
    fn SetDuration();
    fn GetDuration();
    fn GetFinalValue();
    fn InterpolateValue();
    fn InterpolateVelocity();
    fn GetDependencies();
}
impl IUIAnimationInterpolatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationInterpolatorVtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            GetFinalValue: GetFinalValue::<Impl, IMPL_OFFSET>,
            InterpolateValue: InterpolateValue::<Impl, IMPL_OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Impl, IMPL_OFFSET>,
            GetDependencies: GetDependencies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator as ::windows::core::Interface>::IID
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
impl IUIAnimationInterpolator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationInterpolator2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolation: ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDimension: GetDimension::<Impl, IMPL_OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
            GetFinalValue: GetFinalValue::<Impl, IMPL_OFFSET>,
            InterpolateValue: InterpolateValue::<Impl, IMPL_OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Impl, IMPL_OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Impl, IMPL_OFFSET>,
            GetDependencies: GetDependencies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationLoopIterationChangeHandler2Impl: Sized {
    fn OnLoopIterationChanged();
}
impl IUIAnimationLoopIterationChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationLoopIterationChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Impl: IUIAnimationLoopIterationChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as ::windows::core::Interface>::IID
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
impl IUIAnimationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerVtbl {
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateAnimationVariable: CreateAnimationVariable::<Impl, IMPL_OFFSET>,
            ScheduleTransition: ScheduleTransition::<Impl, IMPL_OFFSET>,
            CreateStoryboard: CreateStoryboard::<Impl, IMPL_OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Impl, IMPL_OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Impl, IMPL_OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetAnimationMode: SetAnimationMode::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Impl, IMPL_OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Impl, IMPL_OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Impl, IMPL_OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Impl, IMPL_OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Impl, IMPL_OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManager2Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EstimateNextEventTime<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Impl, IMPL_OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Impl, IMPL_OFFSET>,
            ScheduleTransition: ScheduleTransition::<Impl, IMPL_OFFSET>,
            CreateStoryboard: CreateStoryboard::<Impl, IMPL_OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Impl, IMPL_OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Impl, IMPL_OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Impl, IMPL_OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetAnimationMode: SetAnimationMode::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Impl, IMPL_OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Impl, IMPL_OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Impl, IMPL_OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Impl, IMPL_OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Impl, IMPL_OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandlerImpl: Sized {
    fn OnManagerStatusChanged();
}
impl IUIAnimationManagerEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerEventHandlerVtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler2Impl: Sized {
    fn OnManagerStatusChanged();
}
impl IUIAnimationManagerEventHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerEventHandler2Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPrimitiveInterpolationImpl: Sized {
    fn AddCubic();
    fn AddSinusoidal();
}
impl IUIAnimationPrimitiveInterpolationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPrimitiveInterpolationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPrimitiveInterpolationVtbl {
        unsafe extern "system" fn AddCubic<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddCubic: AddCubic::<Impl, IMPL_OFFSET>,
            AddSinusoidal: AddSinusoidal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPrimitiveInterpolation as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparisonImpl: Sized {
    fn HasPriority();
}
impl IUIAnimationPriorityComparisonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparisonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPriorityComparisonVtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparisonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HasPriority: HasPriority::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison2Impl: Sized {
    fn HasPriority();
}
impl IUIAnimationPriorityComparison2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPriorityComparison2Vtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparison2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HasPriority: HasPriority::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as ::windows::core::Interface>::IID
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
impl IUIAnimationStoryboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardVtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddTransition: AddTransition::<Impl, IMPL_OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Impl, IMPL_OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Impl, IMPL_OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Impl, IMPL_OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Impl, IMPL_OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Impl, IMPL_OFFSET>,
            HoldVariable: HoldVariable::<Impl, IMPL_OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Impl, IMPL_OFFSET>,
            Schedule: Schedule::<Impl, IMPL_OFFSET>,
            Conclude: Conclude::<Impl, IMPL_OFFSET>,
            Finish: Finish::<Impl, IMPL_OFFSET>,
            Abandon: Abandon::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetElapsedTime: GetElapsedTime::<Impl, IMPL_OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationStoryboard2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboard2Vtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::core::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSkipDuration<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddTransition: AddTransition::<Impl, IMPL_OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Impl, IMPL_OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Impl, IMPL_OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Impl, IMPL_OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Impl, IMPL_OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Impl, IMPL_OFFSET>,
            HoldVariable: HoldVariable::<Impl, IMPL_OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Impl, IMPL_OFFSET>,
            SetSkipDuration: SetSkipDuration::<Impl, IMPL_OFFSET>,
            Schedule: Schedule::<Impl, IMPL_OFFSET>,
            Conclude: Conclude::<Impl, IMPL_OFFSET>,
            Finish: Finish::<Impl, IMPL_OFFSET>,
            Abandon: Abandon::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetElapsedTime: GetElapsedTime::<Impl, IMPL_OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandlerImpl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
impl IUIAnimationStoryboardEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardEventHandlerVtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Impl, IMPL_OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler2Impl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
impl IUIAnimationStoryboardEventHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Impl, IMPL_OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler2 as ::windows::core::Interface>::IID
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
impl IUIAnimationTimerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerVtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatehandler: ::windows::core::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimerEventHandler<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disable<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEnabled<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTime<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrameRateThreshold<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Impl, IMPL_OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Disable: Disable::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            GetTime: GetTime::<Impl, IMPL_OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimer as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerClientEventHandlerImpl: Sized {
    fn OnTimerClientStatusChanged();
}
impl IUIAnimationTimerClientEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerClientEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerClientEventHandlerVtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Impl: IUIAnimationTimerClientEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerEventHandlerImpl: Sized {
    fn OnPreUpdate();
    fn OnPostUpdate();
    fn OnRenderingTooSlow();
}
impl IUIAnimationTimerEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerEventHandlerVtbl {
        unsafe extern "system" fn OnPreUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPostUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnPreUpdate: OnPreUpdate::<Impl, IMPL_OFFSET>,
            OnPostUpdate: OnPostUpdate::<Impl, IMPL_OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerUpdateHandlerImpl: Sized {
    fn OnUpdate();
    fn SetTimerClientEventHandler();
    fn ClearTimerClientEventHandler();
}
impl IUIAnimationTimerUpdateHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerUpdateHandlerVtbl {
        unsafe extern "system" fn OnUpdate<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnUpdate: OnUpdate::<Impl, IMPL_OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Impl, IMPL_OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerUpdateHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionImpl: Sized {
    fn SetInitialValue();
    fn SetInitialVelocity();
    fn IsDurationKnown();
    fn GetDuration();
}
impl IUIAnimationTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionVtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Impl, IMPL_OFFSET>,
            IsDurationKnown: IsDurationKnown::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransition as ::windows::core::Interface>::IID
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
impl IUIAnimationTransition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransition2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVectorValue<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDimension: GetDimension::<Impl, IMPL_OFFSET>,
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Impl, IMPL_OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Impl, IMPL_OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Impl, IMPL_OFFSET>,
            IsDurationKnown: IsDurationKnown::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransition2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactoryImpl: Sized {
    fn CreateTransition();
}
impl IUIAnimationTransitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionFactoryVtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTransition: CreateTransition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory2Impl: Sized {
    fn CreateTransition();
}
impl IUIAnimationTransitionFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionFactory2Vtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTransition: CreateTransition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as ::windows::core::Interface>::IID
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
impl IUIAnimationTransitionLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionLibraryVtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Impl, IMPL_OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Impl, IMPL_OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Impl, IMPL_OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Impl, IMPL_OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Impl, IMPL_OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Impl, IMPL_OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Impl, IMPL_OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Impl, IMPL_OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Impl, IMPL_OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Impl, IMPL_OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Impl, IMPL_OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary as ::windows::core::Interface>::IID
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
impl IUIAnimationTransitionLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionLibrary2Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Impl, IMPL_OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Impl, IMPL_OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Impl, IMPL_OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Impl, IMPL_OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Impl, IMPL_OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Impl, IMPL_OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Impl, IMPL_OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Impl, IMPL_OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Impl, IMPL_OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Impl, IMPL_OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Impl, IMPL_OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Impl, IMPL_OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Impl, IMPL_OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Impl, IMPL_OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Impl, IMPL_OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Impl, IMPL_OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Impl, IMPL_OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Impl, IMPL_OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary2 as ::windows::core::Interface>::IID
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
impl IUIAnimationVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableVtbl {
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetFinalValue: GetFinalValue::<Impl, IMPL_OFFSET>,
            GetPreviousValue: GetPreviousValue::<Impl, IMPL_OFFSET>,
            GetIntegerValue: GetIntegerValue::<Impl, IMPL_OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Impl, IMPL_OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Impl, IMPL_OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Impl, IMPL_OFFSET>,
            SetLowerBound: SetLowerBound::<Impl, IMPL_OFFSET>,
            SetUpperBound: SetUpperBound::<Impl, IMPL_OFFSET>,
            SetRoundingMode: SetRoundingMode::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Impl, IMPL_OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
impl IUIAnimationVariable2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariable2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVectorCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *const ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLowerBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpperBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDimension: GetDimension::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetVectorValue: GetVectorValue::<Impl, IMPL_OFFSET>,
            GetCurve: GetCurve::<Impl, IMPL_OFFSET>,
            GetVectorCurve: GetVectorCurve::<Impl, IMPL_OFFSET>,
            GetFinalValue: GetFinalValue::<Impl, IMPL_OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Impl, IMPL_OFFSET>,
            GetPreviousValue: GetPreviousValue::<Impl, IMPL_OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Impl, IMPL_OFFSET>,
            GetIntegerValue: GetIntegerValue::<Impl, IMPL_OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Impl, IMPL_OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Impl, IMPL_OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Impl, IMPL_OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Impl, IMPL_OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Impl, IMPL_OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Impl, IMPL_OFFSET>,
            SetLowerBound: SetLowerBound::<Impl, IMPL_OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Impl, IMPL_OFFSET>,
            SetUpperBound: SetUpperBound::<Impl, IMPL_OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Impl, IMPL_OFFSET>,
            SetRoundingMode: SetRoundingMode::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Impl, IMPL_OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Impl, IMPL_OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariable2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandlerImpl: Sized {
    fn OnValueChanged();
}
impl IUIAnimationVariableChangeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableChangeHandlerVtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnValueChanged: OnValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler2Impl: Sized {
    fn OnValueChanged();
}
impl IUIAnimationVariableChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableChangeHandler2Vtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnValueChanged: OnValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2Impl: Sized {
    fn OnCurveChanged();
}
impl IUIAnimationVariableCurveChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableCurveChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2Vtbl {
        unsafe extern "system" fn OnCurveChanged<Impl: IUIAnimationVariableCurveChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCurveChanged: OnCurveChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandlerImpl: Sized {
    fn OnIntegerValueChanged();
}
impl IUIAnimationVariableIntegerChangeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandlerVtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2Impl: Sized {
    fn OnIntegerValueChanged();
}
impl IUIAnimationVariableIntegerChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as ::windows::core::Interface>::IID
    }
}
