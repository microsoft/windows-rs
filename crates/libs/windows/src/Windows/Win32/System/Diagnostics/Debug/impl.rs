pub trait AsyncIDebugApplicationNodeEventsImpl: Sized {
    fn Begin_onAddChild();
    fn Finish_onAddChild();
    fn Begin_onRemoveChild();
    fn Finish_onRemoveChild();
    fn Begin_onDetach();
    fn Finish_onDetach();
    fn Begin_onAttach();
    fn Finish_onAttach();
}
pub trait DebugBaseEventCallbacksImpl: Sized + IDebugEventCallbacksImpl {}
pub trait DebugBaseEventCallbacksWideImpl: Sized + IDebugEventCallbacksWideImpl {}
pub trait IActiveScriptImpl: Sized {
    fn SetScriptSite();
    fn GetScriptSite();
    fn SetScriptState();
    fn GetScriptState();
    fn Close();
    fn AddNamedItem();
    fn AddTypeLib();
    fn GetScriptDispatch();
    fn GetCurrentScriptThreadID();
    fn GetScriptThreadID();
    fn GetScriptThreadState();
    fn InterruptScriptThread();
    fn Clone();
}
pub trait IActiveScriptAuthorImpl: Sized {
    fn AddNamedItem();
    fn AddScriptlet();
    fn ParseScriptText();
    fn GetScriptTextAttributes();
    fn GetScriptletTextAttributes();
    fn GetRoot();
    fn GetLanguageFlags();
    fn GetEventHandler();
    fn RemoveNamedItem();
    fn AddTypeLib();
    fn RemoveTypeLib();
    fn GetChars();
    fn GetInfoFromContext();
    fn IsCommitChar();
}
pub trait IActiveScriptAuthorProcedureImpl: Sized {
    fn ParseProcedureText();
}
pub trait IActiveScriptDebug32Impl: Sized {
    fn GetScriptTextAttributes();
    fn GetScriptletTextAttributes();
    fn EnumCodeContextsOfPosition();
}
pub trait IActiveScriptDebug64Impl: Sized {
    fn GetScriptTextAttributes();
    fn GetScriptletTextAttributes();
    fn EnumCodeContextsOfPosition();
}
pub trait IActiveScriptEncodeImpl: Sized {
    fn EncodeSection();
    fn DecodeScript();
    fn GetEncodeProgId();
}
pub trait IActiveScriptErrorImpl: Sized {
    fn GetExceptionInfo();
    fn GetSourcePosition();
    fn GetSourceLineText();
}
pub trait IActiveScriptError64Impl: Sized + IActiveScriptErrorImpl {
    fn GetSourcePosition64();
}
pub trait IActiveScriptErrorDebugImpl: Sized + IActiveScriptErrorImpl {
    fn GetDocumentContext();
    fn GetStackFrame();
}
pub trait IActiveScriptErrorDebug110Impl: Sized {
    fn GetExceptionThrownKind();
}
pub trait IActiveScriptGarbageCollectorImpl: Sized {
    fn CollectGarbage();
}
pub trait IActiveScriptHostEncodeImpl: Sized {
    fn EncodeScriptHostFile();
}
pub trait IActiveScriptParse32Impl: Sized {
    fn InitNew();
    fn AddScriptlet();
    fn ParseScriptText();
}
pub trait IActiveScriptParse64Impl: Sized {
    fn InitNew();
    fn AddScriptlet();
    fn ParseScriptText();
}
pub trait IActiveScriptParseProcedure2_32Impl: Sized + IActiveScriptParseProcedure32Impl {}
pub trait IActiveScriptParseProcedure2_64Impl: Sized + IActiveScriptParseProcedure64Impl {}
pub trait IActiveScriptParseProcedure32Impl: Sized {
    fn ParseProcedureText();
}
pub trait IActiveScriptParseProcedure64Impl: Sized {
    fn ParseProcedureText();
}
pub trait IActiveScriptParseProcedureOld32Impl: Sized {
    fn ParseProcedureText();
}
pub trait IActiveScriptParseProcedureOld64Impl: Sized {
    fn ParseProcedureText();
}
pub trait IActiveScriptProfilerCallbackImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn ScriptCompiled();
    fn FunctionCompiled();
    fn OnFunctionEnter();
    fn OnFunctionExit();
}
pub trait IActiveScriptProfilerCallback2Impl: Sized + IActiveScriptProfilerCallbackImpl {
    fn OnFunctionEnterByName();
    fn OnFunctionExitByName();
}
pub trait IActiveScriptProfilerCallback3Impl: Sized + IActiveScriptProfilerCallback2Impl + IActiveScriptProfilerCallbackImpl {
    fn SetWebWorkerId();
}
pub trait IActiveScriptProfilerControlImpl: Sized {
    fn StartProfiling();
    fn SetProfilerEventMask();
    fn StopProfiling();
}
pub trait IActiveScriptProfilerControl2Impl: Sized + IActiveScriptProfilerControlImpl {
    fn CompleteProfilerStart();
    fn PrepareProfilerStop();
}
pub trait IActiveScriptProfilerControl3Impl: Sized + IActiveScriptProfilerControl2Impl + IActiveScriptProfilerControlImpl {
    fn EnumHeap();
}
pub trait IActiveScriptProfilerControl4Impl: Sized + IActiveScriptProfilerControl3Impl + IActiveScriptProfilerControl2Impl + IActiveScriptProfilerControlImpl {
    fn SummarizeHeap();
}
pub trait IActiveScriptProfilerControl5Impl: Sized + IActiveScriptProfilerControl4Impl + IActiveScriptProfilerControl3Impl + IActiveScriptProfilerControl2Impl + IActiveScriptProfilerControlImpl {
    fn EnumHeap2();
}
pub trait IActiveScriptProfilerHeapEnumImpl: Sized {
    fn Next();
    fn GetOptionalInfo();
    fn FreeObjectAndOptionalInfo();
    fn GetNameIdMap();
}
pub trait IActiveScriptPropertyImpl: Sized {
    fn GetProperty();
    fn SetProperty();
}
pub trait IActiveScriptSIPInfoImpl: Sized {
    fn GetSIPOID();
}
pub trait IActiveScriptSiteImpl: Sized {
    fn GetLCID();
    fn GetItemInfo();
    fn GetDocVersionString();
    fn OnScriptTerminate();
    fn OnStateChange();
    fn OnScriptError();
    fn OnEnterScript();
    fn OnLeaveScript();
}
pub trait IActiveScriptSiteDebug32Impl: Sized {
    fn GetDocumentContextFromPosition();
    fn GetApplication();
    fn GetRootApplicationNode();
    fn OnScriptErrorDebug();
}
pub trait IActiveScriptSiteDebug64Impl: Sized {
    fn GetDocumentContextFromPosition();
    fn GetApplication();
    fn GetRootApplicationNode();
    fn OnScriptErrorDebug();
}
pub trait IActiveScriptSiteDebugExImpl: Sized {
    fn OnCanNotJITScriptErrorDebug();
}
pub trait IActiveScriptSiteInterruptPollImpl: Sized {
    fn QueryContinue();
}
pub trait IActiveScriptSiteTraceInfoImpl: Sized {
    fn SendScriptTraceInfo();
}
pub trait IActiveScriptSiteUIControlImpl: Sized {
    fn GetUIBehavior();
}
pub trait IActiveScriptSiteWindowImpl: Sized {
    fn GetWindow();
    fn EnableModeless();
}
pub trait IActiveScriptStatsImpl: Sized {
    fn GetStat();
    fn GetStatEx();
    fn ResetStats();
}
pub trait IActiveScriptStringCompareImpl: Sized {
    fn StrComp();
}
pub trait IActiveScriptTraceInfoImpl: Sized {
    fn StartScriptTracing();
    fn StopScriptTracing();
}
pub trait IActiveScriptWinRTErrorDebugImpl: Sized + IActiveScriptErrorImpl {
    fn GetRestrictedErrorString();
    fn GetRestrictedErrorReference();
    fn GetCapabilitySid();
}
pub trait IApplicationDebuggerImpl: Sized {
    fn QueryAlive();
    fn CreateInstanceAtDebugger();
    fn onDebugOutput();
    fn onHandleBreakPoint();
    fn onClose();
    fn onDebuggerEvent();
}
pub trait IApplicationDebuggerUIImpl: Sized {
    fn BringDocumentToTop();
    fn BringDocumentContextToTop();
}
pub trait IBindEventHandlerImpl: Sized {
    fn BindHandler();
}
pub trait ICodeAddressConceptImpl: Sized {
    fn GetContainingSymbol();
}
pub trait IComparableConceptImpl: Sized {
    fn CompareObjects();
}
pub trait IDataModelConceptImpl: Sized {
    fn InitializeObject();
    fn GetName();
}
pub trait IDataModelManagerImpl: Sized {
    fn Close();
    fn CreateNoValue();
    fn CreateErrorObject();
    fn CreateTypedObject();
    fn CreateTypedObjectReference();
    fn CreateSyntheticObject();
    fn CreateDataModelObject();
    fn CreateIntrinsicObject();
    fn CreateTypedIntrinsicObject();
    fn GetModelForTypeSignature();
    fn GetModelForType();
    fn RegisterModelForTypeSignature();
    fn UnregisterModelForTypeSignature();
    fn RegisterExtensionForTypeSignature();
    fn UnregisterExtensionForTypeSignature();
    fn CreateMetadataStore();
    fn GetRootNamespace();
    fn RegisterNamedModel();
    fn UnregisterNamedModel();
    fn AcquireNamedModel();
}
pub trait IDataModelManager2Impl: Sized + IDataModelManagerImpl {
    fn AcquireSubNamespace();
    fn CreateTypedIntrinsicObjectEx();
}
pub trait IDataModelNameBinderImpl: Sized {
    fn BindValue();
    fn BindReference();
    fn EnumerateValues();
    fn EnumerateReferences();
}
pub trait IDataModelScriptImpl: Sized {
    fn GetName();
    fn Rename();
    fn Populate();
    fn Execute();
    fn Unlink();
    fn IsInvocable();
    fn InvokeMain();
}
pub trait IDataModelScriptClientImpl: Sized {
    fn ReportError();
}
pub trait IDataModelScriptDebugImpl: Sized {
    fn GetDebugState();
    fn GetCurrentPosition();
    fn GetStack();
    fn SetBreakpoint();
    fn FindBreakpointById();
    fn EnumerateBreakpoints();
    fn GetEventFilter();
    fn SetEventFilter();
    fn StartDebugging();
    fn StopDebugging();
}
pub trait IDataModelScriptDebug2Impl: Sized + IDataModelScriptDebugImpl {
    fn SetBreakpointAtFunction();
}
pub trait IDataModelScriptDebugBreakpointImpl: Sized {
    fn GetId();
    fn IsEnabled();
    fn Enable();
    fn Disable();
    fn Remove();
    fn GetPosition();
}
pub trait IDataModelScriptDebugBreakpointEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IDataModelScriptDebugClientImpl: Sized {
    fn NotifyDebugEvent();
}
pub trait IDataModelScriptDebugStackImpl: Sized {
    fn GetFrameCount();
    fn GetStackFrame();
}
pub trait IDataModelScriptDebugStackFrameImpl: Sized {
    fn GetName();
    fn GetPosition();
    fn IsTransitionPoint();
    fn GetTransition();
    fn Evaluate();
    fn EnumerateLocals();
    fn EnumerateArguments();
}
pub trait IDataModelScriptDebugVariableSetEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IDataModelScriptHostContextImpl: Sized {
    fn NotifyScriptChange();
    fn GetNamespaceObject();
}
pub trait IDataModelScriptManagerImpl: Sized {
    fn GetDefaultNameBinder();
    fn RegisterScriptProvider();
    fn UnregisterScriptProvider();
    fn FindProviderForScriptType();
    fn FindProviderForScriptExtension();
    fn EnumerateScriptProviders();
}
pub trait IDataModelScriptProviderImpl: Sized {
    fn GetName();
    fn GetExtension();
    fn CreateScript();
    fn GetDefaultTemplateContent();
    fn EnumerateTemplates();
}
pub trait IDataModelScriptProviderEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IDataModelScriptTemplateImpl: Sized {
    fn GetName();
    fn GetDescription();
    fn GetContent();
}
pub trait IDataModelScriptTemplateEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IDebugAdvancedImpl: Sized {
    fn GetThreadContext();
    fn SetThreadContext();
}
pub trait IDebugAdvanced2Impl: Sized {
    fn GetThreadContext();
    fn SetThreadContext();
    fn Request();
    fn GetSourceFileInformation();
    fn FindSourceFileAndToken();
    fn GetSymbolInformation();
    fn GetSystemObjectInformation();
}
pub trait IDebugAdvanced3Impl: Sized {
    fn GetThreadContext();
    fn SetThreadContext();
    fn Request();
    fn GetSourceFileInformation();
    fn FindSourceFileAndToken();
    fn GetSymbolInformation();
    fn GetSystemObjectInformation();
    fn GetSourceFileInformationWide();
    fn FindSourceFileAndTokenWide();
    fn GetSymbolInformationWide();
}
pub trait IDebugAdvanced4Impl: Sized {
    fn GetThreadContext();
    fn SetThreadContext();
    fn Request();
    fn GetSourceFileInformation();
    fn FindSourceFileAndToken();
    fn GetSymbolInformation();
    fn GetSystemObjectInformation();
    fn GetSourceFileInformationWide();
    fn FindSourceFileAndTokenWide();
    fn GetSymbolInformationWide();
    fn GetSymbolInformationWideEx();
}
pub trait IDebugApplication11032Impl: Sized + IRemoteDebugApplication110Impl {
    fn SynchronousCallInMainThread();
    fn AsynchronousCallInMainThread();
    fn CallableWaitForHandles();
}
pub trait IDebugApplication11064Impl: Sized + IRemoteDebugApplication110Impl {
    fn SynchronousCallInMainThread();
    fn AsynchronousCallInMainThread();
    fn CallableWaitForHandles();
}
pub trait IDebugApplication32Impl: Sized + IRemoteDebugApplicationImpl {
    fn SetName();
    fn StepOutComplete();
    fn DebugOutput();
    fn StartDebugSession();
    fn HandleBreakPoint();
    fn Close();
    fn GetBreakFlags();
    fn GetCurrentThread();
    fn CreateAsyncDebugOperation();
    fn AddStackFrameSniffer();
    fn RemoveStackFrameSniffer();
    fn QueryCurrentThreadIsDebuggerThread();
    fn SynchronousCallInDebuggerThread();
    fn CreateApplicationNode();
    fn FireDebuggerEvent();
    fn HandleRuntimeError();
    fn FCanJitDebug();
    fn FIsAutoJitDebugEnabled();
    fn AddGlobalExpressionContextProvider();
    fn RemoveGlobalExpressionContextProvider();
}
pub trait IDebugApplication64Impl: Sized + IRemoteDebugApplicationImpl {
    fn SetName();
    fn StepOutComplete();
    fn DebugOutput();
    fn StartDebugSession();
    fn HandleBreakPoint();
    fn Close();
    fn GetBreakFlags();
    fn GetCurrentThread();
    fn CreateAsyncDebugOperation();
    fn AddStackFrameSniffer();
    fn RemoveStackFrameSniffer();
    fn QueryCurrentThreadIsDebuggerThread();
    fn SynchronousCallInDebuggerThread();
    fn CreateApplicationNode();
    fn FireDebuggerEvent();
    fn HandleRuntimeError();
    fn FCanJitDebug();
    fn FIsAutoJitDebugEnabled();
    fn AddGlobalExpressionContextProvider();
    fn RemoveGlobalExpressionContextProvider();
}
pub trait IDebugApplicationNodeImpl: Sized + IDebugDocumentProviderImpl + IDebugDocumentInfoImpl {
    fn EnumChildren();
    fn GetParent();
    fn SetDocumentProvider();
    fn Close();
    fn Attach();
    fn Detach();
}
pub trait IDebugApplicationNode100Impl: Sized {
    fn SetFilterForEventSink();
    fn GetExcludedDocuments();
    fn QueryIsChildNode();
}
pub trait IDebugApplicationNodeEventsImpl: Sized {
    fn onAddChild();
    fn onRemoveChild();
    fn onDetach();
    fn onAttach();
}
pub trait IDebugApplicationThreadImpl: Sized + IRemoteDebugApplicationThreadImpl {
    fn SynchronousCallIntoThread32();
    fn QueryIsCurrentThread();
    fn QueryIsDebuggerThread();
    fn SetDescription();
    fn SetStateString();
}
pub trait IDebugApplicationThread11032Impl: Sized {
    fn GetActiveThreadRequestCount();
    fn IsSuspendedForBreakPoint();
    fn IsThreadCallable();
    fn AsynchronousCallIntoThread();
}
pub trait IDebugApplicationThread11064Impl: Sized {
    fn GetActiveThreadRequestCount();
    fn IsSuspendedForBreakPoint();
    fn IsThreadCallable();
    fn AsynchronousCallIntoThread();
}
pub trait IDebugApplicationThread64Impl: Sized + IDebugApplicationThreadImpl + IRemoteDebugApplicationThreadImpl {
    fn SynchronousCallIntoThread64();
}
pub trait IDebugApplicationThreadEvents110Impl: Sized {
    fn OnSuspendForBreakPoint();
    fn OnResumeFromBreakPoint();
    fn OnThreadRequestComplete();
    fn OnBeginThreadRequest();
}
pub trait IDebugAsyncOperationImpl: Sized {
    fn GetSyncDebugOperation();
    fn Start();
    fn Abort();
    fn QueryIsComplete();
    fn GetResult();
}
pub trait IDebugAsyncOperationCallBackImpl: Sized {
    fn onComplete();
}
pub trait IDebugBreakpointImpl: Sized {
    fn GetId();
    fn GetType();
    fn GetAdder();
    fn GetFlags();
    fn AddFlags();
    fn RemoveFlags();
    fn SetFlags();
    fn GetOffset();
    fn SetOffset();
    fn GetDataParameters();
    fn SetDataParameters();
    fn GetPassCount();
    fn SetPassCount();
    fn GetCurrentPassCount();
    fn GetMatchThreadId();
    fn SetMatchThreadId();
    fn GetCommand();
    fn SetCommand();
    fn GetOffsetExpression();
    fn SetOffsetExpression();
    fn GetParameters();
}
pub trait IDebugBreakpoint2Impl: Sized {
    fn GetId();
    fn GetType();
    fn GetAdder();
    fn GetFlags();
    fn AddFlags();
    fn RemoveFlags();
    fn SetFlags();
    fn GetOffset();
    fn SetOffset();
    fn GetDataParameters();
    fn SetDataParameters();
    fn GetPassCount();
    fn SetPassCount();
    fn GetCurrentPassCount();
    fn GetMatchThreadId();
    fn SetMatchThreadId();
    fn GetCommand();
    fn SetCommand();
    fn GetOffsetExpression();
    fn SetOffsetExpression();
    fn GetParameters();
    fn GetCommandWide();
    fn SetCommandWide();
    fn GetOffsetExpressionWide();
    fn SetOffsetExpressionWide();
}
pub trait IDebugBreakpoint3Impl: Sized {
    fn GetId();
    fn GetType();
    fn GetAdder();
    fn GetFlags();
    fn AddFlags();
    fn RemoveFlags();
    fn SetFlags();
    fn GetOffset();
    fn SetOffset();
    fn GetDataParameters();
    fn SetDataParameters();
    fn GetPassCount();
    fn SetPassCount();
    fn GetCurrentPassCount();
    fn GetMatchThreadId();
    fn SetMatchThreadId();
    fn GetCommand();
    fn SetCommand();
    fn GetOffsetExpression();
    fn SetOffsetExpression();
    fn GetParameters();
    fn GetCommandWide();
    fn SetCommandWide();
    fn GetOffsetExpressionWide();
    fn SetOffsetExpressionWide();
    fn GetGuid();
}
pub trait IDebugClientImpl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
}
pub trait IDebugClient2Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
}
pub trait IDebugClient3Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
}
pub trait IDebugClient4Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
    fn OpenDumpFileWide();
    fn WriteDumpFileWide();
    fn AddDumpInformationFileWide();
    fn GetNumberDumpFiles();
    fn GetDumpFile();
    fn GetDumpFileWide();
}
pub trait IDebugClient5Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
    fn OpenDumpFileWide();
    fn WriteDumpFileWide();
    fn AddDumpInformationFileWide();
    fn GetNumberDumpFiles();
    fn GetDumpFile();
    fn GetDumpFileWide();
    fn AttachKernelWide();
    fn GetKernelConnectionOptionsWide();
    fn SetKernelConnectionOptionsWide();
    fn StartProcessServerWide();
    fn ConnectProcessServerWide();
    fn StartServerWide();
    fn OutputServersWide();
    fn GetOutputCallbacksWide();
    fn SetOutputCallbacksWide();
    fn GetOutputLinePrefixWide();
    fn SetOutputLinePrefixWide();
    fn GetIdentityWide();
    fn OutputIdentityWide();
    fn GetEventCallbacksWide();
    fn SetEventCallbacksWide();
    fn CreateProcess2();
    fn CreateProcess2Wide();
    fn CreateProcessAndAttach2();
    fn CreateProcessAndAttach2Wide();
    fn PushOutputLinePrefix();
    fn PushOutputLinePrefixWide();
    fn PopOutputLinePrefix();
    fn GetNumberInputCallbacks();
    fn GetNumberOutputCallbacks();
    fn GetNumberEventCallbacks();
    fn GetQuitLockString();
    fn SetQuitLockString();
    fn GetQuitLockStringWide();
    fn SetQuitLockStringWide();
}
pub trait IDebugClient6Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
    fn OpenDumpFileWide();
    fn WriteDumpFileWide();
    fn AddDumpInformationFileWide();
    fn GetNumberDumpFiles();
    fn GetDumpFile();
    fn GetDumpFileWide();
    fn AttachKernelWide();
    fn GetKernelConnectionOptionsWide();
    fn SetKernelConnectionOptionsWide();
    fn StartProcessServerWide();
    fn ConnectProcessServerWide();
    fn StartServerWide();
    fn OutputServersWide();
    fn GetOutputCallbacksWide();
    fn SetOutputCallbacksWide();
    fn GetOutputLinePrefixWide();
    fn SetOutputLinePrefixWide();
    fn GetIdentityWide();
    fn OutputIdentityWide();
    fn GetEventCallbacksWide();
    fn SetEventCallbacksWide();
    fn CreateProcess2();
    fn CreateProcess2Wide();
    fn CreateProcessAndAttach2();
    fn CreateProcessAndAttach2Wide();
    fn PushOutputLinePrefix();
    fn PushOutputLinePrefixWide();
    fn PopOutputLinePrefix();
    fn GetNumberInputCallbacks();
    fn GetNumberOutputCallbacks();
    fn GetNumberEventCallbacks();
    fn GetQuitLockString();
    fn SetQuitLockString();
    fn GetQuitLockStringWide();
    fn SetQuitLockStringWide();
    fn SetEventContextCallbacks();
}
pub trait IDebugClient7Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
    fn OpenDumpFileWide();
    fn WriteDumpFileWide();
    fn AddDumpInformationFileWide();
    fn GetNumberDumpFiles();
    fn GetDumpFile();
    fn GetDumpFileWide();
    fn AttachKernelWide();
    fn GetKernelConnectionOptionsWide();
    fn SetKernelConnectionOptionsWide();
    fn StartProcessServerWide();
    fn ConnectProcessServerWide();
    fn StartServerWide();
    fn OutputServersWide();
    fn GetOutputCallbacksWide();
    fn SetOutputCallbacksWide();
    fn GetOutputLinePrefixWide();
    fn SetOutputLinePrefixWide();
    fn GetIdentityWide();
    fn OutputIdentityWide();
    fn GetEventCallbacksWide();
    fn SetEventCallbacksWide();
    fn CreateProcess2();
    fn CreateProcess2Wide();
    fn CreateProcessAndAttach2();
    fn CreateProcessAndAttach2Wide();
    fn PushOutputLinePrefix();
    fn PushOutputLinePrefixWide();
    fn PopOutputLinePrefix();
    fn GetNumberInputCallbacks();
    fn GetNumberOutputCallbacks();
    fn GetNumberEventCallbacks();
    fn GetQuitLockString();
    fn SetQuitLockString();
    fn GetQuitLockStringWide();
    fn SetQuitLockStringWide();
    fn SetEventContextCallbacks();
    fn SetClientContext();
}
pub trait IDebugClient8Impl: Sized {
    fn AttachKernel();
    fn GetKernelConnectionOptions();
    fn SetKernelConnectionOptions();
    fn StartProcessServer();
    fn ConnectProcessServer();
    fn DisconnectProcessServer();
    fn GetRunningProcessSystemIds();
    fn GetRunningProcessSystemIdByExecutableName();
    fn GetRunningProcessDescription();
    fn AttachProcess();
    fn CreateProcessA();
    fn CreateProcessAndAttach();
    fn GetProcessOptions();
    fn AddProcessOptions();
    fn RemoveProcessOptions();
    fn SetProcessOptions();
    fn OpenDumpFile();
    fn WriteDumpFile();
    fn ConnectSession();
    fn StartServer();
    fn OutputServers();
    fn TerminateProcesses();
    fn DetachProcesses();
    fn EndSession();
    fn GetExitCode();
    fn DispatchCallbacks();
    fn ExitDispatch();
    fn CreateClient();
    fn GetInputCallbacks();
    fn SetInputCallbacks();
    fn GetOutputCallbacks();
    fn SetOutputCallbacks();
    fn GetOutputMask();
    fn SetOutputMask();
    fn GetOtherOutputMask();
    fn SetOtherOutputMask();
    fn GetOutputWidth();
    fn SetOutputWidth();
    fn GetOutputLinePrefix();
    fn SetOutputLinePrefix();
    fn GetIdentity();
    fn OutputIdentity();
    fn GetEventCallbacks();
    fn SetEventCallbacks();
    fn FlushCallbacks();
    fn WriteDumpFile2();
    fn AddDumpInformationFile();
    fn EndProcessServer();
    fn WaitForProcessServerEnd();
    fn IsKernelDebuggerEnabled();
    fn TerminateCurrentProcess();
    fn DetachCurrentProcess();
    fn AbandonCurrentProcess();
    fn GetRunningProcessSystemIdByExecutableNameWide();
    fn GetRunningProcessDescriptionWide();
    fn CreateProcessWide();
    fn CreateProcessAndAttachWide();
    fn OpenDumpFileWide();
    fn WriteDumpFileWide();
    fn AddDumpInformationFileWide();
    fn GetNumberDumpFiles();
    fn GetDumpFile();
    fn GetDumpFileWide();
    fn AttachKernelWide();
    fn GetKernelConnectionOptionsWide();
    fn SetKernelConnectionOptionsWide();
    fn StartProcessServerWide();
    fn ConnectProcessServerWide();
    fn StartServerWide();
    fn OutputServersWide();
    fn GetOutputCallbacksWide();
    fn SetOutputCallbacksWide();
    fn GetOutputLinePrefixWide();
    fn SetOutputLinePrefixWide();
    fn GetIdentityWide();
    fn OutputIdentityWide();
    fn GetEventCallbacksWide();
    fn SetEventCallbacksWide();
    fn CreateProcess2();
    fn CreateProcess2Wide();
    fn CreateProcessAndAttach2();
    fn CreateProcessAndAttach2Wide();
    fn PushOutputLinePrefix();
    fn PushOutputLinePrefixWide();
    fn PopOutputLinePrefix();
    fn GetNumberInputCallbacks();
    fn GetNumberOutputCallbacks();
    fn GetNumberEventCallbacks();
    fn GetQuitLockString();
    fn SetQuitLockString();
    fn GetQuitLockStringWide();
    fn SetQuitLockStringWide();
    fn SetEventContextCallbacks();
    fn SetClientContext();
    fn OpenDumpFileWide2();
}
pub trait IDebugCodeContextImpl: Sized {
    fn GetDocumentContext();
    fn SetBreakPoint();
}
pub trait IDebugControlImpl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
}
pub trait IDebugControl2Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
}
pub trait IDebugControl3Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
    fn GetAssemblyOptions();
    fn AddAssemblyOptions();
    fn RemoveAssemblyOptions();
    fn SetAssemblyOptions();
    fn GetExpressionSyntax();
    fn SetExpressionSyntax();
    fn SetExpressionSyntaxByName();
    fn GetNumberExpressionSyntaxes();
    fn GetExpressionSyntaxNames();
    fn GetNumberEvents();
    fn GetEventIndexDescription();
    fn GetCurrentEventIndex();
    fn SetNextEventIndex();
}
pub trait IDebugControl4Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
    fn GetAssemblyOptions();
    fn AddAssemblyOptions();
    fn RemoveAssemblyOptions();
    fn SetAssemblyOptions();
    fn GetExpressionSyntax();
    fn SetExpressionSyntax();
    fn SetExpressionSyntaxByName();
    fn GetNumberExpressionSyntaxes();
    fn GetExpressionSyntaxNames();
    fn GetNumberEvents();
    fn GetEventIndexDescription();
    fn GetCurrentEventIndex();
    fn SetNextEventIndex();
    fn GetLogFileWide();
    fn OpenLogFileWide();
    fn InputWide();
    fn ReturnInputWide();
    fn OutputWide();
    fn OutputVaListWide();
    fn ControlledOutputWide();
    fn ControlledOutputVaListWide();
    fn OutputPromptWide();
    fn OutputPromptVaListWide();
    fn GetPromptTextWide();
    fn AssembleWide();
    fn DisassembleWide();
    fn GetProcessorTypeNamesWide();
    fn GetTextMacroWide();
    fn SetTextMacroWide();
    fn EvaluateWide();
    fn ExecuteWide();
    fn ExecuteCommandFileWide();
    fn GetBreakpointByIndex2();
    fn GetBreakpointById2();
    fn AddBreakpoint2();
    fn RemoveBreakpoint2();
    fn AddExtensionWide();
    fn GetExtensionByPathWide();
    fn CallExtensionWide();
    fn GetExtensionFunctionWide();
    fn GetEventFilterTextWide();
    fn GetEventFilterCommandWide();
    fn SetEventFilterCommandWide();
    fn GetSpecificFilterArgumentWide();
    fn SetSpecificFilterArgumentWide();
    fn GetExceptionFilterSecondCommandWide();
    fn SetExceptionFilterSecondCommandWide();
    fn GetLastEventInformationWide();
    fn GetTextReplacementWide();
    fn SetTextReplacementWide();
    fn SetExpressionSyntaxByNameWide();
    fn GetExpressionSyntaxNamesWide();
    fn GetEventIndexDescriptionWide();
    fn GetLogFile2();
    fn OpenLogFile2();
    fn GetLogFile2Wide();
    fn OpenLogFile2Wide();
    fn GetSystemVersionValues();
    fn GetSystemVersionString();
    fn GetSystemVersionStringWide();
    fn GetContextStackTrace();
    fn OutputContextStackTrace();
    fn GetStoredEventInformation();
    fn GetManagedStatus();
    fn GetManagedStatusWide();
    fn ResetManagedStatus();
}
pub trait IDebugControl5Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
    fn GetAssemblyOptions();
    fn AddAssemblyOptions();
    fn RemoveAssemblyOptions();
    fn SetAssemblyOptions();
    fn GetExpressionSyntax();
    fn SetExpressionSyntax();
    fn SetExpressionSyntaxByName();
    fn GetNumberExpressionSyntaxes();
    fn GetExpressionSyntaxNames();
    fn GetNumberEvents();
    fn GetEventIndexDescription();
    fn GetCurrentEventIndex();
    fn SetNextEventIndex();
    fn GetLogFileWide();
    fn OpenLogFileWide();
    fn InputWide();
    fn ReturnInputWide();
    fn OutputWide();
    fn OutputVaListWide();
    fn ControlledOutputWide();
    fn ControlledOutputVaListWide();
    fn OutputPromptWide();
    fn OutputPromptVaListWide();
    fn GetPromptTextWide();
    fn AssembleWide();
    fn DisassembleWide();
    fn GetProcessorTypeNamesWide();
    fn GetTextMacroWide();
    fn SetTextMacroWide();
    fn EvaluateWide();
    fn ExecuteWide();
    fn ExecuteCommandFileWide();
    fn GetBreakpointByIndex2();
    fn GetBreakpointById2();
    fn AddBreakpoint2();
    fn RemoveBreakpoint2();
    fn AddExtensionWide();
    fn GetExtensionByPathWide();
    fn CallExtensionWide();
    fn GetExtensionFunctionWide();
    fn GetEventFilterTextWide();
    fn GetEventFilterCommandWide();
    fn SetEventFilterCommandWide();
    fn GetSpecificFilterArgumentWide();
    fn SetSpecificFilterArgumentWide();
    fn GetExceptionFilterSecondCommandWide();
    fn SetExceptionFilterSecondCommandWide();
    fn GetLastEventInformationWide();
    fn GetTextReplacementWide();
    fn SetTextReplacementWide();
    fn SetExpressionSyntaxByNameWide();
    fn GetExpressionSyntaxNamesWide();
    fn GetEventIndexDescriptionWide();
    fn GetLogFile2();
    fn OpenLogFile2();
    fn GetLogFile2Wide();
    fn OpenLogFile2Wide();
    fn GetSystemVersionValues();
    fn GetSystemVersionString();
    fn GetSystemVersionStringWide();
    fn GetContextStackTrace();
    fn OutputContextStackTrace();
    fn GetStoredEventInformation();
    fn GetManagedStatus();
    fn GetManagedStatusWide();
    fn ResetManagedStatus();
    fn GetStackTraceEx();
    fn OutputStackTraceEx();
    fn GetContextStackTraceEx();
    fn OutputContextStackTraceEx();
    fn GetBreakpointByGuid();
}
pub trait IDebugControl6Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
    fn GetAssemblyOptions();
    fn AddAssemblyOptions();
    fn RemoveAssemblyOptions();
    fn SetAssemblyOptions();
    fn GetExpressionSyntax();
    fn SetExpressionSyntax();
    fn SetExpressionSyntaxByName();
    fn GetNumberExpressionSyntaxes();
    fn GetExpressionSyntaxNames();
    fn GetNumberEvents();
    fn GetEventIndexDescription();
    fn GetCurrentEventIndex();
    fn SetNextEventIndex();
    fn GetLogFileWide();
    fn OpenLogFileWide();
    fn InputWide();
    fn ReturnInputWide();
    fn OutputWide();
    fn OutputVaListWide();
    fn ControlledOutputWide();
    fn ControlledOutputVaListWide();
    fn OutputPromptWide();
    fn OutputPromptVaListWide();
    fn GetPromptTextWide();
    fn AssembleWide();
    fn DisassembleWide();
    fn GetProcessorTypeNamesWide();
    fn GetTextMacroWide();
    fn SetTextMacroWide();
    fn EvaluateWide();
    fn ExecuteWide();
    fn ExecuteCommandFileWide();
    fn GetBreakpointByIndex2();
    fn GetBreakpointById2();
    fn AddBreakpoint2();
    fn RemoveBreakpoint2();
    fn AddExtensionWide();
    fn GetExtensionByPathWide();
    fn CallExtensionWide();
    fn GetExtensionFunctionWide();
    fn GetEventFilterTextWide();
    fn GetEventFilterCommandWide();
    fn SetEventFilterCommandWide();
    fn GetSpecificFilterArgumentWide();
    fn SetSpecificFilterArgumentWide();
    fn GetExceptionFilterSecondCommandWide();
    fn SetExceptionFilterSecondCommandWide();
    fn GetLastEventInformationWide();
    fn GetTextReplacementWide();
    fn SetTextReplacementWide();
    fn SetExpressionSyntaxByNameWide();
    fn GetExpressionSyntaxNamesWide();
    fn GetEventIndexDescriptionWide();
    fn GetLogFile2();
    fn OpenLogFile2();
    fn GetLogFile2Wide();
    fn OpenLogFile2Wide();
    fn GetSystemVersionValues();
    fn GetSystemVersionString();
    fn GetSystemVersionStringWide();
    fn GetContextStackTrace();
    fn OutputContextStackTrace();
    fn GetStoredEventInformation();
    fn GetManagedStatus();
    fn GetManagedStatusWide();
    fn ResetManagedStatus();
    fn GetStackTraceEx();
    fn OutputStackTraceEx();
    fn GetContextStackTraceEx();
    fn OutputContextStackTraceEx();
    fn GetBreakpointByGuid();
    fn GetExecutionStatusEx();
    fn GetSynchronizationStatus();
}
pub trait IDebugControl7Impl: Sized {
    fn GetInterrupt();
    fn SetInterrupt();
    fn GetInterruptTimeout();
    fn SetInterruptTimeout();
    fn GetLogFile();
    fn OpenLogFile();
    fn CloseLogFile();
    fn GetLogMask();
    fn SetLogMask();
    fn Input();
    fn ReturnInput();
    fn Output();
    fn OutputVaList();
    fn ControlledOutput();
    fn ControlledOutputVaList();
    fn OutputPrompt();
    fn OutputPromptVaList();
    fn GetPromptText();
    fn OutputCurrentState();
    fn OutputVersionInformation();
    fn GetNotifyEventHandle();
    fn SetNotifyEventHandle();
    fn Assemble();
    fn Disassemble();
    fn GetDisassembleEffectiveOffset();
    fn OutputDisassembly();
    fn OutputDisassemblyLines();
    fn GetNearInstruction();
    fn GetStackTrace();
    fn GetReturnOffset();
    fn OutputStackTrace();
    fn GetDebuggeeType();
    fn GetActualProcessorType();
    fn GetExecutingProcessorType();
    fn GetNumberPossibleExecutingProcessorTypes();
    fn GetPossibleExecutingProcessorTypes();
    fn GetNumberProcessors();
    fn GetSystemVersion();
    fn GetPageSize();
    fn IsPointer64Bit();
    fn ReadBugCheckData();
    fn GetNumberSupportedProcessorTypes();
    fn GetSupportedProcessorTypes();
    fn GetProcessorTypeNames();
    fn GetEffectiveProcessorType();
    fn SetEffectiveProcessorType();
    fn GetExecutionStatus();
    fn SetExecutionStatus();
    fn GetCodeLevel();
    fn SetCodeLevel();
    fn GetEngineOptions();
    fn AddEngineOptions();
    fn RemoveEngineOptions();
    fn SetEngineOptions();
    fn GetSystemErrorControl();
    fn SetSystemErrorControl();
    fn GetTextMacro();
    fn SetTextMacro();
    fn GetRadix();
    fn SetRadix();
    fn Evaluate();
    fn CoerceValue();
    fn CoerceValues();
    fn Execute();
    fn ExecuteCommandFile();
    fn GetNumberBreakpoints();
    fn GetBreakpointByIndex();
    fn GetBreakpointById();
    fn GetBreakpointParameters();
    fn AddBreakpoint();
    fn RemoveBreakpoint();
    fn AddExtension();
    fn RemoveExtension();
    fn GetExtensionByPath();
    fn CallExtension();
    fn GetExtensionFunction();
    fn GetWindbgExtensionApis32();
    fn GetWindbgExtensionApis64();
    fn GetNumberEventFilters();
    fn GetEventFilterText();
    fn GetEventFilterCommand();
    fn SetEventFilterCommand();
    fn GetSpecificFilterParameters();
    fn SetSpecificFilterParameters();
    fn GetSpecificFilterArgument();
    fn SetSpecificFilterArgument();
    fn GetExceptionFilterParameters();
    fn SetExceptionFilterParameters();
    fn GetExceptionFilterSecondCommand();
    fn SetExceptionFilterSecondCommand();
    fn WaitForEvent();
    fn GetLastEventInformation();
    fn GetCurrentTimeDate();
    fn GetCurrentSystemUpTime();
    fn GetDumpFormatFlags();
    fn GetNumberTextReplacements();
    fn GetTextReplacement();
    fn SetTextReplacement();
    fn RemoveTextReplacements();
    fn OutputTextReplacements();
    fn GetAssemblyOptions();
    fn AddAssemblyOptions();
    fn RemoveAssemblyOptions();
    fn SetAssemblyOptions();
    fn GetExpressionSyntax();
    fn SetExpressionSyntax();
    fn SetExpressionSyntaxByName();
    fn GetNumberExpressionSyntaxes();
    fn GetExpressionSyntaxNames();
    fn GetNumberEvents();
    fn GetEventIndexDescription();
    fn GetCurrentEventIndex();
    fn SetNextEventIndex();
    fn GetLogFileWide();
    fn OpenLogFileWide();
    fn InputWide();
    fn ReturnInputWide();
    fn OutputWide();
    fn OutputVaListWide();
    fn ControlledOutputWide();
    fn ControlledOutputVaListWide();
    fn OutputPromptWide();
    fn OutputPromptVaListWide();
    fn GetPromptTextWide();
    fn AssembleWide();
    fn DisassembleWide();
    fn GetProcessorTypeNamesWide();
    fn GetTextMacroWide();
    fn SetTextMacroWide();
    fn EvaluateWide();
    fn ExecuteWide();
    fn ExecuteCommandFileWide();
    fn GetBreakpointByIndex2();
    fn GetBreakpointById2();
    fn AddBreakpoint2();
    fn RemoveBreakpoint2();
    fn AddExtensionWide();
    fn GetExtensionByPathWide();
    fn CallExtensionWide();
    fn GetExtensionFunctionWide();
    fn GetEventFilterTextWide();
    fn GetEventFilterCommandWide();
    fn SetEventFilterCommandWide();
    fn GetSpecificFilterArgumentWide();
    fn SetSpecificFilterArgumentWide();
    fn GetExceptionFilterSecondCommandWide();
    fn SetExceptionFilterSecondCommandWide();
    fn GetLastEventInformationWide();
    fn GetTextReplacementWide();
    fn SetTextReplacementWide();
    fn SetExpressionSyntaxByNameWide();
    fn GetExpressionSyntaxNamesWide();
    fn GetEventIndexDescriptionWide();
    fn GetLogFile2();
    fn OpenLogFile2();
    fn GetLogFile2Wide();
    fn OpenLogFile2Wide();
    fn GetSystemVersionValues();
    fn GetSystemVersionString();
    fn GetSystemVersionStringWide();
    fn GetContextStackTrace();
    fn OutputContextStackTrace();
    fn GetStoredEventInformation();
    fn GetManagedStatus();
    fn GetManagedStatusWide();
    fn ResetManagedStatus();
    fn GetStackTraceEx();
    fn OutputStackTraceEx();
    fn GetContextStackTraceEx();
    fn OutputContextStackTraceEx();
    fn GetBreakpointByGuid();
    fn GetExecutionStatusEx();
    fn GetSynchronizationStatus();
    fn GetDebuggeeType2();
}
pub trait IDebugCookieImpl: Sized {
    fn SetDebugCookie();
}
pub trait IDebugDataSpacesImpl: Sized {
    fn ReadVirtual();
    fn WriteVirtual();
    fn SearchVirtual();
    fn ReadVirtualUncached();
    fn WriteVirtualUncached();
    fn ReadPointersVirtual();
    fn WritePointersVirtual();
    fn ReadPhysical();
    fn WritePhysical();
    fn ReadControl();
    fn WriteControl();
    fn ReadIo();
    fn WriteIo();
    fn ReadMsr();
    fn WriteMsr();
    fn ReadBusData();
    fn WriteBusData();
    fn CheckLowMemory();
    fn ReadDebuggerData();
    fn ReadProcessorSystemData();
}
pub trait IDebugDataSpaces2Impl: Sized {
    fn ReadVirtual();
    fn WriteVirtual();
    fn SearchVirtual();
    fn ReadVirtualUncached();
    fn WriteVirtualUncached();
    fn ReadPointersVirtual();
    fn WritePointersVirtual();
    fn ReadPhysical();
    fn WritePhysical();
    fn ReadControl();
    fn WriteControl();
    fn ReadIo();
    fn WriteIo();
    fn ReadMsr();
    fn WriteMsr();
    fn ReadBusData();
    fn WriteBusData();
    fn CheckLowMemory();
    fn ReadDebuggerData();
    fn ReadProcessorSystemData();
    fn VirtualToPhysical();
    fn GetVirtualTranslationPhysicalOffsets();
    fn ReadHandleData();
    fn FillVirtual();
    fn FillPhysical();
    fn QueryVirtual();
}
pub trait IDebugDataSpaces3Impl: Sized {
    fn ReadVirtual();
    fn WriteVirtual();
    fn SearchVirtual();
    fn ReadVirtualUncached();
    fn WriteVirtualUncached();
    fn ReadPointersVirtual();
    fn WritePointersVirtual();
    fn ReadPhysical();
    fn WritePhysical();
    fn ReadControl();
    fn WriteControl();
    fn ReadIo();
    fn WriteIo();
    fn ReadMsr();
    fn WriteMsr();
    fn ReadBusData();
    fn WriteBusData();
    fn CheckLowMemory();
    fn ReadDebuggerData();
    fn ReadProcessorSystemData();
    fn VirtualToPhysical();
    fn GetVirtualTranslationPhysicalOffsets();
    fn ReadHandleData();
    fn FillVirtual();
    fn FillPhysical();
    fn QueryVirtual();
    fn ReadImageNtHeaders();
    fn ReadTagged();
    fn StartEnumTagged();
    fn GetNextTagged();
    fn EndEnumTagged();
}
pub trait IDebugDataSpaces4Impl: Sized {
    fn ReadVirtual();
    fn WriteVirtual();
    fn SearchVirtual();
    fn ReadVirtualUncached();
    fn WriteVirtualUncached();
    fn ReadPointersVirtual();
    fn WritePointersVirtual();
    fn ReadPhysical();
    fn WritePhysical();
    fn ReadControl();
    fn WriteControl();
    fn ReadIo();
    fn WriteIo();
    fn ReadMsr();
    fn WriteMsr();
    fn ReadBusData();
    fn WriteBusData();
    fn CheckLowMemory();
    fn ReadDebuggerData();
    fn ReadProcessorSystemData();
    fn VirtualToPhysical();
    fn GetVirtualTranslationPhysicalOffsets();
    fn ReadHandleData();
    fn FillVirtual();
    fn FillPhysical();
    fn QueryVirtual();
    fn ReadImageNtHeaders();
    fn ReadTagged();
    fn StartEnumTagged();
    fn GetNextTagged();
    fn EndEnumTagged();
    fn GetOffsetInformation();
    fn GetNextDifferentlyValidOffsetVirtual();
    fn GetValidRegionVirtual();
    fn SearchVirtual2();
    fn ReadMultiByteStringVirtual();
    fn ReadMultiByteStringVirtualWide();
    fn ReadUnicodeStringVirtual();
    fn ReadUnicodeStringVirtualWide();
    fn ReadPhysical2();
    fn WritePhysical2();
}
pub trait IDebugDocumentImpl: Sized + IDebugDocumentInfoImpl {}
pub trait IDebugDocumentContextImpl: Sized {
    fn GetDocument();
    fn EnumCodeContexts();
}
pub trait IDebugDocumentHelper32Impl: Sized {
    fn Init();
    fn Attach();
    fn Detach();
    fn AddUnicodeText();
    fn AddDBCSText();
    fn SetDebugDocumentHost();
    fn AddDeferredText();
    fn DefineScriptBlock();
    fn SetDefaultTextAttr();
    fn SetTextAttributes();
    fn SetLongName();
    fn SetShortName();
    fn SetDocumentAttr();
    fn GetDebugApplicationNode();
    fn GetScriptBlockInfo();
    fn CreateDebugDocumentContext();
    fn BringDocumentToTop();
    fn BringDocumentContextToTop();
}
pub trait IDebugDocumentHelper64Impl: Sized {
    fn Init();
    fn Attach();
    fn Detach();
    fn AddUnicodeText();
    fn AddDBCSText();
    fn SetDebugDocumentHost();
    fn AddDeferredText();
    fn DefineScriptBlock();
    fn SetDefaultTextAttr();
    fn SetTextAttributes();
    fn SetLongName();
    fn SetShortName();
    fn SetDocumentAttr();
    fn GetDebugApplicationNode();
    fn GetScriptBlockInfo();
    fn CreateDebugDocumentContext();
    fn BringDocumentToTop();
    fn BringDocumentContextToTop();
}
pub trait IDebugDocumentHostImpl: Sized {
    fn GetDeferredText();
    fn GetScriptTextAttributes();
    fn OnCreateDocumentContext();
    fn GetPathName();
    fn GetFileName();
    fn NotifyChanged();
}
pub trait IDebugDocumentInfoImpl: Sized {
    fn GetName();
    fn GetDocumentClassId();
}
pub trait IDebugDocumentProviderImpl: Sized + IDebugDocumentInfoImpl {
    fn GetDocument();
}
pub trait IDebugDocumentTextImpl: Sized + IDebugDocumentImpl + IDebugDocumentInfoImpl {
    fn GetDocumentAttributes();
    fn GetSize();
    fn GetPositionOfLine();
    fn GetLineOfPosition();
    fn GetText();
    fn GetPositionOfContext();
    fn GetContextOfPosition();
}
pub trait IDebugDocumentTextAuthorImpl: Sized + IDebugDocumentTextImpl + IDebugDocumentImpl + IDebugDocumentInfoImpl {
    fn InsertText();
    fn RemoveText();
    fn ReplaceText();
}
pub trait IDebugDocumentTextEventsImpl: Sized {
    fn onDestroy();
    fn onInsertText();
    fn onRemoveText();
    fn onReplaceText();
    fn onUpdateTextAttributes();
    fn onUpdateDocumentAttributes();
}
pub trait IDebugDocumentTextExternalAuthorImpl: Sized {
    fn GetPathName();
    fn GetFileName();
    fn NotifyChanged();
}
pub trait IDebugEventCallbacksImpl: Sized {
    fn GetInterestMask();
    fn Breakpoint();
    fn Exception();
    fn CreateThread();
    fn ExitThread();
    fn CreateProcessA();
    fn ExitProcess();
    fn LoadModule();
    fn UnloadModule();
    fn SystemError();
    fn SessionStatus();
    fn ChangeDebuggeeState();
    fn ChangeEngineState();
    fn ChangeSymbolState();
}
pub trait IDebugEventCallbacksWideImpl: Sized {
    fn GetInterestMask();
    fn Breakpoint();
    fn Exception();
    fn CreateThread();
    fn ExitThread();
    fn CreateProcessA();
    fn ExitProcess();
    fn LoadModule();
    fn UnloadModule();
    fn SystemError();
    fn SessionStatus();
    fn ChangeDebuggeeState();
    fn ChangeEngineState();
    fn ChangeSymbolState();
}
pub trait IDebugEventContextCallbacksImpl: Sized {
    fn GetInterestMask();
    fn Breakpoint();
    fn Exception();
    fn CreateThread();
    fn ExitThread();
    fn CreateProcessA();
    fn ExitProcess();
    fn LoadModule();
    fn UnloadModule();
    fn SystemError();
    fn SessionStatus();
    fn ChangeDebuggeeState();
    fn ChangeEngineState();
    fn ChangeSymbolState();
}
pub trait IDebugExpressionImpl: Sized {
    fn Start();
    fn Abort();
    fn QueryIsComplete();
    fn GetResultAsString();
    fn GetResultAsDebugProperty();
}
pub trait IDebugExpressionCallBackImpl: Sized {
    fn onComplete();
}
pub trait IDebugExpressionContextImpl: Sized {
    fn ParseLanguageText();
    fn GetLanguageInfo();
}
pub trait IDebugExtendedPropertyImpl: Sized + IDebugPropertyImpl {
    fn GetExtendedPropertyInfo();
    fn EnumExtendedMembers();
}
pub trait IDebugFormatterImpl: Sized {
    fn GetStringForVariant();
    fn GetVariantForString();
    fn GetStringForVarType();
}
pub trait IDebugHelperImpl: Sized {
    fn CreatePropertyBrowser();
    fn CreatePropertyBrowserEx();
    fn CreateSimpleConnectionPoint();
}
pub trait IDebugHostImpl: Sized {
    fn GetHostDefinedInterface();
    fn GetCurrentContext();
    fn GetDefaultMetadata();
}
pub trait IDebugHostBaseClassImpl: Sized + IDebugHostSymbolImpl {
    fn GetOffset();
}
pub trait IDebugHostConstantImpl: Sized + IDebugHostSymbolImpl {
    fn GetValue();
}
pub trait IDebugHostContextImpl: Sized {
    fn IsEqualTo();
}
pub trait IDebugHostDataImpl: Sized + IDebugHostSymbolImpl {
    fn GetLocationKind();
    fn GetLocation();
    fn GetValue();
}
pub trait IDebugHostErrorSinkImpl: Sized {
    fn ReportError();
}
pub trait IDebugHostEvaluatorImpl: Sized {
    fn EvaluateExpression();
    fn EvaluateExtendedExpression();
}
pub trait IDebugHostEvaluator2Impl: Sized + IDebugHostEvaluatorImpl {
    fn AssignTo();
}
pub trait IDebugHostExtensibilityImpl: Sized {
    fn CreateFunctionAlias();
    fn DestroyFunctionAlias();
}
pub trait IDebugHostFieldImpl: Sized + IDebugHostSymbolImpl {
    fn GetLocationKind();
    fn GetOffset();
    fn GetLocation();
    fn GetValue();
}
pub trait IDebugHostMemoryImpl: Sized {
    fn ReadBytes();
    fn WriteBytes();
    fn ReadPointers();
    fn WritePointers();
    fn GetDisplayStringForLocation();
}
pub trait IDebugHostMemory2Impl: Sized + IDebugHostMemoryImpl {
    fn LinearizeLocation();
}
pub trait IDebugHostModuleImpl: Sized + IDebugHostSymbolImpl {
    fn GetImageName();
    fn GetBaseLocation();
    fn GetVersion();
    fn FindTypeByName();
    fn FindSymbolByRVA();
    fn FindSymbolByName();
}
pub trait IDebugHostModule2Impl: Sized + IDebugHostModuleImpl + IDebugHostSymbolImpl {
    fn FindContainingSymbolByRVA();
}
pub trait IDebugHostModuleSignatureImpl: Sized {
    fn IsMatch();
}
pub trait IDebugHostPublicImpl: Sized + IDebugHostSymbolImpl {
    fn GetLocationKind();
    fn GetLocation();
}
pub trait IDebugHostScriptHostImpl: Sized {
    fn CreateContext();
}
pub trait IDebugHostStatusImpl: Sized {
    fn PollUserInterrupt();
}
pub trait IDebugHostSymbolImpl: Sized {
    fn GetContext();
    fn EnumerateChildren();
    fn GetSymbolKind();
    fn GetName();
    fn GetType();
    fn GetContainingModule();
    fn CompareAgainst();
}
pub trait IDebugHostSymbol2Impl: Sized + IDebugHostSymbolImpl {
    fn GetLanguage();
}
pub trait IDebugHostSymbolEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IDebugHostSymbolsImpl: Sized {
    fn CreateModuleSignature();
    fn CreateTypeSignature();
    fn CreateTypeSignatureForModuleRange();
    fn EnumerateModules();
    fn FindModuleByName();
    fn FindModuleByLocation();
    fn GetMostDerivedObject();
}
pub trait IDebugHostTypeImpl: Sized + IDebugHostSymbolImpl {
    fn GetTypeKind();
    fn GetSize();
    fn GetBaseType();
    fn GetHashCode();
    fn GetIntrinsicType();
    fn GetBitField();
    fn GetPointerKind();
    fn GetMemberType();
    fn CreatePointerTo();
    fn GetArrayDimensionality();
    fn GetArrayDimensions();
    fn CreateArrayOf();
    fn GetFunctionCallingConvention();
    fn GetFunctionReturnType();
    fn GetFunctionParameterTypeCount();
    fn GetFunctionParameterTypeAt();
    fn IsGeneric();
    fn GetGenericArgumentCount();
    fn GetGenericArgumentAt();
}
pub trait IDebugHostType2Impl: Sized + IDebugHostTypeImpl + IDebugHostSymbolImpl {
    fn IsTypedef();
    fn GetTypedefBaseType();
    fn GetTypedefFinalBaseType();
    fn GetFunctionVarArgsKind();
    fn GetFunctionInstancePointerType();
}
pub trait IDebugHostTypeSignatureImpl: Sized {
    fn GetHashCode();
    fn IsMatch();
    fn CompareAgainst();
}
pub trait IDebugInputCallbacksImpl: Sized {
    fn StartInput();
    fn EndInput();
}
pub trait IDebugOutputCallbacksImpl: Sized {
    fn Output();
}
pub trait IDebugOutputCallbacks2Impl: Sized {
    fn Output();
    fn GetInterestMask();
    fn Output2();
}
pub trait IDebugOutputCallbacksWideImpl: Sized {
    fn Output();
}
pub trait IDebugOutputStreamImpl: Sized {
    fn Write();
}
pub trait IDebugPlmClientImpl: Sized {
    fn LaunchPlmPackageForDebugWide();
}
pub trait IDebugPlmClient2Impl: Sized {
    fn LaunchPlmPackageForDebugWide();
    fn LaunchPlmBgTaskForDebugWide();
}
pub trait IDebugPlmClient3Impl: Sized {
    fn LaunchPlmPackageForDebugWide();
    fn LaunchPlmBgTaskForDebugWide();
    fn QueryPlmPackageWide();
    fn QueryPlmPackageList();
    fn EnablePlmPackageDebugWide();
    fn DisablePlmPackageDebugWide();
    fn SuspendPlmPackageWide();
    fn ResumePlmPackageWide();
    fn TerminatePlmPackageWide();
    fn LaunchAndDebugPlmAppWide();
    fn ActivateAndDebugPlmBgTaskWide();
}
pub trait IDebugPropertyImpl: Sized {
    fn GetPropertyInfo();
    fn GetExtendedInfo();
    fn SetValueAsString();
    fn EnumMembers();
    fn GetParent();
}
pub trait IDebugPropertyEnumType_AllImpl: Sized {
    fn GetName();
}
pub trait IDebugPropertyEnumType_ArgumentsImpl: Sized + IDebugPropertyEnumType_AllImpl {}
pub trait IDebugPropertyEnumType_LocalsImpl: Sized + IDebugPropertyEnumType_AllImpl {}
pub trait IDebugPropertyEnumType_LocalsPlusArgsImpl: Sized + IDebugPropertyEnumType_AllImpl {}
pub trait IDebugPropertyEnumType_RegistersImpl: Sized + IDebugPropertyEnumType_AllImpl {}
pub trait IDebugRegistersImpl: Sized {
    fn GetNumberRegisters();
    fn GetDescription();
    fn GetIndexByName();
    fn GetValue();
    fn SetValue();
    fn GetValues();
    fn SetValues();
    fn OutputRegisters();
    fn GetInstructionOffset();
    fn GetStackOffset();
    fn GetFrameOffset();
}
pub trait IDebugRegisters2Impl: Sized {
    fn GetNumberRegisters();
    fn GetDescription();
    fn GetIndexByName();
    fn GetValue();
    fn SetValue();
    fn GetValues();
    fn SetValues();
    fn OutputRegisters();
    fn GetInstructionOffset();
    fn GetStackOffset();
    fn GetFrameOffset();
    fn GetDescriptionWide();
    fn GetIndexByNameWide();
    fn GetNumberPseudoRegisters();
    fn GetPseudoDescription();
    fn GetPseudoDescriptionWide();
    fn GetPseudoIndexByName();
    fn GetPseudoIndexByNameWide();
    fn GetPseudoValues();
    fn SetPseudoValues();
    fn GetValues2();
    fn SetValues2();
    fn OutputRegisters2();
    fn GetInstructionOffset2();
    fn GetStackOffset2();
    fn GetFrameOffset2();
}
pub trait IDebugSessionProviderImpl: Sized {
    fn StartDebugSession();
}
pub trait IDebugStackFrameImpl: Sized {
    fn GetCodeContext();
    fn GetDescriptionString();
    fn GetLanguageString();
    fn GetThread();
    fn GetDebugProperty();
}
pub trait IDebugStackFrame110Impl: Sized + IDebugStackFrameImpl {
    fn GetStackFrameType();
    fn GetScriptInvocationContext();
}
pub trait IDebugStackFrameSnifferImpl: Sized {
    fn EnumStackFrames();
}
pub trait IDebugStackFrameSnifferEx32Impl: Sized + IDebugStackFrameSnifferImpl {
    fn EnumStackFramesEx32();
}
pub trait IDebugStackFrameSnifferEx64Impl: Sized + IDebugStackFrameSnifferImpl {
    fn EnumStackFramesEx64();
}
pub trait IDebugSymbolGroupImpl: Sized {
    fn GetNumberSymbols();
    fn AddSymbol();
    fn RemoveSymbolByName();
    fn RemoveSymbolByIndex();
    fn GetSymbolName();
    fn GetSymbolParameters();
    fn ExpandSymbol();
    fn OutputSymbols();
    fn WriteSymbol();
    fn OutputAsType();
}
pub trait IDebugSymbolGroup2Impl: Sized {
    fn GetNumberSymbols();
    fn AddSymbol();
    fn RemoveSymbolByName();
    fn RemoveSymbolByIndex();
    fn GetSymbolName();
    fn GetSymbolParameters();
    fn ExpandSymbol();
    fn OutputSymbols();
    fn WriteSymbol();
    fn OutputAsType();
    fn AddSymbolWide();
    fn RemoveSymbolByNameWide();
    fn GetSymbolNameWide();
    fn WriteSymbolWide();
    fn OutputAsTypeWide();
    fn GetSymbolTypeName();
    fn GetSymbolTypeNameWide();
    fn GetSymbolSize();
    fn GetSymbolOffset();
    fn GetSymbolRegister();
    fn GetSymbolValueText();
    fn GetSymbolValueTextWide();
    fn GetSymbolEntryInformation();
}
pub trait IDebugSymbolsImpl: Sized {
    fn GetSymbolOptions();
    fn AddSymbolOptions();
    fn RemoveSymbolOptions();
    fn SetSymbolOptions();
    fn GetNameByOffset();
    fn GetOffsetByName();
    fn GetNearNameByOffset();
    fn GetLineByOffset();
    fn GetOffsetByLine();
    fn GetNumberModules();
    fn GetModuleByIndex();
    fn GetModuleByModuleName();
    fn GetModuleByOffset();
    fn GetModuleNames();
    fn GetModuleParameters();
    fn GetSymbolModule();
    fn GetTypeName();
    fn GetTypeId();
    fn GetTypeSize();
    fn GetFieldOffset();
    fn GetSymbolTypeId();
    fn GetOffsetTypeId();
    fn ReadTypedDataVirtual();
    fn WriteTypedDataVirtual();
    fn OutputTypedDataVirtual();
    fn ReadTypedDataPhysical();
    fn WriteTypedDataPhysical();
    fn OutputTypedDataPhysical();
    fn GetScope();
    fn SetScope();
    fn ResetScope();
    fn GetScopeSymbolGroup();
    fn CreateSymbolGroup();
    fn StartSymbolMatch();
    fn GetNextSymbolMatch();
    fn EndSymbolMatch();
    fn Reload();
    fn GetSymbolPath();
    fn SetSymbolPath();
    fn AppendSymbolPath();
    fn GetImagePath();
    fn SetImagePath();
    fn AppendImagePath();
    fn GetSourcePath();
    fn GetSourcePathElement();
    fn SetSourcePath();
    fn AppendSourcePath();
    fn FindSourceFile();
    fn GetSourceFileLineOffsets();
}
pub trait IDebugSymbols2Impl: Sized {
    fn GetSymbolOptions();
    fn AddSymbolOptions();
    fn RemoveSymbolOptions();
    fn SetSymbolOptions();
    fn GetNameByOffset();
    fn GetOffsetByName();
    fn GetNearNameByOffset();
    fn GetLineByOffset();
    fn GetOffsetByLine();
    fn GetNumberModules();
    fn GetModuleByIndex();
    fn GetModuleByModuleName();
    fn GetModuleByOffset();
    fn GetModuleNames();
    fn GetModuleParameters();
    fn GetSymbolModule();
    fn GetTypeName();
    fn GetTypeId();
    fn GetTypeSize();
    fn GetFieldOffset();
    fn GetSymbolTypeId();
    fn GetOffsetTypeId();
    fn ReadTypedDataVirtual();
    fn WriteTypedDataVirtual();
    fn OutputTypedDataVirtual();
    fn ReadTypedDataPhysical();
    fn WriteTypedDataPhysical();
    fn OutputTypedDataPhysical();
    fn GetScope();
    fn SetScope();
    fn ResetScope();
    fn GetScopeSymbolGroup();
    fn CreateSymbolGroup();
    fn StartSymbolMatch();
    fn GetNextSymbolMatch();
    fn EndSymbolMatch();
    fn Reload();
    fn GetSymbolPath();
    fn SetSymbolPath();
    fn AppendSymbolPath();
    fn GetImagePath();
    fn SetImagePath();
    fn AppendImagePath();
    fn GetSourcePath();
    fn GetSourcePathElement();
    fn SetSourcePath();
    fn AppendSourcePath();
    fn FindSourceFile();
    fn GetSourceFileLineOffsets();
    fn GetModuleVersionInformation();
    fn GetModuleNameString();
    fn GetConstantName();
    fn GetFieldName();
    fn GetTypeOptions();
    fn AddTypeOptions();
    fn RemoveTypeOptions();
    fn SetTypeOptions();
}
pub trait IDebugSymbols3Impl: Sized {
    fn GetSymbolOptions();
    fn AddSymbolOptions();
    fn RemoveSymbolOptions();
    fn SetSymbolOptions();
    fn GetNameByOffset();
    fn GetOffsetByName();
    fn GetNearNameByOffset();
    fn GetLineByOffset();
    fn GetOffsetByLine();
    fn GetNumberModules();
    fn GetModuleByIndex();
    fn GetModuleByModuleName();
    fn GetModuleByOffset();
    fn GetModuleNames();
    fn GetModuleParameters();
    fn GetSymbolModule();
    fn GetTypeName();
    fn GetTypeId();
    fn GetTypeSize();
    fn GetFieldOffset();
    fn GetSymbolTypeId();
    fn GetOffsetTypeId();
    fn ReadTypedDataVirtual();
    fn WriteTypedDataVirtual();
    fn OutputTypedDataVirtual();
    fn ReadTypedDataPhysical();
    fn WriteTypedDataPhysical();
    fn OutputTypedDataPhysical();
    fn GetScope();
    fn SetScope();
    fn ResetScope();
    fn GetScopeSymbolGroup();
    fn CreateSymbolGroup();
    fn StartSymbolMatch();
    fn GetNextSymbolMatch();
    fn EndSymbolMatch();
    fn Reload();
    fn GetSymbolPath();
    fn SetSymbolPath();
    fn AppendSymbolPath();
    fn GetImagePath();
    fn SetImagePath();
    fn AppendImagePath();
    fn GetSourcePath();
    fn GetSourcePathElement();
    fn SetSourcePath();
    fn AppendSourcePath();
    fn FindSourceFile();
    fn GetSourceFileLineOffsets();
    fn GetModuleVersionInformation();
    fn GetModuleNameString();
    fn GetConstantName();
    fn GetFieldName();
    fn GetTypeOptions();
    fn AddTypeOptions();
    fn RemoveTypeOptions();
    fn SetTypeOptions();
    fn GetNameByOffsetWide();
    fn GetOffsetByNameWide();
    fn GetNearNameByOffsetWide();
    fn GetLineByOffsetWide();
    fn GetOffsetByLineWide();
    fn GetModuleByModuleNameWide();
    fn GetSymbolModuleWide();
    fn GetTypeNameWide();
    fn GetTypeIdWide();
    fn GetFieldOffsetWide();
    fn GetSymbolTypeIdWide();
    fn GetScopeSymbolGroup2();
    fn CreateSymbolGroup2();
    fn StartSymbolMatchWide();
    fn GetNextSymbolMatchWide();
    fn ReloadWide();
    fn GetSymbolPathWide();
    fn SetSymbolPathWide();
    fn AppendSymbolPathWide();
    fn GetImagePathWide();
    fn SetImagePathWide();
    fn AppendImagePathWide();
    fn GetSourcePathWide();
    fn GetSourcePathElementWide();
    fn SetSourcePathWide();
    fn AppendSourcePathWide();
    fn FindSourceFileWide();
    fn GetSourceFileLineOffsetsWide();
    fn GetModuleVersionInformationWide();
    fn GetModuleNameStringWide();
    fn GetConstantNameWide();
    fn GetFieldNameWide();
    fn IsManagedModule();
    fn GetModuleByModuleName2();
    fn GetModuleByModuleName2Wide();
    fn GetModuleByOffset2();
    fn AddSyntheticModule();
    fn AddSyntheticModuleWide();
    fn RemoveSyntheticModule();
    fn GetCurrentScopeFrameIndex();
    fn SetScopeFrameByIndex();
    fn SetScopeFromJitDebugInfo();
    fn SetScopeFromStoredEvent();
    fn OutputSymbolByOffset();
    fn GetFunctionEntryByOffset();
    fn GetFieldTypeAndOffset();
    fn GetFieldTypeAndOffsetWide();
    fn AddSyntheticSymbol();
    fn AddSyntheticSymbolWide();
    fn RemoveSyntheticSymbol();
    fn GetSymbolEntriesByOffset();
    fn GetSymbolEntriesByName();
    fn GetSymbolEntriesByNameWide();
    fn GetSymbolEntryByToken();
    fn GetSymbolEntryInformation();
    fn GetSymbolEntryString();
    fn GetSymbolEntryStringWide();
    fn GetSymbolEntryOffsetRegions();
    fn GetSymbolEntryBySymbolEntry();
    fn GetSourceEntriesByOffset();
    fn GetSourceEntriesByLine();
    fn GetSourceEntriesByLineWide();
    fn GetSourceEntryString();
    fn GetSourceEntryStringWide();
    fn GetSourceEntryOffsetRegions();
    fn GetSourceEntryBySourceEntry();
}
pub trait IDebugSymbols4Impl: Sized {
    fn GetSymbolOptions();
    fn AddSymbolOptions();
    fn RemoveSymbolOptions();
    fn SetSymbolOptions();
    fn GetNameByOffset();
    fn GetOffsetByName();
    fn GetNearNameByOffset();
    fn GetLineByOffset();
    fn GetOffsetByLine();
    fn GetNumberModules();
    fn GetModuleByIndex();
    fn GetModuleByModuleName();
    fn GetModuleByOffset();
    fn GetModuleNames();
    fn GetModuleParameters();
    fn GetSymbolModule();
    fn GetTypeName();
    fn GetTypeId();
    fn GetTypeSize();
    fn GetFieldOffset();
    fn GetSymbolTypeId();
    fn GetOffsetTypeId();
    fn ReadTypedDataVirtual();
    fn WriteTypedDataVirtual();
    fn OutputTypedDataVirtual();
    fn ReadTypedDataPhysical();
    fn WriteTypedDataPhysical();
    fn OutputTypedDataPhysical();
    fn GetScope();
    fn SetScope();
    fn ResetScope();
    fn GetScopeSymbolGroup();
    fn CreateSymbolGroup();
    fn StartSymbolMatch();
    fn GetNextSymbolMatch();
    fn EndSymbolMatch();
    fn Reload();
    fn GetSymbolPath();
    fn SetSymbolPath();
    fn AppendSymbolPath();
    fn GetImagePath();
    fn SetImagePath();
    fn AppendImagePath();
    fn GetSourcePath();
    fn GetSourcePathElement();
    fn SetSourcePath();
    fn AppendSourcePath();
    fn FindSourceFile();
    fn GetSourceFileLineOffsets();
    fn GetModuleVersionInformation();
    fn GetModuleNameString();
    fn GetConstantName();
    fn GetFieldName();
    fn GetTypeOptions();
    fn AddTypeOptions();
    fn RemoveTypeOptions();
    fn SetTypeOptions();
    fn GetNameByOffsetWide();
    fn GetOffsetByNameWide();
    fn GetNearNameByOffsetWide();
    fn GetLineByOffsetWide();
    fn GetOffsetByLineWide();
    fn GetModuleByModuleNameWide();
    fn GetSymbolModuleWide();
    fn GetTypeNameWide();
    fn GetTypeIdWide();
    fn GetFieldOffsetWide();
    fn GetSymbolTypeIdWide();
    fn GetScopeSymbolGroup2();
    fn CreateSymbolGroup2();
    fn StartSymbolMatchWide();
    fn GetNextSymbolMatchWide();
    fn ReloadWide();
    fn GetSymbolPathWide();
    fn SetSymbolPathWide();
    fn AppendSymbolPathWide();
    fn GetImagePathWide();
    fn SetImagePathWide();
    fn AppendImagePathWide();
    fn GetSourcePathWide();
    fn GetSourcePathElementWide();
    fn SetSourcePathWide();
    fn AppendSourcePathWide();
    fn FindSourceFileWide();
    fn GetSourceFileLineOffsetsWide();
    fn GetModuleVersionInformationWide();
    fn GetModuleNameStringWide();
    fn GetConstantNameWide();
    fn GetFieldNameWide();
    fn IsManagedModule();
    fn GetModuleByModuleName2();
    fn GetModuleByModuleName2Wide();
    fn GetModuleByOffset2();
    fn AddSyntheticModule();
    fn AddSyntheticModuleWide();
    fn RemoveSyntheticModule();
    fn GetCurrentScopeFrameIndex();
    fn SetScopeFrameByIndex();
    fn SetScopeFromJitDebugInfo();
    fn SetScopeFromStoredEvent();
    fn OutputSymbolByOffset();
    fn GetFunctionEntryByOffset();
    fn GetFieldTypeAndOffset();
    fn GetFieldTypeAndOffsetWide();
    fn AddSyntheticSymbol();
    fn AddSyntheticSymbolWide();
    fn RemoveSyntheticSymbol();
    fn GetSymbolEntriesByOffset();
    fn GetSymbolEntriesByName();
    fn GetSymbolEntriesByNameWide();
    fn GetSymbolEntryByToken();
    fn GetSymbolEntryInformation();
    fn GetSymbolEntryString();
    fn GetSymbolEntryStringWide();
    fn GetSymbolEntryOffsetRegions();
    fn GetSymbolEntryBySymbolEntry();
    fn GetSourceEntriesByOffset();
    fn GetSourceEntriesByLine();
    fn GetSourceEntriesByLineWide();
    fn GetSourceEntryString();
    fn GetSourceEntryStringWide();
    fn GetSourceEntryOffsetRegions();
    fn GetSourceEntryBySourceEntry();
    fn GetScopeEx();
    fn SetScopeEx();
    fn GetNameByInlineContext();
    fn GetNameByInlineContextWide();
    fn GetLineByInlineContext();
    fn GetLineByInlineContextWide();
    fn OutputSymbolByInlineContext();
}
pub trait IDebugSymbols5Impl: Sized {
    fn GetSymbolOptions();
    fn AddSymbolOptions();
    fn RemoveSymbolOptions();
    fn SetSymbolOptions();
    fn GetNameByOffset();
    fn GetOffsetByName();
    fn GetNearNameByOffset();
    fn GetLineByOffset();
    fn GetOffsetByLine();
    fn GetNumberModules();
    fn GetModuleByIndex();
    fn GetModuleByModuleName();
    fn GetModuleByOffset();
    fn GetModuleNames();
    fn GetModuleParameters();
    fn GetSymbolModule();
    fn GetTypeName();
    fn GetTypeId();
    fn GetTypeSize();
    fn GetFieldOffset();
    fn GetSymbolTypeId();
    fn GetOffsetTypeId();
    fn ReadTypedDataVirtual();
    fn WriteTypedDataVirtual();
    fn OutputTypedDataVirtual();
    fn ReadTypedDataPhysical();
    fn WriteTypedDataPhysical();
    fn OutputTypedDataPhysical();
    fn GetScope();
    fn SetScope();
    fn ResetScope();
    fn GetScopeSymbolGroup();
    fn CreateSymbolGroup();
    fn StartSymbolMatch();
    fn GetNextSymbolMatch();
    fn EndSymbolMatch();
    fn Reload();
    fn GetSymbolPath();
    fn SetSymbolPath();
    fn AppendSymbolPath();
    fn GetImagePath();
    fn SetImagePath();
    fn AppendImagePath();
    fn GetSourcePath();
    fn GetSourcePathElement();
    fn SetSourcePath();
    fn AppendSourcePath();
    fn FindSourceFile();
    fn GetSourceFileLineOffsets();
    fn GetModuleVersionInformation();
    fn GetModuleNameString();
    fn GetConstantName();
    fn GetFieldName();
    fn GetTypeOptions();
    fn AddTypeOptions();
    fn RemoveTypeOptions();
    fn SetTypeOptions();
    fn GetNameByOffsetWide();
    fn GetOffsetByNameWide();
    fn GetNearNameByOffsetWide();
    fn GetLineByOffsetWide();
    fn GetOffsetByLineWide();
    fn GetModuleByModuleNameWide();
    fn GetSymbolModuleWide();
    fn GetTypeNameWide();
    fn GetTypeIdWide();
    fn GetFieldOffsetWide();
    fn GetSymbolTypeIdWide();
    fn GetScopeSymbolGroup2();
    fn CreateSymbolGroup2();
    fn StartSymbolMatchWide();
    fn GetNextSymbolMatchWide();
    fn ReloadWide();
    fn GetSymbolPathWide();
    fn SetSymbolPathWide();
    fn AppendSymbolPathWide();
    fn GetImagePathWide();
    fn SetImagePathWide();
    fn AppendImagePathWide();
    fn GetSourcePathWide();
    fn GetSourcePathElementWide();
    fn SetSourcePathWide();
    fn AppendSourcePathWide();
    fn FindSourceFileWide();
    fn GetSourceFileLineOffsetsWide();
    fn GetModuleVersionInformationWide();
    fn GetModuleNameStringWide();
    fn GetConstantNameWide();
    fn GetFieldNameWide();
    fn IsManagedModule();
    fn GetModuleByModuleName2();
    fn GetModuleByModuleName2Wide();
    fn GetModuleByOffset2();
    fn AddSyntheticModule();
    fn AddSyntheticModuleWide();
    fn RemoveSyntheticModule();
    fn GetCurrentScopeFrameIndex();
    fn SetScopeFrameByIndex();
    fn SetScopeFromJitDebugInfo();
    fn SetScopeFromStoredEvent();
    fn OutputSymbolByOffset();
    fn GetFunctionEntryByOffset();
    fn GetFieldTypeAndOffset();
    fn GetFieldTypeAndOffsetWide();
    fn AddSyntheticSymbol();
    fn AddSyntheticSymbolWide();
    fn RemoveSyntheticSymbol();
    fn GetSymbolEntriesByOffset();
    fn GetSymbolEntriesByName();
    fn GetSymbolEntriesByNameWide();
    fn GetSymbolEntryByToken();
    fn GetSymbolEntryInformation();
    fn GetSymbolEntryString();
    fn GetSymbolEntryStringWide();
    fn GetSymbolEntryOffsetRegions();
    fn GetSymbolEntryBySymbolEntry();
    fn GetSourceEntriesByOffset();
    fn GetSourceEntriesByLine();
    fn GetSourceEntriesByLineWide();
    fn GetSourceEntryString();
    fn GetSourceEntryStringWide();
    fn GetSourceEntryOffsetRegions();
    fn GetSourceEntryBySourceEntry();
    fn GetScopeEx();
    fn SetScopeEx();
    fn GetNameByInlineContext();
    fn GetNameByInlineContextWide();
    fn GetLineByInlineContext();
    fn GetLineByInlineContextWide();
    fn OutputSymbolByInlineContext();
    fn GetCurrentScopeFrameIndexEx();
    fn SetScopeFrameByIndexEx();
}
pub trait IDebugSyncOperationImpl: Sized {
    fn GetTargetThread();
    fn Execute();
    fn InProgressAbort();
}
pub trait IDebugSystemObjectsImpl: Sized {
    fn GetEventThread();
    fn GetEventProcess();
    fn GetCurrentThreadId();
    fn SetCurrentThreadId();
    fn GetCurrentProcessId();
    fn SetCurrentProcessId();
    fn GetNumberThreads();
    fn GetTotalNumberThreads();
    fn GetThreadIdsByIndex();
    fn GetThreadIdByProcessor();
    fn GetCurrentThreadDataOffset();
    fn GetThreadIdByDataOffset();
    fn GetCurrentThreadTeb();
    fn GetThreadIdByTeb();
    fn GetCurrentThreadSystemId();
    fn GetThreadIdBySystemId();
    fn GetCurrentThreadHandle();
    fn GetThreadIdByHandle();
    fn GetNumberProcesses();
    fn GetProcessIdsByIndex();
    fn GetCurrentProcessDataOffset();
    fn GetProcessIdByDataOffset();
    fn GetCurrentProcessPeb();
    fn GetProcessIdByPeb();
    fn GetCurrentProcessSystemId();
    fn GetProcessIdBySystemId();
    fn GetCurrentProcessHandle();
    fn GetProcessIdByHandle();
    fn GetCurrentProcessExecutableName();
}
pub trait IDebugSystemObjects2Impl: Sized {
    fn GetEventThread();
    fn GetEventProcess();
    fn GetCurrentThreadId();
    fn SetCurrentThreadId();
    fn GetCurrentProcessId();
    fn SetCurrentProcessId();
    fn GetNumberThreads();
    fn GetTotalNumberThreads();
    fn GetThreadIdsByIndex();
    fn GetThreadIdByProcessor();
    fn GetCurrentThreadDataOffset();
    fn GetThreadIdByDataOffset();
    fn GetCurrentThreadTeb();
    fn GetThreadIdByTeb();
    fn GetCurrentThreadSystemId();
    fn GetThreadIdBySystemId();
    fn GetCurrentThreadHandle();
    fn GetThreadIdByHandle();
    fn GetNumberProcesses();
    fn GetProcessIdsByIndex();
    fn GetCurrentProcessDataOffset();
    fn GetProcessIdByDataOffset();
    fn GetCurrentProcessPeb();
    fn GetProcessIdByPeb();
    fn GetCurrentProcessSystemId();
    fn GetProcessIdBySystemId();
    fn GetCurrentProcessHandle();
    fn GetProcessIdByHandle();
    fn GetCurrentProcessExecutableName();
    fn GetCurrentProcessUpTime();
    fn GetImplicitThreadDataOffset();
    fn SetImplicitThreadDataOffset();
    fn GetImplicitProcessDataOffset();
    fn SetImplicitProcessDataOffset();
}
pub trait IDebugSystemObjects3Impl: Sized {
    fn GetEventThread();
    fn GetEventProcess();
    fn GetCurrentThreadId();
    fn SetCurrentThreadId();
    fn GetCurrentProcessId();
    fn SetCurrentProcessId();
    fn GetNumberThreads();
    fn GetTotalNumberThreads();
    fn GetThreadIdsByIndex();
    fn GetThreadIdByProcessor();
    fn GetCurrentThreadDataOffset();
    fn GetThreadIdByDataOffset();
    fn GetCurrentThreadTeb();
    fn GetThreadIdByTeb();
    fn GetCurrentThreadSystemId();
    fn GetThreadIdBySystemId();
    fn GetCurrentThreadHandle();
    fn GetThreadIdByHandle();
    fn GetNumberProcesses();
    fn GetProcessIdsByIndex();
    fn GetCurrentProcessDataOffset();
    fn GetProcessIdByDataOffset();
    fn GetCurrentProcessPeb();
    fn GetProcessIdByPeb();
    fn GetCurrentProcessSystemId();
    fn GetProcessIdBySystemId();
    fn GetCurrentProcessHandle();
    fn GetProcessIdByHandle();
    fn GetCurrentProcessExecutableName();
    fn GetCurrentProcessUpTime();
    fn GetImplicitThreadDataOffset();
    fn SetImplicitThreadDataOffset();
    fn GetImplicitProcessDataOffset();
    fn SetImplicitProcessDataOffset();
    fn GetEventSystem();
    fn GetCurrentSystemId();
    fn SetCurrentSystemId();
    fn GetNumberSystems();
    fn GetSystemIdsByIndex();
    fn GetTotalNumberThreadsAndProcesses();
    fn GetCurrentSystemServer();
    fn GetSystemByServer();
    fn GetCurrentSystemServerName();
}
pub trait IDebugSystemObjects4Impl: Sized {
    fn GetEventThread();
    fn GetEventProcess();
    fn GetCurrentThreadId();
    fn SetCurrentThreadId();
    fn GetCurrentProcessId();
    fn SetCurrentProcessId();
    fn GetNumberThreads();
    fn GetTotalNumberThreads();
    fn GetThreadIdsByIndex();
    fn GetThreadIdByProcessor();
    fn GetCurrentThreadDataOffset();
    fn GetThreadIdByDataOffset();
    fn GetCurrentThreadTeb();
    fn GetThreadIdByTeb();
    fn GetCurrentThreadSystemId();
    fn GetThreadIdBySystemId();
    fn GetCurrentThreadHandle();
    fn GetThreadIdByHandle();
    fn GetNumberProcesses();
    fn GetProcessIdsByIndex();
    fn GetCurrentProcessDataOffset();
    fn GetProcessIdByDataOffset();
    fn GetCurrentProcessPeb();
    fn GetProcessIdByPeb();
    fn GetCurrentProcessSystemId();
    fn GetProcessIdBySystemId();
    fn GetCurrentProcessHandle();
    fn GetProcessIdByHandle();
    fn GetCurrentProcessExecutableName();
    fn GetCurrentProcessUpTime();
    fn GetImplicitThreadDataOffset();
    fn SetImplicitThreadDataOffset();
    fn GetImplicitProcessDataOffset();
    fn SetImplicitProcessDataOffset();
    fn GetEventSystem();
    fn GetCurrentSystemId();
    fn SetCurrentSystemId();
    fn GetNumberSystems();
    fn GetSystemIdsByIndex();
    fn GetTotalNumberThreadsAndProcesses();
    fn GetCurrentSystemServer();
    fn GetSystemByServer();
    fn GetCurrentSystemServerName();
    fn GetCurrentProcessExecutableNameWide();
    fn GetCurrentSystemServerNameWide();
}
pub trait IDebugThreadCall32Impl: Sized {
    fn ThreadCallHandler();
}
pub trait IDebugThreadCall64Impl: Sized {
    fn ThreadCallHandler();
}
pub trait IDynamicConceptProviderConceptImpl: Sized {
    fn GetConcept();
    fn SetConcept();
    fn NotifyParent();
    fn NotifyParentChange();
    fn NotifyDestruct();
}
pub trait IDynamicKeyProviderConceptImpl: Sized {
    fn GetKey();
    fn SetKey();
    fn EnumerateKeys();
}
pub trait IEnumDebugApplicationNodesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDebugCodeContextsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDebugExpressionContextsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDebugExtendedPropertyInfoImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumDebugPropertyInfoImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumDebugStackFramesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDebugStackFrames64Impl: Sized + IEnumDebugStackFramesImpl {
    fn Next64();
}
pub trait IEnumJsStackFramesImpl: Sized {
    fn Next();
    fn Reset();
}
pub trait IEnumRemoteDebugApplicationThreadsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumRemoteDebugApplicationsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEquatableConceptImpl: Sized {
    fn AreObjectsEqual();
}
pub trait IHostDataModelAccessImpl: Sized {
    fn GetDataModel();
}
pub trait IIndexableConceptImpl: Sized {
    fn GetDimensionality();
    fn GetAt();
    fn SetAt();
}
pub trait IIterableConceptImpl: Sized {
    fn GetDefaultIndexDimensionality();
    fn GetIterator();
}
pub trait IJsDebugImpl: Sized {
    fn OpenVirtualProcess();
}
pub trait IJsDebugBreakPointImpl: Sized {
    fn IsEnabled();
    fn Enable();
    fn Disable();
    fn Delete();
    fn GetDocumentPosition();
}
pub trait IJsDebugDataTargetImpl: Sized {
    fn ReadMemory();
    fn WriteMemory();
    fn AllocateVirtualMemory();
    fn FreeVirtualMemory();
    fn GetTlsValue();
    fn ReadBSTR();
    fn ReadNullTerminatedString();
    fn CreateStackFrameEnumerator();
    fn GetThreadContext();
}
pub trait IJsDebugFrameImpl: Sized {
    fn GetStackRange();
    fn GetName();
    fn GetDocumentPositionWithId();
    fn GetDocumentPositionWithName();
    fn GetDebugProperty();
    fn GetReturnAddress();
    fn Evaluate();
}
pub trait IJsDebugProcessImpl: Sized {
    fn CreateStackWalker();
    fn CreateBreakPoint();
    fn PerformAsyncBreak();
    fn GetExternalStepAddress();
}
pub trait IJsDebugPropertyImpl: Sized {
    fn GetPropertyInfo();
    fn GetMembers();
}
pub trait IJsDebugStackWalkerImpl: Sized {
    fn GetNext();
}
pub trait IJsEnumDebugPropertyImpl: Sized {
    fn Next();
    fn GetCount();
}
pub trait IKeyEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IKeyStoreImpl: Sized {
    fn GetKey();
    fn SetKey();
    fn GetKeyValue();
    fn SetKeyValue();
    fn ClearKeys();
}
pub trait IMachineDebugManagerImpl: Sized {
    fn AddApplication();
    fn RemoveApplication();
    fn EnumApplications();
}
pub trait IMachineDebugManagerCookieImpl: Sized {
    fn AddApplication();
    fn RemoveApplication();
    fn EnumApplications();
}
pub trait IMachineDebugManagerEventsImpl: Sized {
    fn onAddApplication();
    fn onRemoveApplication();
}
pub trait IModelIteratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IModelKeyReferenceImpl: Sized {
    fn GetKeyName();
    fn GetOriginalObject();
    fn GetContextObject();
    fn GetKey();
    fn GetKeyValue();
    fn SetKey();
    fn SetKeyValue();
}
pub trait IModelKeyReference2Impl: Sized + IModelKeyReferenceImpl {
    fn OverrideContextObject();
}
pub trait IModelMethodImpl: Sized {
    fn Call();
}
pub trait IModelObjectImpl: Sized {
    fn GetContext();
    fn GetKind();
    fn GetIntrinsicValue();
    fn GetIntrinsicValueAs();
    fn GetKeyValue();
    fn SetKeyValue();
    fn EnumerateKeyValues();
    fn GetRawValue();
    fn EnumerateRawValues();
    fn Dereference();
    fn TryCastToRuntimeType();
    fn GetConcept();
    fn GetLocation();
    fn GetTypeInfo();
    fn GetTargetInfo();
    fn GetNumberOfParentModels();
    fn GetParentModel();
    fn AddParentModel();
    fn RemoveParentModel();
    fn GetKey();
    fn GetKeyReference();
    fn SetKey();
    fn ClearKeys();
    fn EnumerateKeys();
    fn EnumerateKeyReferences();
    fn SetConcept();
    fn ClearConcepts();
    fn GetRawReference();
    fn EnumerateRawReferences();
    fn SetContextForDataModel();
    fn GetContextForDataModel();
    fn Compare();
    fn IsEqualTo();
}
pub trait IModelPropertyAccessorImpl: Sized {
    fn GetValue();
    fn SetValue();
}
pub trait IObjectSafetyImpl: Sized {
    fn GetInterfaceSafetyOptions();
    fn SetInterfaceSafetyOptions();
}
pub trait IPerPropertyBrowsing2Impl: Sized {
    fn GetDisplayString();
    fn MapPropertyToPage();
    fn GetPredefinedStrings();
    fn SetPredefinedValue();
}
pub trait IPreferredRuntimeTypeConceptImpl: Sized {
    fn CastToPreferredRuntimeType();
}
pub trait IProcessDebugManager32Impl: Sized {
    fn CreateApplication();
    fn GetDefaultApplication();
    fn AddApplication();
    fn RemoveApplication();
    fn CreateDebugDocumentHelper();
}
pub trait IProcessDebugManager64Impl: Sized {
    fn CreateApplication();
    fn GetDefaultApplication();
    fn AddApplication();
    fn RemoveApplication();
    fn CreateDebugDocumentHelper();
}
pub trait IProvideExpressionContextsImpl: Sized {
    fn EnumExpressionContexts();
}
pub trait IRawEnumeratorImpl: Sized {
    fn Reset();
    fn GetNext();
}
pub trait IRemoteDebugApplicationImpl: Sized {
    fn ResumeFromBreakPoint();
    fn CauseBreak();
    fn ConnectDebugger();
    fn DisconnectDebugger();
    fn GetDebugger();
    fn CreateInstanceAtApplication();
    fn QueryAlive();
    fn EnumThreads();
    fn GetName();
    fn GetRootNode();
    fn EnumGlobalExpressionContexts();
}
pub trait IRemoteDebugApplication110Impl: Sized {
    fn SetDebuggerOptions();
    fn GetCurrentDebuggerOptions();
    fn GetMainThread();
}
pub trait IRemoteDebugApplicationEventsImpl: Sized {
    fn OnConnectDebugger();
    fn OnDisconnectDebugger();
    fn OnSetName();
    fn OnDebugOutput();
    fn OnClose();
    fn OnEnterBreakPoint();
    fn OnLeaveBreakPoint();
    fn OnCreateThread();
    fn OnDestroyThread();
    fn OnBreakFlagChange();
}
pub trait IRemoteDebugApplicationThreadImpl: Sized {
    fn GetSystemThreadId();
    fn GetApplication();
    fn EnumStackFrames();
    fn GetDescription();
    fn SetNextStatement();
    fn GetState();
    fn Suspend();
    fn Resume();
    fn GetSuspendCount();
}
pub trait IRemoteDebugCriticalErrorEvent110Impl: Sized {
    fn GetErrorInfo();
}
pub trait IRemoteDebugInfoEvent110Impl: Sized {
    fn GetEventInfo();
}
pub trait IScriptEntryImpl: Sized + IScriptNodeImpl {
    fn GetText();
    fn SetText();
    fn GetBody();
    fn SetBody();
    fn GetName();
    fn SetName();
    fn GetItemName();
    fn SetItemName();
    fn GetSignature();
    fn SetSignature();
    fn GetRange();
}
pub trait IScriptInvocationContextImpl: Sized {
    fn GetContextType();
    fn GetContextDescription();
    fn GetContextObject();
}
pub trait IScriptNodeImpl: Sized {
    fn Alive();
    fn Delete();
    fn GetParent();
    fn GetIndexInParent();
    fn GetCookie();
    fn GetNumberOfChildren();
    fn GetChild();
    fn GetLanguage();
    fn CreateChildEntry();
    fn CreateChildHandler();
}
pub trait IScriptScriptletImpl: Sized + IScriptEntryImpl + IScriptNodeImpl {
    fn GetSubItemName();
    fn SetSubItemName();
    fn GetEventName();
    fn SetEventName();
    fn GetSimpleEventName();
    fn SetSimpleEventName();
}
pub trait ISimpleConnectionPointImpl: Sized {
    fn GetEventCount();
    fn DescribeEvents();
    fn Advise();
    fn Unadvise();
}
pub trait IStringDisplayableConceptImpl: Sized {
    fn ToDisplayString();
}
pub trait ITridentEventSinkImpl: Sized {
    fn FireEvent();
}
pub trait IWebAppDiagnosticsObjectInitializationImpl: Sized {
    fn Initialize();
}
pub trait IWebAppDiagnosticsSetupImpl: Sized {
    fn DiagnosticsSupported();
    fn CreateObjectWithSiteAtWebApp();
}
