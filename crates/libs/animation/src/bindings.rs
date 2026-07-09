windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
pub type CLSCTX = u32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
windows_core::imp::define_interface!(
    IDCompositionAnimation,
    IDCompositionAnimation_Vtbl,
    0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5
);
windows_core::imp::interface_hierarchy!(IDCompositionAnimation, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Reset: usize,
    SetAbsoluteBeginTime: usize,
    AddCubic: usize,
    AddSinusoidal: usize,
    AddRepeat: usize,
    End: usize,
}
impl windows_core::RuntimeName for IDCompositionAnimation {}
windows_core::imp::define_interface!(
    IUIAnimationManager2,
    IUIAnimationManager2_Vtbl,
    0xd8b6f7d4_4109_4d3f_acee_879926968cb1
);
windows_core::imp::interface_hierarchy!(IUIAnimationManager2, windows_core::IUnknown);
impl IUIAnimationManager2 {
    pub(crate) unsafe fn CreateAnimationVariable(
        &self,
        initialvalue: f64,
    ) -> windows_core::Result<IUIAnimationVariable2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAnimationVariable)(
                windows_core::Interface::as_raw(self),
                initialvalue,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn ScheduleTransition<P0, P1>(
        &self,
        variable: P0,
        transition: P1,
        timenow: f64,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ScheduleTransition)(
                windows_core::Interface::as_raw(self),
                variable.param().abi(),
                transition.param().abi(),
                timenow,
            )
        }
    }
    pub(crate) unsafe fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStoryboard)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn Update(
        &self,
        timenow: f64,
        updateresult: Option<*mut UI_ANIMATION_UPDATE_RESULT>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Update)(
                windows_core::Interface::as_raw(self),
                timenow,
                updateresult.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct IUIAnimationManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateAnimationVectorVariable: usize,
    pub CreateAnimationVariable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FinishAllStoryboards: usize,
    AbandonAllStoryboards: usize,
    pub Update: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        *mut UI_ANIMATION_UPDATE_RESULT,
    ) -> windows_core::HRESULT,
    GetVariableFromTag: usize,
    GetStoryboardFromTag: usize,
    EstimateNextEventTime: usize,
    GetStatus: usize,
    SetAnimationMode: usize,
    Pause: usize,
    Resume: usize,
    SetManagerEventHandler: usize,
    SetCancelPriorityComparison: usize,
    SetTrimPriorityComparison: usize,
    SetCompressPriorityComparison: usize,
    SetConcludePriorityComparison: usize,
    SetDefaultLongestAcceptableDelay: usize,
    Shutdown: usize,
}
impl windows_core::RuntimeName for IUIAnimationManager2 {}
windows_core::imp::define_interface!(
    IUIAnimationStoryboard2,
    IUIAnimationStoryboard2_Vtbl,
    0xae289cd2_12d4_4945_9419_9e41be034df2
);
windows_core::imp::interface_hierarchy!(IUIAnimationStoryboard2, windows_core::IUnknown);
impl IUIAnimationStoryboard2 {
    pub(crate) unsafe fn AddTransition<P0, P1>(
        &self,
        variable: P0,
        transition: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddTransition)(
                windows_core::Interface::as_raw(self),
                variable.param().abi(),
                transition.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn AddKeyframeAfterTransition<P0>(
        &self,
        transition: P0,
    ) -> windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: windows_core::Param<IUIAnimationTransition2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddKeyframeAfterTransition)(
                windows_core::Interface::as_raw(self),
                transition.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn AddTransitionAtKeyframe<P0, P1>(
        &self,
        variable: P0,
        transition: P1,
        startkeyframe: UI_ANIMATION_KEYFRAME,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddTransitionAtKeyframe)(
                windows_core::Interface::as_raw(self),
                variable.param().abi(),
                transition.param().abi(),
                startkeyframe,
            )
        }
    }
    pub(crate) unsafe fn Schedule(
        &self,
        timenow: f64,
        schedulingresult: Option<*mut UI_ANIMATION_SCHEDULING_RESULT>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Schedule)(
                windows_core::Interface::as_raw(self),
                timenow,
                schedulingresult.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct IUIAnimationStoryboard2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AddKeyframeAtOffset: usize,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut UI_ANIMATION_KEYFRAME,
    ) -> windows_core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        UI_ANIMATION_KEYFRAME,
    ) -> windows_core::HRESULT,
    AddTransitionBetweenKeyframes: usize,
    RepeatBetweenKeyframes: usize,
    HoldVariable: usize,
    SetLongestAcceptableDelay: usize,
    SetSkipDuration: usize,
    pub Schedule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        *mut UI_ANIMATION_SCHEDULING_RESULT,
    ) -> windows_core::HRESULT,
    Conclude: usize,
    Finish: usize,
    Abandon: usize,
    SetTag: usize,
    GetTag: usize,
    GetStatus: usize,
    GetElapsedTime: usize,
    SetStoryboardEventHandler: usize,
}
impl windows_core::RuntimeName for IUIAnimationStoryboard2 {}
windows_core::imp::define_interface!(
    IUIAnimationTransition2,
    IUIAnimationTransition2_Vtbl,
    0x62ff9123_a85a_4e9b_a218_435a93e268fd
);
windows_core::imp::interface_hierarchy!(IUIAnimationTransition2, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAnimationTransition2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetDimension: usize,
    SetInitialValue: usize,
    SetInitialVectorValue: usize,
    SetInitialVelocity: usize,
    SetInitialVectorVelocity: usize,
    IsDurationKnown: usize,
    GetDuration: usize,
}
impl windows_core::RuntimeName for IUIAnimationTransition2 {}
windows_core::imp::define_interface!(
    IUIAnimationTransitionLibrary2,
    IUIAnimationTransitionLibrary2_Vtbl,
    0x03cfae53_9580_4ee3_b363_2ece51b4af6a
);
windows_core::imp::interface_hierarchy!(IUIAnimationTransitionLibrary2, windows_core::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    pub(crate) unsafe fn CreateInstantaneousTransition(
        &self,
        finalvalue: f64,
    ) -> windows_core::Result<IUIAnimationTransition2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstantaneousTransition)(
                windows_core::Interface::as_raw(self),
                finalvalue,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateLinearTransition(
        &self,
        duration: f64,
        finalvalue: f64,
    ) -> windows_core::Result<IUIAnimationTransition2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearTransition)(
                windows_core::Interface::as_raw(self),
                duration,
                finalvalue,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateAccelerateDecelerateTransition(
        &self,
        duration: f64,
        finalvalue: f64,
        accelerationratio: f64,
        decelerationratio: f64,
    ) -> windows_core::Result<IUIAnimationTransition2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAccelerateDecelerateTransition)(
                windows_core::Interface::as_raw(self),
                duration,
                finalvalue,
                accelerationratio,
                decelerationratio,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAnimationTransitionLibrary2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateInstantaneousVectorTransition: usize,
    CreateConstantTransition: usize,
    CreateDiscreteTransition: usize,
    CreateDiscreteVectorTransition: usize,
    pub CreateLinearTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        f64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateLinearVectorTransition: usize,
    CreateLinearTransitionFromSpeed: usize,
    CreateLinearVectorTransitionFromSpeed: usize,
    CreateSinusoidalTransitionFromVelocity: usize,
    CreateSinusoidalTransitionFromRange: usize,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        f64,
        f64,
        f64,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    CreateReversalTransition: usize,
    CreateCubicTransition: usize,
    CreateCubicVectorTransition: usize,
    CreateSmoothStopTransition: usize,
    CreateParabolicTransitionFromAcceleration: usize,
    CreateCubicBezierLinearTransition: usize,
    CreateCubicBezierLinearVectorTransition: usize,
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary2 {}
windows_core::imp::define_interface!(
    IUIAnimationVariable2,
    IUIAnimationVariable2_Vtbl,
    0x4914b304_96ab_44d9_9e77_d5109b7e7466
);
windows_core::imp::interface_hierarchy!(IUIAnimationVariable2, windows_core::IUnknown);
impl IUIAnimationVariable2 {
    pub(crate) unsafe fn GetValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetCurve<P0>(&self, animation: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetCurve)(
                windows_core::Interface::as_raw(self),
                animation.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IUIAnimationVariable2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetDimension: usize,
    pub GetValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    GetVectorValue: usize,
    pub GetCurve: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetVectorCurve: usize,
    GetFinalValue: usize,
    GetFinalVectorValue: usize,
    GetPreviousValue: usize,
    GetPreviousVectorValue: usize,
    GetIntegerValue: usize,
    GetIntegerVectorValue: usize,
    GetFinalIntegerValue: usize,
    GetFinalIntegerVectorValue: usize,
    GetPreviousIntegerValue: usize,
    GetPreviousIntegerVectorValue: usize,
    GetCurrentStoryboard: usize,
    SetLowerBound: usize,
    SetLowerBoundVector: usize,
    SetUpperBound: usize,
    SetUpperBoundVector: usize,
    SetRoundingMode: usize,
    SetTag: usize,
    GetTag: usize,
    SetVariableChangeHandler: usize,
    SetVariableIntegerChangeHandler: usize,
    SetVariableCurveChangeHandler: usize,
}
impl windows_core::RuntimeName for IUIAnimationVariable2 {}
pub const UIAnimationManager2: windows_core::GUID =
    windows_core::GUID::from_u128(0xd25d8842_8884_4a4a_b321_091314379bdd);
pub const UIAnimationTransitionLibrary2: windows_core::GUID =
    windows_core::GUID::from_u128(0x812f944a_c5c8_4cd9_b0a6_b3da802f228d);
pub type UI_ANIMATION_KEYFRAME = *mut core::ffi::c_void;
pub type UI_ANIMATION_SCHEDULING_RESULT = i32;
pub type UI_ANIMATION_UPDATE_RESULT = i32;
