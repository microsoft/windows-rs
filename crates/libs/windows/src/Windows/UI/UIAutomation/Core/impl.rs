#[cfg(feature = "implement_exclusive")]
pub trait IAutomationRemoteOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AutomationRemoteOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorLocation(&self) -> ::windows::core::Result<i32>;
    fn HasOperand(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<bool>;
    fn GetOperand(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait ICoreAutomationConnectionBoundObjectProviderImpl: Sized {
    fn IsComThreadingRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRegistrarStaticsImpl: Sized {
    fn RegisterAnnotationType(&self, guid: &::windows::core::GUID) -> ::windows::core::Result<AutomationAnnotationTypeRegistration>;
    fn UnregisterAnnotationType(&self, registration: &AutomationAnnotationTypeRegistration) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperationImpl: Sized {
    fn IsOpcodeSupported(&self, opcode: u32) -> ::windows::core::Result<bool>;
    fn ImportElement(&self, operandid: &AutomationRemoteOperationOperandId, element: &::core::option::Option<super::AutomationElement>) -> ::windows::core::Result<()>;
    fn ImportTextRange(&self, operandid: &AutomationRemoteOperationOperandId, textrange: &::core::option::Option<super::AutomationTextRange>) -> ::windows::core::Result<()>;
    fn AddToResults(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<()>;
    fn Execute(&self, bytecodebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<AutomationRemoteOperationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperation2Impl: Sized {
    fn ImportConnectionBoundObject(&self, operandid: &AutomationRemoteOperationOperandId, connectionboundobject: &::core::option::Option<super::AutomationConnectionBoundObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperationContextImpl: Sized {
    fn GetOperand(&self, id: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetOperand(&self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetOperand2(&self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>, operandinterfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
pub trait ICoreAutomationRemoteOperationExtensionProviderImpl: Sized {
    fn CallExtension(&self, extensionid: &::windows::core::GUID, context: &::core::option::Option<CoreAutomationRemoteOperationContext>, operandids: &[<AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn IsExtensionSupported(&self, extensionid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationClientSessionImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn CreateWindowAsync(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ConnectionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationClientSessionFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteAutomationClientSession>;
    fn CreateInstance2(&self, name: &::windows::core::HSTRING, sessionid: &::windows::core::GUID) -> ::windows::core::Result<RemoteAutomationClientSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationConnectionRequestedEventArgsImpl: Sized {
    fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteProcessId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationDisconnectedEventArgsImpl: Sized {
    fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationServerStaticsImpl: Sized {
    fn ReportSession(&self, sessionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationWindowImpl: Sized {
    fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
