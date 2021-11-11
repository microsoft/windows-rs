#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GazeDeviceConfigurationStatePreview();
    fn GazeDevicePreview();
    fn GazeDeviceWatcherAddedPreviewEventArgs();
    fn GazeDeviceWatcherPreview();
    fn GazeDeviceWatcherRemovedPreviewEventArgs();
    fn GazeDeviceWatcherUpdatedPreviewEventArgs();
    fn GazeEnteredPreviewEventArgs();
    fn GazeExitedPreviewEventArgs();
    fn GazeInputSourcePreview();
    fn GazeMovedPreviewEventArgs();
    fn GazePointPreview();
    fn IGazeDevicePreview();
    fn IGazeDeviceWatcherAddedPreviewEventArgs();
    fn IGazeDeviceWatcherPreview();
    fn IGazeDeviceWatcherRemovedPreviewEventArgs();
    fn IGazeDeviceWatcherUpdatedPreviewEventArgs();
    fn IGazeEnteredPreviewEventArgs();
    fn IGazeExitedPreviewEventArgs();
    fn IGazeInputSourcePreview();
    fn IGazeInputSourcePreviewStatics();
    fn IGazeMovedPreviewEventArgs();
    fn IGazePointPreview();
}
