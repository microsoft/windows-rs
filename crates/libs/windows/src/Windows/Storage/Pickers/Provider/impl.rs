#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerUIImpl: Sized {
    fn AddFile(&self, id: &::windows::core::HSTRING, file: &::core::option::Option<super::super::IStorageFile>) -> ::windows::core::Result<AddFileResult>;
    fn RemoveFile(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainsFile(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn CanAddFile(&self, file: &::core::option::Option<super::super::IStorageFile>) -> ::windows::core::Result<bool>;
    fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SelectionMode(&self) -> ::windows::core::Result<FileSelectionMode>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileRemoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IFileRemovedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePickerUIImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrySetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<SetFileNameResult>;
    fn FileNameChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileNameChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetFileRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetFileRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerClosingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerClosingEventArgsImpl: Sized {
    fn ClosingOperation(&self) -> ::windows::core::Result<PickerClosingOperation>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerClosingOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<PickerClosingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetFileRequestImpl: Sized {
    fn TargetFile(&self) -> ::windows::core::Result<super::super::IStorageFile>;
    fn SetTargetFile(&self, value: &::core::option::Option<super::super::IStorageFile>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<TargetFileRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetFileRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetFileRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<TargetFileRequest>;
}
