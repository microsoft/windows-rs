#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRequestingFocusOnKeyboardInputEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRequestingFocusOnKeyboardInputEventArgs {}
impl ::core::clone::Clone for IRequestingFocusOnKeyboardInputEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestion {}
impl ::core::clone::Clone for ISearchSuggestion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestionManager {}
impl ::core::clone::Clone for ISearchSuggestionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for ISearchSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RequestingFocusOnKeyboardInputEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RequestingFocusOnKeyboardInputEventArgs {}
impl ::core::clone::Clone for RequestingFocusOnKeyboardInputEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestion {}
impl ::core::clone::Clone for SearchSuggestion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionKind(pub i32);
impl SearchSuggestionKind {
    pub const Query: Self = Self(0i32);
    pub const Result: Self = Self(1i32);
    pub const Separator: Self = Self(2i32);
}
impl ::core::marker::Copy for SearchSuggestionKind {}
impl ::core::clone::Clone for SearchSuggestionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestionManager {}
impl ::core::clone::Clone for SearchSuggestionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for SearchSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
