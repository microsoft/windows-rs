pub trait IActivatedEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen>;
}
pub trait IActivatedEventArgsWithUserImpl: Sized + IActivatedEventArgsImpl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
pub trait IApplicationViewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32>;
}
pub trait IAppointmentsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IAppointmentsProviderActivatedEventArgsImpl {
    fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
pub trait IBackgroundActivatedEventArgsImpl: Sized {
    fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance>;
}
pub trait IBarcodeScannerPreviewActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait ICachedFileUpdaterActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
pub trait ICameraSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait ICommandLineActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandLineActivationOperationImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()>;
    fn ExitCode(&self) -> ::windows::core::Result<i32>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
pub trait IContactActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IContactCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactMapActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactMessageActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactPanelActivatedEventArgsImpl: Sized {
    fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI>;
}
pub trait IContactPostActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactVideoCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IContactActivatedEventArgsImpl {
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
}
pub trait IContactsProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IContinuationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
pub trait IDeviceActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IDevicePairingActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
pub trait IDialReceiverActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IFileActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyNameImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IFileActivatedEventArgsWithNeighboringFilesImpl: Sized + IActivatedEventArgsImpl + IFileActivatedEventArgsImpl {
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
pub trait IFileOpenPickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
pub trait IFileOpenPickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "deprecated")]
pub trait IFileOpenPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
pub trait IFileSavePickerActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
pub trait IFileSavePickerActivatedEventArgs2Impl: Sized {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "deprecated")]
pub trait IFileSavePickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "deprecated")]
pub trait IFolderPickerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
}
pub trait ILaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait ILaunchActivatedEventArgs2Impl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo>;
}
pub trait ILockScreenActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait ILockScreenCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + ILaunchActivatedEventArgsImpl {
    fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI>;
}
pub trait IPhoneCallActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
pub trait IPickerReturnedActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IPrelaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn PrelaunchActivated(&self) -> ::windows::core::Result<bool>;
}
pub trait IPrint3DWorkflowActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
pub trait IPrintTaskSettingsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
pub trait IProtocolActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataImpl: Sized + IActivatedEventArgsImpl {
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
pub trait IProtocolForResultsActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation>;
}
pub trait IRestrictedLaunchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait ISearchActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait ISearchActivatedEventArgsWithLinguisticDetailsImpl: Sized {
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
pub trait IShareTargetActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplashScreenImpl: Sized {
    fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Dismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDismissed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IStartupTaskActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileActivatedInfoImpl: Sized {
    fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>;
}
pub trait IToastNotificationActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
pub trait IUserDataAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
pub trait IViewSwitcherProviderImpl: Sized + IActivatedEventArgsImpl {
    fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
pub trait IVoiceCommandActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
pub trait IWalletActionActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IWebAccountProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
pub trait IWebAuthenticationBrokerContinuationEventArgsImpl: Sized + IActivatedEventArgsImpl + IContinuationActivatedEventArgsImpl {
    fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
