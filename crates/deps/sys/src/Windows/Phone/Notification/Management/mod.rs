#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: AccessoryNotificationType = AccessoryNotificationType(0u32);
    pub const Phone: AccessoryNotificationType = AccessoryNotificationType(1u32);
    pub const Email: AccessoryNotificationType = AccessoryNotificationType(2u32);
    pub const Reminder: AccessoryNotificationType = AccessoryNotificationType(4u32);
    pub const Alarm: AccessoryNotificationType = AccessoryNotificationType(8u32);
    pub const Toast: AccessoryNotificationType = AccessoryNotificationType(16u32);
    pub const AppUninstalled: AccessoryNotificationType = AccessoryNotificationType(32u32);
    pub const Dnd: AccessoryNotificationType = AccessoryNotificationType(64u32);
    pub const DrivingMode: AccessoryNotificationType = AccessoryNotificationType(128u32);
    pub const BatterySaver: AccessoryNotificationType = AccessoryNotificationType(256u32);
    pub const Media: AccessoryNotificationType = AccessoryNotificationType(512u32);
    pub const CortanaTile: AccessoryNotificationType = AccessoryNotificationType(1024u32);
    pub const ToastCleared: AccessoryNotificationType = AccessoryNotificationType(2048u32);
    pub const CalendarChanged: AccessoryNotificationType = AccessoryNotificationType(4096u32);
    pub const VolumeChanged: AccessoryNotificationType = AccessoryNotificationType(8192u32);
    pub const EmailReadStatusChanged: AccessoryNotificationType = AccessoryNotificationType(16384u32);
}
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppNotificationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BinaryId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: CalendarChangedEvent = CalendarChangedEvent(0i32);
    pub const AppointmentAdded: CalendarChangedEvent = CalendarChangedEvent(1i32);
    pub const AppointmentChanged: CalendarChangedEvent = CalendarChangedEvent(2i32);
    pub const AppointmentDeleted: CalendarChangedEvent = CalendarChangedEvent(3i32);
    pub const CalendarAdded: CalendarChangedEvent = CalendarChangedEvent(4i32);
    pub const CalendarChanged: CalendarChangedEvent = CalendarChangedEvent(5i32);
    pub const CalendarDeleted: CalendarChangedEvent = CalendarChangedEvent(6i32);
}
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailAccountInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailFolderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessoryManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessoryManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessoryManager3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppNotificationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBinaryId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAccountInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailFolderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeedDialEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVolumeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(0i32);
    pub const Speaker: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(1i32);
    pub const Handsfree: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(2i32);
}
#[repr(transparent)]
pub struct PhoneCallDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(1i32);
}
#[repr(transparent)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: PhoneCallState = PhoneCallState(0i32);
    pub const Ringing: PhoneCallState = PhoneCallState(1i32);
    pub const Talking: PhoneCallState = PhoneCallState(2i32);
    pub const Held: PhoneCallState = PhoneCallState(3i32);
    pub const Ended: PhoneCallState = PhoneCallState(4i32);
}
#[repr(transparent)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: PhoneCallTransport = PhoneCallTransport(0i32);
    pub const Voip: PhoneCallTransport = PhoneCallTransport(1i32);
}
#[repr(transparent)]
pub struct PhoneLineDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: PhoneLineRegistrationState = PhoneLineRegistrationState(0i32);
    pub const Home: PhoneLineRegistrationState = PhoneLineRegistrationState(1i32);
    pub const Roaming: PhoneLineRegistrationState = PhoneLineRegistrationState(2i32);
}
#[repr(transparent)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: PhoneMediaType = PhoneMediaType(0i32);
    pub const AudioVideo: PhoneMediaType = PhoneMediaType(1i32);
}
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: PhoneNotificationType = PhoneNotificationType(0i32);
    pub const CallChanged: PhoneNotificationType = PhoneNotificationType(1i32);
    pub const LineChanged: PhoneNotificationType = PhoneNotificationType(2i32);
    pub const PhoneCallAudioEndpointChanged: PhoneNotificationType = PhoneNotificationType(3i32);
    pub const PhoneMuteChanged: PhoneNotificationType = PhoneNotificationType(4i32);
}
#[repr(transparent)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: PlaybackCapability = PlaybackCapability(0u32);
    pub const Play: PlaybackCapability = PlaybackCapability(1u32);
    pub const Pause: PlaybackCapability = PlaybackCapability(2u32);
    pub const Stop: PlaybackCapability = PlaybackCapability(4u32);
    pub const Record: PlaybackCapability = PlaybackCapability(8u32);
    pub const FastForward: PlaybackCapability = PlaybackCapability(16u32);
    pub const Rewind: PlaybackCapability = PlaybackCapability(32u32);
    pub const Next: PlaybackCapability = PlaybackCapability(64u32);
    pub const Previous: PlaybackCapability = PlaybackCapability(128u32);
    pub const ChannelUp: PlaybackCapability = PlaybackCapability(256u32);
    pub const ChannelDown: PlaybackCapability = PlaybackCapability(512u32);
}
#[repr(transparent)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: PlaybackCommand = PlaybackCommand(0i32);
    pub const Pause: PlaybackCommand = PlaybackCommand(1i32);
    pub const Stop: PlaybackCommand = PlaybackCommand(2i32);
    pub const Record: PlaybackCommand = PlaybackCommand(3i32);
    pub const FastForward: PlaybackCommand = PlaybackCommand(4i32);
    pub const Rewind: PlaybackCommand = PlaybackCommand(5i32);
    pub const Next: PlaybackCommand = PlaybackCommand(6i32);
    pub const Previous: PlaybackCommand = PlaybackCommand(7i32);
    pub const ChannelUp: PlaybackCommand = PlaybackCommand(8i32);
    pub const ChannelDown: PlaybackCommand = PlaybackCommand(9i32);
}
#[repr(transparent)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: PlaybackStatus = PlaybackStatus(0i32);
    pub const TrackChanged: PlaybackStatus = PlaybackStatus(1i32);
    pub const Stopped: PlaybackStatus = PlaybackStatus(2i32);
    pub const Playing: PlaybackStatus = PlaybackStatus(3i32);
    pub const Paused: PlaybackStatus = PlaybackStatus(4i32);
}
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: ReminderState = ReminderState(0i32);
    pub const Snoozed: ReminderState = ReminderState(1i32);
    pub const Dismissed: ReminderState = ReminderState(2i32);
}
#[repr(transparent)]
pub struct SpeedDialEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: VibrateState = VibrateState(0i32);
    pub const RingerOffVibrateOn: VibrateState = VibrateState(1i32);
    pub const RingerOnVibrateOff: VibrateState = VibrateState(2i32);
    pub const RingerOnVibrateOn: VibrateState = VibrateState(3i32);
}
#[repr(transparent)]
pub struct VolumeInfo(pub *mut ::core::ffi::c_void);
