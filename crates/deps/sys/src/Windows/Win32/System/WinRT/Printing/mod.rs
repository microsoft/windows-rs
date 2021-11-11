#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPrintManagerInterop();
    fn IPrintWorkflowConfigurationNative();
    fn IPrintWorkflowObjectModelSourceFileContentNative();
    fn IPrintWorkflowXpsObjectModelTargetPackageNative();
    fn IPrintWorkflowXpsReceiver();
    fn IPrintWorkflowXpsReceiver2();
    fn IPrinting3DManagerInterop();
}
