#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IRequestingFocusOnKeyboardInputEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestion(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestionManager(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct RequestingFocusOnKeyboardInputEventArgs(i32);
pub struct SearchCoreContract(i32);
pub struct SearchSuggestion(i32);
pub struct SearchSuggestionKind(i32);
pub struct SearchSuggestionManager(i32);
pub struct SearchSuggestionsRequestedEventArgs(i32);
