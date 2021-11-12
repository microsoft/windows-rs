#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreTextCompositionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextCompositionSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextEditContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextFormatUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CoreTextFormatUpdatingReason(i32);
#[repr(C)]
pub struct CoreTextFormatUpdatingResult(i32);
#[repr(C)]
pub struct CoreTextInputPaneDisplayPolicy(i32);
#[repr(C)]
pub struct CoreTextInputScope(i32);
#[repr(transparent)]
pub struct CoreTextLayoutBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextLayoutRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextLayoutRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CoreTextRange(i32);
#[repr(transparent)]
pub struct CoreTextSelectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextSelectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CoreTextSelectionUpdatingResult(i32);
#[repr(transparent)]
pub struct CoreTextServicesConstants(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CoreTextTextUpdatingResult(i32);
#[repr(transparent)]
pub struct ICoreTextCompositionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextCompositionSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextEditContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextEditContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextFormatUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextUpdatingEventArgs(pub *mut ::core::ffi::c_void);
