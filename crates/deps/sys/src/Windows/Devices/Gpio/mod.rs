#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn GpioChangeCount();
    fn GpioChangeCounter();
    fn GpioChangePolarity();
    fn GpioChangeReader();
    fn GpioChangeRecord();
    fn GpioController();
    fn GpioOpenStatus();
    fn GpioPin();
    fn GpioPinDriveMode();
    fn GpioPinEdge();
    fn GpioPinValue();
    fn GpioPinValueChangedEventArgs();
    fn GpioSharingMode();
    fn IGpioChangeCounter();
    fn IGpioChangeCounterFactory();
    fn IGpioChangeReader();
    fn IGpioChangeReaderFactory();
    fn IGpioController();
    fn IGpioControllerStatics();
    fn IGpioControllerStatics2();
    fn IGpioPin();
    fn IGpioPinValueChangedEventArgs();
}
