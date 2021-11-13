#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HostMessageReceivedCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HostMessageReceivedCallback {}
impl ::core::clone::Clone for HostMessageReceivedCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironment {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironment2 {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironment3 {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironment3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentCreateResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentCreateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentFactory {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentFile {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentFile2 {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentFile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentHostStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentHostStatics {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentHostStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentLaunchFileResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentLaunchFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentOptions {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentOptions2 {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentOwnerRegistrationData {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentOwnerRegistrationResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentPostMessageResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentPostMessageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentProcess {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentProcess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentShareFileRequestOptions {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentShareFileResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentShareFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentShareFolderRequestOptions {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentShareFolderResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentShareFolderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentStartProcessResult {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentStartProcessResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentTelemetryParameters {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsEnvironmentUserInfo {}
impl ::core::clone::Clone for IIsolatedWindowsEnvironmentUserInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsHostMessengerStatics {}
impl ::core::clone::Clone for IIsolatedWindowsHostMessengerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedWindowsHostMessengerStatics2 {}
impl ::core::clone::Clone for IIsolatedWindowsHostMessengerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironment {}
impl ::core::clone::Clone for IsolatedWindowsEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentActivator {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentActivator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAllowedClipboardFormats {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAvailablePrinters {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAvailablePrinters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentFile {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentHostError {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentHostError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentLaunchFileResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IsolatedWindowsEnvironmentLaunchFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOptions {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationData {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentPostMessageResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentPostMessageStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProcess {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProcessState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcessState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProgressState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileRequestOptions {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentStartProcessResult {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentStartProcessStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentTelemetryParameters {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsolatedWindowsEnvironmentUserInfo {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentUserInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageReceivedCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageReceivedCallback {}
impl ::core::clone::Clone for MessageReceivedCallback {
    fn clone(&self) -> Self {
        *self
    }
}
