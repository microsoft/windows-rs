#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotePlacementChangedPreviewEventArgs {}
impl ::core::clone::Clone for INotePlacementChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INoteVisibilityChangedPreviewEventArgs {}
impl ::core::clone::Clone for INoteVisibilityChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotesWindowManagerPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotesWindowManagerPreview {}
impl ::core::clone::Clone for INotesWindowManagerPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotesWindowManagerPreview2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotesWindowManagerPreview2 {}
impl ::core::clone::Clone for INotesWindowManagerPreview2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotesWindowManagerPreviewShowNoteOptions {}
impl ::core::clone::Clone for INotesWindowManagerPreviewShowNoteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotesWindowManagerPreviewStatics {}
impl ::core::clone::Clone for INotesWindowManagerPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotePlacementChangedPreviewEventArgs {}
impl ::core::clone::Clone for NotePlacementChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NoteVisibilityChangedPreviewEventArgs {}
impl ::core::clone::Clone for NoteVisibilityChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotesWindowManagerPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotesWindowManagerPreview {}
impl ::core::clone::Clone for NotesWindowManagerPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotesWindowManagerPreviewShowNoteOptions {}
impl ::core::clone::Clone for NotesWindowManagerPreviewShowNoteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
