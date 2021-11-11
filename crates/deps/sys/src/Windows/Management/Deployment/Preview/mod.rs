#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ClassicAppManager();
    fn DeploymentPreviewContract();
    fn IClassicAppManagerStatics();
    fn IInstalledClassicAppInfo();
    fn InstalledClassicAppInfo();
}
