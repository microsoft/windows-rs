#[cfg(feature = "Win32_System_Com")]
pub trait IActionImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetId();
    fn Type();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn XmlText();
    fn SetXmlText();
    fn Create();
    fn Remove();
    fn Clear();
    fn Context();
    fn SetContext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBootTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComHandlerActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn ClassId();
    fn SetClassId();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDailyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysInterval();
    fn SetDaysInterval();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEmailActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Server();
    fn SetServer();
    fn Subject();
    fn SetSubject();
    fn To();
    fn SetTo();
    fn Cc();
    fn SetCc();
    fn Bcc();
    fn SetBcc();
    fn ReplyTo();
    fn SetReplyTo();
    fn From();
    fn SetFrom();
    fn HeaderFields();
    fn SetHeaderFields();
    fn Body();
    fn SetBody();
    fn Attachments();
    fn SetAttachments();
}
pub trait IEnumWorkItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEventTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Subscription();
    fn SetSubscription();
    fn Delay();
    fn SetDelay();
    fn ValueQueries();
    fn SetValueQueries();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExecActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Path();
    fn SetPath();
    fn Arguments();
    fn SetArguments();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExecAction2Impl: Sized + IExecActionImpl + IActionImpl + IDispatchImpl {
    fn HideAppWindow();
    fn SetHideAppWindow();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIdleSettingsImpl: Sized + IDispatchImpl {
    fn IdleDuration();
    fn SetIdleDuration();
    fn WaitTimeout();
    fn SetWaitTimeout();
    fn StopOnIdleEnd();
    fn SetStopOnIdleEnd();
    fn RestartOnIdle();
    fn SetRestartOnIdle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIdleTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait ILogonTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
    fn UserId();
    fn SetUserId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMaintenanceSettingsImpl: Sized + IDispatchImpl {
    fn SetPeriod();
    fn Period();
    fn SetDeadline();
    fn Deadline();
    fn SetExclusive();
    fn Exclusive();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMonthlyDOWTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfWeek();
    fn SetDaysOfWeek();
    fn WeeksOfMonth();
    fn SetWeeksOfMonth();
    fn MonthsOfYear();
    fn SetMonthsOfYear();
    fn RunOnLastWeekOfMonth();
    fn SetRunOnLastWeekOfMonth();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMonthlyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfMonth();
    fn SetDaysOfMonth();
    fn MonthsOfYear();
    fn SetMonthsOfYear();
    fn RunOnLastDayOfMonth();
    fn SetRunOnLastDayOfMonth();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkSettingsImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Id();
    fn SetId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrincipalImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetId();
    fn DisplayName();
    fn SetDisplayName();
    fn UserId();
    fn SetUserId();
    fn LogonType();
    fn SetLogonType();
    fn GroupId();
    fn SetGroupId();
    fn RunLevel();
    fn SetRunLevel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrincipal2Impl: Sized + IDispatchImpl {
    fn ProcessTokenSidType();
    fn SetProcessTokenSidType();
    fn RequiredPrivilegeCount();
    fn RequiredPrivilege();
    fn AddRequiredPrivilege();
}
pub trait IProvideTaskPageImpl: Sized {
    fn GetPage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegisteredTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn Path();
    fn State();
    fn Enabled();
    fn SetEnabled();
    fn Run();
    fn RunEx();
    fn GetInstances();
    fn LastRunTime();
    fn LastTaskResult();
    fn NumberOfMissedRuns();
    fn NextRunTime();
    fn Definition();
    fn Xml();
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Stop();
    fn GetRunTimes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegisteredTaskCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegistrationInfoImpl: Sized + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn Author();
    fn SetAuthor();
    fn Version();
    fn SetVersion();
    fn Date();
    fn SetDate();
    fn Documentation();
    fn SetDocumentation();
    fn XmlText();
    fn SetXmlText();
    fn URI();
    fn SetURI();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Source();
    fn SetSource();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegistrationTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRepetitionPatternImpl: Sized + IDispatchImpl {
    fn Interval();
    fn SetInterval();
    fn Duration();
    fn SetDuration();
    fn StopAtDurationEnd();
    fn SetStopAtDurationEnd();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRunningTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn InstanceGuid();
    fn Path();
    fn State();
    fn CurrentAction();
    fn Stop();
    fn Refresh();
    fn EnginePID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRunningTaskCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
pub trait IScheduledWorkItemImpl: Sized {
    fn CreateTrigger();
    fn DeleteTrigger();
    fn GetTriggerCount();
    fn GetTrigger();
    fn GetTriggerString();
    fn GetRunTimes();
    fn GetNextRunTime();
    fn SetIdleWait();
    fn GetIdleWait();
    fn Run();
    fn Terminate();
    fn EditWorkItem();
    fn GetMostRecentRunTime();
    fn GetStatus();
    fn GetExitCode();
    fn SetComment();
    fn GetComment();
    fn SetCreator();
    fn GetCreator();
    fn SetWorkItemData();
    fn GetWorkItemData();
    fn SetErrorRetryCount();
    fn GetErrorRetryCount();
    fn SetErrorRetryInterval();
    fn GetErrorRetryInterval();
    fn SetFlags();
    fn GetFlags();
    fn SetAccountInformation();
    fn GetAccountInformation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISessionStateChangeTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
    fn UserId();
    fn SetUserId();
    fn StateChange();
    fn SetStateChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShowMessageActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Title();
    fn SetTitle();
    fn MessageBody();
    fn SetMessageBody();
}
pub trait ITaskImpl: Sized + IScheduledWorkItemImpl {
    fn SetApplicationName();
    fn GetApplicationName();
    fn SetParameters();
    fn GetParameters();
    fn SetWorkingDirectory();
    fn GetWorkingDirectory();
    fn SetPriority();
    fn GetPriority();
    fn SetTaskFlags();
    fn GetTaskFlags();
    fn SetMaxRunTime();
    fn GetMaxRunTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskDefinitionImpl: Sized + IDispatchImpl {
    fn RegistrationInfo();
    fn SetRegistrationInfo();
    fn Triggers();
    fn SetTriggers();
    fn Settings();
    fn SetSettings();
    fn Data();
    fn SetData();
    fn Principal();
    fn SetPrincipal();
    fn Actions();
    fn SetActions();
    fn XmlText();
    fn SetXmlText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskFolderImpl: Sized + IDispatchImpl {
    fn Name();
    fn Path();
    fn GetFolder();
    fn GetFolders();
    fn CreateFolder();
    fn DeleteFolder();
    fn GetTask();
    fn GetTasks();
    fn DeleteTask();
    fn RegisterTask();
    fn RegisterTaskDefinition();
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskFolderCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
pub trait ITaskHandlerImpl: Sized {
    fn Start();
    fn Stop();
    fn Pause();
    fn Resume();
}
pub trait ITaskHandlerStatusImpl: Sized {
    fn UpdateStatus();
    fn TaskCompleted();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskNamedValueCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Create();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskNamedValuePairImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Value();
    fn SetValue();
}
pub trait ITaskSchedulerImpl: Sized {
    fn SetTargetComputer();
    fn GetTargetComputer();
    fn Enum();
    fn Activate();
    fn Delete();
    fn NewWorkItem();
    fn AddWorkItem();
    fn IsOfType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskServiceImpl: Sized + IDispatchImpl {
    fn GetFolder();
    fn GetRunningTasks();
    fn NewTask();
    fn Connect();
    fn Connected();
    fn TargetServer();
    fn ConnectedUser();
    fn ConnectedDomain();
    fn HighestVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettingsImpl: Sized + IDispatchImpl {
    fn AllowDemandStart();
    fn SetAllowDemandStart();
    fn RestartInterval();
    fn SetRestartInterval();
    fn RestartCount();
    fn SetRestartCount();
    fn MultipleInstances();
    fn SetMultipleInstances();
    fn StopIfGoingOnBatteries();
    fn SetStopIfGoingOnBatteries();
    fn DisallowStartIfOnBatteries();
    fn SetDisallowStartIfOnBatteries();
    fn AllowHardTerminate();
    fn SetAllowHardTerminate();
    fn StartWhenAvailable();
    fn SetStartWhenAvailable();
    fn XmlText();
    fn SetXmlText();
    fn RunOnlyIfNetworkAvailable();
    fn SetRunOnlyIfNetworkAvailable();
    fn ExecutionTimeLimit();
    fn SetExecutionTimeLimit();
    fn Enabled();
    fn SetEnabled();
    fn DeleteExpiredTaskAfter();
    fn SetDeleteExpiredTaskAfter();
    fn Priority();
    fn SetPriority();
    fn Compatibility();
    fn SetCompatibility();
    fn Hidden();
    fn SetHidden();
    fn IdleSettings();
    fn SetIdleSettings();
    fn RunOnlyIfIdle();
    fn SetRunOnlyIfIdle();
    fn WakeToRun();
    fn SetWakeToRun();
    fn NetworkSettings();
    fn SetNetworkSettings();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettings2Impl: Sized + IDispatchImpl {
    fn DisallowStartOnRemoteAppSession();
    fn SetDisallowStartOnRemoteAppSession();
    fn UseUnifiedSchedulingEngine();
    fn SetUseUnifiedSchedulingEngine();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITaskSettings3Impl: Sized + ITaskSettingsImpl + IDispatchImpl {
    fn DisallowStartOnRemoteAppSession();
    fn SetDisallowStartOnRemoteAppSession();
    fn UseUnifiedSchedulingEngine();
    fn SetUseUnifiedSchedulingEngine();
    fn MaintenanceSettings();
    fn SetMaintenanceSettings();
    fn CreateMaintenanceSettings();
    fn Volatile();
    fn SetVolatile();
}
pub trait ITaskTriggerImpl: Sized {
    fn SetTrigger();
    fn GetTrigger();
    fn GetTriggerString();
}
pub trait ITaskVariablesImpl: Sized {
    fn GetInput();
    fn SetOutput();
    fn GetContext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITimeTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITriggerImpl: Sized + IDispatchImpl {
    fn Type();
    fn Id();
    fn SetId();
    fn Repetition();
    fn SetRepetition();
    fn ExecutionTimeLimit();
    fn SetExecutionTimeLimit();
    fn StartBoundary();
    fn SetStartBoundary();
    fn EndBoundary();
    fn SetEndBoundary();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITriggerCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Create();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWeeklyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfWeek();
    fn SetDaysOfWeek();
    fn WeeksInterval();
    fn SetWeeksInterval();
    fn RandomDelay();
    fn SetRandomDelay();
}
