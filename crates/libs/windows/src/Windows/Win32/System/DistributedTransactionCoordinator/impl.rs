pub trait IDtcLuConfigureImpl: Sized {
    fn Add();
    fn Delete();
}
pub trait IDtcLuRecoveryImpl: Sized {}
pub trait IDtcLuRecoveryFactoryImpl: Sized {
    fn Create();
}
pub trait IDtcLuRecoveryInitiatedByDtcImpl: Sized {
    fn GetWork();
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWorkImpl: Sized {
    fn HandleCheckLuStatus();
}
pub trait IDtcLuRecoveryInitiatedByDtcTransWorkImpl: Sized {
    fn GetLogNameSizes();
    fn GetOurXln();
    fn HandleConfirmationFromOurXln();
    fn HandleTheirXlnResponse();
    fn HandleErrorFromOurXln();
    fn CheckForCompareStates();
    fn GetOurTransIdSize();
    fn GetOurCompareStates();
    fn HandleTheirCompareStatesResponse();
    fn HandleErrorFromOurCompareStates();
    fn ConversationLost();
    fn GetRecoverySeqNum();
    fn ObsoleteRecoverySeqNum();
}
pub trait IDtcLuRecoveryInitiatedByLuImpl: Sized {
    fn GetObjectToHandleWorkFromLu();
}
pub trait IDtcLuRecoveryInitiatedByLuWorkImpl: Sized {
    fn HandleTheirXln();
    fn GetOurLogNameSize();
    fn GetOurXln();
    fn HandleConfirmationOfOurXln();
    fn HandleTheirCompareStates();
    fn HandleConfirmationOfOurCompareStates();
    fn HandleErrorFromOurCompareStates();
    fn ConversationLost();
}
pub trait IDtcLuRmEnlistmentImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn RequestCommit();
}
pub trait IDtcLuRmEnlistmentFactoryImpl: Sized {
    fn Create();
}
pub trait IDtcLuRmEnlistmentSinkImpl: Sized {
    fn AckUnplug();
    fn TmDown();
    fn SessionLost();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn Prepare();
    fn RequestCommit();
}
pub trait IDtcLuSubordinateDtcImpl: Sized {
    fn Unplug();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn Prepare();
    fn RequestCommit();
}
pub trait IDtcLuSubordinateDtcFactoryImpl: Sized {
    fn Create();
}
pub trait IDtcLuSubordinateDtcSinkImpl: Sized {
    fn AckUnplug();
    fn TmDown();
    fn SessionLost();
    fn BackedOut();
    fn BackOut();
    fn Committed();
    fn Forget();
    fn RequestCommit();
}
pub trait IDtcNetworkAccessConfigImpl: Sized {
    fn GetAnyNetworkAccess();
    fn SetAnyNetworkAccess();
    fn GetNetworkAdministrationAccess();
    fn SetNetworkAdministrationAccess();
    fn GetNetworkTransactionAccess();
    fn SetNetworkTransactionAccess();
    fn GetNetworkClientAccess();
    fn SetNetworkClientAccess();
    fn GetNetworkTIPAccess();
    fn SetNetworkTIPAccess();
    fn GetXAAccess();
    fn SetXAAccess();
    fn RestartDtcService();
}
pub trait IDtcNetworkAccessConfig2Impl: Sized + IDtcNetworkAccessConfigImpl {
    fn GetNetworkInboundAccess();
    fn GetNetworkOutboundAccess();
    fn SetNetworkInboundAccess();
    fn SetNetworkOutboundAccess();
    fn GetAuthenticationLevel();
    fn SetAuthenticationLevel();
}
pub trait IDtcNetworkAccessConfig3Impl: Sized + IDtcNetworkAccessConfig2Impl + IDtcNetworkAccessConfigImpl {
    fn GetLUAccess();
    fn SetLUAccess();
}
pub trait IDtcToXaHelperImpl: Sized {
    fn Close();
    fn TranslateTridToXid();
}
pub trait IDtcToXaHelperFactoryImpl: Sized {
    fn Create();
}
pub trait IDtcToXaHelperSinglePipeImpl: Sized {
    fn XARMCreate();
    fn ConvertTridToXID();
    fn EnlistWithRM();
    fn ReleaseRMCookie();
}
pub trait IDtcToXaMapperImpl: Sized {
    fn RequestNewResourceManager();
    fn TranslateTridToXid();
    fn EnlistResourceManager();
    fn ReleaseResourceManager();
}
pub trait IGetDispenserImpl: Sized {
    fn GetDispenser();
}
pub trait IKernelTransactionImpl: Sized {
    fn GetHandle();
}
pub trait ILastResourceManagerImpl: Sized {
    fn TransactionCommitted();
    fn RecoveryDone();
}
pub trait IPrepareInfoImpl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
pub trait IPrepareInfo2Impl: Sized {
    fn GetPrepareInfoSize();
    fn GetPrepareInfo();
}
pub trait IRMHelperImpl: Sized {
    fn RMCount();
    fn RMInfo();
}
pub trait IResourceManagerImpl: Sized {
    fn Enlist();
    fn Reenlist();
    fn ReenlistmentComplete();
    fn GetDistributedTransactionManager();
}
pub trait IResourceManager2Impl: Sized + IResourceManagerImpl {
    fn Enlist2();
    fn Reenlist2();
}
pub trait IResourceManagerFactoryImpl: Sized {
    fn Create();
}
pub trait IResourceManagerFactory2Impl: Sized + IResourceManagerFactoryImpl {
    fn CreateEx();
}
pub trait IResourceManagerRejoinableImpl: Sized + IResourceManager2Impl + IResourceManagerImpl {
    fn Rejoin();
}
pub trait IResourceManagerSinkImpl: Sized {
    fn TMDown();
}
pub trait ITipHelperImpl: Sized {
    fn Pull();
    fn PullAsync();
    fn GetLocalTmUrl();
}
pub trait ITipPullSinkImpl: Sized {
    fn PullComplete();
}
pub trait ITipTransactionImpl: Sized {
    fn Push();
    fn GetTransactionUrl();
}
pub trait ITmNodeNameImpl: Sized {
    fn GetNodeNameSize();
    fn GetNodeName();
}
pub trait ITransactionImpl: Sized {
    fn Commit();
    fn Abort();
    fn GetTransactionInfo();
}
pub trait ITransaction2Impl: Sized + ITransactionClonerImpl + ITransactionImpl {
    fn GetTransactionInfo2();
}
pub trait ITransactionClonerImpl: Sized + ITransactionImpl {
    fn CloneWithCommitDisabled();
}
pub trait ITransactionDispenserImpl: Sized {
    fn GetOptionsObject();
    fn BeginTransaction();
}
pub trait ITransactionEnlistmentAsyncImpl: Sized {
    fn PrepareRequestDone();
    fn CommitRequestDone();
    fn AbortRequestDone();
}
pub trait ITransactionExportImpl: Sized {
    fn Export();
    fn GetTransactionCookie();
}
pub trait ITransactionExportFactoryImpl: Sized {
    fn GetRemoteClassId();
    fn Create();
}
pub trait ITransactionImportImpl: Sized {
    fn Import();
}
pub trait ITransactionImportWhereaboutsImpl: Sized {
    fn GetWhereaboutsSize();
    fn GetWhereabouts();
}
pub trait ITransactionLastEnlistmentAsyncImpl: Sized {
    fn TransactionOutcome();
}
pub trait ITransactionLastResourceAsyncImpl: Sized {
    fn DelegateCommit();
    fn ForgetRequest();
}
pub trait ITransactionOptionsImpl: Sized {
    fn SetOptions();
    fn GetOptions();
}
pub trait ITransactionOutcomeEventsImpl: Sized {
    fn Committed();
    fn Aborted();
    fn HeuristicDecision();
    fn Indoubt();
}
pub trait ITransactionPhase0EnlistmentAsyncImpl: Sized {
    fn Enable();
    fn WaitForEnlistment();
    fn Phase0Done();
    fn Unenlist();
    fn GetTransaction();
}
pub trait ITransactionPhase0FactoryImpl: Sized {
    fn Create();
}
pub trait ITransactionPhase0NotifyAsyncImpl: Sized {
    fn Phase0Request();
    fn EnlistCompleted();
}
pub trait ITransactionReceiverImpl: Sized {
    fn UnmarshalPropagationToken();
    fn GetReturnTokenSize();
    fn MarshalReturnToken();
    fn Reset();
}
pub trait ITransactionReceiverFactoryImpl: Sized {
    fn Create();
}
pub trait ITransactionResourceImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
pub trait ITransactionResourceAsyncImpl: Sized {
    fn PrepareRequest();
    fn CommitRequest();
    fn AbortRequest();
    fn TMDown();
}
pub trait ITransactionTransmitterImpl: Sized {
    fn Set();
    fn GetPropagationTokenSize();
    fn MarshalPropagationToken();
    fn UnmarshalReturnToken();
    fn Reset();
}
pub trait ITransactionTransmitterFactoryImpl: Sized {
    fn Create();
}
pub trait ITransactionVoterBallotAsync2Impl: Sized {
    fn VoteRequestDone();
}
pub trait ITransactionVoterFactory2Impl: Sized {
    fn Create();
}
pub trait ITransactionVoterNotifyAsync2Impl: Sized + ITransactionOutcomeEventsImpl {
    fn VoteRequest();
}
pub trait IXAConfigImpl: Sized {
    fn Initialize();
    fn Terminate();
}
pub trait IXAObtainRMInfoImpl: Sized {
    fn ObtainRMInfo();
}
pub trait IXATransLookupImpl: Sized {
    fn Lookup();
}
pub trait IXATransLookup2Impl: Sized {
    fn Lookup();
}
