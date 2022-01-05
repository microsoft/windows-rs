pub trait IAccountsSettingsPaneInteropImpl: Sized {
    fn GetForWindow();
    fn ShowManageAccountsForWindowAsync();
    fn ShowAddAccountForWindowAsync();
}
pub trait IActivationFactoryImpl: Sized {
    fn ActivateInstance();
}
pub trait IAgileReferenceImpl: Sized {
    fn Resolve();
}
pub trait IApartmentShutdownImpl: Sized {
    fn OnUninitialize();
}
pub trait IAppServiceConnectionExtendedExecutionImpl: Sized {
    fn OpenForExtendedExecutionAsync();
}
pub trait IBufferByteAccessImpl: Sized {
    fn Buffer();
}
pub trait ICastingControllerImpl: Sized {
    fn Initialize();
    fn Connect();
    fn Disconnect();
    fn Advise();
    fn UnAdvise();
}
pub trait ICastingEventHandlerImpl: Sized {
    fn OnStateChanged();
    fn OnError();
}
pub trait ICastingSourceInfoImpl: Sized {
    fn GetController();
    fn GetProperties();
}
pub trait ICorrelationVectorInformationImpl: Sized {
    fn LastCorrelationVectorForThread();
    fn NextCorrelationVectorForThread();
    fn SetNextCorrelationVectorForThread();
}
pub trait ICorrelationVectorSourceImpl: Sized {
    fn CorrelationVector();
}
pub trait IDragDropManagerInteropImpl: Sized {
    fn GetForWindow();
}
pub trait IHolographicSpaceInteropImpl: Sized {
    fn CreateForWindow();
}
pub trait IInputPaneInteropImpl: Sized {
    fn GetForWindow();
}
pub trait ILanguageExceptionErrorInfoImpl: Sized {
    fn GetLanguageException();
}
pub trait ILanguageExceptionErrorInfo2Impl: Sized + ILanguageExceptionErrorInfoImpl {
    fn GetPreviousLanguageExceptionErrorInfo();
    fn CapturePropagationContext();
    fn GetPropagationContextHead();
}
pub trait ILanguageExceptionStackBackTraceImpl: Sized {
    fn GetStackBackTrace();
}
pub trait ILanguageExceptionTransformImpl: Sized {
    fn GetTransformedRestrictedErrorInfo();
}
pub trait IMemoryBufferByteAccessImpl: Sized {
    fn GetBuffer();
}
pub trait IMessageDispatcherImpl: Sized {
    fn PumpMessages();
}
pub trait IPlayToManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPlayToUIForWindow();
}
pub trait IRestrictedErrorInfoImpl: Sized {
    fn GetErrorDetails();
    fn GetReference();
}
pub trait IRoMetaDataLocatorImpl: Sized {
    fn Locate();
}
pub trait IRoSimpleMetaDataBuilderImpl: Sized {
    fn SetWinRtInterface();
    fn SetDelegate();
    fn SetInterfaceGroupSimpleDefault();
    fn SetInterfaceGroupParameterizedDefault();
    fn SetRuntimeClassSimpleDefault();
    fn SetRuntimeClassParameterizedDefault();
    fn SetStruct();
    fn SetEnum();
    fn SetParameterizedInterface();
    fn SetParameterizedDelegate();
}
pub trait IShareWindowCommandEventArgsInteropImpl: Sized {
    fn GetWindow();
}
pub trait IShareWindowCommandSourceInteropImpl: Sized {
    fn GetForWindow();
}
pub trait ISpatialInteractionManagerInteropImpl: Sized {
    fn GetForWindow();
}
pub trait ISystemMediaTransportControlsInteropImpl: Sized {
    fn GetForWindow();
}
pub trait IUIViewSettingsInteropImpl: Sized {
    fn GetForWindow();
}
pub trait IUserActivityInteropImpl: Sized {
    fn CreateSessionForWindow();
}
pub trait IUserActivityRequestManagerInteropImpl: Sized {
    fn GetForWindow();
}
pub trait IUserActivitySourceHostInteropImpl: Sized {
    fn SetActivitySourceHost();
}
pub trait IUserConsentVerifierInteropImpl: Sized {
    fn RequestVerificationForWindowAsync();
}
pub trait IWeakReferenceImpl: Sized {
    fn Resolve();
}
pub trait IWeakReferenceSourceImpl: Sized {
    fn GetWeakReference();
}
pub trait IWebAuthenticationCoreManagerInteropImpl: Sized {
    fn RequestTokenForWindowAsync();
    fn RequestTokenWithWebAccountForWindowAsync();
}
