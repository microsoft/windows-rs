#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct SearchCoreContract(i32);
#[repr(transparent)]
pub struct SearchSuggestion(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SearchSuggestionKind(i32);
#[repr(transparent)]
pub struct SearchSuggestionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
