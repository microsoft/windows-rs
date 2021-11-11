#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddAppointmentOperation();
    fn AppointmentsProviderLaunchActionVerbs();
    fn IAddAppointmentOperation();
    fn IAppointmentsProviderLaunchActionVerbsStatics();
    fn IAppointmentsProviderLaunchActionVerbsStatics2();
    fn IRemoveAppointmentOperation();
    fn IReplaceAppointmentOperation();
    fn RemoveAppointmentOperation();
    fn ReplaceAppointmentOperation();
}
