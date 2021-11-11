#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AutomationAnnotationTypeRegistration();
    fn AutomationRemoteOperationOperandId();
    fn AutomationRemoteOperationResult();
    fn AutomationRemoteOperationStatus();
    fn CoreAutomationRegistrar();
    fn CoreAutomationRemoteOperation();
    fn CoreAutomationRemoteOperationContext();
    fn IAutomationRemoteOperationResult();
    fn ICoreAutomationConnectionBoundObjectProvider();
    fn ICoreAutomationRegistrarStatics();
    fn ICoreAutomationRemoteOperation();
    fn ICoreAutomationRemoteOperation2();
    fn ICoreAutomationRemoteOperationContext();
    fn ICoreAutomationRemoteOperationExtensionProvider();
    fn IRemoteAutomationClientSession();
    fn IRemoteAutomationClientSessionFactory();
    fn IRemoteAutomationConnectionRequestedEventArgs();
    fn IRemoteAutomationDisconnectedEventArgs();
    fn IRemoteAutomationServerStatics();
    fn IRemoteAutomationWindow();
    fn RemoteAutomationClientSession();
    fn RemoteAutomationConnectionRequestedEventArgs();
    fn RemoteAutomationDisconnectedEventArgs();
    fn RemoteAutomationServer();
    fn RemoteAutomationWindow();
}
