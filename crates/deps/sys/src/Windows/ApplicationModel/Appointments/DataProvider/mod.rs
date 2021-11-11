#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppointmentCalendarCancelMeetingRequest();
    fn AppointmentCalendarCancelMeetingRequestEventArgs();
    fn AppointmentCalendarCreateOrUpdateAppointmentRequest();
    fn AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs();
    fn AppointmentCalendarForwardMeetingRequest();
    fn AppointmentCalendarForwardMeetingRequestEventArgs();
    fn AppointmentCalendarProposeNewTimeForMeetingRequest();
    fn AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs();
    fn AppointmentCalendarSyncManagerSyncRequest();
    fn AppointmentCalendarSyncManagerSyncRequestEventArgs();
    fn AppointmentCalendarUpdateMeetingResponseRequest();
    fn AppointmentCalendarUpdateMeetingResponseRequestEventArgs();
    fn AppointmentDataProviderConnection();
    fn AppointmentDataProviderTriggerDetails();
    fn IAppointmentCalendarCancelMeetingRequest();
    fn IAppointmentCalendarCancelMeetingRequestEventArgs();
    fn IAppointmentCalendarCreateOrUpdateAppointmentRequest();
    fn IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs();
    fn IAppointmentCalendarForwardMeetingRequest();
    fn IAppointmentCalendarForwardMeetingRequestEventArgs();
    fn IAppointmentCalendarProposeNewTimeForMeetingRequest();
    fn IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs();
    fn IAppointmentCalendarSyncManagerSyncRequest();
    fn IAppointmentCalendarSyncManagerSyncRequestEventArgs();
    fn IAppointmentCalendarUpdateMeetingResponseRequest();
    fn IAppointmentCalendarUpdateMeetingResponseRequestEventArgs();
    fn IAppointmentDataProviderConnection();
    fn IAppointmentDataProviderTriggerDetails();
}
