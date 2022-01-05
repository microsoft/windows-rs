#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowImpl: Sized {
    fn DeviceID(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPrintModelPackage(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn IsPrintReady(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrintReady(&self, value: bool) -> ::windows::core::Result<()>;
    fn PrintRequested(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintRequested(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflow2Impl: Sized {
    fn PrinterChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrinterChanged(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowPrintRequestedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<Print3DWorkflowStatus>;
    fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> ::windows::core::Result<()>;
    fn SetSource(&self, source: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetSourceChanged(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowPrinterChangedEventArgsImpl: Sized {
    fn NewDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintExtensionContextStaticImpl: Sized {
    fn FromDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintNotificationEventDetailsImpl: Sized {
    fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EventData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEventData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationImpl: Sized {
    fn PrinterExtensionContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SaveRequested(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSaveRequested(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Save(&self, printerextensioncontext: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequestedDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequest>;
}
