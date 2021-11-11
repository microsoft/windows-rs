#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AssignProcessToJobObject();
    fn CreateJobObjectA();
    fn CreateJobObjectW();
    fn CreateJobSet();
    fn FreeMemoryJobObject();
    fn IsProcessInJob();
    fn OpenJobObjectA();
    fn OpenJobObjectW();
    fn QueryInformationJobObject();
    fn QueryIoRateControlInformationJobObject();
    fn SetInformationJobObject();
    fn SetIoRateControlInformationJobObject();
    fn TerminateJobObject();
    fn UserHandleGrantAccess();
}
