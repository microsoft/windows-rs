impl ::core::default::Default for AdaptiveNotificationContentKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AdaptiveNotificationContentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationContentKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AdaptiveNotificationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveNotificationText {}
impl ::core::fmt::Debug for AdaptiveNotificationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for BadgeNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeNotification {}
impl ::core::fmt::Debug for BadgeNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for BadgeTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BadgeTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeTemplateType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for BadgeUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdateManagerForUser {}
impl ::core::fmt::Debug for BadgeUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdateManagerForUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for BadgeUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdater {}
impl ::core::fmt::Debug for BadgeUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdater").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAdaptiveNotificationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveNotificationContent {}
impl ::core::fmt::Debug for IAdaptiveNotificationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveNotificationContent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Notification {}
impl ::core::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Notification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NotificationBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationBinding {}
impl ::core::fmt::Debug for NotificationBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationBinding").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NotificationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationData {}
impl ::core::fmt::Debug for NotificationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationData").field(&self.0).finish()
    }
}
impl ::core::default::Default for NotificationKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NotificationMirroring {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationMirroring {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationMirroring").field(&self.0).finish()
    }
}
impl ::core::default::Default for NotificationSetting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationSetting").field(&self.0).finish()
    }
}
impl ::core::default::Default for NotificationUpdateResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationUpdateResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NotificationVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationVisual {}
impl ::core::fmt::Debug for NotificationVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationVisual").field(&self.0).finish()
    }
}
impl ::core::default::Default for PeriodicUpdateRecurrence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PeriodicUpdateRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicUpdateRecurrence").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ScheduledTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledTileNotification {}
impl ::core::fmt::Debug for ScheduledTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledTileNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotification {}
impl ::core::fmt::Debug for ScheduledToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotificationShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotificationShowingEventArgs {}
impl ::core::fmt::Debug for ScheduledToastNotificationShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotificationShowingEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ShownTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShownTileNotification {}
impl ::core::fmt::Debug for ShownTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShownTileNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileFlyoutNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutNotification {}
impl ::core::fmt::Debug for TileFlyoutNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for TileFlyoutTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TileFlyoutTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutTemplateType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileFlyoutUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutUpdater {}
impl ::core::fmt::Debug for TileFlyoutUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutUpdater").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileNotification {}
impl ::core::fmt::Debug for TileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for TileTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TileTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileTemplateType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdateManagerForUser {}
impl ::core::fmt::Debug for TileUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdateManagerForUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdater {}
impl ::core::fmt::Debug for TileUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdater").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastActivatedEventArgs {}
impl ::core::fmt::Debug for ToastActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollection {}
impl ::core::fmt::Debug for ToastCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastCollectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollectionManager {}
impl ::core::fmt::Debug for ToastCollectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollectionManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToastDismissalReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToastDismissalReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissalReason").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastDismissedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastDismissedEventArgs {}
impl ::core::fmt::Debug for ToastDismissedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastFailedEventArgs {}
impl ::core::fmt::Debug for ToastFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastFailedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToastHistoryChangedType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToastHistoryChangedType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastHistoryChangedType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotification {}
impl ::core::fmt::Debug for ToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActionTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActionTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationActionTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActionTriggerDetail").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistory {}
impl ::core::fmt::Debug for ToastNotificationHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistoryChangedTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistoryChangedTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationHistoryChangedTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistoryChangedTriggerDetail").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotificationManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationManagerForUser {}
impl ::core::fmt::Debug for ToastNotificationManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationManagerForUser").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToastNotificationPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToastNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationPriority").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotifier {}
impl ::core::fmt::Debug for ToastNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotifier").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToastTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToastTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastTemplateType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotification {}
impl ::core::fmt::Debug for UserNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserNotificationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationChangedEventArgs {}
impl ::core::fmt::Debug for UserNotificationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for UserNotificationChangedKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UserNotificationChangedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedKind").field(&self.0).finish()
    }
}
