#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotesWindowManagerPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotesWindowManagerPreview2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotesWindowManagerPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PreviewNotesContract(i32);
