#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionCompletedEventArgsImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn CompositionSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextCompositionCompletedEventArgsVtbl {
    pub const fn new<Impl: ICoreTextCompositionCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextCompositionCompletedEventArgsVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextCompositionCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompositionSegments<Impl: ICoreTextCompositionCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositionSegments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextCompositionCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextCompositionCompletedEventArgs>, base.5, IsCanceled::<Impl, OFFSET>, CompositionSegments::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionSegmentImpl: Sized {
    fn PreconversionString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionSegment";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextCompositionSegmentVtbl {
    pub const fn new<Impl: ICoreTextCompositionSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextCompositionSegmentVtbl {
        unsafe extern "system" fn PreconversionString<Impl: ICoreTextCompositionSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreconversionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Impl: ICoreTextCompositionSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextCompositionSegment>, base.5, PreconversionString::<Impl, OFFSET>, Range::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionStartedEventArgsImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextCompositionStartedEventArgsVtbl {
    pub const fn new<Impl: ICoreTextCompositionStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextCompositionStartedEventArgsVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextCompositionStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextCompositionStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextCompositionStartedEventArgs>, base.5, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextEditContextImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InputScope(&self) -> ::windows::core::Result<CoreTextInputScope>;
    fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::core::Result<()>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputPaneDisplayPolicy(&self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy>;
    fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()>;
    fn TextRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FormatUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionStarted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionCompleted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FocusRemoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusRemoved(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyFocusEnter(&self) -> ::windows::core::Result<()>;
    fn NotifyFocusLeave(&self) -> ::windows::core::Result<()>;
    fn NotifyTextChanged(&self, modifiedrange: &CoreTextRange, newlength: i32, newselection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifySelectionChanged(&self, selection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifyLayoutChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextEditContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextEditContextVtbl {
    pub const fn new<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextEditContextVtbl {
        unsafe extern "system" fn Name<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputScope<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputScope<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInputScope(value).into()
        }
        unsafe extern "system" fn IsReadOnly<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsReadOnly<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsReadOnly(value).into()
        }
        unsafe extern "system" fn InputPaneDisplayPolicy<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputPaneDisplayPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputPaneDisplayPolicy<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInputPaneDisplayPolicy(value).into()
        }
        unsafe extern "system" fn TextRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTextRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSelectionRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LayoutRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayoutRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLayoutRequested<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLayoutRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTextUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSelectionUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FormatUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFormatUpdating<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFormatUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositionStarted<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositionStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompositionStarted<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompositionStarted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositionCompleted<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositionCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompositionCompleted<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompositionCompleted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusRemoved<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusRemoved(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFocusRemoved<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFocusRemoved(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyFocusEnter<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyFocusEnter().into()
        }
        unsafe extern "system" fn NotifyFocusLeave<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyFocusLeave().into()
        }
        unsafe extern "system" fn NotifyTextChanged<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyTextChanged(&*(&modifiedrange as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType), newlength, &*(&newselection as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifySelectionChanged<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selection: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifySelectionChanged(&*(&selection as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyLayoutChanged<Impl: ICoreTextEditContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyLayoutChanged().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICoreTextEditContext>,
            base.5,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            InputScope::<Impl, OFFSET>,
            SetInputScope::<Impl, OFFSET>,
            IsReadOnly::<Impl, OFFSET>,
            SetIsReadOnly::<Impl, OFFSET>,
            InputPaneDisplayPolicy::<Impl, OFFSET>,
            SetInputPaneDisplayPolicy::<Impl, OFFSET>,
            TextRequested::<Impl, OFFSET>,
            RemoveTextRequested::<Impl, OFFSET>,
            SelectionRequested::<Impl, OFFSET>,
            RemoveSelectionRequested::<Impl, OFFSET>,
            LayoutRequested::<Impl, OFFSET>,
            RemoveLayoutRequested::<Impl, OFFSET>,
            TextUpdating::<Impl, OFFSET>,
            RemoveTextUpdating::<Impl, OFFSET>,
            SelectionUpdating::<Impl, OFFSET>,
            RemoveSelectionUpdating::<Impl, OFFSET>,
            FormatUpdating::<Impl, OFFSET>,
            RemoveFormatUpdating::<Impl, OFFSET>,
            CompositionStarted::<Impl, OFFSET>,
            RemoveCompositionStarted::<Impl, OFFSET>,
            CompositionCompleted::<Impl, OFFSET>,
            RemoveCompositionCompleted::<Impl, OFFSET>,
            FocusRemoved::<Impl, OFFSET>,
            RemoveFocusRemoved::<Impl, OFFSET>,
            NotifyFocusEnter::<Impl, OFFSET>,
            NotifyFocusLeave::<Impl, OFFSET>,
            NotifyTextChanged::<Impl, OFFSET>,
            NotifySelectionChanged::<Impl, OFFSET>,
            NotifyLayoutChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextEditContext2Impl: Sized {
    fn NotifyFocusLeaveCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotifyFocusLeaveCompleted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextEditContext2 {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextEditContext2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextEditContext2Vtbl {
    pub const fn new<Impl: ICoreTextEditContext2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextEditContext2Vtbl {
        unsafe extern "system" fn NotifyFocusLeaveCompleted<Impl: ICoreTextEditContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotifyFocusLeaveCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotifyFocusLeaveCompleted<Impl: ICoreTextEditContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNotifyFocusLeaveCompleted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextEditContext2>, base.5, NotifyFocusLeaveCompleted::<Impl, OFFSET>, RemoveNotifyFocusLeaveCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextFormatUpdatingEventArgsImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn TextColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>>;
    fn Reason(&self) -> ::windows::core::Result<CoreTextFormatUpdatingReason>;
    fn Result(&self) -> ::windows::core::Result<CoreTextFormatUpdatingResult>;
    fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextFormatUpdatingEventArgsVtbl {
    pub const fn new<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextFormatUpdatingEventArgsVtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextColor<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundColor<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlineColor<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnderlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlineType<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnderlineType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextFormatUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextFormatUpdatingEventArgs>, base.5, Range::<Impl, OFFSET>, TextColor::<Impl, OFFSET>, BackgroundColor::<Impl, OFFSET>, UnderlineColor::<Impl, OFFSET>, UnderlineType::<Impl, OFFSET>, Reason::<Impl, OFFSET>, Result::<Impl, OFFSET>, SetResult::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutBoundsImpl: Sized {
    fn TextBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetTextBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ControlBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetControlBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutBounds";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutBoundsVtbl {
    pub const fn new<Impl: ICoreTextLayoutBoundsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextLayoutBoundsVtbl {
        unsafe extern "system" fn TextBounds<Impl: ICoreTextLayoutBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextBounds<Impl: ICoreTextLayoutBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextBounds(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlBounds<Impl: ICoreTextLayoutBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlBounds<Impl: ICoreTextLayoutBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetControlBounds(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextLayoutBounds>, base.5, TextBounds::<Impl, OFFSET>, SetTextBounds::<Impl, OFFSET>, ControlBounds::<Impl, OFFSET>, SetControlBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequestImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn LayoutBounds(&self) -> ::windows::core::Result<CoreTextLayoutBounds>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutRequestVtbl {
    pub const fn new<Impl: ICoreTextLayoutRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextLayoutRequestVtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextLayoutRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayoutBounds<Impl: ICoreTextLayoutRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayoutBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextLayoutRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextLayoutRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextLayoutRequest>, base.5, Range::<Impl, OFFSET>, LayoutBounds::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequest2Impl: Sized {
    fn LayoutBoundsVisualPixels(&self) -> ::windows::core::Result<CoreTextLayoutBounds>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequest2 {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutRequest2Vtbl {
    pub const fn new<Impl: ICoreTextLayoutRequest2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextLayoutRequest2Vtbl {
        unsafe extern "system" fn LayoutBoundsVisualPixels<Impl: ICoreTextLayoutRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayoutBoundsVisualPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextLayoutRequest2>, base.5, LayoutBoundsVisualPixels::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextLayoutRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutRequestedEventArgsVtbl {
    pub const fn new<Impl: ICoreTextLayoutRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextLayoutRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextLayoutRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextLayoutRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionRequestImpl: Sized {
    fn Selection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn SetSelection(&self, value: &CoreTextRange) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextSelectionRequestVtbl {
    pub const fn new<Impl: ICoreTextSelectionRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextSelectionRequestVtbl {
        unsafe extern "system" fn Selection<Impl: ICoreTextSelectionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ICoreTextSelectionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelection(&*(&value as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextSelectionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextSelectionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextSelectionRequest>, base.5, Selection::<Impl, OFFSET>, SetSelection::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextSelectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextSelectionRequestedEventArgsVtbl {
    pub const fn new<Impl: ICoreTextSelectionRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextSelectionRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextSelectionRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextSelectionRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionUpdatingEventArgsImpl: Sized {
    fn Selection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Result(&self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult>;
    fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextSelectionUpdatingEventArgsVtbl {
    pub const fn new<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextSelectionUpdatingEventArgsVtbl {
        unsafe extern "system" fn Selection<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextSelectionUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextSelectionUpdatingEventArgs>, base.5, Selection::<Impl, OFFSET>, Result::<Impl, OFFSET>, SetResult::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesManagerImpl: Sized {
    fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn InputLanguageChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputLanguageChanged(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEditContext(&self) -> ::windows::core::Result<CoreTextEditContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesManager";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextServicesManagerVtbl {
    pub const fn new<Impl: ICoreTextServicesManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextServicesManagerVtbl {
        unsafe extern "system" fn InputLanguage<Impl: ICoreTextServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputLanguageChanged<Impl: ICoreTextServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputLanguageChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputLanguageChanged<Impl: ICoreTextServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInputLanguageChanged(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateEditContext<Impl: ICoreTextServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEditContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextServicesManager>, base.5, InputLanguage::<Impl, OFFSET>, InputLanguageChanged::<Impl, OFFSET>, RemoveInputLanguageChanged::<Impl, OFFSET>, CreateEditContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreTextServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextServicesManagerStatics {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextServicesManagerStaticsVtbl {
    pub const fn new<Impl: ICoreTextServicesManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextServicesManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreTextServicesManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextServicesManagerStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesStaticsImpl: Sized {
    fn HiddenCharacter(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextServicesStatics {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextServicesStaticsVtbl {
    pub const fn new<Impl: ICoreTextServicesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextServicesStaticsVtbl {
        unsafe extern "system" fn HiddenCharacter<Impl: ICoreTextServicesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HiddenCharacter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextServicesStatics>, base.5, HiddenCharacter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextRequestImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextTextRequestVtbl {
    pub const fn new<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextTextRequestVtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextTextRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextTextRequest>, base.5, Range::<Impl, OFFSET>, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextTextRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextTextRequestedEventArgsVtbl {
    pub const fn new<Impl: ICoreTextTextRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextTextRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextTextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextTextRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextUpdatingEventArgsImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewSelection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn Result(&self) -> ::windows::core::Result<CoreTextTextUpdatingResult>;
    fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextTextUpdatingEventArgsVtbl {
    pub const fn new<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreTextTextUpdatingEventArgsVtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewSelection<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputLanguage<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextTextUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreTextTextUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextTextUpdatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreTextTextUpdatingEventArgs>, base.5, Range::<Impl, OFFSET>, Text::<Impl, OFFSET>, NewSelection::<Impl, OFFSET>, InputLanguage::<Impl, OFFSET>, Result::<Impl, OFFSET>, SetResult::<Impl, OFFSET>, IsCanceled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
