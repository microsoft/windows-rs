#[cfg(feature = "implement_exclusive")]
pub trait IAttachableInputObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.IAttachableInputObject";
}
#[cfg(feature = "implement_exclusive")]
impl IAttachableInputObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttachableInputObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAttachableInputObjectVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAttachableInputObject, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAttachableInputObject as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttachableInputObjectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAttachableInputObjectFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAttachableInputObjectFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAttachableInputObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICrossSlidingEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn CrossSlidingState(&mut self) -> ::windows::core::Result<CrossSlidingState>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ICrossSlidingEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ICrossSlidingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrossSlidingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrossSlidingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ICrossSlidingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ICrossSlidingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CrossSlidingState<Impl: ICrossSlidingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CrossSlidingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrossSlidingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICrossSlidingEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            CrossSlidingState: CrossSlidingState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrossSlidingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICrossSlidingEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICrossSlidingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.ICrossSlidingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ICrossSlidingEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrossSlidingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrossSlidingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: ICrossSlidingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICrossSlidingEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrossSlidingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDraggingEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn DraggingState(&mut self) -> ::windows::core::Result<DraggingState>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IDraggingEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IDraggingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDraggingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDraggingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IDraggingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IDraggingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DraggingState<Impl: IDraggingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DraggingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DraggingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDraggingEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            DraggingState: DraggingState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDraggingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDraggingEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDraggingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IDraggingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDraggingEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDraggingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDraggingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IDraggingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDraggingEventArgs2, BASE_OFFSET>(), ContactCount: ContactCount::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDraggingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEdgeGestureImpl: Sized {
    fn Starting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Canceled(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGesture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEdgeGestureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeGestureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeGestureVtbl {
        unsafe extern "system" fn Starting<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Starting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStarting<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Canceled<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IEdgeGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEdgeGesture, BASE_OFFSET>(),
            Starting: Starting::<Impl, IMPL_OFFSET>,
            RemoveStarting: RemoveStarting::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
            RemoveCanceled: RemoveCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeGesture as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureEventArgsImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<EdgeGestureKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGestureEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeGestureEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeGestureEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeGestureEventArgsVtbl {
        unsafe extern "system" fn Kind<Impl: IEdgeGestureEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EdgeGestureKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IEdgeGestureEventArgs, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeGestureEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<EdgeGesture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeGestureStatics {
    const NAME: &'static str = "Windows.UI.Input.IEdgeGestureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeGestureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeGestureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeGestureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IEdgeGestureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEdgeGestureStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeGestureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGestureRecognizerImpl: Sized {
    fn GestureSettings(&mut self) -> ::windows::core::Result<GestureSettings>;
    fn SetGestureSettings(&mut self, value: GestureSettings) -> ::windows::core::Result<()>;
    fn IsInertial(&mut self) -> ::windows::core::Result<bool>;
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn ShowGestureFeedback(&mut self) -> ::windows::core::Result<bool>;
    fn SetShowGestureFeedback(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PivotCenter(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPivotCenter(&mut self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn PivotRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetPivotRadius(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDeceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationDeceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansionDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansionDeceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDisplacement(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDisplacement(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansion(&mut self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansion(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn ManipulationExact(&mut self) -> ::windows::core::Result<bool>;
    fn SetManipulationExact(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideThresholds(&mut self) -> ::windows::core::Result<CrossSlideThresholds>;
    fn SetCrossSlideThresholds(&mut self, value: &CrossSlideThresholds) -> ::windows::core::Result<()>;
    fn CrossSlideHorizontally(&mut self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideHorizontally(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideExact(&mut self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideExact(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoProcessInertia(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoProcessInertia(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MouseWheelParameters(&mut self) -> ::windows::core::Result<MouseWheelParameters>;
    fn CanBeDoubleTap(&mut self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<bool>;
    fn ProcessDownEvent(&mut self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMoveEvents(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<PointerPoint>>) -> ::windows::core::Result<()>;
    fn ProcessUpEvent(&mut self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMouseWheelEvent(&mut self, value: &::core::option::Option<PointerPoint>, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::Result<()>;
    fn ProcessInertia(&mut self) -> ::windows::core::Result<()>;
    fn CompleteGesture(&mut self) -> ::windows::core::Result<()>;
    fn Tapped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Dragging(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragging(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CrossSliding(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCrossSliding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.IGestureRecognizer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGestureRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGestureRecognizerVtbl {
        unsafe extern "system" fn GestureSettings<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GestureSettings) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GestureSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureSettings<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GestureSettings) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGestureSettings(value).into()
        }
        unsafe extern "system" fn IsInertial<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowGestureFeedback<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowGestureFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowGestureFeedback<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowGestureFeedback(value).into()
        }
        unsafe extern "system" fn PivotCenter<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotCenter<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivotCenter(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PivotRadius<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivotRadius(value).into()
        }
        unsafe extern "system" fn InertiaTranslationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaTranslationDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaTranslationDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaRotationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaRotationDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaRotationDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaRotationDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaExpansionDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaExpansionDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaExpansionDeceleration<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaExpansionDeceleration(value).into()
        }
        unsafe extern "system" fn InertiaTranslationDisplacement<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaTranslationDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDisplacement<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaTranslationDisplacement(value).into()
        }
        unsafe extern "system" fn InertiaRotationAngle<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaRotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaRotationAngle<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaRotationAngle(value).into()
        }
        unsafe extern "system" fn InertiaExpansion<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InertiaExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInertiaExpansion<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInertiaExpansion(value).into()
        }
        unsafe extern "system" fn ManipulationExact<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationExact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationExact<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManipulationExact(value).into()
        }
        unsafe extern "system" fn CrossSlideThresholds<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CrossSlideThresholds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrossSlideThresholds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideThresholds<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CrossSlideThresholds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCrossSlideThresholds(&*(&value as *const <CrossSlideThresholds as ::windows::core::Abi>::Abi as *const <CrossSlideThresholds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CrossSlideHorizontally<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrossSlideHorizontally() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideHorizontally<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCrossSlideHorizontally(value).into()
        }
        unsafe extern "system" fn CrossSlideExact<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrossSlideExact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrossSlideExact<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCrossSlideExact(value).into()
        }
        unsafe extern "system" fn AutoProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoProcessInertia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoProcessInertia(value).into()
        }
        unsafe extern "system" fn MouseWheelParameters<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseWheelParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBeDoubleTap<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBeDoubleTap(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessDownEvent<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessDownEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessMoveEvents<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessMoveEvents(&*(&value as *const <super::super::Foundation::Collections::IVector<PointerPoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<PointerPoint> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessUpEvent<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUpEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessMouseWheelEvent<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessMouseWheelEvent(&*(&value as *const <PointerPoint as ::windows::core::Abi>::Abi as *const <PointerPoint as ::windows::core::DefaultType>::DefaultType), isshiftkeydown, iscontrolkeydown).into()
        }
        unsafe extern "system" fn ProcessInertia<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessInertia().into()
        }
        unsafe extern "system" fn CompleteGesture<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteGesture().into()
        }
        unsafe extern "system" fn Tapped<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RightTapped<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightTapped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRightTapped<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRightTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Holding<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Holding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHolding<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Dragging<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dragging(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragging<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragging(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationUpdated<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationUpdated<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationInertiaStarting<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationInertiaStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CrossSliding<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrossSliding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCrossSliding<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCrossSliding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGestureRecognizer, BASE_OFFSET>(),
            GestureSettings: GestureSettings::<Impl, IMPL_OFFSET>,
            SetGestureSettings: SetGestureSettings::<Impl, IMPL_OFFSET>,
            IsInertial: IsInertial::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            ShowGestureFeedback: ShowGestureFeedback::<Impl, IMPL_OFFSET>,
            SetShowGestureFeedback: SetShowGestureFeedback::<Impl, IMPL_OFFSET>,
            PivotCenter: PivotCenter::<Impl, IMPL_OFFSET>,
            SetPivotCenter: SetPivotCenter::<Impl, IMPL_OFFSET>,
            PivotRadius: PivotRadius::<Impl, IMPL_OFFSET>,
            SetPivotRadius: SetPivotRadius::<Impl, IMPL_OFFSET>,
            InertiaTranslationDeceleration: InertiaTranslationDeceleration::<Impl, IMPL_OFFSET>,
            SetInertiaTranslationDeceleration: SetInertiaTranslationDeceleration::<Impl, IMPL_OFFSET>,
            InertiaRotationDeceleration: InertiaRotationDeceleration::<Impl, IMPL_OFFSET>,
            SetInertiaRotationDeceleration: SetInertiaRotationDeceleration::<Impl, IMPL_OFFSET>,
            InertiaExpansionDeceleration: InertiaExpansionDeceleration::<Impl, IMPL_OFFSET>,
            SetInertiaExpansionDeceleration: SetInertiaExpansionDeceleration::<Impl, IMPL_OFFSET>,
            InertiaTranslationDisplacement: InertiaTranslationDisplacement::<Impl, IMPL_OFFSET>,
            SetInertiaTranslationDisplacement: SetInertiaTranslationDisplacement::<Impl, IMPL_OFFSET>,
            InertiaRotationAngle: InertiaRotationAngle::<Impl, IMPL_OFFSET>,
            SetInertiaRotationAngle: SetInertiaRotationAngle::<Impl, IMPL_OFFSET>,
            InertiaExpansion: InertiaExpansion::<Impl, IMPL_OFFSET>,
            SetInertiaExpansion: SetInertiaExpansion::<Impl, IMPL_OFFSET>,
            ManipulationExact: ManipulationExact::<Impl, IMPL_OFFSET>,
            SetManipulationExact: SetManipulationExact::<Impl, IMPL_OFFSET>,
            CrossSlideThresholds: CrossSlideThresholds::<Impl, IMPL_OFFSET>,
            SetCrossSlideThresholds: SetCrossSlideThresholds::<Impl, IMPL_OFFSET>,
            CrossSlideHorizontally: CrossSlideHorizontally::<Impl, IMPL_OFFSET>,
            SetCrossSlideHorizontally: SetCrossSlideHorizontally::<Impl, IMPL_OFFSET>,
            CrossSlideExact: CrossSlideExact::<Impl, IMPL_OFFSET>,
            SetCrossSlideExact: SetCrossSlideExact::<Impl, IMPL_OFFSET>,
            AutoProcessInertia: AutoProcessInertia::<Impl, IMPL_OFFSET>,
            SetAutoProcessInertia: SetAutoProcessInertia::<Impl, IMPL_OFFSET>,
            MouseWheelParameters: MouseWheelParameters::<Impl, IMPL_OFFSET>,
            CanBeDoubleTap: CanBeDoubleTap::<Impl, IMPL_OFFSET>,
            ProcessDownEvent: ProcessDownEvent::<Impl, IMPL_OFFSET>,
            ProcessMoveEvents: ProcessMoveEvents::<Impl, IMPL_OFFSET>,
            ProcessUpEvent: ProcessUpEvent::<Impl, IMPL_OFFSET>,
            ProcessMouseWheelEvent: ProcessMouseWheelEvent::<Impl, IMPL_OFFSET>,
            ProcessInertia: ProcessInertia::<Impl, IMPL_OFFSET>,
            CompleteGesture: CompleteGesture::<Impl, IMPL_OFFSET>,
            Tapped: Tapped::<Impl, IMPL_OFFSET>,
            RemoveTapped: RemoveTapped::<Impl, IMPL_OFFSET>,
            RightTapped: RightTapped::<Impl, IMPL_OFFSET>,
            RemoveRightTapped: RemoveRightTapped::<Impl, IMPL_OFFSET>,
            Holding: Holding::<Impl, IMPL_OFFSET>,
            RemoveHolding: RemoveHolding::<Impl, IMPL_OFFSET>,
            Dragging: Dragging::<Impl, IMPL_OFFSET>,
            RemoveDragging: RemoveDragging::<Impl, IMPL_OFFSET>,
            ManipulationStarted: ManipulationStarted::<Impl, IMPL_OFFSET>,
            RemoveManipulationStarted: RemoveManipulationStarted::<Impl, IMPL_OFFSET>,
            ManipulationUpdated: ManipulationUpdated::<Impl, IMPL_OFFSET>,
            RemoveManipulationUpdated: RemoveManipulationUpdated::<Impl, IMPL_OFFSET>,
            ManipulationInertiaStarting: ManipulationInertiaStarting::<Impl, IMPL_OFFSET>,
            RemoveManipulationInertiaStarting: RemoveManipulationInertiaStarting::<Impl, IMPL_OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Impl, IMPL_OFFSET>,
            RemoveManipulationCompleted: RemoveManipulationCompleted::<Impl, IMPL_OFFSET>,
            CrossSliding: CrossSliding::<Impl, IMPL_OFFSET>,
            RemoveCrossSliding: RemoveCrossSliding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGestureRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGestureRecognizer2Impl: Sized {
    fn TapMinContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetTapMinContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn TapMaxContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetTapMaxContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMinContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetHoldMinContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMaxContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetHoldMaxContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HoldRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetHoldRadius(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn HoldStartDelay(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetHoldStartDelay(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TranslationMinContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetTranslationMinContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn TranslationMaxContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetTranslationMaxContactCount(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGestureRecognizer2 {
    const NAME: &'static str = "Windows.UI.Input.IGestureRecognizer2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGestureRecognizer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGestureRecognizer2Vtbl {
        unsafe extern "system" fn TapMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTapMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTapMinContactCount(value).into()
        }
        unsafe extern "system" fn TapMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTapMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTapMaxContactCount(value).into()
        }
        unsafe extern "system" fn HoldMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoldMinContactCount(value).into()
        }
        unsafe extern "system" fn HoldMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoldMaxContactCount(value).into()
        }
        unsafe extern "system" fn HoldRadius<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldRadius<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoldRadius(value).into()
        }
        unsafe extern "system" fn HoldStartDelay<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldStartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoldStartDelay<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoldStartDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslationMinContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationMinContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslationMinContactCount(value).into()
        }
        unsafe extern "system" fn TranslationMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslationMaxContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationMaxContactCount<Impl: IGestureRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslationMaxContactCount(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGestureRecognizer2, BASE_OFFSET>(),
            TapMinContactCount: TapMinContactCount::<Impl, IMPL_OFFSET>,
            SetTapMinContactCount: SetTapMinContactCount::<Impl, IMPL_OFFSET>,
            TapMaxContactCount: TapMaxContactCount::<Impl, IMPL_OFFSET>,
            SetTapMaxContactCount: SetTapMaxContactCount::<Impl, IMPL_OFFSET>,
            HoldMinContactCount: HoldMinContactCount::<Impl, IMPL_OFFSET>,
            SetHoldMinContactCount: SetHoldMinContactCount::<Impl, IMPL_OFFSET>,
            HoldMaxContactCount: HoldMaxContactCount::<Impl, IMPL_OFFSET>,
            SetHoldMaxContactCount: SetHoldMaxContactCount::<Impl, IMPL_OFFSET>,
            HoldRadius: HoldRadius::<Impl, IMPL_OFFSET>,
            SetHoldRadius: SetHoldRadius::<Impl, IMPL_OFFSET>,
            HoldStartDelay: HoldStartDelay::<Impl, IMPL_OFFSET>,
            SetHoldStartDelay: SetHoldStartDelay::<Impl, IMPL_OFFSET>,
            TranslationMinContactCount: TranslationMinContactCount::<Impl, IMPL_OFFSET>,
            SetTranslationMinContactCount: SetTranslationMinContactCount::<Impl, IMPL_OFFSET>,
            TranslationMaxContactCount: TranslationMaxContactCount::<Impl, IMPL_OFFSET>,
            SetTranslationMaxContactCount: SetTranslationMaxContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGestureRecognizer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHoldingEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn HoldingState(&mut self) -> ::windows::core::Result<HoldingState>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IHoldingEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IHoldingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHoldingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHoldingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IHoldingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IHoldingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingState<Impl: IHoldingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HoldingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHoldingEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            HoldingState: HoldingState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHoldingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHoldingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IHoldingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IHoldingEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHoldingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHoldingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IHoldingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IHoldingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHoldingEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
            CurrentContactCount: CurrentContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHoldingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInputActivationListenerImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<InputActivationState>;
    fn InputActivationChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputActivationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.IInputActivationListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInputActivationListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputActivationListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputActivationListenerVtbl {
        unsafe extern "system" fn State<Impl: IInputActivationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputActivationChanged<Impl: IInputActivationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputActivationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputActivationChanged<Impl: IInputActivationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputActivationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputActivationListener, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            InputActivationChanged: InputActivationChanged::<Impl, IMPL_OFFSET>,
            RemoveInputActivationChanged: RemoveInputActivationChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputActivationListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerActivationChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<InputActivationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IInputActivationListenerActivationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInputActivationListenerActivationChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputActivationListenerActivationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputActivationListenerActivationChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IInputActivationListenerActivationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputActivationListenerActivationChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputActivationListenerActivationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IKeyboardDeliveryInterceptorImpl: Sized {
    fn IsInterceptionEnabledWhenInForeground(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsInterceptionEnabledWhenInForeground(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeyDown(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.IKeyboardDeliveryInterceptor";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IKeyboardDeliveryInterceptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardDeliveryInterceptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardDeliveryInterceptorVtbl {
        unsafe extern "system" fn IsInterceptionEnabledWhenInForeground<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterceptionEnabledWhenInForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterceptionEnabledWhenInForeground<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInterceptionEnabledWhenInForeground(value).into()
        }
        unsafe extern "system" fn KeyDown<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyUp<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: IKeyboardDeliveryInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardDeliveryInterceptor, BASE_OFFSET>(),
            IsInterceptionEnabledWhenInForeground: IsInterceptionEnabledWhenInForeground::<Impl, IMPL_OFFSET>,
            SetIsInterceptionEnabledWhenInForeground: SetIsInterceptionEnabledWhenInForeground::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardDeliveryInterceptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardDeliveryInterceptorStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<KeyboardDeliveryInterceptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardDeliveryInterceptorStatics {
    const NAME: &'static str = "Windows.UI.Input.IKeyboardDeliveryInterceptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardDeliveryInterceptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardDeliveryInterceptorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardDeliveryInterceptorStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IKeyboardDeliveryInterceptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardDeliveryInterceptorStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardDeliveryInterceptorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationCompletedEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&mut self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationCompletedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationCompletedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationCompletedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationCompletedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationCompletedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationCompletedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationCompletedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationCompletedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IManipulationCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationCompletedEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
            CurrentContactCount: CurrentContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationCompletedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationInertiaStartingEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&mut self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&mut self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationInertiaStartingEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationInertiaStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationInertiaStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationInertiaStartingEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationInertiaStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationInertiaStartingEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Delta: Delta::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationInertiaStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationInertiaStartingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationInertiaStartingEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationInertiaStartingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationInertiaStartingEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationInertiaStartingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationInertiaStartingEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationInertiaStartingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationStartedEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&mut self) -> ::windows::core::Result<ManipulationDelta>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationStartedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationStartedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationStartedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationStartedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationStartedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationStartedEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationStartedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationUpdatedEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&mut self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&mut self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IManipulationUpdatedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationUpdatedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationUpdatedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Delta: Delta::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationUpdatedEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationUpdatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IManipulationUpdatedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationUpdatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationUpdatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationUpdatedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IManipulationUpdatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentContactCount<Impl: IManipulationUpdatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationUpdatedEventArgs2, BASE_OFFSET>(),
            ContactCount: ContactCount::<Impl, IMPL_OFFSET>,
            CurrentContactCount: CurrentContactCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationUpdatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMouseWheelParametersImpl: Sized {
    fn CharTranslation(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetCharTranslation(&mut self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn DeltaScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetDeltaScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn DeltaRotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetDeltaRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn PageTranslation(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPageTranslation(&mut self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.IMouseWheelParameters";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMouseWheelParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMouseWheelParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMouseWheelParametersVtbl {
        unsafe extern "system" fn CharTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharTranslation(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeltaScale<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaScale<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaScale(value).into()
        }
        unsafe extern "system" fn DeltaRotationAngle<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaRotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeltaRotationAngle<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeltaRotationAngle(value).into()
        }
        unsafe extern "system" fn PageTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageTranslation<Impl: IMouseWheelParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageTranslation(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMouseWheelParameters, BASE_OFFSET>(),
            CharTranslation: CharTranslation::<Impl, IMPL_OFFSET>,
            SetCharTranslation: SetCharTranslation::<Impl, IMPL_OFFSET>,
            DeltaScale: DeltaScale::<Impl, IMPL_OFFSET>,
            SetDeltaScale: SetDeltaScale::<Impl, IMPL_OFFSET>,
            DeltaRotationAngle: DeltaRotationAngle::<Impl, IMPL_OFFSET>,
            SetDeltaRotationAngle: SetDeltaRotationAngle::<Impl, IMPL_OFFSET>,
            PageTranslation: PageTranslation::<Impl, IMPL_OFFSET>,
            SetPageTranslation: SetPageTranslation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMouseWheelParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointerPointImpl: Sized {
    fn PointerDevice(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDevice>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn RawPosition(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn PointerId(&mut self) -> ::windows::core::Result<u32>;
    fn FrameId(&mut self) -> ::windows::core::Result<u32>;
    fn Timestamp(&mut self) -> ::windows::core::Result<u64>;
    fn IsInContact(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<PointerPointProperties>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerPoint {
    const NAME: &'static str = "Windows.UI.Input.IPointerPoint";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IPointerPointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointVtbl {
        unsafe extern "system" fn PointerDevice<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawPosition<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerId<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInContact<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInContact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPointerPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPoint, BASE_OFFSET>(),
            PointerDevice: PointerDevice::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            RawPosition: RawPosition::<Impl, IMPL_OFFSET>,
            PointerId: PointerId::<Impl, IMPL_OFFSET>,
            FrameId: FrameId::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            IsInContact: IsInContact::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPoint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointerPointPropertiesImpl: Sized {
    fn Pressure(&mut self) -> ::windows::core::Result<f32>;
    fn IsInverted(&mut self) -> ::windows::core::Result<bool>;
    fn IsEraser(&mut self) -> ::windows::core::Result<bool>;
    fn Orientation(&mut self) -> ::windows::core::Result<f32>;
    fn XTilt(&mut self) -> ::windows::core::Result<f32>;
    fn YTilt(&mut self) -> ::windows::core::Result<f32>;
    fn Twist(&mut self) -> ::windows::core::Result<f32>;
    fn ContactRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ContactRectRaw(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TouchConfidence(&mut self) -> ::windows::core::Result<bool>;
    fn IsLeftButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsRightButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsMiddleButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn MouseWheelDelta(&mut self) -> ::windows::core::Result<i32>;
    fn IsHorizontalMouseWheel(&mut self) -> ::windows::core::Result<bool>;
    fn IsPrimary(&mut self) -> ::windows::core::Result<bool>;
    fn IsInRange(&mut self) -> ::windows::core::Result<bool>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn IsBarrelButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsXButton1Pressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsXButton2Pressed(&mut self) -> ::windows::core::Result<bool>;
    fn PointerUpdateKind(&mut self) -> ::windows::core::Result<PointerUpdateKind>;
    fn HasUsage(&mut self, usagepage: u32, usageid: u32) -> ::windows::core::Result<bool>;
    fn GetUsageValue(&mut self, usagepage: u32, usageid: u32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointerPointPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointPropertiesVtbl {
        unsafe extern "system" fn Pressure<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInverted<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInverted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEraser<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEraser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn XTilt<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XTilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YTilt<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YTilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Twist<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Twist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRect<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRectRaw<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactRectRaw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchConfidence<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeftButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLeftButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRightButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRightButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMiddleButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMiddleButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseWheelDelta<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseWheelDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHorizontalMouseWheel<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHorizontalMouseWheel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrimary<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRange<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceled<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCanceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBarrelButtonPressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBarrelButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsXButton1Pressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsXButton1Pressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsXButton2Pressed<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsXButton2Pressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerUpdateKind<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PointerUpdateKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerUpdateKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasUsage<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasUsage(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsageValue<Impl: IPointerPointPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsageValue(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointProperties, BASE_OFFSET>(),
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
            IsInverted: IsInverted::<Impl, IMPL_OFFSET>,
            IsEraser: IsEraser::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            XTilt: XTilt::<Impl, IMPL_OFFSET>,
            YTilt: YTilt::<Impl, IMPL_OFFSET>,
            Twist: Twist::<Impl, IMPL_OFFSET>,
            ContactRect: ContactRect::<Impl, IMPL_OFFSET>,
            ContactRectRaw: ContactRectRaw::<Impl, IMPL_OFFSET>,
            TouchConfidence: TouchConfidence::<Impl, IMPL_OFFSET>,
            IsLeftButtonPressed: IsLeftButtonPressed::<Impl, IMPL_OFFSET>,
            IsRightButtonPressed: IsRightButtonPressed::<Impl, IMPL_OFFSET>,
            IsMiddleButtonPressed: IsMiddleButtonPressed::<Impl, IMPL_OFFSET>,
            MouseWheelDelta: MouseWheelDelta::<Impl, IMPL_OFFSET>,
            IsHorizontalMouseWheel: IsHorizontalMouseWheel::<Impl, IMPL_OFFSET>,
            IsPrimary: IsPrimary::<Impl, IMPL_OFFSET>,
            IsInRange: IsInRange::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            IsBarrelButtonPressed: IsBarrelButtonPressed::<Impl, IMPL_OFFSET>,
            IsXButton1Pressed: IsXButton1Pressed::<Impl, IMPL_OFFSET>,
            IsXButton2Pressed: IsXButton2Pressed::<Impl, IMPL_OFFSET>,
            PointerUpdateKind: PointerUpdateKind::<Impl, IMPL_OFFSET>,
            HasUsage: HasUsage::<Impl, IMPL_OFFSET>,
            GetUsageValue: GetUsageValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointerPointProperties2Impl: Sized {
    fn ZDistance(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerPointProperties2 {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointProperties2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointerPointProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointProperties2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointProperties2Vtbl {
        unsafe extern "system" fn ZDistance<Impl: IPointerPointProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointProperties2, BASE_OFFSET>(), ZDistance: ZDistance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPointerPointStaticsImpl: Sized {
    fn GetCurrentPoint(&mut self, pointerid: u32) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePoints(&mut self, pointerid: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
    fn GetCurrentPointTransformed(&mut self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePointsTransformed(&mut self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerPointStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPointerPointStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointStaticsVtbl {
        unsafe extern "system" fn GetCurrentPoint<Impl: IPointerPointStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPoint(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerPointStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePoints(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPointTransformed<Impl: IPointerPointStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPointTransformed(pointerid, &*(&transform as *const <IPointerPointTransform as ::windows::core::Abi>::Abi as *const <IPointerPointTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePointsTransformed<Impl: IPointerPointStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePointsTransformed(pointerid, &*(&transform as *const <IPointerPointTransform as ::windows::core::Abi>::Abi as *const <IPointerPointTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointStatics, BASE_OFFSET>(),
            GetCurrentPoint: GetCurrentPoint::<Impl, IMPL_OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Impl, IMPL_OFFSET>,
            GetCurrentPointTransformed: GetCurrentPointTransformed::<Impl, IMPL_OFFSET>,
            GetIntermediatePointsTransformed: GetIntermediatePointsTransformed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPointerPointTransformImpl: Sized {
    fn Inverse(&mut self) -> ::windows::core::Result<IPointerPointTransform>;
    fn TryTransform(&mut self, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&mut self, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointTransform";
}
#[cfg(feature = "Foundation")]
impl IPointerPointTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointTransformVtbl {
        unsafe extern "system" fn Inverse<Impl: IPointerPointTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Impl: IPointerPointTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTransform(&*(&inpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Impl: IPointerPointTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformBounds(&*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointTransform, BASE_OFFSET>(),
            Inverse: Inverse::<Impl, IMPL_OFFSET>,
            TryTransform: TryTransform::<Impl, IMPL_OFFSET>,
            TransformBounds: TransformBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsImpl: Sized {
    fn SetIsContactFeedbackEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsContactFeedbackEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsBarrelButtonFeedbackEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsBarrelButtonFeedbackEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.IPointerVisualizationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerVisualizationSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerVisualizationSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerVisualizationSettingsVtbl {
        unsafe extern "system" fn SetIsContactFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsContactFeedbackEnabled(value).into()
        }
        unsafe extern "system" fn IsContactFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContactFeedbackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBarrelButtonFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBarrelButtonFeedbackEnabled(value).into()
        }
        unsafe extern "system" fn IsBarrelButtonFeedbackEnabled<Impl: IPointerVisualizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBarrelButtonFeedbackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerVisualizationSettings, BASE_OFFSET>(),
            SetIsContactFeedbackEnabled: SetIsContactFeedbackEnabled::<Impl, IMPL_OFFSET>,
            IsContactFeedbackEnabled: IsContactFeedbackEnabled::<Impl, IMPL_OFFSET>,
            SetIsBarrelButtonFeedbackEnabled: SetIsBarrelButtonFeedbackEnabled::<Impl, IMPL_OFFSET>,
            IsBarrelButtonFeedbackEnabled: IsBarrelButtonFeedbackEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerVisualizationSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<PointerVisualizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerVisualizationSettingsStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerVisualizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerVisualizationSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerVisualizationSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerVisualizationSettingsStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPointerVisualizationSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerVisualizationSettingsStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerVisualizationSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRadialControllerImpl: Sized {
    fn Menu(&mut self) -> ::windows::core::Result<RadialControllerMenu>;
    fn RotationResolutionInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotationResolutionInDegrees(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn UseAutomaticHapticFeedback(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseAutomaticHapticFeedback(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ScreenContactStarted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactStarted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactEnded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactEnded(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactContinued(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactContinued(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlLost(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlLost(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RotationChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRotationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonClicked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonClicked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlAcquired(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlAcquired(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialController {
    const NAME: &'static str = "Windows.UI.Input.IRadialController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRadialControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerVtbl {
        unsafe extern "system" fn Menu<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Menu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationResolutionInDegrees<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationResolutionInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationResolutionInDegrees<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationResolutionInDegrees(value).into()
        }
        unsafe extern "system" fn UseAutomaticHapticFeedback<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseAutomaticHapticFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAutomaticHapticFeedback<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseAutomaticHapticFeedback(value).into()
        }
        unsafe extern "system" fn ScreenContactStarted<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenContactStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactStarted<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactStarted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenContactEnded<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenContactEnded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactEnded<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactEnded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenContactContinued<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenContactContinued(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScreenContactContinued<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScreenContactContinued(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlLost<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveControlLost<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveControlLost(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationChanged<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRotationChanged<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRotationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonClicked<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonClicked<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlAcquired<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlAcquired(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveControlAcquired<Impl: IRadialControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveControlAcquired(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialController, BASE_OFFSET>(),
            Menu: Menu::<Impl, IMPL_OFFSET>,
            RotationResolutionInDegrees: RotationResolutionInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationResolutionInDegrees: SetRotationResolutionInDegrees::<Impl, IMPL_OFFSET>,
            UseAutomaticHapticFeedback: UseAutomaticHapticFeedback::<Impl, IMPL_OFFSET>,
            SetUseAutomaticHapticFeedback: SetUseAutomaticHapticFeedback::<Impl, IMPL_OFFSET>,
            ScreenContactStarted: ScreenContactStarted::<Impl, IMPL_OFFSET>,
            RemoveScreenContactStarted: RemoveScreenContactStarted::<Impl, IMPL_OFFSET>,
            ScreenContactEnded: ScreenContactEnded::<Impl, IMPL_OFFSET>,
            RemoveScreenContactEnded: RemoveScreenContactEnded::<Impl, IMPL_OFFSET>,
            ScreenContactContinued: ScreenContactContinued::<Impl, IMPL_OFFSET>,
            RemoveScreenContactContinued: RemoveScreenContactContinued::<Impl, IMPL_OFFSET>,
            ControlLost: ControlLost::<Impl, IMPL_OFFSET>,
            RemoveControlLost: RemoveControlLost::<Impl, IMPL_OFFSET>,
            RotationChanged: RotationChanged::<Impl, IMPL_OFFSET>,
            RemoveRotationChanged: RemoveRotationChanged::<Impl, IMPL_OFFSET>,
            ButtonClicked: ButtonClicked::<Impl, IMPL_OFFSET>,
            RemoveButtonClicked: RemoveButtonClicked::<Impl, IMPL_OFFSET>,
            ControlAcquired: ControlAcquired::<Impl, IMPL_OFFSET>,
            RemoveControlAcquired: RemoveControlAcquired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRadialController2Impl: Sized {
    fn ButtonPressed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonHolding(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonHolding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonReleased(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonReleased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialController2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialController2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRadialController2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialController2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialController2Vtbl {
        unsafe extern "system" fn ButtonPressed<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonPressed<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHolding<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonHolding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonHolding<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonReleased<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonReleased<Impl: IRadialController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialController2, BASE_OFFSET>(),
            ButtonPressed: ButtonPressed::<Impl, IMPL_OFFSET>,
            RemoveButtonPressed: RemoveButtonPressed::<Impl, IMPL_OFFSET>,
            ButtonHolding: ButtonHolding::<Impl, IMPL_OFFSET>,
            RemoveButtonHolding: RemoveButtonHolding::<Impl, IMPL_OFFSET>,
            ButtonReleased: ButtonReleased::<Impl, IMPL_OFFSET>,
            RemoveButtonReleased: RemoveButtonReleased::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonClickedEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerButtonClickedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerButtonClickedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerButtonClickedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonClickedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerButtonClickedEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerButtonClickedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerButtonClickedEventArgs2Impl: Sized {
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerButtonClickedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonClickedEventArgs2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerButtonClickedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerButtonClickedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerButtonClickedEventArgs2Vtbl {
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonClickedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerButtonClickedEventArgs2, BASE_OFFSET>(),
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerButtonClickedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerButtonHoldingEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonHoldingEventArgs";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerButtonHoldingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerButtonHoldingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerButtonHoldingEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonHoldingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonHoldingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerButtonHoldingEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerButtonHoldingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerButtonPressedEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonPressedEventArgs";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerButtonPressedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerButtonPressedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerButtonPressedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerButtonPressedEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerButtonPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerButtonReleasedEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerButtonReleasedEventArgs";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerButtonReleasedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerButtonReleasedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerButtonReleasedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerButtonReleasedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerButtonReleasedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerButtonReleasedEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerButtonReleasedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRadialControllerConfigurationImpl: Sized {
    fn SetDefaultMenuItems(&mut self, buttons: &::core::option::Option<super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>) -> ::windows::core::Result<()>;
    fn ResetToDefaultMenuItems(&mut self) -> ::windows::core::Result<()>;
    fn TrySelectDefaultMenuItem(&mut self, r#type: RadialControllerSystemMenuItemKind) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRadialControllerConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerConfigurationVtbl {
        unsafe extern "system" fn SetDefaultMenuItems<Impl: IRadialControllerConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttons: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultMenuItems(&*(&buttons as *const <super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResetToDefaultMenuItems<Impl: IRadialControllerConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetToDefaultMenuItems().into()
        }
        unsafe extern "system" fn TrySelectDefaultMenuItem<Impl: IRadialControllerConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySelectDefaultMenuItem(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerConfiguration, BASE_OFFSET>(),
            SetDefaultMenuItems: SetDefaultMenuItems::<Impl, IMPL_OFFSET>,
            ResetToDefaultMenuItems: ResetToDefaultMenuItems::<Impl, IMPL_OFFSET>,
            TrySelectDefaultMenuItem: TrySelectDefaultMenuItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfiguration2Impl: Sized {
    fn SetActiveControllerWhenMenuIsSuppressed(&mut self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn ActiveControllerWhenMenuIsSuppressed(&mut self) -> ::windows::core::Result<RadialController>;
    fn SetIsMenuSuppressed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsMenuSuppressed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfiguration2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerConfiguration2Vtbl {
        unsafe extern "system" fn SetActiveControllerWhenMenuIsSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveControllerWhenMenuIsSuppressed(&*(&value as *const <RadialController as ::windows::core::Abi>::Abi as *const <RadialController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActiveControllerWhenMenuIsSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveControllerWhenMenuIsSuppressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMenuSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMenuSuppressed(value).into()
        }
        unsafe extern "system" fn IsMenuSuppressed<Impl: IRadialControllerConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMenuSuppressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerConfiguration2, BASE_OFFSET>(),
            SetActiveControllerWhenMenuIsSuppressed: SetActiveControllerWhenMenuIsSuppressed::<Impl, IMPL_OFFSET>,
            ActiveControllerWhenMenuIsSuppressed: ActiveControllerWhenMenuIsSuppressed::<Impl, IMPL_OFFSET>,
            SetIsMenuSuppressed: SetIsMenuSuppressed::<Impl, IMPL_OFFSET>,
            IsMenuSuppressed: IsMenuSuppressed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<RadialControllerConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfigurationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerConfigurationStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IRadialControllerConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerConfigurationStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStatics2Impl: Sized {
    fn SetAppController(&mut self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn AppController(&mut self) -> ::windows::core::Result<RadialController>;
    fn SetIsAppControllerEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppControllerEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationStatics2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerConfigurationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerConfigurationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerConfigurationStatics2Vtbl {
        unsafe extern "system" fn SetAppController<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppController(&*(&value as *const <RadialController as ::windows::core::Abi>::Abi as *const <RadialController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppController<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAppControllerEnabled<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAppControllerEnabled(value).into()
        }
        unsafe extern "system" fn IsAppControllerEnabled<Impl: IRadialControllerConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppControllerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerConfigurationStatics2, BASE_OFFSET>(),
            SetAppController: SetAppController::<Impl, IMPL_OFFSET>,
            AppController: AppController::<Impl, IMPL_OFFSET>,
            SetIsAppControllerEnabled: SetIsAppControllerEnabled::<Impl, IMPL_OFFSET>,
            IsAppControllerEnabled: IsAppControllerEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerControlAcquiredEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerControlAcquiredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerControlAcquiredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerControlAcquiredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerControlAcquiredEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerControlAcquiredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerControlAcquiredEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerControlAcquiredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerControlAcquiredEventArgs2Impl: Sized {
    fn IsButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerControlAcquiredEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerControlAcquiredEventArgs2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerControlAcquiredEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerControlAcquiredEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerControlAcquiredEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerControlAcquiredEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerControlAcquiredEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerControlAcquiredEventArgs2, BASE_OFFSET>(),
            IsButtonPressed: IsButtonPressed::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerControlAcquiredEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRadialControllerMenuImpl: Sized {
    fn Items(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetSelectedMenuItem(&mut self) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn SelectMenuItem(&mut self, menuitem: &::core::option::Option<RadialControllerMenuItem>) -> ::windows::core::Result<()>;
    fn TrySelectPreviouslySelectedMenuItem(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenu";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRadialControllerMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerMenuVtbl {
        unsafe extern "system" fn Items<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn GetSelectedMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectedMenuItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectMenuItem(&*(&menuitem as *const <RadialControllerMenuItem as ::windows::core::Abi>::Abi as *const <RadialControllerMenuItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrySelectPreviouslySelectedMenuItem<Impl: IRadialControllerMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySelectPreviouslySelectedMenuItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerMenu, BASE_OFFSET>(),
            Items: Items::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            GetSelectedMenuItem: GetSelectedMenuItem::<Impl, IMPL_OFFSET>,
            SelectMenuItem: SelectMenuItem::<Impl, IMPL_OFFSET>,
            TrySelectPreviouslySelectedMenuItem: TrySelectPreviouslySelectedMenuItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRadialControllerMenuItemImpl: Sized {
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Invoked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRadialControllerMenuItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerMenuItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerMenuItemVtbl {
        unsafe extern "system" fn DisplayText<Impl: IRadialControllerMenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IRadialControllerMenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IRadialControllerMenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invoked<Impl: IRadialControllerMenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvoked<Impl: IRadialControllerMenuItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerMenuItem, BASE_OFFSET>(),
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Invoked: Invoked::<Impl, IMPL_OFFSET>,
            RemoveInvoked: RemoveInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerMenuItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRadialControllerMenuItemStaticsImpl: Sized {
    fn CreateFromIcon(&mut self, displaytext: &::windows::core::HSTRING, icon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromKnownIcon(&mut self, displaytext: &::windows::core::HSTRING, value: RadialControllerMenuKnownIcon) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerMenuItemStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItemStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRadialControllerMenuItemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerMenuItemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerMenuItemStaticsVtbl {
        unsafe extern "system" fn CreateFromIcon<Impl: IRadialControllerMenuItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, icon: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIcon(&*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&icon as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromKnownIcon<Impl: IRadialControllerMenuItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: RadialControllerMenuKnownIcon, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromKnownIcon(&*(&displaytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerMenuItemStatics, BASE_OFFSET>(),
            CreateFromIcon: CreateFromIcon::<Impl, IMPL_OFFSET>,
            CreateFromKnownIcon: CreateFromKnownIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerMenuItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRadialControllerMenuItemStatics2Impl: Sized {
    fn CreateFromFontGlyph(&mut self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromFontGlyphWithUri(&mut self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING, fonturi: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerMenuItemStatics2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerMenuItemStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRadialControllerMenuItemStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerMenuItemStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerMenuItemStatics2Vtbl {
        unsafe extern "system" fn CreateFromFontGlyph<Impl: IRadialControllerMenuItemStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateFromFontGlyphWithUri<Impl: IRadialControllerMenuItemStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fonturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerMenuItemStatics2, BASE_OFFSET>(),
            CreateFromFontGlyph: CreateFromFontGlyph::<Impl, IMPL_OFFSET>,
            CreateFromFontGlyphWithUri: CreateFromFontGlyphWithUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerMenuItemStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerRotationChangedEventArgsImpl: Sized {
    fn RotationDeltaInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerRotationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerRotationChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerRotationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerRotationChangedEventArgsVtbl {
        unsafe extern "system" fn RotationDeltaInDegrees<Impl: IRadialControllerRotationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationDeltaInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Impl: IRadialControllerRotationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerRotationChangedEventArgs, BASE_OFFSET>(),
            RotationDeltaInDegrees: RotationDeltaInDegrees::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerRotationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerRotationChangedEventArgs2Impl: Sized {
    fn IsButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerRotationChangedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerRotationChangedEventArgs2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerRotationChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerRotationChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerRotationChangedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerRotationChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerRotationChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerRotationChangedEventArgs2, BASE_OFFSET>(),
            IsButtonPressed: IsButtonPressed::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerRotationChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRadialControllerScreenContactImpl: Sized {
    fn Bounds(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContact";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRadialControllerScreenContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactVtbl {
        unsafe extern "system" fn Bounds<Impl: IRadialControllerScreenContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRadialControllerScreenContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContact, BASE_OFFSET>(),
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContact as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactContinuedEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactContinuedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactContinuedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactContinuedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerScreenContactContinuedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContactContinuedEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContactContinuedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerScreenContactContinuedEventArgs2Impl: Sized {
    fn IsButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactContinuedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactContinuedEventArgs2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerScreenContactContinuedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactContinuedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactContinuedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContactContinuedEventArgs2, BASE_OFFSET>(),
            IsButtonPressed: IsButtonPressed::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContactContinuedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerScreenContactEndedEventArgsImpl: Sized {
    fn IsButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactEndedEventArgs";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerScreenContactEndedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactEndedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactEndedEventArgsVtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactEndedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactEndedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContactEndedEventArgs, BASE_OFFSET>(),
            IsButtonPressed: IsButtonPressed::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContactEndedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactStartedEventArgsImpl: Sized {
    fn Contact(&mut self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerScreenContactStartedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactStartedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactStartedEventArgsVtbl {
        unsafe extern "system" fn Contact<Impl: IRadialControllerScreenContactStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContactStartedEventArgs, BASE_OFFSET>(),
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContactStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait IRadialControllerScreenContactStartedEventArgs2Impl: Sized {
    fn IsButtonPressed(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerScreenContactStartedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerScreenContactStartedEventArgs2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl IRadialControllerScreenContactStartedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerScreenContactStartedEventArgs2Vtbl {
        unsafe extern "system" fn IsButtonPressed<Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsButtonPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: IRadialControllerScreenContactStartedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerScreenContactStartedEventArgs2, BASE_OFFSET>(),
            IsButtonPressed: IsButtonPressed::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerScreenContactStartedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerStaticsImpl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn CreateForCurrentView(&mut self) -> ::windows::core::Result<RadialController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerStatics {
    const NAME: &'static str = "Windows.UI.Input.IRadialControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IRadialControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCurrentView<Impl: IRadialControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            CreateForCurrentView: CreateForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRightTappedEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.IRightTappedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IRightTappedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRightTappedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRightTappedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRightTappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRightTappedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRightTappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRightTappedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.IRightTappedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IRightTappedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRightTappedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRightTappedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: IRightTappedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRightTappedEventArgs2, BASE_OFFSET>(), ContactCount: ContactCount::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRightTappedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemButtonEventControllerImpl: Sized {
    fn SystemFunctionButtonPressed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionButtonReleased(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonReleased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockIndicatorChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockIndicatorChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.ISystemButtonEventController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemButtonEventControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemButtonEventControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemButtonEventControllerVtbl {
        unsafe extern "system" fn SystemFunctionButtonPressed<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFunctionButtonPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionButtonPressed<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionButtonPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionButtonReleased<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFunctionButtonReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionButtonReleased<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionButtonReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionLockChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFunctionLockChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionLockChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionLockChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemFunctionLockIndicatorChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFunctionLockIndicatorChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemFunctionLockIndicatorChanged<Impl: ISystemButtonEventControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemFunctionLockIndicatorChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemButtonEventController, BASE_OFFSET>(),
            SystemFunctionButtonPressed: SystemFunctionButtonPressed::<Impl, IMPL_OFFSET>,
            RemoveSystemFunctionButtonPressed: RemoveSystemFunctionButtonPressed::<Impl, IMPL_OFFSET>,
            SystemFunctionButtonReleased: SystemFunctionButtonReleased::<Impl, IMPL_OFFSET>,
            RemoveSystemFunctionButtonReleased: RemoveSystemFunctionButtonReleased::<Impl, IMPL_OFFSET>,
            SystemFunctionLockChanged: SystemFunctionLockChanged::<Impl, IMPL_OFFSET>,
            RemoveSystemFunctionLockChanged: RemoveSystemFunctionLockChanged::<Impl, IMPL_OFFSET>,
            SystemFunctionLockIndicatorChanged: SystemFunctionLockIndicatorChanged::<Impl, IMPL_OFFSET>,
            RemoveSystemFunctionLockIndicatorChanged: RemoveSystemFunctionLockIndicatorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemButtonEventController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ISystemButtonEventControllerStaticsImpl: Sized {
    fn CreateForDispatcherQueue(&mut self, queue: &::core::option::Option<super::super::System::DispatcherQueue>) -> ::windows::core::Result<SystemButtonEventController>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemButtonEventControllerStatics {
    const NAME: &'static str = "Windows.UI.Input.ISystemButtonEventControllerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ISystemButtonEventControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemButtonEventControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemButtonEventControllerStaticsVtbl {
        unsafe extern "system" fn CreateForDispatcherQueue<Impl: ISystemButtonEventControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForDispatcherQueue(&*(&queue as *const <super::super::System::DispatcherQueue as ::windows::core::Abi>::Abi as *const <super::super::System::DispatcherQueue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemButtonEventControllerStatics, BASE_OFFSET>(),
            CreateForDispatcherQueue: CreateForDispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemButtonEventControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionButtonEventArgsImpl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<u64>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionButtonEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionButtonEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemFunctionButtonEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemFunctionButtonEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionButtonEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemFunctionButtonEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemFunctionButtonEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockChangedEventArgsImpl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<u64>;
    fn IsLocked(&mut self) -> ::windows::core::Result<bool>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionLockChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionLockChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemFunctionLockChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemFunctionLockChangedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocked<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionLockChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemFunctionLockChangedEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            IsLocked: IsLocked::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemFunctionLockChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockIndicatorChangedEventArgsImpl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<u64>;
    fn IsIndicatorOn(&mut self) -> ::windows::core::Result<bool>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ISystemFunctionLockIndicatorChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemFunctionLockIndicatorChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemFunctionLockIndicatorChangedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIndicatorOn<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIndicatorOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemFunctionLockIndicatorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemFunctionLockIndicatorChangedEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            IsIndicatorOn: IsIndicatorOn::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemFunctionLockIndicatorChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITappedEventArgsImpl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn TapCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ITappedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ITappedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITappedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITappedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ITappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ITappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapCount<Impl: ITappedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITappedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            TapCount: TapCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITappedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedEventArgs2Impl: Sized {
    fn ContactCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITappedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.ITappedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ITappedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITappedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITappedEventArgs2Vtbl {
        unsafe extern "system" fn ContactCount<Impl: ITappedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITappedEventArgs2, BASE_OFFSET>(), ContactCount: ContactCount::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITappedEventArgs2 as ::windows::core::Interface>::IID
    }
}
