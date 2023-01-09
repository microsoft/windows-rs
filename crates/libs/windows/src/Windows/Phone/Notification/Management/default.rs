impl ::core::default::Default for AccessoryNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AccessoryNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessoryNotificationType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for AlarmNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlarmNotificationTriggerDetails {}
impl ::core::fmt::Debug for AlarmNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlarmNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppNotificationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationInfo {}
impl ::core::fmt::Debug for AppNotificationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for BinaryId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BinaryId {}
impl ::core::fmt::Debug for BinaryId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BinaryId").field(&self.0).finish()
    }
}
impl ::core::default::Default for CalendarChangedEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CalendarChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CalendarChangedNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CalendarChangedNotificationTriggerDetails {}
impl ::core::fmt::Debug for CalendarChangedNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CortanaTileNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CortanaTileNotificationTriggerDetails {}
impl ::core::fmt::Debug for CortanaTileNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaTileNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for EmailAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailAccountInfo {}
impl ::core::fmt::Debug for EmailAccountInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAccountInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for EmailFolderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailFolderInfo {}
impl ::core::fmt::Debug for EmailFolderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFolderInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for EmailNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for EmailReadNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailReadNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailReadNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailReadNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessoryNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessoryNotificationTriggerDetails {}
impl ::core::fmt::Debug for IAccessoryNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessoryNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MediaControlsTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaControlsTriggerDetails {}
impl ::core::fmt::Debug for MediaControlsTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaControlsTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MediaMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaMetadata {}
impl ::core::fmt::Debug for MediaMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMetadata").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneCallAudioEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneCallAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioEndpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneCallDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallDetails {}
impl ::core::fmt::Debug for PhoneCallDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneCallDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneCallState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallState").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneCallTransport {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneCallTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneLineDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineDetails {}
impl ::core::fmt::Debug for PhoneLineDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneLineRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneLineRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineRegistrationState").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneMediaType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneMediaType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PlaybackCapability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PlaybackCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCapability").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PlaybackCommand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PlaybackCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCommand").field(&self.0).finish()
    }
}
impl ::core::default::Default for PlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ReminderNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReminderNotificationTriggerDetails {}
impl ::core::fmt::Debug for ReminderNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for ReminderState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ReminderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpeedDialEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeedDialEntry {}
impl ::core::fmt::Debug for SpeedDialEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeedDialEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TextResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextResponse {}
impl ::core::fmt::Debug for TextResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextResponse").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ToastNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationTriggerDetails {}
impl ::core::fmt::Debug for ToastNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for VibrateState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VibrateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrateState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VolumeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeInfo {}
impl ::core::fmt::Debug for VolumeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeInfo").field(&self.0).finish()
    }
}
