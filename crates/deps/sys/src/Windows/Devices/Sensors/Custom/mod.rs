#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CustomSensor();
    fn CustomSensorReading();
    fn CustomSensorReadingChangedEventArgs();
    fn ICustomSensor();
    fn ICustomSensor2();
    fn ICustomSensorReading();
    fn ICustomSensorReading2();
    fn ICustomSensorReadingChangedEventArgs();
    fn ICustomSensorStatics();
}
