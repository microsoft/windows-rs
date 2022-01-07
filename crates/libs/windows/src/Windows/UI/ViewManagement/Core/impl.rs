#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewImpl: Sized {
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputView";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreFrameworkInputViewVtbl {
    pub const fn new<Impl: ICoreFrameworkInputViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFrameworkInputViewVtbl {
        unsafe extern "system" fn PrimaryViewAnimationStarting<Impl: ICoreFrameworkInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryViewAnimationStarting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewAnimationStarting<Impl: ICoreFrameworkInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewAnimationStarting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OcclusionsChanged<Impl: ICoreFrameworkInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OcclusionsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOcclusionsChanged<Impl: ICoreFrameworkInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOcclusionsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputView>, base.5, PrimaryViewAnimationStarting::<Impl, OFFSET>, RemovePrimaryViewAnimationStarting::<Impl, OFFSET>, OcclusionsChanged::<Impl, OFFSET>, RemoveOcclusionsChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreFrameworkInputViewAnimationStartingEventArgsVtbl {
    pub const fn new<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFrameworkInputViewAnimationStartingEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameworkAnimationRecommended<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameworkAnimationRecommended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnimationDuration<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewAnimationStartingEventArgs>, base.5, Occlusions::<Impl, OFFSET>, FrameworkAnimationRecommended::<Impl, OFFSET>, AnimationDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl {
    pub const fn new<Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewOcclusionsChangedEventArgs>, base.5, Occlusions::<Impl, OFFSET>, Handled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewStaticsImpl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UIContext>) -> ::windows::core::Result<CoreFrameworkInputView>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreFrameworkInputView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreFrameworkInputViewStaticsVtbl {
    pub const fn new<Impl: ICoreFrameworkInputViewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFrameworkInputViewStaticsVtbl {
        unsafe extern "system" fn GetForUIContext<Impl: ICoreFrameworkInputViewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreFrameworkInputViewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewStatics>, base.5, GetForUIContext::<Impl, OFFSET>, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewImpl: Sized {
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool>;
    fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewVtbl {
    pub const fn new<Impl: ICoreInputViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewVtbl {
        unsafe extern "system" fn OcclusionsChanged<Impl: ICoreInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OcclusionsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOcclusionsChanged<Impl: ICoreInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOcclusionsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCoreInputViewOcclusions<Impl: ICoreInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCoreInputViewOcclusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowPrimaryView<Impl: ICoreInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryShowPrimaryView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryHidePrimaryView<Impl: ICoreInputViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryHidePrimaryView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputView>, base.5, OcclusionsChanged::<Impl, OFFSET>, RemoveOcclusionsChanged::<Impl, OFFSET>, GetCoreInputViewOcclusions::<Impl, OFFSET>, TryShowPrimaryView::<Impl, OFFSET>, TryHidePrimaryView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView2Impl: Sized {
    fn XYFocusTransferringFromPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferringFromPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XYFocusTransferredToPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferredToPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryTransferXYFocusToPrimaryView(&self, origin: &super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputView2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputView2Vtbl {
    pub const fn new<Impl: ICoreInputView2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputView2Vtbl {
        unsafe extern "system" fn XYFocusTransferringFromPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusTransferringFromPrimaryView(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveXYFocusTransferringFromPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveXYFocusTransferringFromPrimaryView(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusTransferredToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusTransferredToPrimaryView(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveXYFocusTransferredToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveXYFocusTransferredToPrimaryView(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryTransferXYFocusToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTransferXYFocusToPrimaryView(&*(&origin as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputView2>, base.5, XYFocusTransferringFromPrimaryView::<Impl, OFFSET>, RemoveXYFocusTransferringFromPrimaryView::<Impl, OFFSET>, XYFocusTransferredToPrimaryView::<Impl, OFFSET>, RemoveXYFocusTransferredToPrimaryView::<Impl, OFFSET>, TryTransferXYFocusToPrimaryView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView3Impl: Sized {
    fn TryShow(&self) -> ::windows::core::Result<bool>;
    fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool>;
    fn TryHide(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputView3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView3";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputView3Vtbl {
    pub const fn new<Impl: ICoreInputView3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputView3Vtbl {
        unsafe extern "system" fn TryShow<Impl: ICoreInputView3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryShow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowWithKind<Impl: ICoreInputView3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryShowWithKind(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryHide<Impl: ICoreInputView3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryHide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputView3>, base.5, TryShow::<Impl, OFFSET>, TryShowWithKind::<Impl, OFFSET>, TryHide::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView4Impl: Sized {
    fn PrimaryViewShowing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewShowing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewHiding(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewHiding(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputView4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView4";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputView4Vtbl {
    pub const fn new<Impl: ICoreInputView4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputView4Vtbl {
        unsafe extern "system" fn PrimaryViewShowing<Impl: ICoreInputView4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryViewShowing(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewShowing<Impl: ICoreInputView4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewShowing(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryViewHiding<Impl: ICoreInputView4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryViewHiding(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewHiding<Impl: ICoreInputView4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewHiding(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputView4>, base.5, PrimaryViewShowing::<Impl, OFFSET>, RemovePrimaryViewShowing::<Impl, OFFSET>, PrimaryViewHiding::<Impl, OFFSET>, RemovePrimaryViewHiding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView5Impl: Sized {
    fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool>;
    fn SupportedKindsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedKindsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputView5 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView5";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputView5Vtbl {
    pub const fn new<Impl: ICoreInputView5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputView5Vtbl {
        unsafe extern "system" fn IsKindSupported<Impl: ICoreInputView5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsKindSupported(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedKindsChanged<Impl: ICoreInputView5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedKindsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSupportedKindsChanged<Impl: ICoreInputView5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSupportedKindsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryViewAnimationStarting<Impl: ICoreInputView5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryViewAnimationStarting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewAnimationStarting<Impl: ICoreInputView5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewAnimationStarting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputView5>, base.5, IsKindSupported::<Impl, OFFSET>, SupportedKindsChanged::<Impl, OFFSET>, RemoveSupportedKindsChanged::<Impl, OFFSET>, PrimaryViewAnimationStarting::<Impl, OFFSET>, RemovePrimaryViewAnimationStarting::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewAnimationStartingEventArgsVtbl {
    pub const fn new<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewAnimationStartingEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn AnimationDuration<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewAnimationStartingEventArgs>, base.5, Occlusions::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, AnimationDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewHidingEventArgsImpl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewHidingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewHidingEventArgsVtbl {
    pub const fn new<Impl: ICoreInputViewHidingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewHidingEventArgsVtbl {
        unsafe extern "system" fn TryCancel<Impl: ICoreInputViewHidingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewHidingEventArgs>, base.5, TryCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewOcclusionImpl: Sized {
    fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewOcclusionVtbl {
    pub const fn new<Impl: ICoreInputViewOcclusionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewOcclusionVtbl {
        unsafe extern "system" fn OccludingRect<Impl: ICoreInputViewOcclusionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OccludingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcclusionKind<Impl: ICoreInputViewOcclusionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewOcclusionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OcclusionKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewOcclusion>, base.5, OccludingRect::<Impl, OFFSET>, OcclusionKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewOcclusionsChangedEventArgsVtbl {
    pub const fn new<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewOcclusionsChangedEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewOcclusionsChangedEventArgs>, base.5, Occlusions::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewShowingEventArgsImpl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewShowingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewShowingEventArgsVtbl {
    pub const fn new<Impl: ICoreInputViewShowingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewShowingEventArgsVtbl {
        unsafe extern "system" fn TryCancel<Impl: ICoreInputViewShowingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewShowingEventArgs>, base.5, TryCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreInputView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewStaticsVtbl {
    pub const fn new<Impl: ICoreInputViewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreInputViewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewStatics2Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UIContext>) -> ::windows::core::Result<CoreInputView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewStatics2Vtbl {
    pub const fn new<Impl: ICoreInputViewStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewStatics2Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: ICoreInputViewStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewStatics2>, base.5, GetForUIContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewTransferringXYFocusEventArgsImpl: Sized {
    fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection>;
    fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransferHandled(&self) -> ::windows::core::Result<bool>;
    fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInputViewTransferringXYFocusEventArgsVtbl {
    pub const fn new<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreInputViewTransferringXYFocusEventArgsVtbl {
        unsafe extern "system" fn Origin<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferHandled<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransferHandled(value).into()
        }
        unsafe extern "system" fn TransferHandled<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransferHandled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepPrimaryViewVisible<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeepPrimaryViewVisible(value).into()
        }
        unsafe extern "system" fn KeepPrimaryViewVisible<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeepPrimaryViewVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreInputViewTransferringXYFocusEventArgs>, base.5, Origin::<Impl, OFFSET>, Direction::<Impl, OFFSET>, SetTransferHandled::<Impl, OFFSET>, TransferHandled::<Impl, OFFSET>, SetKeepPrimaryViewVisible::<Impl, OFFSET>, KeepPrimaryViewVisible::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsControllerImpl: Sized {
    fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetAutoHideScrollBars(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetMessageDuration(&self, value: u32) -> ::windows::core::Result<()>;
    fn SetTextScaleFactor(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.IUISettingsController";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsControllerVtbl {
    pub const fn new<Impl: IUISettingsControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUISettingsControllerVtbl {
        unsafe extern "system" fn SetAdvancedEffectsEnabled<Impl: IUISettingsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAdvancedEffectsEnabled(value).into()
        }
        unsafe extern "system" fn SetAnimationsEnabled<Impl: IUISettingsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAnimationsEnabled(value).into()
        }
        unsafe extern "system" fn SetAutoHideScrollBars<Impl: IUISettingsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoHideScrollBars(value).into()
        }
        unsafe extern "system" fn SetMessageDuration<Impl: IUISettingsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMessageDuration(value).into()
        }
        unsafe extern "system" fn SetTextScaleFactor<Impl: IUISettingsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextScaleFactor(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUISettingsController>, base.5, SetAdvancedEffectsEnabled::<Impl, OFFSET>, SetAnimationsEnabled::<Impl, OFFSET>, SetAutoHideScrollBars::<Impl, OFFSET>, SetMessageDuration::<Impl, OFFSET>, SetTextScaleFactor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsControllerStaticsImpl: Sized {
    fn RequestDefaultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsControllerStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.IUISettingsControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsControllerStaticsVtbl {
    pub const fn new<Impl: IUISettingsControllerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUISettingsControllerStaticsVtbl {
        unsafe extern "system" fn RequestDefaultAsync<Impl: IUISettingsControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUISettingsControllerStatics>, base.5, RequestDefaultAsync::<Impl, OFFSET>)
    }
}
