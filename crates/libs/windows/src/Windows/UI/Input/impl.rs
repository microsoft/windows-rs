#[cfg(feature = "implement_exclusive")]
pub trait IAttachableInputObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.IAttachableInputObject";
}
#[cfg(feature = "implement_exclusive")]
impl IAttachableInputObjectVtbl {
    pub const fn new<Impl: IAttachableInputObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAttachableInputObjectVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAttachableInputObject>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAttachableInputObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAttachableInputObjectFactory {
    const NAME: &'static str = "Windows.UI.Input.IAttachableInputObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAttachableInputObjectFactoryVtbl {
    pub const fn new<Impl: IAttachableInputObjectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAttachableInputObjectFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAttachableInputObjectFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICrossSlidingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn CrossSlidingState(&self) -> ::windows::core::Result<CrossSlidingState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ICrossSlidingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICrossSlidingEventArgsVtbl {
    pub const fn new<Impl: ICrossSlidingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICrossSlidingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ICrossSlidingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ICrossSlidingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CrossSlidingState<Impl: ICrossSlidingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CrossSlidingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrossSlidingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICrossSlidingEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, CrossSlidingState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICrossSlidingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICrossSlidingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.ICrossSlidingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ICrossSlidingEventArgs2Vtbl {
    pub const fn new<Impl: ICrossSlidingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICrossSlidingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: ICrossSlidingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICrossSlidingEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDraggingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn DraggingState(&self) -> ::windows::core::Result<DraggingState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IDraggingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDraggingEventArgsVtbl {
    pub const fn new<Impl: IDraggingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDraggingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IDraggingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IDraggingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DraggingState<Impl: IDraggingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DraggingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DraggingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDraggingEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, DraggingState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDraggingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDraggingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IDraggingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDraggingEventArgs2Vtbl {
    pub const fn new<Impl: IDraggingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDraggingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IDraggingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDraggingEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureImpl: Sized {
    fn Starting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Canceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGesture";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeGestureVtbl {
    pub const fn new<Impl: IEdgeGestureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEdgeGestureVtbl {
        unsafe extern "system" fn Starting<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Starting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStarting<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Canceled<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IEdgeGestureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEdgeGesture>, base.5, Starting::<Impl, OFFSET>, RemoveStarting::<Impl, OFFSET>, Completed::<Impl, OFFSET>, RemoveCompleted::<Impl, OFFSET>, Canceled::<Impl, OFFSET>, RemoveCanceled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<EdgeGestureKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGestureEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeGestureEventArgsVtbl {
    pub const fn new<Impl: IEdgeGestureEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEdgeGestureEventArgsVtbl {
        unsafe extern "system" fn Kind<Impl: IEdgeGestureEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EdgeGestureKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEdgeGestureEventArgs>, base.5, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<EdgeGesture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeGestureStatics {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGestureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeGestureStaticsVtbl {
    pub const fn new<Impl: IEdgeGestureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEdgeGestureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IEdgeGestureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEdgeGestureStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGestureRecognizerImpl: Sized {
    fn GestureSettings(&self) -> ::windows::core::Result<GestureSettings>;
    fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::core::Result<()>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn ShowGestureFeedback(&self) -> ::windows::core::Result<bool>;
    fn SetShowGestureFeedback(&self, value: bool) -> ::windows::core::Result<()>;
    fn PivotCenter(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPivotCenter(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn PivotRadius(&self) -> ::windows::core::Result<f32>;
    fn SetPivotRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansionDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDisplacement(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansion(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansion(&self, value: f32) -> ::windows::core::Result<()>;
    fn ManipulationExact(&self) -> ::windows::core::Result<bool>;
    fn SetManipulationExact(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideThresholds(&self) -> ::windows::core::Result<CrossSlideThresholds>;
    fn SetCrossSlideThresholds(&self, value: &CrossSlideThresholds) -> ::windows::core::Result<()>;
    fn CrossSlideHorizontally(&self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideExact(&self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideExact(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoProcessInertia(&self) -> ::windows::core::Result<bool>;
    fn SetAutoProcessInertia(&self, value: bool) -> ::windows::core::Result<()>;
    fn MouseWheelParameters(&self) -> ::windows::core::Result<MouseWheelParameters>;
    fn CanBeDoubleTap(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<bool>;
    fn ProcessDownEvent(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMoveEvents(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<PointerPoint>>) -> ::windows::core::Result<()>;
    fn ProcessUpEvent(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMouseWheelEvent(&self, value: &::core::option::Option<PointerPoint>, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::Result<()>;
    fn ProcessInertia(&self) -> ::windows::core::Result<()>;
    fn CompleteGesture(&self) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Dragging(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragging(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CrossSliding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCrossSliding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.IGestureRecognizer";
}
#[cfg(feature = "implement_exclusive")]
impl IGestureRecognizerVtbl {
    pub const fn new<Impl: IGestureRecognizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGestureRecognizerVtbl {
        unsafe extern "system" fn GestureSettings<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GestureSettings) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GestureSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureSettings<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: GestureSettings) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGestureSettings(value).into()
        }
        unsafe extern "system" fn IsInertial<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowGestureFeedback<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowGestureFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowGestureFeedback<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowGestureFeedback(value).into()
        }
        unsafe extern "system" fn PivotCenter<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PivotCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotCenter<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPivotCenter(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PivotRadius<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PivotRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPivotRadius(value).into()
        }
        unsafe extern "system" fn InertiaTranslationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaTranslationDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaTranslationDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaRotationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaRotationDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaRotationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaRotationDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaExpansionDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaExpansionDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaExpansionDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaExpansionDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaTranslationDisplacement<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaTranslationDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDisplacement<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaTranslationDisplacement(value).into()
        }
        unsafe extern "system" fn InertiaRotationAngle<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaRotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaRotationAngle<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaRotationAngle(value).into()
        }
        unsafe extern "system" fn InertiaExpansion<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InertiaExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaExpansion<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInertiaExpansion(value).into()
        }
        unsafe extern "system" fn ManipulationExact<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationExact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationExact<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetManipulationExact(value).into()
        }
        unsafe extern "system" fn CrossSlideThresholds<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CrossSlideThresholds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrossSlideThresholds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideThresholds<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CrossSlideThresholds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCrossSlideThresholds(&*(&value as *const <CrossSlideThresholds as ::windows::core::Abi>::Abi as *const <CrossSlideThresholds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CrossSlideHorizontally<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrossSlideHorizontally() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideHorizontally<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCrossSlideHorizontally(value).into()
        }
        unsafe extern "system" fn CrossSlideExact<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrossSlideExact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideExact<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCrossSlideExact(value).into()
        }
        unsafe extern "system" fn AutoProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoProcessInertia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoProcessInertia(value).into()
        }
        unsafe extern "system" fn MouseWheelParameters<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MouseWheelParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBeDoubleTap<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanBeDoubleTap(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessDownEvent<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessDownEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessMoveEvents<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessMoveEvents(&*(&value as *const <super::super::Foundation::Collections::IVector<PointerPoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<PointerPoint> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessUpEvent<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessUpEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessMouseWheelEvent<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessMouseWheelEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType), isshiftkeydown, iscontrolkeydown).into()
        }
        unsafe extern "system" fn ProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessInertia().into()
        }
        unsafe extern "system" fn CompleteGesture<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CompleteGesture().into()
        }
        unsafe extern "system" fn Tapped<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RightTapped<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RightTapped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRightTapped<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRightTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Holding<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Holding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHolding<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Dragging<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dragging(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragging<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragging(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationUpdated<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationUpdated<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationInertiaStarting<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationInertiaStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CrossSliding<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrossSliding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCrossSliding<Impl: IGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCrossSliding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IGestureRecognizer>,
            base.5,
            GestureSettings::<Impl, OFFSET>,
            SetGestureSettings::<Impl, OFFSET>,
            IsInertial::<Impl, OFFSET>,
            IsActive::<Impl, OFFSET>,
            ShowGestureFeedback::<Impl, OFFSET>,
            SetShowGestureFeedback::<Impl, OFFSET>,
            PivotCenter::<Impl, OFFSET>,
            SetPivotCenter::<Impl, OFFSET>,
            PivotRadius::<Impl, OFFSET>,
            SetPivotRadius::<Impl, OFFSET>,
            InertiaTranslationDeceleration::<Impl, OFFSET>,
            SetInertiaTranslationDeceleration::<Impl, OFFSET>,
            InertiaRotationDeceleration::<Impl, OFFSET>,
            SetInertiaRotationDeceleration::<Impl, OFFSET>,
            InertiaExpansionDeceleration::<Impl, OFFSET>,
            SetInertiaExpansionDeceleration::<Impl, OFFSET>,
            InertiaTranslationDisplacement::<Impl, OFFSET>,
            SetInertiaTranslationDisplacement::<Impl, OFFSET>,
            InertiaRotationAngle::<Impl, OFFSET>,
            SetInertiaRotationAngle::<Impl, OFFSET>,
            InertiaExpansion::<Impl, OFFSET>,
            SetInertiaExpansion::<Impl, OFFSET>,
            ManipulationExact::<Impl, OFFSET>,
            SetManipulationExact::<Impl, OFFSET>,
            CrossSlideThresholds::<Impl, OFFSET>,
            SetCrossSlideThresholds::<Impl, OFFSET>,
            CrossSlideHorizontally::<Impl, OFFSET>,
            SetCrossSlideHorizontally::<Impl, OFFSET>,
            CrossSlideExact::<Impl, OFFSET>,
            SetCrossSlideExact::<Impl, OFFSET>,
            AutoProcessInertia::<Impl, OFFSET>,
            SetAutoProcessInertia::<Impl, OFFSET>,
            MouseWheelParameters::<Impl, OFFSET>,
            CanBeDoubleTap::<Impl, OFFSET>,
            ProcessDownEvent::<Impl, OFFSET>,
            ProcessMoveEvents::<Impl, OFFSET>,
            ProcessUpEvent::<Impl, OFFSET>,
            ProcessMouseWheelEvent::<Impl, OFFSET>,
            ProcessInertia::<Impl, OFFSET>,
            CompleteGesture::<Impl, OFFSET>,
            Tapped::<Impl, OFFSET>,
            RemoveTapped::<Impl, OFFSET>,
            RightTapped::<Impl, OFFSET>,
            RemoveRightTapped::<Impl, OFFSET>,
            Holding::<Impl, OFFSET>,
            RemoveHolding::<Impl, OFFSET>,
            Dragging::<Impl, OFFSET>,
            RemoveDragging::<Impl, OFFSET>,
            ManipulationStarted::<Impl, OFFSET>,
            RemoveManipulationStarted::<Impl, OFFSET>,
            ManipulationUpdated::<Impl, OFFSET>,
            RemoveManipulationUpdated::<Impl, OFFSET>,
            ManipulationInertiaStarting::<Impl, OFFSET>,
            RemoveManipulationInertiaStarting::<Impl, OFFSET>,
            ManipulationCompleted::<Impl, OFFSET>,
            RemoveManipulationCompleted::<Impl, OFFSET>,
            CrossSliding::<Impl, OFFSET>,
            RemoveCrossSliding::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGestureRecognizer2Impl: Sized {
    fn TapMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTapMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn TapMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTapMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetHoldMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetHoldMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldRadius(&self) -> ::windows::core::Result<f32>;
    fn SetHoldRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn HoldStartDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetHoldStartDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TranslationMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTranslationMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn TranslationMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTranslationMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGestureRecognizer2 {
    const NAME: &'static str = "Windows.UI.Input.IGestureRecognizer2";
}
#[cfg(feature = "implement_exclusive")]
impl IGestureRecognizer2Vtbl {
    pub const fn new<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGestureRecognizer2Vtbl {
        unsafe extern "system" fn TapMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TapMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTapMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTapMinContactCount(value).into()
        }
        unsafe extern "system" fn TapMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TapMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTapMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTapMaxContactCount(value).into()
        }
        unsafe extern "system" fn HoldMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHoldMinContactCount(value).into()
        }
        unsafe extern "system" fn HoldMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHoldMaxContactCount(value).into()
        }
        unsafe extern "system" fn HoldRadius<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldRadius<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHoldRadius(value).into()
        }
        unsafe extern "system" fn HoldStartDelay<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldStartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldStartDelay<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHoldStartDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslationMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslationMinContactCount(value).into()
        }
        unsafe extern "system" fn TranslationMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslationMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslationMaxContactCount(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IGestureRecognizer2>,
            base.5,
            TapMinContactCount::<Impl, OFFSET>,
            SetTapMinContactCount::<Impl, OFFSET>,
            TapMaxContactCount::<Impl, OFFSET>,
            SetTapMaxContactCount::<Impl, OFFSET>,
            HoldMinContactCount::<Impl, OFFSET>,
            SetHoldMinContactCount::<Impl, OFFSET>,
            HoldMaxContactCount::<Impl, OFFSET>,
            SetHoldMaxContactCount::<Impl, OFFSET>,
            HoldRadius::<Impl, OFFSET>,
            SetHoldRadius::<Impl, OFFSET>,
            HoldStartDelay::<Impl, OFFSET>,
            SetHoldStartDelay::<Impl, OFFSET>,
            TranslationMinContactCount::<Impl, OFFSET>,
            SetTranslationMinContactCount::<Impl, OFFSET>,
            TranslationMaxContactCount::<Impl, OFFSET>,
            SetTranslationMaxContactCount::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn HoldingState(&self) -> ::windows::core::Result<HoldingState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IHoldingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHoldingEventArgsVtbl {
    pub const fn new<Impl: IHoldingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHoldingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IHoldingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IHoldingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingState<Impl: IHoldingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HoldingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHoldingEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, HoldingState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHoldingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IHoldingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IHoldingEventArgs2Vtbl {
    pub const fn new<Impl: IHoldingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHoldingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IHoldingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IHoldingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHoldingEventArgs2>, base.5, ContactCount::<Impl, OFFSET>, CurrentContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerImpl: Sized {
    fn State(&self) -> ::windows::core::Result<InputActivationState>;
    fn InputActivationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputActivationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.IInputActivationListener";
}
#[cfg(feature = "implement_exclusive")]
impl IInputActivationListenerVtbl {
    pub const fn new<Impl: IInputActivationListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputActivationListenerVtbl {
        unsafe extern "system" fn State<Impl: IInputActivationListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputActivationChanged<Impl: IInputActivationListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputActivationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputActivationChanged<Impl: IInputActivationListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInputActivationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputActivationListener>, base.5, State::<Impl, OFFSET>, InputActivationChanged::<Impl, OFFSET>, RemoveInputActivationChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerActivationChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<InputActivationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInputActivationListenerActivationChangedEventArgsVtbl {
    pub const fn new<Impl: IInputActivationListenerActivationChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputActivationListenerActivationChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IInputActivationListenerActivationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputActivationListenerActivationChangedEventArgs>, base.5, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardDeliveryInterceptorImpl: Sized {
    fn IsInterceptionEnabledWhenInForeground(&self) -> ::windows::core::Result<bool>;
    fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.IKeyboardDeliveryInterceptor";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardDeliveryInterceptorVtbl {
    pub const fn new<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardDeliveryInterceptorVtbl {
        unsafe extern "system" fn IsInterceptionEnabledWhenInForeground<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInterceptionEnabledWhenInForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterceptionEnabledWhenInForeground<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsInterceptionEnabledWhenInForeground(value).into()
        }
        unsafe extern "system" fn KeyDown<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyUp<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardDeliveryInterceptor>, base.5, IsInterceptionEnabledWhenInForeground::<Impl, OFFSET>, SetIsInterceptionEnabledWhenInForeground::<Impl, OFFSET>, KeyDown::<Impl, OFFSET>, RemoveKeyDown::<Impl, OFFSET>, KeyUp::<Impl, OFFSET>, RemoveKeyUp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardDeliveryInterceptorStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<KeyboardDeliveryInterceptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardDeliveryInterceptorStatics {
    const NAME: &'static str = "Windows.UI.Input.IKeyboardDeliveryInterceptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardDeliveryInterceptorStaticsVtbl {
    pub const fn new<Impl: IKeyboardDeliveryInterceptorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardDeliveryInterceptorStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IKeyboardDeliveryInterceptorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardDeliveryInterceptorStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationCompletedEventArgsVtbl {
    pub const fn new<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationCompletedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationCompletedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Velocities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationCompletedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationCompletedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationCompletedEventArgs2Vtbl {
    pub const fn new<Impl: IManipulationCompletedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationCompletedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationCompletedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IManipulationCompletedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationCompletedEventArgs2>, base.5, ContactCount::<Impl, OFFSET>, CurrentContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationInertiaStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationInertiaStartingEventArgsVtbl {
    pub const fn new<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationInertiaStartingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationInertiaStartingEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, Delta::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Velocities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationInertiaStartingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationInertiaStartingEventArgs2Vtbl {
    pub const fn new<Impl: IManipulationInertiaStartingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationInertiaStartingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationInertiaStartingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationInertiaStartingEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedEventArgsVtbl {
    pub const fn new<Impl: IManipulationStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationStartedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationStartedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationStartedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedEventArgs2Vtbl {
    pub const fn new<Impl: IManipulationStartedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationStartedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationStartedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationStartedEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationUpdatedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationUpdatedEventArgsVtbl {
    pub const fn new<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationUpdatedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationUpdatedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, Delta::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Velocities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationUpdatedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationUpdatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationUpdatedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationUpdatedEventArgs2Vtbl {
    pub const fn new<Impl: IManipulationUpdatedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationUpdatedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationUpdatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IManipulationUpdatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationUpdatedEventArgs2>, base.5, ContactCount::<Impl, OFFSET>, CurrentContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseWheelParametersImpl: Sized {
    fn CharTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetCharTranslation(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn DeltaScale(&self) -> ::windows::core::Result<f32>;
    fn SetDeltaScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn DeltaRotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn PageTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPageTranslation(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.IMouseWheelParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IMouseWheelParametersVtbl {
    pub const fn new<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMouseWheelParametersVtbl {
        unsafe extern "system" fn CharTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharTranslation(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeltaScale<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaScale<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeltaScale(value).into()
        }
        unsafe extern "system" fn DeltaRotationAngle<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaRotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaRotationAngle<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeltaRotationAngle(value).into()
        }
        unsafe extern "system" fn PageTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPageTranslation(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMouseWheelParameters>, base.5, CharTranslation::<Impl, OFFSET>, SetCharTranslation::<Impl, OFFSET>, DeltaScale::<Impl, OFFSET>, SetDeltaScale::<Impl, OFFSET>, DeltaRotationAngle::<Impl, OFFSET>, SetDeltaRotationAngle::<Impl, OFFSET>, PageTranslation::<Impl, OFFSET>, SetPageTranslation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointImpl: Sized {
    fn PointerDevice(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDevice>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn RawPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn FrameId(&self) -> ::windows::core::Result<u32>;
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsInContact(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<PointerPointProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerPoint {
    const NAME: &'static str = "Windows.UI.Input.IPointerPoint";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerPointVtbl {
    pub const fn new<Impl: IPointerPointImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerPointVtbl {
        unsafe extern "system" fn PointerDevice<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawPosition<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerId<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInContact<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInContact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPointerPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerPoint>, base.5, PointerDevice::<Impl, OFFSET>, Position::<Impl, OFFSET>, RawPosition::<Impl, OFFSET>, PointerId::<Impl, OFFSET>, FrameId::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, IsInContact::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointPropertiesImpl: Sized {
    fn Pressure(&self) -> ::windows::core::Result<f32>;
    fn IsInverted(&self) -> ::windows::core::Result<bool>;
    fn IsEraser(&self) -> ::windows::core::Result<bool>;
    fn Orientation(&self) -> ::windows::core::Result<f32>;
    fn XTilt(&self) -> ::windows::core::Result<f32>;
    fn YTilt(&self) -> ::windows::core::Result<f32>;
    fn Twist(&self) -> ::windows::core::Result<f32>;
    fn ContactRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ContactRectRaw(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TouchConfidence(&self) -> ::windows::core::Result<bool>;
    fn IsLeftButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsRightButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsMiddleButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn MouseWheelDelta(&self) -> ::windows::core::Result<i32>;
    fn IsHorizontalMouseWheel(&self) -> ::windows::core::Result<bool>;
    fn IsPrimary(&self) -> ::windows::core::Result<bool>;
    fn IsInRange(&self) -> ::windows::core::Result<bool>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn IsBarrelButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsXButton1Pressed(&self) -> ::windows::core::Result<bool>;
    fn IsXButton2Pressed(&self) -> ::windows::core::Result<bool>;
    fn PointerUpdateKind(&self) -> ::windows::core::Result<PointerUpdateKind>;
    fn HasUsage(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<bool>;
    fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerPointPropertiesVtbl {
    pub const fn new<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerPointPropertiesVtbl {
        unsafe extern "system" fn Pressure<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInverted<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInverted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEraser<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEraser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XTilt<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XTilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YTilt<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).YTilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Twist<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Twist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRect<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRectRaw<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactRectRaw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchConfidence<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TouchConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeftButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLeftButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRightButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRightButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMiddleButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMiddleButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseWheelDelta<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MouseWheelDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHorizontalMouseWheel<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHorizontalMouseWheel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrimary<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRange<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceled<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCanceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBarrelButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBarrelButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsXButton1Pressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsXButton1Pressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsXButton2Pressed<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsXButton2Pressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerUpdateKind<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PointerUpdateKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerUpdateKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasUsage<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasUsage(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsageValue<Impl: IPointerPointPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUsageValue(usagepage, usageid) {
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
            ::windows::core::GetRuntimeClassName::<IPointerPointProperties>,
            base.5,
            Pressure::<Impl, OFFSET>,
            IsInverted::<Impl, OFFSET>,
            IsEraser::<Impl, OFFSET>,
            Orientation::<Impl, OFFSET>,
            XTilt::<Impl, OFFSET>,
            YTilt::<Impl, OFFSET>,
            Twist::<Impl, OFFSET>,
            ContactRect::<Impl, OFFSET>,
            ContactRectRaw::<Impl, OFFSET>,
            TouchConfidence::<Impl, OFFSET>,
            IsLeftButtonPressed::<Impl, OFFSET>,
            IsRightButtonPressed::<Impl, OFFSET>,
            IsMiddleButtonPressed::<Impl, OFFSET>,
            MouseWheelDelta::<Impl, OFFSET>,
            IsHorizontalMouseWheel::<Impl, OFFSET>,
            IsPrimary::<Impl, OFFSET>,
            IsInRange::<Impl, OFFSET>,
            IsCanceled::<Impl, OFFSET>,
            IsBarrelButtonPressed::<Impl, OFFSET>,
            IsXButton1Pressed::<Impl, OFFSET>,
            IsXButton2Pressed::<Impl, OFFSET>,
            PointerUpdateKind::<Impl, OFFSET>,
            HasUsage::<Impl, OFFSET>,
            GetUsageValue::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointProperties2Impl: Sized {
    fn ZDistance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerPointProperties2 {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerPointProperties2Vtbl {
    pub const fn new<Impl: IPointerPointProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerPointProperties2Vtbl {
        unsafe extern "system" fn ZDistance<Impl: IPointerPointProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerPointProperties2>, base.5, ZDistance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointStaticsImpl: Sized {
    fn GetCurrentPoint(&self, pointerid: u32) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePoints(&self, pointerid: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
    fn GetCurrentPointTransformed(&self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePointsTransformed(&self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerPointStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerPointStaticsVtbl {
    pub const fn new<Impl: IPointerPointStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerPointStaticsVtbl {
        unsafe extern "system" fn GetCurrentPoint<Impl: IPointerPointStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPoint(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerPointStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePoints(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPointTransformed<Impl: IPointerPointStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPointTransformed(pointerid, &*(&transform as *const <IPointerPointTransform as ::windows::core::Abi>::Abi as *const <IPointerPointTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePointsTransformed<Impl: IPointerPointStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePointsTransformed(pointerid, &*(&transform as *const <IPointerPointTransform as ::windows::core::Abi>::Abi as *const <IPointerPointTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerPointStatics>, base.5, GetCurrentPoint::<Impl, OFFSET>, GetIntermediatePoints::<Impl, OFFSET>, GetCurrentPointTransformed::<Impl, OFFSET>, GetIntermediatePointsTransformed::<Impl, OFFSET>)
    }
}
pub trait IPointerPointTransformImpl: Sized {
    fn Inverse(&self) -> ::windows::core::Result<IPointerPointTransform>;
    fn TryTransform(&self, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&self, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
impl ::windows::core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointTransform";
}
impl IPointerPointTransformVtbl {
    pub const fn new<Impl: IPointerPointTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerPointTransformVtbl {
        unsafe extern "system" fn Inverse<Impl: IPointerPointTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Impl: IPointerPointTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTransform(&*(&inpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Impl: IPointerPointTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformBounds(&*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerPointTransform>, base.5, Inverse::<Impl, OFFSET>, TryTransform::<Impl, OFFSET>, TransformBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsImpl: Sized {
    fn SetIsContactFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsContactFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsBarrelButtonFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.IPointerVisualizationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerVisualizationSettingsVtbl {
    pub const fn new<Impl: IPointerVisualizationSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerVisualizationSettingsVtbl {
        unsafe extern "system" fn SetIsContactFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsContactFeedbackEnabled(value).into()
        }
        unsafe extern "system" fn IsContactFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContactFeedbackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBarrelButtonFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsBarrelButtonFeedbackEnabled(value).into()
        }
        unsafe extern "system" fn IsBarrelButtonFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBarrelButtonFeedbackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerVisualizationSettings>, base.5, SetIsContactFeedbackEnabled::<Impl, OFFSET>, IsContactFeedbackEnabled::<Impl, OFFSET>, SetIsBarrelButtonFeedbackEnabled::<Impl, OFFSET>, IsBarrelButtonFeedbackEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<PointerVisualizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerVisualizationSettingsStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerVisualizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerVisualizationSettingsStaticsVtbl {
    pub const fn new<Impl: IPointerVisualizationSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerVisualizationSettingsStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPointerVisualizationSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerVisualizationSettingsStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerImpl: Sized {
    fn Menu(&self) -> ::windows::core::Result<RadialControllerMenu>;
    fn RotationResolutionInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetRotationResolutionInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn UseAutomaticHapticFeedback(&self) -> ::windows::core::Result<bool>;
    fn SetUseAutomaticHapticFeedback(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScreenContactStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactStarted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactEnded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactContinued(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactContinued(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RotationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRotationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlAcquired(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlAcquired(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialController {
    const NAME: &'static str = "Windows.UI.Input.IRadialController";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerVtbl {
    pub const fn new<Impl: IRadialControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerVtbl {
        unsafe extern "system" fn Menu<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Menu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationResolutionInDegrees<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationResolutionInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationResolutionInDegrees<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationResolutionInDegrees(value).into()
        }
        unsafe extern "system" fn UseAutomaticHapticFeedback<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseAutomaticHapticFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAutomaticHapticFeedback<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUseAutomaticHapticFeedback(value).into()
        }
        unsafe extern "system" fn ScreenContactStarted<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenContactStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactStarted<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactStarted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenContactEnded<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenContactEnded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactEnded<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactEnded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenContactContinued<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenContactContinued(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactContinued<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactContinued(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlLost<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveControlLost<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveControlLost(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationChanged<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRotationChanged<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRotationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonClicked<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonClicked<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveButtonClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlAcquired<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlAcquired(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveControlAcquired<Impl: IRadialControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveControlAcquired(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IRadialController>,
            base.5,
            Menu::<Impl, OFFSET>,
            RotationResolutionInDegrees::<Impl, OFFSET>,
            SetRotationResolutionInDegrees::<Impl, OFFSET>,
            UseAutomaticHapticFeedback::<Impl, OFFSET>,
            SetUseAutomaticHapticFeedback::<Impl, OFFSET>,
            ScreenContactStarted::<Impl, OFFSET>,
            RemoveScreenContactStarted::<Impl, OFFSET>,
            ScreenContactEnded::<Impl, OFFSET>,
            RemoveScreenContactEnded::<Impl, OFFSET>,
            ScreenContactContinued::<Impl, OFFSET>,
            RemoveScreenContactContinued::<Impl, OFFSET>,
            ControlLost::<Impl, OFFSET>,
            RemoveControlLost::<Impl, OFFSET>,
            RotationChanged::<Impl, OFFSET>,
            RemoveRotationChanged::<Impl, OFFSET>,
            ButtonClicked::<Impl, OFFSET>,
            RemoveButtonClicked::<Impl, OFFSET>,
            ControlAcquired::<Impl, OFFSET>,
            RemoveControlAcquired::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialController2Impl: Sized {
    fn ButtonPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonHolding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialController2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialController2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialController2Vtbl {
    pub const fn new<Impl: IRadialController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialController2Vtbl {
        unsafe extern "system" fn ButtonPressed<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonPressed<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveButtonPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHolding<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonHolding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonHolding<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveButtonHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonReleased<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonReleased<Impl: IRadialController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveButtonReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialController2>, base.5, ButtonPressed::<Impl, OFFSET>, RemoveButtonPressed::<Impl, OFFSET>, ButtonHolding::<Impl, OFFSET>, RemoveButtonHolding::<Impl, OFFSET>, ButtonReleased::<Impl, OFFSET>, RemoveButtonReleased::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonClickedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonClickedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerButtonClickedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerButtonClickedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonClickedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerButtonClickedEventArgs>, base.5, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonClickedEventArgs2Impl: Sized {
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonClickedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonClickedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonClickedEventArgs2Vtbl {
    pub const fn new<Impl: IRadialControllerButtonClickedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerButtonClickedEventArgs2Vtbl {
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonClickedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerButtonClickedEventArgs2>, base.5, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonHoldingEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonHoldingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonHoldingEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerButtonHoldingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerButtonHoldingEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonHoldingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonHoldingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerButtonHoldingEventArgs>, base.5, Contact::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonPressedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonPressedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerButtonPressedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerButtonPressedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerButtonPressedEventArgs>, base.5, Contact::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonReleasedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonReleasedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonReleasedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerButtonReleasedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerButtonReleasedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonReleasedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonReleasedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerButtonReleasedEventArgs>, base.5, Contact::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationImpl: Sized {
    fn SetDefaultMenuItems(&self, buttons: &::core::option::Option<super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>) -> ::windows::core::Result<()>;
    fn ResetToDefaultMenuItems(&self) -> ::windows::core::Result<()>;
    fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfigurationVtbl {
    pub const fn new<Impl: IRadialControllerConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerConfigurationVtbl {
        unsafe extern "system" fn SetDefaultMenuItems<Impl: IRadialControllerConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buttons: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultMenuItems(&*(&buttons as *const <super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResetToDefaultMenuItems<Impl: IRadialControllerConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetToDefaultMenuItems().into()
        }
        unsafe extern "system" fn TrySelectDefaultMenuItem<Impl: IRadialControllerConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySelectDefaultMenuItem(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerConfiguration>, base.5, SetDefaultMenuItems::<Impl, OFFSET>, ResetToDefaultMenuItems::<Impl, OFFSET>, TrySelectDefaultMenuItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfiguration2Impl: Sized {
    fn SetActiveControllerWhenMenuIsSuppressed(&self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn ActiveControllerWhenMenuIsSuppressed(&self) -> ::windows::core::Result<RadialController>;
    fn SetIsMenuSuppressed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMenuSuppressed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfiguration2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfiguration2Vtbl {
    pub const fn new<Impl: IRadialControllerConfiguration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerConfiguration2Vtbl {
        unsafe extern "system" fn SetActiveControllerWhenMenuIsSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetActiveControllerWhenMenuIsSuppressed(&*(&value as *const <RadialController as ::windows::core::Abi>::Abi as *const <RadialController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActiveControllerWhenMenuIsSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActiveControllerWhenMenuIsSuppressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMenuSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsMenuSuppressed(value).into()
        }
        unsafe extern "system" fn IsMenuSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMenuSuppressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerConfiguration2>, base.5, SetActiveControllerWhenMenuIsSuppressed::<Impl, OFFSET>, ActiveControllerWhenMenuIsSuppressed::<Impl, OFFSET>, SetIsMenuSuppressed::<Impl, OFFSET>, IsMenuSuppressed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<RadialControllerConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfigurationStaticsVtbl {
    pub const fn new<Impl: IRadialControllerConfigurationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerConfigurationStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IRadialControllerConfigurationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerConfigurationStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStatics2Impl: Sized {
    fn SetAppController(&self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn AppController(&self) -> ::windows::core::Result<RadialController>;
    fn SetIsAppControllerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppControllerEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationStatics2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfigurationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfigurationStatics2Vtbl {
    pub const fn new<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerConfigurationStatics2Vtbl {
        unsafe extern "system" fn SetAppController<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppController(&*(&value as *const <RadialController as ::windows::core::Abi>::Abi as *const <RadialController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppController<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAppControllerEnabled<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAppControllerEnabled(value).into()
        }
        unsafe extern "system" fn IsAppControllerEnabled<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAppControllerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerConfigurationStatics2>, base.5, SetAppController::<Impl, OFFSET>, AppController::<Impl, OFFSET>, SetIsAppControllerEnabled::<Impl, OFFSET>, IsAppControllerEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerControlAcquiredEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerControlAcquiredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerControlAcquiredEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerControlAcquiredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerControlAcquiredEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerControlAcquiredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerControlAcquiredEventArgs>, base.5, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerControlAcquiredEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerControlAcquiredEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerControlAcquiredEventArgs2Vtbl {
    pub const fn new<Impl: IRadialControllerControlAcquiredEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerControlAcquiredEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerControlAcquiredEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerControlAcquiredEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerControlAcquiredEventArgs2>, base.5, IsButtonPressed::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetSelectedMenuItem(&self) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn SelectMenuItem(&self, menuitem: &::core::option::Option<RadialControllerMenuItem>) -> ::windows::core::Result<()>;
    fn TrySelectPreviouslySelectedMenuItem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenu";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerMenuVtbl {
    pub const fn new<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerMenuVtbl {
        unsafe extern "system" fn Items<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn GetSelectedMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSelectedMenuItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SelectMenuItem(&*(&menuitem as *const <RadialControllerMenuItem as ::windows::core::Abi>::Abi as *const <RadialControllerMenuItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrySelectPreviouslySelectedMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySelectPreviouslySelectedMenuItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerMenu>, base.5, Items::<Impl, OFFSET>, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, GetSelectedMenuItem::<Impl, OFFSET>, SelectMenuItem::<Impl, OFFSET>, TrySelectPreviouslySelectedMenuItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItem";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerMenuItemVtbl {
    pub const fn new<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerMenuItemVtbl {
        unsafe extern "system" fn DisplayText<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invoked<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvoked<Impl: IRadialControllerMenuItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerMenuItem>, base.5, DisplayText::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetTag::<Impl, OFFSET>, Invoked::<Impl, OFFSET>, RemoveInvoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemStaticsImpl: Sized {
    fn CreateFromIcon(&self, displaytext: &::windows::core::HSTRING, icon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromKnownIcon(&self, displaytext: &::windows::core::HSTRING, value: RadialControllerMenuKnownIcon) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerMenuItemStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerMenuItemStaticsVtbl {
    pub const fn new<Impl: IRadialControllerMenuItemStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerMenuItemStaticsVtbl {
        unsafe extern "system" fn CreateFromIcon<Impl: IRadialControllerMenuItemStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, icon: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromIcon(&*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&icon as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromKnownIcon<Impl: IRadialControllerMenuItemStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: RadialControllerMenuKnownIcon, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromKnownIcon(&*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerMenuItemStatics>, base.5, CreateFromIcon::<Impl, OFFSET>, CreateFromKnownIcon::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemStatics2Impl: Sized {
    fn CreateFromFontGlyph(&self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromFontGlyphWithUri(&self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING, fonturi: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerMenuItemStatics2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItemStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerMenuItemStatics2Vtbl {
    pub const fn new<Impl: IRadialControllerMenuItemStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerMenuItemStatics2Vtbl {
        unsafe extern "system" fn CreateFromFontGlyph<Impl: IRadialControllerMenuItemStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromFontGlyph(
                &*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&glyph as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&fontfamily as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromFontGlyphWithUri<Impl: IRadialControllerMenuItemStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fonturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromFontGlyphWithUri(
                &*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&glyph as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&fontfamily as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&fonturi as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerMenuItemStatics2>, base.5, CreateFromFontGlyph::<Impl, OFFSET>, CreateFromFontGlyphWithUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerRotationChangedEventArgsImpl: Sized {
    fn RotationDeltaInDegrees(&self) -> ::windows::core::Result<f64>;
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerRotationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerRotationChangedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerRotationChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerRotationChangedEventArgsVtbl {
        unsafe extern "system" fn RotationDeltaInDegrees<Impl: IRadialControllerRotationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationDeltaInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IRadialControllerRotationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerRotationChangedEventArgs>, base.5, RotationDeltaInDegrees::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerRotationChangedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerRotationChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerRotationChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerRotationChangedEventArgs2Vtbl {
    pub const fn new<Impl: IRadialControllerRotationChangedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerRotationChangedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerRotationChangedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerRotationChangedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerRotationChangedEventArgs2>, base.5, IsButtonPressed::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContact";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactVtbl {
    pub const fn new<Impl: IRadialControllerScreenContactImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactVtbl {
        unsafe extern "system" fn Bounds<Impl: IRadialControllerScreenContactImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRadialControllerScreenContactImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContact>, base.5, Bounds::<Impl, OFFSET>, Position::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactContinuedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactContinuedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerScreenContactContinuedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactContinuedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerScreenContactContinuedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContactContinuedEventArgs>, base.5, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactContinuedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactContinuedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactContinuedEventArgs2Vtbl {
    pub const fn new<Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactContinuedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContactContinuedEventArgs2>, base.5, IsButtonPressed::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactEndedEventArgsImpl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactEndedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerScreenContactEndedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactEndedEventArgsVtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactEndedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactEndedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContactEndedEventArgs>, base.5, IsButtonPressed::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactStartedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactStartedEventArgsVtbl {
    pub const fn new<Impl: IRadialControllerScreenContactStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactStartedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerScreenContactStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContactStartedEventArgs>, base.5, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactStartedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactStartedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactStartedEventArgs2Vtbl {
    pub const fn new<Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerScreenContactStartedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerScreenContactStartedEventArgs2>, base.5, IsButtonPressed::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn CreateForCurrentView(&self) -> ::windows::core::Result<RadialController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerStaticsVtbl {
    pub const fn new<Impl: IRadialControllerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IRadialControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCurrentView<Impl: IRadialControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerStatics>, base.5, IsSupported::<Impl, OFFSET>, CreateForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRightTappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRightTappedEventArgsVtbl {
    pub const fn new<Impl: IRightTappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRightTappedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRightTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRightTappedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRightTappedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRightTappedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRightTappedEventArgs2Vtbl {
    pub const fn new<Impl: IRightTappedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRightTappedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IRightTappedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRightTappedEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemButtonEventControllerImpl: Sized {
    fn SystemFunctionButtonPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionButtonReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockIndicatorChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockIndicatorChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.ISystemButtonEventController";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemButtonEventControllerVtbl {
    pub const fn new<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemButtonEventControllerVtbl {
        unsafe extern "system" fn SystemFunctionButtonPressed<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemFunctionButtonPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionButtonPressed<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionButtonPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionButtonReleased<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemFunctionButtonReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionButtonReleased<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionButtonReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionLockChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemFunctionLockChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionLockChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionLockChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionLockIndicatorChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemFunctionLockIndicatorChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionLockIndicatorChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionLockIndicatorChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISystemButtonEventController>,
            base.5,
            SystemFunctionButtonPressed::<Impl, OFFSET>,
            RemoveSystemFunctionButtonPressed::<Impl, OFFSET>,
            SystemFunctionButtonReleased::<Impl, OFFSET>,
            RemoveSystemFunctionButtonReleased::<Impl, OFFSET>,
            SystemFunctionLockChanged::<Impl, OFFSET>,
            RemoveSystemFunctionLockChanged::<Impl, OFFSET>,
            SystemFunctionLockIndicatorChanged::<Impl, OFFSET>,
            RemoveSystemFunctionLockIndicatorChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemButtonEventControllerStaticsImpl: Sized {
    fn CreateForDispatcherQueue(&self, queue: &::core::option::Option<super::super::System::DispatcherQueue>) -> ::windows::core::Result<SystemButtonEventController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemButtonEventControllerStatics {
    const NAME: &'static str = "Windows.UI.Input.ISystemButtonEventControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemButtonEventControllerStaticsVtbl {
    pub const fn new<Impl: ISystemButtonEventControllerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemButtonEventControllerStaticsVtbl {
        unsafe extern "system" fn CreateForDispatcherQueue<Impl: ISystemButtonEventControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForDispatcherQueue(&*(&queue as *const <super::super::System::DispatcherQueue as ::windows::core::Abi>::Abi as *const <super::super::System::DispatcherQueue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemButtonEventControllerStatics>, base.5, CreateForDispatcherQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionButtonEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionButtonEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionButtonEventArgsVtbl {
    pub const fn new<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemFunctionButtonEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemFunctionButtonEventArgs>, base.5, Timestamp::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsLocked(&self) -> ::windows::core::Result<bool>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionLockChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionLockChangedEventArgsVtbl {
    pub const fn new<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemFunctionLockChangedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocked<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemFunctionLockChangedEventArgs>, base.5, Timestamp::<Impl, OFFSET>, IsLocked::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockIndicatorChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsIndicatorOn(&self) -> ::windows::core::Result<bool>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionLockIndicatorChangedEventArgsVtbl {
    pub const fn new<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemFunctionLockIndicatorChangedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIndicatorOn<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIndicatorOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemFunctionLockIndicatorChangedEventArgs>, base.5, Timestamp::<Impl, OFFSET>, IsIndicatorOn::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn TapCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ITappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITappedEventArgsVtbl {
    pub const fn new<Impl: ITappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITappedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ITappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ITappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapCount<Impl: ITappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TapCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITappedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Position::<Impl, OFFSET>, TapCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITappedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.ITappedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ITappedEventArgs2Vtbl {
    pub const fn new<Impl: ITappedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITappedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: ITappedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITappedEventArgs2>, base.5, ContactCount::<Impl, OFFSET>)
    }
}
