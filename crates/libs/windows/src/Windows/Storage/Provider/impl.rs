#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterStaticsImpl: Sized {
    fn SetUpdateInformation(&self, file: &::core::option::Option<super::IStorageFile>, contentid: &::windows::core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterUIImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateTarget(&self) -> ::windows::core::Result<CachedFileTarget>;
    fn FileUpdateRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileUpdateRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UIRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUIRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UIStatus(&self) -> ::windows::core::Result<UIStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterUI2Impl: Sized + ICachedFileUpdaterUIImpl {
    fn UpdateRequest(&self) -> ::windows::core::Result<FileUpdateRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestImpl: Sized {
    fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn File(&self) -> ::windows::core::Result<super::StorageFile>;
    fn Status(&self) -> ::windows::core::Result<FileUpdateStatus>;
    fn SetStatus(&self, value: FileUpdateStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral>;
    fn UpdateLocalFile(&self, value: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequest2Impl: Sized + IFileUpdateRequestImpl {
    fn UserInputNeededMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserInputNeededMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUpdateRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<FileUpdateRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFilePath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrimaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetPrimaryAction(&self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
    fn SecondaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetSecondaryAction(&self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
    fn InformationalLink(&self) -> ::windows::core::Result<StorageProviderErrorCommand>;
    fn SetInformationalLink(&self, value: &::core::option::Option<StorageProviderErrorCommand>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorCommandImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorCommandFactoryImpl: Sized {
    fn CreateInstance(&self, label: &::windows::core::HSTRING, actionuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<StorageProviderErrorCommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderErrorFactoryImpl: Sized {
    fn CreateInstance(&self, id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderFileTypeInfoImpl: Sized {
    fn FileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderFileTypeInfoFactoryImpl: Sized {
    fn CreateInstance(&self, fileextension: &::windows::core::HSTRING, iconresource: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderFileTypeInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderGetContentInfoForPathResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus>;
    fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()>;
    fn ContentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentUri(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderGetPathForContentUriResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus>;
    fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IStorageProviderHandlerFactoryImpl: Sized {
    fn GetStatusSource(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderItemPropertiesStaticsImpl: Sized {
    fn SetAsync(&self, item: &::core::option::Option<super::IStorageItem>, itemproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderItemPropertyImpl: Sized {
    fn SetId(&self, value: i32) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderItemPropertyDefinitionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn SetId(&self, value: i32) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IStorageProviderItemPropertySourceImpl: Sized {
    fn GetItemProperties(&self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
pub trait IStorageProviderPropertyCapabilitiesImpl: Sized {
    fn IsPropertySupported(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderStatusImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<StorageProviderState>;
    fn ErrorMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderError>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderStatusFactoryImpl: Sized {
    fn CreateInstance(&self, state: StorageProviderState, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderStatus>;
    fn CreateInstance2(&self, state: StorageProviderState, message: &::windows::core::HSTRING, errormessages: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorageProviderError>>) -> ::windows::core::Result<StorageProviderStatus>;
}
pub trait IStorageProviderStatusSourceImpl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<StorageProviderStatus>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Context(&self) -> ::windows::core::Result<super::Streams::IBuffer>;
    fn SetContext(&self, value: &::core::option::Option<super::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<super::IStorageFolder>;
    fn SetPath(&self, value: &::core::option::Option<super::IStorageFolder>) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HydrationPolicy(&self) -> ::windows::core::Result<StorageProviderHydrationPolicy>;
    fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()>;
    fn HydrationPolicyModifier(&self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier>;
    fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()>;
    fn PopulationPolicy(&self) -> ::windows::core::Result<StorageProviderPopulationPolicy>;
    fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()>;
    fn InSyncPolicy(&self) -> ::windows::core::Result<StorageProviderInSyncPolicy>;
    fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()>;
    fn HardlinkPolicy(&self) -> ::windows::core::Result<StorageProviderHardlinkPolicy>;
    fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()>;
    fn ShowSiblingsAsGroup(&self) -> ::windows::core::Result<bool>;
    fn SetShowSiblingsAsGroup(&self, value: bool) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProtectionMode(&self) -> ::windows::core::Result<StorageProviderProtectionMode>;
    fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()>;
    fn AllowPinning(&self) -> ::windows::core::Result<bool>;
    fn SetAllowPinning(&self, value: bool) -> ::windows::core::Result<()>;
    fn StorageProviderItemPropertyDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>;
    fn RecycleBinUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetRecycleBinUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootInfo2Impl: Sized {
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetProviderId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootInfo3Impl: Sized {
    fn FallbackFileTypeInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootManagerStaticsImpl: Sized {
    fn Register(&self, syncrootinformation: &::core::option::Option<StorageProviderSyncRootInfo>) -> ::windows::core::Result<()>;
    fn Unregister(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetSyncRootInformationForFolder(&self, folder: &::core::option::Option<super::IStorageFolder>) -> ::windows::core::Result<StorageProviderSyncRootInfo>;
    fn GetSyncRootInformationForId(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderSyncRootInfo>;
    fn GetCurrentSyncRoots(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderSyncRootManagerStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
pub trait IStorageProviderUriSourceImpl: Sized {
    fn GetPathForContentUri(&self, contenturi: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetPathForContentUriResult>) -> ::windows::core::Result<()>;
    fn GetContentInfoForPath(&self, path: &::windows::core::HSTRING, result: &::core::option::Option<StorageProviderGetContentInfoForPathResult>) -> ::windows::core::Result<()>;
}
