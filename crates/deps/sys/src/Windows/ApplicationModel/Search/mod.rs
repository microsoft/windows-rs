#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(pub *mut ::core::ffi::c_void);
pub struct SearchContract(i32);
#[repr(transparent)]
pub struct SearchPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
