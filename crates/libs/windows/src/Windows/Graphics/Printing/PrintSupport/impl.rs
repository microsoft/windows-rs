#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportExtensionSessionImpl: Sized {
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn PrintTicketValidationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTicketValidationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrintDeviceCapabilitiesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintDeviceCapabilitiesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportExtensionTriggerDetailsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintSupportExtensionSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl: Sized {
    fn GetCurrentPrintDeviceCapabilities(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument>;
    fn UpdatePrintDeviceCapabilities(&self, updatedpdc: &::core::option::Option<super::super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportPrintTicketValidationRequestedEventArgsImpl: Sized {
    fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportSessionInfoImpl: Sized {
    fn SourceAppInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportSettingsActivatedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportSettingsUISessionImpl: Sized {
    fn SessionPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind>;
    fn UpdatePrintTicket(&self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<()>;
    fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo>;
}
