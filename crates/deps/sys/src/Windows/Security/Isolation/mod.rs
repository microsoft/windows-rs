#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HostMessageReceivedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentHostStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: IsolatedWindowsEnvironmentActivator = IsolatedWindowsEnvironmentActivator(0i32);
    pub const User: IsolatedWindowsEnvironmentActivator = IsolatedWindowsEnvironmentActivator(1i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(0u32);
    pub const Text: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(1u32);
    pub const Image: IsolatedWindowsEnvironmentAllowedClipboardFormats = IsolatedWindowsEnvironmentAllowedClipboardFormats(2u32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(0u32);
    pub const Local: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(1u32);
    pub const Network: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(2u32);
    pub const SystemPrintToPdf: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(4u32);
    pub const SystemPrintToXps: IsolatedWindowsEnvironmentAvailablePrinters = IsolatedWindowsEnvironmentAvailablePrinters(8u32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(0u32);
    pub const HostToIsolatedWindowsEnvironment: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(1u32);
    pub const IsolatedWindowsEnvironmentToHost: IsolatedWindowsEnvironmentClipboardCopyPasteDirections = IsolatedWindowsEnvironmentClipboardCopyPasteDirections(2u32);
}
#[repr(C)]
pub struct IsolatedWindowsEnvironmentContract(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(0i32);
    pub const FailureByPolicy: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(1i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentCreateStatus = IsolatedWindowsEnvironmentCreateStatus(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(0i32);
    pub const FeatureNotInstalled: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(1i32);
    pub const HardwareRequirementsNotMet: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(2i32);
    pub const RebootRequired: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(3i32);
    pub const UnknownError: IsolatedWindowsEnvironmentHostError = IsolatedWindowsEnvironmentHostError(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(2i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(3i32);
    pub const TimedOut: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(4i32);
    pub const AlreadySharedWithConflictingOptions: IsolatedWindowsEnvironmentLaunchFileStatus = IsolatedWindowsEnvironmentLaunchFileStatus(5i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(0i32);
    pub const InvalidArgument: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(1i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(2i32);
    pub const InsufficientMemory: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(3i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentOwnerRegistrationStatus = IsolatedWindowsEnvironmentOwnerRegistrationStatus(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentPostMessageStatus = IsolatedWindowsEnvironmentPostMessageStatus(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(1i32);
    pub const Aborted: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(2i32);
    pub const Completed: IsolatedWindowsEnvironmentProcessState = IsolatedWindowsEnvironmentProcessState(3i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(0i32);
    pub const Processing: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(1i32);
    pub const Completed: IsolatedWindowsEnvironmentProgressState = IsolatedWindowsEnvironmentProgressState(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(2i32);
    pub const AlreadySharedWithConflictingOptions: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(3i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(4i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentShareFileStatus = IsolatedWindowsEnvironmentShareFileStatus(5i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(2i32);
    pub const FolderNotFound: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(3i32);
    pub const AccessDenied: IsolatedWindowsEnvironmentShareFolderStatus = IsolatedWindowsEnvironmentShareFolderStatus(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(0i32);
    pub const UnknownFailure: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(1i32);
    pub const EnvironmentUnavailable: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(2i32);
    pub const FileNotFound: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(3i32);
    pub const AppNotRegistered: IsolatedWindowsEnvironmentStartProcessStatus = IsolatedWindowsEnvironmentStartProcessStatus(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageReceivedCallback(pub *mut ::core::ffi::c_void);
