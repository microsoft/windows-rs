#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HttpBaseProtocolFilter();
    fn HttpCacheControl();
    fn HttpCacheReadBehavior();
    fn HttpCacheWriteBehavior();
    fn HttpCookieUsageBehavior();
    fn HttpServerCustomValidationRequestedEventArgs();
    fn IHttpBaseProtocolFilter();
    fn IHttpBaseProtocolFilter2();
    fn IHttpBaseProtocolFilter3();
    fn IHttpBaseProtocolFilter4();
    fn IHttpBaseProtocolFilter5();
    fn IHttpBaseProtocolFilterStatics();
    fn IHttpCacheControl();
    fn IHttpFilter();
    fn IHttpServerCustomValidationRequestedEventArgs();
}
