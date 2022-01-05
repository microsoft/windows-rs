#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerImpl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn PickMultipleFilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSingleFileAndContinue(&self) -> ::windows::core::Result<()>;
    fn PickMultipleFilesAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerStaticsImpl: Sized {
    fn ResumePickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerStatics2Impl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileOpenPicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerWithOperationIdImpl: Sized {
    fn PickSingleFileAsync(&self, pickeroperationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePickerImpl: Sized {
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeChoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn DefaultFileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultFileExtension(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedSaveFile(&self) -> ::windows::core::Result<super::StorageFile>;
    fn SetSuggestedSaveFile(&self, value: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<()>;
    fn SuggestedFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuggestedFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PickSaveFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSaveFileAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker4Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileSavePicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPickerImpl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickFolderAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FolderPicker>;
}
