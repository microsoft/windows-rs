#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IRfcommDeviceService();
    fn IRfcommDeviceService2();
    fn IRfcommDeviceService3();
    fn IRfcommDeviceServiceStatics();
    fn IRfcommDeviceServiceStatics2();
    fn IRfcommDeviceServicesResult();
    fn IRfcommServiceId();
    fn IRfcommServiceIdStatics();
    fn IRfcommServiceProvider();
    fn IRfcommServiceProvider2();
    fn IRfcommServiceProviderStatics();
    fn RfcommDeviceService();
    fn RfcommDeviceServicesResult();
    fn RfcommServiceId();
    fn RfcommServiceProvider();
}
