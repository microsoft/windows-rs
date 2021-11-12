#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobBackgroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobNotificationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowJobUISession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelTargetPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPdlSourceContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPdlTargetStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowPrinterJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowSourceContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowSpoolStreamContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowStreamTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowUIActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowUILauncher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowForegroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobAbortReason(pub i32);
impl PrintWorkflowJobAbortReason {
    pub const JobFailed: PrintWorkflowJobAbortReason = PrintWorkflowJobAbortReason(0i32);
    pub const UserCanceled: PrintWorkflowJobAbortReason = PrintWorkflowJobAbortReason(1i32);
}
#[repr(transparent)]
pub struct PrintWorkflowJobActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobBackgroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobNotificationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowJobUISession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowObjectModelSourceFileContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowObjectModelTargetPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPdlConversionType(pub i32);
impl PrintWorkflowPdlConversionType {
    pub const XpsToPdf: PrintWorkflowPdlConversionType = PrintWorkflowPdlConversionType(0i32);
    pub const XpsToPwgr: PrintWorkflowPdlConversionType = PrintWorkflowPdlConversionType(1i32);
    pub const XpsToPclm: PrintWorkflowPdlConversionType = PrintWorkflowPdlConversionType(2i32);
}
#[repr(transparent)]
pub struct PrintWorkflowPdlConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPdlSourceContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPdlTargetStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPrinterJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowPrinterJobStatus(pub i32);
impl PrintWorkflowPrinterJobStatus {
    pub const Error: PrintWorkflowPrinterJobStatus = PrintWorkflowPrinterJobStatus(0i32);
    pub const Aborted: PrintWorkflowPrinterJobStatus = PrintWorkflowPrinterJobStatus(1i32);
    pub const InProgress: PrintWorkflowPrinterJobStatus = PrintWorkflowPrinterJobStatus(2i32);
    pub const Completed: PrintWorkflowPrinterJobStatus = PrintWorkflowPrinterJobStatus(3i32);
}
#[repr(transparent)]
pub struct PrintWorkflowSessionStatus(pub i32);
impl PrintWorkflowSessionStatus {
    pub const Started: PrintWorkflowSessionStatus = PrintWorkflowSessionStatus(0i32);
    pub const Completed: PrintWorkflowSessionStatus = PrintWorkflowSessionStatus(1i32);
    pub const Aborted: PrintWorkflowSessionStatus = PrintWorkflowSessionStatus(2i32);
    pub const Closed: PrintWorkflowSessionStatus = PrintWorkflowSessionStatus(3i32);
    pub const PdlDataAvailableForModification: PrintWorkflowSessionStatus = PrintWorkflowSessionStatus(4i32);
}
#[repr(transparent)]
pub struct PrintWorkflowSourceContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowSpoolStreamContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowStreamTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowSubmittedOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowSubmittedStatus(pub i32);
impl PrintWorkflowSubmittedStatus {
    pub const Succeeded: PrintWorkflowSubmittedStatus = PrintWorkflowSubmittedStatus(0i32);
    pub const Canceled: PrintWorkflowSubmittedStatus = PrintWorkflowSubmittedStatus(1i32);
    pub const Failed: PrintWorkflowSubmittedStatus = PrintWorkflowSubmittedStatus(2i32);
}
#[repr(transparent)]
pub struct PrintWorkflowTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowUIActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowUICompletionStatus(pub i32);
impl PrintWorkflowUICompletionStatus {
    pub const Completed: PrintWorkflowUICompletionStatus = PrintWorkflowUICompletionStatus(0i32);
    pub const LaunchFailed: PrintWorkflowUICompletionStatus = PrintWorkflowUICompletionStatus(1i32);
    pub const JobFailed: PrintWorkflowUICompletionStatus = PrintWorkflowUICompletionStatus(2i32);
    pub const UserCanceled: PrintWorkflowUICompletionStatus = PrintWorkflowUICompletionStatus(3i32);
}
#[repr(transparent)]
pub struct PrintWorkflowUILauncher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
