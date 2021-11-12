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
    pub const Busy: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Free: Self = Self(2i32);
    pub const OutOfOffice: Self = Self(3i32);
    pub const WorkingElsewhere: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentBusyStatus {}
impl ::core::clone::Clone for AppointmentBusyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentCalendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppReadAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppWriteAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentCalendarSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentCalendarSyncStatus {}
impl ::core::clone::Clone for AppointmentCalendarSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentConflictResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: Self = Self(0i32);
    pub const Adjacent: Self = Self(1i32);
    pub const Overlap: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentConflictType {}
impl ::core::clone::Clone for AppointmentConflictType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for AppointmentDaysOfWeek {}
impl ::core::clone::Clone for AppointmentDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentDetailsKind {}
impl ::core::clone::Clone for AppointmentDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const None: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Accepted: Self = Self(2i32);
    pub const Declined: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentParticipantResponse {}
impl ::core::clone::Clone for AppointmentParticipantResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: Self = Self(0i32);
    pub const OptionalAttendee: Self = Self(1i32);
    pub const Resource: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentParticipantRole {}
impl ::core::clone::Clone for AppointmentParticipantRole {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentRecurrence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for AppointmentRecurrenceUnit {}
impl ::core::clone::Clone for AppointmentRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSensitivity {}
impl ::core::clone::Clone for AppointmentSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: Self = Self(0i32);
    pub const AllCalendarsReadOnly: Self = Self(1i32);
    pub const AllCalendarsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentStoreAccessType {}
impl ::core::clone::Clone for AppointmentStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const AppointmentCreated: Self = Self(0i32);
    pub const AppointmentModified: Self = Self(1i32);
    pub const AppointmentDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
    pub const CalendarCreated: Self = Self(4i32);
    pub const CalendarModified: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentStoreChangeType {}
impl ::core::clone::Clone for AppointmentStoreChangeType {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const System: Self = Self(0i32);
    pub const App: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSummaryCardView {}
impl ::core::clone::Clone for AppointmentSummaryCardView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentWeekOfMonth {}
impl ::core::clone::Clone for AppointmentWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeHidden: Self = Self(1u32);
}
impl ::core::marker::Copy for FindAppointmentCalendarsOptions {}
impl ::core::clone::Clone for FindAppointmentCalendarsOptions {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Master: Self = Self(0i32);
    pub const Instance: Self = Self(1i32);
    pub const ExceptionInstance: Self = Self(2i32);
}
impl ::core::marker::Copy for RecurrenceType {}
impl ::core::clone::Clone for RecurrenceType {
    fn clone(&self) -> Self {
        *self
    }
}
