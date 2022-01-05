pub trait IAdaptiveCardImpl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IAdaptiveCardBuilderStaticsImpl: Sized {
    fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecurityAppManagerImpl: Sized {
    fn Register(&self, kind: SecurityAppKind, displayname: &::windows::core::HSTRING, detailsuri: &::core::option::Option<super::super::Foundation::Uri>, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID>;
    fn Unregister(&self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateState(&self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandEventArgsImpl: Sized {
    fn WindowId(&self) -> ::windows::core::Result<super::WindowId>;
    fn Command(&self) -> ::windows::core::Result<ShareWindowCommand>;
    fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandSourceImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn ReportCommandChanged(&self) -> ::windows::core::Result<()>;
    fn CommandRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CommandInvoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandSourceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ShareWindowCommandSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManagerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinningAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IsAppListEntryPinnedAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManager2Impl: Sized + ITaskbarManagerImpl {
    fn IsSecondaryTilePinnedAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinSecondaryTileAsync(&self, secondarytile: &::core::option::Option<super::StartScreen::SecondaryTile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUnpinSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<TaskbarManager>;
}
