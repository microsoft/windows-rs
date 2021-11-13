#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl ::core::marker::Copy for AutomationAnnotationTypeRegistration {}
impl ::core::clone::Clone for AutomationAnnotationTypeRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl ::core::marker::Copy for AutomationRemoteOperationOperandId {}
impl ::core::clone::Clone for AutomationRemoteOperationOperandId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationRemoteOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationRemoteOperationResult {}
impl ::core::clone::Clone for AutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const MalformedBytecode: Self = Self(1i32);
    pub const InstructionLimitExceeded: Self = Self(2i32);
    pub const UnhandledException: Self = Self(3i32);
    pub const ExecutionFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationRemoteOperationStatus {}
impl ::core::clone::Clone for AutomationRemoteOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreAutomationRemoteOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreAutomationRemoteOperation {}
impl ::core::clone::Clone for CoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreAutomationRemoteOperationContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreAutomationRemoteOperationContext {}
impl ::core::clone::Clone for CoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationRemoteOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationRemoteOperationResult {}
impl ::core::clone::Clone for IAutomationRemoteOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationConnectionBoundObjectProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationConnectionBoundObjectProvider {}
impl ::core::clone::Clone for ICoreAutomationConnectionBoundObjectProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationRegistrarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationRegistrarStatics {}
impl ::core::clone::Clone for ICoreAutomationRegistrarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationRemoteOperation {}
impl ::core::clone::Clone for ICoreAutomationRemoteOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationRemoteOperation2 {}
impl ::core::clone::Clone for ICoreAutomationRemoteOperation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationRemoteOperationContext {}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAutomationRemoteOperationExtensionProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAutomationRemoteOperationExtensionProvider {}
impl ::core::clone::Clone for ICoreAutomationRemoteOperationExtensionProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationClientSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationClientSession {}
impl ::core::clone::Clone for IRemoteAutomationClientSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationClientSessionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationClientSessionFactory {}
impl ::core::clone::Clone for IRemoteAutomationClientSessionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationConnectionRequestedEventArgs {}
impl ::core::clone::Clone for IRemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationDisconnectedEventArgs {}
impl ::core::clone::Clone for IRemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationServerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationServerStatics {}
impl ::core::clone::Clone for IRemoteAutomationServerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteAutomationWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteAutomationWindow {}
impl ::core::clone::Clone for IRemoteAutomationWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteAutomationClientSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteAutomationClientSession {}
impl ::core::clone::Clone for RemoteAutomationClientSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteAutomationConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteAutomationConnectionRequestedEventArgs {}
impl ::core::clone::Clone for RemoteAutomationConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteAutomationDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteAutomationDisconnectedEventArgs {}
impl ::core::clone::Clone for RemoteAutomationDisconnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteAutomationWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteAutomationWindow {}
impl ::core::clone::Clone for RemoteAutomationWindow {
    fn clone(&self) -> Self {
        *self
    }
}
