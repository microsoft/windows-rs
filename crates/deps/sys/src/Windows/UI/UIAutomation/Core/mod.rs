#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AutomationAnnotationTypeRegistration(i32);
#[repr(C)]
pub struct AutomationRemoteOperationOperandId(i32);
#[repr(transparent)]
pub struct AutomationRemoteOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(0i32);
    pub const MalformedBytecode: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(1i32);
    pub const InstructionLimitExceeded: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(2i32);
    pub const UnhandledException: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(3i32);
    pub const ExecutionFailure: AutomationRemoteOperationStatus = AutomationRemoteOperationStatus(4i32);
}
#[repr(transparent)]
pub struct CoreAutomationRemoteOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreAutomationRemoteOperationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationRemoteOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationConnectionBoundObjectProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationRegistrarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationExtensionProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationClientSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationClientSessionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationServerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteAutomationWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteAutomationClientSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteAutomationConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteAutomationDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteAutomationWindow(pub *mut ::core::ffi::c_void);
