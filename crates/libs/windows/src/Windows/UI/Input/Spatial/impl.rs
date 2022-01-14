#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialGestureRecognizer_Impl: Sized {
    fn RecognitionStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionStarted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecognitionEnded(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionEnded(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldStarted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCanceled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCanceled(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCanceled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCanceled(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationUpdated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCanceled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCanceled(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureInteraction(&mut self, interaction: &::core::option::Option<SpatialInteraction>) -> ::windows::core::Result<()>;
    fn CancelPendingGestures(&mut self) -> ::windows::core::Result<()>;
    fn TrySetGestureSettings(&mut self, settings: SpatialGestureSettings) -> ::windows::core::Result<bool>;
    fn GestureSettings(&mut self) -> ::windows::core::Result<SpatialGestureSettings>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialGestureRecognizer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialGestureRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialGestureRecognizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialGestureRecognizer_Vtbl {
        unsafe extern "system" fn RecognitionStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognitionStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecognitionStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognitionEnded<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognitionEnded(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionEnded<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecognitionEnded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tapped<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHoldStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHoldCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHoldCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationUpdated<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationUpdated<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationStarted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationUpdated<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationUpdated<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCanceled<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationCanceled(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureInteraction<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CaptureInteraction(&*(&interaction as *const <SpatialInteraction as ::windows::core::Abi>::Abi as *const <SpatialInteraction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelPendingGestures<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelPendingGestures().into()
        }
        unsafe extern "system" fn TrySetGestureSettings<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetGestureSettings(settings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GestureSettings<Impl: ISpatialGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialGestureSettings) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialGestureRecognizer, BASE_OFFSET>(),
            RecognitionStarted: RecognitionStarted::<Impl, IMPL_OFFSET>,
            RemoveRecognitionStarted: RemoveRecognitionStarted::<Impl, IMPL_OFFSET>,
            RecognitionEnded: RecognitionEnded::<Impl, IMPL_OFFSET>,
            RemoveRecognitionEnded: RemoveRecognitionEnded::<Impl, IMPL_OFFSET>,
            Tapped: Tapped::<Impl, IMPL_OFFSET>,
            RemoveTapped: RemoveTapped::<Impl, IMPL_OFFSET>,
            HoldStarted: HoldStarted::<Impl, IMPL_OFFSET>,
            RemoveHoldStarted: RemoveHoldStarted::<Impl, IMPL_OFFSET>,
            HoldCompleted: HoldCompleted::<Impl, IMPL_OFFSET>,
            RemoveHoldCompleted: RemoveHoldCompleted::<Impl, IMPL_OFFSET>,
            HoldCanceled: HoldCanceled::<Impl, IMPL_OFFSET>,
            RemoveHoldCanceled: RemoveHoldCanceled::<Impl, IMPL_OFFSET>,
            ManipulationStarted: ManipulationStarted::<Impl, IMPL_OFFSET>,
            RemoveManipulationStarted: RemoveManipulationStarted::<Impl, IMPL_OFFSET>,
            ManipulationUpdated: ManipulationUpdated::<Impl, IMPL_OFFSET>,
            RemoveManipulationUpdated: RemoveManipulationUpdated::<Impl, IMPL_OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Impl, IMPL_OFFSET>,
            RemoveManipulationCompleted: RemoveManipulationCompleted::<Impl, IMPL_OFFSET>,
            ManipulationCanceled: ManipulationCanceled::<Impl, IMPL_OFFSET>,
            RemoveManipulationCanceled: RemoveManipulationCanceled::<Impl, IMPL_OFFSET>,
            NavigationStarted: NavigationStarted::<Impl, IMPL_OFFSET>,
            RemoveNavigationStarted: RemoveNavigationStarted::<Impl, IMPL_OFFSET>,
            NavigationUpdated: NavigationUpdated::<Impl, IMPL_OFFSET>,
            RemoveNavigationUpdated: RemoveNavigationUpdated::<Impl, IMPL_OFFSET>,
            NavigationCompleted: NavigationCompleted::<Impl, IMPL_OFFSET>,
            RemoveNavigationCompleted: RemoveNavigationCompleted::<Impl, IMPL_OFFSET>,
            NavigationCanceled: NavigationCanceled::<Impl, IMPL_OFFSET>,
            RemoveNavigationCanceled: RemoveNavigationCanceled::<Impl, IMPL_OFFSET>,
            CaptureInteraction: CaptureInteraction::<Impl, IMPL_OFFSET>,
            CancelPendingGestures: CancelPendingGestures::<Impl, IMPL_OFFSET>,
            TrySetGestureSettings: TrySetGestureSettings::<Impl, IMPL_OFFSET>,
            GestureSettings: GestureSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialGestureRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGestureRecognizerFactory_Impl: Sized {
    fn Create(&mut self, settings: SpatialGestureSettings) -> ::windows::core::Result<SpatialGestureRecognizer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGestureRecognizerFactory {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialGestureRecognizerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGestureRecognizerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialGestureRecognizerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialGestureRecognizerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISpatialGestureRecognizerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(settings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialGestureRecognizerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialGestureRecognizerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCanceledEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialHoldCanceledEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialHoldCanceledEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialHoldCanceledEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldCanceledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialHoldCanceledEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialHoldCanceledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCompletedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialHoldCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialHoldCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialHoldCompletedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialHoldCompletedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialHoldCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialHoldStartedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialHoldStartedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialHoldStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialHoldStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialHoldStartedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialHoldStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialHoldStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialHoldStartedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialHoldStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteraction_Impl: Sized {
    fn SourceState(&mut self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteraction";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteraction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteraction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteraction_Vtbl {
        unsafe extern "system" fn SourceState<Impl: ISpatialInteraction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteraction, BASE_OFFSET>(), SourceState: SourceState::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteraction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
pub trait ISpatialInteractionController_Impl: Sized {
    fn HasTouchpad(&mut self) -> ::windows::core::Result<bool>;
    fn HasThumbstick(&mut self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<super::super::super::Devices::Haptics::SimpleHapticsController>;
    fn VendorId(&mut self) -> ::windows::core::Result<u16>;
    fn ProductId(&mut self) -> ::windows::core::Result<u16>;
    fn Version(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController";
}
#[cfg(all(feature = "Devices_Haptics", feature = "implement_exclusive"))]
impl ISpatialInteractionController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionController_Vtbl {
        unsafe extern "system" fn HasTouchpad<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasTouchpad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasThumbstick<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasThumbstick() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimpleHapticsController<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VendorId<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ISpatialInteractionController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionController, BASE_OFFSET>(),
            HasTouchpad: HasTouchpad::<Impl, IMPL_OFFSET>,
            HasThumbstick: HasThumbstick::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
            VendorId: VendorId::<Impl, IMPL_OFFSET>,
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpatialInteractionController2_Impl: Sized + ISpatialInteractionController_Impl {
    fn TryGetRenderableModelAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionController2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpatialInteractionController2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionController2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionController2_Vtbl {
        unsafe extern "system" fn TryGetRenderableModelAsync<Impl: ISpatialInteractionController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetRenderableModelAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionController2, BASE_OFFSET>(),
            TryGetRenderableModelAsync: TryGetRenderableModelAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "Devices_Power", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpatialInteractionController3_Impl: Sized + ISpatialInteractionController_Impl + ISpatialInteractionController2_Impl {
    fn TryGetBatteryReport(&mut self) -> ::windows::core::Result<super::super::super::Devices::Power::BatteryReport>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "Devices_Power", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionController3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionController3";
}
#[cfg(all(feature = "Devices_Haptics", feature = "Devices_Power", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpatialInteractionController3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionController3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionController3_Vtbl {
        unsafe extern "system" fn TryGetBatteryReport<Impl: ISpatialInteractionController3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetBatteryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionController3, BASE_OFFSET>(),
            TryGetBatteryReport: TryGetBatteryReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionController3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionControllerProperties_Impl: Sized {
    fn IsTouchpadTouched(&mut self) -> ::windows::core::Result<bool>;
    fn IsTouchpadPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsThumbstickPressed(&mut self) -> ::windows::core::Result<bool>;
    fn ThumbstickX(&mut self) -> ::windows::core::Result<f64>;
    fn ThumbstickY(&mut self) -> ::windows::core::Result<f64>;
    fn TouchpadX(&mut self) -> ::windows::core::Result<f64>;
    fn TouchpadY(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionControllerProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionControllerProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionControllerProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionControllerProperties_Vtbl {
        unsafe extern "system" fn IsTouchpadTouched<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTouchpadTouched() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTouchpadPressed<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTouchpadPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThumbstickPressed<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThumbstickPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbstickX<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbstickX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbstickY<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbstickY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchpadX<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchpadX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchpadY<Impl: ISpatialInteractionControllerProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchpadY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionControllerProperties, BASE_OFFSET>(),
            IsTouchpadTouched: IsTouchpadTouched::<Impl, IMPL_OFFSET>,
            IsTouchpadPressed: IsTouchpadPressed::<Impl, IMPL_OFFSET>,
            IsThumbstickPressed: IsThumbstickPressed::<Impl, IMPL_OFFSET>,
            ThumbstickX: ThumbstickX::<Impl, IMPL_OFFSET>,
            ThumbstickY: ThumbstickY::<Impl, IMPL_OFFSET>,
            TouchpadX: TouchpadX::<Impl, IMPL_OFFSET>,
            TouchpadY: TouchpadY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionControllerProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionDetectedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn Interaction(&mut self) -> ::windows::core::Result<SpatialInteraction>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionDetectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionDetectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionDetectedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialInteractionDetectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialInteractionDetectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interaction<Impl: ISpatialInteractionDetectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionDetectedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
            Interaction: Interaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionDetectedEventArgs2_Impl: Sized + ISpatialInteractionDetectedEventArgs_Impl {
    fn InteractionSource(&mut self) -> ::windows::core::Result<SpatialInteractionSource>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionDetectedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionDetectedEventArgs2";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionDetectedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionDetectedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionDetectedEventArgs2_Vtbl {
        unsafe extern "system" fn InteractionSource<Impl: ISpatialInteractionDetectedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionDetectedEventArgs2, BASE_OFFSET>(),
            InteractionSource: InteractionSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionDetectedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
pub trait ISpatialInteractionManager_Impl: Sized {
    fn SourceDetected(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceDetected(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceLost(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceLost(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceUpdated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourcePressed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourcePressed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceReleased(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceReleased(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InteractionDetected(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInteractionDetected(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDetectedSourcesAtTimestamp(&mut self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
impl ISpatialInteractionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionManager_Vtbl {
        unsafe extern "system" fn SourceDetected<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDetected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceDetected<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceDetected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceLost<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceLost(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceLost<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceUpdated<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceUpdated<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourcePressed<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePressed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourcePressed<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourcePressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceReleased<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceReleased(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceReleased<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceReleased(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InteractionDetected<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionDetected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInteractionDetected<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInteractionDetected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDetectedSourcesAtTimestamp<Impl: ISpatialInteractionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDetectedSourcesAtTimestamp(&*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionManager, BASE_OFFSET>(),
            SourceDetected: SourceDetected::<Impl, IMPL_OFFSET>,
            RemoveSourceDetected: RemoveSourceDetected::<Impl, IMPL_OFFSET>,
            SourceLost: SourceLost::<Impl, IMPL_OFFSET>,
            RemoveSourceLost: RemoveSourceLost::<Impl, IMPL_OFFSET>,
            SourceUpdated: SourceUpdated::<Impl, IMPL_OFFSET>,
            RemoveSourceUpdated: RemoveSourceUpdated::<Impl, IMPL_OFFSET>,
            SourcePressed: SourcePressed::<Impl, IMPL_OFFSET>,
            RemoveSourcePressed: RemoveSourcePressed::<Impl, IMPL_OFFSET>,
            SourceReleased: SourceReleased::<Impl, IMPL_OFFSET>,
            RemoveSourceReleased: RemoveSourceReleased::<Impl, IMPL_OFFSET>,
            InteractionDetected: InteractionDetected::<Impl, IMPL_OFFSET>,
            RemoveInteractionDetected: RemoveInteractionDetected::<Impl, IMPL_OFFSET>,
            GetDetectedSourcesAtTimestamp: GetDetectedSourcesAtTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<SpatialInteractionManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerStatics {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionManagerStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISpatialInteractionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStatics2_Impl: Sized {
    fn IsSourceKindSupported(&mut self, kind: SpatialInteractionSourceKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionManagerStatics2_Vtbl {
        unsafe extern "system" fn IsSourceKindSupported<Impl: ISpatialInteractionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSourceKindSupported(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionManagerStatics2, BASE_OFFSET>(),
            IsSourceKindSupported: IsSourceKindSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Kind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSource_Vtbl {
        unsafe extern "system" fn Id<Impl: ISpatialInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: ISpatialInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSource, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSource2_Impl: Sized + ISpatialInteractionSource_Impl {
    fn IsPointingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsMenuSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsGraspSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Controller(&mut self) -> ::windows::core::Result<SpatialInteractionController>;
    fn TryGetStateAtTimestamp(&mut self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSource2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource2";
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
impl ISpatialInteractionSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSource2_Vtbl {
        unsafe extern "system" fn IsPointingSupported<Impl: ISpatialInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPointingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMenuSupported<Impl: ISpatialInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMenuSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGraspSupported<Impl: ISpatialInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGraspSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: ISpatialInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetStateAtTimestamp<Impl: ISpatialInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetStateAtTimestamp(&*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSource2, BASE_OFFSET>(),
            IsPointingSupported: IsPointingSupported::<Impl, IMPL_OFFSET>,
            IsMenuSupported: IsMenuSupported::<Impl, IMPL_OFFSET>,
            IsGraspSupported: IsGraspSupported::<Impl, IMPL_OFFSET>,
            Controller: Controller::<Impl, IMPL_OFFSET>,
            TryGetStateAtTimestamp: TryGetStateAtTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSource3_Impl: Sized + ISpatialInteractionSource_Impl + ISpatialInteractionSource2_Impl {
    fn Handedness(&mut self) -> ::windows::core::Result<SpatialInteractionSourceHandedness>;
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSource3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource3";
}
#[cfg(all(feature = "Perception", feature = "implement_exclusive"))]
impl ISpatialInteractionSource3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSource3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSource3_Vtbl {
        unsafe extern "system" fn Handedness<Impl: ISpatialInteractionSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceHandedness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handedness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSource3, BASE_OFFSET>(), Handedness: Handedness::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Perception_People", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSource4_Impl: Sized {
    fn TryCreateHandMeshObserver(&mut self) -> ::windows::core::Result<super::super::super::Perception::People::HandMeshObserver>;
    fn TryCreateHandMeshObserverAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>;
}
#[cfg(all(feature = "Foundation", feature = "Perception_People", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSource4 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSource4";
}
#[cfg(all(feature = "Foundation", feature = "Perception_People", feature = "implement_exclusive"))]
impl ISpatialInteractionSource4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSource4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSource4_Vtbl {
        unsafe extern "system" fn TryCreateHandMeshObserver<Impl: ISpatialInteractionSource4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateHandMeshObserver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateHandMeshObserverAsync<Impl: ISpatialInteractionSource4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateHandMeshObserverAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSource4, BASE_OFFSET>(),
            TryCreateHandMeshObserver: TryCreateHandMeshObserver::<Impl, IMPL_OFFSET>,
            TryCreateHandMeshObserverAsync: TryCreateHandMeshObserverAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSource4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgs_Impl: Sized {
    fn State(&mut self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceEventArgs_Vtbl {
        unsafe extern "system" fn State<Impl: ISpatialInteractionSourceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceEventArgs, BASE_OFFSET>(), State: State::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgs2_Impl: Sized + ISpatialInteractionSourceEventArgs_Impl {
    fn PressKind(&mut self) -> ::windows::core::Result<SpatialInteractionPressKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceEventArgs2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialInteractionSourceEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceEventArgs2_Vtbl {
        unsafe extern "system" fn PressKind<Impl: ISpatialInteractionSourceEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionPressKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PressKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceEventArgs2, BASE_OFFSET>(),
            PressKind: PressKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceLocation_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn Velocity(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceLocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceLocation_Vtbl {
        unsafe extern "system" fn Position<Impl: ISpatialInteractionSourceLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocity<Impl: ISpatialInteractionSourceLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceLocation, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Velocity: Velocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceLocation2_Impl: Sized {
    fn Orientation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceLocation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceLocation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceLocation2_Vtbl {
        unsafe extern "system" fn Orientation<Impl: ISpatialInteractionSourceLocation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceLocation2, BASE_OFFSET>(),
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceLocation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceLocation3_Impl: Sized + ISpatialInteractionSourceLocation2_Impl {
    fn PositionAccuracy(&mut self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
    fn AngularVelocity(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourcePointerPose(&mut self) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceLocation3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceLocation3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceLocation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceLocation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceLocation3_Vtbl {
        unsafe extern "system" fn PositionAccuracy<Impl: ISpatialInteractionSourceLocation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngularVelocity<Impl: ISpatialInteractionSourceLocation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePointerPose<Impl: ISpatialInteractionSourceLocation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePointerPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceLocation3, BASE_OFFSET>(),
            PositionAccuracy: PositionAccuracy::<Impl, IMPL_OFFSET>,
            AngularVelocity: AngularVelocity::<Impl, IMPL_OFFSET>,
            SourcePointerPose: SourcePointerPose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceLocation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceProperties_Impl: Sized {
    fn TryGetSourceLossMitigationDirection(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourceLossRisk(&mut self) -> ::windows::core::Result<f64>;
    fn TryGetLocation(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialInteractionSourceLocation>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceProperties";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceProperties_Vtbl {
        unsafe extern "system" fn TryGetSourceLossMitigationDirection<Impl: ISpatialInteractionSourceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetSourceLossMitigationDirection(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceLossRisk<Impl: ISpatialInteractionSourceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceLossRisk() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetLocation<Impl: ISpatialInteractionSourceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetLocation(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceProperties, BASE_OFFSET>(),
            TryGetSourceLossMitigationDirection: TryGetSourceLossMitigationDirection::<Impl, IMPL_OFFSET>,
            SourceLossRisk: SourceLossRisk::<Impl, IMPL_OFFSET>,
            TryGetLocation: TryGetLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceState_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<SpatialInteractionSource>;
    fn Properties(&mut self) -> ::windows::core::Result<SpatialInteractionSourceProperties>;
    fn IsPressed(&mut self) -> ::windows::core::Result<bool>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState";
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceState_Vtbl {
        unsafe extern "system" fn Source<Impl: ISpatialInteractionSourceState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpatialInteractionSourceState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPressed<Impl: ISpatialInteractionSourceState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ISpatialInteractionSourceState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialInteractionSourceState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceState, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            IsPressed: IsPressed::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceState2_Impl: Sized + ISpatialInteractionSourceState_Impl {
    fn IsSelectPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsMenuPressed(&mut self) -> ::windows::core::Result<bool>;
    fn IsGrasped(&mut self) -> ::windows::core::Result<bool>;
    fn SelectPressedValue(&mut self) -> ::windows::core::Result<f64>;
    fn ControllerProperties(&mut self) -> ::windows::core::Result<SpatialInteractionControllerProperties>;
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState2";
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceState2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceState2_Vtbl {
        unsafe extern "system" fn IsSelectPressed<Impl: ISpatialInteractionSourceState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMenuPressed<Impl: ISpatialInteractionSourceState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMenuPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrasped<Impl: ISpatialInteractionSourceState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrasped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPressedValue<Impl: ISpatialInteractionSourceState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectPressedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControllerProperties<Impl: ISpatialInteractionSourceState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControllerProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceState2, BASE_OFFSET>(),
            IsSelectPressed: IsSelectPressed::<Impl, IMPL_OFFSET>,
            IsMenuPressed: IsMenuPressed::<Impl, IMPL_OFFSET>,
            IsGrasped: IsGrasped::<Impl, IMPL_OFFSET>,
            SelectPressedValue: SelectPressedValue::<Impl, IMPL_OFFSET>,
            ControllerProperties: ControllerProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceState2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialInteractionSourceState3_Impl: Sized + ISpatialInteractionSourceState_Impl + ISpatialInteractionSourceState2_Impl {
    fn TryGetHandPose(&mut self) -> ::windows::core::Result<super::super::super::Perception::People::HandPose>;
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialInteractionSourceState3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialInteractionSourceState3";
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialInteractionSourceState3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionSourceState3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionSourceState3_Vtbl {
        unsafe extern "system" fn TryGetHandPose<Impl: ISpatialInteractionSourceState3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetHandPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionSourceState3, BASE_OFFSET>(),
            TryGetHandPose: TryGetHandPose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionSourceState3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationCanceledEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialManipulationCanceledEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialManipulationCanceledEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialManipulationCanceledEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationCanceledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialManipulationCanceledEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialManipulationCanceledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialManipulationCompletedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationCompletedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialManipulationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialManipulationCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialManipulationCompletedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCumulativeDelta<Impl: ISpatialManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetCumulativeDelta(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialManipulationCompletedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetCumulativeDelta: TryGetCumulativeDelta::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialManipulationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialManipulationDelta_Impl: Sized {
    fn Translation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationDelta";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialManipulationDelta_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialManipulationDelta_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialManipulationDelta_Vtbl {
        unsafe extern "system" fn Translation<Impl: ISpatialManipulationDelta_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Translation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialManipulationDelta, BASE_OFFSET>(),
            Translation: Translation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialManipulationDelta as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialManipulationStartedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationStartedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialManipulationStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialManipulationStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialManipulationStartedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialManipulationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialManipulationStartedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialManipulationStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialManipulationUpdatedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialManipulationUpdatedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialManipulationUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialManipulationUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialManipulationUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCumulativeDelta<Impl: ISpatialManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetCumulativeDelta(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialManipulationUpdatedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetCumulativeDelta: TryGetCumulativeDelta::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialManipulationUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationCanceledEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationCanceledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialNavigationCanceledEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialNavigationCanceledEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialNavigationCanceledEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationCanceledEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialNavigationCanceledEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialNavigationCanceledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialNavigationCompletedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationCompletedEventArgs";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialNavigationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialNavigationCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialNavigationCompletedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOffset<Impl: ISpatialNavigationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialNavigationCompletedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            NormalizedOffset: NormalizedOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialNavigationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialNavigationStartedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsNavigatingX(&mut self) -> ::windows::core::Result<bool>;
    fn IsNavigatingY(&mut self) -> ::windows::core::Result<bool>;
    fn IsNavigatingZ(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationStartedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialNavigationStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialNavigationStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialNavigationStartedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialNavigationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingX<Impl: ISpatialNavigationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNavigatingX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingY<Impl: ISpatialNavigationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNavigatingY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNavigatingZ<Impl: ISpatialNavigationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNavigatingZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialNavigationStartedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
            IsNavigatingX: IsNavigatingX::<Impl, IMPL_OFFSET>,
            IsNavigatingY: IsNavigatingY::<Impl, IMPL_OFFSET>,
            IsNavigatingZ: IsNavigatingZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialNavigationStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialNavigationUpdatedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialNavigationUpdatedEventArgs";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialNavigationUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialNavigationUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialNavigationUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialNavigationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOffset<Impl: ISpatialNavigationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialNavigationUpdatedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            NormalizedOffset: NormalizedOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialNavigationUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialPointerInteractionSourcePose_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialPointerInteractionSourcePose_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerInteractionSourcePose_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerInteractionSourcePose_Vtbl {
        unsafe extern "system" fn Position<Impl: ISpatialPointerInteractionSourcePose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForwardDirection<Impl: ISpatialPointerInteractionSourcePose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpDirection<Impl: ISpatialPointerInteractionSourcePose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerInteractionSourcePose, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            ForwardDirection: ForwardDirection::<Impl, IMPL_OFFSET>,
            UpDirection: UpDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerInteractionSourcePose as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialPointerInteractionSourcePose2_Impl: Sized + ISpatialPointerInteractionSourcePose_Impl {
    fn Orientation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
    fn PositionAccuracy(&mut self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerInteractionSourcePose2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerInteractionSourcePose2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialPointerInteractionSourcePose2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerInteractionSourcePose2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerInteractionSourcePose2_Vtbl {
        unsafe extern "system" fn Orientation<Impl: ISpatialPointerInteractionSourcePose2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionAccuracy<Impl: ISpatialPointerInteractionSourcePose2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerInteractionSourcePose2, BASE_OFFSET>(),
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            PositionAccuracy: PositionAccuracy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerInteractionSourcePose2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
pub trait ISpatialPointerPose_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn Head(&mut self) -> ::windows::core::Result<super::super::super::Perception::People::HeadPose>;
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose";
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
impl ISpatialPointerPose_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerPose_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerPose_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: ISpatialPointerPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Head<Impl: ISpatialPointerPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Head() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerPose, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Head: Head::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerPose as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
pub trait ISpatialPointerPose2_Impl: Sized + ISpatialPointerPose_Impl {
    fn TryGetInteractionSourcePose(&mut self, source: &::core::option::Option<SpatialInteractionSource>) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerPose2 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose2";
}
#[cfg(all(feature = "Perception", feature = "Perception_People", feature = "implement_exclusive"))]
impl ISpatialPointerPose2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerPose2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerPose2_Vtbl {
        unsafe extern "system" fn TryGetInteractionSourcePose<Impl: ISpatialPointerPose2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetInteractionSourcePose(&*(&source as *const <SpatialInteractionSource as ::windows::core::Abi>::Abi as *const <SpatialInteractionSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerPose2, BASE_OFFSET>(),
            TryGetInteractionSourcePose: TryGetInteractionSourcePose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerPose2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_People", feature = "implement_exclusive"))]
pub trait ISpatialPointerPose3_Impl: Sized {
    fn Eyes(&mut self) -> ::windows::core::Result<super::super::super::Perception::People::EyesPose>;
    fn IsHeadCapturedBySystem(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Perception_People", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerPose3 {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPose3";
}
#[cfg(all(feature = "Perception_People", feature = "implement_exclusive"))]
impl ISpatialPointerPose3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerPose3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerPose3_Vtbl {
        unsafe extern "system" fn Eyes<Impl: ISpatialPointerPose3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eyes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCapturedBySystem<Impl: ISpatialPointerPose3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHeadCapturedBySystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerPose3, BASE_OFFSET>(),
            Eyes: Eyes::<Impl, IMPL_OFFSET>,
            IsHeadCapturedBySystem: IsHeadCapturedBySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerPose3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialPointerPoseStatics_Impl: Sized {
    fn TryGetAtTimestamp(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialPointerPoseStatics {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialPointerPoseStatics";
}
#[cfg(all(feature = "Perception", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialPointerPoseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialPointerPoseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialPointerPoseStatics_Vtbl {
        unsafe extern "system" fn TryGetAtTimestamp<Impl: ISpatialPointerPoseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetAtTimestamp(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialPointerPoseStatics, BASE_OFFSET>(),
            TryGetAtTimestamp: TryGetAtTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialPointerPoseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialRecognitionEndedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialRecognitionEndedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialRecognitionEndedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialRecognitionEndedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialRecognitionEndedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialRecognitionEndedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialRecognitionEndedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialRecognitionEndedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialRecognitionStartedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsGesturePossible(&mut self, gesture: SpatialGestureSettings) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialRecognitionStartedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialRecognitionStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialRecognitionStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialRecognitionStartedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialRecognitionStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialRecognitionStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGesturePossible<Impl: ISpatialRecognitionStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: SpatialGestureSettings, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGesturePossible(gesture) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialRecognitionStartedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
            IsGesturePossible: IsGesturePossible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialRecognitionStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ISpatialTappedEventArgs_Impl: Sized {
    fn InteractionSourceKind(&mut self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn TapCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.ISpatialTappedEventArgs";
}
#[cfg(all(feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ISpatialTappedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialTappedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialTappedEventArgs_Vtbl {
        unsafe extern "system" fn InteractionSourceKind<Impl: ISpatialTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPointerPose<Impl: ISpatialTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPointerPose(&*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapCount<Impl: ISpatialTappedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialTappedEventArgs, BASE_OFFSET>(),
            InteractionSourceKind: InteractionSourceKind::<Impl, IMPL_OFFSET>,
            TryGetPointerPose: TryGetPointerPose::<Impl, IMPL_OFFSET>,
            TapCount: TapCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialTappedEventArgs as ::windows::core::Interface>::IID
    }
}
