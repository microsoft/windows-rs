#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
}
#[repr(transparent)]
pub struct CIMTYPE_ENUMERATION(pub i32);
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4095i32);
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(0i32);
pub const CIM_SINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(16i32);
pub const CIM_UINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(17i32);
pub const CIM_SINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(2i32);
pub const CIM_UINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(18i32);
pub const CIM_SINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(3i32);
pub const CIM_UINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(19i32);
pub const CIM_SINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(20i32);
pub const CIM_UINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(21i32);
pub const CIM_REAL32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4i32);
pub const CIM_REAL64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(5i32);
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(11i32);
pub const CIM_STRING: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8i32);
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(101i32);
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(102i32);
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(103i32);
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(13i32);
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8192i32);
#[repr(transparent)]
pub struct IEnumWbemClassObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMofCompiler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemDateTime(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemLastError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemMethod(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemMethodSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemNamedValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemNamedValueSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemObjectEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemObjectPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemObjectSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemPrivilege(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemPrivilegeSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemQualifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemQualifierSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemRefreshableItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemRefresher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemSecurity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemServicesEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISWbemSinkEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnsecuredApartment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMIExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemAddressResolution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemBackupRestore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemBackupRestoreEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemCallResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemClassObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemClientConnectionTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemClientTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemConfigureRefresher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemConnectorLogin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemConstructClassObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemDecoupledBasicEventProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemDecoupledRegistrar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemEventConsumerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemEventProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemEventProviderQuerySink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemEventProviderSecurity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemHiPerfEnum(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemHiPerfProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemLevel1Login(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemObjectAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemObjectSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemObjectSinkEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemObjectTextSrc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemPathKeyList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemPropertyProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemProviderIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemProviderInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemProviderInitSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemQualifierSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemRefresher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemShutdown(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemStatusCodeText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemUnboundObjectSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWbemUnsecuredApartment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MI_Application(i32);
#[repr(C)]
pub struct MI_ApplicationFT(i32);
#[repr(C)]
pub struct MI_Array(i32);
#[repr(C)]
pub struct MI_ArrayField(i32);
#[repr(C)]
pub struct MI_BooleanA(i32);
#[repr(C)]
pub struct MI_BooleanAField(i32);
#[repr(C)]
pub struct MI_BooleanField(i32);
pub const MI_CALL_VERSION: u32 = 1u32;
#[repr(transparent)]
pub struct MI_CallbackMode(pub i32);
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = MI_CallbackMode(0i32);
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = MI_CallbackMode(1i32);
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = MI_CallbackMode(2i32);
#[repr(C)]
pub struct MI_CancelCallback(i32);
#[repr(transparent)]
pub struct MI_CancellationReason(pub i32);
pub const MI_REASON_NONE: MI_CancellationReason = MI_CancellationReason(0i32);
pub const MI_REASON_TIMEOUT: MI_CancellationReason = MI_CancellationReason(1i32);
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = MI_CancellationReason(2i32);
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = MI_CancellationReason(3i32);
#[repr(C)]
pub struct MI_Char16A(i32);
#[repr(C)]
pub struct MI_Char16AField(i32);
#[repr(C)]
pub struct MI_Char16Field(i32);
#[repr(C)]
pub struct MI_Class(i32);
#[repr(C)]
pub struct MI_ClassDecl(i32);
#[repr(C)]
pub struct MI_ClassFT(i32);
#[repr(C)]
pub struct MI_ClientFT_V1(i32);
#[repr(C)]
pub struct MI_ConstBooleanA(i32);
#[repr(C)]
pub struct MI_ConstBooleanAField(i32);
#[repr(C)]
pub struct MI_ConstBooleanField(i32);
#[repr(C)]
pub struct MI_ConstChar16A(i32);
#[repr(C)]
pub struct MI_ConstChar16AField(i32);
#[repr(C)]
pub struct MI_ConstChar16Field(i32);
#[repr(C)]
pub struct MI_ConstDatetimeA(i32);
#[repr(C)]
pub struct MI_ConstDatetimeAField(i32);
#[repr(C)]
pub struct MI_ConstDatetimeField(i32);
#[repr(C)]
pub struct MI_ConstInstanceA(i32);
#[repr(C)]
pub struct MI_ConstInstanceAField(i32);
#[repr(C)]
pub struct MI_ConstInstanceField(i32);
#[repr(C)]
pub struct MI_ConstReal32A(i32);
#[repr(C)]
pub struct MI_ConstReal32AField(i32);
#[repr(C)]
pub struct MI_ConstReal32Field(i32);
#[repr(C)]
pub struct MI_ConstReal64A(i32);
#[repr(C)]
pub struct MI_ConstReal64AField(i32);
#[repr(C)]
pub struct MI_ConstReal64Field(i32);
#[repr(C)]
pub struct MI_ConstReferenceA(i32);
#[repr(C)]
pub struct MI_ConstReferenceAField(i32);
#[repr(C)]
pub struct MI_ConstReferenceField(i32);
#[repr(C)]
pub struct MI_ConstSint16A(i32);
#[repr(C)]
pub struct MI_ConstSint16AField(i32);
#[repr(C)]
pub struct MI_ConstSint16Field(i32);
#[repr(C)]
pub struct MI_ConstSint32A(i32);
#[repr(C)]
pub struct MI_ConstSint32AField(i32);
#[repr(C)]
pub struct MI_ConstSint32Field(i32);
#[repr(C)]
pub struct MI_ConstSint64A(i32);
#[repr(C)]
pub struct MI_ConstSint64AField(i32);
#[repr(C)]
pub struct MI_ConstSint64Field(i32);
#[repr(C)]
pub struct MI_ConstSint8A(i32);
#[repr(C)]
pub struct MI_ConstSint8AField(i32);
#[repr(C)]
pub struct MI_ConstSint8Field(i32);
#[repr(C)]
pub struct MI_ConstStringA(i32);
#[repr(C)]
pub struct MI_ConstStringAField(i32);
#[repr(C)]
pub struct MI_ConstStringField(i32);
#[repr(C)]
pub struct MI_ConstUint16A(i32);
#[repr(C)]
pub struct MI_ConstUint16AField(i32);
#[repr(C)]
pub struct MI_ConstUint16Field(i32);
#[repr(C)]
pub struct MI_ConstUint32A(i32);
#[repr(C)]
pub struct MI_ConstUint32AField(i32);
#[repr(C)]
pub struct MI_ConstUint32Field(i32);
#[repr(C)]
pub struct MI_ConstUint64A(i32);
#[repr(C)]
pub struct MI_ConstUint64AField(i32);
#[repr(C)]
pub struct MI_ConstUint64Field(i32);
#[repr(C)]
pub struct MI_ConstUint8A(i32);
#[repr(C)]
pub struct MI_ConstUint8AField(i32);
#[repr(C)]
pub struct MI_ConstUint8Field(i32);
#[repr(C)]
pub struct MI_Context(i32);
#[repr(C)]
pub struct MI_ContextFT(i32);
#[repr(C)]
pub struct MI_Datetime(i32);
#[repr(C)]
pub struct MI_DatetimeA(i32);
#[repr(C)]
pub struct MI_DatetimeAField(i32);
#[repr(C)]
pub struct MI_DatetimeField(i32);
#[repr(C)]
pub struct MI_Deserializer(i32);
#[repr(C)]
pub struct MI_DeserializerFT(i32);
#[repr(C)]
pub struct MI_Deserializer_ClassObjectNeeded(i32);
#[repr(C)]
pub struct MI_DestinationOptions(i32);
#[repr(C)]
pub struct MI_DestinationOptionsFT(i32);
#[repr(transparent)]
pub struct MI_DestinationOptions_ImpersonationType(pub i32);
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(0i32);
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(1i32);
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(2i32);
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(3i32);
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(4i32);
#[repr(transparent)]
pub struct MI_ErrorCategory(pub i32);
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = MI_ErrorCategory(0i32);
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = MI_ErrorCategory(1i32);
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = MI_ErrorCategory(2i32);
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = MI_ErrorCategory(3i32);
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = MI_ErrorCategory(4i32);
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = MI_ErrorCategory(5i32);
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = MI_ErrorCategory(6i32);
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = MI_ErrorCategory(7i32);
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = MI_ErrorCategory(8i32);
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = MI_ErrorCategory(9i32);
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = MI_ErrorCategory(10i32);
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = MI_ErrorCategory(11i32);
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = MI_ErrorCategory(12i32);
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = MI_ErrorCategory(13i32);
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = MI_ErrorCategory(14i32);
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = MI_ErrorCategory(15i32);
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = MI_ErrorCategory(16i32);
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = MI_ErrorCategory(17i32);
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = MI_ErrorCategory(18i32);
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = MI_ErrorCategory(19i32);
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = MI_ErrorCategory(20i32);
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = MI_ErrorCategory(21i32);
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = MI_ErrorCategory(22i32);
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = MI_ErrorCategory(23i32);
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = MI_ErrorCategory(24i32);
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = MI_ErrorCategory(25i32);
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = MI_ErrorCategory(26i32);
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = MI_ErrorCategory(27i32);
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = MI_ErrorCategory(28i32);
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(29i32);
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(30i32);
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = MI_ErrorCategory(31i32);
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
pub const MI_FLAG_ANY: u32 = 127u32;
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
pub const MI_FLAG_CLASS: u32 = 1u32;
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
pub const MI_FLAG_IN: u32 = 8192u32;
pub const MI_FLAG_INDICATION: u32 = 32u32;
pub const MI_FLAG_KEY: u32 = 4096u32;
pub const MI_FLAG_METHOD: u32 = 2u32;
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
pub const MI_FLAG_NULL: u32 = 536870912u32;
pub const MI_FLAG_OUT: u32 = 16384u32;
pub const MI_FLAG_PARAMETER: u32 = 8u32;
pub const MI_FLAG_PROPERTY: u32 = 4u32;
pub const MI_FLAG_READONLY: u32 = 2097152u32;
pub const MI_FLAG_REFERENCE: u32 = 64u32;
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
pub const MI_FLAG_STATIC: u32 = 65536u32;
pub const MI_FLAG_STREAM: u32 = 1048576u32;
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
pub const MI_FLAG_VERSION: u32 = 469762048u32;
#[repr(C)]
pub struct MI_FeatureDecl(i32);
#[repr(C)]
pub struct MI_Filter(i32);
#[repr(C)]
pub struct MI_FilterFT(i32);
#[repr(C)]
pub struct MI_HostedProvider(i32);
#[repr(C)]
pub struct MI_HostedProviderFT(i32);
#[repr(C)]
pub struct MI_Instance(i32);
#[repr(C)]
pub struct MI_InstanceA(i32);
#[repr(C)]
pub struct MI_InstanceAField(i32);
#[repr(C)]
pub struct MI_InstanceExFT(i32);
#[repr(C)]
pub struct MI_InstanceFT(i32);
#[repr(C)]
pub struct MI_InstanceField(i32);
#[repr(C)]
pub struct MI_Interval(i32);
#[repr(transparent)]
pub struct MI_LocaleType(pub i32);
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = MI_LocaleType(0i32);
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = MI_LocaleType(1i32);
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = MI_LocaleType(2i32);
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = MI_LocaleType(3i32);
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
#[repr(C)]
pub struct MI_MainFunction(i32);
#[repr(C)]
pub struct MI_MethodDecl(i32);
#[repr(C)]
pub struct MI_MethodDecl_Invoke(i32);
#[repr(C)]
pub struct MI_Module(i32);
#[repr(C)]
pub struct MI_Module_Load(i32);
#[repr(C)]
pub struct MI_Module_Self(i32);
#[repr(C)]
pub struct MI_Module_Unload(i32);
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
#[repr(C)]
pub struct MI_ObjectDecl(i32);
#[repr(C)]
pub struct MI_Operation(i32);
#[repr(C)]
pub struct MI_OperationCallback_Class(i32);
#[repr(C)]
pub struct MI_OperationCallback_Indication(i32);
#[repr(C)]
pub struct MI_OperationCallback_Instance(i32);
#[repr(C)]
pub struct MI_OperationCallback_PromptUser(i32);
#[repr(transparent)]
pub struct MI_OperationCallback_ResponseType(pub i32);
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(0i32);
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(1i32);
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(2i32);
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(3i32);
#[repr(C)]
pub struct MI_OperationCallback_StreamedParameter(i32);
#[repr(C)]
pub struct MI_OperationCallback_WriteError(i32);
#[repr(C)]
pub struct MI_OperationCallback_WriteMessage(i32);
#[repr(C)]
pub struct MI_OperationCallback_WriteProgress(i32);
#[repr(C)]
pub struct MI_OperationCallbacks(i32);
#[repr(C)]
pub struct MI_OperationFT(i32);
#[repr(C)]
pub struct MI_OperationOptions(i32);
#[repr(C)]
pub struct MI_OperationOptionsFT(i32);
#[repr(C)]
pub struct MI_ParameterDecl(i32);
#[repr(C)]
pub struct MI_ParameterSet(i32);
#[repr(C)]
pub struct MI_ParameterSetFT(i32);
#[repr(transparent)]
pub struct MI_PromptType(pub i32);
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = MI_PromptType(0i32);
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = MI_PromptType(1i32);
#[repr(C)]
pub struct MI_PropertyDecl(i32);
#[repr(C)]
pub struct MI_PropertySet(i32);
#[repr(C)]
pub struct MI_PropertySetFT(i32);
#[repr(transparent)]
pub struct MI_ProviderArchitecture(pub i32);
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(0i32);
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(1i32);
#[repr(C)]
pub struct MI_ProviderFT(i32);
#[repr(C)]
pub struct MI_ProviderFT_AssociatorInstances(i32);
#[repr(C)]
pub struct MI_ProviderFT_CreateInstance(i32);
#[repr(C)]
pub struct MI_ProviderFT_DeleteInstance(i32);
#[repr(C)]
pub struct MI_ProviderFT_DisableIndications(i32);
#[repr(C)]
pub struct MI_ProviderFT_EnableIndications(i32);
#[repr(C)]
pub struct MI_ProviderFT_EnumerateInstances(i32);
#[repr(C)]
pub struct MI_ProviderFT_GetInstance(i32);
#[repr(C)]
pub struct MI_ProviderFT_Invoke(i32);
#[repr(C)]
pub struct MI_ProviderFT_Load(i32);
#[repr(C)]
pub struct MI_ProviderFT_ModifyInstance(i32);
#[repr(C)]
pub struct MI_ProviderFT_ReferenceInstances(i32);
#[repr(C)]
pub struct MI_ProviderFT_Subscribe(i32);
#[repr(C)]
pub struct MI_ProviderFT_Unload(i32);
#[repr(C)]
pub struct MI_ProviderFT_Unsubscribe(i32);
#[repr(C)]
pub struct MI_Qualifier(i32);
#[repr(C)]
pub struct MI_QualifierDecl(i32);
#[repr(C)]
pub struct MI_QualifierSet(i32);
#[repr(C)]
pub struct MI_QualifierSetFT(i32);
#[repr(C)]
pub struct MI_Real32A(i32);
#[repr(C)]
pub struct MI_Real32AField(i32);
#[repr(C)]
pub struct MI_Real32Field(i32);
#[repr(C)]
pub struct MI_Real64A(i32);
#[repr(C)]
pub struct MI_Real64AField(i32);
#[repr(C)]
pub struct MI_Real64Field(i32);
#[repr(C)]
pub struct MI_ReferenceA(i32);
#[repr(C)]
pub struct MI_ReferenceAField(i32);
#[repr(C)]
pub struct MI_ReferenceField(i32);
#[repr(transparent)]
pub struct MI_Result(pub i32);
pub const MI_RESULT_OK: MI_Result = MI_Result(0i32);
pub const MI_RESULT_FAILED: MI_Result = MI_Result(1i32);
pub const MI_RESULT_ACCESS_DENIED: MI_Result = MI_Result(2i32);
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = MI_Result(3i32);
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = MI_Result(4i32);
pub const MI_RESULT_INVALID_CLASS: MI_Result = MI_Result(5i32);
pub const MI_RESULT_NOT_FOUND: MI_Result = MI_Result(6i32);
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = MI_Result(7i32);
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = MI_Result(8i32);
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = MI_Result(9i32);
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = MI_Result(10i32);
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = MI_Result(11i32);
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = MI_Result(12i32);
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = MI_Result(13i32);
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = MI_Result(14i32);
pub const MI_RESULT_INVALID_QUERY: MI_Result = MI_Result(15i32);
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = MI_Result(16i32);
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = MI_Result(17i32);
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = MI_Result(20i32);
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = MI_Result(21i32);
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = MI_Result(22i32);
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = MI_Result(23i32);
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = MI_Result(24i32);
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = MI_Result(25i32);
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = MI_Result(26i32);
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = MI_Result(27i32);
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = MI_Result(28i32);
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
#[repr(C)]
pub struct MI_SchemaDecl(i32);
#[repr(C)]
pub struct MI_Serializer(i32);
#[repr(C)]
pub struct MI_SerializerFT(i32);
#[repr(C)]
pub struct MI_Server(i32);
#[repr(C)]
pub struct MI_ServerFT(i32);
#[repr(C)]
pub struct MI_Session(i32);
#[repr(C)]
pub struct MI_SessionCallbacks(i32);
#[repr(C)]
pub struct MI_SessionFT(i32);
#[repr(C)]
pub struct MI_Sint16A(i32);
#[repr(C)]
pub struct MI_Sint16AField(i32);
#[repr(C)]
pub struct MI_Sint16Field(i32);
#[repr(C)]
pub struct MI_Sint32A(i32);
#[repr(C)]
pub struct MI_Sint32AField(i32);
#[repr(C)]
pub struct MI_Sint32Field(i32);
#[repr(C)]
pub struct MI_Sint64A(i32);
#[repr(C)]
pub struct MI_Sint64AField(i32);
#[repr(C)]
pub struct MI_Sint64Field(i32);
#[repr(C)]
pub struct MI_Sint8A(i32);
#[repr(C)]
pub struct MI_Sint8AField(i32);
#[repr(C)]
pub struct MI_Sint8Field(i32);
#[repr(C)]
pub struct MI_StringA(i32);
#[repr(C)]
pub struct MI_StringAField(i32);
#[repr(C)]
pub struct MI_StringField(i32);
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptions(i32);
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptionsFT(i32);
#[repr(transparent)]
pub struct MI_SubscriptionDeliveryType(pub i32);
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(1i32);
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(2i32);
#[repr(C)]
pub struct MI_Timestamp(i32);
#[repr(transparent)]
pub struct MI_Type(pub i32);
pub const MI_BOOLEAN: MI_Type = MI_Type(0i32);
pub const MI_UINT8: MI_Type = MI_Type(1i32);
pub const MI_SINT8: MI_Type = MI_Type(2i32);
pub const MI_UINT16: MI_Type = MI_Type(3i32);
pub const MI_SINT16: MI_Type = MI_Type(4i32);
pub const MI_UINT32: MI_Type = MI_Type(5i32);
pub const MI_SINT32: MI_Type = MI_Type(6i32);
pub const MI_UINT64: MI_Type = MI_Type(7i32);
pub const MI_SINT64: MI_Type = MI_Type(8i32);
pub const MI_REAL32: MI_Type = MI_Type(9i32);
pub const MI_REAL64: MI_Type = MI_Type(10i32);
pub const MI_CHAR16: MI_Type = MI_Type(11i32);
pub const MI_DATETIME: MI_Type = MI_Type(12i32);
pub const MI_STRING: MI_Type = MI_Type(13i32);
pub const MI_REFERENCE: MI_Type = MI_Type(14i32);
pub const MI_INSTANCE: MI_Type = MI_Type(15i32);
pub const MI_BOOLEANA: MI_Type = MI_Type(16i32);
pub const MI_UINT8A: MI_Type = MI_Type(17i32);
pub const MI_SINT8A: MI_Type = MI_Type(18i32);
pub const MI_UINT16A: MI_Type = MI_Type(19i32);
pub const MI_SINT16A: MI_Type = MI_Type(20i32);
pub const MI_UINT32A: MI_Type = MI_Type(21i32);
pub const MI_SINT32A: MI_Type = MI_Type(22i32);
pub const MI_UINT64A: MI_Type = MI_Type(23i32);
pub const MI_SINT64A: MI_Type = MI_Type(24i32);
pub const MI_REAL32A: MI_Type = MI_Type(25i32);
pub const MI_REAL64A: MI_Type = MI_Type(26i32);
pub const MI_CHAR16A: MI_Type = MI_Type(27i32);
pub const MI_DATETIMEA: MI_Type = MI_Type(28i32);
pub const MI_STRINGA: MI_Type = MI_Type(29i32);
pub const MI_REFERENCEA: MI_Type = MI_Type(30i32);
pub const MI_INSTANCEA: MI_Type = MI_Type(31i32);
pub const MI_ARRAY: MI_Type = MI_Type(16i32);
#[repr(C)]
pub struct MI_Uint16A(i32);
#[repr(C)]
pub struct MI_Uint16AField(i32);
#[repr(C)]
pub struct MI_Uint16Field(i32);
#[repr(C)]
pub struct MI_Uint32A(i32);
#[repr(C)]
pub struct MI_Uint32AField(i32);
#[repr(C)]
pub struct MI_Uint32Field(i32);
#[repr(C)]
pub struct MI_Uint64A(i32);
#[repr(C)]
pub struct MI_Uint64AField(i32);
#[repr(C)]
pub struct MI_Uint64Field(i32);
#[repr(C)]
pub struct MI_Uint8A(i32);
#[repr(C)]
pub struct MI_Uint8AField(i32);
#[repr(C)]
pub struct MI_Uint8Field(i32);
#[repr(C)]
pub struct MI_UserCredentials(i32);
#[repr(C)]
pub struct MI_UsernamePasswordCreds(i32);
#[repr(C)]
pub struct MI_UtilitiesFT(i32);
#[repr(C)]
pub struct MI_Value(i32);
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
#[repr(C)]
pub struct MofCompiler(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemAnalysisMatrix(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemAnalysisMatrixList(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemAssocQueryInf(i32);
#[repr(C)]
pub struct SWbemDateTime(i32);
#[repr(C)]
pub struct SWbemEventSource(i32);
#[repr(C)]
pub struct SWbemLastError(i32);
#[repr(C)]
pub struct SWbemLocator(i32);
#[repr(C)]
pub struct SWbemMethod(i32);
#[repr(C)]
pub struct SWbemMethodSet(i32);
#[repr(C)]
pub struct SWbemNamedValue(i32);
#[repr(C)]
pub struct SWbemNamedValueSet(i32);
#[repr(C)]
pub struct SWbemObject(i32);
#[repr(C)]
pub struct SWbemObjectEx(i32);
#[repr(C)]
pub struct SWbemObjectPath(i32);
#[repr(C)]
pub struct SWbemObjectSet(i32);
#[repr(C)]
pub struct SWbemPrivilege(i32);
#[repr(C)]
pub struct SWbemPrivilegeSet(i32);
#[repr(C)]
pub struct SWbemProperty(i32);
#[repr(C)]
pub struct SWbemPropertySet(i32);
#[repr(C)]
pub struct SWbemQualifier(i32);
#[repr(C)]
pub struct SWbemQualifierSet(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemQueryQualifiedName(i32);
#[repr(C)]
pub struct SWbemRefreshableItem(i32);
#[repr(C)]
pub struct SWbemRefresher(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemRpnConst(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemRpnEncodedQuery(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWbemRpnQueryToken(i32);
#[repr(C)]
pub struct SWbemRpnTokenList(i32);
#[repr(C)]
pub struct SWbemSecurity(i32);
#[repr(C)]
pub struct SWbemServices(i32);
#[repr(C)]
pub struct SWbemServicesEx(i32);
#[repr(C)]
pub struct SWbemSink(i32);
#[repr(C)]
pub struct UnsecuredApartment(i32);
#[repr(transparent)]
pub struct WBEMSTATUS(pub i32);
pub const WBEM_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_SAME: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_FALSE: WBEMSTATUS = WBEMSTATUS(1i32);
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(262145i32);
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = WBEMSTATUS(262146i32);
pub const WBEM_S_DIFFERENT: WBEMSTATUS = WBEMSTATUS(262147i32);
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = WBEMSTATUS(262148i32);
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = WBEMSTATUS(262149i32);
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = WBEMSTATUS(262150i32);
pub const WBEM_S_PENDING: WBEMSTATUS = WBEMSTATUS(262151i32);
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = WBEMSTATUS(262152i32);
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(262153i32);
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = WBEMSTATUS(262160i32);
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(262167i32);
pub const WBEM_E_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217407i32);
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217406i32);
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(-2147217405i32);
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217404i32);
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217403i32);
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = WBEMSTATUS(-2147217402i32);
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = WBEMSTATUS(-2147217401i32);
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217400i32);
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(-2147217399i32);
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217398i32);
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = WBEMSTATUS(-2147217397i32);
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147217396i32);
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = WBEMSTATUS(-2147217395i32);
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = WBEMSTATUS(-2147217394i32);
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217393i32);
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217392i32);
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217391i32);
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = WBEMSTATUS(-2147217390i32);
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217389i32);
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217388i32);
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217387i32);
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217386i32);
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217385i32);
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217384i32);
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(-2147217383i32);
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217382i32);
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217381i32);
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217380i32);
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = WBEMSTATUS(-2147217379i32);
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217378i32);
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = WBEMSTATUS(-2147217377i32);
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217376i32);
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147217375i32);
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217374i32);
pub const WBEM_E_READ_ONLY: WBEMSTATUS = WBEMSTATUS(-2147217373i32);
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = WBEMSTATUS(-2147217372i32);
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = WBEMSTATUS(-2147217371i32);
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = WBEMSTATUS(-2147217370i32);
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217369i32);
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = WBEMSTATUS(-2147217368i32);
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217367i32);
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217366i32);
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147217365i32);
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = WBEMSTATUS(-2147217364i32);
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217363i32);
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217362i32);
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = WBEMSTATUS(-2147217361i32);
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217360i32);
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217359i32);
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = WBEMSTATUS(-2147217358i32);
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = WBEMSTATUS(-2147217357i32);
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217356i32);
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217355i32);
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217354i32);
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217353i32);
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = WBEMSTATUS(-2147217352i32);
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = WBEMSTATUS(-2147217351i32);
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = WBEMSTATUS(-2147217350i32);
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = WBEMSTATUS(-2147217349i32);
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = WBEMSTATUS(-2147217348i32);
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = WBEMSTATUS(-2147217347i32);
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217346i32);
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217345i32);
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217344i32);
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = WBEMSTATUS(-2147217343i32);
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217342i32);
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217341i32);
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = WBEMSTATUS(-2147217340i32);
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217339i32);
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = WBEMSTATUS(-2147217338i32);
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = WBEMSTATUS(-2147217337i32);
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = WBEMSTATUS(-2147217336i32);
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217335i32);
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217328i32);
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = WBEMSTATUS(-2147217327i32);
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217326i32);
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217325i32);
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217324i32);
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217323i32);
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217322i32);
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217321i32);
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217320i32);
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217319i32);
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = WBEMSTATUS(-2147217318i32);
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = WBEMSTATUS(-2147217317i32);
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217316i32);
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217315i32);
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217313i32);
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = WBEMSTATUS(-2147217312i32);
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = WBEMSTATUS(-2147217311i32);
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = WBEMSTATUS(-2147217310i32);
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = WBEMSTATUS(-2147217309i32);
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = WBEMSTATUS(-2147217308i32);
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = WBEMSTATUS(-2147217307i32);
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217306i32);
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = WBEMSTATUS(-2147217305i32);
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = WBEMSTATUS(-2147217304i32);
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217303i32);
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = WBEMSTATUS(-2147217302i32);
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217301i32);
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = WBEMSTATUS(-2147217300i32);
pub const WBEM_E_RESERVED_001: WBEMSTATUS = WBEMSTATUS(-2147217299i32);
pub const WBEM_E_RESERVED_002: WBEMSTATUS = WBEMSTATUS(-2147217298i32);
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217297i32);
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = WBEMSTATUS(-2147217296i32);
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217295i32);
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = WBEMSTATUS(-2147217294i32);
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217293i32);
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217292i32);
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217291i32);
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217290i32);
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = WBEMSTATUS(-2147217289i32);
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217288i32);
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = WBEMSTATUS(-2147217287i32);
pub const WBEM_E_VETO_PUT: WBEMSTATUS = WBEMSTATUS(-2147217286i32);
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217280i32);
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = WBEMSTATUS(-2147217279i32);
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217278i32);
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = WBEMSTATUS(-2147217277i32);
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217276i32);
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217275i32);
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217274i32);
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217273i32);
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217272i32);
pub const WBEM_E_NO_KEY: WBEMSTATUS = WBEMSTATUS(-2147217271i32);
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217270i32);
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = WBEMSTATUS(-2147213311i32);
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = WBEMSTATUS(-2147213310i32);
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = WBEMSTATUS(-2147213309i32);
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = WBEMSTATUS(-2147205119i32);
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = WBEMSTATUS(-2147205118i32);
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205117i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205116i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = WBEMSTATUS(-2147205115i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205114i32);
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205113i32);
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = WBEMSTATUS(-2147205112i32);
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205111i32);
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = WBEMSTATUS(-2147205110i32);
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205109i32);
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = WBEMSTATUS(-2147205108i32);
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147205107i32);
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = WBEMSTATUS(-2147205106i32);
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = WBEMSTATUS(-2147205105i32);
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205104i32);
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = WBEMSTATUS(-2147205103i32);
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = WBEMSTATUS(-2147205102i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205101i32);
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205100i32);
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147205099i32);
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205098i32);
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205097i32);
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205096i32);
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = WBEMSTATUS(-2147205095i32);
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205094i32);
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147205093i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = WBEMSTATUS(-2147205092i32);
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147205091i32);
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = WBEMSTATUS(-2147205090i32);
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = WBEMSTATUS(-2147205089i32);
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = WBEMSTATUS(-2147205088i32);
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205087i32);
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205086i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = WBEMSTATUS(-2147205085i32);
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = WBEMSTATUS(-2147205084i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = WBEMSTATUS(-2147205083i32);
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = WBEMSTATUS(-2147205082i32);
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = WBEMSTATUS(-2147205081i32);
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205080i32);
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205079i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205078i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205077i32);
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205076i32);
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205075i32);
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = WBEMSTATUS(-2147205074i32);
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = WBEMSTATUS(-2147205073i32);
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = WBEMSTATUS(-2147205072i32);
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205071i32);
#[repr(transparent)]
pub struct WBEMSTATUS_FORMAT(pub i32);
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(0i32);
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(1i32);
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
#[repr(transparent)]
pub struct WBEM_BACKUP_RESTORE_FLAGS(pub i32);
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(0i32);
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(1i32);
#[repr(transparent)]
pub struct WBEM_BATCH_TYPE(pub i32);
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(0i32);
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(1i32);
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(2i32);
#[repr(transparent)]
pub struct WBEM_CHANGE_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(1i32);
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(2i32);
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(32i32);
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(64i32);
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(96i32);
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(65536i32);
#[repr(transparent)]
pub struct WBEM_COMPARISON_FLAG(pub i32);
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(0i32);
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(1i32);
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(2i32);
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(4i32);
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(8i32);
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(16i32);
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(32i32);
#[repr(transparent)]
pub struct WBEM_COMPILER_OPTIONS(pub i32);
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(1i32);
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(2i32);
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(4i32);
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(8i32);
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(16i32);
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(32i32);
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(256i32);
#[repr(C)]
pub struct WBEM_COMPILE_STATUS_INFO(i32);
#[repr(transparent)]
pub struct WBEM_CONDITION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(0i32);
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(2i32);
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(4i32);
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(8i32);
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(32i32);
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(48i32);
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(64i32);
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(112i32);
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(256i32);
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(512i32);
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(768i32);
#[repr(transparent)]
pub struct WBEM_CONNECT_OPTIONS(pub i32);
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(64i32);
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(128i32);
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(256i32);
#[repr(transparent)]
pub struct WBEM_EXTRA_RETURN_CODES(pub i32);
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(0i32);
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274433i32);
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274434i32);
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274435i32);
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209215i32);
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209214i32);
#[repr(transparent)]
pub struct WBEM_FLAVOR_TYPE(pub i32);
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(1i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(2i32);
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(15i32);
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(32i32);
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(64i32);
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(96i32);
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
#[repr(transparent)]
pub struct WBEM_GENERIC_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(32i32);
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(64i32);
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(128i32);
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(256i32);
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(512i32);
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(126976i32);
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(131072i32);
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(1048576i32);
#[repr(transparent)]
pub struct WBEM_GENUS_TYPE(pub i32);
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(1i32);
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(2i32);
#[repr(transparent)]
pub struct WBEM_GET_KEY_FLAGS(pub i32);
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(1i32);
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(2i32);
#[repr(transparent)]
pub struct WBEM_GET_TEXT_FLAGS(pub i32);
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(1i32);
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(2i32);
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(4i32);
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(8i32);
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(16i32);
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(32i32);
#[repr(transparent)]
pub struct WBEM_INFORMATION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(2i32);
#[repr(transparent)]
pub struct WBEM_LIMITATION_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(32i32);
#[repr(transparent)]
pub struct WBEM_LIMITS(pub i32);
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = WBEM_LIMITS(4096i32);
pub const WBEM_MAX_QUERY: WBEM_LIMITS = WBEM_LIMITS(16384i32);
pub const WBEM_MAX_PATH: WBEM_LIMITS = WBEM_LIMITS(8192i32);
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = WBEM_LIMITS(64i32);
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = WBEM_LIMITS(1024i32);
#[repr(transparent)]
pub struct WBEM_LOCKING(pub i32);
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING = WBEM_LOCKING(1i32);
#[repr(transparent)]
pub struct WBEM_PATH_CREATE_FLAG(pub i32);
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(1i32);
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(2i32);
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(4i32);
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(8i32);
#[repr(transparent)]
pub struct WBEM_PATH_STATUS_FLAG(pub i32);
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1i32);
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2i32);
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4i32);
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8i32);
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16i32);
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32i32);
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(64i32);
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(128i32);
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(256i32);
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(512i32);
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1024i32);
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2048i32);
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4096i32);
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8192i32);
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16384i32);
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32768i32);
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(65536i32);
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(131072i32);
#[repr(transparent)]
pub struct WBEM_PROVIDER_FLAGS(pub i32);
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = WBEM_PROVIDER_FLAGS(65536i32);
#[repr(transparent)]
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(pub i32);
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(0i32);
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(1i32);
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(2i32);
#[repr(transparent)]
pub struct WBEM_QUERY_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(1i32);
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(2i32);
#[repr(transparent)]
pub struct WBEM_REFRESHER_FLAGS(pub i32);
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(0i32);
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(1i32);
#[repr(transparent)]
pub struct WBEM_SECURITY_FLAGS(pub i32);
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(1i32);
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(2i32);
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(4i32);
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(8i32);
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(16i32);
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(32i32);
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(64i32);
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(128i32);
#[repr(transparent)]
pub struct WBEM_SHUTDOWN_FLAGS(pub i32);
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(1i32);
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(2i32);
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(3i32);
#[repr(transparent)]
pub struct WBEM_STATUS_TYPE(pub i32);
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(0i32);
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1i32);
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2i32);
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(256i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(512i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1024i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2048i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(4096i32);
#[repr(transparent)]
pub struct WBEM_TEXT_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = WBEM_TEXT_FLAG_TYPE(1i32);
#[repr(transparent)]
pub struct WBEM_TIMEOUT_TYPE(pub i32);
pub const WBEM_NO_WAIT: WBEM_TIMEOUT_TYPE = WBEM_TIMEOUT_TYPE(0i32);
pub const WBEM_INFINITE: WBEM_TIMEOUT_TYPE = WBEM_TIMEOUT_TYPE(-1i32);
#[repr(transparent)]
pub struct WBEM_UNSECAPP_FLAG_TYPE(pub i32);
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(1i32);
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(2i32);
#[repr(C)]
pub struct WMIExtension(i32);
#[repr(transparent)]
pub struct WMIQ_ANALYSIS_TYPE(pub i32);
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(1i32);
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(2i32);
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(3i32);
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(4i32);
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(134217728i32);
#[repr(transparent)]
pub struct WMIQ_ASSOCQ_FLAGS(pub i32);
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1i32);
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2i32);
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(4i32);
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(8i32);
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(16i32);
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(32i32);
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(64i32);
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(128i32);
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(256i32);
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(512i32);
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1024i32);
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2048i32);
#[repr(transparent)]
pub struct WMIQ_LANGUAGE_FEATURES(pub i32);
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(1i32);
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(2i32);
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(3i32);
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(4i32);
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(5i32);
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(6i32);
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(7i32);
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(8i32);
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(9i32);
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(10i32);
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(11i32);
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(12i32);
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(13i32);
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(14i32);
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(15i32);
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(16i32);
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(17i32);
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(18i32);
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(19i32);
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(20i32);
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(21i32);
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(22i32);
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(23i32);
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(24i32);
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(25i32);
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(26i32);
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(27i32);
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(28i32);
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(29i32);
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(30i32);
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(31i32);
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(32i32);
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(33i32);
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(34i32);
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(35i32);
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(36i32);
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(37i32);
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(38i32);
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(39i32);
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
#[repr(transparent)]
pub struct WMIQ_RPNQ_FEATURE(pub i32);
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(1i32);
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(2i32);
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(4i32);
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(8i32);
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(16i32);
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(32i32);
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(64i32);
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(128i32);
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(256i32);
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(512i32);
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(1024i32);
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(2048i32);
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(4096i32);
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNQ_FEATURE = WMIQ_RPNQ_FEATURE(8192i32);
#[repr(transparent)]
pub struct WMIQ_RPN_TOKEN_FLAGS(pub i32);
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(0i32);
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(6i32);
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(7i32);
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(9i32);
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(10i32);
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(11i32);
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(16i32);
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(32i32);
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(64i32);
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
#[repr(transparent)]
pub struct WMI_OBJ_TEXT(pub i32);
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(1i32);
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(2i32);
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = WMI_OBJ_TEXT(3i32);
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = WMI_OBJ_TEXT(4i32);
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = WMI_OBJ_TEXT(5i32);
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = WMI_OBJ_TEXT(6i32);
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = WMI_OBJ_TEXT(7i32);
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = WMI_OBJ_TEXT(8i32);
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = WMI_OBJ_TEXT(9i32);
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = WMI_OBJ_TEXT(10i32);
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = WMI_OBJ_TEXT(11i32);
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = WMI_OBJ_TEXT(12i32);
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = WMI_OBJ_TEXT(13i32);
#[repr(C)]
pub struct WbemAdministrativeLocator(i32);
#[repr(C)]
pub struct WbemAuthenticatedLocator(i32);
#[repr(transparent)]
pub struct WbemAuthenticationLevelEnum(pub i32);
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(0i32);
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(1i32);
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(2i32);
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(3i32);
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(4i32);
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(5i32);
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(6i32);
#[repr(C)]
pub struct WbemBackupRestore(i32);
#[repr(transparent)]
pub struct WbemChangeFlagEnum(pub i32);
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(1i32);
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(2i32);
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = WbemChangeFlagEnum(32i32);
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = WbemChangeFlagEnum(64i32);
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = WbemChangeFlagEnum(128i32);
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = WbemChangeFlagEnum(65536i32);
#[repr(transparent)]
pub struct WbemCimtypeEnum(pub i32);
pub const wbemCimtypeSint8: WbemCimtypeEnum = WbemCimtypeEnum(16i32);
pub const wbemCimtypeUint8: WbemCimtypeEnum = WbemCimtypeEnum(17i32);
pub const wbemCimtypeSint16: WbemCimtypeEnum = WbemCimtypeEnum(2i32);
pub const wbemCimtypeUint16: WbemCimtypeEnum = WbemCimtypeEnum(18i32);
pub const wbemCimtypeSint32: WbemCimtypeEnum = WbemCimtypeEnum(3i32);
pub const wbemCimtypeUint32: WbemCimtypeEnum = WbemCimtypeEnum(19i32);
pub const wbemCimtypeSint64: WbemCimtypeEnum = WbemCimtypeEnum(20i32);
pub const wbemCimtypeUint64: WbemCimtypeEnum = WbemCimtypeEnum(21i32);
pub const wbemCimtypeReal32: WbemCimtypeEnum = WbemCimtypeEnum(4i32);
pub const wbemCimtypeReal64: WbemCimtypeEnum = WbemCimtypeEnum(5i32);
pub const wbemCimtypeBoolean: WbemCimtypeEnum = WbemCimtypeEnum(11i32);
pub const wbemCimtypeString: WbemCimtypeEnum = WbemCimtypeEnum(8i32);
pub const wbemCimtypeDatetime: WbemCimtypeEnum = WbemCimtypeEnum(101i32);
pub const wbemCimtypeReference: WbemCimtypeEnum = WbemCimtypeEnum(102i32);
pub const wbemCimtypeChar16: WbemCimtypeEnum = WbemCimtypeEnum(103i32);
pub const wbemCimtypeObject: WbemCimtypeEnum = WbemCimtypeEnum(13i32);
#[repr(C)]
pub struct WbemClassObject(i32);
#[repr(transparent)]
pub struct WbemComparisonFlagEnum(pub i32);
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = WbemComparisonFlagEnum(0i32);
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = WbemComparisonFlagEnum(1i32);
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = WbemComparisonFlagEnum(2i32);
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = WbemComparisonFlagEnum(4i32);
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = WbemComparisonFlagEnum(8i32);
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = WbemComparisonFlagEnum(16i32);
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = WbemComparisonFlagEnum(32i32);
#[repr(transparent)]
pub struct WbemConnectOptionsEnum(pub i32);
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = WbemConnectOptionsEnum(128i32);
#[repr(C)]
pub struct WbemContext(i32);
#[repr(C)]
pub struct WbemDCOMTransport(i32);
#[repr(C)]
pub struct WbemDecoupledBasicEventProvider(i32);
#[repr(C)]
pub struct WbemDecoupledRegistrar(i32);
#[repr(C)]
pub struct WbemDefPath(i32);
#[repr(transparent)]
pub struct WbemErrorEnum(pub i32);
pub const wbemNoErr: WbemErrorEnum = WbemErrorEnum(0i32);
pub const wbemErrFailed: WbemErrorEnum = WbemErrorEnum(-2147217407i32);
pub const wbemErrNotFound: WbemErrorEnum = WbemErrorEnum(-2147217406i32);
pub const wbemErrAccessDenied: WbemErrorEnum = WbemErrorEnum(-2147217405i32);
pub const wbemErrProviderFailure: WbemErrorEnum = WbemErrorEnum(-2147217404i32);
pub const wbemErrTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217403i32);
pub const wbemErrOutOfMemory: WbemErrorEnum = WbemErrorEnum(-2147217402i32);
pub const wbemErrInvalidContext: WbemErrorEnum = WbemErrorEnum(-2147217401i32);
pub const wbemErrInvalidParameter: WbemErrorEnum = WbemErrorEnum(-2147217400i32);
pub const wbemErrNotAvailable: WbemErrorEnum = WbemErrorEnum(-2147217399i32);
pub const wbemErrCriticalError: WbemErrorEnum = WbemErrorEnum(-2147217398i32);
pub const wbemErrInvalidStream: WbemErrorEnum = WbemErrorEnum(-2147217397i32);
pub const wbemErrNotSupported: WbemErrorEnum = WbemErrorEnum(-2147217396i32);
pub const wbemErrInvalidSuperclass: WbemErrorEnum = WbemErrorEnum(-2147217395i32);
pub const wbemErrInvalidNamespace: WbemErrorEnum = WbemErrorEnum(-2147217394i32);
pub const wbemErrInvalidObject: WbemErrorEnum = WbemErrorEnum(-2147217393i32);
pub const wbemErrInvalidClass: WbemErrorEnum = WbemErrorEnum(-2147217392i32);
pub const wbemErrProviderNotFound: WbemErrorEnum = WbemErrorEnum(-2147217391i32);
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = WbemErrorEnum(-2147217390i32);
pub const wbemErrProviderLoadFailure: WbemErrorEnum = WbemErrorEnum(-2147217389i32);
pub const wbemErrInitializationFailure: WbemErrorEnum = WbemErrorEnum(-2147217388i32);
pub const wbemErrTransportFailure: WbemErrorEnum = WbemErrorEnum(-2147217387i32);
pub const wbemErrInvalidOperation: WbemErrorEnum = WbemErrorEnum(-2147217386i32);
pub const wbemErrInvalidQuery: WbemErrorEnum = WbemErrorEnum(-2147217385i32);
pub const wbemErrInvalidQueryType: WbemErrorEnum = WbemErrorEnum(-2147217384i32);
pub const wbemErrAlreadyExists: WbemErrorEnum = WbemErrorEnum(-2147217383i32);
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217382i32);
pub const wbemErrPropagatedQualifier: WbemErrorEnum = WbemErrorEnum(-2147217381i32);
pub const wbemErrPropagatedProperty: WbemErrorEnum = WbemErrorEnum(-2147217380i32);
pub const wbemErrUnexpected: WbemErrorEnum = WbemErrorEnum(-2147217379i32);
pub const wbemErrIllegalOperation: WbemErrorEnum = WbemErrorEnum(-2147217378i32);
pub const wbemErrCannotBeKey: WbemErrorEnum = WbemErrorEnum(-2147217377i32);
pub const wbemErrIncompleteClass: WbemErrorEnum = WbemErrorEnum(-2147217376i32);
pub const wbemErrInvalidSyntax: WbemErrorEnum = WbemErrorEnum(-2147217375i32);
pub const wbemErrNondecoratedObject: WbemErrorEnum = WbemErrorEnum(-2147217374i32);
pub const wbemErrReadOnly: WbemErrorEnum = WbemErrorEnum(-2147217373i32);
pub const wbemErrProviderNotCapable: WbemErrorEnum = WbemErrorEnum(-2147217372i32);
pub const wbemErrClassHasChildren: WbemErrorEnum = WbemErrorEnum(-2147217371i32);
pub const wbemErrClassHasInstances: WbemErrorEnum = WbemErrorEnum(-2147217370i32);
pub const wbemErrQueryNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217369i32);
pub const wbemErrIllegalNull: WbemErrorEnum = WbemErrorEnum(-2147217368i32);
pub const wbemErrInvalidQualifierType: WbemErrorEnum = WbemErrorEnum(-2147217367i32);
pub const wbemErrInvalidPropertyType: WbemErrorEnum = WbemErrorEnum(-2147217366i32);
pub const wbemErrValueOutOfRange: WbemErrorEnum = WbemErrorEnum(-2147217365i32);
pub const wbemErrCannotBeSingleton: WbemErrorEnum = WbemErrorEnum(-2147217364i32);
pub const wbemErrInvalidCimType: WbemErrorEnum = WbemErrorEnum(-2147217363i32);
pub const wbemErrInvalidMethod: WbemErrorEnum = WbemErrorEnum(-2147217362i32);
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = WbemErrorEnum(-2147217361i32);
pub const wbemErrSystemProperty: WbemErrorEnum = WbemErrorEnum(-2147217360i32);
pub const wbemErrInvalidProperty: WbemErrorEnum = WbemErrorEnum(-2147217359i32);
pub const wbemErrCallCancelled: WbemErrorEnum = WbemErrorEnum(-2147217358i32);
pub const wbemErrShuttingDown: WbemErrorEnum = WbemErrorEnum(-2147217357i32);
pub const wbemErrPropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217356i32);
pub const wbemErrUnsupportedParameter: WbemErrorEnum = WbemErrorEnum(-2147217355i32);
pub const wbemErrMissingParameter: WbemErrorEnum = WbemErrorEnum(-2147217354i32);
pub const wbemErrInvalidParameterId: WbemErrorEnum = WbemErrorEnum(-2147217353i32);
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = WbemErrorEnum(-2147217352i32);
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = WbemErrorEnum(-2147217351i32);
pub const wbemErrInvalidObjectPath: WbemErrorEnum = WbemErrorEnum(-2147217350i32);
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = WbemErrorEnum(-2147217349i32);
pub const wbemErrBufferTooSmall: WbemErrorEnum = WbemErrorEnum(-2147217348i32);
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = WbemErrorEnum(-2147217347i32);
pub const wbemErrUnknownObjectType: WbemErrorEnum = WbemErrorEnum(-2147217346i32);
pub const wbemErrUnknownPacketType: WbemErrorEnum = WbemErrorEnum(-2147217345i32);
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = WbemErrorEnum(-2147217344i32);
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = WbemErrorEnum(-2147217343i32);
pub const wbemErrInvalidQualifier: WbemErrorEnum = WbemErrorEnum(-2147217342i32);
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = WbemErrorEnum(-2147217341i32);
pub const wbemErrTooMuchData: WbemErrorEnum = WbemErrorEnum(-2147217340i32);
pub const wbemErrServerTooBusy: WbemErrorEnum = WbemErrorEnum(-2147217339i32);
pub const wbemErrInvalidFlavor: WbemErrorEnum = WbemErrorEnum(-2147217338i32);
pub const wbemErrCircularReference: WbemErrorEnum = WbemErrorEnum(-2147217337i32);
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = WbemErrorEnum(-2147217336i32);
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = WbemErrorEnum(-2147217335i32);
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = WbemErrorEnum(-2147217328i32);
pub const wbemErrTooManyProperties: WbemErrorEnum = WbemErrorEnum(-2147217327i32);
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217326i32);
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217325i32);
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217324i32);
pub const wbemErrMethodNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217323i32);
pub const wbemErrMethodDisabled: WbemErrorEnum = WbemErrorEnum(-2147217322i32);
pub const wbemErrRefresherBusy: WbemErrorEnum = WbemErrorEnum(-2147217321i32);
pub const wbemErrUnparsableQuery: WbemErrorEnum = WbemErrorEnum(-2147217320i32);
pub const wbemErrNotEventClass: WbemErrorEnum = WbemErrorEnum(-2147217319i32);
pub const wbemErrMissingGroupWithin: WbemErrorEnum = WbemErrorEnum(-2147217318i32);
pub const wbemErrMissingAggregationList: WbemErrorEnum = WbemErrorEnum(-2147217317i32);
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = WbemErrorEnum(-2147217316i32);
pub const wbemErrAggregatingByObject: WbemErrorEnum = WbemErrorEnum(-2147217315i32);
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = WbemErrorEnum(-2147217313i32);
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = WbemErrorEnum(-2147217312i32);
pub const wbemErrQueueOverflow: WbemErrorEnum = WbemErrorEnum(-2147217311i32);
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = WbemErrorEnum(-2147217310i32);
pub const wbemErrInvalidOperator: WbemErrorEnum = WbemErrorEnum(-2147217309i32);
pub const wbemErrLocalCredentials: WbemErrorEnum = WbemErrorEnum(-2147217308i32);
pub const wbemErrCannotBeAbstract: WbemErrorEnum = WbemErrorEnum(-2147217307i32);
pub const wbemErrAmendedObject: WbemErrorEnum = WbemErrorEnum(-2147217306i32);
pub const wbemErrClientTooSlow: WbemErrorEnum = WbemErrorEnum(-2147217305i32);
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = WbemErrorEnum(-2147217304i32);
pub const wbemErrTimeout: WbemErrorEnum = WbemErrorEnum(-2147217303i32);
pub const wbemErrInvalidAssociation: WbemErrorEnum = WbemErrorEnum(-2147217302i32);
pub const wbemErrAmbiguousOperation: WbemErrorEnum = WbemErrorEnum(-2147217301i32);
pub const wbemErrQuotaViolation: WbemErrorEnum = WbemErrorEnum(-2147217300i32);
pub const wbemErrTransactionConflict: WbemErrorEnum = WbemErrorEnum(-2147217299i32);
pub const wbemErrForcedRollback: WbemErrorEnum = WbemErrorEnum(-2147217298i32);
pub const wbemErrUnsupportedLocale: WbemErrorEnum = WbemErrorEnum(-2147217297i32);
pub const wbemErrHandleOutOfDate: WbemErrorEnum = WbemErrorEnum(-2147217296i32);
pub const wbemErrConnectionFailed: WbemErrorEnum = WbemErrorEnum(-2147217295i32);
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = WbemErrorEnum(-2147217294i32);
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217293i32);
pub const wbemErrClassNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217292i32);
pub const wbemErrMethodNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217291i32);
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217290i32);
pub const wbemErrRerunCommand: WbemErrorEnum = WbemErrorEnum(-2147217289i32);
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = WbemErrorEnum(-2147217288i32);
pub const wbemErrVetoPut: WbemErrorEnum = WbemErrorEnum(-2147217287i32);
pub const wbemErrVetoDelete: WbemErrorEnum = WbemErrorEnum(-2147217286i32);
pub const wbemErrInvalidLocale: WbemErrorEnum = WbemErrorEnum(-2147217280i32);
pub const wbemErrProviderSuspended: WbemErrorEnum = WbemErrorEnum(-2147217279i32);
pub const wbemErrSynchronizationRequired: WbemErrorEnum = WbemErrorEnum(-2147217278i32);
pub const wbemErrNoSchema: WbemErrorEnum = WbemErrorEnum(-2147217277i32);
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = WbemErrorEnum(-2147217276i32);
pub const wbemErrProviderNotRegistered: WbemErrorEnum = WbemErrorEnum(-2147217275i32);
pub const wbemErrFatalTransportError: WbemErrorEnum = WbemErrorEnum(-2147217274i32);
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = WbemErrorEnum(-2147217273i32);
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = WbemErrorEnum(-2147213311i32);
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = WbemErrorEnum(-2147213310i32);
pub const wbemErrTimedout: WbemErrorEnum = WbemErrorEnum(-2147209215i32);
pub const wbemErrResetToDefault: WbemErrorEnum = WbemErrorEnum(-2147209214i32);
#[repr(transparent)]
pub struct WbemFlagEnum(pub i32);
pub const wbemFlagReturnImmediately: WbemFlagEnum = WbemFlagEnum(16i32);
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagBidirectional: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagForwardOnly: WbemFlagEnum = WbemFlagEnum(32i32);
pub const wbemFlagNoErrorObject: WbemFlagEnum = WbemFlagEnum(64i32);
pub const wbemFlagReturnErrorObject: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSendStatus: WbemFlagEnum = WbemFlagEnum(128i32);
pub const wbemFlagDontSendStatus: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagEnsureLocatable: WbemFlagEnum = WbemFlagEnum(256i32);
pub const wbemFlagDirectRead: WbemFlagEnum = WbemFlagEnum(512i32);
pub const wbemFlagSendOnlySelected: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = WbemFlagEnum(131072i32);
pub const wbemFlagGetDefault: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSpawnInstance: WbemFlagEnum = WbemFlagEnum(1i32);
pub const wbemFlagUseCurrentTime: WbemFlagEnum = WbemFlagEnum(1i32);
#[repr(transparent)]
pub struct WbemImpersonationLevelEnum(pub i32);
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(1i32);
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(2i32);
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(3i32);
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(4i32);
#[repr(C)]
pub struct WbemLevel1Login(i32);
#[repr(C)]
pub struct WbemLocalAddrRes(i32);
#[repr(C)]
pub struct WbemLocator(i32);
#[repr(transparent)]
pub struct WbemObjectTextFormatEnum(pub i32);
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(1i32);
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(2i32);
#[repr(C)]
pub struct WbemObjectTextSrc(i32);
#[repr(transparent)]
pub struct WbemPrivilegeEnum(pub i32);
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = WbemPrivilegeEnum(1i32);
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = WbemPrivilegeEnum(2i32);
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = WbemPrivilegeEnum(3i32);
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = WbemPrivilegeEnum(4i32);
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = WbemPrivilegeEnum(5i32);
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = WbemPrivilegeEnum(6i32);
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = WbemPrivilegeEnum(7i32);
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = WbemPrivilegeEnum(8i32);
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = WbemPrivilegeEnum(9i32);
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = WbemPrivilegeEnum(10i32);
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = WbemPrivilegeEnum(11i32);
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = WbemPrivilegeEnum(12i32);
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = WbemPrivilegeEnum(13i32);
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = WbemPrivilegeEnum(14i32);
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = WbemPrivilegeEnum(15i32);
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = WbemPrivilegeEnum(16i32);
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = WbemPrivilegeEnum(17i32);
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(18i32);
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = WbemPrivilegeEnum(19i32);
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = WbemPrivilegeEnum(20i32);
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = WbemPrivilegeEnum(21i32);
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = WbemPrivilegeEnum(22i32);
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(23i32);
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = WbemPrivilegeEnum(24i32);
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = WbemPrivilegeEnum(25i32);
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = WbemPrivilegeEnum(26i32);
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = WbemPrivilegeEnum(27i32);
#[repr(C)]
pub struct WbemQuery(i32);
#[repr(transparent)]
pub struct WbemQueryFlagEnum(pub i32);
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = WbemQueryFlagEnum(0i32);
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = WbemQueryFlagEnum(1i32);
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = WbemQueryFlagEnum(2i32);
#[repr(C)]
pub struct WbemRefresher(i32);
#[repr(C)]
pub struct WbemStatusCodeText(i32);
#[repr(transparent)]
pub struct WbemTextFlagEnum(pub i32);
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = WbemTextFlagEnum(1i32);
#[repr(transparent)]
pub struct WbemTimeout(pub i32);
pub const wbemTimeoutInfinite: WbemTimeout = WbemTimeout(-1i32);
#[repr(C)]
pub struct WbemUnauthenticatedLocator(i32);
#[repr(C)]
pub struct WbemUninitializedClassObject(i32);
#[repr(transparent)]
pub struct tag_WBEM_LOGIN_TYPE(pub i32);
pub const WBEM_FLAG_INPROC_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(0i32);
pub const WBEM_FLAG_LOCAL_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(1i32);
pub const WBEM_FLAG_REMOTE_LOGIN: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(2i32);
pub const WBEM_AUTHENTICATION_METHOD_MASK: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(15i32);
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: tag_WBEM_LOGIN_TYPE = tag_WBEM_LOGIN_TYPE(16i32);
