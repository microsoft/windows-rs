#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartProcessSilentlyAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>;
    fn StartProcessSilentlyWithTelemetryAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>;
    fn ShareFolderAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &::core::option::Option<IsolatedWindowsEnvironmentShareFolderRequestOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>;
    fn ShareFolderWithTelemetryAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &::core::option::Option<IsolatedWindowsEnvironmentShareFolderRequestOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>;
    fn LaunchFileWithUIAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>;
    fn LaunchFileWithUIAndTelemetryAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>;
    fn TerminateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TerminateWithTelemetryAsync(&self, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RegisterMessageReceiver(&self, receiverid: &::windows::core::GUID, messagereceivedcallback: &::core::option::Option<MessageReceivedCallback>) -> ::windows::core::Result<()>;
    fn UnregisterMessageReceiver(&self, receiverid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironment2Impl: Sized {
    fn PostMessageToReceiverAsync(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>;
    fn PostMessageToReceiverWithTelemetryAsync(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironment3Impl: Sized {
    fn GetUserInfo(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentUserInfo>;
    fn ShareFileAsync(&self, filepath: &::windows::core::HSTRING, options: &::core::option::Option<IsolatedWindowsEnvironmentShareFileRequestOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>;
    fn ShareFileWithTelemetryAsync(&self, filepath: &::windows::core::HSTRING, options: &::core::option::Option<IsolatedWindowsEnvironmentShareFileRequestOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentCreateResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreateStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Environment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFactoryImpl: Sized {
    fn CreateAsync(&self, options: &::core::option::Option<IsolatedWindowsEnvironmentOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>;
    fn CreateWithTelemetryAsync(&self, options: &::core::option::Option<IsolatedWindowsEnvironmentOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>;
    fn GetById(&self, environmentid: &::windows::core::HSTRING) -> ::windows::core::Result<IsolatedWindowsEnvironment>;
    fn FindByOwnerId(&self, environmentownerid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFileImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HostPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFile2Impl: Sized {
    fn GuestPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentHostStaticsImpl: Sized {
    fn IsReady(&self) -> ::windows::core::Result<bool>;
    fn HostErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentLaunchFileResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentLaunchFileStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOptionsImpl: Sized {
    fn EnvironmentOwnerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnvironmentOwnerId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowedClipboardFormats(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats>;
    fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()>;
    fn ClipboardCopyPasteDirections(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>;
    fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::Result<()>;
    fn AvailablePrinters(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAvailablePrinters>;
    fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::Result<()>;
    fn SharedHostFolderPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedFolderNameInEnvironment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShareHostFolderForUntrustedItems(&self, sharedhostfolderpath: &::windows::core::HSTRING, sharefoldernameinenvironment: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PersistUserProfile(&self) -> ::windows::core::Result<bool>;
    fn SetPersistUserProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::core::Result<bool>;
    fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOptions2Impl: Sized {
    fn WindowAnnotationOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetWindowAnnotationOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl: Sized {
    fn ShareableFolders(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProcessesRunnableAsSystem(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProcessesRunnableAsUser(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ActivationFileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationStaticsImpl: Sized {
    fn Register(&self, ownername: &::windows::core::HSTRING, ownerregistrationdata: &::core::option::Option<IsolatedWindowsEnvironmentOwnerRegistrationData>) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult>;
    fn Unregister(&self, ownername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentPostMessageResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentPostMessageStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentProcessImpl: Sized {
    fn State(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcessState>;
    fn ExitCode(&self) -> ::windows::core::Result<u32>;
    fn WaitForExit(&self) -> ::windows::core::Result<()>;
    fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::core::Result<()>;
    fn WaitForExitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFileRequestOptionsImpl: Sized {
    fn AllowWrite(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFileResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFileStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFolderRequestOptionsImpl: Sized {
    fn AllowWrite(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFolderResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFolderStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentStartProcessResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentStartProcessStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Process(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcess>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentTelemetryParametersImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetCorrelationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentUserInfoImpl: Sized {
    fn EnvironmentUserSid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnvironmentUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryWaitForSignInAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsHostMessengerStaticsImpl: Sized {
    fn PostMessageToReceiver(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()>;
    fn GetFileId(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsHostMessengerStatics2Impl: Sized {
    fn RegisterHostMessageReceiver(&self, receiverid: &::windows::core::GUID, hostmessagereceivedcallback: &::core::option::Option<HostMessageReceivedCallback>) -> ::windows::core::Result<()>;
    fn UnregisterHostMessageReceiver(&self, receiverid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
