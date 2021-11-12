#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalContentSuggestionSettings {}
impl ::core::clone::Clone for ILocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPane(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPane {}
impl ::core::clone::Clone for ISearchPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneQueryChangedEventArgs {}
impl ::core::clone::Clone for ISearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneQueryLinguisticDetails {}
impl ::core::clone::Clone for ISearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneQuerySubmittedEventArgs {}
impl ::core::clone::Clone for ISearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {}
impl ::core::clone::Clone for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneResultSuggestionChosenEventArgs {}
impl ::core::clone::Clone for ISearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneStatics {}
impl ::core::clone::Clone for ISearchPaneStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneStaticsWithHideThisApplication {}
impl ::core::clone::Clone for ISearchPaneStaticsWithHideThisApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneSuggestionsRequest {}
impl ::core::clone::Clone for ISearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneSuggestionsRequestDeferral {}
impl ::core::clone::Clone for ISearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for ISearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchPaneVisibilityChangedEventArgs {}
impl ::core::clone::Clone for ISearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchQueryLinguisticDetails {}
impl ::core::clone::Clone for ISearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchQueryLinguisticDetailsFactory {}
impl ::core::clone::Clone for ISearchQueryLinguisticDetailsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestionCollection {}
impl ::core::clone::Clone for ISearchSuggestionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestionsRequest {}
impl ::core::clone::Clone for ISearchSuggestionsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestionsRequestDeferral {}
impl ::core::clone::Clone for ISearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocalContentSuggestionSettings {}
impl ::core::clone::Clone for LocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPane(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPane {}
impl ::core::clone::Clone for SearchPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneQueryChangedEventArgs {}
impl ::core::clone::Clone for SearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneQueryLinguisticDetails {}
impl ::core::clone::Clone for SearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneQuerySubmittedEventArgs {}
impl ::core::clone::Clone for SearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneResultSuggestionChosenEventArgs {}
impl ::core::clone::Clone for SearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneSuggestionsRequest {}
impl ::core::clone::Clone for SearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneSuggestionsRequestDeferral {}
impl ::core::clone::Clone for SearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for SearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchPaneVisibilityChangedEventArgs {}
impl ::core::clone::Clone for SearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchQueryLinguisticDetails {}
impl ::core::clone::Clone for SearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestionCollection {}
impl ::core::clone::Clone for SearchSuggestionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestionsRequest {}
impl ::core::clone::Clone for SearchSuggestionsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestionsRequestDeferral {}
impl ::core::clone::Clone for SearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
