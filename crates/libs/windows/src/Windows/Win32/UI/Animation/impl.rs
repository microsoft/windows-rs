pub trait IUIAnimationInterpolator_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn InterpolateValue(&self, offset: f64) -> windows_core::Result<f64>;
    fn InterpolateVelocity(&self, offset: f64) -> windows_core::Result<f64>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator {}
impl IUIAnimationInterpolator_Vtbl {
    pub const fn new<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator_Vtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    velocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, OFFSET>,
            GetDependencies: GetDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationInterpolator2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateValue(&self, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateVelocity(&self, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPrimitiveInterpolation(&self, interpolation: Option<&IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator2 {}
impl IUIAnimationInterpolator2_Vtbl {
    pub const fn new<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetFinalValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateValue<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolation: *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetPrimitiveInterpolation(this, windows_core::from_raw_borrowed(&interpolation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetDependencies<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Identity, OFFSET>,
            GetDependencies: GetDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationLoopIterationChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnLoopIterationChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationLoopIterationChangeHandler2 {}
impl IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2_Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Identity: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationLoopIterationChangeHandler2_Impl::OnLoopIterationChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&id), core::mem::transmute_copy(&newiterationcount), core::mem::transmute_copy(&olditerationcount)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManager_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler>) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager {}
impl IUIAnimationManager_Vtbl {
    pub const fn new<Identity: IUIAnimationManager_Impl, const OFFSET: isize>() -> IUIAnimationManager_Vtbl {
        unsafe extern "system" fn CreateAnimationVariable<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVariable: CreateAnimationVariable::<Identity, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManager2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateAnimationVectorVariable(&self, initialvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(&self) -> windows_core::Result<f64>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager2 {}
impl IUIAnimationManager2_Vtbl {
    pub const fn new<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>() -> IUIAnimationManager2_Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateAnimationVectorVariable(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::EstimateNextEventTime(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Identity, OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler {}
impl IUIAnimationManagerEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManagerEventHandler_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler2 {}
impl IUIAnimationManagerEventHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler2_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManagerEventHandler2_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPrimitiveInterpolation_Impl: Sized + windows_core::IUnknownImpl {
    fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPrimitiveInterpolation {}
impl IUIAnimationPrimitiveInterpolation_Vtbl {
    pub const fn new<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>() -> IUIAnimationPrimitiveInterpolation_Vtbl {
        unsafe extern "system" fn AddCubic<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPrimitiveInterpolation_Impl::AddCubic(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPrimitiveInterpolation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddCubic: AddCubic::<Identity, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPrimitiveInterpolation as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison_Impl: Sized + windows_core::IUnknownImpl {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard>, newstoryboard: Option<&IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison {}
impl IUIAnimationPriorityComparison_Vtbl {
    pub const fn new<Identity: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPriorityComparison_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison2_Impl: Sized + windows_core::IUnknownImpl {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard2>, newstoryboard: Option<&IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison2 {}
impl IUIAnimationPriorityComparison2_Vtbl {
    pub const fn new<Identity: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison2_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPriorityComparison2_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboard_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard {}
impl IUIAnimationStoryboard_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&repetitioncount)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Schedule<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    elapsedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, OFFSET>,
            HoldVariable: HoldVariable::<Identity, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            Conclude: Conclude::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            Abandon: Abandon::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboard2_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: Option<&IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn SetSkipDuration(&self, secondsduration: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard2 {}
impl IUIAnimationStoryboard2_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard2_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&crepetition), core::mem::transmute_copy(&repeatmode), windows_core::from_raw_borrowed(&piterationchangehandler), core::mem::transmute_copy(&id), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn SetSkipDuration<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, secondsduration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetSkipDuration(this, core::mem::transmute_copy(&secondsduration)).into()
        }
        unsafe extern "system" fn Schedule<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    elapsedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, OFFSET>,
            HoldVariable: HoldVariable::<Identity, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, OFFSET>,
            SetSkipDuration: SetSkipDuration::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            Conclude: Conclude::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            Abandon: Abandon::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler {}
impl IUIAnimationStoryboardEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler2 {}
impl IUIAnimationStoryboardEventHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimer_Impl: Sized + windows_core::IUnknownImpl {
    fn SetTimerUpdateHandler(&self, updatehandler: Option<&IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::Result<()>;
    fn SetTimerEventHandler(&self, handler: Option<&IUIAnimationTimerEventHandler>) -> windows_core::Result<()>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<()>;
    fn GetTime(&self) -> windows_core::Result<f64>;
    fn SetFrameRateThreshold(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimer {}
impl IUIAnimationTimer_Vtbl {
    pub const fn new<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>() -> IUIAnimationTimer_Vtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatehandler: *mut core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetTimerUpdateHandler(this, windows_core::from_raw_borrowed(&updatehandler), core::mem::transmute_copy(&idlebehavior)).into()
        }
        unsafe extern "system" fn SetTimerEventHandler<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetTimerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Enable<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Disable<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::Disable(this).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::IsEnabled(this).into()
        }
        unsafe extern "system" fn GetTime<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTimer_Impl::GetTime(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetFrameRateThreshold(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Identity, OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Disable: Disable::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            GetTime: GetTime::<Identity, OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimer as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerClientEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerClientEventHandler {}
impl IUIAnimationTimerClientEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerClientEventHandler_Vtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Identity: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerClientEventHandler_Impl::OnTimerClientStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnPreUpdate(&self) -> windows_core::Result<()>;
    fn OnPostUpdate(&self) -> windows_core::Result<()>;
    fn OnRenderingTooSlow(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerEventHandler {}
impl IUIAnimationTimerEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerEventHandler_Vtbl {
        unsafe extern "system" fn OnPreUpdate<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnPreUpdate(this).into()
        }
        unsafe extern "system" fn OnPostUpdate<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnPostUpdate(this).into()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnRenderingTooSlow(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPreUpdate: OnPreUpdate::<Identity, OFFSET>,
            OnPostUpdate: OnPostUpdate::<Identity, OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerUpdateHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnUpdate(&self, timenow: f64) -> windows_core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(&self, handler: Option<&IUIAnimationTimerClientEventHandler>) -> windows_core::Result<()>;
    fn ClearTimerClientEventHandler(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerUpdateHandler {}
impl IUIAnimationTimerUpdateHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerUpdateHandler_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTimerUpdateHandler_Impl::OnUpdate(this, core::mem::transmute_copy(&timenow)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerUpdateHandler_Impl::SetTimerClientEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerUpdateHandler_Impl::ClearTimerClientEventHandler(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Identity, OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerUpdateHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransition_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition {}
impl IUIAnimationTransition_Vtbl {
    pub const fn new<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>() -> IUIAnimationTransition_Vtbl {
        unsafe extern "system" fn SetInitialValue<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValue: SetInitialValue::<Identity, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransition2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVectorValue(&self, value: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn SetInitialVectorVelocity(&self, velocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition2 {}
impl IUIAnimationTransition2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>() -> IUIAnimationTransition2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVectorValue<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVectorVelocity(this, core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            SetInitialValue: SetInitialValue::<Identity, OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Identity, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Identity, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator>) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory {}
impl IUIAnimationTransitionFactory_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionFactory_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator2>) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory2 {}
impl IUIAnimationTransitionFactory2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory2_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionFactory2_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary {}
impl IUIAnimationTransitionLibrary_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateInstantaneousVectorTransition(&self, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary2 {}
impl IUIAnimationTransitionLibrary2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary2_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousVectorTransition(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteVectorTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    pptransition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    pptransition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Identity, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Identity, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Identity, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Identity, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Identity, OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariable_Impl: Sized + windows_core::IUnknownImpl {
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler>) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariable {}
impl IUIAnimationVariable_Vtbl {
    pub const fn new<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>() -> IUIAnimationVariable_Vtbl {
        unsafe extern "system" fn GetValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
pub trait IUIAnimationVariable2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetVectorValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurve(&self, animation: Option<&super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> windows_core::Result<()>;
    fn GetVectorCurve(&self, animation: *const Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetFinalVectorValue(&self, finalvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousVectorValue(&self, previousvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetIntegerVectorValue(&self, value: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerVectorValue(&self, finalvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerVectorValue(&self, previousvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetLowerBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableCurveChangeHandler(&self, handler: Option<&IUIAnimationVariableCurveChangeHandler2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl windows_core::RuntimeName for IUIAnimationVariable2 {}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl IUIAnimationVariable2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>() -> IUIAnimationVariable2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurve<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetCurve(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn GetVectorCurve<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *const *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetVectorCurve(this, core::mem::transmute_copy(&animation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetFinalVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetPreviousVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetIntegerVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetFinalIntegerVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetPreviousIntegerVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetLowerBoundVector<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetLowerBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBoundVector<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetUpperBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableCurveChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetVectorValue: GetVectorValue::<Identity, OFFSET>,
            GetCurve: GetCurve::<Identity, OFFSET>,
            GetVectorCurve: GetVectorCurve::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Identity, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Identity, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Identity, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Identity, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Identity, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Identity, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Identity, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler {}
impl IUIAnimationVariableChangeHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: f64, previousvalue: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableChangeHandler_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler2 {}
impl IUIAnimationVariableChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler2_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableChangeHandler2_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnCurveChanged(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableCurveChangeHandler2 {}
impl IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2_Vtbl {
        unsafe extern "system" fn OnCurveChanged<Identity: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableCurveChangeHandler2_Impl::OnCurveChanged(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCurveChanged: OnCurveChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler {}
impl IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: i32, previousvalue: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableIntegerChangeHandler_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler2 {}
impl IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableIntegerChangeHandler2_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as windows_core::Interface>::IID
    }
}
