#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowBackgroundSession {}
impl ::core::clone::Clone for IPrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowBackgroundSetupRequestedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowConfiguration {}
impl ::core::clone::Clone for IPrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowConfiguration2 {}
impl ::core::clone::Clone for IPrintWorkflowConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowForegroundSession {}
impl ::core::clone::Clone for IPrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowForegroundSetupRequestedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobActivatedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobBackgroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobBackgroundSession {}
impl ::core::clone::Clone for IPrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobNotificationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobNotificationEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobStartingEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobTriggerDetails {}
impl ::core::clone::Clone for IPrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowJobUISession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowJobUISession {}
impl ::core::clone::Clone for IPrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowObjectModelSourceFileContent {}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowObjectModelSourceFileContentFactory {}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelTargetPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowObjectModelTargetPackage {}
impl ::core::clone::Clone for IPrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPdlConverter {}
impl ::core::clone::Clone for IPrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPdlDataAvailableEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPdlModificationRequestedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPdlSourceContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPdlSourceContent {}
impl ::core::clone::Clone for IPrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPdlTargetStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPdlTargetStream {}
impl ::core::clone::Clone for IPrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowPrinterJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowPrinterJob {}
impl ::core::clone::Clone for IPrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowSourceContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowSourceContent {}
impl ::core::clone::Clone for IPrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowSpoolStreamContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowSpoolStreamContent {}
impl ::core::clone::Clone for IPrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowStreamTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowStreamTarget {}
impl ::core::clone::Clone for IPrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowSubmittedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowSubmittedOperation {}
impl ::core::clone::Clone for IPrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowTarget {}
impl ::core::clone::Clone for IPrintWorkflowTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowTriggerDetails {}
impl ::core::clone::Clone for IPrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowUIActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowUIActivatedEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowUILauncher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowUILauncher {}
impl ::core::clone::Clone for IPrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowXpsDataAvailableEventArgs {}
impl ::core::clone::Clone for IPrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowBackgroundSession {}
impl ::core::clone::Clone for PrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowBackgroundSetupRequestedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowConfiguration {}
impl ::core::clone::Clone for PrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowForegroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowForegroundSession {}
impl ::core::clone::Clone for PrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowForegroundSetupRequestedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobAbortReason(pub i32);
impl PrintWorkflowJobAbortReason {
    pub const JobFailed: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintWorkflowJobAbortReason {}
impl ::core::clone::Clone for PrintWorkflowJobAbortReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobActivatedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobBackgroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobBackgroundSession {}
impl ::core::clone::Clone for PrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobNotificationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobNotificationEventArgs {}
impl ::core::clone::Clone for PrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobStartingEventArgs {}
impl ::core::clone::Clone for PrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobTriggerDetails {}
impl ::core::clone::Clone for PrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobUISession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowJobUISession {}
impl ::core::clone::Clone for PrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowObjectModelSourceFileContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowObjectModelSourceFileContent {}
impl ::core::clone::Clone for PrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowObjectModelTargetPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowObjectModelTargetPackage {}
impl ::core::clone::Clone for PrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlConversionType(pub i32);
impl PrintWorkflowPdlConversionType {
    pub const XpsToPdf: Self = Self(0i32);
    pub const XpsToPwgr: Self = Self(1i32);
    pub const XpsToPclm: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowPdlConversionType {}
impl ::core::clone::Clone for PrintWorkflowPdlConversionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPdlConverter {}
impl ::core::clone::Clone for PrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPdlDataAvailableEventArgs {}
impl ::core::clone::Clone for PrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPdlModificationRequestedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlSourceContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPdlSourceContent {}
impl ::core::clone::Clone for PrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlTargetStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPdlTargetStream {}
impl ::core::clone::Clone for PrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPrinterJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowPrinterJob {}
impl ::core::clone::Clone for PrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPrinterJobStatus(pub i32);
impl PrintWorkflowPrinterJobStatus {
    pub const Error: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const InProgress: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowPrinterJobStatus {}
impl ::core::clone::Clone for PrintWorkflowPrinterJobStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSessionStatus(pub i32);
impl PrintWorkflowSessionStatus {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Closed: Self = Self(3i32);
    pub const PdlDataAvailableForModification: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintWorkflowSessionStatus {}
impl ::core::clone::Clone for PrintWorkflowSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSourceContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowSourceContent {}
impl ::core::clone::Clone for PrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSpoolStreamContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowSpoolStreamContent {}
impl ::core::clone::Clone for PrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowStreamTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowStreamTarget {}
impl ::core::clone::Clone for PrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowSubmittedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSubmittedOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowSubmittedOperation {}
impl ::core::clone::Clone for PrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSubmittedStatus(pub i32);
impl PrintWorkflowSubmittedStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowSubmittedStatus {}
impl ::core::clone::Clone for PrintWorkflowSubmittedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowTarget {}
impl ::core::clone::Clone for PrintWorkflowTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowTriggerDetails {}
impl ::core::clone::Clone for PrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowUIActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowUIActivatedEventArgs {}
impl ::core::clone::Clone for PrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowUICompletionStatus(pub i32);
impl PrintWorkflowUICompletionStatus {
    pub const Completed: Self = Self(0i32);
    pub const LaunchFailed: Self = Self(1i32);
    pub const JobFailed: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowUICompletionStatus {}
impl ::core::clone::Clone for PrintWorkflowUICompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowUILauncher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowUILauncher {}
impl ::core::clone::Clone for PrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintWorkflowXpsDataAvailableEventArgs {}
impl ::core::clone::Clone for PrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
