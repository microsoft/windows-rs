#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: Self = Self(0u32);
    pub const Phone: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Reminder: Self = Self(4u32);
    pub const Alarm: Self = Self(8u32);
    pub const Toast: Self = Self(16u32);
    pub const AppUninstalled: Self = Self(32u32);
    pub const Dnd: Self = Self(64u32);
    pub const DrivingMode: Self = Self(128u32);
    pub const BatterySaver: Self = Self(256u32);
    pub const Media: Self = Self(512u32);
    pub const CortanaTile: Self = Self(1024u32);
    pub const ToastCleared: Self = Self(2048u32);
    pub const CalendarChanged: Self = Self(4096u32);
    pub const VolumeChanged: Self = Self(8192u32);
    pub const EmailReadStatusChanged: Self = Self(16384u32);
}
impl ::core::marker::Copy for AccessoryNotificationType {}
impl ::core::clone::Clone for AccessoryNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AlarmNotificationTriggerDetails {}
impl ::core::clone::Clone for AlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppNotificationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppNotificationInfo {}
impl ::core::clone::Clone for AppNotificationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BinaryId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BinaryId {}
impl ::core::clone::Clone for BinaryId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: Self = Self(0i32);
    pub const AppointmentAdded: Self = Self(1i32);
    pub const AppointmentChanged: Self = Self(2i32);
    pub const AppointmentDeleted: Self = Self(3i32);
    pub const CalendarAdded: Self = Self(4i32);
    pub const CalendarChanged: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for CalendarChangedEvent {}
impl ::core::clone::Clone for CalendarChangedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarChangedNotificationTriggerDetails {}
impl ::core::clone::Clone for CalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CortanaTileNotificationTriggerDetails {}
impl ::core::clone::Clone for CortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailAccountInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailAccountInfo {}
impl ::core::clone::Clone for EmailAccountInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailFolderInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailFolderInfo {}
impl ::core::clone::Clone for EmailFolderInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailNotificationTriggerDetails {}
impl ::core::clone::Clone for EmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailReadNotificationTriggerDetails {}
impl ::core::clone::Clone for EmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccessoryManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccessoryManager {}
impl ::core::clone::Clone for IAccessoryManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccessoryManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccessoryManager2 {}
impl ::core::clone::Clone for IAccessoryManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccessoryManager3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccessoryManager3 {}
impl ::core::clone::Clone for IAccessoryManager3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccessoryNotificationTriggerDetails {}
impl ::core::clone::Clone for IAccessoryNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAlarmNotificationTriggerDetails {}
impl ::core::clone::Clone for IAlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAlarmNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IAlarmNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppNotificationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppNotificationInfo {}
impl ::core::clone::Clone for IAppNotificationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinaryId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinaryId {}
impl ::core::clone::Clone for IBinaryId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarChangedNotificationTriggerDetails {}
impl ::core::clone::Clone for ICalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaTileNotificationTriggerDetails {}
impl ::core::clone::Clone for ICortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailAccountInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailAccountInfo {}
impl ::core::clone::Clone for IEmailAccountInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailFolderInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailFolderInfo {}
impl ::core::clone::Clone for IEmailFolderInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailNotificationTriggerDetails {}
impl ::core::clone::Clone for IEmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IEmailNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailReadNotificationTriggerDetails {}
impl ::core::clone::Clone for IEmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaControlsTriggerDetails {}
impl ::core::clone::Clone for IMediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaMetadata {}
impl ::core::clone::Clone for IMediaMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallDetails {}
impl ::core::clone::Clone for IPhoneCallDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineDetails {}
impl ::core::clone::Clone for IPhoneLineDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineDetails2 {}
impl ::core::clone::Clone for IPhoneLineDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneNotificationTriggerDetails {}
impl ::core::clone::Clone for IPhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReminderNotificationTriggerDetails {}
impl ::core::clone::Clone for IReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReminderNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IReminderNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeedDialEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeedDialEntry {}
impl ::core::clone::Clone for ISpeedDialEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextResponse {}
impl ::core::clone::Clone for ITextResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationTriggerDetails {}
impl ::core::clone::Clone for IToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IToastNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVolumeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVolumeInfo {}
impl ::core::clone::Clone for IVolumeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaControlsTriggerDetails {}
impl ::core::clone::Clone for MediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaMetadata {}
impl ::core::clone::Clone for MediaMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Speaker: Self = Self(1i32);
    pub const Handsfree: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioEndpoint {}
impl ::core::clone::Clone for PhoneCallAudioEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallDetails {}
impl ::core::clone::Clone for PhoneCallDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: Self = Self(0i32);
    pub const Outgoing: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: Self = Self(0i32);
    pub const Ringing: Self = Self(1i32);
    pub const Talking: Self = Self(2i32);
    pub const Held: Self = Self(3i32);
    pub const Ended: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallState {}
impl ::core::clone::Clone for PhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: Self = Self(0i32);
    pub const Voip: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallTransport {}
impl ::core::clone::Clone for PhoneCallTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineDetails {}
impl ::core::clone::Clone for PhoneLineDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Roaming: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineRegistrationState {}
impl ::core::clone::Clone for PhoneLineRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: Self = Self(0i32);
    pub const AudioVideo: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneMediaType {}
impl ::core::clone::Clone for PhoneMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneNotificationTriggerDetails {}
impl ::core::clone::Clone for PhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: Self = Self(0i32);
    pub const CallChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const PhoneCallAudioEndpointChanged: Self = Self(3i32);
    pub const PhoneMuteChanged: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNotificationType {}
impl ::core::clone::Clone for PhoneNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: Self = Self(0u32);
    pub const Play: Self = Self(1u32);
    pub const Pause: Self = Self(2u32);
    pub const Stop: Self = Self(4u32);
    pub const Record: Self = Self(8u32);
    pub const FastForward: Self = Self(16u32);
    pub const Rewind: Self = Self(32u32);
    pub const Next: Self = Self(64u32);
    pub const Previous: Self = Self(128u32);
    pub const ChannelUp: Self = Self(256u32);
    pub const ChannelDown: Self = Self(512u32);
}
impl ::core::marker::Copy for PlaybackCapability {}
impl ::core::clone::Clone for PlaybackCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for PlaybackCommand {}
impl ::core::clone::Clone for PlaybackCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: Self = Self(0i32);
    pub const TrackChanged: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for PlaybackStatus {}
impl ::core::clone::Clone for PlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReminderNotificationTriggerDetails {}
impl ::core::clone::Clone for ReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: Self = Self(0i32);
    pub const Snoozed: Self = Self(1i32);
    pub const Dismissed: Self = Self(2i32);
}
impl ::core::marker::Copy for ReminderState {}
impl ::core::clone::Clone for ReminderState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeedDialEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeedDialEntry {}
impl ::core::clone::Clone for SpeedDialEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextResponse {}
impl ::core::clone::Clone for TextResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationTriggerDetails {}
impl ::core::clone::Clone for ToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: Self = Self(0i32);
    pub const RingerOffVibrateOn: Self = Self(1i32);
    pub const RingerOnVibrateOff: Self = Self(2i32);
    pub const RingerOnVibrateOn: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrateState {}
impl ::core::clone::Clone for VibrateState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VolumeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VolumeInfo {}
impl ::core::clone::Clone for VolumeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
