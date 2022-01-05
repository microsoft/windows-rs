#[cfg(feature = "implement_exclusive")]
pub trait INotePlacementChangedPreviewEventArgsImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INoteVisibilityChangedPreviewEventArgsImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewImpl: Sized {
    fn IsScreenLocked(&self) -> ::windows::core::Result<bool>;
    fn ShowNote(&self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacement(&self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn HideNote(&self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn TrySetNoteSize(&self, noteviewid: i32, size: &super::super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetFocusToNextView(&self) -> ::windows::core::Result<()>;
    fn SetNotesThumbnailAsync(&self, thumbnail: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SystemLockStateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemLockStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotePlacementChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotePlacementChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoteVisibilityChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoteVisibilityChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreview2Impl: Sized {
    fn ShowNoteRelativeToWithOptions(&self, noteviewid: i32, anchornoteviewid: i32, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacementWithOptions(&self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn SetFocusToPreviousView(&self) -> ::windows::core::Result<()>;
    fn SetThumbnailImageForTaskSwitcherAsync(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewShowNoteOptionsImpl: Sized {
    fn ShowWithFocus(&self) -> ::windows::core::Result<bool>;
    fn SetShowWithFocus(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewStaticsImpl: Sized {
    fn GetForCurrentApp(&self) -> ::windows::core::Result<NotesWindowManagerPreview>;
}
