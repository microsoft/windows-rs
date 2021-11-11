#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn K32EmptyWorkingSet();
    fn K32EnumDeviceDrivers();
    fn K32EnumPageFilesA();
    fn K32EnumPageFilesW();
    fn K32EnumProcessModules();
    fn K32EnumProcessModulesEx();
    fn K32EnumProcesses();
    fn K32GetDeviceDriverBaseNameA();
    fn K32GetDeviceDriverBaseNameW();
    fn K32GetDeviceDriverFileNameA();
    fn K32GetDeviceDriverFileNameW();
    fn K32GetMappedFileNameA();
    fn K32GetMappedFileNameW();
    fn K32GetModuleBaseNameA();
    fn K32GetModuleBaseNameW();
    fn K32GetModuleFileNameExA();
    fn K32GetModuleFileNameExW();
    fn K32GetModuleInformation();
    fn K32GetPerformanceInfo();
    fn K32GetProcessImageFileNameA();
    fn K32GetProcessImageFileNameW();
    fn K32GetProcessMemoryInfo();
    fn K32GetWsChanges();
    fn K32GetWsChangesEx();
    fn K32InitializeProcessForWsWatch();
    fn K32QueryWorkingSet();
    fn K32QueryWorkingSetEx();
}
