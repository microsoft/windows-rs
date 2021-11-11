#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BackPressedEventArgs();
    fn CameraEventArgs();
    fn HardwareButtons();
    fn IBackPressedEventArgs();
    fn ICameraEventArgs();
    fn IHardwareButtonsStatics();
    fn IHardwareButtonsStatics2();
}
