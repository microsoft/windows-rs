#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: AdaptiveNotificationContentKind = AdaptiveNotificationContentKind(0i32);
}
#[repr(transparent)]
pub struct AdaptiveNotificationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: BadgeTemplateType = BadgeTemplateType(0i32);
    pub const BadgeNumber: BadgeTemplateType = BadgeTemplateType(1i32);
}
#[repr(transparent)]
pub struct BadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveNotificationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveNotificationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBadgeUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationHintsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownNotificationBindingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledTileNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotification4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShownTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileFlyoutUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdateManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileUpdater2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastActivatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastCollectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotification6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerForUser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotifier3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Notification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: NotificationKinds = NotificationKinds(0u32);
    pub const Toast: NotificationKinds = NotificationKinds(1u32);
}
#[repr(transparent)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: NotificationMirroring = NotificationMirroring(0i32);
    pub const Disabled: NotificationMirroring = NotificationMirroring(1i32);
}
#[repr(transparent)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: NotificationSetting = NotificationSetting(0i32);
    pub const DisabledForApplication: NotificationSetting = NotificationSetting(1i32);
    pub const DisabledForUser: NotificationSetting = NotificationSetting(2i32);
    pub const DisabledByGroupPolicy: NotificationSetting = NotificationSetting(3i32);
    pub const DisabledByManifest: NotificationSetting = NotificationSetting(4i32);
}
#[repr(transparent)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: NotificationUpdateResult = NotificationUpdateResult(0i32);
    pub const Failed: NotificationUpdateResult = NotificationUpdateResult(1i32);
    pub const NotificationNotFound: NotificationUpdateResult = NotificationUpdateResult(2i32);
}
#[repr(transparent)]
pub struct NotificationVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(0i32);
    pub const Hour: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(1i32);
    pub const SixHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(2i32);
    pub const TwelveHours: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(3i32);
    pub const Daily: PeriodicUpdateRecurrence = PeriodicUpdateRecurrence(4i32);
}
#[repr(transparent)]
pub struct ScheduledTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScheduledToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScheduledToastNotificationShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShownTileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileFlyoutNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: TileFlyoutTemplateType = TileFlyoutTemplateType(0i32);
}
#[repr(transparent)]
pub struct TileFlyoutUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: TileTemplateType = TileTemplateType(0i32);
    pub const TileSquareBlock: TileTemplateType = TileTemplateType(1i32);
    pub const TileSquareText01: TileTemplateType = TileTemplateType(2i32);
    pub const TileSquareText02: TileTemplateType = TileTemplateType(3i32);
    pub const TileSquareText03: TileTemplateType = TileTemplateType(4i32);
    pub const TileSquareText04: TileTemplateType = TileTemplateType(5i32);
    pub const TileSquarePeekImageAndText01: TileTemplateType = TileTemplateType(6i32);
    pub const TileSquarePeekImageAndText02: TileTemplateType = TileTemplateType(7i32);
    pub const TileSquarePeekImageAndText03: TileTemplateType = TileTemplateType(8i32);
    pub const TileSquarePeekImageAndText04: TileTemplateType = TileTemplateType(9i32);
    pub const TileWideImage: TileTemplateType = TileTemplateType(10i32);
    pub const TileWideImageCollection: TileTemplateType = TileTemplateType(11i32);
    pub const TileWideImageAndText01: TileTemplateType = TileTemplateType(12i32);
    pub const TileWideImageAndText02: TileTemplateType = TileTemplateType(13i32);
    pub const TileWideBlockAndText01: TileTemplateType = TileTemplateType(14i32);
    pub const TileWideBlockAndText02: TileTemplateType = TileTemplateType(15i32);
    pub const TileWidePeekImageCollection01: TileTemplateType = TileTemplateType(16i32);
    pub const TileWidePeekImageCollection02: TileTemplateType = TileTemplateType(17i32);
    pub const TileWidePeekImageCollection03: TileTemplateType = TileTemplateType(18i32);
    pub const TileWidePeekImageCollection04: TileTemplateType = TileTemplateType(19i32);
    pub const TileWidePeekImageCollection05: TileTemplateType = TileTemplateType(20i32);
    pub const TileWidePeekImageCollection06: TileTemplateType = TileTemplateType(21i32);
    pub const TileWidePeekImageAndText01: TileTemplateType = TileTemplateType(22i32);
    pub const TileWidePeekImageAndText02: TileTemplateType = TileTemplateType(23i32);
    pub const TileWidePeekImage01: TileTemplateType = TileTemplateType(24i32);
    pub const TileWidePeekImage02: TileTemplateType = TileTemplateType(25i32);
    pub const TileWidePeekImage03: TileTemplateType = TileTemplateType(26i32);
    pub const TileWidePeekImage04: TileTemplateType = TileTemplateType(27i32);
    pub const TileWidePeekImage05: TileTemplateType = TileTemplateType(28i32);
    pub const TileWidePeekImage06: TileTemplateType = TileTemplateType(29i32);
    pub const TileWideSmallImageAndText01: TileTemplateType = TileTemplateType(30i32);
    pub const TileWideSmallImageAndText02: TileTemplateType = TileTemplateType(31i32);
    pub const TileWideSmallImageAndText03: TileTemplateType = TileTemplateType(32i32);
    pub const TileWideSmallImageAndText04: TileTemplateType = TileTemplateType(33i32);
    pub const TileWideSmallImageAndText05: TileTemplateType = TileTemplateType(34i32);
    pub const TileWideText01: TileTemplateType = TileTemplateType(35i32);
    pub const TileWideText02: TileTemplateType = TileTemplateType(36i32);
    pub const TileWideText03: TileTemplateType = TileTemplateType(37i32);
    pub const TileWideText04: TileTemplateType = TileTemplateType(38i32);
    pub const TileWideText05: TileTemplateType = TileTemplateType(39i32);
    pub const TileWideText06: TileTemplateType = TileTemplateType(40i32);
    pub const TileWideText07: TileTemplateType = TileTemplateType(41i32);
    pub const TileWideText08: TileTemplateType = TileTemplateType(42i32);
    pub const TileWideText09: TileTemplateType = TileTemplateType(43i32);
    pub const TileWideText10: TileTemplateType = TileTemplateType(44i32);
    pub const TileWideText11: TileTemplateType = TileTemplateType(45i32);
    pub const TileSquare150x150Image: TileTemplateType = TileTemplateType(0i32);
    pub const TileSquare150x150Block: TileTemplateType = TileTemplateType(1i32);
    pub const TileSquare150x150Text01: TileTemplateType = TileTemplateType(2i32);
    pub const TileSquare150x150Text02: TileTemplateType = TileTemplateType(3i32);
    pub const TileSquare150x150Text03: TileTemplateType = TileTemplateType(4i32);
    pub const TileSquare150x150Text04: TileTemplateType = TileTemplateType(5i32);
    pub const TileSquare150x150PeekImageAndText01: TileTemplateType = TileTemplateType(6i32);
    pub const TileSquare150x150PeekImageAndText02: TileTemplateType = TileTemplateType(7i32);
    pub const TileSquare150x150PeekImageAndText03: TileTemplateType = TileTemplateType(8i32);
    pub const TileSquare150x150PeekImageAndText04: TileTemplateType = TileTemplateType(9i32);
    pub const TileWide310x150Image: TileTemplateType = TileTemplateType(10i32);
    pub const TileWide310x150ImageCollection: TileTemplateType = TileTemplateType(11i32);
    pub const TileWide310x150ImageAndText01: TileTemplateType = TileTemplateType(12i32);
    pub const TileWide310x150ImageAndText02: TileTemplateType = TileTemplateType(13i32);
    pub const TileWide310x150BlockAndText01: TileTemplateType = TileTemplateType(14i32);
    pub const TileWide310x150BlockAndText02: TileTemplateType = TileTemplateType(15i32);
    pub const TileWide310x150PeekImageCollection01: TileTemplateType = TileTemplateType(16i32);
    pub const TileWide310x150PeekImageCollection02: TileTemplateType = TileTemplateType(17i32);
    pub const TileWide310x150PeekImageCollection03: TileTemplateType = TileTemplateType(18i32);
    pub const TileWide310x150PeekImageCollection04: TileTemplateType = TileTemplateType(19i32);
    pub const TileWide310x150PeekImageCollection05: TileTemplateType = TileTemplateType(20i32);
    pub const TileWide310x150PeekImageCollection06: TileTemplateType = TileTemplateType(21i32);
    pub const TileWide310x150PeekImageAndText01: TileTemplateType = TileTemplateType(22i32);
    pub const TileWide310x150PeekImageAndText02: TileTemplateType = TileTemplateType(23i32);
    pub const TileWide310x150PeekImage01: TileTemplateType = TileTemplateType(24i32);
    pub const TileWide310x150PeekImage02: TileTemplateType = TileTemplateType(25i32);
    pub const TileWide310x150PeekImage03: TileTemplateType = TileTemplateType(26i32);
    pub const TileWide310x150PeekImage04: TileTemplateType = TileTemplateType(27i32);
    pub const TileWide310x150PeekImage05: TileTemplateType = TileTemplateType(28i32);
    pub const TileWide310x150PeekImage06: TileTemplateType = TileTemplateType(29i32);
    pub const TileWide310x150SmallImageAndText01: TileTemplateType = TileTemplateType(30i32);
    pub const TileWide310x150SmallImageAndText02: TileTemplateType = TileTemplateType(31i32);
    pub const TileWide310x150SmallImageAndText03: TileTemplateType = TileTemplateType(32i32);
    pub const TileWide310x150SmallImageAndText04: TileTemplateType = TileTemplateType(33i32);
    pub const TileWide310x150SmallImageAndText05: TileTemplateType = TileTemplateType(34i32);
    pub const TileWide310x150Text01: TileTemplateType = TileTemplateType(35i32);
    pub const TileWide310x150Text02: TileTemplateType = TileTemplateType(36i32);
    pub const TileWide310x150Text03: TileTemplateType = TileTemplateType(37i32);
    pub const TileWide310x150Text04: TileTemplateType = TileTemplateType(38i32);
    pub const TileWide310x150Text05: TileTemplateType = TileTemplateType(39i32);
    pub const TileWide310x150Text06: TileTemplateType = TileTemplateType(40i32);
    pub const TileWide310x150Text07: TileTemplateType = TileTemplateType(41i32);
    pub const TileWide310x150Text08: TileTemplateType = TileTemplateType(42i32);
    pub const TileWide310x150Text09: TileTemplateType = TileTemplateType(43i32);
    pub const TileWide310x150Text10: TileTemplateType = TileTemplateType(44i32);
    pub const TileWide310x150Text11: TileTemplateType = TileTemplateType(45i32);
    pub const TileSquare310x310BlockAndText01: TileTemplateType = TileTemplateType(46i32);
    pub const TileSquare310x310BlockAndText02: TileTemplateType = TileTemplateType(47i32);
    pub const TileSquare310x310Image: TileTemplateType = TileTemplateType(48i32);
    pub const TileSquare310x310ImageAndText01: TileTemplateType = TileTemplateType(49i32);
    pub const TileSquare310x310ImageAndText02: TileTemplateType = TileTemplateType(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: TileTemplateType = TileTemplateType(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: TileTemplateType = TileTemplateType(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: TileTemplateType = TileTemplateType(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: TileTemplateType = TileTemplateType(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: TileTemplateType = TileTemplateType(55i32);
    pub const TileSquare310x310ImageCollection: TileTemplateType = TileTemplateType(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: TileTemplateType = TileTemplateType(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: TileTemplateType = TileTemplateType(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: TileTemplateType = TileTemplateType(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: TileTemplateType = TileTemplateType(60i32);
    pub const TileSquare310x310Text01: TileTemplateType = TileTemplateType(61i32);
    pub const TileSquare310x310Text02: TileTemplateType = TileTemplateType(62i32);
    pub const TileSquare310x310Text03: TileTemplateType = TileTemplateType(63i32);
    pub const TileSquare310x310Text04: TileTemplateType = TileTemplateType(64i32);
    pub const TileSquare310x310Text05: TileTemplateType = TileTemplateType(65i32);
    pub const TileSquare310x310Text06: TileTemplateType = TileTemplateType(66i32);
    pub const TileSquare310x310Text07: TileTemplateType = TileTemplateType(67i32);
    pub const TileSquare310x310Text08: TileTemplateType = TileTemplateType(68i32);
    pub const TileSquare310x310TextList01: TileTemplateType = TileTemplateType(69i32);
    pub const TileSquare310x310TextList02: TileTemplateType = TileTemplateType(70i32);
    pub const TileSquare310x310TextList03: TileTemplateType = TileTemplateType(71i32);
    pub const TileSquare310x310SmallImageAndText01: TileTemplateType = TileTemplateType(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: TileTemplateType = TileTemplateType(73i32);
    pub const TileSquare310x310Text09: TileTemplateType = TileTemplateType(74i32);
    pub const TileSquare71x71IconWithBadge: TileTemplateType = TileTemplateType(75i32);
    pub const TileSquare150x150IconWithBadge: TileTemplateType = TileTemplateType(76i32);
    pub const TileWide310x150IconWithBadgeAndText: TileTemplateType = TileTemplateType(77i32);
    pub const TileSquare71x71Image: TileTemplateType = TileTemplateType(78i32);
    pub const TileTall150x310Image: TileTemplateType = TileTemplateType(79i32);
}
#[repr(transparent)]
pub struct TileUpdateManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastCollectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: ToastDismissalReason = ToastDismissalReason(0i32);
    pub const ApplicationHidden: ToastDismissalReason = ToastDismissalReason(1i32);
    pub const TimedOut: ToastDismissalReason = ToastDismissalReason(2i32);
}
#[repr(transparent)]
pub struct ToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: ToastHistoryChangedType = ToastHistoryChangedType(0i32);
    pub const Removed: ToastHistoryChangedType = ToastHistoryChangedType(1i32);
    pub const Expired: ToastHistoryChangedType = ToastHistoryChangedType(2i32);
    pub const Added: ToastHistoryChangedType = ToastHistoryChangedType(3i32);
}
#[repr(transparent)]
pub struct ToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: ToastNotificationPriority = ToastNotificationPriority(0i32);
    pub const High: ToastNotificationPriority = ToastNotificationPriority(1i32);
}
#[repr(transparent)]
pub struct ToastNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: ToastTemplateType = ToastTemplateType(0i32);
    pub const ToastImageAndText02: ToastTemplateType = ToastTemplateType(1i32);
    pub const ToastImageAndText03: ToastTemplateType = ToastTemplateType(2i32);
    pub const ToastImageAndText04: ToastTemplateType = ToastTemplateType(3i32);
    pub const ToastText01: ToastTemplateType = ToastTemplateType(4i32);
    pub const ToastText02: ToastTemplateType = ToastTemplateType(5i32);
    pub const ToastText03: ToastTemplateType = ToastTemplateType(6i32);
    pub const ToastText04: ToastTemplateType = ToastTemplateType(7i32);
}
#[repr(transparent)]
pub struct UserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: UserNotificationChangedKind = UserNotificationChangedKind(0i32);
    pub const Removed: UserNotificationChangedKind = UserNotificationChangedKind(1i32);
}
