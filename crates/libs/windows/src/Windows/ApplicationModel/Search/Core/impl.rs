#[cfg(feature = "implement_exclusive")]
pub trait IRequestingFocusOnKeyboardInputEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<SearchSuggestionKind>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetailText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Image(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ImageAlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionManagerImpl: Sized {
    fn SearchHistoryEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SearchHistoryContext(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSearchHistoryContext(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetLocalContentSuggestionSettings(&self, settings: &::core::option::Option<super::LocalContentSuggestionSettings>) -> ::windows::core::Result<()>;
    fn SetQuery(&self, querytext: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetQueryWithLanguage(&self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetQueryWithSearchQueryLinguisticDetails(&self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING, linguisticdetails: &::core::option::Option<super::SearchQueryLinguisticDetails>) -> ::windows::core::Result<()>;
    fn Suggestions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<SearchSuggestion>>;
    fn AddToHistory(&self, querytext: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddToHistoryWithLanguage(&self, querytext: &::windows::core::HSTRING, language: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClearHistory(&self) -> ::windows::core::Result<()>;
    fn SuggestionsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuggestionsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestingFocusOnKeyboardInput(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestingFocusOnKeyboardInput(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionsRequestedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::SearchQueryLinguisticDetails>;
    fn Request(&self) -> ::windows::core::Result<super::SearchSuggestionsRequest>;
}
