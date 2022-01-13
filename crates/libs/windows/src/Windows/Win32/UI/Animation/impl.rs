pub trait IUIAnimationInterpolatorImpl: Sized {
    fn SetInitialValueAndVelocity(&mut self, initialvalue: f64, initialvelocity: f64) -> ::windows::core::Result<()>;
    fn SetDuration(&mut self, duration: f64) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalValue(&mut self) -> ::windows::core::Result<f64>;
    fn InterpolateValue(&mut self, offset: f64) -> ::windows::core::Result<f64>;
    fn InterpolateVelocity(&mut self, offset: f64) -> ::windows::core::Result<f64>;
    fn GetDependencies(&mut self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()>;
}
impl IUIAnimationInterpolatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationInterpolatorVtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValueAndVelocity(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity)).into()
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterpolateValue(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterpolateVelocity(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into()
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
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialValueAndVelocity(&mut self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetDuration(&mut self, duration: f64) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalValue(&mut self, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn InterpolateValue(&mut self, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn InterpolateVelocity(&mut self, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetPrimitiveInterpolation(&mut self, interpolation: ::core::option::Option<IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetDependencies(&mut self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()>;
}
impl IUIAnimationInterpolator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationInterpolator2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValueAndVelocity(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFinalValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InterpolateValue(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InterpolateVelocity(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolation: ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrimitiveInterpolation(::core::mem::transmute(&interpolation), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into()
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
    fn OnLoopIterationChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationLoopIterationChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationLoopIterationChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Impl: IUIAnimationLoopIterationChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLoopIterationChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&newiterationcount), ::core::mem::transmute_copy(&olditerationcount)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerImpl: Sized {
    fn CreateAnimationVariable(&mut self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(&mut self, variable: ::core::option::Option<IUIAnimationVariable>, transition: ::core::option::Option<IUIAnimationTransition>, timenow: f64) -> ::windows::core::Result<()>;
    fn CreateStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn AbandonAllStoryboards(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn GetVariableFromTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&mut self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn SetManagerEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationManagerEventHandler>) -> ::windows::core::Result<()>;
    fn SetCancelPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetTrimPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetCompressPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetConcludePriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAnimationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerVtbl {
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVariable(::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScheduleTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAllStoryboards(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbandonAllStoryboards().into()
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *updateresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryboardFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManagerEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancelPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompressPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConcludePriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
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
    fn CreateAnimationVectorVariable(&mut self, initialvalue: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(&mut self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>, transition: ::core::option::Option<IUIAnimationTransition2>, timenow: f64) -> ::windows::core::Result<()>;
    fn CreateStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn AbandonAllStoryboards(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn GetVariableFromTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(&mut self) -> ::windows::core::Result<f64>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&mut self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn SetManagerEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCancelPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetTrimPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetCompressPriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetConcludePriorityComparison(&mut self, comparison: ::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManager2Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVectorVariable(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVariable(::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScheduleTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAllStoryboards(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbandonAllStoryboards().into()
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *updateresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryboardFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimateNextEventTime() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManagerEventHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancelPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompressPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConcludePriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
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
    fn OnManagerStatusChanged(&mut self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationManagerEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerEventHandlerVtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManagerStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler2Impl: Sized {
    fn OnManagerStatusChanged(&mut self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationManagerEventHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationManagerEventHandler2Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnManagerStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPrimitiveInterpolationImpl: Sized {
    fn AddCubic(&mut self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()>;
    fn AddSinusoidal(&mut self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()>;
}
impl IUIAnimationPrimitiveInterpolationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPrimitiveInterpolationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPrimitiveInterpolationVtbl {
        unsafe extern "system" fn AddCubic<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCubic(::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSinusoidal(::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into()
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
    fn HasPriority(&mut self, scheduledstoryboard: ::core::option::Option<IUIAnimationStoryboard>, newstoryboard: ::core::option::Option<IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>;
}
impl IUIAnimationPriorityComparisonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparisonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPriorityComparisonVtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparisonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasPriority(::core::mem::transmute(&scheduledstoryboard), ::core::mem::transmute(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HasPriority: HasPriority::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison2Impl: Sized {
    fn HasPriority(&mut self, scheduledstoryboard: ::core::option::Option<IUIAnimationStoryboard2>, newstoryboard: ::core::option::Option<IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>;
}
impl IUIAnimationPriorityComparison2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationPriorityComparison2Vtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparison2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasPriority(::core::mem::transmute(&scheduledstoryboard), ::core::mem::transmute(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HasPriority: HasPriority::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardImpl: Sized {
    fn AddTransition(&mut self, variable: ::core::option::Option<IUIAnimationVariable>, transition: ::core::option::Option<IUIAnimationTransition>) -> ::windows::core::Result<()>;
    fn AddKeyframeAtOffset(&mut self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&mut self, transition: ::core::option::Option<IUIAnimationTransition>) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&mut self, variable: ::core::option::Option<IUIAnimationVariable>, transition: ::core::option::Option<IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn AddTransitionBetweenKeyframes(&mut self, variable: ::core::option::Option<IUIAnimationVariable>, transition: ::core::option::Option<IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn RepeatBetweenKeyframes(&mut self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::Result<()>;
    fn HoldVariable(&mut self, variable: ::core::option::Option<IUIAnimationVariable>) -> ::windows::core::Result<()>;
    fn SetLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Schedule(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT>;
    fn Conclude(&mut self) -> ::windows::core::Result<()>;
    fn Finish(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStoryboardEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationStoryboardEventHandler>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardVtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAtOffset(::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAfterTransition(::core::mem::transmute(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransitionAtKeyframe(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransitionBetweenKeyframes(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RepeatBetweenKeyframes(::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&repetitioncount)).into()
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HoldVariable(::core::mem::transmute(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Schedule(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *schedulingresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Conclude().into()
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abandon().into()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *elapsedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboardEventHandler(::core::mem::transmute(&handler)).into()
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
    fn AddTransition(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>, transition: ::core::option::Option<IUIAnimationTransition2>) -> ::windows::core::Result<()>;
    fn AddKeyframeAtOffset(&mut self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&mut self, transition: ::core::option::Option<IUIAnimationTransition2>) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>, transition: ::core::option::Option<IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn AddTransitionBetweenKeyframes(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>, transition: ::core::option::Option<IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn RepeatBetweenKeyframes(&mut self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::core::option::Option<IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HoldVariable(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>) -> ::windows::core::Result<()>;
    fn SetLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn SetSkipDuration(&mut self, secondsduration: f64) -> ::windows::core::Result<()>;
    fn Schedule(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT>;
    fn Conclude(&mut self) -> ::windows::core::Result<()>;
    fn Finish(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStoryboardEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationStoryboard2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboard2Vtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAtOffset(::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAfterTransition(::core::mem::transmute(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransitionAtKeyframe(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTransitionBetweenKeyframes(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::core::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RepeatBetweenKeyframes(::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&crepetition), ::core::mem::transmute_copy(&repeatmode), ::core::mem::transmute(&piterationchangehandler), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HoldVariable(::core::mem::transmute(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn SetSkipDuration<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSkipDuration(::core::mem::transmute_copy(&secondsduration)).into()
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Schedule(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *schedulingresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Conclude().into()
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abandon().into()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *elapsedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboard2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboardEventHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), ::core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into()
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
    fn OnStoryboardStatusChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>;
    fn OnStoryboardUpdated(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboardEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardEventHandlerVtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStoryboardStatusChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStoryboardUpdated(::core::mem::transmute(&storyboard)).into()
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
    fn OnStoryboardStatusChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>;
    fn OnStoryboardUpdated(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard2>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboardEventHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStoryboardStatusChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStoryboardUpdated(::core::mem::transmute(&storyboard)).into()
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
    fn SetTimerUpdateHandler(&mut self, updatehandler: ::core::option::Option<IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::Result<()>;
    fn SetTimerEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationTimerEventHandler>) -> ::windows::core::Result<()>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<()>;
    fn GetTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetFrameRateThreshold(&mut self, framespersecond: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerVtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatehandler: ::windows::core::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimerUpdateHandler(::core::mem::transmute(&updatehandler), ::core::mem::transmute_copy(&idlebehavior)).into()
        }
        unsafe extern "system" fn SetTimerEventHandler<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimerEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Enable<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Disable<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEnabled().into()
        }
        unsafe extern "system" fn GetTime<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Impl: IUIAnimationTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameRateThreshold(::core::mem::transmute_copy(&framespersecond)).into()
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
    fn OnTimerClientStatusChanged(&mut self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerClientEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerClientEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerClientEventHandlerVtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Impl: IUIAnimationTimerClientEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTimerClientStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerEventHandlerImpl: Sized {
    fn OnPreUpdate(&mut self) -> ::windows::core::Result<()>;
    fn OnPostUpdate(&mut self) -> ::windows::core::Result<()>;
    fn OnRenderingTooSlow(&mut self, framespersecond: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerEventHandlerVtbl {
        unsafe extern "system" fn OnPreUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPreUpdate().into()
        }
        unsafe extern "system" fn OnPostUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPostUpdate().into()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRenderingTooSlow(::core::mem::transmute_copy(&framespersecond)).into()
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
    fn OnUpdate(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(&mut self, handler: ::core::option::Option<IUIAnimationTimerClientEventHandler>) -> ::windows::core::Result<()>;
    fn ClearTimerClientEventHandler(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerUpdateHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTimerUpdateHandlerVtbl {
        unsafe extern "system" fn OnUpdate<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdate(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimerClientEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearTimerClientEventHandler().into()
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
    fn SetInitialValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SetInitialVelocity(&mut self, velocity: f64) -> ::windows::core::Result<()>;
    fn IsDurationKnown(&mut self) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
}
impl IUIAnimationTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionVtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDurationKnown().into()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SetInitialVectorValue(&mut self, value: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetInitialVelocity(&mut self, velocity: f64) -> ::windows::core::Result<()>;
    fn SetInitialVectorVelocity(&mut self, velocity: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn IsDurationKnown(&mut self) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
}
impl IUIAnimationTransition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransition2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVectorValue<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVectorVelocity(::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDurationKnown().into()
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn CreateTransition(&mut self, interpolator: ::core::option::Option<IUIAnimationInterpolator>) -> ::windows::core::Result<IUIAnimationTransition>;
}
impl IUIAnimationTransitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionFactoryVtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransition(::core::mem::transmute(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTransition: CreateTransition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory2Impl: Sized {
    fn CreateTransition(&mut self, interpolator: ::core::option::Option<IUIAnimationInterpolator2>) -> ::windows::core::Result<IUIAnimationTransition2>;
}
impl IUIAnimationTransitionFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionFactory2Vtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransition(::core::mem::transmute(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTransition: CreateTransition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibraryImpl: Sized {
    fn CreateInstantaneousTransition(&mut self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateConstantTransition(&mut self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateDiscreteTransition(&mut self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateLinearTransition(&mut self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateLinearTransitionFromSpeed(&mut self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromVelocity(&mut self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromRange(&mut self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateAccelerateDecelerateTransition(&mut self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateReversalTransition(&mut self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateCubicTransition(&mut self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateSmoothStopTransition(&mut self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition>;
    fn CreateParabolicTransitionFromAcceleration(&mut self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition>;
}
impl IUIAnimationTransitionLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionLibraryVtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousTransition(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConstantTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReversalTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSmoothStopTransition(::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn CreateInstantaneousTransition(&mut self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateInstantaneousVectorTransition(&mut self, finalvalue: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateConstantTransition(&mut self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteTransition(&mut self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteVectorTransition(&mut self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransition(&mut self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransition(&mut self, duration: f64, finalvalue: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransitionFromSpeed(&mut self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransitionFromSpeed(&mut self, speed: f64, finalvalue: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromVelocity(&mut self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromRange(&mut self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateAccelerateDecelerateTransition(&mut self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateReversalTransition(&mut self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateCubicTransition(&mut self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateCubicVectorTransition(&mut self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateSmoothStopTransition(&mut self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateParabolicTransitionFromAcceleration(&mut self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearTransition(&mut self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearVectorTransition(&mut self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2>;
}
impl IUIAnimationTransitionLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationTransitionLibrary2Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousTransition(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousVectorTransition(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConstantTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteVectorTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearVectorTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReversalTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSmoothStopTransition(::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierLinearVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn GetValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetPreviousValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetFinalIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetPreviousIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetCurrentStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard>;
    fn SetLowerBound(&mut self, bound: f64) -> ::windows::core::Result<()>;
    fn SetUpperBound(&mut self, bound: f64) -> ::windows::core::Result<()>;
    fn SetRoundingMode(&mut self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetVariableChangeHandler(&mut self, handler: ::core::option::Option<IUIAnimationVariableChangeHandler>) -> ::windows::core::Result<()>;
    fn SetVariableIntegerChangeHandler(&mut self, handler: ::core::option::Option<IUIAnimationVariableIntegerChangeHandler>) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableVtbl {
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowerBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpperBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoundingMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariableChangeHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariableIntegerChangeHandler(::core::mem::transmute(&handler)).into()
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
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn GetValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetVectorValue(&mut self, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetCurve(&mut self, animation: ::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn GetVectorCurve(&mut self, animation: *const ::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetFinalValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalVectorValue(&mut self, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetPreviousValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetPreviousVectorValue(&mut self, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetIntegerVectorValue(&mut self, value: *mut i32, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetFinalIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetFinalIntegerVectorValue(&mut self, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetPreviousIntegerValue(&mut self) -> ::windows::core::Result<i32>;
    fn GetPreviousIntegerVectorValue(&mut self, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetCurrentStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard2>;
    fn SetLowerBound(&mut self, bound: f64) -> ::windows::core::Result<()>;
    fn SetLowerBoundVector(&mut self, bound: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetUpperBound(&mut self, bound: f64) -> ::windows::core::Result<()>;
    fn SetUpperBoundVector(&mut self, bound: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetRoundingMode(&mut self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetVariableChangeHandler(&mut self, handler: ::core::option::Option<IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetVariableIntegerChangeHandler(&mut self, handler: ::core::option::Option<IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetVariableCurveChangeHandler(&mut self, handler: ::core::option::Option<IUIAnimationVariableCurveChangeHandler2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
impl IUIAnimationVariable2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariable2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurve(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn GetVectorCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *const ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVectorCurve(::core::mem::transmute_copy(&animation), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFinalVectorValue(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreviousVectorValue(::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIntegerVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFinalIntegerVectorValue(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreviousIntegerVectorValue(::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowerBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetLowerBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowerBoundVector(::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpperBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpperBoundVector(::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoundingMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariableChangeHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariableIntegerChangeHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVariableCurveChangeHandler(::core::mem::transmute(&handler)).into()
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
    fn OnValueChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard>, variable: ::core::option::Option<IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableChangeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableChangeHandlerVtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnValueChanged: OnValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler2Impl: Sized {
    fn OnValueChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard2>, variable: ::core::option::Option<IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableChangeHandler2Vtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnValueChanged: OnValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2Impl: Sized {
    fn OnCurveChanged(&mut self, variable: ::core::option::Option<IUIAnimationVariable2>) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableCurveChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableCurveChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2Vtbl {
        unsafe extern "system" fn OnCurveChanged<Impl: IUIAnimationVariableCurveChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCurveChanged(::core::mem::transmute(&variable)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCurveChanged: OnCurveChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandlerImpl: Sized {
    fn OnIntegerValueChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard>, variable: ::core::option::Option<IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableIntegerChangeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandlerVtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIntegerValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2Impl: Sized {
    fn OnIntegerValueChanged(&mut self, storyboard: ::core::option::Option<IUIAnimationStoryboard2>, variable: ::core::option::Option<IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableIntegerChangeHandler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIntegerValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as ::windows::core::Interface>::IID
    }
}
