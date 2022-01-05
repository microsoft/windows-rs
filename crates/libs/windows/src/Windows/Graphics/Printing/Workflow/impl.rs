#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowBackgroundSessionImpl: Sized {
    fn SetupRequested(&self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitted(&self, submittedeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowBackgroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn SetRequiresUI(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfigurationImpl: Sized {
    fn SourceAppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JobTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfiguration2Impl: Sized {
    fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowForegroundSessionImpl: Sized {
    fn SetupRequested(&self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XpsDataAvailable(&self, xpsdataavailableeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXpsDataAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowForegroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobActivatedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintWorkflowJobUISession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobBackgroundSessionImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn JobStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PdlModificationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlModificationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobNotificationEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobStartingEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn SetSkipSystemRendering(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobTriggerDetailsImpl: Sized {
    fn PrintWorkflowJobSession(&self) -> ::windows::core::Result<PrintWorkflowJobBackgroundSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobUISessionImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn PdlDataAvailable(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlDataAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn JobNotification(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobNotification(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelSourceFileContentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelSourceFileContentFactoryImpl: Sized {
    fn CreateInstance(&self, xpsstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelTargetPackageImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlConverterImpl: Sized {
    fn ConvertPdlAsync(&self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlDataAvailableEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlModificationRequestedEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn UILauncher(&self) -> ::windows::core::Result<PrintWorkflowUILauncher>;
    fn CreateJobOnPrinter(&self, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributes(&self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributesBuffer(&self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows::core::Result<PrintWorkflowPdlConverter>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlSourceContentImpl: Sized {
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn GetContentFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlTargetStreamImpl: Sized {
    fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
    fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPrinterJobImpl: Sized {
    fn JobId(&self) -> ::windows::core::Result<i32>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn GetJobStatus(&self) -> ::windows::core::Result<PrintWorkflowPrinterJobStatus>;
    fn GetJobPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn GetJobAttributesAsBuffer(&self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn GetJobAttributes(&self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>;
    fn SetJobAttributesFromBuffer(&self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
    fn SetJobAttributes(&self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSourceContentImpl: Sized {
    fn GetJobPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn GetSourceSpoolDataAsStreamContent(&self) -> ::windows::core::Result<PrintWorkflowSpoolStreamContent>;
    fn GetSourceSpoolDataAsXpsObjectModel(&self) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSpoolStreamContentImpl: Sized {
    fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowStreamTargetImpl: Sized {
    fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSubmittedEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetTarget(&self, jobprintticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<PrintWorkflowTarget>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSubmittedOperationImpl: Sized {
    fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn XpsContent(&self) -> ::windows::core::Result<PrintWorkflowSourceContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTargetImpl: Sized {
    fn TargetAsStream(&self) -> ::windows::core::Result<PrintWorkflowStreamTarget>;
    fn TargetAsXpsObjectModelPackage(&self) -> ::windows::core::Result<PrintWorkflowObjectModelTargetPackage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTriggerDetailsImpl: Sized {
    fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowBackgroundSession>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowUIActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IActivatedEventArgsWithUserImpl {
    fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowForegroundSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowUILauncherImpl: Sized {
    fn IsUILaunchEnabled(&self) -> ::windows::core::Result<bool>;
    fn LaunchAndCompleteUIAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowXpsDataAvailableEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
