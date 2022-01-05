#[cfg(feature = "implement_exclusive")]
pub trait IJumpListImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<JumpListItem>>;
    fn SystemGroupKind(&self) -> ::windows::core::Result<JumpListSystemGroupKind>;
    fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<JumpListItemKind>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemovedByUser(&self) -> ::windows::core::Result<bool>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroupName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemStaticsImpl: Sized {
    fn CreateWithArguments(&self, arguments: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<JumpListItem>;
    fn CreateSeparator(&self) -> ::windows::core::Result<JumpListItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListStaticsImpl: Sized {
    fn LoadCurrentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<JumpList>>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileImpl: Sized {
    fn SetTileId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArguments(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetShortName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ShortName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSmallLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SmallLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetWideLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn WideLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLockScreenBadgeLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LockScreenBadgeLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows::core::Result<()>;
    fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows::core::Result<bool>;
    fn SetTileOptions(&self, value: TileOptions) -> ::windows::core::Result<()>;
    fn TileOptions(&self) -> ::windows::core::Result<TileOptions>;
    fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()>;
    fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn RequestCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithPoint(&self, invocationpoint: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithRect(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithRectAndPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithPoint(&self, invocationpoint: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithRect(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithRectAndPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTile2Impl: Sized + ISecondaryTileImpl {
    fn SetPhoneticName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PhoneticName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements>;
    fn SetRoamingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RoamingEnabled(&self) -> ::windows::core::Result<bool>;
    fn VisualElementsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisualElementsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileFactoryImpl: Sized {
    fn CreateTile(&self, tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SecondaryTile>;
    fn CreateWideTile(&self, tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &::core::option::Option<super::super::Foundation::Uri>, widelogoreference: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SecondaryTile>;
    fn CreateWithId(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<SecondaryTile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileFactory2Impl: Sized + ISecondaryTileFactoryImpl {
    fn CreateMinimalTile(&self, tileid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, square150x150logo: &::core::option::Option<super::super::Foundation::Uri>, desiredsize: TileSize) -> ::windows::core::Result<SecondaryTile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileStaticsImpl: Sized {
    fn Exists(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
    fn FindAllForApplicationAsync(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
    fn FindAllForPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileVisualElementsImpl: Sized {
    fn SetSquare30x30Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare70x70Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square70x70Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare150x150Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square150x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetWide310x150Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Wide310x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare310x310Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square310x310Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()>;
    fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnSquare150x150Logo(&self) -> ::windows::core::Result<bool>;
    fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnWide310x150Logo(&self) -> ::windows::core::Result<bool>;
    fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnSquare310x310Logo(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileVisualElements2Impl: Sized {
    fn SetSquare71x71Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square71x71Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileVisualElements3Impl: Sized {
    fn SetSquare44x44Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square44x44Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileVisualElements4Impl: Sized {
    fn MixedRealityModel(&self) -> ::windows::core::Result<TileMixedRealityModel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStartScreenManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn SupportsAppListEntry(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<bool>;
    fn ContainsAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestAddAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStartScreenManager2Impl: Sized + IStartScreenManagerImpl {
    fn ContainsSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRemoveSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStartScreenManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<StartScreenManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StartScreenManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileMixedRealityModelImpl: Sized {
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetBoundingBox(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>) -> ::windows::core::Result<()>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileMixedRealityModel2Impl: Sized {
    fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::Result<()>;
    fn ActivationBehavior(&self) -> ::windows::core::Result<TileMixedRealityModelActivationBehavior>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualElementsRequestImpl: Sized {
    fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements>;
    fn AlternateVisualElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<VisualElementsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualElementsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualElementsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<VisualElementsRequest>;
}
