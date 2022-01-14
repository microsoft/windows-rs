#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreTextCompositionCompletedEventArgs_Impl: Sized {
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn CompositionSegments(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreTextCompositionCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextCompositionCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextCompositionCompletedEventArgs_Vtbl {
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextCompositionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompositionSegments<Impl: ICoreTextCompositionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionSegments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextCompositionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextCompositionCompletedEventArgs, BASE_OFFSET>(),
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            CompositionSegments: CompositionSegments::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextCompositionCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionSegment_Impl: Sized {
    fn PreconversionString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Range(&mut self) -> ::windows::core::Result<CoreTextRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionSegment";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextCompositionSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextCompositionSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextCompositionSegment_Vtbl {
        unsafe extern "system" fn PreconversionString<Impl: ICoreTextCompositionSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreconversionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Impl: ICoreTextCompositionSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextCompositionSegment, BASE_OFFSET>(),
            PreconversionString: PreconversionString::<Impl, IMPL_OFFSET>,
            Range: Range::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextCompositionSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextCompositionStartedEventArgs_Impl: Sized {
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextCompositionStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextCompositionStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextCompositionStartedEventArgs_Vtbl {
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextCompositionStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextCompositionStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextCompositionStartedEventArgs, BASE_OFFSET>(),
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextCompositionStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextEditContext_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InputScope(&mut self) -> ::windows::core::Result<CoreTextInputScope>;
    fn SetInputScope(&mut self, value: CoreTextInputScope) -> ::windows::core::Result<()>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InputPaneDisplayPolicy(&mut self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy>;
    fn SetInputPaneDisplayPolicy(&mut self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()>;
    fn TextRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextRequested(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionRequested(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutRequested(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextUpdating(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextUpdating(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionUpdating(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionUpdating(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FormatUpdating(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatUpdating(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionStarted(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionCompleted(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FocusRemoved(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusRemoved(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyFocusEnter(&mut self) -> ::windows::core::Result<()>;
    fn NotifyFocusLeave(&mut self) -> ::windows::core::Result<()>;
    fn NotifyTextChanged(&mut self, modifiedrange: &CoreTextRange, newlength: i32, newselection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifySelectionChanged(&mut self, selection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifyLayoutChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextEditContext";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextEditContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextEditContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextEditContext_Vtbl {
        unsafe extern "system" fn Name<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputScope<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputScope<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputScope(value).into()
        }
        unsafe extern "system" fn IsReadOnly<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsReadOnly<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsReadOnly(value).into()
        }
        unsafe extern "system" fn InputPaneDisplayPolicy<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputPaneDisplayPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputPaneDisplayPolicy<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputPaneDisplayPolicy(value).into()
        }
        unsafe extern "system" fn TextRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTextRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectionRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LayoutRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayoutRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLayoutRequested<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLayoutRequested(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTextUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectionUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FormatUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatUpdating(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFormatUpdating<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFormatUpdating(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositionStarted<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompositionStarted<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompositionStarted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositionCompleted<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompositionCompleted<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompositionCompleted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusRemoved<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusRemoved(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFocusRemoved<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFocusRemoved(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyFocusEnter<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyFocusEnter().into()
        }
        unsafe extern "system" fn NotifyFocusLeave<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyFocusLeave().into()
        }
        unsafe extern "system" fn NotifyTextChanged<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyTextChanged(&*(&modifiedrange as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType), newlength, &*(&newselection as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifySelectionChanged<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySelectionChanged(&*(&selection as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyLayoutChanged<Impl: ICoreTextEditContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyLayoutChanged().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextEditContext, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            InputScope: InputScope::<Impl, IMPL_OFFSET>,
            SetInputScope: SetInputScope::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            SetIsReadOnly: SetIsReadOnly::<Impl, IMPL_OFFSET>,
            InputPaneDisplayPolicy: InputPaneDisplayPolicy::<Impl, IMPL_OFFSET>,
            SetInputPaneDisplayPolicy: SetInputPaneDisplayPolicy::<Impl, IMPL_OFFSET>,
            TextRequested: TextRequested::<Impl, IMPL_OFFSET>,
            RemoveTextRequested: RemoveTextRequested::<Impl, IMPL_OFFSET>,
            SelectionRequested: SelectionRequested::<Impl, IMPL_OFFSET>,
            RemoveSelectionRequested: RemoveSelectionRequested::<Impl, IMPL_OFFSET>,
            LayoutRequested: LayoutRequested::<Impl, IMPL_OFFSET>,
            RemoveLayoutRequested: RemoveLayoutRequested::<Impl, IMPL_OFFSET>,
            TextUpdating: TextUpdating::<Impl, IMPL_OFFSET>,
            RemoveTextUpdating: RemoveTextUpdating::<Impl, IMPL_OFFSET>,
            SelectionUpdating: SelectionUpdating::<Impl, IMPL_OFFSET>,
            RemoveSelectionUpdating: RemoveSelectionUpdating::<Impl, IMPL_OFFSET>,
            FormatUpdating: FormatUpdating::<Impl, IMPL_OFFSET>,
            RemoveFormatUpdating: RemoveFormatUpdating::<Impl, IMPL_OFFSET>,
            CompositionStarted: CompositionStarted::<Impl, IMPL_OFFSET>,
            RemoveCompositionStarted: RemoveCompositionStarted::<Impl, IMPL_OFFSET>,
            CompositionCompleted: CompositionCompleted::<Impl, IMPL_OFFSET>,
            RemoveCompositionCompleted: RemoveCompositionCompleted::<Impl, IMPL_OFFSET>,
            FocusRemoved: FocusRemoved::<Impl, IMPL_OFFSET>,
            RemoveFocusRemoved: RemoveFocusRemoved::<Impl, IMPL_OFFSET>,
            NotifyFocusEnter: NotifyFocusEnter::<Impl, IMPL_OFFSET>,
            NotifyFocusLeave: NotifyFocusLeave::<Impl, IMPL_OFFSET>,
            NotifyTextChanged: NotifyTextChanged::<Impl, IMPL_OFFSET>,
            NotifySelectionChanged: NotifySelectionChanged::<Impl, IMPL_OFFSET>,
            NotifyLayoutChanged: NotifyLayoutChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextEditContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextEditContext2_Impl: Sized {
    fn NotifyFocusLeaveCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotifyFocusLeaveCompleted(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextEditContext2 {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextEditContext2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextEditContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextEditContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextEditContext2_Vtbl {
        unsafe extern "system" fn NotifyFocusLeaveCompleted<Impl: ICoreTextEditContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyFocusLeaveCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotifyFocusLeaveCompleted<Impl: ICoreTextEditContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNotifyFocusLeaveCompleted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextEditContext2, BASE_OFFSET>(),
            NotifyFocusLeaveCompleted: NotifyFocusLeaveCompleted::<Impl, IMPL_OFFSET>,
            RemoveNotifyFocusLeaveCompleted: RemoveNotifyFocusLeaveCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextEditContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_ViewManagement", feature = "implement_exclusive"))]
pub trait ICoreTextFormatUpdatingEventArgs_Impl: Sized {
    fn Range(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn TextColor(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineColor(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>>;
    fn Reason(&mut self) -> ::windows::core::Result<CoreTextFormatUpdatingReason>;
    fn Result(&mut self) -> ::windows::core::Result<CoreTextFormatUpdatingResult>;
    fn SetResult(&mut self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "UI_ViewManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "UI_ViewManagement", feature = "implement_exclusive"))]
impl ICoreTextFormatUpdatingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextFormatUpdatingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextFormatUpdatingEventArgs_Vtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextColor<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundColor<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlineColor<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnderlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlineType<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnderlineType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextFormatUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextFormatUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextFormatUpdatingEventArgs, BASE_OFFSET>(),
            Range: Range::<Impl, IMPL_OFFSET>,
            TextColor: TextColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            UnderlineColor: UnderlineColor::<Impl, IMPL_OFFSET>,
            UnderlineType: UnderlineType::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextFormatUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextLayoutBounds_Impl: Sized {
    fn TextBounds(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetTextBounds(&mut self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ControlBounds(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetControlBounds(&mut self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutBounds";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextLayoutBounds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextLayoutBounds_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextLayoutBounds_Vtbl {
        unsafe extern "system" fn TextBounds<Impl: ICoreTextLayoutBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextBounds<Impl: ICoreTextLayoutBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextBounds(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlBounds<Impl: ICoreTextLayoutBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlBounds<Impl: ICoreTextLayoutBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlBounds(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextLayoutBounds, BASE_OFFSET>(),
            TextBounds: TextBounds::<Impl, IMPL_OFFSET>,
            SetTextBounds: SetTextBounds::<Impl, IMPL_OFFSET>,
            ControlBounds: ControlBounds::<Impl, IMPL_OFFSET>,
            SetControlBounds: SetControlBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextLayoutBounds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextLayoutRequest_Impl: Sized {
    fn Range(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn LayoutBounds(&mut self) -> ::windows::core::Result<CoreTextLayoutBounds>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextLayoutRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextLayoutRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextLayoutRequest_Vtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextLayoutRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayoutBounds<Impl: ICoreTextLayoutRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayoutBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextLayoutRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextLayoutRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextLayoutRequest, BASE_OFFSET>(),
            Range: Range::<Impl, IMPL_OFFSET>,
            LayoutBounds: LayoutBounds::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextLayoutRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequest2_Impl: Sized {
    fn LayoutBoundsVisualPixels(&mut self) -> ::windows::core::Result<CoreTextLayoutBounds>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequest2 {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextLayoutRequest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextLayoutRequest2_Vtbl {
        unsafe extern "system" fn LayoutBoundsVisualPixels<Impl: ICoreTextLayoutRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayoutBoundsVisualPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextLayoutRequest2, BASE_OFFSET>(),
            LayoutBoundsVisualPixels: LayoutBoundsVisualPixels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextLayoutRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<CoreTextLayoutRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextLayoutRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextLayoutRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextLayoutRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextLayoutRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextLayoutRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextLayoutRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextSelectionRequest_Impl: Sized {
    fn Selection(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn SetSelection(&mut self, value: &CoreTextRange) -> ::windows::core::Result<()>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextSelectionRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextSelectionRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextSelectionRequest_Vtbl {
        unsafe extern "system" fn Selection<Impl: ICoreTextSelectionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ICoreTextSelectionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(&*(&value as *const <CoreTextRange as ::windows::core::Abi>::Abi as *const <CoreTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextSelectionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextSelectionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextSelectionRequest, BASE_OFFSET>(),
            Selection: Selection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextSelectionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<CoreTextSelectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextSelectionRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextSelectionRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextSelectionRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextSelectionRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextSelectionRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextSelectionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextSelectionUpdatingEventArgs_Impl: Sized {
    fn Selection(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn Result(&mut self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult>;
    fn SetResult(&mut self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextSelectionUpdatingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextSelectionUpdatingEventArgs_Vtbl {
        unsafe extern "system" fn Selection<Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextSelectionUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextSelectionUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextSelectionUpdatingEventArgs, BASE_OFFSET>(),
            Selection: Selection::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextSelectionUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
pub trait ICoreTextServicesManager_Impl: Sized {
    fn InputLanguage(&mut self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn InputLanguageChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputLanguageChanged(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEditContext(&mut self) -> ::windows::core::Result<CoreTextEditContext>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesManager";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ICoreTextServicesManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextServicesManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextServicesManager_Vtbl {
        unsafe extern "system" fn InputLanguage<Impl: ICoreTextServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputLanguageChanged<Impl: ICoreTextServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputLanguageChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputLanguageChanged<Impl: ICoreTextServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputLanguageChanged(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateEditContext<Impl: ICoreTextServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEditContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextServicesManager, BASE_OFFSET>(),
            InputLanguage: InputLanguage::<Impl, IMPL_OFFSET>,
            InputLanguageChanged: InputLanguageChanged::<Impl, IMPL_OFFSET>,
            RemoveInputLanguageChanged: RemoveInputLanguageChanged::<Impl, IMPL_OFFSET>,
            CreateEditContext: CreateEditContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextServicesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesManagerStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<CoreTextServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextServicesManagerStatics {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextServicesManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextServicesManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextServicesManagerStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreTextServicesManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextServicesManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextServicesManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesStatics_Impl: Sized {
    fn HiddenCharacter(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextServicesStatics {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextServicesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextServicesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextServicesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextServicesStatics_Vtbl {
        unsafe extern "system" fn HiddenCharacter<Impl: ICoreTextServicesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HiddenCharacter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextServicesStatics, BASE_OFFSET>(),
            HiddenCharacter: HiddenCharacter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextServicesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTextTextRequest_Impl: Sized {
    fn Range(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTextTextRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextTextRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextTextRequest_Vtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextTextRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ICoreTextTextRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ICoreTextTextRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextTextRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextTextRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextTextRequest, BASE_OFFSET>(),
            Range: Range::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextTextRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<CoreTextTextRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreTextTextRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextTextRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextTextRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: ICoreTextTextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextTextRequestedEventArgs, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextTextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
pub trait ICoreTextTextUpdatingEventArgs_Impl: Sized {
    fn Range(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewSelection(&mut self) -> ::windows::core::Result<CoreTextRange>;
    fn InputLanguage(&mut self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn Result(&mut self) -> ::windows::core::Result<CoreTextTextUpdatingResult>;
    fn SetResult(&mut self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ICoreTextTextUpdatingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTextTextUpdatingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTextTextUpdatingEventArgs_Vtbl {
        unsafe extern "system" fn Range<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewSelection<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputLanguage<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreTextTextUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreTextTextUpdatingResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(value).into()
        }
        unsafe extern "system" fn IsCanceled<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ICoreTextTextUpdatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTextTextUpdatingEventArgs, BASE_OFFSET>(),
            Range: Range::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            NewSelection: NewSelection::<Impl, IMPL_OFFSET>,
            InputLanguage: InputLanguage::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTextTextUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
