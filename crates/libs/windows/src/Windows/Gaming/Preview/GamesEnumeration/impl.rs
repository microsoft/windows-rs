pub trait IGameListEntryImpl: Sized {
    fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(&self) -> ::windows::core::Result<GameListCategory>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListEntry2Impl: Sized + IGameListEntryImpl {
    fn LaunchableState(&self) -> ::windows::core::Result<GameListEntryLaunchableState>;
    fn LauncherExecutable(&self) -> ::windows::core::Result<super::super::super::Storage::IStorageFile>;
    fn LaunchParameters(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLauncherExecutableFileAsync(&self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SetLauncherExecutableFileWithParamsAsync(&self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>, launchparams: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitleIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GameModeConfiguration(&self) -> ::windows::core::Result<GameModeConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn FindAllAsyncPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn GameAdded(&self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameRemoved(&self, handler: &::core::option::Option<GameListRemovedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameUpdated(&self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListStatics2Impl: Sized {
    fn MergeEntriesAsync(&self, left: &::core::option::Option<GameListEntry>, right: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameListEntry>>;
    fn UnmergeEntryAsync(&self, mergedentry: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeConfigurationImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PercentGpuTimeAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuTimeAllocatedToGame(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToGame(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToSystemCompositor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToSystemCompositor(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxCpuCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetMaxCpuCount(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskLow(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskLow(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskHigh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskHigh(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn AffinitizeToExclusiveCpus(&self) -> ::windows::core::Result<bool>;
    fn SetAffinitizeToExclusiveCpus(&self, value: bool) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeUserConfigurationImpl: Sized {
    fn GamingRelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeUserConfigurationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameModeUserConfiguration>;
}
