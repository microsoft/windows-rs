#[cfg(feature = "implement_exclusive")]
pub trait ILocalContentSuggestionSettingsImpl: Sized {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>>;
    fn SetAqsFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AqsFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PropertiesToMatch(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ILocalContentSuggestionSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalContentSuggestionSettingsVtbl {
    pub const fn new<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILocalContentSuggestionSettingsVtbl {
        unsafe extern "system" fn SetEnabled<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAqsFilter<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAqsFilter(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AqsFilter<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AqsFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertiesToMatch<Impl: ILocalContentSuggestionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertiesToMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILocalContentSuggestionSettings>, base.5, SetEnabled::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, Locations::<Impl, OFFSET>, SetAqsFilter::<Impl, OFFSET>, AqsFilter::<Impl, OFFSET>, PropertiesToMatch::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneImpl: Sized {
    fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SearchHistoryEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSearchHistoryContext(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SearchHistoryContext(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QueryChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQueryChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuggestionsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuggestionsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuerySubmitted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuerySubmitted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResultSuggestionChosen(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResultSuggestionChosen(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetLocalContentSuggestionSettings(&self, settings: &::core::option::Option<LocalContentSuggestionSettings>) -> ::windows::core::Result<()>;
    fn ShowOverloadDefault(&self) -> ::windows::core::Result<()>;
    fn ShowOverloadWithQuery(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowOnKeyboardInput(&self) -> ::windows::core::Result<bool>;
    fn TrySetQueryText(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPane";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneVtbl {
    pub const fn new<Impl: ISearchPaneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneVtbl {
        unsafe extern "system" fn SetSearchHistoryEnabled<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSearchHistoryEnabled(value).into()
        }
        unsafe extern "system" fn SearchHistoryEnabled<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchHistoryEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchHistoryContext<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSearchHistoryContext(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SearchHistoryContext<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchHistoryContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaceholderText<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaceholderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderText<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryText<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibilityChanged<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QueryChanged<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveQueryChanged<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveQueryChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestionsRequested<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestionsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuggestionsRequested<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSuggestionsRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuerySubmitted<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySubmitted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveQuerySubmitted<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveQuerySubmitted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResultSuggestionChosen<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultSuggestionChosen(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResultSuggestionChosen<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveResultSuggestionChosen(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetLocalContentSuggestionSettings<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalContentSuggestionSettings(&*(&settings as *const <LocalContentSuggestionSettings as ::windows::core::Abi>::Abi as *const <LocalContentSuggestionSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowOverloadDefault<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowOverloadDefault().into()
        }
        unsafe extern "system" fn ShowOverloadWithQuery<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowOverloadWithQuery(&*(&query as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetShowOnKeyboardInput<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowOnKeyboardInput(value).into()
        }
        unsafe extern "system" fn ShowOnKeyboardInput<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowOnKeyboardInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetQueryText<Impl: ISearchPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetQueryText(&*(&query as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISearchPane>,
            base.5,
            SetSearchHistoryEnabled::<Impl, OFFSET>,
            SearchHistoryEnabled::<Impl, OFFSET>,
            SetSearchHistoryContext::<Impl, OFFSET>,
            SearchHistoryContext::<Impl, OFFSET>,
            SetPlaceholderText::<Impl, OFFSET>,
            PlaceholderText::<Impl, OFFSET>,
            QueryText::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            Visible::<Impl, OFFSET>,
            VisibilityChanged::<Impl, OFFSET>,
            RemoveVisibilityChanged::<Impl, OFFSET>,
            QueryChanged::<Impl, OFFSET>,
            RemoveQueryChanged::<Impl, OFFSET>,
            SuggestionsRequested::<Impl, OFFSET>,
            RemoveSuggestionsRequested::<Impl, OFFSET>,
            QuerySubmitted::<Impl, OFFSET>,
            RemoveQuerySubmitted::<Impl, OFFSET>,
            ResultSuggestionChosen::<Impl, OFFSET>,
            RemoveResultSuggestionChosen::<Impl, OFFSET>,
            SetLocalContentSuggestionSettings::<Impl, OFFSET>,
            ShowOverloadDefault::<Impl, OFFSET>,
            ShowOverloadWithQuery::<Impl, OFFSET>,
            SetShowOnKeyboardInput::<Impl, OFFSET>,
            ShowOnKeyboardInput::<Impl, OFFSET>,
            TrySetQueryText::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ISearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgsVtbl {
    pub const fn new<Impl: ISearchPaneQueryChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneQueryChangedEventArgsVtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchPaneQueryChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchPaneQueryChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchPaneQueryChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneQueryChangedEventArgs>, base.5, QueryText::<Impl, OFFSET>, Language::<Impl, OFFSET>, LinguisticDetails::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchPaneQueryLinguisticDetailsImpl: Sized {
    fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32>;
    fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryLinguisticDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchPaneQueryLinguisticDetailsVtbl {
    pub const fn new<Impl: ISearchPaneQueryLinguisticDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneQueryLinguisticDetailsVtbl {
        unsafe extern "system" fn QueryTextAlternatives<Impl: ISearchPaneQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextAlternatives() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTextCompositionStart<Impl: ISearchPaneQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextCompositionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTextCompositionLength<Impl: ISearchPaneQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextCompositionLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneQueryLinguisticDetails>, base.5, QueryTextAlternatives::<Impl, OFFSET>, QueryTextCompositionStart::<Impl, OFFSET>, QueryTextCompositionLength::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneQuerySubmittedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneQuerySubmittedEventArgsVtbl {
    pub const fn new<Impl: ISearchPaneQuerySubmittedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneQuerySubmittedEventArgsVtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchPaneQuerySubmittedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchPaneQuerySubmittedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneQuerySubmittedEventArgs>, base.5, QueryText::<Impl, OFFSET>, Language::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl {
    pub const fn new<Impl: ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl {
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>, base.5, LinguisticDetails::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneResultSuggestionChosenEventArgsImpl: Sized {
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneResultSuggestionChosenEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneResultSuggestionChosenEventArgsVtbl {
    pub const fn new<Impl: ISearchPaneResultSuggestionChosenEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneResultSuggestionChosenEventArgsVtbl {
        unsafe extern "system" fn Tag<Impl: ISearchPaneResultSuggestionChosenEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneResultSuggestionChosenEventArgs>, base.5, Tag::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SearchPane>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneStaticsVtbl {
    pub const fn new<Impl: ISearchPaneStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISearchPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneStaticsWithHideThisApplicationImpl: Sized {
    fn HideThisApplication(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneStaticsWithHideThisApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneStaticsWithHideThisApplication";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneStaticsWithHideThisApplicationVtbl {
    pub const fn new<Impl: ISearchPaneStaticsWithHideThisApplicationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneStaticsWithHideThisApplicationVtbl {
        unsafe extern "system" fn HideThisApplication<Impl: ISearchPaneStaticsWithHideThisApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).HideThisApplication().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneStaticsWithHideThisApplication>, base.5, HideThisApplication::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection>;
    fn GetDeferral(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequestDeferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequest";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneSuggestionsRequestVtbl {
    pub const fn new<Impl: ISearchPaneSuggestionsRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneSuggestionsRequestVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISearchPaneSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchSuggestionCollection<Impl: ISearchPaneSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchSuggestionCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ISearchPaneSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneSuggestionsRequest>, base.5, IsCanceled::<Impl, OFFSET>, SearchSuggestionCollection::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestDeferral";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneSuggestionsRequestDeferralVtbl {
    pub const fn new<Impl: ISearchPaneSuggestionsRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneSuggestionsRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISearchPaneSuggestionsRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneSuggestionsRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestedEventArgsImpl: Sized + ISearchPaneQueryChangedEventArgsImpl {
    fn Request(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneSuggestionsRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneSuggestionsRequestedEventArgsVtbl {
    pub const fn new<Impl: ISearchPaneSuggestionsRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneSuggestionsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ISearchPaneSuggestionsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneSuggestionsRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneVisibilityChangedEventArgsImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneVisibilityChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISearchPaneVisibilityChangedEventArgsVtbl {
    pub const fn new<Impl: ISearchPaneVisibilityChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchPaneVisibilityChangedEventArgsVtbl {
        unsafe extern "system" fn Visible<Impl: ISearchPaneVisibilityChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchPaneVisibilityChangedEventArgs>, base.5, Visible::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchQueryLinguisticDetailsImpl: Sized {
    fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32>;
    fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchQueryLinguisticDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchQueryLinguisticDetailsVtbl {
    pub const fn new<Impl: ISearchQueryLinguisticDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchQueryLinguisticDetailsVtbl {
        unsafe extern "system" fn QueryTextAlternatives<Impl: ISearchQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextAlternatives() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTextCompositionStart<Impl: ISearchQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextCompositionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryTextCompositionLength<Impl: ISearchQueryLinguisticDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryTextCompositionLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchQueryLinguisticDetails>, base.5, QueryTextAlternatives::<Impl, OFFSET>, QueryTextCompositionStart::<Impl, OFFSET>, QueryTextCompositionLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchQueryLinguisticDetailsFactoryImpl: Sized {
    fn CreateInstance(&self, querytextalternatives: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows::core::Result<SearchQueryLinguisticDetails>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchQueryLinguisticDetailsFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchQueryLinguisticDetailsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchQueryLinguisticDetailsFactoryVtbl {
    pub const fn new<Impl: ISearchQueryLinguisticDetailsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchQueryLinguisticDetailsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISearchQueryLinguisticDetailsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querytextalternatives: ::windows::core::RawPtr, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&querytextalternatives as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), querytextcompositionstart, querytextcompositionlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchQueryLinguisticDetailsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionCollectionImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn AppendQuerySuggestion(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppendQuerySuggestions(&self, suggestions: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn AppendResultSuggestion(&self, text: &::windows::core::HSTRING, detailtext: &::windows::core::HSTRING, tag: &::windows::core::HSTRING, image: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, imagealternatetext: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppendSearchSeparator(&self, label: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchSuggestionCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchSuggestionCollectionVtbl {
    pub const fn new<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchSuggestionCollectionVtbl {
        unsafe extern "system" fn Size<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendQuerySuggestion<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendQuerySuggestion(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendQuerySuggestions<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, suggestions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendQuerySuggestions(&*(&suggestions as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendResultSuggestion<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, image: ::windows::core::RawPtr, imagealternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .AppendResultSuggestion(
                    &*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&detailtext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&image as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
                    &*(&imagealternatetext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn AppendSearchSeparator<Impl: ISearchSuggestionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendSearchSeparator(&*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchSuggestionCollection>, base.5, Size::<Impl, OFFSET>, AppendQuerySuggestion::<Impl, OFFSET>, AppendQuerySuggestions::<Impl, OFFSET>, AppendResultSuggestion::<Impl, OFFSET>, AppendSearchSeparator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionsRequestImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection>;
    fn GetDeferral(&self) -> ::windows::core::Result<SearchSuggestionsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchSuggestionsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchSuggestionsRequestVtbl {
    pub const fn new<Impl: ISearchSuggestionsRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchSuggestionsRequestVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISearchSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchSuggestionCollection<Impl: ISearchSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchSuggestionCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ISearchSuggestionsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchSuggestionsRequest>, base.5, IsCanceled::<Impl, OFFSET>, SearchSuggestionCollection::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchSuggestionsRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchSuggestionsRequestDeferralVtbl {
    pub const fn new<Impl: ISearchSuggestionsRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchSuggestionsRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISearchSuggestionsRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchSuggestionsRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
