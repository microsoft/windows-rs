#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Wmi`*"]
    pub fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
}
pub struct CIMTYPE_ENUMERATION(i32);
pub struct IEnumWbemClassObject(i32);
pub struct IMofCompiler(i32);
pub struct ISWbemDateTime(i32);
pub struct ISWbemEventSource(i32);
pub struct ISWbemLastError(i32);
pub struct ISWbemLocator(i32);
pub struct ISWbemMethod(i32);
pub struct ISWbemMethodSet(i32);
pub struct ISWbemNamedValue(i32);
pub struct ISWbemNamedValueSet(i32);
pub struct ISWbemObject(i32);
pub struct ISWbemObjectEx(i32);
pub struct ISWbemObjectPath(i32);
pub struct ISWbemObjectSet(i32);
pub struct ISWbemPrivilege(i32);
pub struct ISWbemPrivilegeSet(i32);
pub struct ISWbemProperty(i32);
pub struct ISWbemPropertySet(i32);
pub struct ISWbemQualifier(i32);
pub struct ISWbemQualifierSet(i32);
pub struct ISWbemRefreshableItem(i32);
pub struct ISWbemRefresher(i32);
pub struct ISWbemSecurity(i32);
pub struct ISWbemServices(i32);
pub struct ISWbemServicesEx(i32);
pub struct ISWbemSink(i32);
pub struct ISWbemSinkEvents(i32);
pub struct IUnsecuredApartment(i32);
pub struct IWMIExtension(i32);
pub struct IWbemAddressResolution(i32);
pub struct IWbemBackupRestore(i32);
pub struct IWbemBackupRestoreEx(i32);
pub struct IWbemCallResult(i32);
pub struct IWbemClassObject(i32);
pub struct IWbemClientConnectionTransport(i32);
pub struct IWbemClientTransport(i32);
pub struct IWbemConfigureRefresher(i32);
pub struct IWbemConnectorLogin(i32);
pub struct IWbemConstructClassObject(i32);
pub struct IWbemContext(i32);
pub struct IWbemDecoupledBasicEventProvider(i32);
pub struct IWbemDecoupledRegistrar(i32);
pub struct IWbemEventConsumerProvider(i32);
pub struct IWbemEventProvider(i32);
pub struct IWbemEventProviderQuerySink(i32);
pub struct IWbemEventProviderSecurity(i32);
pub struct IWbemEventSink(i32);
pub struct IWbemHiPerfEnum(i32);
pub struct IWbemHiPerfProvider(i32);
pub struct IWbemLevel1Login(i32);
pub struct IWbemLocator(i32);
pub struct IWbemObjectAccess(i32);
pub struct IWbemObjectSink(i32);
pub struct IWbemObjectSinkEx(i32);
pub struct IWbemObjectTextSrc(i32);
pub struct IWbemPath(i32);
pub struct IWbemPathKeyList(i32);
pub struct IWbemPropertyProvider(i32);
pub struct IWbemProviderIdentity(i32);
pub struct IWbemProviderInit(i32);
pub struct IWbemProviderInitSink(i32);
pub struct IWbemQualifierSet(i32);
pub struct IWbemQuery(i32);
pub struct IWbemRefresher(i32);
pub struct IWbemServices(i32);
pub struct IWbemShutdown(i32);
pub struct IWbemStatusCodeText(i32);
pub struct IWbemTransport(i32);
pub struct IWbemUnboundObjectSink(i32);
pub struct IWbemUnsecuredApartment(i32);
pub struct MI_Application(i32);
pub struct MI_ApplicationFT(i32);
pub struct MI_Array(i32);
pub struct MI_ArrayField(i32);
pub struct MI_BooleanA(i32);
pub struct MI_BooleanAField(i32);
pub struct MI_BooleanField(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_CALL_VERSION: u32 = 1u32;
pub struct MI_CallbackMode(i32);
pub struct MI_CancelCallback(i32);
pub struct MI_CancellationReason(i32);
pub struct MI_Char16A(i32);
pub struct MI_Char16AField(i32);
pub struct MI_Char16Field(i32);
pub struct MI_Class(i32);
pub struct MI_ClassDecl(i32);
pub struct MI_ClassFT(i32);
pub struct MI_ClientFT_V1(i32);
pub struct MI_ConstBooleanA(i32);
pub struct MI_ConstBooleanAField(i32);
pub struct MI_ConstBooleanField(i32);
pub struct MI_ConstChar16A(i32);
pub struct MI_ConstChar16AField(i32);
pub struct MI_ConstChar16Field(i32);
pub struct MI_ConstDatetimeA(i32);
pub struct MI_ConstDatetimeAField(i32);
pub struct MI_ConstDatetimeField(i32);
pub struct MI_ConstInstanceA(i32);
pub struct MI_ConstInstanceAField(i32);
pub struct MI_ConstInstanceField(i32);
pub struct MI_ConstReal32A(i32);
pub struct MI_ConstReal32AField(i32);
pub struct MI_ConstReal32Field(i32);
pub struct MI_ConstReal64A(i32);
pub struct MI_ConstReal64AField(i32);
pub struct MI_ConstReal64Field(i32);
pub struct MI_ConstReferenceA(i32);
pub struct MI_ConstReferenceAField(i32);
pub struct MI_ConstReferenceField(i32);
pub struct MI_ConstSint16A(i32);
pub struct MI_ConstSint16AField(i32);
pub struct MI_ConstSint16Field(i32);
pub struct MI_ConstSint32A(i32);
pub struct MI_ConstSint32AField(i32);
pub struct MI_ConstSint32Field(i32);
pub struct MI_ConstSint64A(i32);
pub struct MI_ConstSint64AField(i32);
pub struct MI_ConstSint64Field(i32);
pub struct MI_ConstSint8A(i32);
pub struct MI_ConstSint8AField(i32);
pub struct MI_ConstSint8Field(i32);
pub struct MI_ConstStringA(i32);
pub struct MI_ConstStringAField(i32);
pub struct MI_ConstStringField(i32);
pub struct MI_ConstUint16A(i32);
pub struct MI_ConstUint16AField(i32);
pub struct MI_ConstUint16Field(i32);
pub struct MI_ConstUint32A(i32);
pub struct MI_ConstUint32AField(i32);
pub struct MI_ConstUint32Field(i32);
pub struct MI_ConstUint64A(i32);
pub struct MI_ConstUint64AField(i32);
pub struct MI_ConstUint64Field(i32);
pub struct MI_ConstUint8A(i32);
pub struct MI_ConstUint8AField(i32);
pub struct MI_ConstUint8Field(i32);
pub struct MI_Context(i32);
pub struct MI_ContextFT(i32);
pub struct MI_Datetime(i32);
pub struct MI_DatetimeA(i32);
pub struct MI_DatetimeAField(i32);
pub struct MI_DatetimeField(i32);
pub struct MI_Deserializer(i32);
pub struct MI_DeserializerFT(i32);
pub struct MI_Deserializer_ClassObjectNeeded(i32);
pub struct MI_DestinationOptions(i32);
pub struct MI_DestinationOptionsFT(i32);
pub struct MI_DestinationOptions_ImpersonationType(i32);
pub struct MI_ErrorCategory(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_ANY: u32 = 127u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_CLASS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_IN: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_INDICATION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_KEY: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_METHOD: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_NULL: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_OUT: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_PARAMETER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_READONLY: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_REFERENCE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_STATIC: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_STREAM: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_FLAG_VERSION: u32 = 469762048u32;
pub struct MI_FeatureDecl(i32);
pub struct MI_Filter(i32);
pub struct MI_FilterFT(i32);
pub struct MI_HostedProvider(i32);
pub struct MI_HostedProviderFT(i32);
pub struct MI_Instance(i32);
pub struct MI_InstanceA(i32);
pub struct MI_InstanceAField(i32);
pub struct MI_InstanceExFT(i32);
pub struct MI_InstanceFT(i32);
pub struct MI_InstanceField(i32);
pub struct MI_Interval(i32);
pub struct MI_LocaleType(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
pub struct MI_MainFunction(i32);
pub struct MI_MethodDecl(i32);
pub struct MI_MethodDecl_Invoke(i32);
pub struct MI_Module(i32);
pub struct MI_Module_Load(i32);
pub struct MI_Module_Self(i32);
pub struct MI_Module_Unload(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
pub struct MI_ObjectDecl(i32);
pub struct MI_Operation(i32);
pub struct MI_OperationCallback_Class(i32);
pub struct MI_OperationCallback_Indication(i32);
pub struct MI_OperationCallback_Instance(i32);
pub struct MI_OperationCallback_PromptUser(i32);
pub struct MI_OperationCallback_ResponseType(i32);
pub struct MI_OperationCallback_StreamedParameter(i32);
pub struct MI_OperationCallback_WriteError(i32);
pub struct MI_OperationCallback_WriteMessage(i32);
pub struct MI_OperationCallback_WriteProgress(i32);
pub struct MI_OperationCallbacks(i32);
pub struct MI_OperationFT(i32);
pub struct MI_OperationOptions(i32);
pub struct MI_OperationOptionsFT(i32);
pub struct MI_ParameterDecl(i32);
pub struct MI_ParameterSet(i32);
pub struct MI_ParameterSetFT(i32);
pub struct MI_PromptType(i32);
pub struct MI_PropertyDecl(i32);
pub struct MI_PropertySet(i32);
pub struct MI_PropertySetFT(i32);
pub struct MI_ProviderArchitecture(i32);
pub struct MI_ProviderFT(i32);
pub struct MI_ProviderFT_AssociatorInstances(i32);
pub struct MI_ProviderFT_CreateInstance(i32);
pub struct MI_ProviderFT_DeleteInstance(i32);
pub struct MI_ProviderFT_DisableIndications(i32);
pub struct MI_ProviderFT_EnableIndications(i32);
pub struct MI_ProviderFT_EnumerateInstances(i32);
pub struct MI_ProviderFT_GetInstance(i32);
pub struct MI_ProviderFT_Invoke(i32);
pub struct MI_ProviderFT_Load(i32);
pub struct MI_ProviderFT_ModifyInstance(i32);
pub struct MI_ProviderFT_ReferenceInstances(i32);
pub struct MI_ProviderFT_Subscribe(i32);
pub struct MI_ProviderFT_Unload(i32);
pub struct MI_ProviderFT_Unsubscribe(i32);
pub struct MI_Qualifier(i32);
pub struct MI_QualifierDecl(i32);
pub struct MI_QualifierSet(i32);
pub struct MI_QualifierSetFT(i32);
pub struct MI_Real32A(i32);
pub struct MI_Real32AField(i32);
pub struct MI_Real32Field(i32);
pub struct MI_Real64A(i32);
pub struct MI_Real64AField(i32);
pub struct MI_Real64Field(i32);
pub struct MI_ReferenceA(i32);
pub struct MI_ReferenceAField(i32);
pub struct MI_ReferenceField(i32);
pub struct MI_Result(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
pub struct MI_SchemaDecl(i32);
pub struct MI_Serializer(i32);
pub struct MI_SerializerFT(i32);
pub struct MI_Server(i32);
pub struct MI_ServerFT(i32);
pub struct MI_Session(i32);
pub struct MI_SessionCallbacks(i32);
pub struct MI_SessionFT(i32);
pub struct MI_Sint16A(i32);
pub struct MI_Sint16AField(i32);
pub struct MI_Sint16Field(i32);
pub struct MI_Sint32A(i32);
pub struct MI_Sint32AField(i32);
pub struct MI_Sint32Field(i32);
pub struct MI_Sint64A(i32);
pub struct MI_Sint64AField(i32);
pub struct MI_Sint64Field(i32);
pub struct MI_Sint8A(i32);
pub struct MI_Sint8AField(i32);
pub struct MI_Sint8Field(i32);
pub struct MI_StringA(i32);
pub struct MI_StringAField(i32);
pub struct MI_StringField(i32);
pub struct MI_SubscriptionDeliveryOptions(i32);
pub struct MI_SubscriptionDeliveryOptionsFT(i32);
pub struct MI_SubscriptionDeliveryType(i32);
pub struct MI_Timestamp(i32);
pub struct MI_Type(i32);
pub struct MI_Uint16A(i32);
pub struct MI_Uint16AField(i32);
pub struct MI_Uint16Field(i32);
pub struct MI_Uint32A(i32);
pub struct MI_Uint32AField(i32);
pub struct MI_Uint32Field(i32);
pub struct MI_Uint64A(i32);
pub struct MI_Uint64AField(i32);
pub struct MI_Uint64Field(i32);
pub struct MI_Uint8A(i32);
pub struct MI_Uint8AField(i32);
pub struct MI_Uint8Field(i32);
pub struct MI_UserCredentials(i32);
pub struct MI_UsernamePasswordCreds(i32);
pub struct MI_UtilitiesFT(i32);
pub struct MI_Value(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub struct MofCompiler(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrix(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrixList(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAssocQueryInf(i32);
pub struct SWbemDateTime(i32);
pub struct SWbemEventSource(i32);
pub struct SWbemLastError(i32);
pub struct SWbemLocator(i32);
pub struct SWbemMethod(i32);
pub struct SWbemMethodSet(i32);
pub struct SWbemNamedValue(i32);
pub struct SWbemNamedValueSet(i32);
pub struct SWbemObject(i32);
pub struct SWbemObjectEx(i32);
pub struct SWbemObjectPath(i32);
pub struct SWbemObjectSet(i32);
pub struct SWbemPrivilege(i32);
pub struct SWbemPrivilegeSet(i32);
pub struct SWbemProperty(i32);
pub struct SWbemPropertySet(i32);
pub struct SWbemQualifier(i32);
pub struct SWbemQualifierSet(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemQueryQualifiedName(i32);
pub struct SWbemRefreshableItem(i32);
pub struct SWbemRefresher(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnConst(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnEncodedQuery(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnQueryToken(i32);
pub struct SWbemRpnTokenList(i32);
pub struct SWbemSecurity(i32);
pub struct SWbemServices(i32);
pub struct SWbemServicesEx(i32);
pub struct SWbemSink(i32);
pub struct UnsecuredApartment(i32);
pub struct WBEMSTATUS(i32);
pub struct WBEMSTATUS_FORMAT(i32);
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Wmi`*"]
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
pub struct WBEM_BACKUP_RESTORE_FLAGS(i32);
pub struct WBEM_BATCH_TYPE(i32);
pub struct WBEM_CHANGE_FLAG_TYPE(i32);
pub struct WBEM_COMPARISON_FLAG(i32);
pub struct WBEM_COMPILER_OPTIONS(i32);
pub struct WBEM_COMPILE_STATUS_INFO(i32);
pub struct WBEM_CONDITION_FLAG_TYPE(i32);
pub struct WBEM_CONNECT_OPTIONS(i32);
pub struct WBEM_EXTRA_RETURN_CODES(i32);
pub struct WBEM_FLAVOR_TYPE(i32);
pub struct WBEM_GENERIC_FLAG_TYPE(i32);
pub struct WBEM_GENUS_TYPE(i32);
pub struct WBEM_GET_KEY_FLAGS(i32);
pub struct WBEM_GET_TEXT_FLAGS(i32);
pub struct WBEM_INFORMATION_FLAG_TYPE(i32);
pub struct WBEM_LIMITATION_FLAG_TYPE(i32);
pub struct WBEM_LIMITS(i32);
pub struct WBEM_LOCKING(i32);
pub struct WBEM_PATH_CREATE_FLAG(i32);
pub struct WBEM_PATH_STATUS_FLAG(i32);
pub struct WBEM_PROVIDER_FLAGS(i32);
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(i32);
pub struct WBEM_QUERY_FLAG_TYPE(i32);
pub struct WBEM_REFRESHER_FLAGS(i32);
pub struct WBEM_SECURITY_FLAGS(i32);
pub struct WBEM_SHUTDOWN_FLAGS(i32);
pub struct WBEM_STATUS_TYPE(i32);
pub struct WBEM_TEXT_FLAG_TYPE(i32);
pub struct WBEM_TIMEOUT_TYPE(i32);
pub struct WBEM_UNSECAPP_FLAG_TYPE(i32);
pub struct WMIExtension(i32);
pub struct WMIQ_ANALYSIS_TYPE(i32);
pub struct WMIQ_ASSOCQ_FLAGS(i32);
pub struct WMIQ_LANGUAGE_FEATURES(i32);
pub struct WMIQ_RPNQ_FEATURE(i32);
pub struct WMIQ_RPN_TOKEN_FLAGS(i32);
pub struct WMI_OBJ_TEXT(i32);
pub struct WbemAdministrativeLocator(i32);
pub struct WbemAuthenticatedLocator(i32);
pub struct WbemAuthenticationLevelEnum(i32);
pub struct WbemBackupRestore(i32);
pub struct WbemChangeFlagEnum(i32);
pub struct WbemCimtypeEnum(i32);
pub struct WbemClassObject(i32);
pub struct WbemComparisonFlagEnum(i32);
pub struct WbemConnectOptionsEnum(i32);
pub struct WbemContext(i32);
pub struct WbemDCOMTransport(i32);
pub struct WbemDecoupledBasicEventProvider(i32);
pub struct WbemDecoupledRegistrar(i32);
pub struct WbemDefPath(i32);
pub struct WbemErrorEnum(i32);
pub struct WbemFlagEnum(i32);
pub struct WbemImpersonationLevelEnum(i32);
pub struct WbemLevel1Login(i32);
pub struct WbemLocalAddrRes(i32);
pub struct WbemLocator(i32);
pub struct WbemObjectTextFormatEnum(i32);
pub struct WbemObjectTextSrc(i32);
pub struct WbemPrivilegeEnum(i32);
pub struct WbemQuery(i32);
pub struct WbemQueryFlagEnum(i32);
pub struct WbemRefresher(i32);
pub struct WbemStatusCodeText(i32);
pub struct WbemTextFlagEnum(i32);
pub struct WbemTimeout(i32);
pub struct WbemUnauthenticatedLocator(i32);
pub struct WbemUninitializedClassObject(i32);
pub struct tag_WBEM_LOGIN_TYPE(i32);
