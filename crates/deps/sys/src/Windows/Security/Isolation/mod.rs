#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct IsolatedWindowsEnvironmentActivator(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentContract(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentHostError(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentProcessState(i32);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentProgressState(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(i32);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageReceivedCallback(pub *mut ::core::ffi::c_void);
