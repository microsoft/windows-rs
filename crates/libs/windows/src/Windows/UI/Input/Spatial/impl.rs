#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGestureRecognizerImpl: Sized {
    fn RecognitionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecognitionEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionEnded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureInteraction(&self, interaction: &::core::option::Option<SpatialInteraction>) -> ::windows::core::Result<()>;
    fn CancelPendingGestures(&self) -> ::windows::core::Result<()>;
    fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> ::windows::core::Result<bool>;
    fn GestureSettings(&self) -> ::windows::core::Result<SpatialGestureSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialGestureRecognizer";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGestureRecognizerVtbl {
    pub const fn new<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialGestureRecognizerVtbl {
        unsafe extern "system" fn RecognitionStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognitionStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRecognitionStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognitionEnded<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognitionEnded(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionEnded<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRecognitionEnded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tapped<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHoldStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHoldCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHoldCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationUpdated<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationUpdated<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNavigationStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationUpdated<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationUpdated<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNavigationUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNavigationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCanceled<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNavigationCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureInteraction<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CaptureInteraction(&*(&interaction as *const <SpatialInteraction as ::windows::core::Abi>::Abi as *const <SpatialInteraction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelPendingGestures<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CancelPendingGestures().into()
        }
        unsafe extern "system" fn TrySetGestureSettings<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetGestureSettings(settings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GestureSettings<Impl: ISpatialGestureRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialGestureSettings) -> ::windows::core::HRESULT {
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
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISpatialGestureRecognizer>,
            base.5,
            RecognitionStarted::<Impl, OFFSET>,
            RemoveRecognitionStarted::<Impl, OFFSET>,
            RecognitionEnded::<Impl, OFFSET>,
            RemoveRecognitionEnded::<Impl, OFFSET>,
            Tapped::<Impl, OFFSET>,
            RemoveTapped::<Impl, OFFSET>,
            HoldStarted::<Impl, OFFSET>,
            RemoveHoldStarted::<Impl, OFFSET>,
            HoldCompleted::<Impl, OFFSET>,
            RemoveHoldCompleted::<Impl, OFFSET>,
            HoldCanceled::<Impl, OFFSET>,
            RemoveHoldCanceled::<Impl, OFFSET>,
            ManipulationStarted::<Impl, OFFSET>,
            RemoveManipulationStarted::<Impl, OFFSET>,
            ManipulationUpdated::<Impl, OFFSET>,
            RemoveManipulationUpdated::<Impl, OFFSET>,
            ManipulationCompleted::<Impl, OFFSET>,
            RemoveManipulationCompleted::<Impl, OFFSET>,
            ManipulationCanceled::<Impl, OFFSET>,
            RemoveManipulationCanceled::<Impl, OFFSET>,
            NavigationStarted::<Impl, OFFSET>,
            RemoveNavigationStarted::<Impl, OFFSET>,
            NavigationUpdated::<Impl, OFFSET>,
            RemoveNavigationUpdated::<Impl, OFFSET>,
            NavigationCompleted::<Impl, OFFSET>,
            RemoveNavigationCompleted::<Impl, OFFSET>,
            NavigationCanceled::<Impl, OFFSET>,
            RemoveNavigationCanceled::<Impl, OFFSET>,
            CaptureInteraction::<Impl, OFFSET>,
            CancelPendingGestures::<Impl, OFFSET>,
            TrySetGestureSettings::<Impl, OFFSET>,
            GestureSettings::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGestureRecognizerFactoryImpl: Sized {
    fn Create(&self, settings: SpatialGestureSettings) -> ::windows::core::Result<SpatialGestureRecognizer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGestureRecognizerFactory {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGestureRecognizerFactoryVtbl {
    pub const fn new<Impl: ISpatialGestureRecognizerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialGestureRecognizerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpatialGestureRecognizerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(settings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialGestureRecognizerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialHoldCanceledEventArgsVtbl {
    pub const fn new<Impl: ISpatialHoldCanceledEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialHoldCanceledEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldCanceledEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialHoldCanceledEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialHoldCompletedEventArgsVtbl {
    pub const fn new<Impl: ISpatialHoldCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialHoldCompletedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialHoldCompletedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialHoldStartedEventArgsVtbl {
    pub const fn new<Impl: ISpatialHoldStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialHoldStartedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialHoldStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialHoldStartedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionImpl: Sized {
    fn SourceState(&self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteraction";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionVtbl {
    pub const fn new<Impl: ISpatialInteractionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionVtbl {
        unsafe extern "system" fn SourceState<Impl: ISpatialInteractionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteraction>, base.5, SourceState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionControllerImpl: Sized {
    fn HasTouchpad(&self) -> ::windows::core::Result<bool>;
    fn HasThumbstick(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::super::Devices::Haptics::SimpleHapticsController>;
    fn VendorId(&self) -> ::windows::core::Result<u16>;
    fn ProductId(&self) -> ::windows::core::Result<u16>;
    fn Version(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionControllerVtbl {
    pub const fn new<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionControllerVtbl {
        unsafe extern "system" fn HasTouchpad<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasTouchpad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasThumbstick<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasThumbstick() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VendorId<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ISpatialInteractionControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionController>, base.5, HasTouchpad::<Impl, OFFSET>, HasThumbstick::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>, VendorId::<Impl, OFFSET>, ProductId::<Impl, OFFSET>, Version::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionController2Impl: Sized + ISpatialInteractionControllerImpl {
    fn TryGetRenderableModelAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionController2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionController2Vtbl {
    pub const fn new<Impl: ISpatialInteractionController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionController2Vtbl {
        unsafe extern "system" fn TryGetRenderableModelAsync<Impl: ISpatialInteractionController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetRenderableModelAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionController2>, base.5, TryGetRenderableModelAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionController3Impl: Sized + ISpatialInteractionControllerImpl + ISpatialInteractionController2Impl {
    fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionController3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionController3Vtbl {
    pub const fn new<Impl: ISpatialInteractionController3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionController3Vtbl {
        unsafe extern "system" fn TryGetBatteryReport<Impl: ISpatialInteractionController3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetBatteryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionController3>, base.5, TryGetBatteryReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionControllerPropertiesImpl: Sized {
    fn IsTouchpadTouched(&self) -> ::windows::core::Result<bool>;
    fn IsTouchpadPressed(&self) -> ::windows::core::Result<bool>;
    fn IsThumbstickPressed(&self) -> ::windows::core::Result<bool>;
    fn ThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn ThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn TouchpadX(&self) -> ::windows::core::Result<f64>;
    fn TouchpadY(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionControllerPropertiesVtbl {
    pub const fn new<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionControllerPropertiesVtbl {
        unsafe extern "system" fn IsTouchpadTouched<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTouchpadTouched() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTouchpadPressed<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTouchpadPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThumbstickPressed<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThumbstickPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbstickX<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbstickX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbstickY<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbstickY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchpadX<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TouchpadX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchpadY<Impl: ISpatialInteractionControllerPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TouchpadY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionControllerProperties>, base.5, IsTouchpadTouched::<Impl, OFFSET>, IsTouchpadPressed::<Impl, OFFSET>, IsThumbstickPressed::<Impl, OFFSET>, ThumbstickX::<Impl, OFFSET>, ThumbstickY::<Impl, OFFSET>, TouchpadX::<Impl, OFFSET>, TouchpadY::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionDetectedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn Interaction(&self) -> ::windows::core::Result<SpatialInteraction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionDetectedEventArgsVtbl {
    pub const fn new<Impl: ISpatialInteractionDetectedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionDetectedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialInteractionDetectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialInteractionDetectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interaction<Impl: ISpatialInteractionDetectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Interaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionDetectedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>, Interaction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionDetectedEventArgs2Impl: Sized + ISpatialInteractionDetectedEventArgsImpl {
    fn InteractionSource(&self) -> ::windows::core::Result<SpatialInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionDetectedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionDetectedEventArgs2Vtbl {
    pub const fn new<Impl: ISpatialInteractionDetectedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionDetectedEventArgs2Vtbl {
        unsafe extern "system" fn InteractionSource<Impl: ISpatialInteractionDetectedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionDetectedEventArgs2>, base.5, InteractionSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerImpl: Sized {
    fn SourceDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourcePressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourcePressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceReleased(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InteractionDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInteractionDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDetectedSourcesAtTimestamp(&self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManager";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionManagerVtbl {
    pub const fn new<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionManagerVtbl {
        unsafe extern "system" fn SourceDetected<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceDetected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceDetected<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceDetected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceLost<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceLost(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceLost<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceUpdated<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceUpdated<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourcePressed<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePressed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourcePressed<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourcePressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceReleased<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceReleased(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceReleased<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceReleased(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InteractionDetected<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionDetected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInteractionDetected<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInteractionDetected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDetectedSourcesAtTimestamp<Impl: ISpatialInteractionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDetectedSourcesAtTimestamp(&*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISpatialInteractionManager>,
            base.5,
            SourceDetected::<Impl, OFFSET>,
            RemoveSourceDetected::<Impl, OFFSET>,
            SourceLost::<Impl, OFFSET>,
            RemoveSourceLost::<Impl, OFFSET>,
            SourceUpdated::<Impl, OFFSET>,
            RemoveSourceUpdated::<Impl, OFFSET>,
            SourcePressed::<Impl, OFFSET>,
            RemoveSourcePressed::<Impl, OFFSET>,
            SourceReleased::<Impl, OFFSET>,
            RemoveSourceReleased::<Impl, OFFSET>,
            InteractionDetected::<Impl, OFFSET>,
            RemoveInteractionDetected::<Impl, OFFSET>,
            GetDetectedSourcesAtTimestamp::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SpatialInteractionManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerStatics {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionManagerStaticsVtbl {
    pub const fn new<Impl: ISpatialInteractionManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISpatialInteractionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionManagerStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStatics2Impl: Sized {
    fn IsSourceKindSupported(&self, kind: SpatialInteractionSourceKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionManagerStatics2Vtbl {
    pub const fn new<Impl: ISpatialInteractionManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionManagerStatics2Vtbl {
        unsafe extern "system" fn IsSourceKindSupported<Impl: ISpatialInteractionManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSourceKindSupported(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionManagerStatics2>, base.5, IsSourceKindSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceVtbl {
    pub const fn new<Impl: ISpatialInteractionSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceVtbl {
        unsafe extern "system" fn Id<Impl: ISpatialInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: ISpatialInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSource>, base.5, Id::<Impl, OFFSET>, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource2Impl: Sized + ISpatialInteractionSourceImpl {
    fn IsPointingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsMenuSupported(&self) -> ::windows::core::Result<bool>;
    fn IsGraspSupported(&self) -> ::windows::core::Result<bool>;
    fn Controller(&self) -> ::windows::core::Result<SpatialInteractionController>;
    fn TryGetStateAtTimestamp(&self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSource2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSource2Vtbl {
    pub const fn new<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSource2Vtbl {
        unsafe extern "system" fn IsPointingSupported<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPointingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMenuSupported<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMenuSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGraspSupported<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGraspSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetStateAtTimestamp<Impl: ISpatialInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetStateAtTimestamp(&*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSource2>, base.5, IsPointingSupported::<Impl, OFFSET>, IsMenuSupported::<Impl, OFFSET>, IsGraspSupported::<Impl, OFFSET>, Controller::<Impl, OFFSET>, TryGetStateAtTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource3Impl: Sized + ISpatialInteractionSourceImpl + ISpatialInteractionSource2Impl {
    fn Handedness(&self) -> ::windows::core::Result<SpatialInteractionSourceHandedness>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSource3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSource3Vtbl {
    pub const fn new<Impl: ISpatialInteractionSource3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSource3Vtbl {
        unsafe extern "system" fn Handedness<Impl: ISpatialInteractionSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceHandedness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handedness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSource3>, base.5, Handedness::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource4Impl: Sized {
    fn TryCreateHandMeshObserver(&self) -> ::windows::core::Result<super::super::super::Perception::People::HandMeshObserver>;
    fn TryCreateHandMeshObserverAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSource4 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource4";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSource4Vtbl {
    pub const fn new<Impl: ISpatialInteractionSource4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSource4Vtbl {
        unsafe extern "system" fn TryCreateHandMeshObserver<Impl: ISpatialInteractionSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateHandMeshObserver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateHandMeshObserverAsync<Impl: ISpatialInteractionSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateHandMeshObserverAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSource4>, base.5, TryCreateHandMeshObserver::<Impl, OFFSET>, TryCreateHandMeshObserverAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceEventArgsVtbl {
    pub const fn new<Impl: ISpatialInteractionSourceEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceEventArgsVtbl {
        unsafe extern "system" fn State<Impl: ISpatialInteractionSourceEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceEventArgs>, base.5, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgs2Impl: Sized + ISpatialInteractionSourceEventArgsImpl {
    fn PressKind(&self) -> ::windows::core::Result<SpatialInteractionPressKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceEventArgs2Vtbl {
    pub const fn new<Impl: ISpatialInteractionSourceEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceEventArgs2Vtbl {
        unsafe extern "system" fn PressKind<Impl: ISpatialInteractionSourceEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionPressKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PressKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceEventArgs2>, base.5, PressKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocationImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn Velocity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceLocationVtbl {
    pub const fn new<Impl: ISpatialInteractionSourceLocationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceLocationVtbl {
        unsafe extern "system" fn Position<Impl: ISpatialInteractionSourceLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocity<Impl: ISpatialInteractionSourceLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceLocation>, base.5, Position::<Impl, OFFSET>, Velocity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocation2Impl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceLocation2Vtbl {
    pub const fn new<Impl: ISpatialInteractionSourceLocation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceLocation2Vtbl {
        unsafe extern "system" fn Orientation<Impl: ISpatialInteractionSourceLocation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceLocation2>, base.5, Orientation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocation3Impl: Sized + ISpatialInteractionSourceLocation2Impl {
    fn PositionAccuracy(&self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
    fn AngularVelocity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourcePointerPose(&self) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceLocation3Vtbl {
    pub const fn new<Impl: ISpatialInteractionSourceLocation3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceLocation3Vtbl {
        unsafe extern "system" fn PositionAccuracy<Impl: ISpatialInteractionSourceLocation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngularVelocity<Impl: ISpatialInteractionSourceLocation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePointerPose<Impl: ISpatialInteractionSourceLocation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePointerPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceLocation3>, base.5, PositionAccuracy::<Impl, OFFSET>, AngularVelocity::<Impl, OFFSET>, SourcePointerPose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourcePropertiesImpl: Sized {
    fn TryGetSourceLossMitigationDirection(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourceLossRisk(&self) -> ::windows::core::Result<f64>;
    fn TryGetLocation(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialInteractionSourceLocation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourcePropertiesVtbl {
    pub const fn new<Impl: ISpatialInteractionSourcePropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourcePropertiesVtbl {
        unsafe extern "system" fn TryGetSourceLossMitigationDirection<Impl: ISpatialInteractionSourcePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetSourceLossMitigationDirection(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceLossRisk<Impl: ISpatialInteractionSourcePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceLossRisk() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetLocation<Impl: ISpatialInteractionSourcePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetLocation(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceProperties>, base.5, TryGetSourceLossMitigationDirection::<Impl, OFFSET>, SourceLossRisk::<Impl, OFFSET>, TryGetLocation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceStateImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<SpatialInteractionSource>;
    fn Properties(&self) -> ::windows::core::Result<SpatialInteractionSourceProperties>;
    fn IsPressed(&self) -> ::windows::core::Result<bool>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceStateVtbl {
    pub const fn new<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceStateVtbl {
        unsafe extern "system" fn Source<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPressed<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialInteractionSourceStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceState>, base.5, Source::<Impl, OFFSET>, Properties::<Impl, OFFSET>, IsPressed::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceState2Impl: Sized + ISpatialInteractionSourceStateImpl {
    fn IsSelectPressed(&self) -> ::windows::core::Result<bool>;
    fn IsMenuPressed(&self) -> ::windows::core::Result<bool>;
    fn IsGrasped(&self) -> ::windows::core::Result<bool>;
    fn SelectPressedValue(&self) -> ::windows::core::Result<f64>;
    fn ControllerProperties(&self) -> ::windows::core::Result<SpatialInteractionControllerProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceState2Vtbl {
    pub const fn new<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceState2Vtbl {
        unsafe extern "system" fn IsSelectPressed<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSelectPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMenuPressed<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMenuPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrasped<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGrasped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPressedValue<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectPressedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControllerProperties<Impl: ISpatialInteractionSourceState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControllerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceState2>, base.5, IsSelectPressed::<Impl, OFFSET>, IsMenuPressed::<Impl, OFFSET>, IsGrasped::<Impl, OFFSET>, SelectPressedValue::<Impl, OFFSET>, ControllerProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceState3Impl: Sized + ISpatialInteractionSourceStateImpl + ISpatialInteractionSourceState2Impl {
    fn TryGetHandPose(&self) -> ::windows::core::Result<super::super::super::Perception::People::HandPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceState3Vtbl {
    pub const fn new<Impl: ISpatialInteractionSourceState3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionSourceState3Vtbl {
        unsafe extern "system" fn TryGetHandPose<Impl: ISpatialInteractionSourceState3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetHandPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionSourceState3>, base.5, TryGetHandPose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationCanceledEventArgsVtbl {
    pub const fn new<Impl: ISpatialManipulationCanceledEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialManipulationCanceledEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationCanceledEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialManipulationCanceledEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationCompletedEventArgsVtbl {
    pub const fn new<Impl: ISpatialManipulationCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialManipulationCompletedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCumulativeDelta<Impl: ISpatialManipulationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetCumulativeDelta(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialManipulationCompletedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetCumulativeDelta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationDeltaImpl: Sized {
    fn Translation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationDelta";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationDeltaVtbl {
    pub const fn new<Impl: ISpatialManipulationDeltaImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialManipulationDeltaVtbl {
        unsafe extern "system" fn Translation<Impl: ISpatialManipulationDeltaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Translation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialManipulationDelta>, base.5, Translation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationStartedEventArgsVtbl {
    pub const fn new<Impl: ISpatialManipulationStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialManipulationStartedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialManipulationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialManipulationStartedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationUpdatedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationUpdatedEventArgsVtbl {
    pub const fn new<Impl: ISpatialManipulationUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialManipulationUpdatedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCumulativeDelta<Impl: ISpatialManipulationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetCumulativeDelta(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialManipulationUpdatedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetCumulativeDelta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialNavigationCanceledEventArgsVtbl {
    pub const fn new<Impl: ISpatialNavigationCanceledEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialNavigationCanceledEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationCanceledEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialNavigationCanceledEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialNavigationCompletedEventArgsVtbl {
    pub const fn new<Impl: ISpatialNavigationCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialNavigationCompletedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOffset<Impl: ISpatialNavigationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialNavigationCompletedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, NormalizedOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsNavigatingX(&self) -> ::windows::core::Result<bool>;
    fn IsNavigatingY(&self) -> ::windows::core::Result<bool>;
    fn IsNavigatingZ(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialNavigationStartedEventArgsVtbl {
    pub const fn new<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialNavigationStartedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingX<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNavigatingX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingY<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNavigatingY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingZ<Impl: ISpatialNavigationStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNavigatingZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialNavigationStartedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>, IsNavigatingX::<Impl, OFFSET>, IsNavigatingY::<Impl, OFFSET>, IsNavigatingZ::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationUpdatedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialNavigationUpdatedEventArgsVtbl {
    pub const fn new<Impl: ISpatialNavigationUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialNavigationUpdatedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOffset<Impl: ISpatialNavigationUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialNavigationUpdatedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, NormalizedOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerInteractionSourcePoseImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerInteractionSourcePoseVtbl {
    pub const fn new<Impl: ISpatialPointerInteractionSourcePoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerInteractionSourcePoseVtbl {
        unsafe extern "system" fn Position<Impl: ISpatialPointerInteractionSourcePoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForwardDirection<Impl: ISpatialPointerInteractionSourcePoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpDirection<Impl: ISpatialPointerInteractionSourcePoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerInteractionSourcePose>, base.5, Position::<Impl, OFFSET>, ForwardDirection::<Impl, OFFSET>, UpDirection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerInteractionSourcePose2Impl: Sized + ISpatialPointerInteractionSourcePoseImpl {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
    fn PositionAccuracy(&self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerInteractionSourcePose2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerInteractionSourcePose2Vtbl {
    pub const fn new<Impl: ISpatialPointerInteractionSourcePose2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerInteractionSourcePose2Vtbl {
        unsafe extern "system" fn Orientation<Impl: ISpatialPointerInteractionSourcePose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionAccuracy<Impl: ISpatialPointerInteractionSourcePose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerInteractionSourcePose2>, base.5, Orientation::<Impl, OFFSET>, PositionAccuracy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPoseImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn Head(&self) -> ::windows::core::Result<super::super::super::Perception::People::HeadPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerPoseVtbl {
    pub const fn new<Impl: ISpatialPointerPoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerPoseVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISpatialPointerPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Head<Impl: ISpatialPointerPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Head() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerPose>, base.5, Timestamp::<Impl, OFFSET>, Head::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPose2Impl: Sized + ISpatialPointerPoseImpl {
    fn TryGetInteractionSourcePose(&self, source: &::core::option::Option<SpatialInteractionSource>) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerPose2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerPose2Vtbl {
    pub const fn new<Impl: ISpatialPointerPose2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerPose2Vtbl {
        unsafe extern "system" fn TryGetInteractionSourcePose<Impl: ISpatialPointerPose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetInteractionSourcePose(&*(&source as *const <SpatialInteractionSource as ::windows::core::Abi>::Abi as *const <SpatialInteractionSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerPose2>, base.5, TryGetInteractionSourcePose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPose3Impl: Sized {
    fn Eyes(&self) -> ::windows::core::Result<super::super::super::Perception::People::EyesPose>;
    fn IsHeadCapturedBySystem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerPose3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerPose3Vtbl {
    pub const fn new<Impl: ISpatialPointerPose3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerPose3Vtbl {
        unsafe extern "system" fn Eyes<Impl: ISpatialPointerPose3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Eyes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCapturedBySystem<Impl: ISpatialPointerPose3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHeadCapturedBySystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerPose3>, base.5, Eyes::<Impl, OFFSET>, IsHeadCapturedBySystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPoseStaticsImpl: Sized {
    fn TryGetAtTimestamp(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialPointerPoseStatics {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPoseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialPointerPoseStaticsVtbl {
    pub const fn new<Impl: ISpatialPointerPoseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialPointerPoseStaticsVtbl {
        unsafe extern "system" fn TryGetAtTimestamp<Impl: ISpatialPointerPoseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetAtTimestamp(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialPointerPoseStatics>, base.5, TryGetAtTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialRecognitionEndedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialRecognitionEndedEventArgsVtbl {
    pub const fn new<Impl: ISpatialRecognitionEndedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialRecognitionEndedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialRecognitionEndedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialRecognitionEndedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialRecognitionStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialRecognitionStartedEventArgsVtbl {
    pub const fn new<Impl: ISpatialRecognitionStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialRecognitionStartedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialRecognitionStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialRecognitionStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGesturePossible<Impl: ISpatialRecognitionStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gesture: SpatialGestureSettings, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGesturePossible(gesture) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialRecognitionStartedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>, IsGesturePossible::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialTappedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn TapCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialTappedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialTappedEventArgsVtbl {
    pub const fn new<Impl: ISpatialTappedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialTappedEventArgsVtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapCount<Impl: ISpatialTappedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialTappedEventArgs>, base.5, InteractionSourceKind::<Impl, OFFSET>, TryGetPointerPose::<Impl, OFFSET>, TapCount::<Impl, OFFSET>)
    }
}
