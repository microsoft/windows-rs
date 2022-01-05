pub trait AsyncIAdviseSinkImpl: Sized {
    fn Begin_OnDataChange();
    fn Finish_OnDataChange();
    fn Begin_OnViewChange();
    fn Finish_OnViewChange();
    fn Begin_OnRename();
    fn Finish_OnRename();
    fn Begin_OnSave();
    fn Finish_OnSave();
    fn Begin_OnClose();
    fn Finish_OnClose();
}
pub trait AsyncIAdviseSink2Impl: Sized + AsyncIAdviseSinkImpl {
    fn Begin_OnLinkSrcChange();
    fn Finish_OnLinkSrcChange();
}
pub trait AsyncIMultiQIImpl: Sized {
    fn Begin_QueryMultipleInterfaces();
    fn Finish_QueryMultipleInterfaces();
}
pub trait AsyncIPipeByteImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
pub trait AsyncIPipeDoubleImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
pub trait AsyncIPipeLongImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
pub trait AsyncIUnknownImpl: Sized {
    fn Begin_QueryInterface();
    fn Finish_QueryInterface();
    fn Begin_AddRef();
    fn Finish_AddRef();
    fn Begin_Release();
    fn Finish_Release();
}
pub trait IActivationFilterImpl: Sized {
    fn HandleActivation();
}
pub trait IAddrExclusionControlImpl: Sized {
    fn GetCurrentAddrExclusionList();
    fn UpdateAddrExclusionList();
}
pub trait IAddrTrackingControlImpl: Sized {
    fn EnableCOMDynamicAddrTracking();
    fn DisableCOMDynamicAddrTracking();
}
pub trait IAdviseSinkImpl: Sized {
    fn OnDataChange();
    fn OnViewChange();
    fn OnRename();
    fn OnSave();
    fn OnClose();
}
pub trait IAdviseSink2Impl: Sized + IAdviseSinkImpl {
    fn OnLinkSrcChange();
}
pub trait IAgileObjectImpl: Sized {}
pub trait IAsyncManagerImpl: Sized {
    fn CompleteCall();
    fn GetCallContext();
    fn GetState();
}
pub trait IAsyncRpcChannelBufferImpl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn GetDestCtxEx();
}
pub trait IAuthenticateImpl: Sized {
    fn Authenticate();
}
pub trait IAuthenticateExImpl: Sized + IAuthenticateImpl {
    fn AuthenticateEx();
}
pub trait IBindCtxImpl: Sized {
    fn RegisterObjectBound();
    fn RevokeObjectBound();
    fn ReleaseBoundObjects();
    fn SetBindOptions();
    fn GetBindOptions();
    fn GetRunningObjectTable();
    fn RegisterObjectParam();
    fn GetObjectParam();
    fn EnumObjectParam();
    fn RevokeObjectParam();
}
pub trait IBindHostImpl: Sized {
    fn CreateMoniker();
    fn MonikerBindToStorage();
    fn MonikerBindToObject();
}
pub trait IBindStatusCallbackImpl: Sized {
    fn OnStartBinding();
    fn GetPriority();
    fn OnLowResource();
    fn OnProgress();
    fn OnStopBinding();
    fn GetBindInfo();
    fn OnDataAvailable();
    fn OnObjectAvailable();
}
pub trait IBindStatusCallbackExImpl: Sized + IBindStatusCallbackImpl {
    fn GetBindInfoEx();
}
pub trait IBindingImpl: Sized {
    fn Abort();
    fn Suspend();
    fn Resume();
    fn SetPriority();
    fn GetPriority();
    fn GetBindResult();
}
pub trait IBlockingLockImpl: Sized {
    fn Lock();
    fn Unlock();
}
pub trait ICallFactoryImpl: Sized {
    fn CreateCall();
}
pub trait ICancelMethodCallsImpl: Sized {
    fn Cancel();
    fn TestCancel();
}
pub trait ICatInformationImpl: Sized {
    fn EnumCategories();
    fn GetCategoryDesc();
    fn EnumClassesOfCategories();
    fn IsClassOfCategories();
    fn EnumImplCategoriesOfClass();
    fn EnumReqCategoriesOfClass();
}
pub trait ICatRegisterImpl: Sized {
    fn RegisterCategories();
    fn UnRegisterCategories();
    fn RegisterClassImplCategories();
    fn UnRegisterClassImplCategories();
    fn RegisterClassReqCategories();
    fn UnRegisterClassReqCategories();
}
pub trait IChannelHookImpl: Sized {
    fn ClientGetSize();
    fn ClientFillBuffer();
    fn ClientNotify();
    fn ServerNotify();
    fn ServerGetSize();
    fn ServerFillBuffer();
}
pub trait IClassActivatorImpl: Sized {
    fn GetClassObject();
}
pub trait IClassFactoryImpl: Sized {
    fn CreateInstance();
    fn LockServer();
}
pub trait IClientSecurityImpl: Sized {
    fn QueryBlanket();
    fn SetBlanket();
    fn CopyProxy();
}
pub trait IComThreadingInfoImpl: Sized {
    fn GetCurrentApartmentType();
    fn GetCurrentThreadType();
    fn GetCurrentLogicalThreadId();
    fn SetCurrentLogicalThreadId();
}
pub trait IConnectionPointImpl: Sized {
    fn GetConnectionInterface();
    fn GetConnectionPointContainer();
    fn Advise();
    fn Unadvise();
    fn EnumConnections();
}
pub trait IConnectionPointContainerImpl: Sized {
    fn EnumConnectionPoints();
    fn FindConnectionPoint();
}
pub trait IContextCallbackImpl: Sized {
    fn ContextCallback();
}
pub trait IDataAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnDataChange();
}
pub trait IDataObjectImpl: Sized {
    fn GetData();
    fn GetDataHere();
    fn QueryGetData();
    fn GetCanonicalFormatEtc();
    fn SetData();
    fn EnumFormatEtc();
    fn DAdvise();
    fn DUnadvise();
    fn EnumDAdvise();
}
pub trait IDispatchImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetIDsOfNames();
    fn Invoke();
}
pub trait IEnumCATEGORYINFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumConnectionPointsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumConnectionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumFORMATETCImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumGUIDImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumMonikerImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSTATDATAImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumStringImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumUnknownImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IErrorInfoImpl: Sized {
    fn GetGUID();
    fn GetSource();
    fn GetDescription();
    fn GetHelpFile();
    fn GetHelpContext();
}
pub trait IErrorLogImpl: Sized {
    fn AddError();
}
pub trait IExternalConnectionImpl: Sized {
    fn AddConnection();
    fn ReleaseConnection();
}
pub trait IFastRundownImpl: Sized {}
pub trait IForegroundTransferImpl: Sized {
    fn AllowForegroundTransfer();
}
pub trait IGlobalInterfaceTableImpl: Sized {
    fn RegisterInterfaceInGlobal();
    fn RevokeInterfaceFromGlobal();
    fn GetInterfaceFromGlobal();
}
pub trait IGlobalOptionsImpl: Sized {
    fn Set();
    fn Query();
}
pub trait IInitializeSpyImpl: Sized {
    fn PreInitialize();
    fn PostInitialize();
    fn PreUninitialize();
    fn PostUninitialize();
}
pub trait IInternalUnknownImpl: Sized {
    fn QueryInternalInterface();
}
pub trait IMachineGlobalObjectTableImpl: Sized {
    fn RegisterObject();
    fn GetObject();
    fn RevokeObject();
}
pub trait IMallocImpl: Sized {
    fn Alloc();
    fn Realloc();
    fn Free();
    fn GetSize();
    fn DidAlloc();
    fn HeapMinimize();
}
pub trait IMallocSpyImpl: Sized {
    fn PreAlloc();
    fn PostAlloc();
    fn PreFree();
    fn PostFree();
    fn PreRealloc();
    fn PostRealloc();
    fn PreGetSize();
    fn PostGetSize();
    fn PreDidAlloc();
    fn PostDidAlloc();
    fn PreHeapMinimize();
    fn PostHeapMinimize();
}
pub trait IMonikerImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn BindToObject();
    fn BindToStorage();
    fn Reduce();
    fn ComposeWith();
    fn Enum();
    fn IsEqual();
    fn Hash();
    fn IsRunning();
    fn GetTimeOfLastChange();
    fn Inverse();
    fn CommonPrefixWith();
    fn RelativePathTo();
    fn GetDisplayName();
    fn ParseDisplayName();
    fn IsSystemMoniker();
}
pub trait IMultiQIImpl: Sized {
    fn QueryMultipleInterfaces();
}
pub trait INoMarshalImpl: Sized {}
pub trait IOplockStorageImpl: Sized {
    fn CreateStorageEx();
    fn OpenStorageEx();
}
pub trait IPSFactoryBufferImpl: Sized {
    fn CreateProxy();
    fn CreateStub();
}
pub trait IPersistImpl: Sized {
    fn GetClassID();
}
pub trait IPersistFileImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn GetCurFile();
}
pub trait IPersistMemoryImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
pub trait IPersistStreamImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
}
pub trait IPersistStreamInitImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
pub trait IPipeByteImpl: Sized {
    fn Pull();
    fn Push();
}
pub trait IPipeDoubleImpl: Sized {
    fn Pull();
    fn Push();
}
pub trait IPipeLongImpl: Sized {
    fn Pull();
    fn Push();
}
pub trait IProcessInitControlImpl: Sized {
    fn ResetInitializerTimeout();
}
pub trait IProcessLockImpl: Sized {
    fn AddRefOnProcess();
    fn ReleaseRefOnProcess();
}
pub trait IProgressNotifyImpl: Sized {
    fn OnProgress();
}
pub trait IROTDataImpl: Sized {
    fn GetComparisonData();
}
pub trait IReleaseMarshalBuffersImpl: Sized {
    fn ReleaseMarshalBuffer();
}
pub trait IRpcChannelBufferImpl: Sized {
    fn GetBuffer();
    fn SendReceive();
    fn FreeBuffer();
    fn GetDestCtx();
    fn IsConnected();
}
pub trait IRpcChannelBuffer2Impl: Sized + IRpcChannelBufferImpl {
    fn GetProtocolVersion();
}
pub trait IRpcChannelBuffer3Impl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn Cancel();
    fn GetCallContext();
    fn GetDestCtxEx();
    fn GetState();
    fn RegisterAsync();
}
pub trait IRpcHelperImpl: Sized {
    fn GetDCOMProtocolVersion();
    fn GetIIDFromOBJREF();
}
pub trait IRpcOptionsImpl: Sized {
    fn Set();
    fn Query();
}
pub trait IRpcProxyBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
}
pub trait IRpcStubBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Invoke();
    fn IsIIDSupported();
    fn CountRefs();
    fn DebugServerQueryInterface();
    fn DebugServerRelease();
}
pub trait IRpcSyntaxNegotiateImpl: Sized {
    fn NegotiateSyntax();
}
pub trait IRunnableObjectImpl: Sized {
    fn GetRunningClass();
    fn Run();
    fn IsRunning();
    fn LockRunning();
    fn SetContainedObject();
}
pub trait IRunningObjectTableImpl: Sized {
    fn Register();
    fn Revoke();
    fn IsRunning();
    fn GetObject();
    fn NoteChangeTime();
    fn GetTimeOfLastChange();
    fn EnumRunning();
}
pub trait ISequentialStreamImpl: Sized {
    fn Read();
    fn Write();
}
pub trait IServerSecurityImpl: Sized {
    fn QueryBlanket();
    fn ImpersonateClient();
    fn RevertToSelf();
    fn IsImpersonating();
}
pub trait IServiceProviderImpl: Sized {
    fn QueryService();
}
pub trait IStdMarshalInfoImpl: Sized {
    fn GetClassForHandler();
}
pub trait IStreamImpl: Sized + ISequentialStreamImpl {
    fn Seek();
    fn SetSize();
    fn CopyTo();
    fn Commit();
    fn Revert();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
    fn Clone();
}
pub trait ISupportErrorInfoImpl: Sized {
    fn InterfaceSupportsErrorInfo();
}
pub trait ISurrogateImpl: Sized {
    fn LoadDllServer();
    fn FreeSurrogate();
}
pub trait ISurrogateServiceImpl: Sized {
    fn Init();
    fn ApplicationLaunch();
    fn ApplicationFree();
    fn CatalogRefresh();
    fn ProcessShutdown();
}
pub trait ISynchronizeImpl: Sized {
    fn Wait();
    fn Signal();
    fn Reset();
}
pub trait ISynchronizeContainerImpl: Sized {
    fn AddSynchronize();
    fn WaitMultiple();
}
pub trait ISynchronizeEventImpl: Sized + ISynchronizeHandleImpl {
    fn SetEventHandle();
}
pub trait ISynchronizeHandleImpl: Sized {
    fn GetHandle();
}
pub trait ISynchronizeMutexImpl: Sized + ISynchronizeImpl {
    fn ReleaseMutex();
}
pub trait ITimeAndNoticeControlImpl: Sized {
    fn SuppressChanges();
}
pub trait ITypeCompImpl: Sized {
    fn Bind();
    fn BindType();
}
pub trait ITypeInfoImpl: Sized {
    fn GetTypeAttr();
    fn GetTypeComp();
    fn GetFuncDesc();
    fn GetVarDesc();
    fn GetNames();
    fn GetRefTypeOfImplType();
    fn GetImplTypeFlags();
    fn GetIDsOfNames();
    fn Invoke();
    fn GetDocumentation();
    fn GetDllEntry();
    fn GetRefTypeInfo();
    fn AddressOfMember();
    fn CreateInstance();
    fn GetMops();
    fn GetContainingTypeLib();
    fn ReleaseTypeAttr();
    fn ReleaseFuncDesc();
    fn ReleaseVarDesc();
}
pub trait ITypeInfo2Impl: Sized + ITypeInfoImpl {
    fn GetTypeKind();
    fn GetTypeFlags();
    fn GetFuncIndexOfMemId();
    fn GetVarIndexOfMemId();
    fn GetCustData();
    fn GetFuncCustData();
    fn GetParamCustData();
    fn GetVarCustData();
    fn GetImplTypeCustData();
    fn GetDocumentation2();
    fn GetAllCustData();
    fn GetAllFuncCustData();
    fn GetAllParamCustData();
    fn GetAllVarCustData();
    fn GetAllImplTypeCustData();
}
pub trait ITypeLibImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetTypeInfoType();
    fn GetTypeInfoOfGuid();
    fn GetLibAttr();
    fn GetTypeComp();
    fn GetDocumentation();
    fn IsName();
    fn FindName();
    fn ReleaseTLibAttr();
}
pub trait ITypeLib2Impl: Sized + ITypeLibImpl {
    fn GetCustData();
    fn GetLibStatistics();
    fn GetDocumentation2();
    fn GetAllCustData();
}
pub trait ITypeLibRegistrationImpl: Sized {
    fn GetGuid();
    fn GetVersion();
    fn GetLcid();
    fn GetWin32Path();
    fn GetWin64Path();
    fn GetDisplayName();
    fn GetFlags();
    fn GetHelpDir();
}
pub trait ITypeLibRegistrationReaderImpl: Sized {
    fn EnumTypeLibRegistrations();
}
pub trait IUriImpl: Sized {
    fn GetPropertyBSTR();
    fn GetPropertyLength();
    fn GetPropertyDWORD();
    fn HasProperty();
    fn GetAbsoluteUri();
    fn GetAuthority();
    fn GetDisplayUri();
    fn GetDomain();
    fn GetExtension();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPathAndQuery();
    fn GetQuery();
    fn GetRawUri();
    fn GetSchemeName();
    fn GetUserInfo();
    fn GetUserName();
    fn GetHostType();
    fn GetPort();
    fn GetScheme();
    fn GetZone();
    fn GetProperties();
    fn IsEqual();
}
pub trait IUriBuilderImpl: Sized {
    fn CreateUriSimple();
    fn CreateUri();
    fn CreateUriWithFlags();
    fn GetIUri();
    fn SetIUri();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPort();
    fn GetQuery();
    fn GetSchemeName();
    fn GetUserName();
    fn SetFragment();
    fn SetHost();
    fn SetPassword();
    fn SetPath();
    fn SetPort();
    fn SetQuery();
    fn SetSchemeName();
    fn SetUserName();
    fn RemoveProperties();
    fn HasBeenModified();
}
pub trait IUrlMonImpl: Sized {
    fn AsyncGetClassBits();
}
pub trait IWaitMultipleImpl: Sized {
    fn WaitMultiple();
    fn AddSynchronize();
}
