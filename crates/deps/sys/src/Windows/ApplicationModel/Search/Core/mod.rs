#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRequestingFocusOnKeyboardInputEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RequestingFocusOnKeyboardInputEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionKind(pub i32);
impl SearchSuggestionKind {
    pub const Query: Self = Self(0i32);
    pub const Result: Self = Self(1i32);
    pub const Separator: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SearchSuggestionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
