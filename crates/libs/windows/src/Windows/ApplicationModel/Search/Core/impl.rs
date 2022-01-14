#[cfg(feature = "implement_exclusive")]
pub trait IRequestingFocusOnKeyboardInputEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRequestingFocusOnKeyboardInputEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.IRequestingFocusOnKeyboardInputEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRequestingFocusOnKeyboardInputEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequestingFocusOnKeyboardInputEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequestingFocusOnKeyboardInputEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRequestingFocusOnKeyboardInputEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequestingFocusOnKeyboardInputEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISearchSuggestion_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<SearchSuggestionKind>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetailText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ImageAlternateText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchSuggestion {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.ISearchSuggestion";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISearchSuggestion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchSuggestion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchSuggestion_Vtbl {
        unsafe extern "system" fn Kind<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SearchSuggestionKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Tag<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DetailText<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageAlternateText<Impl: ISearchSuggestion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageAlternateText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchSuggestion, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            DetailText: DetailText::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            ImageAlternateText: ImageAlternateText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchSuggestion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISearchSuggestionManager_Impl: Sized {
    fn SearchHistoryEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetSearchHistoryEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SearchHistoryContext(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSearchHistoryContext(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetLocalContentSuggestionSettings(&mut self, settings: &::core::option::Option<super::LocalContentSuggestionSettings>) -> ::windows::core::Result<()>;
    fn SetQuery(&mut self, querytext: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetQueryWithLanguage(&mut self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetQueryWithSearchQueryLinguisticDetails(&mut self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING, linguisticdetails: &::core::option::Option<super::SearchQueryLinguisticDetails>) -> ::windows::core::Result<()>;
    fn Suggestions(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<SearchSuggestion>>;
    fn AddToHistory(&mut self, querytext: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddToHistoryWithLanguage(&mut self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClearHistory(&mut self) -> ::windows::core::Result<()>;
    fn SuggestionsRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuggestionsRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestingFocusOnKeyboardInput(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestingFocusOnKeyboardInput(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchSuggestionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.ISearchSuggestionManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISearchSuggestionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchSuggestionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchSuggestionManager_Vtbl {
        unsafe extern "system" fn SearchHistoryEnabled<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchHistoryEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchHistoryEnabled<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchHistoryEnabled(value).into()
        }
        unsafe extern "system" fn SearchHistoryContext<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchHistoryContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchHistoryContext<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchHistoryContext(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetLocalContentSuggestionSettings<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalContentSuggestionSettings(&*(&settings as *const <super::LocalContentSuggestionSettings as ::windows::core::Abi>::Abi as *const <super::LocalContentSuggestionSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetQuery<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuery(&*(&querytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetQueryWithLanguage<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQueryWithLanguage(&*(&querytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetQueryWithSearchQueryLinguisticDetails<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linguisticdetails: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetQueryWithSearchQueryLinguisticDetails(
                    &*(&querytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&linguisticdetails as *const <super::SearchQueryLinguisticDetails as ::windows::core::Abi>::Abi as *const <super::SearchQueryLinguisticDetails as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Suggestions<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suggestions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToHistory<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToHistory(&*(&querytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddToHistoryWithLanguage<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToHistoryWithLanguage(&*(&querytext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearHistory<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearHistory().into()
        }
        unsafe extern "system" fn SuggestionsRequested<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestionsRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuggestionsRequested<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSuggestionsRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestingFocusOnKeyboardInput<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestingFocusOnKeyboardInput(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRequestingFocusOnKeyboardInput<Impl: ISearchSuggestionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestingFocusOnKeyboardInput(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchSuggestionManager, BASE_OFFSET>(),
            SearchHistoryEnabled: SearchHistoryEnabled::<Impl, IMPL_OFFSET>,
            SetSearchHistoryEnabled: SetSearchHistoryEnabled::<Impl, IMPL_OFFSET>,
            SearchHistoryContext: SearchHistoryContext::<Impl, IMPL_OFFSET>,
            SetSearchHistoryContext: SetSearchHistoryContext::<Impl, IMPL_OFFSET>,
            SetLocalContentSuggestionSettings: SetLocalContentSuggestionSettings::<Impl, IMPL_OFFSET>,
            SetQuery: SetQuery::<Impl, IMPL_OFFSET>,
            SetQueryWithLanguage: SetQueryWithLanguage::<Impl, IMPL_OFFSET>,
            SetQueryWithSearchQueryLinguisticDetails: SetQueryWithSearchQueryLinguisticDetails::<Impl, IMPL_OFFSET>,
            Suggestions: Suggestions::<Impl, IMPL_OFFSET>,
            AddToHistory: AddToHistory::<Impl, IMPL_OFFSET>,
            AddToHistoryWithLanguage: AddToHistoryWithLanguage::<Impl, IMPL_OFFSET>,
            ClearHistory: ClearHistory::<Impl, IMPL_OFFSET>,
            SuggestionsRequested: SuggestionsRequested::<Impl, IMPL_OFFSET>,
            RemoveSuggestionsRequested: RemoveSuggestionsRequested::<Impl, IMPL_OFFSET>,
            RequestingFocusOnKeyboardInput: RequestingFocusOnKeyboardInput::<Impl, IMPL_OFFSET>,
            RemoveRequestingFocusOnKeyboardInput: RemoveRequestingFocusOnKeyboardInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchSuggestionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionsRequestedEventArgs_Impl: Sized {
    fn QueryText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&mut self) -> ::windows::core::Result<super::SearchQueryLinguisticDetails>;
    fn Request(&mut self) -> ::windows::core::Result<super::SearchSuggestionsRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.ISearchSuggestionsRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchSuggestionsRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchSuggestionsRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchSuggestionsRequestedEventArgs_Vtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchSuggestionsRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchSuggestionsRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchSuggestionsRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: ISearchSuggestionsRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchSuggestionsRequestedEventArgs, BASE_OFFSET>(),
            QueryText: QueryText::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            LinguisticDetails: LinguisticDetails::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchSuggestionsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
