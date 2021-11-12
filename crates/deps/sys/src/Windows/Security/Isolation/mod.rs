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
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: Self = Self(0u32);
    pub const Local: Self = Self(1u32);
    pub const Network: Self = Self(2u32);
    pub const SystemPrintToPdf: Self = Self(4u32);
    pub const SystemPrintToXps: Self = Self(8u32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const TimedOut: Self = Self(4i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(5i32);
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
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(3i32);
    pub const FileNotFound: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageReceivedCallback(pub *mut ::core::ffi::c_void);
