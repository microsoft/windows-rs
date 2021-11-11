#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IRequestingFocusOnKeyboardInputEventArgs();
    fn ISearchSuggestion();
    fn ISearchSuggestionManager();
    fn ISearchSuggestionsRequestedEventArgs();
    fn RequestingFocusOnKeyboardInputEventArgs();
    fn SearchCoreContract();
    fn SearchSuggestion();
    fn SearchSuggestionKind();
    fn SearchSuggestionManager();
    fn SearchSuggestionsRequestedEventArgs();
}
