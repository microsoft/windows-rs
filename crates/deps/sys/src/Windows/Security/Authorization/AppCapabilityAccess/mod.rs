#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppCapability();
    fn AppCapabilityAccessChangedEventArgs();
    fn AppCapabilityAccessStatus();
    fn IAppCapability();
    fn IAppCapabilityAccessChangedEventArgs();
    fn IAppCapabilityStatics();
}
