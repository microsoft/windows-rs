pub trait IBindCallbackRedirectImpl: Sized {
    fn Redirect();
}
pub trait IBindHttpSecurityImpl: Sized {
    fn GetIgnoreCertMask();
}
pub trait IBindProtocolImpl: Sized {
    fn CreateBinding();
}
pub trait ICatalogFileInfoImpl: Sized {
    fn GetCatalogFile();
    fn GetJavaTrust();
}
pub trait ICodeInstallImpl: Sized + IWindowForBindingUIImpl {
    fn OnCodeInstallProblem();
}
pub trait IDataFilterImpl: Sized {
    fn DoEncode();
    fn DoDecode();
    fn SetEncodingLevel();
}
pub trait IEncodingFilterFactoryImpl: Sized {
    fn FindBestFilter();
    fn GetDefaultFilter();
}
pub trait IGetBindHandleImpl: Sized {
    fn GetBindHandle();
}
pub trait IHttpNegotiateImpl: Sized {
    fn BeginningTransaction();
    fn OnResponse();
}
pub trait IHttpNegotiate2Impl: Sized + IHttpNegotiateImpl {
    fn GetRootSecurityId();
}
pub trait IHttpNegotiate3Impl: Sized + IHttpNegotiate2Impl + IHttpNegotiateImpl {
    fn GetSerializedClientCertContext();
}
pub trait IHttpSecurityImpl: Sized + IWindowForBindingUIImpl {
    fn OnSecurityProblem();
}
pub trait IInternetImpl: Sized {}
pub trait IInternetBindInfoImpl: Sized {
    fn GetBindInfo();
    fn GetBindString();
}
pub trait IInternetBindInfoExImpl: Sized + IInternetBindInfoImpl {
    fn GetBindInfoEx();
}
pub trait IInternetHostSecurityManagerImpl: Sized {
    fn GetSecurityId();
    fn ProcessUrlAction();
    fn QueryCustomPolicy();
}
pub trait IInternetPriorityImpl: Sized {
    fn SetPriority();
    fn GetPriority();
}
pub trait IInternetProtocolImpl: Sized + IInternetProtocolRootImpl {
    fn Read();
    fn Seek();
    fn LockRequest();
    fn UnlockRequest();
}
pub trait IInternetProtocolExImpl: Sized + IInternetProtocolImpl + IInternetProtocolRootImpl {
    fn StartEx();
}
pub trait IInternetProtocolInfoImpl: Sized {
    fn ParseUrl();
    fn CombineUrl();
    fn CompareUrl();
    fn QueryInfo();
}
pub trait IInternetProtocolRootImpl: Sized {
    fn Start();
    fn Continue();
    fn Abort();
    fn Terminate();
    fn Suspend();
    fn Resume();
}
pub trait IInternetProtocolSinkImpl: Sized {
    fn Switch();
    fn ReportProgress();
    fn ReportData();
    fn ReportResult();
}
pub trait IInternetProtocolSinkStackableImpl: Sized {
    fn SwitchSink();
    fn CommitSwitch();
    fn RollbackSwitch();
}
pub trait IInternetSecurityManagerImpl: Sized {
    fn SetSecuritySite();
    fn GetSecuritySite();
    fn MapUrlToZone();
    fn GetSecurityId();
    fn ProcessUrlAction();
    fn QueryCustomPolicy();
    fn SetZoneMapping();
    fn GetZoneMappings();
}
pub trait IInternetSecurityManagerExImpl: Sized + IInternetSecurityManagerImpl {
    fn ProcessUrlActionEx();
}
pub trait IInternetSecurityManagerEx2Impl: Sized + IInternetSecurityManagerExImpl + IInternetSecurityManagerImpl {
    fn MapUrlToZoneEx2();
    fn ProcessUrlActionEx2();
    fn GetSecurityIdEx2();
    fn QueryCustomPolicyEx2();
}
pub trait IInternetSecurityMgrSiteImpl: Sized {
    fn GetWindow();
    fn EnableModeless();
}
pub trait IInternetSessionImpl: Sized {
    fn RegisterNameSpace();
    fn UnregisterNameSpace();
    fn RegisterMimeFilter();
    fn UnregisterMimeFilter();
    fn CreateBinding();
    fn SetSessionOption();
    fn GetSessionOption();
}
pub trait IInternetThreadSwitchImpl: Sized {
    fn Prepare();
    fn Continue();
}
pub trait IInternetZoneManagerImpl: Sized {
    fn GetZoneAttributes();
    fn SetZoneAttributes();
    fn GetZoneCustomPolicy();
    fn SetZoneCustomPolicy();
    fn GetZoneActionPolicy();
    fn SetZoneActionPolicy();
    fn PromptAction();
    fn LogAction();
    fn CreateZoneEnumerator();
    fn GetZoneAt();
    fn DestroyZoneEnumerator();
    fn CopyTemplatePoliciesToZone();
}
pub trait IInternetZoneManagerExImpl: Sized + IInternetZoneManagerImpl {
    fn GetZoneActionPolicyEx();
    fn SetZoneActionPolicyEx();
}
pub trait IInternetZoneManagerEx2Impl: Sized + IInternetZoneManagerExImpl + IInternetZoneManagerImpl {
    fn GetZoneAttributesEx();
    fn GetZoneSecurityState();
    fn GetIESecurityState();
    fn FixUnsecureSettings();
}
pub trait IMonikerPropImpl: Sized {
    fn PutProperty();
}
pub trait IPersistMonikerImpl: Sized {
    fn GetClassID();
    fn IsDirty();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn GetCurMoniker();
}
pub trait ISoftDistExtImpl: Sized {
    fn ProcessSoftDist();
    fn GetFirstCodeBase();
    fn GetNextCodeBase();
    fn AsyncInstallDistributionUnit();
}
pub trait IUriBuilderFactoryImpl: Sized {
    fn CreateIUriBuilder();
    fn CreateInitializedIUriBuilder();
}
pub trait IUriContainerImpl: Sized {
    fn GetIUri();
}
pub trait IWinInetCacheHintsImpl: Sized {
    fn SetCacheExtension();
}
pub trait IWinInetCacheHints2Impl: Sized + IWinInetCacheHintsImpl {
    fn SetCacheExtension2();
}
pub trait IWinInetFileStreamImpl: Sized {
    fn SetHandleForUnlock();
    fn SetDeleteFile();
}
pub trait IWinInetHttpInfoImpl: Sized + IWinInetInfoImpl {
    fn QueryInfo();
}
pub trait IWinInetHttpTimeoutsImpl: Sized {
    fn GetRequestTimeouts();
}
pub trait IWinInetInfoImpl: Sized {
    fn QueryOption();
}
pub trait IWindowForBindingUIImpl: Sized {
    fn GetWindow();
}
pub trait IWrappedProtocolImpl: Sized {
    fn GetWrapperCode();
}
pub trait IZoneIdentifierImpl: Sized {
    fn GetId();
    fn SetId();
    fn Remove();
}
pub trait IZoneIdentifier2Impl: Sized + IZoneIdentifierImpl {
    fn GetLastWriterPackageFamilyName();
    fn SetLastWriterPackageFamilyName();
    fn RemoveLastWriterPackageFamilyName();
    fn GetAppZoneId();
    fn SetAppZoneId();
    fn RemoveAppZoneId();
}
