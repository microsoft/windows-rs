#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BroadcastSystemMessageA();
    fn BroadcastSystemMessageExA();
    fn BroadcastSystemMessageExW();
    fn BroadcastSystemMessageW();
    fn CloseDesktop();
    fn CloseWindowStation();
    fn CreateDesktopA();
    fn CreateDesktopExA();
    fn CreateDesktopExW();
    fn CreateDesktopW();
    fn CreateWindowStationA();
    fn CreateWindowStationW();
    fn EnumDesktopWindows();
    fn EnumDesktopsA();
    fn EnumDesktopsW();
    fn EnumWindowStationsA();
    fn EnumWindowStationsW();
    fn GetProcessWindowStation();
    fn GetThreadDesktop();
    fn GetUserObjectInformationA();
    fn GetUserObjectInformationW();
    fn OpenDesktopA();
    fn OpenDesktopW();
    fn OpenInputDesktop();
    fn OpenWindowStationA();
    fn OpenWindowStationW();
    fn SetProcessWindowStation();
    fn SetThreadDesktop();
    fn SetUserObjectInformationA();
    fn SetUserObjectInformationW();
    fn SwitchDesktop();
}
