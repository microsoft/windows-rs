#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreFrameworkInputViewImpl: Sized {
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreFrameworkInputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFrameworkInputViewVtbl {
        unsafe extern "system" fn PrimaryViewAnimationStarting<Impl: ICoreFrameworkInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryViewAnimationStarting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewAnimationStarting<Impl: ICoreFrameworkInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewAnimationStarting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OcclusionsChanged<Impl: ICoreFrameworkInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcclusionsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOcclusionsChanged<Impl: ICoreFrameworkInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOcclusionsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreFrameworkInputView, BASE_OFFSET>(),
            PrimaryViewAnimationStarting: PrimaryViewAnimationStarting::<Impl, IMPL_OFFSET>,
            RemovePrimaryViewAnimationStarting: RemovePrimaryViewAnimationStarting::<Impl, IMPL_OFFSET>,
            OcclusionsChanged: OcclusionsChanged::<Impl, IMPL_OFFSET>,
            RemoveOcclusionsChanged: RemoveOcclusionsChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreFrameworkInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewAnimationStartingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreFrameworkInputViewAnimationStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFrameworkInputViewAnimationStartingEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameworkAnimationRecommended<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameworkAnimationRecommended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnimationDuration<Impl: ICoreFrameworkInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreFrameworkInputViewAnimationStartingEventArgs, BASE_OFFSET>(),
            Occlusions: Occlusions::<Impl, IMPL_OFFSET>,
            FrameworkAnimationRecommended: FrameworkAnimationRecommended::<Impl, IMPL_OFFSET>,
            AnimationDuration: AnimationDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewAnimationStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreFrameworkInputViewOcclusionsChangedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFrameworkInputViewOcclusionsChangedEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreFrameworkInputViewOcclusionsChangedEventArgs, BASE_OFFSET>(),
            Occlusions: Occlusions::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewOcclusionsChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFrameworkInputViewStaticsVtbl {
        unsafe extern "system" fn GetForUIContext<Impl: ICoreFrameworkInputViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreFrameworkInputViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreFrameworkInputViewStatics, BASE_OFFSET>(),
            GetForUIContext: GetForUIContext::<Impl, IMPL_OFFSET>,
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreInputViewImpl: Sized {
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool>;
    fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreInputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewVtbl {
        unsafe extern "system" fn OcclusionsChanged<Impl: ICoreInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcclusionsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOcclusionsChanged<Impl: ICoreInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOcclusionsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCoreInputViewOcclusions<Impl: ICoreInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCoreInputViewOcclusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowPrimaryView<Impl: ICoreInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowPrimaryView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryHidePrimaryView<Impl: ICoreInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryHidePrimaryView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputView, BASE_OFFSET>(),
            OcclusionsChanged: OcclusionsChanged::<Impl, IMPL_OFFSET>,
            RemoveOcclusionsChanged: RemoveOcclusionsChanged::<Impl, IMPL_OFFSET>,
            GetCoreInputViewOcclusions: GetCoreInputViewOcclusions::<Impl, IMPL_OFFSET>,
            TryShowPrimaryView: TryShowPrimaryView::<Impl, IMPL_OFFSET>,
            TryHidePrimaryView: TryHidePrimaryView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreInputView2Impl: Sized {
    fn XYFocusTransferringFromPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferringFromPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XYFocusTransferredToPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferredToPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryTransferXYFocusToPrimaryView(&self, origin: &super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputView2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreInputView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputView2Vtbl {
        unsafe extern "system" fn XYFocusTransferringFromPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusTransferringFromPrimaryView(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveXYFocusTransferringFromPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveXYFocusTransferringFromPrimaryView(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusTransferredToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusTransferredToPrimaryView(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveXYFocusTransferredToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveXYFocusTransferredToPrimaryView(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryTransferXYFocusToPrimaryView<Impl: ICoreInputView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTransferXYFocusToPrimaryView(&*(&origin as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputView2, BASE_OFFSET>(),
            XYFocusTransferringFromPrimaryView: XYFocusTransferringFromPrimaryView::<Impl, IMPL_OFFSET>,
            RemoveXYFocusTransferringFromPrimaryView: RemoveXYFocusTransferringFromPrimaryView::<Impl, IMPL_OFFSET>,
            XYFocusTransferredToPrimaryView: XYFocusTransferredToPrimaryView::<Impl, IMPL_OFFSET>,
            RemoveXYFocusTransferredToPrimaryView: RemoveXYFocusTransferredToPrimaryView::<Impl, IMPL_OFFSET>,
            TryTransferXYFocusToPrimaryView: TryTransferXYFocusToPrimaryView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputView2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputView3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputView3Vtbl {
        unsafe extern "system" fn TryShow<Impl: ICoreInputView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowWithKind<Impl: ICoreInputView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowWithKind(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryHide<Impl: ICoreInputView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryHide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputView3, BASE_OFFSET>(),
            TryShow: TryShow::<Impl, IMPL_OFFSET>,
            TryShowWithKind: TryShowWithKind::<Impl, IMPL_OFFSET>,
            TryHide: TryHide::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreInputView4Impl: Sized {
    fn PrimaryViewShowing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewShowing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewHiding(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewHiding(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputView4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreInputView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputView4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputView4Vtbl {
        unsafe extern "system" fn PrimaryViewShowing<Impl: ICoreInputView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryViewShowing(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewShowing<Impl: ICoreInputView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewShowing(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryViewHiding<Impl: ICoreInputView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryViewHiding(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewHiding<Impl: ICoreInputView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewHiding(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputView4, BASE_OFFSET>(),
            PrimaryViewShowing: PrimaryViewShowing::<Impl, IMPL_OFFSET>,
            RemovePrimaryViewShowing: RemovePrimaryViewShowing::<Impl, IMPL_OFFSET>,
            PrimaryViewHiding: PrimaryViewHiding::<Impl, IMPL_OFFSET>,
            RemovePrimaryViewHiding: RemovePrimaryViewHiding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputView4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreInputView5Impl: Sized {
    fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool>;
    fn SupportedKindsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedKindsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputView5 {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputView5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreInputView5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputView5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputView5Vtbl {
        unsafe extern "system" fn IsKindSupported<Impl: ICoreInputView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKindSupported(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedKindsChanged<Impl: ICoreInputView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedKindsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSupportedKindsChanged<Impl: ICoreInputView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSupportedKindsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryViewAnimationStarting<Impl: ICoreInputView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryViewAnimationStarting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrimaryViewAnimationStarting<Impl: ICoreInputView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrimaryViewAnimationStarting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputView5, BASE_OFFSET>(),
            IsKindSupported: IsKindSupported::<Impl, IMPL_OFFSET>,
            SupportedKindsChanged: SupportedKindsChanged::<Impl, IMPL_OFFSET>,
            RemoveSupportedKindsChanged: RemoveSupportedKindsChanged::<Impl, IMPL_OFFSET>,
            PrimaryViewAnimationStarting: PrimaryViewAnimationStarting::<Impl, IMPL_OFFSET>,
            RemovePrimaryViewAnimationStarting: RemovePrimaryViewAnimationStarting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputView5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewAnimationStartingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreInputViewAnimationStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewAnimationStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewAnimationStartingEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn AnimationDuration<Impl: ICoreInputViewAnimationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewAnimationStartingEventArgs, BASE_OFFSET>(),
            Occlusions: Occlusions::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            AnimationDuration: AnimationDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewAnimationStartingEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewHidingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewHidingEventArgsVtbl {
        unsafe extern "system" fn TryCancel<Impl: ICoreInputViewHidingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewHidingEventArgs, BASE_OFFSET>(),
            TryCancel: TryCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewHidingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreInputViewOcclusionImpl: Sized {
    fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewOcclusion";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreInputViewOcclusionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewOcclusionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewOcclusionVtbl {
        unsafe extern "system" fn OccludingRect<Impl: ICoreInputViewOcclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OccludingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcclusionKind<Impl: ICoreInputViewOcclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewOcclusionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcclusionKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewOcclusion, BASE_OFFSET>(),
            OccludingRect: OccludingRect::<Impl, IMPL_OFFSET>,
            OcclusionKind: OcclusionKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewOcclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewOcclusionsChangedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreInputViewOcclusionsChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewOcclusionsChangedEventArgsVtbl {
        unsafe extern "system" fn Occlusions<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occlusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICoreInputViewOcclusionsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewOcclusionsChangedEventArgs, BASE_OFFSET>(),
            Occlusions: Occlusions::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewOcclusionsChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewShowingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewShowingEventArgsVtbl {
        unsafe extern "system" fn TryCancel<Impl: ICoreInputViewShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewShowingEventArgs, BASE_OFFSET>(),
            TryCancel: TryCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewShowingEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreInputViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewStatics2Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: ICoreInputViewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::UIContext as ::windows::core::Abi>::Abi as *const <super::super::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewStatics2, BASE_OFFSET>(),
            GetForUIContext: GetForUIContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreInputViewTransferringXYFocusEventArgsImpl: Sized {
    fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection>;
    fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransferHandled(&self) -> ::windows::core::Result<bool>;
    fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.ICoreInputViewTransferringXYFocusEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreInputViewTransferringXYFocusEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputViewTransferringXYFocusEventArgsVtbl {
        unsafe extern "system" fn Origin<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferHandled<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferHandled(value).into()
        }
        unsafe extern "system" fn TransferHandled<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferHandled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepPrimaryViewVisible<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepPrimaryViewVisible(value).into()
        }
        unsafe extern "system" fn KeepPrimaryViewVisible<Impl: ICoreInputViewTransferringXYFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepPrimaryViewVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputViewTransferringXYFocusEventArgs, BASE_OFFSET>(),
            Origin: Origin::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetTransferHandled: SetTransferHandled::<Impl, IMPL_OFFSET>,
            TransferHandled: TransferHandled::<Impl, IMPL_OFFSET>,
            SetKeepPrimaryViewVisible: SetKeepPrimaryViewVisible::<Impl, IMPL_OFFSET>,
            KeepPrimaryViewVisible: KeepPrimaryViewVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputViewTransferringXYFocusEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsControllerVtbl {
        unsafe extern "system" fn SetAdvancedEffectsEnabled<Impl: IUISettingsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvancedEffectsEnabled(value).into()
        }
        unsafe extern "system" fn SetAnimationsEnabled<Impl: IUISettingsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationsEnabled(value).into()
        }
        unsafe extern "system" fn SetAutoHideScrollBars<Impl: IUISettingsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoHideScrollBars(value).into()
        }
        unsafe extern "system" fn SetMessageDuration<Impl: IUISettingsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageDuration(value).into()
        }
        unsafe extern "system" fn SetTextScaleFactor<Impl: IUISettingsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextScaleFactor(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsController, BASE_OFFSET>(),
            SetAdvancedEffectsEnabled: SetAdvancedEffectsEnabled::<Impl, IMPL_OFFSET>,
            SetAnimationsEnabled: SetAnimationsEnabled::<Impl, IMPL_OFFSET>,
            SetAutoHideScrollBars: SetAutoHideScrollBars::<Impl, IMPL_OFFSET>,
            SetMessageDuration: SetMessageDuration::<Impl, IMPL_OFFSET>,
            SetTextScaleFactor: SetTextScaleFactor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettingsControllerStaticsImpl: Sized {
    fn RequestDefaultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettingsControllerStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.IUISettingsControllerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettingsControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsControllerStaticsVtbl {
        unsafe extern "system" fn RequestDefaultAsync<Impl: IUISettingsControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsControllerStatics, BASE_OFFSET>(),
            RequestDefaultAsync: RequestDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsControllerStatics as ::windows::core::Interface>::IID
    }
}
