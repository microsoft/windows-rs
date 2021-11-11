#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GpioPinProviderValueChangedEventArgs();
    fn IGpioControllerProvider();
    fn IGpioPinProvider();
    fn IGpioPinProviderValueChangedEventArgs();
    fn IGpioPinProviderValueChangedEventArgsFactory();
    fn IGpioProvider();
    fn ProviderGpioPinDriveMode();
    fn ProviderGpioPinEdge();
    fn ProviderGpioPinValue();
    fn ProviderGpioSharingMode();
}
