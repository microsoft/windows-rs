#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AutomationAnnotationTypeRegistration(i32);
pub struct AutomationRemoteOperationOperandId(i32);
pub struct AutomationRemoteOperationResult(i32);
pub struct AutomationRemoteOperationStatus(i32);
pub struct CoreAutomationRegistrar(i32);
pub struct CoreAutomationRemoteOperation(i32);
pub struct CoreAutomationRemoteOperationContext(i32);
pub struct IAutomationRemoteOperationResult(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationConnectionBoundObjectProvider(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationRegistrarStatics(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationRemoteOperation(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationRemoteOperation2(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationRemoteOperationContext(pub *mut ::core::ffi::c_void);
pub struct ICoreAutomationRemoteOperationExtensionProvider(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationClientSession(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationClientSessionFactory(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationServerStatics(pub *mut ::core::ffi::c_void);
pub struct IRemoteAutomationWindow(pub *mut ::core::ffi::c_void);
pub struct RemoteAutomationClientSession(i32);
pub struct RemoteAutomationConnectionRequestedEventArgs(i32);
pub struct RemoteAutomationDisconnectedEventArgs(i32);
pub struct RemoteAutomationServer(i32);
pub struct RemoteAutomationWindow(i32);
