#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppointmentCalendarCancelMeetingRequest(i32);
pub struct AppointmentCalendarCancelMeetingRequestEventArgs(i32);
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequest(i32);
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(i32);
pub struct AppointmentCalendarForwardMeetingRequest(i32);
pub struct AppointmentCalendarForwardMeetingRequestEventArgs(i32);
pub struct AppointmentCalendarProposeNewTimeForMeetingRequest(i32);
pub struct AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(i32);
pub struct AppointmentCalendarSyncManagerSyncRequest(i32);
pub struct AppointmentCalendarSyncManagerSyncRequestEventArgs(i32);
pub struct AppointmentCalendarUpdateMeetingResponseRequest(i32);
pub struct AppointmentCalendarUpdateMeetingResponseRequestEventArgs(i32);
pub struct AppointmentDataProviderConnection(i32);
pub struct AppointmentDataProviderTriggerDetails(i32);
pub struct IAppointmentCalendarCancelMeetingRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarForwardMeetingRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarUpdateMeetingResponseRequest(pub *mut ::core::ffi::c_void);
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppointmentDataProviderConnection(pub *mut ::core::ffi::c_void);
pub struct IAppointmentDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
