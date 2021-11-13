#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
impl ::core::marker::Copy for AdaptiveNotificationContentKind {}
impl ::core::clone::Clone for AdaptiveNotificationContentKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveNotificationText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveNotificationText {}
impl ::core::clone::Clone for AdaptiveNotificationText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BadgeNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BadgeNotification {}
impl ::core::clone::Clone for BadgeNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
impl ::core::marker::Copy for BadgeTemplateType {}
impl ::core::clone::Clone for BadgeTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BadgeUpdateManagerForUser {}
impl ::core::clone::Clone for BadgeUpdateManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BadgeUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BadgeUpdater {}
impl ::core::clone::Clone for BadgeUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveNotificationContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveNotificationContent {}
impl ::core::clone::Clone for IAdaptiveNotificationContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveNotificationText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveNotificationText {}
impl ::core::clone::Clone for IAdaptiveNotificationText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeNotification {}
impl ::core::clone::Clone for IBadgeNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeNotificationFactory {}
impl ::core::clone::Clone for IBadgeNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeUpdateManagerForUser {}
impl ::core::clone::Clone for IBadgeUpdateManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeUpdateManagerStatics {}
impl ::core::clone::Clone for IBadgeUpdateManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeUpdateManagerStatics2 {}
impl ::core::clone::Clone for IBadgeUpdateManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBadgeUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBadgeUpdater {}
impl ::core::clone::Clone for IBadgeUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationHintsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownAdaptiveNotificationHintsStatics {}
impl ::core::clone::Clone for IKnownAdaptiveNotificationHintsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownAdaptiveNotificationTextStylesStatics {}
impl ::core::clone::Clone for IKnownAdaptiveNotificationTextStylesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownNotificationBindingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownNotificationBindingsStatics {}
impl ::core::clone::Clone for IKnownNotificationBindingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotification {}
impl ::core::clone::Clone for INotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotificationBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotificationBinding {}
impl ::core::clone::Clone for INotificationBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotificationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotificationData {}
impl ::core::clone::Clone for INotificationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotificationDataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotificationDataFactory {}
impl ::core::clone::Clone for INotificationDataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotificationVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotificationVisual {}
impl ::core::clone::Clone for INotificationVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledTileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledTileNotification {}
impl ::core::clone::Clone for IScheduledTileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledTileNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledTileNotificationFactory {}
impl ::core::clone::Clone for IScheduledTileNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotification {}
impl ::core::clone::Clone for IScheduledToastNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotification2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotification2 {}
impl ::core::clone::Clone for IScheduledToastNotification2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotification3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotification3 {}
impl ::core::clone::Clone for IScheduledToastNotification3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotification4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotification4 {}
impl ::core::clone::Clone for IScheduledToastNotification4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotificationFactory {}
impl ::core::clone::Clone for IScheduledToastNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledToastNotificationShowingEventArgs {}
impl ::core::clone::Clone for IScheduledToastNotificationShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShownTileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShownTileNotification {}
impl ::core::clone::Clone for IShownTileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileFlyoutNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileFlyoutNotification {}
impl ::core::clone::Clone for ITileFlyoutNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileFlyoutNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileFlyoutNotificationFactory {}
impl ::core::clone::Clone for ITileFlyoutNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileFlyoutUpdateManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileFlyoutUpdateManagerStatics {}
impl ::core::clone::Clone for ITileFlyoutUpdateManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileFlyoutUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileFlyoutUpdater {}
impl ::core::clone::Clone for ITileFlyoutUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileNotification {}
impl ::core::clone::Clone for ITileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileNotificationFactory {}
impl ::core::clone::Clone for ITileNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileUpdateManagerForUser {}
impl ::core::clone::Clone for ITileUpdateManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileUpdateManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileUpdateManagerStatics {}
impl ::core::clone::Clone for ITileUpdateManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileUpdateManagerStatics2 {}
impl ::core::clone::Clone for ITileUpdateManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileUpdater {}
impl ::core::clone::Clone for ITileUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileUpdater2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileUpdater2 {}
impl ::core::clone::Clone for ITileUpdater2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastActivatedEventArgs {}
impl ::core::clone::Clone for IToastActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastActivatedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastActivatedEventArgs2 {}
impl ::core::clone::Clone for IToastActivatedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastCollection {}
impl ::core::clone::Clone for IToastCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastCollectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastCollectionFactory {}
impl ::core::clone::Clone for IToastCollectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastCollectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastCollectionManager {}
impl ::core::clone::Clone for IToastCollectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastDismissedEventArgs {}
impl ::core::clone::Clone for IToastDismissedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastFailedEventArgs {}
impl ::core::clone::Clone for IToastFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotification {}
impl ::core::clone::Clone for IToastNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotification2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotification2 {}
impl ::core::clone::Clone for IToastNotification2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotification3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotification3 {}
impl ::core::clone::Clone for IToastNotification3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotification4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotification4 {}
impl ::core::clone::Clone for IToastNotification4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotification6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotification6 {}
impl ::core::clone::Clone for IToastNotification6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationActionTriggerDetail {}
impl ::core::clone::Clone for IToastNotificationActionTriggerDetail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationFactory {}
impl ::core::clone::Clone for IToastNotificationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationHistory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationHistory {}
impl ::core::clone::Clone for IToastNotificationHistory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationHistory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationHistory2 {}
impl ::core::clone::Clone for IToastNotificationHistory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationHistoryChangedTriggerDetail {}
impl ::core::clone::Clone for IToastNotificationHistoryChangedTriggerDetail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationHistoryChangedTriggerDetail2 {}
impl ::core::clone::Clone for IToastNotificationHistoryChangedTriggerDetail2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerForUser {}
impl ::core::clone::Clone for IToastNotificationManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerForUser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerForUser2 {}
impl ::core::clone::Clone for IToastNotificationManagerForUser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerStatics {}
impl ::core::clone::Clone for IToastNotificationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerStatics2 {}
impl ::core::clone::Clone for IToastNotificationManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerStatics4 {}
impl ::core::clone::Clone for IToastNotificationManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerStatics5 {}
impl ::core::clone::Clone for IToastNotificationManagerStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotifier {}
impl ::core::clone::Clone for IToastNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotifier2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotifier2 {}
impl ::core::clone::Clone for IToastNotifier2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotifier3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotifier3 {}
impl ::core::clone::Clone for IToastNotifier3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserNotification {}
impl ::core::clone::Clone for IUserNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserNotificationChangedEventArgs {}
impl ::core::clone::Clone for IUserNotificationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Notification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Notification {}
impl ::core::clone::Clone for Notification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotificationBinding {}
impl ::core::clone::Clone for NotificationBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotificationData {}
impl ::core::clone::Clone for NotificationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
impl ::core::marker::Copy for NotificationKinds {}
impl ::core::clone::Clone for NotificationKinds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NotificationMirroring {}
impl ::core::clone::Clone for NotificationMirroring {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl ::core::marker::Copy for NotificationSetting {}
impl ::core::clone::Clone for NotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for NotificationUpdateResult {}
impl ::core::clone::Clone for NotificationUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotificationVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotificationVisual {}
impl ::core::clone::Clone for NotificationVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicUpdateRecurrence {}
impl ::core::clone::Clone for PeriodicUpdateRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScheduledTileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScheduledTileNotification {}
impl ::core::clone::Clone for ScheduledTileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScheduledToastNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScheduledToastNotification {}
impl ::core::clone::Clone for ScheduledToastNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScheduledToastNotificationShowingEventArgs {}
impl ::core::clone::Clone for ScheduledToastNotificationShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShownTileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShownTileNotification {}
impl ::core::clone::Clone for ShownTileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileFlyoutNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileFlyoutNotification {}
impl ::core::clone::Clone for TileFlyoutNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
impl ::core::marker::Copy for TileFlyoutTemplateType {}
impl ::core::clone::Clone for TileFlyoutTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileFlyoutUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileFlyoutUpdater {}
impl ::core::clone::Clone for TileFlyoutUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileNotification {}
impl ::core::clone::Clone for TileNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
impl ::core::marker::Copy for TileTemplateType {}
impl ::core::clone::Clone for TileTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileUpdateManagerForUser {}
impl ::core::clone::Clone for TileUpdateManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileUpdater {}
impl ::core::clone::Clone for TileUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastActivatedEventArgs {}
impl ::core::clone::Clone for ToastActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastCollection {}
impl ::core::clone::Clone for ToastCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastCollectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastCollectionManager {}
impl ::core::clone::Clone for ToastCollectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
impl ::core::marker::Copy for ToastDismissalReason {}
impl ::core::clone::Clone for ToastDismissalReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastDismissedEventArgs {}
impl ::core::clone::Clone for ToastDismissedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastFailedEventArgs {}
impl ::core::clone::Clone for ToastFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
impl ::core::marker::Copy for ToastHistoryChangedType {}
impl ::core::clone::Clone for ToastHistoryChangedType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotification {}
impl ::core::clone::Clone for ToastNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationActionTriggerDetail {}
impl ::core::clone::Clone for ToastNotificationActionTriggerDetail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationHistory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationHistory {}
impl ::core::clone::Clone for ToastNotificationHistory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationHistoryChangedTriggerDetail {}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTriggerDetail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationManagerForUser {}
impl ::core::clone::Clone for ToastNotificationManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ToastNotificationPriority {}
impl ::core::clone::Clone for ToastNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotifier {}
impl ::core::clone::Clone for ToastNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
impl ::core::marker::Copy for ToastTemplateType {}
impl ::core::clone::Clone for ToastTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserNotification {}
impl ::core::clone::Clone for UserNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserNotificationChangedEventArgs {}
impl ::core::clone::Clone for UserNotificationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
impl ::core::marker::Copy for UserNotificationChangedKind {}
impl ::core::clone::Clone for UserNotificationChangedKind {
    fn clone(&self) -> Self {
        *self
    }
}
