#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct ILocalContentSuggestionSettings(pub *mut ::core::ffi::c_void);
pub struct ISearchPane(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneStatics(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneStaticsWithHideThisApplication(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneSuggestionsRequest(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchPaneVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISearchQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
pub struct ISearchQueryLinguisticDetailsFactory(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestionCollection(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestionsRequest(pub *mut ::core::ffi::c_void);
pub struct ISearchSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
pub struct LocalContentSuggestionSettings(i32);
pub struct SearchContract(i32);
pub struct SearchPane(i32);
pub struct SearchPaneQueryChangedEventArgs(i32);
pub struct SearchPaneQueryLinguisticDetails(i32);
pub struct SearchPaneQuerySubmittedEventArgs(i32);
pub struct SearchPaneResultSuggestionChosenEventArgs(i32);
pub struct SearchPaneSuggestionsRequest(i32);
pub struct SearchPaneSuggestionsRequestDeferral(i32);
pub struct SearchPaneSuggestionsRequestedEventArgs(i32);
pub struct SearchPaneVisibilityChangedEventArgs(i32);
pub struct SearchQueryLinguisticDetails(i32);
pub struct SearchSuggestionCollection(i32);
pub struct SearchSuggestionsRequest(i32);
pub struct SearchSuggestionsRequestDeferral(i32);
