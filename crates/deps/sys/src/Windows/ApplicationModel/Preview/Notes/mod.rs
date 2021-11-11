#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn INotePlacementChangedPreviewEventArgs();
    fn INoteVisibilityChangedPreviewEventArgs();
    fn INotesWindowManagerPreview();
    fn INotesWindowManagerPreview2();
    fn INotesWindowManagerPreviewShowNoteOptions();
    fn INotesWindowManagerPreviewStatics();
    fn NotePlacementChangedPreviewEventArgs();
    fn NoteVisibilityChangedPreviewEventArgs();
    fn NotesWindowManagerPreview();
    fn NotesWindowManagerPreviewShowNoteOptions();
    fn PreviewNotesContract();
}
