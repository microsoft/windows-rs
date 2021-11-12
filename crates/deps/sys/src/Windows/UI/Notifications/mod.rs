#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AdaptiveNotificationContentKind(i32);
#[repr(transparent)]
pub struct AdaptiveNotificationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BadgeNotification(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BadgeTemplateType(i32);
#[repr(transparent)]
pub struct BadgeUpdateManager(pub *mut ::core::ffi::c_void);
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
pub struct KnownAdaptiveNotificationHints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownAdaptiveNotificationTextStyles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownNotificationBindings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Notification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotificationData(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NotificationKinds(i32);
#[repr(C)]
pub struct NotificationMirroring(i32);
#[repr(C)]
pub struct NotificationSetting(i32);
#[repr(C)]
pub struct NotificationUpdateResult(i32);
#[repr(transparent)]
pub struct NotificationVisual(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PeriodicUpdateRecurrence(i32);
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
#[repr(C)]
pub struct TileFlyoutTemplateType(i32);
#[repr(transparent)]
pub struct TileFlyoutUpdateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileFlyoutUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileNotification(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TileTemplateType(i32);
#[repr(transparent)]
pub struct TileUpdateManager(pub *mut ::core::ffi::c_void);
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
#[repr(C)]
pub struct ToastDismissalReason(i32);
#[repr(transparent)]
pub struct ToastDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ToastHistoryChangedType(i32);
#[repr(transparent)]
pub struct ToastNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ToastNotificationPriority(i32);
#[repr(transparent)]
pub struct ToastNotifier(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ToastTemplateType(i32);
#[repr(transparent)]
pub struct UserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserNotificationChangedKind(i32);
