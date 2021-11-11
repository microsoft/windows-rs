#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppServiceCatalog();
    fn AppServiceClosedEventArgs();
    fn AppServiceClosedStatus();
    fn AppServiceConnection();
    fn AppServiceConnectionStatus();
    fn AppServiceDeferral();
    fn AppServiceRequest();
    fn AppServiceRequestReceivedEventArgs();
    fn AppServiceResponse();
    fn AppServiceResponseStatus();
    fn AppServiceTriggerDetails();
    fn IAppServiceCatalogStatics();
    fn IAppServiceClosedEventArgs();
    fn IAppServiceConnection();
    fn IAppServiceConnection2();
    fn IAppServiceConnectionStatics();
    fn IAppServiceDeferral();
    fn IAppServiceRequest();
    fn IAppServiceRequestReceivedEventArgs();
    fn IAppServiceResponse();
    fn IAppServiceTriggerDetails();
    fn IAppServiceTriggerDetails2();
    fn IAppServiceTriggerDetails3();
    fn IAppServiceTriggerDetails4();
    fn IStatelessAppServiceResponse();
    fn StatelessAppServiceResponse();
    fn StatelessAppServiceResponseStatus();
}
