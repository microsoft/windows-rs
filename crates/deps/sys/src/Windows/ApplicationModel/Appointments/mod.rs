#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Appointment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentBusyStatus(i32);
#[repr(transparent)]
pub struct AppointmentCalendar(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentCalendarOtherAppReadAccess(i32);
#[repr(C)]
pub struct AppointmentCalendarOtherAppWriteAccess(i32);
#[repr(transparent)]
pub struct AppointmentCalendarSyncManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentCalendarSyncStatus(i32);
#[repr(transparent)]
pub struct AppointmentConflictResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentConflictType(i32);
#[repr(C)]
pub struct AppointmentDaysOfWeek(i32);
#[repr(C)]
pub struct AppointmentDetailsKind(i32);
#[repr(transparent)]
pub struct AppointmentException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentInvitee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentOrganizer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentParticipantResponse(i32);
#[repr(C)]
pub struct AppointmentParticipantRole(i32);
#[repr(transparent)]
pub struct AppointmentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentRecurrence(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentRecurrenceUnit(i32);
#[repr(C)]
pub struct AppointmentSensitivity(i32);
#[repr(transparent)]
pub struct AppointmentStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentStoreAccessType(i32);
#[repr(transparent)]
pub struct AppointmentStoreChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentStoreChangeType(i32);
#[repr(transparent)]
pub struct AppointmentStoreChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppointmentStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppointmentSummaryCardView(i32);
#[repr(C)]
pub struct AppointmentWeekOfMonth(i32);
#[repr(C)]
pub struct FindAppointmentCalendarsOptions(i32);
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
#[repr(C)]
pub struct RecurrenceType(i32);
