#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct INotePlacementChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct INoteVisibilityChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct INotesWindowManagerPreview(pub *mut ::core::ffi::c_void);
pub struct INotesWindowManagerPreview2(pub *mut ::core::ffi::c_void);
pub struct INotesWindowManagerPreviewShowNoteOptions(pub *mut ::core::ffi::c_void);
pub struct INotesWindowManagerPreviewStatics(pub *mut ::core::ffi::c_void);
pub struct NotePlacementChangedPreviewEventArgs(i32);
pub struct NoteVisibilityChangedPreviewEventArgs(i32);
pub struct NotesWindowManagerPreview(i32);
pub struct NotesWindowManagerPreviewShowNoteOptions(i32);
pub struct PreviewNotesContract(i32);
