#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ISysStorageProviderEventReceivedEventArgs();
    fn ISysStorageProviderEventReceivedEventArgsFactory();
    fn ISysStorageProviderEventSource();
    fn ISysStorageProviderHandlerFactory();
    fn ISysStorageProviderHttpRequestProvider();
    fn SysStorageProviderEventReceivedEventArgs();
}
