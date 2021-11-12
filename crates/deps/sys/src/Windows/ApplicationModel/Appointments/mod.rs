#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Appointment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: AppointmentBusyStatus = AppointmentBusyStatus(0i32);
    pub const Tentative: AppointmentBusyStatus = AppointmentBusyStatus(1i32);
    pub const Free: AppointmentBusyStatus = AppointmentBusyStatus(2i32);
    pub const OutOfOffice: AppointmentBusyStatus = AppointmentBusyStatus(3i32);
    pub const WorkingElsewhere: AppointmentBusyStatus = AppointmentBusyStatus(4i32);
}
#[repr(transparent)]
pub struct AppointmentCalendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(0i32);
    pub const Limited: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(1i32);
    pub const Full: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(2i32);
    pub const None: AppointmentCalendarOtherAppReadAccess = AppointmentCalendarOtherAppReadAccess(3i32);
}
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(0i32);
    pub const SystemOnly: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(1i32);
    pub const Limited: AppointmentCalendarOtherAppWriteAccess = AppointmentCalendarOtherAppWriteAccess(2i32);
}
#[repr(transparent)]
pub struct AppointmentCalendarSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(0i32);
    pub const Syncing: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(1i32);
    pub const UpToDate: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(2i32);
    pub const AuthenticationError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(3i32);
    pub const PolicyError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(4i32);
    pub const UnknownError: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(5i32);
    pub const ManualAccountRemovalRequired: AppointmentCalendarSyncStatus = AppointmentCalendarSyncStatus(6i32);
}
#[repr(transparent)]
pub struct AppointmentConflictResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: AppointmentConflictType = AppointmentConflictType(0i32);
    pub const Adjacent: AppointmentConflictType = AppointmentConflictType(1i32);
    pub const Overlap: AppointmentConflictType = AppointmentConflictType(2i32);
}
#[repr(transparent)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: AppointmentDaysOfWeek = AppointmentDaysOfWeek(0u32);
    pub const Sunday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(1u32);
    pub const Monday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(2u32);
    pub const Tuesday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(4u32);
    pub const Wednesday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(8u32);
    pub const Thursday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(16u32);
    pub const Friday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(32u32);
    pub const Saturday: AppointmentDaysOfWeek = AppointmentDaysOfWeek(64u32);
}
#[repr(transparent)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: AppointmentDetailsKind = AppointmentDetailsKind(0i32);
    pub const Html: AppointmentDetailsKind = AppointmentDetailsKind(1i32);
}
#[repr(transparent)]
pub struct AppointmentException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentInvitee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentOrganizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: AppointmentParticipantResponse = AppointmentParticipantResponse(0i32);
    pub const Tentative: AppointmentParticipantResponse = AppointmentParticipantResponse(1i32);
    pub const Accepted: AppointmentParticipantResponse = AppointmentParticipantResponse(2i32);
    pub const Declined: AppointmentParticipantResponse = AppointmentParticipantResponse(3i32);
    pub const Unknown: AppointmentParticipantResponse = AppointmentParticipantResponse(4i32);
}
#[repr(transparent)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: AppointmentParticipantRole = AppointmentParticipantRole(0i32);
    pub const OptionalAttendee: AppointmentParticipantRole = AppointmentParticipantRole(1i32);
    pub const Resource: AppointmentParticipantRole = AppointmentParticipantRole(2i32);
}
#[repr(transparent)]
pub struct AppointmentRecurrence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(0i32);
    pub const Weekly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(1i32);
    pub const Monthly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(2i32);
    pub const MonthlyOnDay: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(3i32);
    pub const Yearly: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(4i32);
    pub const YearlyOnDay: AppointmentRecurrenceUnit = AppointmentRecurrenceUnit(5i32);
}
#[repr(transparent)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: AppointmentSensitivity = AppointmentSensitivity(0i32);
    pub const Private: AppointmentSensitivity = AppointmentSensitivity(1i32);
}
#[repr(transparent)]
pub struct AppointmentStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: AppointmentStoreAccessType = AppointmentStoreAccessType(0i32);
    pub const AllCalendarsReadOnly: AppointmentStoreAccessType = AppointmentStoreAccessType(1i32);
    pub const AllCalendarsReadWrite: AppointmentStoreAccessType = AppointmentStoreAccessType(2i32);
}
#[repr(transparent)]
pub struct AppointmentStoreChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: AppointmentStoreChangeType = AppointmentStoreChangeType(0i32);
    pub const AppointmentModified: AppointmentStoreChangeType = AppointmentStoreChangeType(1i32);
    pub const AppointmentDeleted: AppointmentStoreChangeType = AppointmentStoreChangeType(2i32);
    pub const ChangeTrackingLost: AppointmentStoreChangeType = AppointmentStoreChangeType(3i32);
    pub const CalendarCreated: AppointmentStoreChangeType = AppointmentStoreChangeType(4i32);
    pub const CalendarModified: AppointmentStoreChangeType = AppointmentStoreChangeType(5i32);
    pub const CalendarDeleted: AppointmentStoreChangeType = AppointmentStoreChangeType(6i32);
}
#[repr(transparent)]
pub struct AppointmentStoreChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: AppointmentSummaryCardView = AppointmentSummaryCardView(0i32);
    pub const App: AppointmentSummaryCardView = AppointmentSummaryCardView(1i32);
}
#[repr(transparent)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: AppointmentWeekOfMonth = AppointmentWeekOfMonth(0i32);
    pub const Second: AppointmentWeekOfMonth = AppointmentWeekOfMonth(1i32);
    pub const Third: AppointmentWeekOfMonth = AppointmentWeekOfMonth(2i32);
    pub const Fourth: AppointmentWeekOfMonth = AppointmentWeekOfMonth(3i32);
    pub const Last: AppointmentWeekOfMonth = AppointmentWeekOfMonth(4i32);
}
#[repr(transparent)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: FindAppointmentCalendarsOptions = FindAppointmentCalendarsOptions(0u32);
    pub const IncludeHidden: FindAppointmentCalendarsOptions = FindAppointmentCalendarsOptions(1u32);
}
#[repr(transparent)]
pub struct FindAppointmentsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointment3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentCalendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentCalendar2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentCalendar3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentConflictResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentInvitee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentParticipant(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentRecurrence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentRecurrence2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentRecurrence3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStore3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChange2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChangeTracker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFindAppointmentsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: RecurrenceType = RecurrenceType(0i32);
    pub const Instance: RecurrenceType = RecurrenceType(1i32);
    pub const ExceptionInstance: RecurrenceType = RecurrenceType(2i32);
}
