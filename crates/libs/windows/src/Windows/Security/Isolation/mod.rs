#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, activator: IsolatedWindowsEnvironmentActivator, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartProcessSilentlyWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProcessSilentlyWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: *mut ::core::ffi::c_void, requestoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFolderWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: *mut ::core::ffi::c_void, requestoptions: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFolderWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: *mut ::core::ffi::c_void, argumentstemplate: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFileWithUIAndTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: *mut ::core::ffi::c_void, argumentstemplate: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFileWithUIAndTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TerminateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TerminateWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, messagereceivedcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterMessageReceiver: usize,
    pub UnregisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironment2 {
    type Vtable = IIsolatedWindowsEnvironment2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d365f39_88bd_4ab4_93cf_7e2bcef337c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiverWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiverWithTelemetryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironment3 {
    type Vtable = IIsolatedWindowsEnvironment3_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb7fc7d2_d06e_4c26_8ada_dacdaaad03f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShareFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShareFileWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareFileWithTelemetryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironment4 {
    type Vtable = IIsolatedWindowsEnvironment4_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironment4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11e3701a_dd9e_4f1b_812c_4020f307f93c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentCreateResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Environment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentCreateResult2 {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentCreateResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa547dbc7_61d4_4fb8_ab5c_edefa3d388ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeCreationPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentFactory {
    type Vtable = IIsolatedWindowsEnvironmentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aca93e7_e804_454d_8466_f9897c20b0f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, telemetryparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithTelemetryAsync: usize,
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindByOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentownerid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindByOwnerId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HostPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentFile2 {
    type Vtable = IIsolatedWindowsEnvironmentFile2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentFile2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4eeb8dec_ad5d_4b0a_b754_f36c3d46d684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GuestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentHostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentHostStatics {
    type Vtable = IIsolatedWindowsEnvironmentHostStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentHostStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0e22c7_05a0_517a_b81c_6ee8790c381f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub SetAllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub ClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub SetClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT,
    pub AvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub SetAvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT,
    pub SharedHostFolderPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SharedFolderNameInEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShareHostFolderForUntrustedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharedhostfolderpath: *mut ::core::ffi::c_void, sharefoldernameinenvironment: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOptions2 {
    type Vtable = IIsolatedWindowsEnvironmentOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d7cc31_8b8f_4b9d_b22c_617103b55b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOptions3 {
    type Vtable = IIsolatedWindowsEnvironmentOptions3_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOptions3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98d5aa23_161f_4cd9_8a9c_269b30122b0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowedClipboardFormatsToEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub SetAllowedClipboardFormatsToEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub AllowedClipboardFormatsToHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub SetAllowedClipboardFormatsToHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT,
    pub CreationPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::HRESULT,
    pub SetCreationPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ShareableFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShareableFolders: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsSystem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessesRunnableAsUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessesRunnableAsUser: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivationFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivationFileExtensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dab9451_6169_55df_8f51_790e99d7277d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10951754_204b_5ec9_9de3_df792d074a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: *mut ::core::ffi::c_void, ownerregistrationdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentProcess(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentProcess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa858c3ef_8172_4f10_af93_cbe60af88d09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub WaitForExit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForExitWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForExitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForExitAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9190ed8_0fd0_4946_bb88_117a60737b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentUserInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9c75ae_69ba_4001_96fc_19a02703b340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnvironmentUserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnvironmentUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryWaitForSignInAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryWaitForSignInAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsEnvironmentUserInfo2 {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsEnvironmentUserInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bdd5dd_91d7_481e_94f2_2a5a6bdf9383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryWaitForSignInWithProgressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryWaitForSignInWithProgressAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsHostMessengerStatics {
    type Vtable = IIsolatedWindowsHostMessengerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06e444bb_53c0_4889_8fa3_53592e37cf21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PostMessageToReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PostMessageToReceiver: usize,
    pub GetFileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsolatedWindowsHostMessengerStatics2 {
    type Vtable = IIsolatedWindowsHostMessengerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedWindowsHostMessengerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ef9ebc_0444_42ad_832d_1b89c089d1ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, hostmessagereceivedcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterHostMessageReceiver: usize,
    pub UnregisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironment(::windows::core::IUnknown);
impl IsolatedWindowsEnvironment {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProcessSilentlyAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostexepath), ::core::mem::transmute_copy(arguments), activator, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartProcessSilentlyWithTelemetryAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProcessSilentlyWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostexepath), ::core::mem::transmute_copy(arguments), activator, ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShareFolderAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostfolder), ::core::mem::transmute_copy(requestoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFolderWithTelemetryAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &IsolatedWindowsEnvironmentShareFolderRequestOptions, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShareFolderWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostfolder), ::core::mem::transmute_copy(requestoptions), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFileWithUIAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appexepath), ::core::mem::transmute_copy(argumentstemplate), ::core::mem::transmute_copy(filepath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFileWithUIAndTelemetryAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFileWithUIAndTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appexepath), ::core::mem::transmute_copy(argumentstemplate), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TerminateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TerminateAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TerminateWithTelemetryAsync(&self, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TerminateWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterMessageReceiver(&self, receiverid: ::windows::core::GUID, messagereceivedcallback: &MessageReceivedCallback) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RegisterMessageReceiver)(::windows::core::Vtable::as_raw(this), receiverid, ::core::mem::transmute_copy(messagereceivedcallback)).ok() }
    }
    pub fn UnregisterMessageReceiver(&self, receiverid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UnregisterMessageReceiver)(::windows::core::Vtable::as_raw(this), receiverid).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiverAsync<P0, E0>(&self, receiverid: ::windows::core::GUID, message: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostMessageToReceiverAsync)(::windows::core::Vtable::as_raw(this), receiverid, message.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiverWithTelemetryAsync<P0, E0>(&self, receiverid: ::windows::core::GUID, message: P0, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostMessageToReceiverWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), receiverid, message.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUserInfo(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUserInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFileAsync(&self, filepath: &::windows::core::HSTRING, options: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShareFileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareFileWithTelemetryAsync(&self, filepath: &::windows::core::HSTRING, options: &IsolatedWindowsEnvironmentShareFileRequestOptions, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShareFileWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChangePriority(&self, priority: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironment4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ChangePriority)(::windows::core::Vtable::as_raw(this), priority).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(options: &IsolatedWindowsEnvironmentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithTelemetryAsync(options: &IsolatedWindowsEnvironmentOptions, telemetryparameters: &IsolatedWindowsEnvironmentTelemetryParameters) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), ::core::mem::transmute_copy(telemetryparameters), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetById(environmentid: &::windows::core::HSTRING) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetById)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(environmentid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindByOwnerId(environmentownerid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindByOwnerId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(environmentownerid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironment {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironment;{41d24597-c328-4467-b37f-4dfc6f60b6bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironment {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironment {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironment {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentCreateResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Environment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Environment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChangeCreationPriority(&self, priority: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentCreateResult2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ChangeCreationPriority)(::windows::core::Vtable::as_raw(this), priority).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult;{ef9a5e58-dcd7-45c2-9c85-ab642a715e8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentCreateResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentCreateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentCreateResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentCreateResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentCreateResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentFile {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HostPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HostPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GuestPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GuestPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentFile {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentFile;{4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentFile {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentFile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentFile, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentFile {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentFile {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentHost;
impl IsolatedWindowsEnvironmentHost {
    pub fn IsReady() -> ::windows::core::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReady)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostErrors() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HostErrors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentLaunchFileResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).File)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentLaunchFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentLaunchFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult;{685d4176-f6e0-4569-b1aa-215c0ff5b257})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentLaunchFileResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentLaunchFileResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentLaunchFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn EnvironmentOwnerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnvironmentOwnerId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEnvironmentOwnerId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnvironmentOwnerId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowedClipboardFormats(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedClipboardFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowedClipboardFormats)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ClipboardCopyPasteDirections(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClipboardCopyPasteDirections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetClipboardCopyPasteDirections)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AvailablePrinters(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AvailablePrinters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAvailablePrinters)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SharedHostFolderPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharedHostFolderPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SharedFolderNameInEnvironment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharedFolderNameInEnvironment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShareHostFolderForUntrustedItems(&self, sharedhostfolderpath: &::windows::core::HSTRING, sharefoldernameinenvironment: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShareHostFolderForUntrustedItems)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sharedhostfolderpath), ::core::mem::transmute_copy(sharefoldernameinenvironment)).ok() }
    }
    pub fn PersistUserProfile(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PersistUserProfile)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPersistUserProfile(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPersistUserProfile)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowGraphicsHardwareAcceleration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowGraphicsHardwareAcceleration)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowCameraAndMicrophoneAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowCameraAndMicrophoneAccess)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WindowAnnotationOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WindowAnnotationOverride)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetWindowAnnotationOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetWindowAnnotationOverride)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowedClipboardFormatsToEnvironment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedClipboardFormatsToEnvironment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowedClipboardFormatsToEnvironment(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowedClipboardFormatsToEnvironment)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AllowedClipboardFormatsToHost(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedClipboardFormatsToHost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowedClipboardFormatsToHost(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowedClipboardFormatsToHost)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CreationPriority(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreationPriority> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreationPriority)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCreationPriority(&self, value: IsolatedWindowsEnvironmentCreationPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCreationPriority)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions;{b71d98f7-61f0-4008-b207-0bf9eb2d76f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOptions {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsEnvironmentOwnerRegistration;
impl IsolatedWindowsEnvironmentOwnerRegistration {
    pub fn Register(ownername: &::windows::core::HSTRING, ownerregistrationdata: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Register)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(ownername), ::core::mem::transmute_copy(ownerregistrationdata), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Unregister(ownername: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Unregister)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(ownername)).ok() })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShareableFolders(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShareableFolders)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsSystem(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessesRunnableAsSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessesRunnableAsUser(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessesRunnableAsUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivationFileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivationFileExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationData {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData;{f888ec22-e8cf-56c0-b1df-90af4ad80e84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult;{6dab9451-6169-55df-8f51-790e99d7277d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentOwnerRegistrationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentPostMessageResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentPostMessageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentPostMessageResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult;{0dfa28fa-2ef0-4d8f-b341-3171b2df93b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentPostMessageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentPostMessageResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentPostMessageResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentProcess {
    pub fn State(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WaitForExit(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WaitForExit)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WaitForExitWithTimeout)(::windows::core::Vtable::as_raw(this), timeoutmilliseconds).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitForExitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WaitForExitAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentProcess {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess;{a858c3ef-8172-4f10-af93-cbe60af88d09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentProcess {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentProcess as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentProcess, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentProcess {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentProcess {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowWrite)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions;{c9190ed8-0fd0-4946-bb88-117a60737b61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).File)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult;{aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFileResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFileResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentShareFileResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowWrite)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions;{c405eb7d-7053-4f6a-9b87-746846ed19b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult;{556ba72e-ca9d-4211-b143-1cedc86eb2fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentShareFolderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentShareFolderResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentStartProcessResult {
    pub fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Process(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Process)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentStartProcessResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentStartProcessResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult;{8fa1dc2f-57da-4bb5-9c06-fa072d2032e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentStartProcessResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentStartProcessResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentStartProcessResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CorrelationId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCorrelationId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCorrelationId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentTelemetryParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentTelemetryParameters {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentTelemetryParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters;{ebdb3cab-7a3a-4524-a0f4-f96e284d33cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentTelemetryParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentTelemetryParameters {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(::windows::core::IUnknown);
impl IsolatedWindowsEnvironmentUserInfo {
    pub fn EnvironmentUserSid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnvironmentUserSid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EnvironmentUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnvironmentUserName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryWaitForSignInAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryWaitForSignInAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryWaitForSignInWithProgressAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, IsolatedWindowsEnvironmentSignInProgress>> {
        let this = &::windows::core::Interface::cast::<IIsolatedWindowsEnvironmentUserInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryWaitForSignInWithProgressAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentUserInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentUserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentUserInfo {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentUserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentUserInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo;{8a9c75ae-69ba-4001-96fc-19a02703b340})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IsolatedWindowsEnvironmentUserInfo {
    const IID: ::windows::core::GUID = <IIsolatedWindowsEnvironmentUserInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
::windows::core::interface_hierarchy!(IsolatedWindowsEnvironmentUserInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentUserInfo {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentUserInfo {}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
pub struct IsolatedWindowsHostMessenger;
impl IsolatedWindowsHostMessenger {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PostMessageToReceiver<P0, E0>(receiverid: ::windows::core::GUID, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).PostMessageToReceiver)(::windows::core::Vtable::as_raw(this), receiverid, message.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn GetFileId(filepath: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFileId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterHostMessageReceiver(receiverid: ::windows::core::GUID, hostmessagereceivedcallback: &HostMessageReceivedCallback) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).RegisterHostMessageReceiver)(::windows::core::Vtable::as_raw(this), receiverid, ::core::mem::transmute_copy(hostmessagereceivedcallback)).ok() })
    }
    pub fn UnregisterHostMessageReceiver(receiverid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).UnregisterHostMessageReceiver)(::windows::core::Vtable::as_raw(this), receiverid).ok() })
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentActivator {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentActivator {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
    pub const Rtf: Self = Self(4u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAllowedClipboardFormats {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAllowedClipboardFormats").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentAvailablePrinters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentAvailablePrinters {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAvailablePrinters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAvailablePrinters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentClipboardCopyPasteDirections").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsolatedWindowsEnvironmentCreationPriority(pub i32);
impl IsolatedWindowsEnvironmentCreationPriority {
    pub const Low: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreationPriority {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreationPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreationPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreationPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentHostError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentHostError {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentHostError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentHostError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentLaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentPostMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentPostMessageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentProcessState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProcessState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcessState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcessState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Creating: Self = Self(3i32);
    pub const Retrying: Self = Self(4i32);
    pub const Starting: Self = Self(5i32);
    pub const Finalizing: Self = Self(6i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProgressState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentProgressState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProgressState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentShareFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsolatedWindowsEnvironmentSignInProgress(pub i32);
impl IsolatedWindowsEnvironmentSignInProgress {
    pub const Connecting: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Authenticating: Self = Self(2i32);
    pub const SettingUpAccount: Self = Self(3i32);
    pub const Finalizing: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentSignInProgress {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentSignInProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentSignInProgress {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentSignInProgress {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentSignInProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentSignInProgress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentSignInProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for IsolatedWindowsEnvironmentStartProcessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentStartProcessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Security_Isolation\"`*"]
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
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedWindowsEnvironmentCreateProgress").field("State", &self.State).field("PercentComplete", &self.PercentComplete).finish()
    }
}
unsafe impl ::windows::core::Abi for IsolatedWindowsEnvironmentCreateProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.PercentComplete == other.PercentComplete
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct HostMessageReceivedCallback(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = HostMessageReceivedCallbackBox::<F> { vtable: &HostMessageReceivedCallbackBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, E0>(&self, receiverid: ::windows::core::GUID, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), receiverid, message.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct HostMessageReceivedCallbackBox<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const HostMessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> HostMessageReceivedCallbackBox<F> {
    const VTABLE: HostMessageReceivedCallback_Vtbl = HostMessageReceivedCallback_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<HostMessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for HostMessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for HostMessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for HostMessageReceivedCallback {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for HostMessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostMessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for HostMessageReceivedCallback {
    type Vtable = HostMessageReceivedCallback_Vtbl;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for HostMessageReceivedCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf26ffa_8ce1_4cc1_b278_322d31a5e4a3);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faf26ffa-8ce1-4cc1-b278-322d31a5e4a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct HostMessageReceivedCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Security_Isolation\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct MessageReceivedCallback(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedCallbackBox::<F> { vtable: &MessageReceivedCallbackBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, E0>(&self, receiverid: ::windows::core::GUID, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), receiverid, message.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct MessageReceivedCallbackBox<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::windows::core::GUID, &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageReceivedCallbackBox<F> {
    const VTABLE: MessageReceivedCallback_Vtbl = MessageReceivedCallback_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MessageReceivedCallback as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for MessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for MessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for MessageReceivedCallback {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for MessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for MessageReceivedCallback {
    type Vtable = MessageReceivedCallback_Vtbl;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MessageReceivedCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b4c8ff_1d9d_4995_9fea_4d15257c0757);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f5b4c8ff-1d9d-4995-9fea-4d15257c0757}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
