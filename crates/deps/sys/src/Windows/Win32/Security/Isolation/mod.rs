#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateAppContainerProfile();
    fn DeleteAppContainerProfile();
    fn DeriveAppContainerSidFromAppContainerName();
    fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName();
    fn GetAppContainerFolderPath();
    fn GetAppContainerNamedObjectPath();
    fn GetAppContainerRegistryLocation();
    fn IIsolatedAppLauncher();
    fn IsProcessInIsolatedContainer();
    fn IsProcessInIsolatedWindowsEnvironment();
    fn IsProcessInWDAGContainer();
    fn IsolatedAppLauncher();
    fn IsolatedAppLauncherTelemetryParameters();
}
