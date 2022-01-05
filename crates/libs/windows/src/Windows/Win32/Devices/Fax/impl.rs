#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountImpl: Sized + IDispatchImpl {
    fn AccountName();
    fn Folders();
    fn ListenToAccountEvents();
    fn RegisteredEvents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountFoldersImpl: Sized + IDispatchImpl {
    fn OutgoingQueue();
    fn IncomingQueue();
    fn IncomingArchive();
    fn OutgoingArchive();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingArchiveImpl: Sized + IDispatchImpl {
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingQueueImpl: Sized + IDispatchImpl {
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountNotifyImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingArchiveImpl: Sized + IDispatchImpl {
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingQueueImpl: Sized + IDispatchImpl {
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountSetImpl: Sized + IDispatchImpl {
    fn GetAccounts();
    fn GetAccount();
    fn AddAccount();
    fn RemoveAccount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivityImpl: Sized + IDispatchImpl {
    fn IncomingMessages();
    fn RoutingMessages();
    fn OutgoingMessages();
    fn QueuedMessages();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivityLoggingImpl: Sized + IDispatchImpl {
    fn LogIncoming();
    fn SetLogIncoming();
    fn LogOutgoing();
    fn SetLogOutgoing();
    fn DatabasePath();
    fn SetDatabasePath();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxConfigurationImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveLocation();
    fn SetArchiveLocation();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn ArchiveAgeLimit();
    fn SetArchiveAgeLimit();
    fn ArchiveSizeLow();
    fn ArchiveSizeHigh();
    fn OutgoingQueueBlocked();
    fn SetOutgoingQueueBlocked();
    fn OutgoingQueuePaused();
    fn SetOutgoingQueuePaused();
    fn AllowPersonalCoverPages();
    fn SetAllowPersonalCoverPages();
    fn UseDeviceTSID();
    fn SetUseDeviceTSID();
    fn Retries();
    fn SetRetries();
    fn RetryDelay();
    fn SetRetryDelay();
    fn DiscountRateStart();
    fn SetDiscountRateStart();
    fn DiscountRateEnd();
    fn SetDiscountRateEnd();
    fn OutgoingQueueAgeLimit();
    fn SetOutgoingQueueAgeLimit();
    fn Branding();
    fn SetBranding();
    fn IncomingQueueBlocked();
    fn SetIncomingQueueBlocked();
    fn AutoCreateAccountOnConnect();
    fn SetAutoCreateAccountOnConnect();
    fn IncomingFaxesArePublic();
    fn SetIncomingFaxesArePublic();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceImpl: Sized + IDispatchImpl {
    fn Id();
    fn DeviceName();
    fn ProviderUniqueName();
    fn PoweredOff();
    fn ReceivingNow();
    fn SendingNow();
    fn UsedRoutingMethods();
    fn Description();
    fn SetDescription();
    fn SendEnabled();
    fn SetSendEnabled();
    fn ReceiveMode();
    fn SetReceiveMode();
    fn RingsBeforeAnswer();
    fn SetRingsBeforeAnswer();
    fn CSID();
    fn SetCSID();
    fn TSID();
    fn SetTSID();
    fn Refresh();
    fn Save();
    fn GetExtensionProperty();
    fn SetExtensionProperty();
    fn UseRoutingMethod();
    fn RingingNow();
    fn AnswerCall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceIdsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn SetOrder();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProviderImpl: Sized + IDispatchImpl {
    fn FriendlyName();
    fn ImageName();
    fn UniqueName();
    fn TapiProviderName();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Status();
    fn InitErrorCode();
    fn DeviceIds();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProvidersImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDevicesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemById();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocumentImpl: Sized + IDispatchImpl {
    fn Body();
    fn SetBody();
    fn Sender();
    fn Recipients();
    fn CoverPage();
    fn SetCoverPage();
    fn Subject();
    fn SetSubject();
    fn Note();
    fn SetNote();
    fn ScheduleTime();
    fn SetScheduleTime();
    fn ReceiptAddress();
    fn SetReceiptAddress();
    fn DocumentName();
    fn SetDocumentName();
    fn CallHandle();
    fn SetCallHandle();
    fn CoverPageType();
    fn SetCoverPageType();
    fn ScheduleType();
    fn SetScheduleType();
    fn ReceiptType();
    fn SetReceiptType();
    fn GroupBroadcastReceipts();
    fn SetGroupBroadcastReceipts();
    fn Priority();
    fn SetPriority();
    fn TapiConnection();
    fn putref_TapiConnection();
    fn Submit();
    fn ConnectedSubmit();
    fn AttachFaxToReceipt();
    fn SetAttachFaxToReceipt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocument2Impl: Sized + IFaxDocumentImpl + IDispatchImpl {
    fn SubmissionId();
    fn Bodies();
    fn SetBodies();
    fn Submit2();
    fn ConnectedSubmit2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxEventLoggingImpl: Sized + IDispatchImpl {
    fn InitEventsLevel();
    fn SetInitEventsLevel();
    fn InboundEventsLevel();
    fn SetInboundEventsLevel();
    fn OutboundEventsLevel();
    fn SetOutboundEventsLevel();
    fn GeneralEventsLevel();
    fn SetGeneralEventsLevel();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxFoldersImpl: Sized + IDispatchImpl {
    fn OutgoingQueue();
    fn IncomingQueue();
    fn IncomingArchive();
    fn OutgoingArchive();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingImpl: Sized + IDispatchImpl {
    fn GetExtensions();
    fn GetMethods();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtensionImpl: Sized + IDispatchImpl {
    fn FriendlyName();
    fn ImageName();
    fn UniqueName();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Status();
    fn InitErrorCode();
    fn Methods();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethodImpl: Sized + IDispatchImpl {
    fn Name();
    fn GUID();
    fn FunctionName();
    fn ExtensionFriendlyName();
    fn ExtensionImageName();
    fn Priority();
    fn SetPriority();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethodsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingArchiveImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveFolder();
    fn SetArchiveFolder();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn AgeLimit();
    fn SetAgeLimit();
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn Save();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJobImpl: Sized + IDispatchImpl {
    fn Size();
    fn Id();
    fn CurrentPage();
    fn DeviceId();
    fn Status();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CallerId();
    fn RoutingInformation();
    fn JobType();
    fn Cancel();
    fn Refresh();
    fn CopyTiff();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJobsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessageImpl: Sized + IDispatchImpl {
    fn Id();
    fn Pages();
    fn Size();
    fn DeviceName();
    fn Retries();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CallerId();
    fn RoutingInformation();
    fn CopyTiff();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessage2Impl: Sized + IFaxIncomingMessageImpl + IDispatchImpl {
    fn Subject();
    fn SetSubject();
    fn SenderName();
    fn SetSenderName();
    fn SenderFaxNumber();
    fn SetSenderFaxNumber();
    fn HasCoverPage();
    fn SetHasCoverPage();
    fn Recipients();
    fn SetRecipients();
    fn WasReAssigned();
    fn Read();
    fn SetRead();
    fn ReAssign();
    fn Save();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessageIteratorImpl: Sized + IDispatchImpl {
    fn Message();
    fn PrefetchSize();
    fn SetPrefetchSize();
    fn AtEOF();
    fn MoveFirst();
    fn MoveNext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingQueueImpl: Sized + IDispatchImpl {
    fn Blocked();
    fn SetBlocked();
    fn Refresh();
    fn Save();
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxJobStatusImpl: Sized + IDispatchImpl {
    fn Status();
    fn Pages();
    fn Size();
    fn CurrentPage();
    fn DeviceId();
    fn CSID();
    fn TSID();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn JobType();
    fn ScheduledTime();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CallerId();
    fn RoutingInformation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxLoggingOptionsImpl: Sized + IDispatchImpl {
    fn EventLogging();
    fn ActivityLogging();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingImpl: Sized + IDispatchImpl {
    fn GetGroups();
    fn GetRules();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn Status();
    fn DeviceIds();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroupsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRuleImpl: Sized + IDispatchImpl {
    fn CountryCode();
    fn AreaCode();
    fn Status();
    fn UseDevice();
    fn SetUseDevice();
    fn DeviceId();
    fn SetDeviceId();
    fn GroupName();
    fn SetGroupName();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRulesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByCountryAndArea();
    fn RemoveByCountryAndArea();
    fn Remove();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingArchiveImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveFolder();
    fn SetArchiveFolder();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn AgeLimit();
    fn SetAgeLimit();
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn Save();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJobImpl: Sized + IDispatchImpl {
    fn Subject();
    fn DocumentName();
    fn Pages();
    fn Size();
    fn SubmissionId();
    fn Id();
    fn OriginalScheduledTime();
    fn SubmissionTime();
    fn ReceiptType();
    fn Priority();
    fn Sender();
    fn Recipient();
    fn CurrentPage();
    fn DeviceId();
    fn Status();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn ScheduledTime();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn GroupBroadcastReceipts();
    fn Pause();
    fn Resume();
    fn Restart();
    fn CopyTiff();
    fn Refresh();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJob2Impl: Sized + IFaxOutgoingJobImpl + IDispatchImpl {
    fn HasCoverPage();
    fn ReceiptAddress();
    fn ScheduleType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJobsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessageImpl: Sized + IDispatchImpl {
    fn SubmissionId();
    fn Id();
    fn Subject();
    fn DocumentName();
    fn Retries();
    fn Pages();
    fn Size();
    fn OriginalScheduledTime();
    fn SubmissionTime();
    fn Priority();
    fn Sender();
    fn Recipient();
    fn DeviceName();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CopyTiff();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessage2Impl: Sized + IFaxOutgoingMessageImpl + IDispatchImpl {
    fn HasCoverPage();
    fn ReceiptType();
    fn ReceiptAddress();
    fn Read();
    fn SetRead();
    fn Save();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessageIteratorImpl: Sized + IDispatchImpl {
    fn Message();
    fn AtEOF();
    fn PrefetchSize();
    fn SetPrefetchSize();
    fn MoveFirst();
    fn MoveNext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingQueueImpl: Sized + IDispatchImpl {
    fn Blocked();
    fn SetBlocked();
    fn Paused();
    fn SetPaused();
    fn AllowPersonalCoverPages();
    fn SetAllowPersonalCoverPages();
    fn UseDeviceTSID();
    fn SetUseDeviceTSID();
    fn Retries();
    fn SetRetries();
    fn RetryDelay();
    fn SetRetryDelay();
    fn DiscountRateStart();
    fn SetDiscountRateStart();
    fn DiscountRateEnd();
    fn SetDiscountRateEnd();
    fn AgeLimit();
    fn SetAgeLimit();
    fn Branding();
    fn SetBranding();
    fn Refresh();
    fn Save();
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxReceiptOptionsImpl: Sized + IDispatchImpl {
    fn AuthenticationType();
    fn SetAuthenticationType();
    fn SMTPServer();
    fn SetSMTPServer();
    fn SMTPPort();
    fn SetSMTPPort();
    fn SMTPSender();
    fn SetSMTPSender();
    fn SMTPUser();
    fn SetSMTPUser();
    fn AllowedReceipts();
    fn SetAllowedReceipts();
    fn SMTPPassword();
    fn SetSMTPPassword();
    fn Refresh();
    fn Save();
    fn UseForInboundRouting();
    fn SetUseForInboundRouting();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipientImpl: Sized + IDispatchImpl {
    fn FaxNumber();
    fn SetFaxNumber();
    fn Name();
    fn SetName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipientsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurityImpl: Sized + IDispatchImpl {
    fn Descriptor();
    fn SetDescriptor();
    fn GrantedRights();
    fn Refresh();
    fn Save();
    fn InformationType();
    fn SetInformationType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurity2Impl: Sized + IDispatchImpl {
    fn Descriptor();
    fn SetDescriptor();
    fn GrantedRights();
    fn Refresh();
    fn Save();
    fn InformationType();
    fn SetInformationType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSenderImpl: Sized + IDispatchImpl {
    fn BillingCode();
    fn SetBillingCode();
    fn City();
    fn SetCity();
    fn Company();
    fn SetCompany();
    fn Country();
    fn SetCountry();
    fn Department();
    fn SetDepartment();
    fn Email();
    fn SetEmail();
    fn FaxNumber();
    fn SetFaxNumber();
    fn HomePhone();
    fn SetHomePhone();
    fn Name();
    fn SetName();
    fn TSID();
    fn SetTSID();
    fn OfficePhone();
    fn SetOfficePhone();
    fn OfficeLocation();
    fn SetOfficeLocation();
    fn State();
    fn SetState();
    fn StreetAddress();
    fn SetStreetAddress();
    fn Title();
    fn SetTitle();
    fn ZipCode();
    fn SetZipCode();
    fn LoadDefaultSender();
    fn SaveDefaultSender();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerImpl: Sized + IDispatchImpl {
    fn Connect();
    fn ServerName();
    fn GetDeviceProviders();
    fn GetDevices();
    fn InboundRouting();
    fn Folders();
    fn LoggingOptions();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Activity();
    fn OutboundRouting();
    fn ReceiptOptions();
    fn Security();
    fn Disconnect();
    fn GetExtensionProperty();
    fn SetExtensionProperty();
    fn ListenToServerEvents();
    fn RegisterDeviceProvider();
    fn UnregisterDeviceProvider();
    fn RegisterInboundRoutingExtension();
    fn UnregisterInboundRoutingExtension();
    fn RegisteredEvents();
    fn APIVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServer2Impl: Sized + IFaxServerImpl + IDispatchImpl {
    fn Configuration();
    fn CurrentAccount();
    fn FaxAccountSet();
    fn Security2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotifyImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotify2Impl: Sized + IDispatchImpl {}
pub trait IStiDeviceImpl: Sized {
    fn Initialize();
    fn GetCapabilities();
    fn GetStatus();
    fn DeviceReset();
    fn Diagnostic();
    fn Escape();
    fn GetLastError();
    fn LockDevice();
    fn UnLockDevice();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn Subscribe();
    fn GetLastNotificationData();
    fn UnSubscribe();
    fn GetLastErrorInfo();
}
pub trait IStiDeviceControlImpl: Sized {
    fn Initialize();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn RawDeviceControl();
    fn GetLastError();
    fn GetMyDevicePortName();
    fn GetMyDeviceHandle();
    fn GetMyDeviceOpenMode();
    fn WriteToErrorLog();
}
pub trait IStiUSDImpl: Sized {
    fn Initialize();
    fn GetCapabilities();
    fn GetStatus();
    fn DeviceReset();
    fn Diagnostic();
    fn Escape();
    fn GetLastError();
    fn LockDevice();
    fn UnLockDevice();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn SetNotificationHandle();
    fn GetNotificationData();
    fn GetLastErrorInfo();
}
pub trait IStillImageWImpl: Sized {
    fn Initialize();
    fn GetDeviceList();
    fn GetDeviceInfo();
    fn CreateDevice();
    fn GetDeviceValue();
    fn SetDeviceValue();
    fn GetSTILaunchInformation();
    fn RegisterLaunchApplication();
    fn UnregisterLaunchApplication();
    fn EnableHwNotifications();
    fn GetHwNotificationState();
    fn RefreshDeviceBus();
    fn LaunchApplicationForDevice();
    fn SetupDeviceParameters();
    fn WriteToErrorLog();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IFaxAccountNotifyImpl: Sized + IDispatchImpl {
    fn OnIncomingJobAdded();
    fn OnIncomingJobRemoved();
    fn OnIncomingJobChanged();
    fn OnOutgoingJobAdded();
    fn OnOutgoingJobRemoved();
    fn OnOutgoingJobChanged();
    fn OnIncomingMessageAdded();
    fn OnIncomingMessageRemoved();
    fn OnOutgoingMessageAdded();
    fn OnOutgoingMessageRemoved();
    fn OnServerShutDown();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IFaxServerNotify2Impl: Sized + IDispatchImpl {
    fn OnIncomingJobAdded();
    fn OnIncomingJobRemoved();
    fn OnIncomingJobChanged();
    fn OnOutgoingJobAdded();
    fn OnOutgoingJobRemoved();
    fn OnOutgoingJobChanged();
    fn OnIncomingMessageAdded();
    fn OnIncomingMessageRemoved();
    fn OnOutgoingMessageAdded();
    fn OnOutgoingMessageRemoved();
    fn OnReceiptOptionsChange();
    fn OnActivityLoggingConfigChange();
    fn OnSecurityConfigChange();
    fn OnEventLoggingConfigChange();
    fn OnOutgoingQueueConfigChange();
    fn OnOutgoingArchiveConfigChange();
    fn OnIncomingArchiveConfigChange();
    fn OnDevicesConfigChange();
    fn OnOutboundRoutingGroupsConfigChange();
    fn OnOutboundRoutingRulesConfigChange();
    fn OnServerActivityChange();
    fn OnQueuesStatusChange();
    fn OnNewCall();
    fn OnServerShutDown();
    fn OnDeviceStatusChange();
    fn OnGeneralServerConfigChanged();
}
