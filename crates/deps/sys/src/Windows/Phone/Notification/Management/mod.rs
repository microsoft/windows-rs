#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AccessoryNotificationType(i32);
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppNotificationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BinaryId(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CalendarChangedEvent(i32);
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
#[repr(C)]
pub struct PhoneCallAudioEndpoint(i32);
#[repr(transparent)]
pub struct PhoneCallDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallDirection(i32);
#[repr(C)]
pub struct PhoneCallState(i32);
#[repr(C)]
pub struct PhoneCallTransport(i32);
#[repr(transparent)]
pub struct PhoneLineDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneLineRegistrationState(i32);
#[repr(C)]
pub struct PhoneMediaType(i32);
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneNotificationType(i32);
#[repr(C)]
pub struct PlaybackCapability(i32);
#[repr(C)]
pub struct PlaybackCommand(i32);
#[repr(C)]
pub struct PlaybackStatus(i32);
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ReminderState(i32);
#[repr(transparent)]
pub struct SpeedDialEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VibrateState(i32);
#[repr(transparent)]
pub struct VolumeInfo(pub *mut ::core::ffi::c_void);
