pub trait IUIAnimationInterpolator_Impl: Sized {
    fn SetInitialValueAndVelocity(&mut self, initialvalue: f64, initialvelocity: f64) -> ::windows::core::Result<()>;
    fn SetDuration(&mut self, duration: f64) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalValue(&mut self) -> ::windows::core::Result<f64>;
    fn InterpolateValue(&mut self, offset: f64) -> ::windows::core::Result<f64>;
    fn InterpolateVelocity(&mut self, offset: f64) -> ::windows::core::Result<f64>;
    fn GetDependencies(&mut self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()>;
}
impl IUIAnimationInterpolator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator_Vtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialValueAndVelocity(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterpolateValue(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterpolateVelocity(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationInterpolator2_Impl: Sized {
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialValueAndVelocity(&mut self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetDuration(&mut self, duration: f64) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
    fn GetFinalValue(&mut self, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn InterpolateValue(&mut self, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn InterpolateVelocity(&mut self, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetPrimitiveInterpolation(&mut self, interpolation: &::core::option::Option<IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetDependencies(&mut self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()>;
}
impl IUIAnimationInterpolator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialValueAndVelocity(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&initialvelocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFinalValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InterpolateValue(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InterpolateVelocity(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolation: ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrimitiveInterpolation(::core::mem::transmute(&interpolation), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationLoopIterationChangeHandler2_Impl: Sized {
    fn OnLoopIterationChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2_Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLoopIterationChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&newiterationcount), ::core::mem::transmute_copy(&olditerationcount)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManager_Impl: Sized {
    fn CreateAnimationVariable(&mut self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(&mut self, variable: &::core::option::Option<IUIAnimationVariable>, transition: &::core::option::Option<IUIAnimationTransition>, timenow: f64) -> ::windows::core::Result<()>;
    fn CreateStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn AbandonAllStoryboards(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn GetVariableFromTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&mut self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn SetManagerEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationManagerEventHandler>) -> ::windows::core::Result<()>;
    fn SetCancelPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetTrimPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetCompressPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetConcludePriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison>) -> ::windows::core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAnimationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>() -> IUIAnimationManager_Vtbl {
        unsafe extern "system" fn CreateAnimationVariable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAnimationVariable(::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScheduleTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAllStoryboards(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbandonAllStoryboards().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Update(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *updateresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVariableFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryboardFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAnimationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManagerEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCancelPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrimPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompressPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConcludePriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAnimationManager2_Impl: Sized {
    fn CreateAnimationVectorVariable(&mut self, initialvalue: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(&mut self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>, transition: &::core::option::Option<IUIAnimationTransition2>, timenow: f64) -> ::windows::core::Result<()>;
    fn CreateStoryboard(&mut self) -> ::windows::core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn AbandonAllStoryboards(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn GetVariableFromTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(&mut self) -> ::windows::core::Result<f64>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&mut self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn SetManagerEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCancelPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetTrimPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetCompressPriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetConcludePriorityComparison(&mut self, comparison: &::core::option::Option<IUIAnimationPriorityComparison2>) -> ::windows::core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>() -> IUIAnimationManager2_Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAnimationVectorVariable(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAnimationVariable(::core::mem::transmute_copy(&initialvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScheduleTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAllStoryboards(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbandonAllStoryboards().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Update(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *updateresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVariableFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *variable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryboardFromTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EstimateNextEventTime() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAnimationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManagerEventHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCancelPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrimPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompressPriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConcludePriorityComparison(::core::mem::transmute(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Identity, Impl, OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler_Impl: Sized {
    fn OnManagerStatusChanged(&mut self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationManagerEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManagerStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler2_Impl: Sized {
    fn OnManagerStatusChanged(&mut self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationManagerEventHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler2_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnManagerStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPrimitiveInterpolation_Impl: Sized {
    fn AddCubic(&mut self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()>;
    fn AddSinusoidal(&mut self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()>;
}
impl IUIAnimationPrimitiveInterpolation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>() -> IUIAnimationPrimitiveInterpolation_Vtbl {
        unsafe extern "system" fn AddCubic<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddCubic(::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSinusoidal(::core::mem::transmute_copy(&dimension), ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddCubic: AddCubic::<Identity, Impl, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPrimitiveInterpolation as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison_Impl: Sized {
    fn HasPriority(&mut self, scheduledstoryboard: &::core::option::Option<IUIAnimationStoryboard>, newstoryboard: &::core::option::Option<IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>;
}
impl IUIAnimationPriorityComparison_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasPriority(::core::mem::transmute(&scheduledstoryboard), ::core::mem::transmute(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison2_Impl: Sized {
    fn HasPriority(&mut self, scheduledstoryboard: &::core::option::Option<IUIAnimationStoryboard2>, newstoryboard: &::core::option::Option<IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>;
}
impl IUIAnimationPriorityComparison2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison2_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasPriority(::core::mem::transmute(&scheduledstoryboard), ::core::mem::transmute(&newstoryboard), ::core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboard_Impl: Sized {
    fn AddTransition(&mut self, variable: &::core::option::Option<IUIAnimationVariable>, transition: &::core::option::Option<IUIAnimationTransition>) -> ::windows::core::Result<()>;
    fn AddKeyframeAtOffset(&mut self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&mut self, transition: &::core::option::Option<IUIAnimationTransition>) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&mut self, variable: &::core::option::Option<IUIAnimationVariable>, transition: &::core::option::Option<IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn AddTransitionBetweenKeyframes(&mut self, variable: &::core::option::Option<IUIAnimationVariable>, transition: &::core::option::Option<IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn RepeatBetweenKeyframes(&mut self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::Result<()>;
    fn HoldVariable(&mut self, variable: &::core::option::Option<IUIAnimationVariable>) -> ::windows::core::Result<()>;
    fn SetLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn Schedule(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT>;
    fn Conclude(&mut self) -> ::windows::core::Result<()>;
    fn Finish(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStoryboardEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationStoryboardEventHandler>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddKeyframeAtOffset(::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddKeyframeAfterTransition(::core::mem::transmute(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransitionAtKeyframe(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransitionBetweenKeyframes(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RepeatBetweenKeyframes(::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&repetitioncount)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HoldVariable(::core::mem::transmute(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Schedule<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Schedule(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *schedulingresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Conclude().into()
        }
        unsafe extern "system" fn Finish<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abandon().into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *elapsedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStoryboardEventHandler(::core::mem::transmute(&handler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAnimationStoryboard2_Impl: Sized {
    fn AddTransition(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>, transition: &::core::option::Option<IUIAnimationTransition2>) -> ::windows::core::Result<()>;
    fn AddKeyframeAtOffset(&mut self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&mut self, transition: &::core::option::Option<IUIAnimationTransition2>) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>, transition: &::core::option::Option<IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn AddTransitionBetweenKeyframes(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>, transition: &::core::option::Option<IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::Result<()>;
    fn RepeatBetweenKeyframes(&mut self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: &::core::option::Option<IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HoldVariable(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>) -> ::windows::core::Result<()>;
    fn SetLongestAcceptableDelay(&mut self, delay: f64) -> ::windows::core::Result<()>;
    fn SetSkipDuration(&mut self, secondsduration: f64) -> ::windows::core::Result<()>;
    fn Schedule(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT>;
    fn Conclude(&mut self) -> ::windows::core::Result<()>;
    fn Finish(&mut self, completiondeadline: f64) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStoryboardEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAnimationStoryboard2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard2_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransition(::core::mem::transmute(&variable), ::core::mem::transmute(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddKeyframeAtOffset(::core::mem::transmute_copy(&existingkeyframe), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddKeyframeAfterTransition(::core::mem::transmute(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *keyframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransitionAtKeyframe(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTransitionBetweenKeyframes(::core::mem::transmute(&variable), ::core::mem::transmute(&transition), ::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::core::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RepeatBetweenKeyframes(::core::mem::transmute_copy(&startkeyframe), ::core::mem::transmute_copy(&endkeyframe), ::core::mem::transmute_copy(&crepetition), ::core::mem::transmute_copy(&repeatmode), ::core::mem::transmute(&piterationchangehandler), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HoldVariable(::core::mem::transmute(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLongestAcceptableDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn SetSkipDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSkipDuration(::core::mem::transmute_copy(&secondsduration)).into()
        }
        unsafe extern "system" fn Schedule<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Schedule(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *schedulingresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Conclude().into()
        }
        unsafe extern "system" fn Finish<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abandon().into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *elapsedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStoryboardEventHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), ::core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            SetSkipDuration: SetSkipDuration::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler_Impl: Sized {
    fn OnStoryboardStatusChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>;
    fn OnStoryboardUpdated(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboardEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStoryboardStatusChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStoryboardUpdated(::core::mem::transmute(&storyboard)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler2_Impl: Sized {
    fn OnStoryboardStatusChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>;
    fn OnStoryboardUpdated(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard2>) -> ::windows::core::Result<()>;
}
impl IUIAnimationStoryboardEventHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStoryboardStatusChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStoryboardUpdated(::core::mem::transmute(&storyboard)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimer_Impl: Sized {
    fn SetTimerUpdateHandler(&mut self, updatehandler: &::core::option::Option<IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::Result<()>;
    fn SetTimerEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationTimerEventHandler>) -> ::windows::core::Result<()>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<()>;
    fn GetTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetFrameRateThreshold(&mut self, framespersecond: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>() -> IUIAnimationTimer_Vtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatehandler: ::windows::core::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimerUpdateHandler(::core::mem::transmute(&updatehandler), ::core::mem::transmute_copy(&idlebehavior)).into()
        }
        unsafe extern "system" fn SetTimerEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimerEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEnabled().into()
        }
        unsafe extern "system" fn GetTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrameRateThreshold(::core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Identity, Impl, OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimer as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerClientEventHandler_Impl: Sized {
    fn OnTimerClientStatusChanged(&mut self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerClientEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerClientEventHandler_Vtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTimerClientStatusChanged(::core::mem::transmute_copy(&newstatus), ::core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerEventHandler_Impl: Sized {
    fn OnPreUpdate(&mut self) -> ::windows::core::Result<()>;
    fn OnPostUpdate(&mut self) -> ::windows::core::Result<()>;
    fn OnRenderingTooSlow(&mut self, framespersecond: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerEventHandler_Vtbl {
        unsafe extern "system" fn OnPreUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPreUpdate().into()
        }
        unsafe extern "system" fn OnPostUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPostUpdate().into()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRenderingTooSlow(::core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnPreUpdate: OnPreUpdate::<Identity, Impl, OFFSET>,
            OnPostUpdate: OnPostUpdate::<Identity, Impl, OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTimerUpdateHandler_Impl: Sized {
    fn OnUpdate(&mut self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(&mut self, handler: &::core::option::Option<IUIAnimationTimerClientEventHandler>) -> ::windows::core::Result<()>;
    fn ClearTimerClientEventHandler(&mut self) -> ::windows::core::Result<()>;
}
impl IUIAnimationTimerUpdateHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerUpdateHandler_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnUpdate(::core::mem::transmute_copy(&timenow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimerClientEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearTimerClientEventHandler().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Identity, Impl, OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTimerUpdateHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransition_Impl: Sized {
    fn SetInitialValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SetInitialVelocity(&mut self, velocity: f64) -> ::windows::core::Result<()>;
    fn IsDurationKnown(&mut self) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
}
impl IUIAnimationTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>() -> IUIAnimationTransition_Vtbl {
        unsafe extern "system" fn SetInitialValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDurationKnown().into()
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransition2_Impl: Sized {
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SetInitialVectorValue(&mut self, value: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn SetInitialVelocity(&mut self, velocity: f64) -> ::windows::core::Result<()>;
    fn SetInitialVectorVelocity(&mut self, velocity: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn IsDurationKnown(&mut self) -> ::windows::core::Result<()>;
    fn GetDuration(&mut self) -> ::windows::core::Result<f64>;
}
impl IUIAnimationTransition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>() -> IUIAnimationTransition2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVectorVelocity(::core::mem::transmute_copy(&velocity), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDurationKnown().into()
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *duration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransition2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory_Impl: Sized {
    fn CreateTransition(&mut self, interpolator: &::core::option::Option<IUIAnimationInterpolator>) -> ::windows::core::Result<IUIAnimationTransition>;
}
impl IUIAnimationTransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransition(::core::mem::transmute(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory2_Impl: Sized {
    fn CreateTransition(&mut self, interpolator: &::core::option::Option<IUIAnimationInterpolator2>) -> ::windows::core::Result<IUIAnimationTransition2>;
}
impl IUIAnimationTransitionFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory2_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransition(::core::mem::transmute(&interpolator)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary_Impl: Sized {
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
impl IUIAnimationTransitionLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstantaneousTransition(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateConstantTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDiscreteTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateReversalTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCubicTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSmoothStopTransition(::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary2_Impl: Sized {
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
impl IUIAnimationTransitionLibrary2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary2_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstantaneousTransition(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstantaneousVectorTransition(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateConstantTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDiscreteTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDiscreteVectorTransition(::core::mem::transmute_copy(&delay), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&hold)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearVectorTransitionFromSpeed(::core::mem::transmute_copy(&speed), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&minimumvalue), ::core::mem::transmute_copy(&maximumvalue), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&slope)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&accelerationratio), ::core::mem::transmute_copy(&decelerationratio)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateReversalTransition(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCubicTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCubicVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&cdimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSmoothStopTransition(::core::mem::transmute_copy(&maximumduration), ::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&finalvelocity), ::core::mem::transmute_copy(&acceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCubicBezierLinearTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCubicBezierLinearVectorTransition(::core::mem::transmute_copy(&duration), ::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension), ::core::mem::transmute_copy(&x1), ::core::mem::transmute_copy(&y1), ::core::mem::transmute_copy(&x2), ::core::mem::transmute_copy(&y2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariable_Impl: Sized {
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
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetVariableChangeHandler(&mut self, handler: &::core::option::Option<IUIAnimationVariableChangeHandler>) -> ::windows::core::Result<()>;
    fn SetVariableIntegerChangeHandler(&mut self, handler: &::core::option::Option<IUIAnimationVariableIntegerChangeHandler>) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>() -> IUIAnimationVariable_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLowerBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpperBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRoundingMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVariableChangeHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVariableIntegerChangeHandler(::core::mem::transmute(&handler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
pub trait IUIAnimationVariable2_Impl: Sized {
    fn GetDimension(&mut self) -> ::windows::core::Result<u32>;
    fn GetValue(&mut self) -> ::windows::core::Result<f64>;
    fn GetVectorValue(&mut self, value: *mut f64, cdimension: u32) -> ::windows::core::Result<()>;
    fn GetCurve(&mut self, animation: &::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> ::windows::core::Result<()>;
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
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetVariableChangeHandler(&mut self, handler: &::core::option::Option<IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetVariableIntegerChangeHandler(&mut self, handler: &::core::option::Option<IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetVariableCurveChangeHandler(&mut self, handler: &::core::option::Option<IUIAnimationVariableCurveChangeHandler2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectComposition"))]
impl IUIAnimationVariable2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>() -> IUIAnimationVariable2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *dimension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurve<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurve(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn GetVectorCurve<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *const ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVectorCurve(::core::mem::transmute_copy(&animation), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFinalVectorValue(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreviousVectorValue(::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIntegerVectorValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *finalvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFinalIntegerVectorValue(::core::mem::transmute_copy(&finalvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousIntegerValue() {
                ::core::result::Result::Ok(ok__) => {
                    *previousvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreviousIntegerVectorValue(::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentStoryboard() {
                ::core::result::Result::Ok(ok__) => {
                    *storyboard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLowerBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetLowerBoundVector<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLowerBoundVector(::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpperBound(::core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBoundVector<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpperBoundVector(::core::mem::transmute_copy(&bound), ::core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRoundingMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVariableChangeHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVariableIntegerChangeHandler(::core::mem::transmute(&handler), ::core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVariableCurveChangeHandler(::core::mem::transmute(&handler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetVectorValue: GetVectorValue::<Identity, Impl, OFFSET>,
            GetCurve: GetCurve::<Identity, Impl, OFFSET>,
            GetVectorCurve: GetVectorCurve::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariable2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler_Impl: Sized {
    fn OnValueChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard>, variable: &::core::option::Option<IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableChangeHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler2_Impl: Sized {
    fn OnValueChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard2>, variable: &::core::option::Option<IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableChangeHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler2_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2_Impl: Sized {
    fn OnCurveChanged(&mut self, variable: &::core::option::Option<IUIAnimationVariable2>) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2_Vtbl {
        unsafe extern "system" fn OnCurveChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCurveChanged(::core::mem::transmute(&variable)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnCurveChanged: OnCurveChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler_Impl: Sized {
    fn OnIntegerValueChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard>, variable: &::core::option::Option<IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnIntegerValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2_Impl: Sized {
    fn OnIntegerValueChanged(&mut self, storyboard: &::core::option::Option<IUIAnimationStoryboard2>, variable: &::core::option::Option<IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::Result<()>;
}
impl IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnIntegerValueChanged(::core::mem::transmute(&storyboard), ::core::mem::transmute(&variable), ::core::mem::transmute_copy(&newvalue), ::core::mem::transmute_copy(&previousvalue), ::core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as ::windows::core::Interface>::IID
    }
}
