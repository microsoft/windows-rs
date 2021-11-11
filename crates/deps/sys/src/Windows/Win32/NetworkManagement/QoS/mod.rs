#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn QOSAddSocketToFlow();
    fn QOSCancel();
    fn QOSCloseHandle();
    fn QOSCreateHandle();
    fn QOSEnumerateFlows();
    fn QOSNotifyFlow();
    fn QOSQueryFlow();
    fn QOSRemoveSocketFromFlow();
    fn QOSSetFlow();
    fn QOSStartTrackingClient();
    fn QOSStopTrackingClient();
    fn TcAddFilter();
    fn TcAddFlow();
    fn TcCloseInterface();
    fn TcDeleteFilter();
    fn TcDeleteFlow();
    fn TcDeregisterClient();
    fn TcEnumerateFlows();
    fn TcEnumerateInterfaces();
    fn TcGetFlowNameA();
    fn TcGetFlowNameW();
    fn TcModifyFlow();
    fn TcOpenInterfaceA();
    fn TcOpenInterfaceW();
    fn TcQueryFlowA();
    fn TcQueryFlowW();
    fn TcQueryInterface();
    fn TcRegisterClient();
    fn TcSetFlowA();
    fn TcSetFlowW();
    fn TcSetInterface();
}
