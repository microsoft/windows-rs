#[cfg(feature = "implement_exclusive")]
pub trait ILocalContentSuggestionSettingsImpl: Sized {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>>;
    fn SetAqsFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AqsFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PropertiesToMatch(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
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
#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchPaneQueryLinguisticDetailsImpl: Sized {
    fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32>;
    fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneQuerySubmittedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneResultSuggestionChosenEventArgsImpl: Sized {
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SearchPane>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneStaticsWithHideThisApplicationImpl: Sized {
    fn HideThisApplication(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection>;
    fn GetDeferral(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequestDeferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneSuggestionsRequestedEventArgsImpl: Sized + ISearchPaneQueryChangedEventArgsImpl {
    fn Request(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISearchPaneVisibilityChangedEventArgsImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchQueryLinguisticDetailsImpl: Sized {
    fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32>;
    fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchQueryLinguisticDetailsFactoryImpl: Sized {
    fn CreateInstance(&self, querytextalternatives: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows::core::Result<SearchQueryLinguisticDetails>;
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
pub trait ISearchSuggestionsRequestImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection>;
    fn GetDeferral(&self) -> ::windows::core::Result<SearchSuggestionsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchSuggestionsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
