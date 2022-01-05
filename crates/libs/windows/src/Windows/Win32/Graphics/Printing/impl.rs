pub trait IAsyncGetSendNotificationCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
pub trait IAsyncGetSrvReferralCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
    fn FinishAsyncCallWithData();
}
pub trait IBidiAsyncNotifyChannelImpl: Sized + IPrintAsyncNotifyChannelImpl {
    fn CreateNotificationChannel();
    fn GetPrintName();
    fn GetChannelNotificationType();
    fn AsyncGetNotificationSendResponse();
    fn AsyncCloseChannel();
}
pub trait IPrintAsyncCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
}
pub trait IPrintAsyncNewChannelCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
pub trait IPrintAsyncNotifyImpl: Sized {
    fn CreatePrintAsyncNotifyChannel();
    fn CreatePrintAsyncNotifyRegistration();
}
pub trait IPrintAsyncNotifyCallbackImpl: Sized {
    fn OnEventNotify();
    fn ChannelClosed();
}
pub trait IPrintAsyncNotifyChannelImpl: Sized {
    fn SendNotification();
    fn CloseChannel();
}
pub trait IPrintAsyncNotifyDataObjectImpl: Sized {
    fn AcquireData();
    fn ReleaseData();
}
pub trait IPrintAsyncNotifyRegistrationImpl: Sized {
    fn RegisterForNotifications();
    fn UnregisterForNotifications();
}
pub trait IPrintAsyncNotifyServerReferralImpl: Sized {
    fn GetServerReferral();
    fn AsyncGetServerReferral();
    fn SetServerReferral();
}
pub trait IPrintBidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNewChannel();
}
pub trait IPrintCoreHelperImpl: Sized {
    fn GetOption();
    fn SetOptions();
    fn EnumConstrainedOptions();
    fn WhyConstrained();
    fn EnumFeatures();
    fn EnumOptions();
    fn GetFontSubstitution();
    fn SetFontSubstitution();
    fn CreateInstanceOfMSXMLObject();
}
pub trait IPrintCoreHelperPSImpl: Sized + IPrintCoreHelperImpl {
    fn GetGlobalAttribute();
    fn GetFeatureAttribute();
    fn GetOptionAttribute();
}
pub trait IPrintCoreHelperUniImpl: Sized + IPrintCoreHelperImpl {
    fn CreateGDLSnapshot();
    fn CreateDefaultGDLSnapshot();
}
pub trait IPrintCoreHelperUni2Impl: Sized + IPrintCoreHelperUniImpl + IPrintCoreHelperImpl {
    fn GetNamedCommand();
}
pub trait IPrintCoreUI2Impl: Sized + IPrintOemDriverUIImpl {
    fn GetOptions();
    fn SetOptions();
    fn EnumConstrainedOptions();
    fn WhyConstrained();
    fn GetGlobalAttribute();
    fn GetFeatureAttribute();
    fn GetOptionAttribute();
    fn EnumFeatures();
    fn EnumOptions();
    fn QuerySimulationSupport();
}
pub trait IPrintJobImpl: Sized {
    fn Name();
    fn Id();
    fn PrintedPages();
    fn TotalPages();
    fn Status();
    fn SubmissionTime();
    fn RequestCancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintJobCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
pub trait IPrintOemCommonImpl: Sized {
    fn GetInfo();
    fn DevMode();
}
pub trait IPrintOemDriverUIImpl: Sized {
    fn DrvGetDriverSetting();
    fn DrvUpgradeRegistrySetting();
    fn DrvUpdateUISetting();
}
pub trait IPrintOemUIImpl: Sized + IPrintOemCommonImpl {
    fn PublishDriverInterface();
    fn CommonUIProp();
    fn DocumentPropertySheets();
    fn DevicePropertySheets();
    fn DevQueryPrintEx();
    fn DeviceCapabilitiesA();
    fn UpgradePrinter();
    fn PrinterEvent();
    fn DriverEvent();
    fn QueryColorProfile();
    fn FontInstallerDlgProc();
    fn UpdateExternalFonts();
}
pub trait IPrintOemUI2Impl: Sized + IPrintOemUIImpl + IPrintOemCommonImpl {
    fn QueryJobAttributes();
    fn HideStandardUI();
    fn DocumentEvent();
}
pub trait IPrintOemUIMXDCImpl: Sized {
    fn AdjustImageableArea();
    fn AdjustImageCompression();
    fn AdjustDPI();
}
pub trait IPrintPreviewDxgiPackageTargetImpl: Sized {
    fn SetJobPageCount();
    fn DrawPage();
    fn InvalidatePreview();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperationImpl: Sized + IDispatchImpl {
    fn Start();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperationEventImpl: Sized + IDispatchImpl {
    fn Completed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilitiesImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetFeatureByKeyName();
    fn GetFeature();
    fn PageImageableSize();
    fn JobCopiesAllDocumentsMinValue();
    fn JobCopiesAllDocumentsMaxValue();
    fn GetSelectedOptionInPrintTicket();
    fn GetOptions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilities2Impl: Sized + IPrintSchemaCapabilitiesImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterDefinition();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaDisplayableElementImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn DisplayName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaElementImpl: Sized + IDispatchImpl {
    fn XmlNode();
    fn Name();
    fn NamespaceUri();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaFeatureImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn SelectedOption();
    fn SetSelectedOption();
    fn SelectionType();
    fn GetOption();
    fn DisplayUI();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaNUpOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn PagesPerSheet();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOptionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn Selected();
    fn Constrained();
    fn GetPropertyValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOptionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageImageableSizeImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn ImageableSizeWidthInMicrons();
    fn ImageableSizeHeightInMicrons();
    fn OriginWidthInMicrons();
    fn OriginHeightInMicrons();
    fn ExtentWidthInMicrons();
    fn ExtentHeightInMicrons();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageMediaSizeOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn WidthInMicrons();
    fn HeightInMicrons();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterDefinitionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn UserInputRequired();
    fn UnitType();
    fn DataType();
    fn RangeMin();
    fn RangeMax();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterInitializerImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaTicketImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetFeatureByKeyName();
    fn GetFeature();
    fn ValidateAsync();
    fn CommitAsync();
    fn NotifyXmlChanged();
    fn GetCapabilities();
    fn JobCopiesAllDocuments();
    fn SetJobCopiesAllDocuments();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaTicket2Impl: Sized + IPrintSchemaTicketImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterInitializer();
}
pub trait IPrintTicketProviderImpl: Sized {
    fn GetSupportedVersions();
    fn BindPrinter();
    fn QueryDeviceNamespace();
    fn ConvertPrintTicketToDevMode();
    fn ConvertDevModeToPrintTicket();
    fn GetPrintCapabilities();
    fn ValidatePrintTicket();
}
pub trait IPrintTicketProvider2Impl: Sized + IPrintTicketProviderImpl {
    fn GetPrintDeviceCapabilities();
    fn GetPrintDeviceResources();
}
pub trait IPrintUnidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNotification();
}
pub trait IPrinterBidiSetRequestCallbackImpl: Sized {
    fn Completed();
}
pub trait IPrinterExtensionAsyncOperationImpl: Sized {
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContextImpl: Sized + IDispatchImpl {
    fn PrinterQueue();
    fn PrintSchemaTicket();
    fn DriverProperties();
    fn UserProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContextCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEventImpl: Sized + IDispatchImpl {
    fn OnDriverEvent();
    fn OnPrinterQueuesEnumerated();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEventArgsImpl: Sized + IPrinterExtensionContextImpl + IDispatchImpl {
    fn BidiNotification();
    fn ReasonId();
    fn Request();
    fn SourceApplication();
    fn DetailedReasonId();
    fn WindowModal();
    fn WindowParent();
}
pub trait IPrinterExtensionManagerImpl: Sized {
    fn EnableEvents();
    fn DisableEvents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionRequestImpl: Sized + IDispatchImpl {
    fn Cancel();
    fn Complete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterPropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool();
    fn SetBool();
    fn GetInt32();
    fn SetInt32();
    fn GetString();
    fn SetString();
    fn GetBytes();
    fn SetBytes();
    fn GetReadStream();
    fn GetWriteStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueImpl: Sized + IDispatchImpl {
    fn Handle();
    fn Name();
    fn SendBidiQuery();
    fn GetProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueue2Impl: Sized + IPrinterQueueImpl + IDispatchImpl {
    fn SendBidiSetRequestAsync();
    fn GetPrinterQueueView();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueEventImpl: Sized + IDispatchImpl {
    fn OnBidiResponseReceived();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueViewImpl: Sized + IDispatchImpl {
    fn SetViewRange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueViewEventImpl: Sized + IDispatchImpl {
    fn OnChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptContextImpl: Sized + IDispatchImpl {
    fn DriverProperties();
    fn QueueProperties();
    fn UserProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptablePropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool();
    fn SetBool();
    fn GetInt32();
    fn SetInt32();
    fn GetString();
    fn SetString();
    fn GetBytes();
    fn SetBytes();
    fn GetReadStream();
    fn GetWriteStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptablePropertyBag2Impl: Sized + IPrinterScriptablePropertyBagImpl + IDispatchImpl {
    fn GetReadStreamAsXML();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableSequentialStreamImpl: Sized + IDispatchImpl {
    fn Read();
    fn Write();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableStreamImpl: Sized + IPrinterScriptableSequentialStreamImpl + IDispatchImpl {
    fn Commit();
    fn Seek();
    fn SetSize();
}
pub trait IXpsRasterizationFactoryImpl: Sized {
    fn CreateRasterizer();
}
pub trait IXpsRasterizationFactory1Impl: Sized {
    fn CreateRasterizer();
}
pub trait IXpsRasterizationFactory2Impl: Sized {
    fn CreateRasterizer();
}
pub trait IXpsRasterizerImpl: Sized {
    fn RasterizeRect();
    fn SetMinimalLineWidth();
}
pub trait IXpsRasterizerNotificationCallbackImpl: Sized {
    fn Continue();
}
