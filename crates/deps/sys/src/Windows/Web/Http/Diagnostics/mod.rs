#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HttpDiagnosticProvider();
    fn HttpDiagnosticProviderRequestResponseCompletedEventArgs();
    fn HttpDiagnosticProviderRequestResponseTimestamps();
    fn HttpDiagnosticProviderRequestSentEventArgs();
    fn HttpDiagnosticProviderResponseReceivedEventArgs();
    fn HttpDiagnosticRequestInitiator();
    fn HttpDiagnosticSourceLocation();
    fn HttpDiagnosticsContract();
    fn IHttpDiagnosticProvider();
    fn IHttpDiagnosticProviderRequestResponseCompletedEventArgs();
    fn IHttpDiagnosticProviderRequestResponseTimestamps();
    fn IHttpDiagnosticProviderRequestSentEventArgs();
    fn IHttpDiagnosticProviderResponseReceivedEventArgs();
    fn IHttpDiagnosticProviderStatics();
    fn IHttpDiagnosticSourceLocation();
}
