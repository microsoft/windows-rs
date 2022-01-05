#[cfg(feature = "Win32_System_Com")]
pub trait DIFsrmClassificationEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmAccessDeniedRemediationClientImpl: Sized + IDispatchImpl {
    fn Show();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionImpl: Sized + IDispatchImpl {
    fn Id();
    fn ActionType();
    fn RunLimitInterval();
    fn SetRunLimitInterval();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionCommandImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn ExecutablePath();
    fn SetExecutablePath();
    fn Arguments();
    fn SetArguments();
    fn Account();
    fn SetAccount();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
    fn MonitorCommand();
    fn SetMonitorCommand();
    fn KillTimeOut();
    fn SetKillTimeOut();
    fn LogResult();
    fn SetLogResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionEmailImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn MailFrom();
    fn SetMailFrom();
    fn MailReplyTo();
    fn SetMailReplyTo();
    fn MailTo();
    fn SetMailTo();
    fn MailCc();
    fn SetMailCc();
    fn MailBcc();
    fn SetMailBcc();
    fn MailSubject();
    fn SetMailSubject();
    fn MessageText();
    fn SetMessageText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionEmail2Impl: Sized + IFsrmActionEmailImpl + IFsrmActionImpl + IDispatchImpl {
    fn AttachmentFileListSize();
    fn SetAttachmentFileListSize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionEventLogImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn EventType();
    fn SetEventType();
    fn MessageText();
    fn SetMessageText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmActionReportImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn ReportTypes();
    fn SetReportTypes();
    fn MailTo();
    fn SetMailTo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmAutoApplyQuotaImpl: Sized + IFsrmQuotaObjectImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn ExcludeFolders();
    fn SetExcludeFolders();
    fn CommitAndUpdateDerived();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmClassificationManagerImpl: Sized + IDispatchImpl {
    fn ClassificationReportFormats();
    fn SetClassificationReportFormats();
    fn Logging();
    fn SetLogging();
    fn ClassificationReportMailTo();
    fn SetClassificationReportMailTo();
    fn ClassificationReportEnabled();
    fn SetClassificationReportEnabled();
    fn ClassificationLastReportPathWithoutExtension();
    fn ClassificationLastError();
    fn ClassificationRunningStatus();
    fn EnumPropertyDefinitions();
    fn CreatePropertyDefinition();
    fn GetPropertyDefinition();
    fn EnumRules();
    fn CreateRule();
    fn GetRule();
    fn EnumModuleDefinitions();
    fn CreateModuleDefinition();
    fn GetModuleDefinition();
    fn RunClassification();
    fn WaitForClassificationCompletion();
    fn CancelClassification();
    fn EnumFileProperties();
    fn GetFileProperty();
    fn SetFileProperty();
    fn ClearFileProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmClassificationManager2Impl: Sized + IFsrmClassificationManagerImpl + IDispatchImpl {
    fn ClassifyFiles();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmClassificationRuleImpl: Sized + IFsrmRuleImpl + IFsrmObjectImpl + IDispatchImpl {
    fn ExecutionOption();
    fn SetExecutionOption();
    fn PropertyAffected();
    fn SetPropertyAffected();
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmClassifierModuleDefinitionImpl: Sized + IFsrmPipelineModuleDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn PropertiesAffected();
    fn SetPropertiesAffected();
    fn PropertiesUsed();
    fn SetPropertiesUsed();
    fn NeedsExplicitValue();
    fn SetNeedsExplicitValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmClassifierModuleImplementationImpl: Sized + IFsrmPipelineModuleImplementationImpl + IDispatchImpl {
    fn LastModified();
    fn UseRulesAndDefinitions();
    fn OnBeginFile();
    fn DoesPropertyValueApply();
    fn GetPropertyValueToApply();
    fn OnEndFile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn State();
    fn Cancel();
    fn WaitForCompletion();
    fn GetById();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmCommittableCollectionImpl: Sized + IFsrmMutableCollectionImpl + IFsrmCollectionImpl + IDispatchImpl {
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmDerivedObjectsResultImpl: Sized + IDispatchImpl {
    fn DerivedObjects();
    fn Results();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmExportImportImpl: Sized + IDispatchImpl {
    fn ExportFileGroups();
    fn ImportFileGroups();
    fn ExportFileScreenTemplates();
    fn ImportFileScreenTemplates();
    fn ExportQuotaTemplates();
    fn ImportQuotaTemplates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileConditionImpl: Sized + IDispatchImpl {
    fn Type();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileConditionPropertyImpl: Sized + IFsrmFileConditionImpl + IDispatchImpl {
    fn PropertyName();
    fn SetPropertyName();
    fn PropertyId();
    fn SetPropertyId();
    fn Operator();
    fn SetOperator();
    fn ValueType();
    fn SetValueType();
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileGroupImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Members();
    fn SetMembers();
    fn NonMembers();
    fn SetNonMembers();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileGroupImportedImpl: Sized + IFsrmFileGroupImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileGroupManagerImpl: Sized + IDispatchImpl {
    fn CreateFileGroup();
    fn GetFileGroup();
    fn EnumFileGroups();
    fn ExportFileGroups();
    fn ImportFileGroups();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileManagementJobImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn Enabled();
    fn SetEnabled();
    fn OperationType();
    fn SetOperationType();
    fn ExpirationDirectory();
    fn SetExpirationDirectory();
    fn CustomAction();
    fn Notifications();
    fn Logging();
    fn SetLogging();
    fn ReportEnabled();
    fn SetReportEnabled();
    fn Formats();
    fn SetFormats();
    fn MailTo();
    fn SetMailTo();
    fn DaysSinceFileCreated();
    fn SetDaysSinceFileCreated();
    fn DaysSinceFileLastAccessed();
    fn SetDaysSinceFileLastAccessed();
    fn DaysSinceFileLastModified();
    fn SetDaysSinceFileLastModified();
    fn PropertyConditions();
    fn FromDate();
    fn SetFromDate();
    fn Task();
    fn SetTask();
    fn Parameters();
    fn SetParameters();
    fn RunningStatus();
    fn LastError();
    fn LastReportPathWithoutExtension();
    fn LastRun();
    fn FileNamePattern();
    fn SetFileNamePattern();
    fn Run();
    fn WaitForCompletion();
    fn Cancel();
    fn AddNotification();
    fn DeleteNotification();
    fn ModifyNotification();
    fn CreateNotificationAction();
    fn EnumNotificationActions();
    fn CreatePropertyCondition();
    fn CreateCustomAction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileManagementJobManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn EnumFileManagementJobs();
    fn CreateFileManagementJob();
    fn GetFileManagementJob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenImpl: Sized + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn SourceTemplateName();
    fn MatchesSourceTemplate();
    fn UserSid();
    fn UserAccount();
    fn ApplyTemplate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenBaseImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn BlockedFileGroups();
    fn SetBlockedFileGroups();
    fn FileScreenFlags();
    fn SetFileScreenFlags();
    fn CreateAction();
    fn EnumActions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenExceptionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn AllowedFileGroups();
    fn SetAllowedFileGroups();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn CreateFileScreen();
    fn GetFileScreen();
    fn EnumFileScreens();
    fn CreateFileScreenException();
    fn GetFileScreenException();
    fn EnumFileScreenExceptions();
    fn CreateFileScreenCollection();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenTemplateImpl: Sized + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn CopyTemplate();
    fn CommitAndUpdateDerived();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenTemplateImportedImpl: Sized + IFsrmFileScreenTemplateImpl + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmFileScreenTemplateManagerImpl: Sized + IDispatchImpl {
    fn CreateTemplate();
    fn GetTemplate();
    fn EnumTemplates();
    fn ExportTemplates();
    fn ImportTemplates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmMutableCollectionImpl: Sized + IFsrmCollectionImpl + IDispatchImpl {
    fn Add();
    fn Remove();
    fn RemoveById();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmObjectImpl: Sized + IDispatchImpl {
    fn Id();
    fn Description();
    fn SetDescription();
    fn Delete();
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPathMapperImpl: Sized + IDispatchImpl {
    fn GetSharePathsForLocalPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPipelineModuleConnectorImpl: Sized + IDispatchImpl {
    fn ModuleImplementation();
    fn ModuleName();
    fn HostingUserAccount();
    fn HostingProcessPid();
    fn Bind();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPipelineModuleDefinitionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn ModuleClsid();
    fn SetModuleClsid();
    fn Name();
    fn SetName();
    fn Company();
    fn SetCompany();
    fn Version();
    fn SetVersion();
    fn ModuleType();
    fn Enabled();
    fn SetEnabled();
    fn NeedsFileContent();
    fn SetNeedsFileContent();
    fn Account();
    fn SetAccount();
    fn SupportedExtensions();
    fn SetSupportedExtensions();
    fn Parameters();
    fn SetParameters();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPipelineModuleImplementationImpl: Sized + IDispatchImpl {
    fn OnLoad();
    fn OnUnload();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
    fn Sources();
    fn PropertyFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyBagImpl: Sized + IDispatchImpl {
    fn Name();
    fn RelativePath();
    fn VolumeName();
    fn RelativeNamespaceRoot();
    fn VolumeIndex();
    fn FileId();
    fn ParentDirectoryId();
    fn Size();
    fn SizeAllocated();
    fn CreationTime();
    fn LastAccessTime();
    fn LastModificationTime();
    fn Attributes();
    fn OwnerSid();
    fn FilePropertyNames();
    fn Messages();
    fn PropertyBagFlags();
    fn GetFileProperty();
    fn SetFileProperty();
    fn AddMessage();
    fn GetFileStreamInterface();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyBag2Impl: Sized + IFsrmPropertyBagImpl + IDispatchImpl {
    fn GetFieldValue();
    fn GetUntrustedInFileProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyConditionImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn Value();
    fn SetValue();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyDefinitionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn PossibleValues();
    fn SetPossibleValues();
    fn ValueDescriptions();
    fn SetValueDescriptions();
    fn Parameters();
    fn SetParameters();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyDefinition2Impl: Sized + IFsrmPropertyDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn PropertyDefinitionFlags();
    fn DisplayName();
    fn SetDisplayName();
    fn AppliesTo();
    fn ValueDefinitions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmPropertyDefinitionValueImpl: Sized + IDispatchImpl {
    fn Name();
    fn DisplayName();
    fn Description();
    fn UniqueID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaImpl: Sized + IFsrmQuotaObjectImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn QuotaUsed();
    fn QuotaPeakUsage();
    fn QuotaPeakUsageTime();
    fn ResetPeakUsage();
    fn RefreshUsageProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaBaseImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn QuotaLimit();
    fn SetQuotaLimit();
    fn QuotaFlags();
    fn SetQuotaFlags();
    fn Thresholds();
    fn AddThreshold();
    fn DeleteThreshold();
    fn ModifyThreshold();
    fn CreateThresholdAction();
    fn EnumThresholdActions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn CreateQuota();
    fn CreateAutoApplyQuota();
    fn GetQuota();
    fn GetAutoApplyQuota();
    fn GetRestrictiveQuota();
    fn EnumQuotas();
    fn EnumAutoApplyQuotas();
    fn EnumEffectiveQuotas();
    fn Scan();
    fn CreateQuotaCollection();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaManagerExImpl: Sized + IFsrmQuotaManagerImpl + IDispatchImpl {
    fn IsAffectedByQuota();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaObjectImpl: Sized + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn UserSid();
    fn UserAccount();
    fn SourceTemplateName();
    fn MatchesSourceTemplate();
    fn ApplyTemplate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaTemplateImpl: Sized + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn CopyTemplate();
    fn CommitAndUpdateDerived();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaTemplateImportedImpl: Sized + IFsrmQuotaTemplateImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmQuotaTemplateManagerImpl: Sized + IDispatchImpl {
    fn CreateTemplate();
    fn GetTemplate();
    fn EnumTemplates();
    fn ExportTemplates();
    fn ImportTemplates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmReportImpl: Sized + IDispatchImpl {
    fn Type();
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn LastGeneratedFileNamePrefix();
    fn GetFilter();
    fn SetFilter();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmReportJobImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Task();
    fn SetTask();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn Formats();
    fn SetFormats();
    fn MailTo();
    fn SetMailTo();
    fn RunningStatus();
    fn LastRun();
    fn LastError();
    fn LastGeneratedInDirectory();
    fn EnumReports();
    fn CreateReport();
    fn Run();
    fn WaitForCompletion();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmReportManagerImpl: Sized + IDispatchImpl {
    fn EnumReportJobs();
    fn CreateReportJob();
    fn GetReportJob();
    fn GetOutputDirectory();
    fn SetOutputDirectory();
    fn IsFilterValidForReportType();
    fn GetDefaultFilter();
    fn SetDefaultFilter();
    fn GetReportSizeLimit();
    fn SetReportSizeLimit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmReportSchedulerImpl: Sized + IDispatchImpl {
    fn VerifyNamespaces();
    fn CreateScheduleTask();
    fn ModifyScheduleTask();
    fn DeleteScheduleTask();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmRuleImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn RuleType();
    fn ModuleDefinitionName();
    fn SetModuleDefinitionName();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn RuleFlags();
    fn SetRuleFlags();
    fn Parameters();
    fn SetParameters();
    fn LastModified();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmSettingImpl: Sized + IDispatchImpl {
    fn SmtpServer();
    fn SetSmtpServer();
    fn MailFrom();
    fn SetMailFrom();
    fn AdminEmail();
    fn SetAdminEmail();
    fn DisableCommandLine();
    fn SetDisableCommandLine();
    fn EnableScreeningAudit();
    fn SetEnableScreeningAudit();
    fn EmailTest();
    fn SetActionRunLimitInterval();
    fn GetActionRunLimitInterval();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmStorageModuleDefinitionImpl: Sized + IFsrmPipelineModuleDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Capabilities();
    fn SetCapabilities();
    fn StorageType();
    fn SetStorageType();
    fn UpdatesFileContent();
    fn SetUpdatesFileContent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsrmStorageModuleImplementationImpl: Sized + IFsrmPipelineModuleImplementationImpl + IDispatchImpl {
    fn UseDefinitions();
    fn LoadProperties();
    fn SaveProperties();
}
