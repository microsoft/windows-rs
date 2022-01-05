#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfoImpl: Sized + IDispatchImpl {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfo2Impl: Sized + ContextInfoImpl + IDispatchImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppDomainHelperImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn DoCallback();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyLocatorImpl: Sized + IDispatchImpl {
    fn GetModules();
}
pub trait IAsyncErrorNotifyImpl: Sized {
    fn OnError();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalogImpl: Sized + IDispatchImpl {
    fn GetCollection();
    fn Connect();
    fn MajorVersion();
    fn MinorVersion();
    fn GetCollectionByQuery();
    fn ImportComponent();
    fn InstallComponent();
    fn ShutdownApplication();
    fn ExportApplication();
    fn InstallApplication();
    fn StopRouter();
    fn RefreshRouter();
    fn StartRouter();
    fn Reserved1();
    fn Reserved2();
    fn InstallMultipleComponents();
    fn GetMultipleComponentsInfo();
    fn RefreshComponents();
    fn BackupREGDB();
    fn RestoreREGDB();
    fn QueryApplicationFile();
    fn StartApplication();
    fn ServiceCheck();
    fn InstallMultipleEventClasses();
    fn InstallEventClass();
    fn GetEventClassesForIID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalog2Impl: Sized + ICOMAdminCatalogImpl + IDispatchImpl {
    fn GetCollectionByQuery2();
    fn GetApplicationInstanceIDFromProcessID();
    fn ShutdownApplicationInstances();
    fn PauseApplicationInstances();
    fn ResumeApplicationInstances();
    fn RecycleApplicationInstances();
    fn AreApplicationInstancesPaused();
    fn DumpApplicationInstance();
    fn IsApplicationInstanceDumpSupported();
    fn CreateServiceForApplication();
    fn DeleteServiceForApplication();
    fn GetPartitionID();
    fn GetPartitionName();
    fn SetCurrentPartition();
    fn CurrentPartitionID();
    fn CurrentPartitionName();
    fn GlobalPartitionID();
    fn FlushPartitionCache();
    fn CopyApplications();
    fn CopyComponents();
    fn MoveComponents();
    fn AliasComponent();
    fn IsSafeToDelete();
    fn ImportUnconfiguredComponents();
    fn PromoteUnconfiguredComponents();
    fn ImportComponents();
    fn Is64BitCatalogServer();
    fn ExportPartition();
    fn InstallPartition();
    fn QueryApplicationFile2();
    fn GetComponentVersionCount();
}
pub trait ICOMLBArgumentsImpl: Sized {
    fn GetCLSID();
    fn SetCLSID();
    fn GetMachineName();
    fn SetMachineName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
    fn Populate();
    fn SaveChanges();
    fn GetCollection();
    fn Name();
    fn AddEnabled();
    fn RemoveEnabled();
    fn GetUtilInterface();
    fn DataStoreMajorVersion();
    fn DataStoreMinorVersion();
    fn PopulateByKey();
    fn PopulateByQuery();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogObjectImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Key();
    fn Name();
    fn IsPropertyReadOnly();
    fn Valid();
    fn IsPropertyWriteOnly();
}
pub trait ICheckSxsConfigImpl: Sized {
    fn IsSameSxsConfig();
}
pub trait IComActivityEventsImpl: Sized {
    fn OnActivityCreate();
    fn OnActivityDestroy();
    fn OnActivityEnter();
    fn OnActivityTimeout();
    fn OnActivityReenter();
    fn OnActivityLeave();
    fn OnActivityLeaveSame();
}
pub trait IComApp2EventsImpl: Sized {
    fn OnAppActivation2();
    fn OnAppShutdown2();
    fn OnAppForceShutdown2();
    fn OnAppPaused2();
    fn OnAppRecycle2();
}
pub trait IComAppEventsImpl: Sized {
    fn OnAppActivation();
    fn OnAppShutdown();
    fn OnAppForceShutdown();
}
pub trait IComCRMEventsImpl: Sized {
    fn OnCRMRecoveryStart();
    fn OnCRMRecoveryDone();
    fn OnCRMCheckpoint();
    fn OnCRMBegin();
    fn OnCRMPrepare();
    fn OnCRMCommit();
    fn OnCRMAbort();
    fn OnCRMIndoubt();
    fn OnCRMDone();
    fn OnCRMRelease();
    fn OnCRMAnalyze();
    fn OnCRMWrite();
    fn OnCRMForget();
    fn OnCRMForce();
    fn OnCRMDeliver();
}
pub trait IComExceptionEventsImpl: Sized {
    fn OnExceptionUser();
}
pub trait IComIdentityEventsImpl: Sized {
    fn OnIISRequestInfo();
}
pub trait IComInstance2EventsImpl: Sized {
    fn OnObjectCreate2();
    fn OnObjectDestroy2();
}
pub trait IComInstanceEventsImpl: Sized {
    fn OnObjectCreate();
    fn OnObjectDestroy();
}
pub trait IComLTxEventsImpl: Sized {
    fn OnLtxTransactionStart();
    fn OnLtxTransactionPrepare();
    fn OnLtxTransactionAbort();
    fn OnLtxTransactionCommit();
    fn OnLtxTransactionPromote();
}
pub trait IComMethod2EventsImpl: Sized {
    fn OnMethodCall2();
    fn OnMethodReturn2();
    fn OnMethodException2();
}
pub trait IComMethodEventsImpl: Sized {
    fn OnMethodCall();
    fn OnMethodReturn();
    fn OnMethodException();
}
pub trait IComMtaThreadPoolKnobsImpl: Sized {
    fn MTASetMaxThreadCount();
    fn MTAGetMaxThreadCount();
    fn MTASetThrottleValue();
    fn MTAGetThrottleValue();
}
pub trait IComObjectConstruction2EventsImpl: Sized {
    fn OnObjectConstruct2();
}
pub trait IComObjectConstructionEventsImpl: Sized {
    fn OnObjectConstruct();
}
pub trait IComObjectEventsImpl: Sized {
    fn OnObjectActivate();
    fn OnObjectDeactivate();
    fn OnDisableCommit();
    fn OnEnableCommit();
    fn OnSetComplete();
    fn OnSetAbort();
}
pub trait IComObjectPool2EventsImpl: Sized {
    fn OnObjPoolPutObject2();
    fn OnObjPoolGetObject2();
    fn OnObjPoolRecycleToTx2();
    fn OnObjPoolGetFromTx2();
}
pub trait IComObjectPoolEventsImpl: Sized {
    fn OnObjPoolPutObject();
    fn OnObjPoolGetObject();
    fn OnObjPoolRecycleToTx();
    fn OnObjPoolGetFromTx();
}
pub trait IComObjectPoolEvents2Impl: Sized {
    fn OnObjPoolCreateObject();
    fn OnObjPoolDestroyObject();
    fn OnObjPoolCreateDecision();
    fn OnObjPoolTimeout();
    fn OnObjPoolCreatePool();
}
pub trait IComQCEventsImpl: Sized {
    fn OnQCRecord();
    fn OnQCQueueOpen();
    fn OnQCReceive();
    fn OnQCReceiveFail();
    fn OnQCMoveToReTryQueue();
    fn OnQCMoveToDeadQueue();
    fn OnQCPlayback();
}
pub trait IComResourceEventsImpl: Sized {
    fn OnResourceCreate();
    fn OnResourceAllocate();
    fn OnResourceRecycle();
    fn OnResourceDestroy();
    fn OnResourceTrack();
}
pub trait IComSecurityEventsImpl: Sized {
    fn OnAuthenticate();
    fn OnAuthenticateFail();
}
pub trait IComStaThreadPoolKnobsImpl: Sized {
    fn SetMinThreadCount();
    fn GetMinThreadCount();
    fn SetMaxThreadCount();
    fn GetMaxThreadCount();
    fn SetActivityPerThread();
    fn GetActivityPerThread();
    fn SetActivityRatio();
    fn GetActivityRatio();
    fn GetThreadCount();
    fn GetQueueDepth();
    fn SetQueueDepth();
}
pub trait IComStaThreadPoolKnobs2Impl: Sized + IComStaThreadPoolKnobsImpl {
    fn GetMaxCPULoad();
    fn SetMaxCPULoad();
    fn GetCPUMetricEnabled();
    fn SetCPUMetricEnabled();
    fn GetCreateThreadsAggressively();
    fn SetCreateThreadsAggressively();
    fn GetMaxCSR();
    fn SetMaxCSR();
    fn GetWaitTimeForThreadCleanup();
    fn SetWaitTimeForThreadCleanup();
}
pub trait IComThreadEventsImpl: Sized {
    fn OnThreadStart();
    fn OnThreadTerminate();
    fn OnThreadBindToApartment();
    fn OnThreadUnBind();
    fn OnThreadWorkEnque();
    fn OnThreadWorkPrivate();
    fn OnThreadWorkPublic();
    fn OnThreadWorkRedirect();
    fn OnThreadWorkReject();
    fn OnThreadAssignApartment();
    fn OnThreadUnassignApartment();
}
pub trait IComTrackingInfoCollectionImpl: Sized {
    fn Type();
    fn Count();
    fn Item();
}
pub trait IComTrackingInfoEventsImpl: Sized {
    fn OnNewTrackingInfo();
}
pub trait IComTrackingInfoObjectImpl: Sized {
    fn GetValue();
}
pub trait IComTrackingInfoPropertiesImpl: Sized {
    fn PropCount();
    fn GetPropName();
}
pub trait IComTransaction2EventsImpl: Sized {
    fn OnTransactionStart2();
    fn OnTransactionPrepare2();
    fn OnTransactionAbort2();
    fn OnTransactionCommit2();
}
pub trait IComTransactionEventsImpl: Sized {
    fn OnTransactionStart();
    fn OnTransactionPrepare();
    fn OnTransactionAbort();
    fn OnTransactionCommit();
}
pub trait IComUserEventImpl: Sized {
    fn OnUserEvent();
}
pub trait IContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
    fn SetProperty();
    fn RemoveProperty();
}
pub trait IContextSecurityPerimeterImpl: Sized {
    fn GetPerimeterFlag();
    fn SetPerimeterFlag();
}
pub trait IContextStateImpl: Sized {
    fn SetDeactivateOnReturn();
    fn GetDeactivateOnReturn();
    fn SetMyTransactionVote();
    fn GetMyTransactionVote();
}
pub trait ICreateWithLocalTransactionImpl: Sized {
    fn CreateInstanceWithSysTx();
}
pub trait ICreateWithTipTransactionExImpl: Sized {
    fn CreateInstance();
}
pub trait ICreateWithTransactionExImpl: Sized {
    fn CreateInstance();
}
pub trait ICrmCompensatorImpl: Sized {
    fn SetLogControl();
    fn BeginPrepare();
    fn PrepareRecord();
    fn EndPrepare();
    fn BeginCommit();
    fn CommitRecord();
    fn EndCommit();
    fn BeginAbort();
    fn AbortRecord();
    fn EndAbort();
}
pub trait ICrmCompensatorVariantsImpl: Sized {
    fn SetLogControlVariants();
    fn BeginPrepareVariants();
    fn PrepareRecordVariants();
    fn EndPrepareVariants();
    fn BeginCommitVariants();
    fn CommitRecordVariants();
    fn EndCommitVariants();
    fn BeginAbortVariants();
    fn AbortRecordVariants();
    fn EndAbortVariants();
}
pub trait ICrmFormatLogRecordsImpl: Sized {
    fn GetColumnCount();
    fn GetColumnHeaders();
    fn GetColumn();
    fn GetColumnVariants();
}
pub trait ICrmLogControlImpl: Sized {
    fn TransactionUOW();
    fn RegisterCompensator();
    fn WriteLogRecordVariants();
    fn ForceLog();
    fn ForgetLogRecord();
    fn ForceTransactionToAbort();
    fn WriteLogRecord();
}
pub trait ICrmMonitorImpl: Sized {
    fn GetClerks();
    fn HoldClerk();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmMonitorClerksImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
    fn ProgIdCompensator();
    fn Description();
    fn TransactionUOW();
    fn ActivityId();
}
pub trait ICrmMonitorLogRecordsImpl: Sized {
    fn Count();
    fn TransactionState();
    fn StructuredRecords();
    fn GetLogRecord();
    fn GetLogRecordVariants();
}
pub trait IDispenserDriverImpl: Sized {
    fn CreateResource();
    fn RateResource();
    fn EnlistResource();
    fn ResetResource();
    fn DestroyResource();
    fn DestroyResourceS();
}
pub trait IDispenserManagerImpl: Sized {
    fn RegisterDispenser();
    fn GetContext();
}
pub trait IEnumNamesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEventServerTraceImpl: Sized + IDispatchImpl {
    fn StartTraceGuid();
    fn StopTraceGuid();
    fn EnumTraceGuid();
}
pub trait IGetAppTrackerDataImpl: Sized {
    fn GetApplicationProcesses();
    fn GetApplicationProcessDetails();
    fn GetApplicationsInProcess();
    fn GetComponentsInProcess();
    fn GetComponentDetails();
    fn GetTrackerDataAsCollectionObject();
    fn GetSuggestedPollingInterval();
}
pub trait IGetContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGetSecurityCallContextImpl: Sized + IDispatchImpl {
    fn GetSecurityCallContext();
}
pub trait IHolderImpl: Sized {
    fn AllocResource();
    fn FreeResource();
    fn TrackResource();
    fn TrackResourceS();
    fn UntrackResource();
    fn UntrackResourceS();
    fn Close();
    fn RequestDestroyResource();
}
pub trait ILBEventsImpl: Sized {
    fn TargetUp();
    fn TargetDown();
    fn EngineDefined();
}
pub trait IMTSActivityImpl: Sized {
    fn SynchronousCall();
    fn AsyncCall();
    fn Reserved1();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
pub trait IMTSCallImpl: Sized {
    fn OnCall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMTSLocatorImpl: Sized + IDispatchImpl {
    fn GetEventDispatcher();
}
pub trait IManagedActivationEventsImpl: Sized {
    fn CreateManagedStub();
    fn DestroyManagedStub();
}
pub trait IManagedObjectInfoImpl: Sized {
    fn GetIUnknown();
    fn GetIObjectControl();
    fn SetInPool();
    fn SetWrapperStrength();
}
pub trait IManagedPoolActionImpl: Sized {
    fn LastRelease();
}
pub trait IManagedPooledObjImpl: Sized {
    fn SetHeld();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageMoverImpl: Sized + IDispatchImpl {
    fn SourcePath();
    fn SetSourcePath();
    fn DestPath();
    fn SetDestPath();
    fn CommitBatchSize();
    fn SetCommitBatchSize();
    fn MoveMessages();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEventInfoImpl: Sized + IDispatchImpl {
    fn Names();
    fn DisplayName();
    fn EventID();
    fn Count();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEventsImpl: Sized + IDispatchImpl {
    fn PackageName();
    fn PackageGuid();
    fn PostEvent();
    fn FireEvents();
    fn GetProcessID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsGrpImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn Refresh();
}
pub trait IObjPoolImpl: Sized {
    fn Reserved1();
    fn Reserved2();
    fn Reserved3();
    fn Reserved4();
    fn PutEndTx();
    fn Reserved5();
    fn Reserved6();
}
pub trait IObjectConstructImpl: Sized {
    fn Construct();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstructStringImpl: Sized + IDispatchImpl {
    fn ConstructString();
}
pub trait IObjectContextImpl: Sized {
    fn CreateInstance();
    fn SetComplete();
    fn SetAbort();
    fn EnableCommit();
    fn DisableCommit();
    fn IsInTransaction();
    fn IsSecurityEnabled();
    fn IsCallerInRole();
}
pub trait IObjectContextActivityImpl: Sized {
    fn GetActivityId();
}
pub trait IObjectContextInfoImpl: Sized {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
pub trait IObjectContextInfo2Impl: Sized + IObjectContextInfoImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
pub trait IObjectContextTipImpl: Sized {
    fn GetTipUrl();
}
pub trait IObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
pub trait IPlaybackControlImpl: Sized {
    fn FinalClientRetry();
    fn FinalServerRetry();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPoolManagerImpl: Sized + IDispatchImpl {
    fn ShutdownPool();
}
pub trait IProcessInitializerImpl: Sized {
    fn Startup();
    fn Shutdown();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallContextImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn IsCallerInRole();
    fn IsSecurityEnabled();
    fn IsUserInRole();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallersCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityIdentityCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
pub trait ISecurityPropertyImpl: Sized {
    fn GetDirectCreatorSID();
    fn GetOriginalCreatorSID();
    fn GetDirectCallerSID();
    fn GetOriginalCallerSID();
    fn ReleaseSID();
}
pub trait ISelectCOMLBServerImpl: Sized {
    fn Init();
    fn GetLBServer();
}
pub trait ISendMethodEventsImpl: Sized {
    fn SendMethodCall();
    fn SendMethodReturn();
}
pub trait IServiceActivityImpl: Sized {
    fn SynchronousCall();
    fn AsynchronousCall();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
pub trait IServiceCallImpl: Sized {
    fn OnCall();
}
pub trait IServiceComTIIntrinsicsConfigImpl: Sized {
    fn ComTIIntrinsicsConfig();
}
pub trait IServiceIISIntrinsicsConfigImpl: Sized {
    fn IISIntrinsicsConfig();
}
pub trait IServiceInheritanceConfigImpl: Sized {
    fn ContainingContextTreatment();
}
pub trait IServicePartitionConfigImpl: Sized {
    fn PartitionConfig();
    fn PartitionID();
}
pub trait IServicePoolImpl: Sized {
    fn Initialize();
    fn GetObject();
    fn Shutdown();
}
pub trait IServicePoolConfigImpl: Sized {
    fn SetMaxPoolSize();
    fn MaxPoolSize();
    fn SetMinPoolSize();
    fn MinPoolSize();
    fn SetCreationTimeout();
    fn CreationTimeout();
    fn SetTransactionAffinity();
    fn TransactionAffinity();
    fn SetClassFactory();
    fn ClassFactory();
}
pub trait IServiceSxsConfigImpl: Sized {
    fn SxsConfig();
    fn SxsName();
    fn SxsDirectory();
}
pub trait IServiceSynchronizationConfigImpl: Sized {
    fn ConfigureSynchronization();
}
pub trait IServiceSysTxnConfigImpl: Sized + IServiceTransactionConfigImpl + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOTSysTxn();
}
pub trait IServiceThreadPoolConfigImpl: Sized {
    fn SelectThreadPool();
    fn SetBindingInfo();
}
pub trait IServiceTrackerConfigImpl: Sized {
    fn TrackerConfig();
}
pub trait IServiceTransactionConfigImpl: Sized + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOT();
}
pub trait IServiceTransactionConfigBaseImpl: Sized {
    fn ConfigureTransaction();
    fn IsolationLevel();
    fn TransactionTimeout();
    fn BringYourOwnTransaction();
    fn NewTransactionDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroupImpl: Sized + IDispatchImpl {
    fn CreatePropertyByPosition();
    fn PropertyByPosition();
    fn CreateProperty();
    fn Property();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroupManagerImpl: Sized + IDispatchImpl {
    fn CreatePropertyGroup();
    fn Group();
    fn _NewEnum();
}
pub trait ISystemAppEventDataImpl: Sized {
    fn Startup();
    fn OnDataChanged();
}
pub trait IThreadPoolKnobsImpl: Sized {
    fn GetMaxThreads();
    fn GetCurrentThreads();
    fn SetMaxThreads();
    fn GetDeleteDelay();
    fn SetDeleteDelay();
    fn GetMaxQueuedRequests();
    fn GetCurrentQueuedRequests();
    fn SetMaxQueuedRequests();
    fn SetMinThreads();
    fn SetQueueDepth();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionContextImpl: Sized + IDispatchImpl {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
pub trait ITransactionContextExImpl: Sized {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
pub trait ITransactionPropertyImpl: Sized {
    fn Reserved1();
    fn Reserved2();
    fn Reserved3();
    fn Reserved4();
    fn Reserved5();
    fn Reserved6();
    fn Reserved7();
    fn Reserved8();
    fn Reserved9();
    fn GetTransactionResourcePool();
    fn Reserved10();
    fn Reserved11();
    fn Reserved12();
    fn Reserved13();
    fn Reserved14();
    fn Reserved15();
    fn Reserved16();
    fn Reserved17();
}
pub trait ITransactionProxyImpl: Sized {
    fn Commit();
    fn Abort();
    fn Promote();
    fn CreateVoter();
    fn GetIsolationLevel();
    fn GetIdentifier();
    fn IsReusable();
}
pub trait ITransactionResourcePoolImpl: Sized {
    fn PutResource();
    fn GetResource();
}
pub trait ITransactionStatusImpl: Sized {
    fn SetTransactionStatus();
    fn GetTransactionStatus();
}
pub trait ITxProxyHolderImpl: Sized {
    fn GetIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ObjectContextImpl: Sized + IDispatchImpl {
    fn CreateInstance();
    fn SetComplete();
    fn SetAbort();
    fn EnableCommit();
    fn DisableCommit();
    fn IsInTransaction();
    fn IsSecurityEnabled();
    fn IsCallerInRole();
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Security();
    fn ContextInfo();
}
pub trait ObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait SecurityPropertyImpl: Sized + IDispatchImpl {
    fn GetDirectCallerName();
    fn GetDirectCreatorName();
    fn GetOriginalCallerName();
    fn GetOriginalCreatorName();
}
