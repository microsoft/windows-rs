pub trait IEnumWbemClassObjectImpl: Sized {
    fn Reset();
    fn Next();
    fn NextAsync();
    fn Clone();
    fn Skip();
}
pub trait IMofCompilerImpl: Sized {
    fn CompileFile();
    fn CompileBuffer();
    fn CreateBMOF();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemDateTimeImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Year();
    fn SetYear();
    fn YearSpecified();
    fn SetYearSpecified();
    fn Month();
    fn SetMonth();
    fn MonthSpecified();
    fn SetMonthSpecified();
    fn Day();
    fn SetDay();
    fn DaySpecified();
    fn SetDaySpecified();
    fn Hours();
    fn SetHours();
    fn HoursSpecified();
    fn SetHoursSpecified();
    fn Minutes();
    fn SetMinutes();
    fn MinutesSpecified();
    fn SetMinutesSpecified();
    fn Seconds();
    fn SetSeconds();
    fn SecondsSpecified();
    fn SetSecondsSpecified();
    fn Microseconds();
    fn SetMicroseconds();
    fn MicrosecondsSpecified();
    fn SetMicrosecondsSpecified();
    fn UTC();
    fn SetUTC();
    fn UTCSpecified();
    fn SetUTCSpecified();
    fn IsInterval();
    fn SetIsInterval();
    fn GetVarDate();
    fn SetVarDate();
    fn GetFileTime();
    fn SetFileTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemEventSourceImpl: Sized + IDispatchImpl {
    fn NextEvent();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLastErrorImpl: Sized + ISWbemObjectImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLocatorImpl: Sized + IDispatchImpl {
    fn ConnectServer();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethodImpl: Sized + IDispatchImpl {
    fn Name();
    fn Origin();
    fn InParameters();
    fn OutParameters();
    fn Qualifiers_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethodSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValueImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValueSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn Clone();
    fn DeleteAll();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectImpl: Sized + IDispatchImpl {
    fn Put_();
    fn PutAsync_();
    fn Delete_();
    fn DeleteAsync_();
    fn Instances_();
    fn InstancesAsync_();
    fn Subclasses_();
    fn SubclassesAsync_();
    fn Associators_();
    fn AssociatorsAsync_();
    fn References_();
    fn ReferencesAsync_();
    fn ExecMethod_();
    fn ExecMethodAsync_();
    fn Clone_();
    fn GetObjectText_();
    fn SpawnDerivedClass_();
    fn SpawnInstance_();
    fn CompareTo_();
    fn Qualifiers_();
    fn Properties_();
    fn Methods_();
    fn Derivation_();
    fn Path_();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectExImpl: Sized + ISWbemObjectImpl + IDispatchImpl {
    fn Refresh_();
    fn SystemProperties_();
    fn GetText_();
    fn SetFromText_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectPathImpl: Sized + IDispatchImpl {
    fn Path();
    fn SetPath();
    fn RelPath();
    fn SetRelPath();
    fn Server();
    fn SetServer();
    fn Namespace();
    fn SetNamespace();
    fn ParentNamespace();
    fn DisplayName();
    fn SetDisplayName();
    fn Class();
    fn SetClass();
    fn IsClass();
    fn SetAsClass();
    fn IsSingleton();
    fn SetAsSingleton();
    fn Keys();
    fn Security_();
    fn Locale();
    fn SetLocale();
    fn Authority();
    fn SetAuthority();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Security_();
    fn ItemIndex();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilegeImpl: Sized + IDispatchImpl {
    fn IsEnabled();
    fn SetIsEnabled();
    fn Name();
    fn DisplayName();
    fn Identifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilegeSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn DeleteAll();
    fn AddAsString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
    fn IsLocal();
    fn Origin();
    fn CIMType();
    fn Qualifiers_();
    fn IsArray();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPropertySetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifierImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
    fn IsLocal();
    fn PropagatesToSubclass();
    fn SetPropagatesToSubclass();
    fn PropagatesToInstance();
    fn SetPropagatesToInstance();
    fn IsOverridable();
    fn SetIsOverridable();
    fn IsAmended();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifierSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefreshableItemImpl: Sized + IDispatchImpl {
    fn Index();
    fn Refresher();
    fn IsSet();
    fn Object();
    fn ObjectSet();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefresherImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn AddEnum();
    fn Remove();
    fn Refresh();
    fn AutoReconnect();
    fn SetAutoReconnect();
    fn DeleteAll();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSecurityImpl: Sized + IDispatchImpl {
    fn ImpersonationLevel();
    fn SetImpersonationLevel();
    fn AuthenticationLevel();
    fn SetAuthenticationLevel();
    fn Privileges();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServicesImpl: Sized + IDispatchImpl {
    fn Get();
    fn GetAsync();
    fn Delete();
    fn DeleteAsync();
    fn InstancesOf();
    fn InstancesOfAsync();
    fn SubclassesOf();
    fn SubclassesOfAsync();
    fn ExecQuery();
    fn ExecQueryAsync();
    fn AssociatorsOf();
    fn AssociatorsOfAsync();
    fn ReferencesTo();
    fn ReferencesToAsync();
    fn ExecNotificationQuery();
    fn ExecNotificationQueryAsync();
    fn ExecMethod();
    fn ExecMethodAsync();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServicesExImpl: Sized + ISWbemServicesImpl + IDispatchImpl {
    fn Put();
    fn PutAsync();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSinkImpl: Sized + IDispatchImpl {
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSinkEventsImpl: Sized + IDispatchImpl {}
pub trait IUnsecuredApartmentImpl: Sized {
    fn CreateObjectStub();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMIExtensionImpl: Sized + IDispatchImpl {
    fn WMIObjectPath();
    fn GetWMIObject();
    fn GetWMIServices();
}
pub trait IWbemAddressResolutionImpl: Sized {
    fn Resolve();
}
pub trait IWbemBackupRestoreImpl: Sized {
    fn Backup();
    fn Restore();
}
pub trait IWbemBackupRestoreExImpl: Sized + IWbemBackupRestoreImpl {
    fn Pause();
    fn Resume();
}
pub trait IWbemCallResultImpl: Sized {
    fn GetResultObject();
    fn GetResultString();
    fn GetResultServices();
    fn GetCallStatus();
}
pub trait IWbemClassObjectImpl: Sized {
    fn GetQualifierSet();
    fn Get();
    fn Put();
    fn Delete();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
    fn GetPropertyQualifierSet();
    fn Clone();
    fn GetObjectText();
    fn SpawnDerivedClass();
    fn SpawnInstance();
    fn CompareTo();
    fn GetPropertyOrigin();
    fn InheritsFrom();
    fn GetMethod();
    fn PutMethod();
    fn DeleteMethod();
    fn BeginMethodEnumeration();
    fn NextMethod();
    fn EndMethodEnumeration();
    fn GetMethodQualifierSet();
    fn GetMethodOrigin();
}
pub trait IWbemClientConnectionTransportImpl: Sized {
    fn Open();
    fn OpenAsync();
    fn Cancel();
}
pub trait IWbemClientTransportImpl: Sized {
    fn ConnectServer();
}
pub trait IWbemConfigureRefresherImpl: Sized {
    fn AddObjectByPath();
    fn AddObjectByTemplate();
    fn AddRefresher();
    fn Remove();
    fn AddEnum();
}
pub trait IWbemConnectorLoginImpl: Sized {
    fn ConnectorLogin();
}
pub trait IWbemConstructClassObjectImpl: Sized {
    fn SetInheritanceChain();
    fn SetPropertyOrigin();
    fn SetMethodOrigin();
    fn SetServerNamespace();
}
pub trait IWbemContextImpl: Sized {
    fn Clone();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
    fn SetValue();
    fn GetValue();
    fn DeleteValue();
    fn DeleteAll();
}
pub trait IWbemDecoupledBasicEventProviderImpl: Sized + IWbemDecoupledRegistrarImpl {
    fn GetSink();
    fn GetService();
}
pub trait IWbemDecoupledRegistrarImpl: Sized {
    fn Register();
    fn UnRegister();
}
pub trait IWbemEventConsumerProviderImpl: Sized {
    fn FindConsumer();
}
pub trait IWbemEventProviderImpl: Sized {
    fn ProvideEvents();
}
pub trait IWbemEventProviderQuerySinkImpl: Sized {
    fn NewQuery();
    fn CancelQuery();
}
pub trait IWbemEventProviderSecurityImpl: Sized {
    fn AccessCheck();
}
pub trait IWbemEventSinkImpl: Sized + IWbemObjectSinkImpl {
    fn SetSinkSecurity();
    fn IsActive();
    fn GetRestrictedSink();
    fn SetBatchingParameters();
}
pub trait IWbemHiPerfEnumImpl: Sized {
    fn AddObjects();
    fn RemoveObjects();
    fn GetObjects();
    fn RemoveAll();
}
pub trait IWbemHiPerfProviderImpl: Sized {
    fn QueryInstances();
    fn CreateRefresher();
    fn CreateRefreshableObject();
    fn StopRefreshing();
    fn CreateRefreshableEnum();
    fn GetObjects();
}
pub trait IWbemLevel1LoginImpl: Sized {
    fn EstablishPosition();
    fn RequestChallenge();
    fn WBEMLogin();
    fn NTLMLogin();
}
pub trait IWbemLocatorImpl: Sized {
    fn ConnectServer();
}
pub trait IWbemObjectAccessImpl: Sized + IWbemClassObjectImpl {
    fn GetPropertyHandle();
    fn WritePropertyValue();
    fn ReadPropertyValue();
    fn ReadDWORD();
    fn WriteDWORD();
    fn ReadQWORD();
    fn WriteQWORD();
    fn GetPropertyInfoByHandle();
    fn Lock();
    fn Unlock();
}
pub trait IWbemObjectSinkImpl: Sized {
    fn Indicate();
    fn SetStatus();
}
pub trait IWbemObjectSinkExImpl: Sized + IWbemObjectSinkImpl {
    fn WriteMessage();
    fn WriteError();
    fn PromptUser();
    fn WriteProgress();
    fn WriteStreamParameter();
}
pub trait IWbemObjectTextSrcImpl: Sized {
    fn GetText();
    fn CreateFromText();
}
pub trait IWbemPathImpl: Sized {
    fn SetText();
    fn GetText();
    fn GetInfo();
    fn SetServer();
    fn GetServer();
    fn GetNamespaceCount();
    fn SetNamespaceAt();
    fn GetNamespaceAt();
    fn RemoveNamespaceAt();
    fn RemoveAllNamespaces();
    fn GetScopeCount();
    fn SetScope();
    fn SetScopeFromText();
    fn GetScope();
    fn GetScopeAsText();
    fn RemoveScope();
    fn RemoveAllScopes();
    fn SetClassName();
    fn GetClassName();
    fn GetKeyList();
    fn CreateClassPart();
    fn DeleteClassPart();
    fn IsRelative();
    fn IsRelativeOrChild();
    fn IsLocal();
    fn IsSameClassName();
}
pub trait IWbemPathKeyListImpl: Sized {
    fn GetCount();
    fn SetKey();
    fn SetKey2();
    fn GetKey();
    fn GetKey2();
    fn RemoveKey();
    fn RemoveAllKeys();
    fn MakeSingleton();
    fn GetInfo();
    fn GetText();
}
pub trait IWbemPropertyProviderImpl: Sized {
    fn GetProperty();
    fn PutProperty();
}
pub trait IWbemProviderIdentityImpl: Sized {
    fn SetRegistrationObject();
}
pub trait IWbemProviderInitImpl: Sized {
    fn Initialize();
}
pub trait IWbemProviderInitSinkImpl: Sized {
    fn SetStatus();
}
pub trait IWbemQualifierSetImpl: Sized {
    fn Get();
    fn Put();
    fn Delete();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
}
pub trait IWbemQueryImpl: Sized {
    fn Empty();
    fn SetLanguageFeatures();
    fn TestLanguageFeatures();
    fn Parse();
    fn GetAnalysis();
    fn FreeMemory();
    fn GetQueryInfo();
}
pub trait IWbemRefresherImpl: Sized {
    fn Refresh();
}
pub trait IWbemServicesImpl: Sized {
    fn OpenNamespace();
    fn CancelAsyncCall();
    fn QueryObjectSink();
    fn GetObject();
    fn GetObjectAsync();
    fn PutClass();
    fn PutClassAsync();
    fn DeleteClass();
    fn DeleteClassAsync();
    fn CreateClassEnum();
    fn CreateClassEnumAsync();
    fn PutInstance();
    fn PutInstanceAsync();
    fn DeleteInstance();
    fn DeleteInstanceAsync();
    fn CreateInstanceEnum();
    fn CreateInstanceEnumAsync();
    fn ExecQuery();
    fn ExecQueryAsync();
    fn ExecNotificationQuery();
    fn ExecNotificationQueryAsync();
    fn ExecMethod();
    fn ExecMethodAsync();
}
pub trait IWbemShutdownImpl: Sized {
    fn Shutdown();
}
pub trait IWbemStatusCodeTextImpl: Sized {
    fn GetErrorCodeText();
    fn GetFacilityCodeText();
}
pub trait IWbemTransportImpl: Sized {
    fn Initialize();
}
pub trait IWbemUnboundObjectSinkImpl: Sized {
    fn IndicateToConsumer();
}
pub trait IWbemUnsecuredApartmentImpl: Sized + IUnsecuredApartmentImpl {
    fn CreateSinkStub();
}
