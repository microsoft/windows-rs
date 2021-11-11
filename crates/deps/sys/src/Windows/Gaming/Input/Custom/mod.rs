#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GameControllerFactoryManager();
    fn GameControllerVersionInfo();
    fn GipFirmwareUpdateProgress();
    fn GipFirmwareUpdateResult();
    fn GipFirmwareUpdateStatus();
    fn GipGameControllerProvider();
    fn GipMessageClass();
    fn HidGameControllerProvider();
    fn ICustomGameControllerFactory();
    fn IGameControllerFactoryManagerStatics();
    fn IGameControllerFactoryManagerStatics2();
    fn IGameControllerInputSink();
    fn IGameControllerProvider();
    fn IGipFirmwareUpdateResult();
    fn IGipGameControllerInputSink();
    fn IGipGameControllerProvider();
    fn IHidGameControllerInputSink();
    fn IHidGameControllerProvider();
    fn IXusbGameControllerInputSink();
    fn IXusbGameControllerProvider();
    fn XusbDeviceSubtype();
    fn XusbDeviceType();
    fn XusbGameControllerProvider();
}
