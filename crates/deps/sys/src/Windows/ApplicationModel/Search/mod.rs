#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn ILocalContentSuggestionSettings();
    fn ISearchPane();
    fn ISearchPaneQueryChangedEventArgs();
    fn ISearchPaneQueryLinguisticDetails();
    fn ISearchPaneQuerySubmittedEventArgs();
    fn ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails();
    fn ISearchPaneResultSuggestionChosenEventArgs();
    fn ISearchPaneStatics();
    fn ISearchPaneStaticsWithHideThisApplication();
    fn ISearchPaneSuggestionsRequest();
    fn ISearchPaneSuggestionsRequestDeferral();
    fn ISearchPaneSuggestionsRequestedEventArgs();
    fn ISearchPaneVisibilityChangedEventArgs();
    fn ISearchQueryLinguisticDetails();
    fn ISearchQueryLinguisticDetailsFactory();
    fn ISearchSuggestionCollection();
    fn ISearchSuggestionsRequest();
    fn ISearchSuggestionsRequestDeferral();
    fn LocalContentSuggestionSettings();
    fn SearchContract();
    fn SearchPane();
    fn SearchPaneQueryChangedEventArgs();
    fn SearchPaneQueryLinguisticDetails();
    fn SearchPaneQuerySubmittedEventArgs();
    fn SearchPaneResultSuggestionChosenEventArgs();
    fn SearchPaneSuggestionsRequest();
    fn SearchPaneSuggestionsRequestDeferral();
    fn SearchPaneSuggestionsRequestedEventArgs();
    fn SearchPaneVisibilityChangedEventArgs();
    fn SearchQueryLinguisticDetails();
    fn SearchSuggestionCollection();
    fn SearchSuggestionsRequest();
    fn SearchSuggestionsRequestDeferral();
}
