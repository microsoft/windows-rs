#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationInterpolator(::windows::core::IUnknown);
impl IUIAnimationInterpolator {
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialValueAndVelocity)(::windows::core::Vtable::as_raw(self), initialvalue, initialvelocity).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDuration)(::windows::core::Vtable::as_raw(self), duration).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFinalValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InterpolateValue(&self, offset: f64) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InterpolateValue)(::windows::core::Vtable::as_raw(self), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InterpolateVelocity)(::windows::core::Vtable::as_raw(self), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDependencies)(::windows::core::Vtable::as_raw(self), initialvaluedependencies, initialvelocitydependencies, durationdependencies).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationInterpolator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationInterpolator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationInterpolator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationInterpolator {}
impl ::core::fmt::Debug for IUIAnimationInterpolator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationInterpolator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationInterpolator {
    type Vtable = IUIAnimationInterpolator_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationInterpolator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7815cbba_ddf7_478c_a46c_7b6c738b7978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationInterpolator2(::windows::core::IUnknown);
impl IUIAnimationInterpolator2 {
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDimension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialValueAndVelocity)(::windows::core::Vtable::as_raw(self), initialvalue, initialvelocity, cdimension).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDuration)(::windows::core::Vtable::as_raw(self), duration).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFinalValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    pub unsafe fn InterpolateValue(&self, offset: f64, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InterpolateValue)(::windows::core::Vtable::as_raw(self), offset, ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64, velocity: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InterpolateVelocity)(::windows::core::Vtable::as_raw(self), offset, ::core::mem::transmute(velocity.as_ptr()), velocity.len() as _).ok()
    }
    pub unsafe fn GetPrimitiveInterpolation<P0>(&self, interpolation: P0, cdimension: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPrimitiveInterpolation>>,
    {
        (::windows::core::Vtable::vtable(self).GetPrimitiveInterpolation)(::windows::core::Vtable::as_raw(self), interpolation.into().abi(), cdimension).ok()
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDependencies)(::windows::core::Vtable::as_raw(self), initialvaluedependencies, initialvelocitydependencies, durationdependencies).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationInterpolator2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationInterpolator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationInterpolator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationInterpolator2 {}
impl ::core::fmt::Debug for IUIAnimationInterpolator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationInterpolator2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationInterpolator2 {
    type Vtable = IUIAnimationInterpolator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationInterpolator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea76aff8_ea22_4a23_a0ef_a6a966703518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPrimitiveInterpolation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolation: *mut ::core::ffi::c_void, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationLoopIterationChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationLoopIterationChangeHandler2 {
    pub unsafe fn OnLoopIterationChanged<P0>(&self, storyboard: P0, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
    {
        (::windows::core::Vtable::vtable(self).OnLoopIterationChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), id, newiterationcount, olditerationcount).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationLoopIterationChangeHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationLoopIterationChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationLoopIterationChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationLoopIterationChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationLoopIterationChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationLoopIterationChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationLoopIterationChangeHandler2 {
    type Vtable = IUIAnimationLoopIterationChangeHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationLoopIterationChangeHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3b15a4_4762_47ab_a030_b23221df3ae0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnLoopIterationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationManager(::windows::core::IUnknown);
impl IUIAnimationManager {
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAnimationVariable)(::windows::core::Vtable::as_raw(self), initialvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition>>,
    {
        (::windows::core::Vtable::vtable(self).ScheduleTransition)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), timenow).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStoryboard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishAllStoryboards)(::windows::core::Vtable::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AbandonAllStoryboards)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: ::core::option::Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self), timenow, ::core::mem::transmute(updateresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<IUIAnimationVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVariableFromTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStoryboardFromTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAnimationMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetManagerEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationManagerEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetManagerEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison>>,
    {
        (::windows::core::Vtable::vtable(self).SetCancelPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison>>,
    {
        (::windows::core::Vtable::vtable(self).SetTrimPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison>>,
    {
        (::windows::core::Vtable::vtable(self).SetCompressPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison>>,
    {
        (::windows::core::Vtable::vtable(self).SetConcludePriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultLongestAcceptableDelay)(::windows::core::Vtable::as_raw(self), delay).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManager {}
impl ::core::fmt::Debug for IUIAnimationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationManager {
    type Vtable = IUIAnimationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9169896c_ac8d_4e7d_94e5_67fa4dc2f2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationManager2(::windows::core::IUnknown);
impl IUIAnimationManager2 {
    pub unsafe fn CreateAnimationVectorVariable(&self, initialvalue: &[f64]) -> ::windows::core::Result<IUIAnimationVariable2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAnimationVectorVariable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(initialvalue.as_ptr()), initialvalue.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAnimationVariable)(::windows::core::Vtable::as_raw(self), initialvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition2>>,
    {
        (::windows::core::Vtable::vtable(self).ScheduleTransition)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), timenow).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStoryboard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishAllStoryboards)(::windows::core::Vtable::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AbandonAllStoryboards)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: ::core::option::Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self), timenow, ::core::mem::transmute(updateresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<IUIAnimationVariable2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVariableFromTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStoryboardFromTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EstimateNextEventTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EstimateNextEventTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAnimationMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManagerEventHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationManagerEventHandler2>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetManagerEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi(), fregisterfornextanimationevent.into()).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison2>>,
    {
        (::windows::core::Vtable::vtable(self).SetCancelPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison2>>,
    {
        (::windows::core::Vtable::vtable(self).SetTrimPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison2>>,
    {
        (::windows::core::Vtable::vtable(self).SetCompressPriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationPriorityComparison2>>,
    {
        (::windows::core::Vtable::vtable(self).SetConcludePriorityComparison)(::windows::core::Vtable::as_raw(self), comparison.into().abi()).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultLongestAcceptableDelay)(::windows::core::Vtable::as_raw(self), delay).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationManager2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManager2 {}
impl ::core::fmt::Debug for IUIAnimationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationManager2 {
    type Vtable = IUIAnimationManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8b6f7d4_4109_4d3f_acee_879926968cb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateAnimationVectorVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EstimateNextEventTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManagerEventHandler: usize,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler(::windows::core::IUnknown);
impl IUIAnimationManagerEventHandler {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnManagerStatusChanged)(::windows::core::Vtable::as_raw(self), newstatus, previousstatus).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationManagerEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationManagerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManagerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManagerEventHandler {}
impl ::core::fmt::Debug for IUIAnimationManagerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManagerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationManagerEventHandler {
    type Vtable = IUIAnimationManagerEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationManagerEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x783321ed_78a3_4366_b574_6af607a64788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler2(::windows::core::IUnknown);
impl IUIAnimationManagerEventHandler2 {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnManagerStatusChanged)(::windows::core::Vtable::as_raw(self), newstatus, previousstatus).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationManagerEventHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationManagerEventHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManagerEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManagerEventHandler2 {}
impl ::core::fmt::Debug for IUIAnimationManagerEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManagerEventHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationManagerEventHandler2 {
    type Vtable = IUIAnimationManagerEventHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationManagerEventHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e022ba_bff3_42ec_9033_e073f33e83c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationPrimitiveInterpolation(::windows::core::IUnknown);
impl IUIAnimationPrimitiveInterpolation {
    pub unsafe fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddCubic)(::windows::core::Vtable::as_raw(self), dimension, beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient).ok()
    }
    pub unsafe fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddSinusoidal)(::windows::core::Vtable::as_raw(self), dimension, beginoffset, bias, amplitude, frequency, phase).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationPrimitiveInterpolation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationPrimitiveInterpolation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPrimitiveInterpolation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPrimitiveInterpolation {}
impl ::core::fmt::Debug for IUIAnimationPrimitiveInterpolation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPrimitiveInterpolation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationPrimitiveInterpolation {
    type Vtable = IUIAnimationPrimitiveInterpolation_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationPrimitiveInterpolation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab20d63_4361_45da_a24f_ab8508846b5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPrimitiveInterpolation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison(::windows::core::IUnknown);
impl IUIAnimationPriorityComparison {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
    {
        (::windows::core::Vtable::vtable(self).HasPriority)(::windows::core::Vtable::as_raw(self), scheduledstoryboard.into().abi(), newstoryboard.into().abi(), priorityeffect).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationPriorityComparison, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationPriorityComparison {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPriorityComparison {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPriorityComparison {}
impl ::core::fmt::Debug for IUIAnimationPriorityComparison {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPriorityComparison").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationPriorityComparison {
    type Vtable = IUIAnimationPriorityComparison_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationPriorityComparison {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83fa9b74_5f86_4618_bc6a_a2fac19b3f44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison2(::windows::core::IUnknown);
impl IUIAnimationPriorityComparison2 {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
    {
        (::windows::core::Vtable::vtable(self).HasPriority)(::windows::core::Vtable::as_raw(self), scheduledstoryboard.into().abi(), newstoryboard.into().abi(), priorityeffect).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationPriorityComparison2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationPriorityComparison2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPriorityComparison2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPriorityComparison2 {}
impl ::core::fmt::Debug for IUIAnimationPriorityComparison2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPriorityComparison2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationPriorityComparison2 {
    type Vtable = IUIAnimationPriorityComparison2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationPriorityComparison2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b6d7a37_4621_467c_8b05_70131de62ddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboard(::windows::core::IUnknown);
impl IUIAnimationStoryboard {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition>>,
    {
        (::windows::core::Vtable::vtable(self).AddTransition)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi()).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddKeyframeAtOffset)(::windows::core::Vtable::as_raw(self), existingkeyframe.into(), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddKeyframeAfterTransition)(::windows::core::Vtable::as_raw(self), transition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition>>,
        P2: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        (::windows::core::Vtable::vtable(self).AddTransitionAtKeyframe)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), startkeyframe.into()).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition>>,
        P2: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
        P3: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        (::windows::core::Vtable::vtable(self).AddTransitionBetweenKeyframes)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), startkeyframe.into(), endkeyframe.into()).ok()
    }
    pub unsafe fn RepeatBetweenKeyframes<P0, P1>(&self, startkeyframe: P0, endkeyframe: P1, repetitioncount: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
        P1: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        (::windows::core::Vtable::vtable(self).RepeatBetweenKeyframes)(::windows::core::Vtable::as_raw(self), startkeyframe.into(), endkeyframe.into(), repetitioncount).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
    {
        (::windows::core::Vtable::vtable(self).HoldVariable)(::windows::core::Vtable::as_raw(self), variable.into().abi()).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLongestAcceptableDelay)(::windows::core::Vtable::as_raw(self), delay).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: ::core::option::Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Schedule)(::windows::core::Vtable::as_raw(self), timenow, ::core::mem::transmute(schedulingresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Conclude(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Conclude)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish)(::windows::core::Vtable::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abandon)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElapsedTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStoryboardEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboardEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetStoryboardEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationStoryboard, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationStoryboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboard {}
impl ::core::fmt::Debug for IUIAnimationStoryboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationStoryboard {
    type Vtable = IUIAnimationStoryboard_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8ff128f_9bf9_4af1_9e67_e5e410defb84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboard2(::windows::core::IUnknown);
impl IUIAnimationStoryboard2 {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition2>>,
    {
        (::windows::core::Vtable::vtable(self).AddTransition)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi()).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddKeyframeAtOffset)(::windows::core::Vtable::as_raw(self), existingkeyframe.into(), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddKeyframeAfterTransition)(::windows::core::Vtable::as_raw(self), transition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition2>>,
        P2: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        (::windows::core::Vtable::vtable(self).AddTransitionAtKeyframe)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), startkeyframe.into()).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationTransition2>>,
        P2: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
        P3: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
    {
        (::windows::core::Vtable::vtable(self).AddTransitionBetweenKeyframes)(::windows::core::Vtable::as_raw(self), variable.into().abi(), transition.into().abi(), startkeyframe.into(), endkeyframe.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RepeatBetweenKeyframes<P0, P1, P2, P3>(&self, startkeyframe: P0, endkeyframe: P1, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: P2, id: usize, fregisterfornextanimationevent: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
        P1: ::std::convert::Into<UI_ANIMATION_KEYFRAME>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAnimationLoopIterationChangeHandler2>>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).RepeatBetweenKeyframes)(::windows::core::Vtable::as_raw(self), startkeyframe.into(), endkeyframe.into(), crepetition, repeatmode, piterationchangehandler.into().abi(), id, fregisterfornextanimationevent.into()).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
    {
        (::windows::core::Vtable::vtable(self).HoldVariable)(::windows::core::Vtable::as_raw(self), variable.into().abi()).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLongestAcceptableDelay)(::windows::core::Vtable::as_raw(self), delay).ok()
    }
    pub unsafe fn SetSkipDuration(&self, secondsduration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSkipDuration)(::windows::core::Vtable::as_raw(self), secondsduration).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: ::core::option::Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Schedule)(::windows::core::Vtable::as_raw(self), timenow, ::core::mem::transmute(schedulingresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Conclude(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Conclude)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish)(::windows::core::Vtable::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abandon)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElapsedTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStoryboardEventHandler<P0, P1, P2>(&self, handler: P0, fregisterstatuschangefornextanimationevent: P1, fregisterupdatefornextanimationevent: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboardEventHandler2>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetStoryboardEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi(), fregisterstatuschangefornextanimationevent.into(), fregisterupdatefornextanimationevent.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationStoryboard2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationStoryboard2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboard2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboard2 {}
impl ::core::fmt::Debug for IUIAnimationStoryboard2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboard2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationStoryboard2 {
    type Vtable = IUIAnimationStoryboard2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboard2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae289cd2_12d4_4945_9419_9e41be034df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut ::core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RepeatBetweenKeyframes: usize,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub SetSkipDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStoryboardEventHandler: usize,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler(::windows::core::IUnknown);
impl IUIAnimationStoryboardEventHandler {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
    {
        (::windows::core::Vtable::vtable(self).OnStoryboardStatusChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), newstatus, previousstatus).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
    {
        (::windows::core::Vtable::vtable(self).OnStoryboardUpdated)(::windows::core::Vtable::as_raw(self), storyboard.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationStoryboardEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationStoryboardEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboardEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboardEventHandler {}
impl ::core::fmt::Debug for IUIAnimationStoryboardEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboardEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationStoryboardEventHandler {
    type Vtable = IUIAnimationStoryboardEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboardEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d5c9008_ec7c_4364_9f8a_9af3c58cbae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler2(::windows::core::IUnknown);
impl IUIAnimationStoryboardEventHandler2 {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
    {
        (::windows::core::Vtable::vtable(self).OnStoryboardStatusChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), newstatus, previousstatus).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
    {
        (::windows::core::Vtable::vtable(self).OnStoryboardUpdated)(::windows::core::Vtable::as_raw(self), storyboard.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationStoryboardEventHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationStoryboardEventHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboardEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboardEventHandler2 {}
impl ::core::fmt::Debug for IUIAnimationStoryboardEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboardEventHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationStoryboardEventHandler2 {
    type Vtable = IUIAnimationStoryboardEventHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboardEventHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac5f55a_ba7c_414c_b599_fbf850f553c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTimer(::windows::core::IUnknown);
impl IUIAnimationTimer {
    pub unsafe fn SetTimerUpdateHandler<P0>(&self, updatehandler: P0, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationTimerUpdateHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetTimerUpdateHandler)(::windows::core::Vtable::as_raw(self), updatehandler.into().abi(), idlebehavior).ok()
    }
    pub unsafe fn SetTimerEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationTimerEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetTimerEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Enable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Disable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsEnabled)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFrameRateThreshold(&self, framespersecond: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFrameRateThreshold)(::windows::core::Vtable::as_raw(self), framespersecond).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTimer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimer {}
impl ::core::fmt::Debug for IUIAnimationTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTimer {
    type Vtable = IUIAnimationTimer_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTimer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b0efad1_a053_41d6_9085_33a689144665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetTimerUpdateHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatehandler: *mut ::core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT,
    pub SetTimerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT,
    pub SetFrameRateThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTimerClientEventHandler(::windows::core::IUnknown);
impl IUIAnimationTimerClientEventHandler {
    pub unsafe fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnTimerClientStatusChanged)(::windows::core::Vtable::as_raw(self), newstatus, previousstatus).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTimerClientEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTimerClientEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerClientEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerClientEventHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerClientEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerClientEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTimerClientEventHandler {
    type Vtable = IUIAnimationTimerClientEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerClientEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbedb4db6_94fa_4bfb_a47f_ef2d9e408c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerClientEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnTimerClientStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTimerEventHandler(::windows::core::IUnknown);
impl IUIAnimationTimerEventHandler {
    pub unsafe fn OnPreUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnPreUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnPostUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnPostUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnRenderingTooSlow(&self, framespersecond: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnRenderingTooSlow)(::windows::core::Vtable::as_raw(self), framespersecond).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTimerEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTimerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerEventHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTimerEventHandler {
    type Vtable = IUIAnimationTimerEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274a7dea_d771_4095_abbd_8df7abd23ce3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnPreUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPostUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnRenderingTooSlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTimerUpdateHandler(::windows::core::IUnknown);
impl IUIAnimationTimerUpdateHandler {
    pub unsafe fn OnUpdate(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OnUpdate)(::windows::core::Vtable::as_raw(self), timenow, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTimerClientEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationTimerClientEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetTimerClientEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn ClearTimerClientEventHandler(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearTimerClientEventHandler)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTimerUpdateHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTimerUpdateHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerUpdateHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerUpdateHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerUpdateHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerUpdateHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTimerUpdateHandler {
    type Vtable = IUIAnimationTimerUpdateHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerUpdateHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x195509b7_5d5e_4e3e_b278_ee3759b367ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerUpdateHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub SetTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransition(::windows::core::IUnknown);
impl IUIAnimationTransition {
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialValue)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVelocity)(::windows::core::Vtable::as_raw(self), velocity).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsDurationKnown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransition, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransition {}
impl ::core::fmt::Debug for IUIAnimationTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransition {
    type Vtable = IUIAnimationTransition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc6ce252_f731_41cf_b610_614b6ca049ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransition2(::windows::core::IUnknown);
impl IUIAnimationTransition2 {
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDimension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialValue)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn SetInitialVectorValue(&self, value: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVelocity)(::windows::core::Vtable::as_raw(self), velocity).ok()
    }
    pub unsafe fn SetInitialVectorVelocity(&self, velocity: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVectorVelocity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(velocity.as_ptr()), velocity.len() as _).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsDurationKnown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransition2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransition2 {}
impl ::core::fmt::Debug for IUIAnimationTransition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransition2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransition2 {
    type Vtable = IUIAnimationTransition2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransition2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62ff9123_a85a_4e9b_a218_435a93e268fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SetInitialVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT,
    pub SetInitialVectorVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory(::windows::core::IUnknown);
impl IUIAnimationTransitionFactory {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> ::windows::core::Result<IUIAnimationTransition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationInterpolator>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransition)(::windows::core::Vtable::as_raw(self), interpolator.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransitionFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionFactory {}
impl ::core::fmt::Debug for IUIAnimationTransitionFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransitionFactory {
    type Vtable = IUIAnimationTransitionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd91e03_3e3b_45ad_bbb1_6dfc8153743d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory2(::windows::core::IUnknown);
impl IUIAnimationTransitionFactory2 {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> ::windows::core::Result<IUIAnimationTransition2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationInterpolator2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransition)(::windows::core::Vtable::as_raw(self), interpolator.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransitionFactory2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransitionFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionFactory2 {}
impl ::core::fmt::Debug for IUIAnimationTransitionFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransitionFactory2 {
    type Vtable = IUIAnimationTransitionFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x937d4916_c1a6_42d5_88d8_30344d6efe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary(::windows::core::IUnknown);
impl IUIAnimationTransitionLibrary {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstantaneousTransition)(::windows::core::Vtable::as_raw(self), finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateConstantTransition)(::windows::core::Vtable::as_raw(self), duration, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDiscreteTransition)(::windows::core::Vtable::as_raw(self), delay, finalvalue, hold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearTransitionFromSpeed)(::windows::core::Vtable::as_raw(self), speed, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSinusoidalTransitionFromVelocity)(::windows::core::Vtable::as_raw(self), duration, period, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSinusoidalTransitionFromRange)(::windows::core::Vtable::as_raw(self), duration, minimumvalue, maximumvalue, period, slope, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAccelerateDecelerateTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, accelerationratio, decelerationratio, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReversalTransition)(::windows::core::Vtable::as_raw(self), duration, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCubicTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, finalvelocity, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSmoothStopTransition)(::windows::core::Vtable::as_raw(self), maximumduration, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateParabolicTransitionFromAcceleration)(::windows::core::Vtable::as_raw(self), finalvalue, finalvelocity, acceleration, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransitionLibrary, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransitionLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionLibrary {}
impl ::core::fmt::Debug for IUIAnimationTransitionLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransitionLibrary {
    type Vtable = IUIAnimationTransitionLibrary_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionLibrary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca5a14b1_d24f_48b8_8fe4_c78169ba954e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary2(::windows::core::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstantaneousTransition)(::windows::core::Vtable::as_raw(self), finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInstantaneousVectorTransition(&self, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstantaneousVectorTransition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateConstantTransition)(::windows::core::Vtable::as_raw(self), duration, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDiscreteTransition)(::windows::core::Vtable::as_raw(self), delay, finalvalue, hold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: &[f64], hold: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDiscreteVectorTransition)(::windows::core::Vtable::as_raw(self), delay, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _, hold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearVectorTransition)(::windows::core::Vtable::as_raw(self), duration, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearTransitionFromSpeed)(::windows::core::Vtable::as_raw(self), speed, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearVectorTransitionFromSpeed)(::windows::core::Vtable::as_raw(self), speed, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSinusoidalTransitionFromVelocity)(::windows::core::Vtable::as_raw(self), duration, period, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSinusoidalTransitionFromRange)(::windows::core::Vtable::as_raw(self), duration, minimumvalue, maximumvalue, period, slope, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAccelerateDecelerateTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, accelerationratio, decelerationratio, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReversalTransition)(::windows::core::Vtable::as_raw(self), duration, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCubicTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, finalvelocity, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCubicVectorTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, finalvelocity, cdimension, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSmoothStopTransition)(::windows::core::Vtable::as_raw(self), maximumduration, finalvalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateParabolicTransitionFromAcceleration)(::windows::core::Vtable::as_raw(self), finalvalue, finalvelocity, acceleration, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCubicBezierLinearTransition)(::windows::core::Vtable::as_raw(self), duration, finalvalue, x1, y1, x2, y2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: &[f64], x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCubicBezierLinearVectorTransition)(::windows::core::Vtable::as_raw(self), duration, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _, x1, y1, x2, y2, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAnimationTransitionLibrary2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationTransitionLibrary2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionLibrary2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionLibrary2 {}
impl ::core::fmt::Debug for IUIAnimationTransitionLibrary2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionLibrary2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationTransitionLibrary2 {
    type Vtable = IUIAnimationTransitionLibrary2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionLibrary2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03cfae53_9580_4ee3_b363_2ece51b4af6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstantaneousVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDiscreteVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearVectorTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCubicVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCubicBezierLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCubicBezierLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariable(::windows::core::IUnknown);
impl IUIAnimationVariable {
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFinalValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFinalIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentStoryboard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLowerBound)(::windows::core::Vtable::as_raw(self), bound).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUpperBound)(::windows::core::Vtable::as_raw(self), bound).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRoundingMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetVariableChangeHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariableChangeHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetVariableChangeHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn SetVariableIntegerChangeHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariableIntegerChangeHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetVariableIntegerChangeHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariable {}
impl ::core::fmt::Debug for IUIAnimationVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariable {
    type Vtable = IUIAnimationVariable_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ceeb155_2849_4ce5_9448_91ff70e1e4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariable2(::windows::core::IUnknown);
impl IUIAnimationVariable2 {
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDimension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVectorValue(&self, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetCurve<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::DirectComposition::IDCompositionAnimation>>,
    {
        (::windows::core::Vtable::vtable(self).GetCurve)(::windows::core::Vtable::as_raw(self), animation.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetVectorCurve(&self, animation: &[super::super::Graphics::DirectComposition::IDCompositionAnimation]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetVectorCurve)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(animation.as_ptr()), animation.len() as _).ok()
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFinalValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalVectorValue(&self, finalvalue: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFinalVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _).ok()
    }
    pub unsafe fn GetPreviousValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousVectorValue(&self, previousvalue: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPreviousVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(previousvalue.as_ptr()), previousvalue.len() as _).ok()
    }
    pub unsafe fn GetIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIntegerVectorValue(&self, value: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIntegerVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFinalIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFinalIntegerVectorValue(&self, finalvalue: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFinalIntegerVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len() as _).ok()
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousIntegerValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousIntegerVectorValue(&self, previousvalue: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPreviousIntegerVectorValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(previousvalue.as_ptr()), previousvalue.len() as _).ok()
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentStoryboard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLowerBound)(::windows::core::Vtable::as_raw(self), bound).ok()
    }
    pub unsafe fn SetLowerBoundVector(&self, bound: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLowerBoundVector)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bound.as_ptr()), bound.len() as _).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUpperBound)(::windows::core::Vtable::as_raw(self), bound).ok()
    }
    pub unsafe fn SetUpperBoundVector(&self, bound: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUpperBoundVector)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(bound.as_ptr()), bound.len() as _).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRoundingMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTag)(::windows::core::Vtable::as_raw(self), object.into().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetTag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariableChangeHandler2>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetVariableChangeHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi(), fregisterfornextanimationevent.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableIntegerChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariableIntegerChangeHandler2>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetVariableIntegerChangeHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi(), fregisterfornextanimationevent.into()).ok()
    }
    pub unsafe fn SetVariableCurveChangeHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariableCurveChangeHandler2>>,
    {
        (::windows::core::Vtable::vtable(self).SetVariableCurveChangeHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariable2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariable2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariable2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariable2 {}
impl ::core::fmt::Debug for IUIAnimationVariable2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariable2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariable2 {
    type Vtable = IUIAnimationVariable2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariable2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4914b304_96ab_44d9_9e77_d5109b7e7466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub GetVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetCurve: usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetVectorCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *const *mut ::core::ffi::c_void, cdimension: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetVectorCurve: usize,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetPreviousVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub GetIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetLowerBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetUpperBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableChangeHandler: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableIntegerChangeHandler: usize,
    pub SetVariableCurveChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler(::windows::core::IUnknown);
impl IUIAnimationVariableChangeHandler {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: f64, previousvalue: f64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
    {
        (::windows::core::Vtable::vtable(self).OnValueChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), variable.into().abi(), newvalue, previousvalue).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariableChangeHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariableChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableChangeHandler {}
impl ::core::fmt::Debug for IUIAnimationVariableChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableChangeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariableChangeHandler {
    type Vtable = IUIAnimationVariableChangeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableChangeHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6358b7ba_87d2_42d5_bf71_82e919dd5862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableChangeHandler2 {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
    {
        (::windows::core::Vtable::vtable(self).OnValueChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), variable.into().abi(), newvalue, previousvalue, cdimension).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariableChangeHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariableChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariableChangeHandler2 {
    type Vtable = IUIAnimationVariableChangeHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableChangeHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63acc8d2_6eae_4bb0_b879_586dd8cfbe42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariableCurveChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableCurveChangeHandler2 {
    pub unsafe fn OnCurveChanged<P0>(&self, variable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
    {
        (::windows::core::Vtable::vtable(self).OnCurveChanged)(::windows::core::Vtable::as_raw(self), variable.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariableCurveChangeHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariableCurveChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableCurveChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableCurveChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableCurveChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableCurveChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariableCurveChangeHandler2 {
    type Vtable = IUIAnimationVariableCurveChangeHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableCurveChangeHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72895e91_0145_4c21_9192_5aab40eddf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCurveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler(::windows::core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: i32, previousvalue: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable>>,
    {
        (::windows::core::Vtable::vtable(self).OnIntegerValueChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), variable.into().abi(), newvalue, previousvalue).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariableIntegerChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableIntegerChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableIntegerChangeHandler {}
impl ::core::fmt::Debug for IUIAnimationVariableIntegerChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableIntegerChangeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariableIntegerChangeHandler {
    type Vtable = IUIAnimationVariableIntegerChangeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableIntegerChangeHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3e1550_356e_44b0_99da_85ac6017865e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler2 {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAnimationStoryboard2>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAnimationVariable2>>,
    {
        (::windows::core::Vtable::vtable(self).OnIntegerValueChanged)(::windows::core::Vtable::as_raw(self), storyboard.into().abi(), variable.into().abi(), newvalue, previousvalue, cdimension).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAnimationVariableIntegerChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableIntegerChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableIntegerChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableIntegerChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableIntegerChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIAnimationVariableIntegerChangeHandler2 {
    type Vtable = IUIAnimationVariableIntegerChangeHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableIntegerChangeHandler2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x829b6cf1_4f3a_4412_ae09_b243eb4c6b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationManager2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd25d8842_8884_4a4a_b321_091314379bdd);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationTimer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcd4a0c_06b6_4384_b768_0daa792c380e);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationTransitionFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9b1cdd_fcd7_419c_8b44_42fd17db1887);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationTransitionFactory2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84302f97_7f7b_4040_b190_72ac9d18e420);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationTransitionLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d6322ad_aa85_4ef5_a828_86d71067d145);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UIAnimationTransitionLibrary2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x812f944a_c5c8_4cd9_b0a6_b3da802f228d);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_DEPENDENCIES(pub u32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0u32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1u32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2u32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4u32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8u32);
impl ::core::marker::Copy for UI_ANIMATION_DEPENDENCIES {}
impl ::core::clone::Clone for UI_ANIMATION_DEPENDENCIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_DEPENDENCIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_DEPENDENCIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_DEPENDENCIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_DEPENDENCIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
impl ::core::marker::Copy for UI_ANIMATION_IDLE_BEHAVIOR {}
impl ::core::clone::Clone for UI_ANIMATION_IDLE_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_IDLE_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_IDLE_BEHAVIOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_IDLE_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_IDLE_BEHAVIOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
impl ::core::marker::Copy for UI_ANIMATION_MANAGER_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_MANAGER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MANAGER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_MANAGER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_MANAGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MANAGER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
impl ::core::marker::Copy for UI_ANIMATION_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
impl ::core::marker::Copy for UI_ANIMATION_PRIORITY_EFFECT {}
impl ::core::clone::Clone for UI_ANIMATION_PRIORITY_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_PRIORITY_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_PRIORITY_EFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_PRIORITY_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_PRIORITY_EFFECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
impl ::core::marker::Copy for UI_ANIMATION_REPEAT_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_REPEAT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_REPEAT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_REPEAT_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_REPEAT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_REPEAT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
impl ::core::marker::Copy for UI_ANIMATION_ROUNDING_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_ROUNDING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_ROUNDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_ROUNDING_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_ROUNDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_ROUNDING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
impl ::core::marker::Copy for UI_ANIMATION_SCHEDULING_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_SCHEDULING_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SCHEDULING_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_SCHEDULING_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_SCHEDULING_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SCHEDULING_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_SLOPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
impl ::core::marker::Copy for UI_ANIMATION_SLOPE {}
impl ::core::clone::Clone for UI_ANIMATION_SLOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SLOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_SLOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_SLOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SLOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
impl ::core::marker::Copy for UI_ANIMATION_STORYBOARD_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_STORYBOARD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_STORYBOARD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_STORYBOARD_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_STORYBOARD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_STORYBOARD_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
impl ::core::marker::Copy for UI_ANIMATION_TIMER_CLIENT_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_TIMER_CLIENT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_TIMER_CLIENT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
#[doc = "*Required features: `\"Win32_UI_Animation\"`*"]
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);
impl ::core::marker::Copy for UI_ANIMATION_UPDATE_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_UPDATE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_UPDATE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_UPDATE_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_UPDATE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_UPDATE_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_KEYFRAME(pub isize);
impl ::core::default::Default for UI_ANIMATION_KEYFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for UI_ANIMATION_KEYFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for UI_ANIMATION_KEYFRAME {}
impl ::core::fmt::Debug for UI_ANIMATION_KEYFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_KEYFRAME").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<UI_ANIMATION_KEYFRAME>> for UI_ANIMATION_KEYFRAME {
    fn from(optional: ::core::option::Option<UI_ANIMATION_KEYFRAME>) -> UI_ANIMATION_KEYFRAME {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_KEYFRAME {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
