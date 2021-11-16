#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
}
pub const CIM_ILLEGAL: i32 = 4095i32;
pub const CIM_EMPTY: i32 = 0i32;
pub const CIM_SINT8: i32 = 16i32;
pub const CIM_UINT8: i32 = 17i32;
pub const CIM_SINT16: i32 = 2i32;
pub const CIM_UINT16: i32 = 18i32;
pub const CIM_SINT32: i32 = 3i32;
pub const CIM_UINT32: i32 = 19i32;
pub const CIM_SINT64: i32 = 20i32;
pub const CIM_UINT64: i32 = 21i32;
pub const CIM_REAL32: i32 = 4i32;
pub const CIM_REAL64: i32 = 5i32;
pub const CIM_BOOLEAN: i32 = 11i32;
pub const CIM_STRING: i32 = 8i32;
pub const CIM_DATETIME: i32 = 101i32;
pub const CIM_REFERENCE: i32 = 102i32;
pub const CIM_CHAR16: i32 = 103i32;
pub const CIM_OBJECT: i32 = 13i32;
pub const CIM_FLAG_ARRAY: i32 = 8192i32;
#[repr(transparent)]
pub struct IEnumWbemClassObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumWbemClassObject {}
impl ::core::clone::Clone for IEnumWbemClassObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMofCompiler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMofCompiler {}
impl ::core::clone::Clone for IMofCompiler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemDateTime(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemDateTime {}
impl ::core::clone::Clone for ISWbemDateTime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemEventSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemEventSource {}
impl ::core::clone::Clone for ISWbemEventSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemLastError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemLastError {}
impl ::core::clone::Clone for ISWbemLastError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemLocator {}
impl ::core::clone::Clone for ISWbemLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemMethod(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemMethod {}
impl ::core::clone::Clone for ISWbemMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemMethodSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemMethodSet {}
impl ::core::clone::Clone for ISWbemMethodSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemNamedValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemNamedValue {}
impl ::core::clone::Clone for ISWbemNamedValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemNamedValueSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemNamedValueSet {}
impl ::core::clone::Clone for ISWbemNamedValueSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemObject {}
impl ::core::clone::Clone for ISWbemObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemObjectEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemObjectEx {}
impl ::core::clone::Clone for ISWbemObjectEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemObjectPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemObjectPath {}
impl ::core::clone::Clone for ISWbemObjectPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemObjectSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemObjectSet {}
impl ::core::clone::Clone for ISWbemObjectSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemPrivilege(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemPrivilege {}
impl ::core::clone::Clone for ISWbemPrivilege {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemPrivilegeSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemPrivilegeSet {}
impl ::core::clone::Clone for ISWbemPrivilegeSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemProperty {}
impl ::core::clone::Clone for ISWbemProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemPropertySet {}
impl ::core::clone::Clone for ISWbemPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemQualifier {}
impl ::core::clone::Clone for ISWbemQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemQualifierSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemQualifierSet {}
impl ::core::clone::Clone for ISWbemQualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemRefreshableItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemRefreshableItem {}
impl ::core::clone::Clone for ISWbemRefreshableItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemRefresher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemRefresher {}
impl ::core::clone::Clone for ISWbemRefresher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemSecurity {}
impl ::core::clone::Clone for ISWbemSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemServices {}
impl ::core::clone::Clone for ISWbemServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemServicesEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemServicesEx {}
impl ::core::clone::Clone for ISWbemServicesEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemSink {}
impl ::core::clone::Clone for ISWbemSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISWbemSinkEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISWbemSinkEvents {}
impl ::core::clone::Clone for ISWbemSinkEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnsecuredApartment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnsecuredApartment {}
impl ::core::clone::Clone for IUnsecuredApartment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMIExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMIExtension {}
impl ::core::clone::Clone for IWMIExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemAddressResolution(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemAddressResolution {}
impl ::core::clone::Clone for IWbemAddressResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemBackupRestore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemBackupRestore {}
impl ::core::clone::Clone for IWbemBackupRestore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemBackupRestoreEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemBackupRestoreEx {}
impl ::core::clone::Clone for IWbemBackupRestoreEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemCallResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemCallResult {}
impl ::core::clone::Clone for IWbemCallResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemClassObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemClassObject {}
impl ::core::clone::Clone for IWbemClassObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemClientConnectionTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemClientConnectionTransport {}
impl ::core::clone::Clone for IWbemClientConnectionTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemClientTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemClientTransport {}
impl ::core::clone::Clone for IWbemClientTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemConfigureRefresher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemConfigureRefresher {}
impl ::core::clone::Clone for IWbemConfigureRefresher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemConnectorLogin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemConnectorLogin {}
impl ::core::clone::Clone for IWbemConnectorLogin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemConstructClassObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemConstructClassObject {}
impl ::core::clone::Clone for IWbemConstructClassObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemContext {}
impl ::core::clone::Clone for IWbemContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemDecoupledBasicEventProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemDecoupledBasicEventProvider {}
impl ::core::clone::Clone for IWbemDecoupledBasicEventProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemDecoupledRegistrar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemDecoupledRegistrar {}
impl ::core::clone::Clone for IWbemDecoupledRegistrar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemEventConsumerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemEventConsumerProvider {}
impl ::core::clone::Clone for IWbemEventConsumerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemEventProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemEventProvider {}
impl ::core::clone::Clone for IWbemEventProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemEventProviderQuerySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemEventProviderQuerySink {}
impl ::core::clone::Clone for IWbemEventProviderQuerySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemEventProviderSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemEventProviderSecurity {}
impl ::core::clone::Clone for IWbemEventProviderSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemEventSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemEventSink {}
impl ::core::clone::Clone for IWbemEventSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemHiPerfEnum(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemHiPerfEnum {}
impl ::core::clone::Clone for IWbemHiPerfEnum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemHiPerfProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemHiPerfProvider {}
impl ::core::clone::Clone for IWbemHiPerfProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemLevel1Login(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemLevel1Login {}
impl ::core::clone::Clone for IWbemLevel1Login {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemLocator {}
impl ::core::clone::Clone for IWbemLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemObjectAccess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemObjectAccess {}
impl ::core::clone::Clone for IWbemObjectAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemObjectSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemObjectSink {}
impl ::core::clone::Clone for IWbemObjectSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemObjectSinkEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemObjectSinkEx {}
impl ::core::clone::Clone for IWbemObjectSinkEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemObjectTextSrc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemObjectTextSrc {}
impl ::core::clone::Clone for IWbemObjectTextSrc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemPath {}
impl ::core::clone::Clone for IWbemPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemPathKeyList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemPathKeyList {}
impl ::core::clone::Clone for IWbemPathKeyList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemPropertyProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemPropertyProvider {}
impl ::core::clone::Clone for IWbemPropertyProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemProviderIdentity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemProviderIdentity {}
impl ::core::clone::Clone for IWbemProviderIdentity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemProviderInit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemProviderInit {}
impl ::core::clone::Clone for IWbemProviderInit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemProviderInitSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemProviderInitSink {}
impl ::core::clone::Clone for IWbemProviderInitSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemQualifierSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemQualifierSet {}
impl ::core::clone::Clone for IWbemQualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemQuery {}
impl ::core::clone::Clone for IWbemQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemRefresher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemRefresher {}
impl ::core::clone::Clone for IWbemRefresher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemServices {}
impl ::core::clone::Clone for IWbemServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemShutdown(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemShutdown {}
impl ::core::clone::Clone for IWbemShutdown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemStatusCodeText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemStatusCodeText {}
impl ::core::clone::Clone for IWbemStatusCodeText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemTransport {}
impl ::core::clone::Clone for IWbemTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemUnboundObjectSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemUnboundObjectSink {}
impl ::core::clone::Clone for IWbemUnboundObjectSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWbemUnsecuredApartment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWbemUnsecuredApartment {}
impl ::core::clone::Clone for IWbemUnsecuredApartment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_ApplicationFT,
}
impl ::core::marker::Copy for MI_Application {}
impl ::core::clone::Clone for MI_Application {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ApplicationFT {
    pub Close: isize,
    pub NewSession: isize,
    pub NewHostedProvider: isize,
    pub NewInstance: isize,
    pub NewDestinationOptions: isize,
    pub NewOperationOptions: isize,
    pub NewSubscriptionDeliveryOptions: isize,
    pub NewSerializer: isize,
    pub NewDeserializer: isize,
    pub NewInstanceFromClass: isize,
    pub NewClass: isize,
}
impl ::core::marker::Copy for MI_ApplicationFT {}
impl ::core::clone::Clone for MI_ApplicationFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Array {
    pub data: *mut ::core::ffi::c_void,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Array {}
impl ::core::clone::Clone for MI_Array {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ArrayField {}
impl ::core::clone::Clone for MI_ArrayField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_BooleanA {}
impl ::core::clone::Clone for MI_BooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanAField {}
impl ::core::clone::Clone for MI_BooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanField {}
impl ::core::clone::Clone for MI_BooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_CALL_VERSION: u32 = 1u32;
pub const MI_CALLBACKMODE_REPORT: i32 = 0i32;
pub const MI_CALLBACKMODE_INQUIRE: i32 = 1i32;
pub const MI_CALLBACKMODE_IGNORE: i32 = 2i32;
pub type MI_CancelCallback = unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const ::core::ffi::c_void);
pub const MI_REASON_NONE: i32 = 0i32;
pub const MI_REASON_TIMEOUT: i32 = 1i32;
pub const MI_REASON_SHUTDOWN: i32 = 2i32;
pub const MI_REASON_SERVICESTOP: i32 = 3i32;
#[repr(C)]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Char16A {}
impl ::core::clone::Clone for MI_Char16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16AField {}
impl ::core::clone::Clone for MI_Char16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16Field {}
impl ::core::clone::Clone for MI_Char16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Class {
    pub ft: *mut MI_ClassFT,
    pub classDecl: *mut MI_ClassDecl,
    pub namespaceName: *mut u16,
    pub serverName: *mut u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Class {}
impl ::core::clone::Clone for MI_Class {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *mut *mut MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *mut u16,
    pub superClassDecl: *mut MI_ClassDecl,
    pub methods: *mut *mut MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *mut MI_SchemaDecl,
    pub providerFT: *mut MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl ::core::marker::Copy for MI_ClassDecl {}
impl ::core::clone::Clone for MI_ClassDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ClassFT {
    pub GetClassNameA: isize,
    pub GetNameSpace: isize,
    pub GetServerName: isize,
    pub GetElementCount: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub GetClassQualifierSet: isize,
    pub GetMethodCount: isize,
    pub GetMethodAt: isize,
    pub GetMethod: isize,
    pub GetParentClassName: isize,
    pub GetParentClass: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_ClassFT {}
impl ::core::clone::Clone for MI_ClassFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *mut MI_ApplicationFT,
    pub sessionFT: *mut MI_SessionFT,
    pub operationFT: *mut MI_OperationFT,
    pub hostedProviderFT: *mut MI_HostedProviderFT,
    pub serializerFT: *mut MI_SerializerFT,
    pub deserializerFT: *mut MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *mut MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *mut MI_DestinationOptionsFT,
    pub operationOptionsFT: *mut MI_OperationOptionsFT,
    pub utilitiesFT: *mut MI_UtilitiesFT,
}
impl ::core::marker::Copy for MI_ClientFT_V1 {}
impl ::core::clone::Clone for MI_ClientFT_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstBooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstBooleanA {}
impl ::core::clone::Clone for MI_ConstBooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanAField {}
impl ::core::clone::Clone for MI_ConstBooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanField {}
impl ::core::clone::Clone for MI_ConstBooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstChar16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstChar16A {}
impl ::core::clone::Clone for MI_ConstChar16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16AField {}
impl ::core::clone::Clone for MI_ConstChar16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16Field {}
impl ::core::clone::Clone for MI_ConstChar16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstDatetimeA {}
impl ::core::clone::Clone for MI_ConstDatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeAField {}
impl ::core::clone::Clone for MI_ConstDatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeField {}
impl ::core::clone::Clone for MI_ConstDatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstInstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstInstanceA {}
impl ::core::clone::Clone for MI_ConstInstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceAField {}
impl ::core::clone::Clone for MI_ConstInstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstInstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceField {}
impl ::core::clone::Clone for MI_ConstInstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal32A {}
impl ::core::clone::Clone for MI_ConstReal32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32AField {}
impl ::core::clone::Clone for MI_ConstReal32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32Field {}
impl ::core::clone::Clone for MI_ConstReal32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal64A {}
impl ::core::clone::Clone for MI_ConstReal64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64AField {}
impl ::core::clone::Clone for MI_ConstReal64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64Field {}
impl ::core::clone::Clone for MI_ConstReal64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReferenceA {}
impl ::core::clone::Clone for MI_ConstReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceAField {}
impl ::core::clone::Clone for MI_ConstReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceField {}
impl ::core::clone::Clone for MI_ConstReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint16A {}
impl ::core::clone::Clone for MI_ConstSint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16AField {}
impl ::core::clone::Clone for MI_ConstSint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16Field {}
impl ::core::clone::Clone for MI_ConstSint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint32A {}
impl ::core::clone::Clone for MI_ConstSint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32AField {}
impl ::core::clone::Clone for MI_ConstSint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32Field {}
impl ::core::clone::Clone for MI_ConstSint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint64A {}
impl ::core::clone::Clone for MI_ConstSint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64AField {}
impl ::core::clone::Clone for MI_ConstSint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64Field {}
impl ::core::clone::Clone for MI_ConstSint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint8A {}
impl ::core::clone::Clone for MI_ConstSint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8AField {}
impl ::core::clone::Clone for MI_ConstSint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8Field {}
impl ::core::clone::Clone for MI_ConstSint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstStringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstStringA {}
impl ::core::clone::Clone for MI_ConstStringA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringAField {}
impl ::core::clone::Clone for MI_ConstStringAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstStringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringField {}
impl ::core::clone::Clone for MI_ConstStringField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint16A {}
impl ::core::clone::Clone for MI_ConstUint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16AField {}
impl ::core::clone::Clone for MI_ConstUint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16Field {}
impl ::core::clone::Clone for MI_ConstUint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint32A {}
impl ::core::clone::Clone for MI_ConstUint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32AField {}
impl ::core::clone::Clone for MI_ConstUint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32Field {}
impl ::core::clone::Clone for MI_ConstUint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint64A {}
impl ::core::clone::Clone for MI_ConstUint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64AField {}
impl ::core::clone::Clone for MI_ConstUint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64Field {}
impl ::core::clone::Clone for MI_ConstUint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint8A {}
impl ::core::clone::Clone for MI_ConstUint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8AField {}
impl ::core::clone::Clone for MI_ConstUint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8Field {}
impl ::core::clone::Clone for MI_ConstUint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Context {
    pub ft: *mut MI_ContextFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Context {}
impl ::core::clone::Clone for MI_Context {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ContextFT {
    pub PostResult: isize,
    pub PostInstance: isize,
    pub PostIndication: isize,
    pub ConstructInstance: isize,
    pub ConstructParameters: isize,
    pub NewInstance: isize,
    pub NewDynamicInstance: isize,
    pub NewParameters: isize,
    pub Canceled: isize,
    pub GetLocale: isize,
    pub RegisterCancel: isize,
    pub RequestUnload: isize,
    pub RefuseUnload: isize,
    pub GetLocalSession: isize,
    pub SetStringOption: isize,
    pub GetStringOption: isize,
    pub GetNumberOption: isize,
    pub GetCustomOption: isize,
    pub GetCustomOptionCount: isize,
    pub GetCustomOptionAt: isize,
    pub WriteMessage: isize,
    pub WriteProgress: isize,
    pub WriteStreamParameter: isize,
    pub WriteCimError: isize,
    pub PromptUser: isize,
    pub ShouldProcess: isize,
    pub ShouldContinue: isize,
    pub PostError: isize,
    pub PostCimError: isize,
    pub WriteError: isize,
}
impl ::core::marker::Copy for MI_ContextFT {}
impl ::core::clone::Clone for MI_ContextFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
}
impl ::core::marker::Copy for MI_Datetime {}
impl ::core::clone::Clone for MI_Datetime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MI_Datetime_0 {
    pub timestamp: MI_Timestamp,
    pub interval: MI_Interval,
}
impl ::core::marker::Copy for MI_Datetime_0 {}
impl ::core::clone::Clone for MI_Datetime_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_DatetimeA {}
impl ::core::clone::Clone for MI_DatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeAField {}
impl ::core::clone::Clone for MI_DatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeField {}
impl ::core::clone::Clone for MI_DatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Deserializer {}
impl ::core::clone::Clone for MI_Deserializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
impl ::core::marker::Copy for MI_DeserializerFT {}
impl ::core::clone::Clone for MI_DeserializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MI_Deserializer_ClassObjectNeeded = unsafe extern "system" fn(context: *const ::core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result;
#[repr(C)]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_DestinationOptionsFT,
}
impl ::core::marker::Copy for MI_DestinationOptions {}
impl ::core::clone::Clone for MI_DestinationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_DestinationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub AddCredentials: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_DestinationOptionsFT {}
impl ::core::clone::Clone for MI_DestinationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_DestinationOptions_ImpersonationType_Default: i32 = 0i32;
pub const MI_DestinationOptions_ImpersonationType_None: i32 = 1i32;
pub const MI_DestinationOptions_ImpersonationType_Identify: i32 = 2i32;
pub const MI_DestinationOptions_ImpersonationType_Impersonate: i32 = 3i32;
pub const MI_DestinationOptions_ImpersonationType_Delegate: i32 = 4i32;
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: i32 = 0i32;
pub const MI_ERRORCATEGORY_OPEN_ERROR: i32 = 1i32;
pub const MI_ERRORCATEGORY_CLOS_EERROR: i32 = 2i32;
pub const MI_ERRORCATEGORY_DEVICE_ERROR: i32 = 3i32;
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: i32 = 4i32;
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: i32 = 5i32;
pub const MI_ERRORCATEGORY_INVALID_DATA: i32 = 6i32;
pub const MI_ERRORCATEGORY_INVALID_OPERATION: i32 = 7i32;
pub const MI_ERRORCATEGORY_INVALID_RESULT: i32 = 8i32;
pub const MI_ERRORCATEGORY_INVALID_TYPE: i32 = 9i32;
pub const MI_ERRORCATEGORY_METADATA_ERROR: i32 = 10i32;
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: i32 = 11i32;
pub const MI_ERRORCATEGORY_NOT_INSTALLED: i32 = 12i32;
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: i32 = 13i32;
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: i32 = 14i32;
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: i32 = 15i32;
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: i32 = 16i32;
pub const MI_ERRORCATEGORY_PARSER_ERROR: i32 = 17i32;
pub const MI_ERRORCATEGORY_ACCESS_DENIED: i32 = 18i32;
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: i32 = 19i32;
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: i32 = 20i32;
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: i32 = 21i32;
pub const MI_ERRORCATEGORY_READ_ERROR: i32 = 22i32;
pub const MI_ERRORCATEGORY_WRITE_ERROR: i32 = 23i32;
pub const MI_ERRORCATEGORY_FROM_STDERR: i32 = 24i32;
pub const MI_ERRORCATEGORY_SECURITY_ERROR: i32 = 25i32;
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: i32 = 26i32;
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: i32 = 27i32;
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: i32 = 28i32;
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: i32 = 29i32;
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: i32 = 30i32;
pub const MI_ERRORCATEGORY_NOT_ENABLED: i32 = 31i32;
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
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
}
impl ::core::marker::Copy for MI_FeatureDecl {}
impl ::core::clone::Clone for MI_FeatureDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Filter {
    pub ft: *mut MI_FilterFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Filter {}
impl ::core::clone::Clone for MI_Filter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
impl ::core::marker::Copy for MI_FilterFT {}
impl ::core::clone::Clone for MI_FilterFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_HostedProviderFT,
}
impl ::core::marker::Copy for MI_HostedProvider {}
impl ::core::clone::Clone for MI_HostedProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
impl ::core::marker::Copy for MI_HostedProviderFT {}
impl ::core::clone::Clone for MI_HostedProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Instance {
    pub ft: *mut MI_InstanceFT,
    pub classDecl: *mut MI_ClassDecl,
    pub serverName: *mut u16,
    pub nameSpace: *mut u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Instance {}
impl ::core::clone::Clone for MI_Instance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_InstanceA {}
impl ::core::clone::Clone for MI_InstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceAField {}
impl ::core::clone::Clone for MI_InstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
impl ::core::marker::Copy for MI_InstanceExFT {}
impl ::core::clone::Clone for MI_InstanceExFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_InstanceFT {
    pub Clone: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub IsA: isize,
    pub GetClassNameA: isize,
    pub SetNameSpace: isize,
    pub GetNameSpace: isize,
    pub GetElementCount: isize,
    pub AddElement: isize,
    pub SetElement: isize,
    pub SetElementAt: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub ClearElement: isize,
    pub ClearElementAt: isize,
    pub GetServerName: isize,
    pub SetServerName: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_InstanceFT {}
impl ::core::clone::Clone for MI_InstanceFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceField {}
impl ::core::clone::Clone for MI_InstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Interval {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub microseconds: u32,
    pub __padding1: u32,
    pub __padding2: u32,
    pub __padding3: u32,
}
impl ::core::marker::Copy for MI_Interval {}
impl ::core::clone::Clone for MI_Interval {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_LOCALE_TYPE_REQUESTED_UI: i32 = 0i32;
pub const MI_LOCALE_TYPE_REQUESTED_DATA: i32 = 1i32;
pub const MI_LOCALE_TYPE_CLOSEST_UI: i32 = 2i32;
pub const MI_LOCALE_TYPE_CLOSEST_DATA: i32 = 3i32;
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
pub type MI_MainFunction = unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module;
#[repr(C)]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *mut *mut MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *mut u16,
    pub propagator: *mut u16,
    pub schema: *mut MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl ::core::marker::Copy for MI_MethodDecl {}
impl ::core::clone::Clone for MI_MethodDecl {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MI_MethodDecl_Invoke = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance);
#[repr(C)]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *mut MI_ProviderFT,
}
impl ::core::marker::Copy for MI_Module {}
impl ::core::clone::Clone for MI_Module {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MI_Module_Load = unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context);
#[repr(C)]
pub struct MI_Module_Self(pub u8);
pub type MI_Module_Unload = unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context);
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
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *mut *mut MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ObjectDecl {}
impl ::core::clone::Clone for MI_ObjectDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_OperationFT,
}
impl ::core::marker::Copy for MI_Operation {}
impl ::core::clone::Clone for MI_Operation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MI_OperationCallback_Class = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize);
pub type MI_OperationCallback_Indication = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize);
pub type MI_OperationCallback_Instance = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize);
pub type MI_OperationCallback_PromptUser = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize);
pub const MI_OperationCallback_ResponseType_No: i32 = 0i32;
pub const MI_OperationCallback_ResponseType_Yes: i32 = 1i32;
pub const MI_OperationCallback_ResponseType_NoToAll: i32 = 2i32;
pub const MI_OperationCallback_ResponseType_YesToAll: i32 = 3i32;
pub type MI_OperationCallback_StreamedParameter = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize);
pub type MI_OperationCallback_WriteError = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize);
pub type MI_OperationCallback_WriteMessage = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, channel: u32, message: *const u16);
pub type MI_OperationCallback_WriteProgress = unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32);
#[repr(C)]
pub struct MI_OperationCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub promptUser: MI_OperationCallback_PromptUser,
    pub writeError: MI_OperationCallback_WriteError,
    pub writeMessage: MI_OperationCallback_WriteMessage,
    pub writeProgress: MI_OperationCallback_WriteProgress,
    pub instanceResult: MI_OperationCallback_Instance,
    pub indicationResult: MI_OperationCallback_Indication,
    pub classResult: MI_OperationCallback_Class,
    pub streamedParameterResult: MI_OperationCallback_StreamedParameter,
}
impl ::core::marker::Copy for MI_OperationCallbacks {}
impl ::core::clone::Clone for MI_OperationCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_OperationFT {}
impl ::core::clone::Clone for MI_OperationFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_OperationOptionsFT,
}
impl ::core::marker::Copy for MI_OperationOptions {}
impl ::core::clone::Clone for MI_OperationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_OperationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetCustomOption: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetEnabledChannels: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_OperationOptionsFT {}
impl ::core::clone::Clone for MI_OperationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *mut u16,
    pub subscript: u32,
    pub offset: u32,
}
impl ::core::marker::Copy for MI_ParameterDecl {}
impl ::core::clone::Clone for MI_ParameterDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_ParameterSetFT,
}
impl ::core::marker::Copy for MI_ParameterSet {}
impl ::core::clone::Clone for MI_ParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
impl ::core::marker::Copy for MI_ParameterSetFT {}
impl ::core::clone::Clone for MI_ParameterSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_PROMPTTYPE_NORMAL: i32 = 0i32;
pub const MI_PROMPTTYPE_CRITICAL: i32 = 1i32;
#[repr(C)]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *mut u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *mut u16,
    pub propagator: *mut u16,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_PropertyDecl {}
impl ::core::clone::Clone for MI_PropertyDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_PropertySet {
    pub ft: *mut MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_PropertySet {}
impl ::core::clone::Clone for MI_PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_PropertySetFT {
    pub GetElementCount: isize,
    pub ContainsElement: isize,
    pub AddElement: isize,
    pub GetElementAt: isize,
    pub Clear: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_PropertySetFT {}
impl ::core::clone::Clone for MI_PropertySetFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_PROVIDER_ARCHITECTURE_32BIT: i32 = 0i32;
pub const MI_PROVIDER_ARCHITECTURE_64BIT: i32 = 1i32;
#[repr(C)]
pub struct MI_ProviderFT {
    pub Load: MI_ProviderFT_Load,
    pub Unload: MI_ProviderFT_Unload,
    pub GetInstance: MI_ProviderFT_GetInstance,
    pub EnumerateInstances: MI_ProviderFT_EnumerateInstances,
    pub CreateInstance: MI_ProviderFT_CreateInstance,
    pub ModifyInstance: MI_ProviderFT_ModifyInstance,
    pub DeleteInstance: MI_ProviderFT_DeleteInstance,
    pub AssociatorInstances: MI_ProviderFT_AssociatorInstances,
    pub ReferenceInstances: MI_ProviderFT_ReferenceInstances,
    pub EnableIndications: MI_ProviderFT_EnableIndications,
    pub DisableIndications: MI_ProviderFT_DisableIndications,
    pub Subscribe: MI_ProviderFT_Subscribe,
    pub Unsubscribe: MI_ProviderFT_Unsubscribe,
    pub Invoke: MI_ProviderFT_Invoke,
}
impl ::core::marker::Copy for MI_ProviderFT {}
impl ::core::clone::Clone for MI_ProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MI_ProviderFT_AssociatorInstances = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter);
pub type MI_ProviderFT_CreateInstance = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance);
pub type MI_ProviderFT_DeleteInstance = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance);
pub type MI_ProviderFT_DisableIndications = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16);
pub type MI_ProviderFT_EnableIndications = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16);
pub type MI_ProviderFT_EnumerateInstances = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter);
pub type MI_ProviderFT_GetInstance = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet);
pub type MI_ProviderFT_Invoke = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance);
pub type MI_ProviderFT_Load = unsafe extern "system" fn(self_: *mut *mut ::core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context);
pub type MI_ProviderFT_ModifyInstance = unsafe extern "system" fn(self_: *mut ::core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet);
pub type MI_ProviderFT_ReferenceInstances = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter);
pub type MI_ProviderFT_Subscribe = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut ::core::ffi::c_void);
pub type MI_ProviderFT_Unload = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context);
pub type MI_ProviderFT_Unsubscribe = unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const ::core::ffi::c_void);
#[repr(C)]
pub struct MI_Qualifier {
    pub name: *mut u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_Qualifier {}
impl ::core::clone::Clone for MI_Qualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_QualifierDecl {
    pub name: *mut u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_QualifierDecl {}
impl ::core::clone::Clone for MI_QualifierDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_QualifierSetFT,
}
impl ::core::marker::Copy for MI_QualifierSet {}
impl ::core::clone::Clone for MI_QualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
impl ::core::marker::Copy for MI_QualifierSetFT {}
impl ::core::clone::Clone for MI_QualifierSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real32A {}
impl ::core::clone::Clone for MI_Real32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32AField {}
impl ::core::clone::Clone for MI_Real32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32Field {}
impl ::core::clone::Clone for MI_Real32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real64A {}
impl ::core::clone::Clone for MI_Real64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64AField {}
impl ::core::clone::Clone for MI_Real64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64Field {}
impl ::core::clone::Clone for MI_Real64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ReferenceA {}
impl ::core::clone::Clone for MI_ReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceAField {}
impl ::core::clone::Clone for MI_ReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceField {}
impl ::core::clone::Clone for MI_ReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_RESULT_OK: i32 = 0i32;
pub const MI_RESULT_FAILED: i32 = 1i32;
pub const MI_RESULT_ACCESS_DENIED: i32 = 2i32;
pub const MI_RESULT_INVALID_NAMESPACE: i32 = 3i32;
pub const MI_RESULT_INVALID_PARAMETER: i32 = 4i32;
pub const MI_RESULT_INVALID_CLASS: i32 = 5i32;
pub const MI_RESULT_NOT_FOUND: i32 = 6i32;
pub const MI_RESULT_NOT_SUPPORTED: i32 = 7i32;
pub const MI_RESULT_CLASS_HAS_CHILDREN: i32 = 8i32;
pub const MI_RESULT_CLASS_HAS_INSTANCES: i32 = 9i32;
pub const MI_RESULT_INVALID_SUPERCLASS: i32 = 10i32;
pub const MI_RESULT_ALREADY_EXISTS: i32 = 11i32;
pub const MI_RESULT_NO_SUCH_PROPERTY: i32 = 12i32;
pub const MI_RESULT_TYPE_MISMATCH: i32 = 13i32;
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: i32 = 14i32;
pub const MI_RESULT_INVALID_QUERY: i32 = 15i32;
pub const MI_RESULT_METHOD_NOT_AVAILABLE: i32 = 16i32;
pub const MI_RESULT_METHOD_NOT_FOUND: i32 = 17i32;
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: i32 = 20i32;
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: i32 = 21i32;
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: i32 = 22i32;
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: i32 = 23i32;
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: i32 = 24i32;
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: i32 = 25i32;
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: i32 = 26i32;
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: i32 = 27i32;
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: i32 = 28i32;
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
#[repr(C)]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *mut *mut MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *mut *mut MI_ClassDecl,
    pub numClassDecls: u32,
}
impl ::core::marker::Copy for MI_SchemaDecl {}
impl ::core::clone::Clone for MI_SchemaDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Serializer {}
impl ::core::clone::Clone for MI_Serializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
impl ::core::marker::Copy for MI_SerializerFT {}
impl ::core::clone::Clone for MI_SerializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Server {
    pub serverFT: *mut MI_ServerFT,
    pub contextFT: *mut MI_ContextFT,
    pub instanceFT: *mut MI_InstanceFT,
    pub propertySetFT: *mut MI_PropertySetFT,
    pub filterFT: *mut MI_FilterFT,
}
impl ::core::marker::Copy for MI_Server {}
impl ::core::clone::Clone for MI_Server {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
impl ::core::marker::Copy for MI_ServerFT {}
impl ::core::clone::Clone for MI_ServerFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_SessionFT,
}
impl ::core::marker::Copy for MI_Session {}
impl ::core::clone::Clone for MI_Session {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl ::core::marker::Copy for MI_SessionCallbacks {}
impl ::core::clone::Clone for MI_SessionCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_SessionFT {
    pub Close: isize,
    pub GetApplication: isize,
    pub GetInstance: isize,
    pub ModifyInstance: isize,
    pub CreateInstance: isize,
    pub DeleteInstance: isize,
    pub Invoke: isize,
    pub EnumerateInstances: isize,
    pub QueryInstances: isize,
    pub AssociatorInstances: isize,
    pub ReferenceInstances: isize,
    pub Subscribe: isize,
    pub GetClass: isize,
    pub EnumerateClasses: isize,
    pub TestConnection: isize,
}
impl ::core::marker::Copy for MI_SessionFT {}
impl ::core::clone::Clone for MI_SessionFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint16A {}
impl ::core::clone::Clone for MI_Sint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16AField {}
impl ::core::clone::Clone for MI_Sint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16Field {}
impl ::core::clone::Clone for MI_Sint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint32A {}
impl ::core::clone::Clone for MI_Sint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32AField {}
impl ::core::clone::Clone for MI_Sint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32Field {}
impl ::core::clone::Clone for MI_Sint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint64A {}
impl ::core::clone::Clone for MI_Sint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64AField {}
impl ::core::clone::Clone for MI_Sint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64Field {}
impl ::core::clone::Clone for MI_Sint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint8A {}
impl ::core::clone::Clone for MI_Sint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8AField {}
impl ::core::clone::Clone for MI_Sint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8Field {}
impl ::core::clone::Clone for MI_Sint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_StringA {}
impl ::core::clone::Clone for MI_StringA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringAField {}
impl ::core::clone::Clone for MI_StringAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringField {}
impl ::core::clone::Clone for MI_StringField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_SubscriptionDeliveryOptionsFT,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptions {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_SubscriptionDeliveryOptionsFT {
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetDateTime: isize,
    pub SetInterval: isize,
    pub AddCredentials: isize,
    pub Delete: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetDateTime: isize,
    pub GetInterval: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_SubscriptionDeliveryType_Pull: i32 = 1i32;
pub const MI_SubscriptionDeliveryType_Push: i32 = 2i32;
#[repr(C)]
pub struct MI_Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microseconds: u32,
    pub utc: i32,
}
impl ::core::marker::Copy for MI_Timestamp {}
impl ::core::clone::Clone for MI_Timestamp {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_BOOLEAN: i32 = 0i32;
pub const MI_UINT8: i32 = 1i32;
pub const MI_SINT8: i32 = 2i32;
pub const MI_UINT16: i32 = 3i32;
pub const MI_SINT16: i32 = 4i32;
pub const MI_UINT32: i32 = 5i32;
pub const MI_SINT32: i32 = 6i32;
pub const MI_UINT64: i32 = 7i32;
pub const MI_SINT64: i32 = 8i32;
pub const MI_REAL32: i32 = 9i32;
pub const MI_REAL64: i32 = 10i32;
pub const MI_CHAR16: i32 = 11i32;
pub const MI_DATETIME: i32 = 12i32;
pub const MI_STRING: i32 = 13i32;
pub const MI_REFERENCE: i32 = 14i32;
pub const MI_INSTANCE: i32 = 15i32;
pub const MI_BOOLEANA: i32 = 16i32;
pub const MI_UINT8A: i32 = 17i32;
pub const MI_SINT8A: i32 = 18i32;
pub const MI_UINT16A: i32 = 19i32;
pub const MI_SINT16A: i32 = 20i32;
pub const MI_UINT32A: i32 = 21i32;
pub const MI_SINT32A: i32 = 22i32;
pub const MI_UINT64A: i32 = 23i32;
pub const MI_SINT64A: i32 = 24i32;
pub const MI_REAL32A: i32 = 25i32;
pub const MI_REAL64A: i32 = 26i32;
pub const MI_CHAR16A: i32 = 27i32;
pub const MI_DATETIMEA: i32 = 28i32;
pub const MI_STRINGA: i32 = 29i32;
pub const MI_REFERENCEA: i32 = 30i32;
pub const MI_INSTANCEA: i32 = 31i32;
pub const MI_ARRAY: i32 = 16i32;
#[repr(C)]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint16A {}
impl ::core::clone::Clone for MI_Uint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16AField {}
impl ::core::clone::Clone for MI_Uint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16Field {}
impl ::core::clone::Clone for MI_Uint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint32A {}
impl ::core::clone::Clone for MI_Uint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32AField {}
impl ::core::clone::Clone for MI_Uint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32Field {}
impl ::core::clone::Clone for MI_Uint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint64A {}
impl ::core::clone::Clone for MI_Uint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64AField {}
impl ::core::clone::Clone for MI_Uint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64Field {}
impl ::core::clone::Clone for MI_Uint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint8A {}
impl ::core::clone::Clone for MI_Uint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8AField {}
impl ::core::clone::Clone for MI_Uint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8Field {}
impl ::core::clone::Clone for MI_Uint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_UserCredentials {
    pub authenticationType: *mut u16,
    pub credentials: MI_UserCredentials_0,
}
impl ::core::marker::Copy for MI_UserCredentials {}
impl ::core::clone::Clone for MI_UserCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *mut u16,
}
impl ::core::marker::Copy for MI_UserCredentials_0 {}
impl ::core::clone::Clone for MI_UserCredentials_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_UsernamePasswordCreds {
    pub domain: *mut u16,
    pub username: *mut u16,
    pub password: *mut u16,
}
impl ::core::marker::Copy for MI_UsernamePasswordCreds {}
impl ::core::clone::Clone for MI_UsernamePasswordCreds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
}
impl ::core::marker::Copy for MI_UtilitiesFT {}
impl ::core::clone::Clone for MI_UtilitiesFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MI_Value {
    pub boolean: u8,
    pub uint8: u8,
    pub sint8: i8,
    pub uint16: u16,
    pub sint16: i16,
    pub uint32: u32,
    pub sint32: i32,
    pub uint64: u64,
    pub sint64: i64,
    pub real32: f32,
    pub real64: f64,
    pub char16: u16,
    pub datetime: MI_Datetime,
    pub string: *mut u16,
    pub instance: *mut MI_Instance,
    pub reference: *mut MI_Instance,
    pub booleana: MI_BooleanA,
    pub uint8a: MI_Uint8A,
    pub sint8a: MI_Sint8A,
    pub uint16a: MI_Uint16A,
    pub sint16a: MI_Sint16A,
    pub uint32a: MI_Uint32A,
    pub sint32a: MI_Sint32A,
    pub uint64a: MI_Uint64A,
    pub sint64a: MI_Sint64A,
    pub real32a: MI_Real32A,
    pub real64a: MI_Real64A,
    pub char16a: MI_Char16A,
    pub datetimea: MI_DatetimeA,
    pub stringa: MI_StringA,
    pub referencea: MI_ReferenceA,
    pub instancea: MI_InstanceA,
    pub array: MI_Array,
}
impl ::core::marker::Copy for MI_Value {}
impl ::core::clone::Clone for MI_Value {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const MofCompiler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1840224087, data2: 11831, data3: 4562, data4: [174, 201, 0, 192, 79, 182, 136, 32] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: super::super::Foundation::PWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut ::core::ffi::c_void,
    pub m_pbTruthTable: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrix {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrixList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrixList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: IWbemPath,
    pub m_pszPath: super::super::Foundation::PWSTR,
    pub m_pszQueryText: super::super::Foundation::PWSTR,
    pub m_pszResultClass: super::super::Foundation::PWSTR,
    pub m_pszAssocClass: super::super::Foundation::PWSTR,
    pub m_pszRole: super::super::Foundation::PWSTR,
    pub m_pszResultRole: super::super::Foundation::PWSTR,
    pub m_pszRequiredQualifier: super::super::Foundation::PWSTR,
    pub m_pszRequiredAssocQualifier: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAssocQueryInf {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemDateTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205845588, data2: 53110, data3: 4563, data4: [179, 143, 0, 16, 90, 31, 71, 58] };
pub const SWbemEventSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183192, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemLastError: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3271487148, data2: 53197, data3: 4561, data4: [139, 5, 0, 96, 8, 6, 217, 182] };
pub const SWbemLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1990607192, data2: 52033, data3: 4561, data4: [139, 2, 0, 96, 8, 6, 217, 182] };
pub const SWbemMethod: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183195, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemMethodSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183194, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemNamedValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183200, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemNamedValueSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2599237710, data2: 52875, data3: 4561, data4: [139, 5, 0, 96, 8, 6, 217, 182] };
pub const SWbemObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183202, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemObjectEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3602755506,
    data2: 37941,
    data3: 18719,
    data4: [187, 135, 106, 160, 240, 188, 49, 162],
};
pub const SWbemObjectPath: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1469168678, data2: 52892, data3: 4561, data4: [151, 191, 0, 0, 248, 30, 132, 156] };
pub const SWbemObjectSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183201, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemPrivilege: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653158332, data2: 22532, data3: 4562, data4: [139, 74, 0, 96, 8, 6, 217, 182] };
pub const SWbemPrivilegeSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653158334, data2: 22532, data3: 4562, data4: [139, 74, 0, 96, 8, 6, 217, 182] };
pub const SWbemProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183197, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemPropertySet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183196, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemQualifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183199, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemQualifierSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183198, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut super::super::Foundation::PWSTR,
    pub m_bArraysUsed: super::super::Foundation::BOOL,
    pub m_pbArrayElUsed: *mut super::super::Foundation::BOOL,
    pub m_puArrayIndex: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemQueryQualifiedName {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemQueryQualifiedName {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemRefreshableItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2355647676, data2: 56907, data3: 4563, data4: [179, 144, 0, 16, 90, 31, 71, 58] };
pub const SWbemRefresher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530145628, data2: 55745, data3: 4563, data4: [179, 143, 0, 16, 90, 31, 71, 58] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union SWbemRpnConst {
    pub m_pszStrVal: super::super::Foundation::PWSTR,
    pub m_bBoolVal: super::super::Foundation::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnConst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnConst {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: super::super::Foundation::PWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut super::super::Foundation::PWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut super::super::Foundation::PWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnEncodedQuery {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnEncodedQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: super::super::Foundation::PWSTR,
    pub m_pszLeftFunc: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnQueryToken {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnQueryToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
impl ::core::marker::Copy for SWbemRpnTokenList {}
impl ::core::clone::Clone for SWbemRpnTokenList {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemSecurity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3041748713, data2: 8839, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183203, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemServicesEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1659183836, data2: 36083, data3: 16552, data4: [139, 46, 55, 213, 149, 101, 30, 64] };
pub const SWbemSink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1970375834, data2: 61481, data3: 4561, data4: [161, 172, 0, 192, 79, 182, 194, 35] };
pub const UnsecuredApartment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1237131304, data2: 5411, data3: 4561, data4: [173, 121, 0, 192, 79, 216, 253, 255] };
pub const WBEM_NO_ERROR: i32 = 0i32;
pub const WBEM_S_NO_ERROR: i32 = 0i32;
pub const WBEM_S_SAME: i32 = 0i32;
pub const WBEM_S_FALSE: i32 = 1i32;
pub const WBEM_S_ALREADY_EXISTS: i32 = 262145i32;
pub const WBEM_S_RESET_TO_DEFAULT: i32 = 262146i32;
pub const WBEM_S_DIFFERENT: i32 = 262147i32;
pub const WBEM_S_TIMEDOUT: i32 = 262148i32;
pub const WBEM_S_NO_MORE_DATA: i32 = 262149i32;
pub const WBEM_S_OPERATION_CANCELLED: i32 = 262150i32;
pub const WBEM_S_PENDING: i32 = 262151i32;
pub const WBEM_S_DUPLICATE_OBJECTS: i32 = 262152i32;
pub const WBEM_S_ACCESS_DENIED: i32 = 262153i32;
pub const WBEM_S_PARTIAL_RESULTS: i32 = 262160i32;
pub const WBEM_S_SOURCE_NOT_AVAILABLE: i32 = 262167i32;
pub const WBEM_E_FAILED: i32 = -2147217407i32;
pub const WBEM_E_NOT_FOUND: i32 = -2147217406i32;
pub const WBEM_E_ACCESS_DENIED: i32 = -2147217405i32;
pub const WBEM_E_PROVIDER_FAILURE: i32 = -2147217404i32;
pub const WBEM_E_TYPE_MISMATCH: i32 = -2147217403i32;
pub const WBEM_E_OUT_OF_MEMORY: i32 = -2147217402i32;
pub const WBEM_E_INVALID_CONTEXT: i32 = -2147217401i32;
pub const WBEM_E_INVALID_PARAMETER: i32 = -2147217400i32;
pub const WBEM_E_NOT_AVAILABLE: i32 = -2147217399i32;
pub const WBEM_E_CRITICAL_ERROR: i32 = -2147217398i32;
pub const WBEM_E_INVALID_STREAM: i32 = -2147217397i32;
pub const WBEM_E_NOT_SUPPORTED: i32 = -2147217396i32;
pub const WBEM_E_INVALID_SUPERCLASS: i32 = -2147217395i32;
pub const WBEM_E_INVALID_NAMESPACE: i32 = -2147217394i32;
pub const WBEM_E_INVALID_OBJECT: i32 = -2147217393i32;
pub const WBEM_E_INVALID_CLASS: i32 = -2147217392i32;
pub const WBEM_E_PROVIDER_NOT_FOUND: i32 = -2147217391i32;
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: i32 = -2147217390i32;
pub const WBEM_E_PROVIDER_LOAD_FAILURE: i32 = -2147217389i32;
pub const WBEM_E_INITIALIZATION_FAILURE: i32 = -2147217388i32;
pub const WBEM_E_TRANSPORT_FAILURE: i32 = -2147217387i32;
pub const WBEM_E_INVALID_OPERATION: i32 = -2147217386i32;
pub const WBEM_E_INVALID_QUERY: i32 = -2147217385i32;
pub const WBEM_E_INVALID_QUERY_TYPE: i32 = -2147217384i32;
pub const WBEM_E_ALREADY_EXISTS: i32 = -2147217383i32;
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: i32 = -2147217382i32;
pub const WBEM_E_PROPAGATED_QUALIFIER: i32 = -2147217381i32;
pub const WBEM_E_PROPAGATED_PROPERTY: i32 = -2147217380i32;
pub const WBEM_E_UNEXPECTED: i32 = -2147217379i32;
pub const WBEM_E_ILLEGAL_OPERATION: i32 = -2147217378i32;
pub const WBEM_E_CANNOT_BE_KEY: i32 = -2147217377i32;
pub const WBEM_E_INCOMPLETE_CLASS: i32 = -2147217376i32;
pub const WBEM_E_INVALID_SYNTAX: i32 = -2147217375i32;
pub const WBEM_E_NONDECORATED_OBJECT: i32 = -2147217374i32;
pub const WBEM_E_READ_ONLY: i32 = -2147217373i32;
pub const WBEM_E_PROVIDER_NOT_CAPABLE: i32 = -2147217372i32;
pub const WBEM_E_CLASS_HAS_CHILDREN: i32 = -2147217371i32;
pub const WBEM_E_CLASS_HAS_INSTANCES: i32 = -2147217370i32;
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: i32 = -2147217369i32;
pub const WBEM_E_ILLEGAL_NULL: i32 = -2147217368i32;
pub const WBEM_E_INVALID_QUALIFIER_TYPE: i32 = -2147217367i32;
pub const WBEM_E_INVALID_PROPERTY_TYPE: i32 = -2147217366i32;
pub const WBEM_E_VALUE_OUT_OF_RANGE: i32 = -2147217365i32;
pub const WBEM_E_CANNOT_BE_SINGLETON: i32 = -2147217364i32;
pub const WBEM_E_INVALID_CIM_TYPE: i32 = -2147217363i32;
pub const WBEM_E_INVALID_METHOD: i32 = -2147217362i32;
pub const WBEM_E_INVALID_METHOD_PARAMETERS: i32 = -2147217361i32;
pub const WBEM_E_SYSTEM_PROPERTY: i32 = -2147217360i32;
pub const WBEM_E_INVALID_PROPERTY: i32 = -2147217359i32;
pub const WBEM_E_CALL_CANCELLED: i32 = -2147217358i32;
pub const WBEM_E_SHUTTING_DOWN: i32 = -2147217357i32;
pub const WBEM_E_PROPAGATED_METHOD: i32 = -2147217356i32;
pub const WBEM_E_UNSUPPORTED_PARAMETER: i32 = -2147217355i32;
pub const WBEM_E_MISSING_PARAMETER_ID: i32 = -2147217354i32;
pub const WBEM_E_INVALID_PARAMETER_ID: i32 = -2147217353i32;
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: i32 = -2147217352i32;
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: i32 = -2147217351i32;
pub const WBEM_E_INVALID_OBJECT_PATH: i32 = -2147217350i32;
pub const WBEM_E_OUT_OF_DISK_SPACE: i32 = -2147217349i32;
pub const WBEM_E_BUFFER_TOO_SMALL: i32 = -2147217348i32;
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: i32 = -2147217347i32;
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: i32 = -2147217346i32;
pub const WBEM_E_UNKNOWN_PACKET_TYPE: i32 = -2147217345i32;
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: i32 = -2147217344i32;
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: i32 = -2147217343i32;
pub const WBEM_E_INVALID_QUALIFIER: i32 = -2147217342i32;
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: i32 = -2147217341i32;
pub const WBEM_E_TOO_MUCH_DATA: i32 = -2147217340i32;
pub const WBEM_E_SERVER_TOO_BUSY: i32 = -2147217339i32;
pub const WBEM_E_INVALID_FLAVOR: i32 = -2147217338i32;
pub const WBEM_E_CIRCULAR_REFERENCE: i32 = -2147217337i32;
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: i32 = -2147217336i32;
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: i32 = -2147217335i32;
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: i32 = -2147217328i32;
pub const WBEM_E_TOO_MANY_PROPERTIES: i32 = -2147217327i32;
pub const WBEM_E_UPDATE_TYPE_MISMATCH: i32 = -2147217326i32;
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: i32 = -2147217325i32;
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: i32 = -2147217324i32;
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: i32 = -2147217323i32;
pub const WBEM_E_METHOD_DISABLED: i32 = -2147217322i32;
pub const WBEM_E_REFRESHER_BUSY: i32 = -2147217321i32;
pub const WBEM_E_UNPARSABLE_QUERY: i32 = -2147217320i32;
pub const WBEM_E_NOT_EVENT_CLASS: i32 = -2147217319i32;
pub const WBEM_E_MISSING_GROUP_WITHIN: i32 = -2147217318i32;
pub const WBEM_E_MISSING_AGGREGATION_LIST: i32 = -2147217317i32;
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: i32 = -2147217316i32;
pub const WBEM_E_AGGREGATING_BY_OBJECT: i32 = -2147217315i32;
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: i32 = -2147217313i32;
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: i32 = -2147217312i32;
pub const WBEM_E_QUEUE_OVERFLOW: i32 = -2147217311i32;
pub const WBEM_E_PRIVILEGE_NOT_HELD: i32 = -2147217310i32;
pub const WBEM_E_INVALID_OPERATOR: i32 = -2147217309i32;
pub const WBEM_E_LOCAL_CREDENTIALS: i32 = -2147217308i32;
pub const WBEM_E_CANNOT_BE_ABSTRACT: i32 = -2147217307i32;
pub const WBEM_E_AMENDED_OBJECT: i32 = -2147217306i32;
pub const WBEM_E_CLIENT_TOO_SLOW: i32 = -2147217305i32;
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: i32 = -2147217304i32;
pub const WBEM_E_TIMED_OUT: i32 = -2147217303i32;
pub const WBEM_E_INVALID_ASSOCIATION: i32 = -2147217302i32;
pub const WBEM_E_AMBIGUOUS_OPERATION: i32 = -2147217301i32;
pub const WBEM_E_QUOTA_VIOLATION: i32 = -2147217300i32;
pub const WBEM_E_RESERVED_001: i32 = -2147217299i32;
pub const WBEM_E_RESERVED_002: i32 = -2147217298i32;
pub const WBEM_E_UNSUPPORTED_LOCALE: i32 = -2147217297i32;
pub const WBEM_E_HANDLE_OUT_OF_DATE: i32 = -2147217296i32;
pub const WBEM_E_CONNECTION_FAILED: i32 = -2147217295i32;
pub const WBEM_E_INVALID_HANDLE_REQUEST: i32 = -2147217294i32;
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: i32 = -2147217293i32;
pub const WBEM_E_CLASS_NAME_TOO_WIDE: i32 = -2147217292i32;
pub const WBEM_E_METHOD_NAME_TOO_WIDE: i32 = -2147217291i32;
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: i32 = -2147217290i32;
pub const WBEM_E_RERUN_COMMAND: i32 = -2147217289i32;
pub const WBEM_E_DATABASE_VER_MISMATCH: i32 = -2147217288i32;
pub const WBEM_E_VETO_DELETE: i32 = -2147217287i32;
pub const WBEM_E_VETO_PUT: i32 = -2147217286i32;
pub const WBEM_E_INVALID_LOCALE: i32 = -2147217280i32;
pub const WBEM_E_PROVIDER_SUSPENDED: i32 = -2147217279i32;
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: i32 = -2147217278i32;
pub const WBEM_E_NO_SCHEMA: i32 = -2147217277i32;
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: i32 = -2147217276i32;
pub const WBEM_E_PROVIDER_NOT_REGISTERED: i32 = -2147217275i32;
pub const WBEM_E_FATAL_TRANSPORT_ERROR: i32 = -2147217274i32;
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: i32 = -2147217273i32;
pub const WBEM_E_PROVIDER_TIMED_OUT: i32 = -2147217272i32;
pub const WBEM_E_NO_KEY: i32 = -2147217271i32;
pub const WBEM_E_PROVIDER_DISABLED: i32 = -2147217270i32;
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: i32 = -2147213311i32;
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: i32 = -2147213310i32;
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: i32 = -2147213309i32;
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: i32 = -2147205119i32;
pub const WBEMMOF_E_EXPECTED_SEMI: i32 = -2147205118i32;
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: i32 = -2147205117i32;
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: i32 = -2147205116i32;
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: i32 = -2147205115i32;
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: i32 = -2147205114i32;
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: i32 = -2147205113i32;
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: i32 = -2147205112i32;
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: i32 = -2147205111i32;
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: i32 = -2147205110i32;
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: i32 = -2147205109i32;
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: i32 = -2147205108i32;
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: i32 = -2147205107i32;
pub const WBEMMOF_E_UNEXPECTED_ALIAS: i32 = -2147205106i32;
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: i32 = -2147205105i32;
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: i32 = -2147205104i32;
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: i32 = -2147205103i32;
pub const WBEMMOF_E_INVALID_PRAGMA: i32 = -2147205102i32;
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: i32 = -2147205101i32;
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: i32 = -2147205100i32;
pub const WBEMMOF_E_TYPE_MISMATCH: i32 = -2147205099i32;
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: i32 = -2147205098i32;
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: i32 = -2147205097i32;
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: i32 = -2147205096i32;
pub const WBEMMOF_E_EXPECTED_DOLLAR: i32 = -2147205095i32;
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: i32 = -2147205094i32;
pub const WBEMMOF_E_DUPLICATE_PROPERTY: i32 = -2147205093i32;
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: i32 = -2147205092i32;
pub const WBEMMOF_E_OUT_OF_RANGE: i32 = -2147205091i32;
pub const WBEMMOF_E_INVALID_FILE: i32 = -2147205090i32;
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: i32 = -2147205089i32;
pub const WBEMMOF_E_NULL_ARRAY_ELEM: i32 = -2147205088i32;
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: i32 = -2147205087i32;
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: i32 = -2147205086i32;
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: i32 = -2147205085i32;
pub const WBEMMOF_E_MULTIPLE_ALIASES: i32 = -2147205084i32;
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: i32 = -2147205083i32;
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: i32 = -2147205082i32;
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: i32 = -2147205081i32;
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: i32 = -2147205080i32;
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: i32 = -2147205079i32;
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: i32 = -2147205078i32;
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: i32 = -2147205077i32;
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: i32 = -2147205076i32;
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: i32 = -2147205075i32;
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: i32 = -2147205074i32;
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: i32 = -2147205073i32;
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: i32 = -2147205072i32;
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: i32 = -2147205071i32;
pub const WBEMSTATUS_FORMAT_NEWLINE: i32 = 0i32;
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: i32 = 1i32;
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: i32 = 0i32;
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: i32 = 1i32;
pub const WBEM_FLAG_BATCH_IF_NEEDED: i32 = 0i32;
pub const WBEM_FLAG_MUST_BATCH: i32 = 1i32;
pub const WBEM_FLAG_MUST_NOT_BATCH: i32 = 2i32;
pub const WBEM_FLAG_CREATE_OR_UPDATE: i32 = 0i32;
pub const WBEM_FLAG_UPDATE_ONLY: i32 = 1i32;
pub const WBEM_FLAG_CREATE_ONLY: i32 = 2i32;
pub const WBEM_FLAG_UPDATE_COMPATIBLE: i32 = 0i32;
pub const WBEM_FLAG_UPDATE_SAFE_MODE: i32 = 32i32;
pub const WBEM_FLAG_UPDATE_FORCE_MODE: i32 = 64i32;
pub const WBEM_MASK_UPDATE_MODE: i32 = 96i32;
pub const WBEM_FLAG_ADVISORY: i32 = 65536i32;
pub const WBEM_COMPARISON_INCLUDE_ALL: i32 = 0i32;
pub const WBEM_FLAG_IGNORE_QUALIFIERS: i32 = 1i32;
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: i32 = 2i32;
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: i32 = 4i32;
pub const WBEM_FLAG_IGNORE_CLASS: i32 = 8i32;
pub const WBEM_FLAG_IGNORE_CASE: i32 = 16i32;
pub const WBEM_FLAG_IGNORE_FLAVOR: i32 = 32i32;
pub const WBEM_FLAG_CHECK_ONLY: i32 = 1i32;
pub const WBEM_FLAG_AUTORECOVER: i32 = 2i32;
pub const WBEM_FLAG_WMI_CHECK: i32 = 4i32;
pub const WBEM_FLAG_CONSOLE_PRINT: i32 = 8i32;
pub const WBEM_FLAG_DONT_ADD_TO_LIST: i32 = 16i32;
pub const WBEM_FLAG_SPLIT_FILES: i32 = 32i32;
pub const WBEM_FLAG_STORE_FILE: i32 = 256i32;
#[repr(C)]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: ::windows_sys::core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
impl ::core::marker::Copy for WBEM_COMPILE_STATUS_INFO {}
impl ::core::clone::Clone for WBEM_COMPILE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WBEM_FLAG_ALWAYS: i32 = 0i32;
pub const WBEM_FLAG_ONLY_IF_TRUE: i32 = 1i32;
pub const WBEM_FLAG_ONLY_IF_FALSE: i32 = 2i32;
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: i32 = 3i32;
pub const WBEM_MASK_PRIMARY_CONDITION: i32 = 3i32;
pub const WBEM_FLAG_KEYS_ONLY: i32 = 4i32;
pub const WBEM_FLAG_REFS_ONLY: i32 = 8i32;
pub const WBEM_FLAG_LOCAL_ONLY: i32 = 16i32;
pub const WBEM_FLAG_PROPAGATED_ONLY: i32 = 32i32;
pub const WBEM_FLAG_SYSTEM_ONLY: i32 = 48i32;
pub const WBEM_FLAG_NONSYSTEM_ONLY: i32 = 64i32;
pub const WBEM_MASK_CONDITION_ORIGIN: i32 = 112i32;
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: i32 = 256i32;
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: i32 = 512i32;
pub const WBEM_MASK_CLASS_CONDITION: i32 = 768i32;
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: i32 = 64i32;
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: i32 = 128i32;
pub const WBEM_FLAG_CONNECT_PROVIDERS: i32 = 256i32;
pub const WBEM_S_INITIALIZED: i32 = 0i32;
pub const WBEM_S_LIMITED_SERVICE: i32 = 274433i32;
pub const WBEM_S_INDIRECTLY_UPDATED: i32 = 274434i32;
pub const WBEM_S_SUBJECT_TO_SDS: i32 = 274435i32;
pub const WBEM_E_RETRY_LATER: i32 = -2147209215i32;
pub const WBEM_E_RESOURCE_CONTENTION: i32 = -2147209214i32;
pub const WBEM_FLAVOR_DONT_PROPAGATE: i32 = 0i32;
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: i32 = 1i32;
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: i32 = 2i32;
pub const WBEM_FLAVOR_MASK_PROPAGATION: i32 = 15i32;
pub const WBEM_FLAVOR_OVERRIDABLE: i32 = 0i32;
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: i32 = 16i32;
pub const WBEM_FLAVOR_MASK_PERMISSIONS: i32 = 16i32;
pub const WBEM_FLAVOR_ORIGIN_LOCAL: i32 = 0i32;
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: i32 = 32i32;
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: i32 = 64i32;
pub const WBEM_FLAVOR_MASK_ORIGIN: i32 = 96i32;
pub const WBEM_FLAVOR_NOT_AMENDED: i32 = 0i32;
pub const WBEM_FLAVOR_AMENDED: i32 = 128i32;
pub const WBEM_FLAVOR_MASK_AMENDED: i32 = 128i32;
pub const WBEM_FLAG_RETURN_IMMEDIATELY: i32 = 16i32;
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: i32 = 0i32;
pub const WBEM_FLAG_BIDIRECTIONAL: i32 = 0i32;
pub const WBEM_FLAG_FORWARD_ONLY: i32 = 32i32;
pub const WBEM_FLAG_NO_ERROR_OBJECT: i32 = 64i32;
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: i32 = 0i32;
pub const WBEM_FLAG_SEND_STATUS: i32 = 128i32;
pub const WBEM_FLAG_DONT_SEND_STATUS: i32 = 0i32;
pub const WBEM_FLAG_ENSURE_LOCATABLE: i32 = 256i32;
pub const WBEM_FLAG_DIRECT_READ: i32 = 512i32;
pub const WBEM_FLAG_SEND_ONLY_SELECTED: i32 = 0i32;
pub const WBEM_RETURN_WHEN_COMPLETE: i32 = 0i32;
pub const WBEM_RETURN_IMMEDIATELY: i32 = 16i32;
pub const WBEM_MASK_RESERVED_FLAGS: i32 = 126976i32;
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: i32 = 131072i32;
pub const WBEM_FLAG_STRONG_VALIDATION: i32 = 1048576i32;
pub const WBEM_GENUS_CLASS: i32 = 1i32;
pub const WBEM_GENUS_INSTANCE: i32 = 2i32;
pub const WBEMPATH_TEXT: i32 = 1i32;
pub const WBEMPATH_QUOTEDTEXT: i32 = 2i32;
pub const WBEMPATH_COMPRESSED: i32 = 1i32;
pub const WBEMPATH_GET_RELATIVE_ONLY: i32 = 2i32;
pub const WBEMPATH_GET_SERVER_TOO: i32 = 4i32;
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: i32 = 8i32;
pub const WBEMPATH_GET_NAMESPACE_ONLY: i32 = 16i32;
pub const WBEMPATH_GET_ORIGINAL: i32 = 32i32;
pub const WBEM_FLAG_SHORT_NAME: i32 = 1i32;
pub const WBEM_FLAG_LONG_NAME: i32 = 2i32;
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: i32 = 16i32;
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: i32 = 32i32;
pub const WBEM_MAX_IDENTIFIER: i32 = 4096i32;
pub const WBEM_MAX_QUERY: i32 = 16384i32;
pub const WBEM_MAX_PATH: i32 = 8192i32;
pub const WBEM_MAX_OBJECT_NESTING: i32 = 64i32;
pub const WBEM_MAX_USER_PROPERTIES: i32 = 1024i32;
pub const WBEM_FLAG_ALLOW_READ: i32 = 1i32;
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: i32 = 1i32;
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: i32 = 2i32;
pub const WBEMPATH_CREATE_ACCEPT_ALL: i32 = 4i32;
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: i32 = 8i32;
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: i32 = 1i32;
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: i32 = 2i32;
pub const WBEMPATH_INFO_IS_CLASS_REF: i32 = 4i32;
pub const WBEMPATH_INFO_IS_INST_REF: i32 = 8i32;
pub const WBEMPATH_INFO_HAS_SUBSCOPES: i32 = 16i32;
pub const WBEMPATH_INFO_IS_COMPOUND: i32 = 32i32;
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: i32 = 64i32;
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: i32 = 128i32;
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: i32 = 256i32;
pub const WBEMPATH_INFO_V1_COMPLIANT: i32 = 512i32;
pub const WBEMPATH_INFO_V2_COMPLIANT: i32 = 1024i32;
pub const WBEMPATH_INFO_CIM_COMPLIANT: i32 = 2048i32;
pub const WBEMPATH_INFO_IS_SINGLETON: i32 = 4096i32;
pub const WBEMPATH_INFO_IS_PARENT: i32 = 8192i32;
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: i32 = 16384i32;
pub const WBEMPATH_INFO_NATIVE_PATH: i32 = 32768i32;
pub const WBEMPATH_INFO_WMI_PATH: i32 = 65536i32;
pub const WBEMPATH_INFO_PATH_HAD_SERVER: i32 = 131072i32;
pub const WBEM_FLAG_OWNER_UPDATE: i32 = 65536i32;
pub const WBEM_REQUIREMENTS_START_POSTFILTER: i32 = 0i32;
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: i32 = 1i32;
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: i32 = 2i32;
pub const WBEM_FLAG_DEEP: i32 = 0i32;
pub const WBEM_FLAG_SHALLOW: i32 = 1i32;
pub const WBEM_FLAG_PROTOTYPE: i32 = 2i32;
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: i32 = 0i32;
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: i32 = 1i32;
pub const WBEM_ENABLE: i32 = 1i32;
pub const WBEM_METHOD_EXECUTE: i32 = 2i32;
pub const WBEM_FULL_WRITE_REP: i32 = 4i32;
pub const WBEM_PARTIAL_WRITE_REP: i32 = 8i32;
pub const WBEM_WRITE_PROVIDER: i32 = 16i32;
pub const WBEM_REMOTE_ACCESS: i32 = 32i32;
pub const WBEM_RIGHT_SUBSCRIBE: i32 = 64i32;
pub const WBEM_RIGHT_PUBLISH: i32 = 128i32;
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: i32 = 1i32;
pub const WBEM_SHUTDOWN_WMI: i32 = 2i32;
pub const WBEM_SHUTDOWN_OS: i32 = 3i32;
pub const WBEM_STATUS_COMPLETE: i32 = 0i32;
pub const WBEM_STATUS_REQUIREMENTS: i32 = 1i32;
pub const WBEM_STATUS_PROGRESS: i32 = 2i32;
pub const WBEM_STATUS_LOGGING_INFORMATION: i32 = 256i32;
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: i32 = 512i32;
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: i32 = 1024i32;
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: i32 = 2048i32;
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: i32 = 4096i32;
pub const WBEM_FLAG_NO_FLAVORS: i32 = 1i32;
pub const WBEM_NO_WAIT: i32 = 0i32;
pub const WBEM_INFINITE: i32 = -1i32;
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: i32 = 0i32;
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: i32 = 1i32;
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: i32 = 2i32;
pub const WMIExtension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4036451070, data2: 23679, data3: 4562, data4: [139, 116, 0, 16, 75, 42, 251, 65] };
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: i32 = 1i32;
pub const WMIQ_ANALYSIS_ASSOC_QUERY: i32 = 2i32;
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: i32 = 3i32;
pub const WMIQ_ANALYSIS_QUERY_TEXT: i32 = 4i32;
pub const WMIQ_ANALYSIS_RESERVED: i32 = 134217728i32;
pub const WMIQ_ASSOCQ_ASSOCIATORS: i32 = 1i32;
pub const WMIQ_ASSOCQ_REFERENCES: i32 = 2i32;
pub const WMIQ_ASSOCQ_RESULTCLASS: i32 = 4i32;
pub const WMIQ_ASSOCQ_ASSOCCLASS: i32 = 8i32;
pub const WMIQ_ASSOCQ_ROLE: i32 = 16i32;
pub const WMIQ_ASSOCQ_RESULTROLE: i32 = 32i32;
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: i32 = 64i32;
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: i32 = 128i32;
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: i32 = 256i32;
pub const WMIQ_ASSOCQ_KEYSONLY: i32 = 512i32;
pub const WMIQ_ASSOCQ_SCHEMAONLY: i32 = 1024i32;
pub const WMIQ_ASSOCQ_CLASSREFSONLY: i32 = 2048i32;
pub const WMIQ_LF1_BASIC_SELECT: i32 = 1i32;
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: i32 = 2i32;
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: i32 = 3i32;
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: i32 = 4i32;
pub const WMIQ_LF5_COUNT_STAR: i32 = 5i32;
pub const WMIQ_LF6_ORDER_BY: i32 = 6i32;
pub const WMIQ_LF7_DISTINCT: i32 = 7i32;
pub const WMIQ_LF8_ISA: i32 = 8i32;
pub const WMIQ_LF9_THIS: i32 = 9i32;
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: i32 = 10i32;
pub const WMIQ_LF11_ALIASING: i32 = 11i32;
pub const WMIQ_LF12_GROUP_BY_HAVING: i32 = 12i32;
pub const WMIQ_LF13_WMI_WITHIN: i32 = 13i32;
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: i32 = 14i32;
pub const WMIQ_LF15_GO: i32 = 15i32;
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: i32 = 16i32;
pub const WMIQ_LF17_QUALIFIED_NAMES: i32 = 17i32;
pub const WMIQ_LF18_ASSOCIATONS: i32 = 18i32;
pub const WMIQ_LF19_SYSTEM_PROPERTIES: i32 = 19i32;
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: i32 = 20i32;
pub const WMIQ_LF21_SQL89_JOINS: i32 = 21i32;
pub const WMIQ_LF22_SQL92_JOINS: i32 = 22i32;
pub const WMIQ_LF23_SUBSELECTS: i32 = 23i32;
pub const WMIQ_LF24_UMI_EXTENSIONS: i32 = 24i32;
pub const WMIQ_LF25_DATEPART: i32 = 25i32;
pub const WMIQ_LF26_LIKE: i32 = 26i32;
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: i32 = 27i32;
pub const WMIQ_LF28_STANDARD_AGGREGATES: i32 = 28i32;
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: i32 = 29i32;
pub const WMIQ_LF30_WMI_PRAGMAS: i32 = 30i32;
pub const WMIQ_LF31_QUALIFIER_TESTS: i32 = 31i32;
pub const WMIQ_LF32_SP_EXECUTE: i32 = 32i32;
pub const WMIQ_LF33_ARRAY_ACCESS: i32 = 33i32;
pub const WMIQ_LF34_UNION: i32 = 34i32;
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: i32 = 35i32;
pub const WMIQ_LF36_REFERENCE_TESTS: i32 = 36i32;
pub const WMIQ_LF37_SELECT_INTO: i32 = 37i32;
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: i32 = 38i32;
pub const WMIQ_LF39_COUNT_COLUMN: i32 = 39i32;
pub const WMIQ_LF40_BETWEEN: i32 = 40i32;
pub const WMIQ_LF_LAST: i32 = 40i32;
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: i32 = 1i32;
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: i32 = 2i32;
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: i32 = 4i32;
pub const WMIQ_RPNF_PROJECTION: i32 = 8i32;
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: i32 = 16i32;
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: i32 = 32i32;
pub const WMIQ_RPNF_COUNT_STAR: i32 = 64i32;
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: i32 = 128i32;
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: i32 = 256i32;
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: i32 = 512i32;
pub const WMIQ_RPNF_ORDER_BY: i32 = 1024i32;
pub const WMIQ_RPNF_ISA_USED: i32 = 2048i32;
pub const WMIQ_RPNF_GROUP_BY_HAVING: i32 = 4096i32;
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: i32 = 8192i32;
pub const WMIQ_RPN_TOKEN_EXPRESSION: i32 = 1i32;
pub const WMIQ_RPN_TOKEN_AND: i32 = 2i32;
pub const WMIQ_RPN_TOKEN_OR: i32 = 3i32;
pub const WMIQ_RPN_TOKEN_NOT: i32 = 4i32;
pub const WMIQ_RPN_OP_UNDEFINED: i32 = 0i32;
pub const WMIQ_RPN_OP_EQ: i32 = 1i32;
pub const WMIQ_RPN_OP_NE: i32 = 2i32;
pub const WMIQ_RPN_OP_GE: i32 = 3i32;
pub const WMIQ_RPN_OP_LE: i32 = 4i32;
pub const WMIQ_RPN_OP_LT: i32 = 5i32;
pub const WMIQ_RPN_OP_GT: i32 = 6i32;
pub const WMIQ_RPN_OP_LIKE: i32 = 7i32;
pub const WMIQ_RPN_OP_ISA: i32 = 8i32;
pub const WMIQ_RPN_OP_ISNOTA: i32 = 9i32;
pub const WMIQ_RPN_OP_ISNULL: i32 = 10i32;
pub const WMIQ_RPN_OP_ISNOTNULL: i32 = 11i32;
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: i32 = 1i32;
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: i32 = 2i32;
pub const WMIQ_RPN_CONST2: i32 = 4i32;
pub const WMIQ_RPN_CONST: i32 = 8i32;
pub const WMIQ_RPN_RELOP: i32 = 16i32;
pub const WMIQ_RPN_LEFT_FUNCTION: i32 = 32i32;
pub const WMIQ_RPN_RIGHT_FUNCTION: i32 = 64i32;
pub const WMIQ_RPN_GET_TOKEN_TYPE: i32 = 1i32;
pub const WMIQ_RPN_GET_EXPR_SHAPE: i32 = 2i32;
pub const WMIQ_RPN_GET_LEFT_FUNCTION: i32 = 3i32;
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: i32 = 4i32;
pub const WMIQ_RPN_GET_RELOP: i32 = 5i32;
pub const WMIQ_RPN_NEXT_TOKEN: i32 = 1i32;
pub const WMIQ_RPN_FROM_UNARY: i32 = 1i32;
pub const WMIQ_RPN_FROM_PATH: i32 = 2i32;
pub const WMIQ_RPN_FROM_CLASS_LIST: i32 = 4i32;
pub const WMIQ_RPN_FROM_MULTIPLE: i32 = 8i32;
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: i32 = 1i32;
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: i32 = 2i32;
pub const WMI_OBJ_TEXT_WMI_EXT1: i32 = 3i32;
pub const WMI_OBJ_TEXT_WMI_EXT2: i32 = 4i32;
pub const WMI_OBJ_TEXT_WMI_EXT3: i32 = 5i32;
pub const WMI_OBJ_TEXT_WMI_EXT4: i32 = 6i32;
pub const WMI_OBJ_TEXT_WMI_EXT5: i32 = 7i32;
pub const WMI_OBJ_TEXT_WMI_EXT6: i32 = 8i32;
pub const WMI_OBJ_TEXT_WMI_EXT7: i32 = 9i32;
pub const WMI_OBJ_TEXT_WMI_EXT8: i32 = 10i32;
pub const WMI_OBJ_TEXT_WMI_EXT9: i32 = 11i32;
pub const WMI_OBJ_TEXT_WMI_EXT10: i32 = 12i32;
pub const WMI_OBJ_TEXT_LAST: i32 = 13i32;
pub const WbemAdministrativeLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414513100, data2: 37160, data3: 4561, data4: [173, 155, 0, 192, 79, 216, 253, 255] };
pub const WbemAuthenticatedLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3440919350, data2: 37160, data3: 4561, data4: [173, 155, 0, 192, 79, 216, 253, 255] };
pub const wbemAuthenticationLevelDefault: i32 = 0i32;
pub const wbemAuthenticationLevelNone: i32 = 1i32;
pub const wbemAuthenticationLevelConnect: i32 = 2i32;
pub const wbemAuthenticationLevelCall: i32 = 3i32;
pub const wbemAuthenticationLevelPkt: i32 = 4i32;
pub const wbemAuthenticationLevelPktIntegrity: i32 = 5i32;
pub const wbemAuthenticationLevelPktPrivacy: i32 = 6i32;
pub const WbemBackupRestore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3298702022, data2: 48267, data3: 4562, data4: [133, 212, 0, 16, 90, 31, 131, 4] };
pub const wbemChangeFlagCreateOrUpdate: i32 = 0i32;
pub const wbemChangeFlagUpdateOnly: i32 = 1i32;
pub const wbemChangeFlagCreateOnly: i32 = 2i32;
pub const wbemChangeFlagUpdateCompatible: i32 = 0i32;
pub const wbemChangeFlagUpdateSafeMode: i32 = 32i32;
pub const wbemChangeFlagUpdateForceMode: i32 = 64i32;
pub const wbemChangeFlagStrongValidation: i32 = 128i32;
pub const wbemChangeFlagAdvisory: i32 = 65536i32;
pub const wbemCimtypeSint8: i32 = 16i32;
pub const wbemCimtypeUint8: i32 = 17i32;
pub const wbemCimtypeSint16: i32 = 2i32;
pub const wbemCimtypeUint16: i32 = 18i32;
pub const wbemCimtypeSint32: i32 = 3i32;
pub const wbemCimtypeUint32: i32 = 19i32;
pub const wbemCimtypeSint64: i32 = 20i32;
pub const wbemCimtypeUint64: i32 = 21i32;
pub const wbemCimtypeReal32: i32 = 4i32;
pub const wbemCimtypeReal64: i32 = 5i32;
pub const wbemCimtypeBoolean: i32 = 11i32;
pub const wbemCimtypeString: i32 = 8i32;
pub const wbemCimtypeDatetime: i32 = 101i32;
pub const wbemCimtypeReference: i32 = 102i32;
pub const wbemCimtypeChar16: i32 = 103i32;
pub const wbemCimtypeObject: i32 = 13i32;
pub const WbemClassObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2590322822, data2: 5967, data3: 4562, data4: [181, 249, 0, 16, 75, 112, 62, 253] };
pub const wbemComparisonFlagIncludeAll: i32 = 0i32;
pub const wbemComparisonFlagIgnoreQualifiers: i32 = 1i32;
pub const wbemComparisonFlagIgnoreObjectSource: i32 = 2i32;
pub const wbemComparisonFlagIgnoreDefaultValues: i32 = 4i32;
pub const wbemComparisonFlagIgnoreClass: i32 = 8i32;
pub const wbemComparisonFlagIgnoreCase: i32 = 16i32;
pub const wbemComparisonFlagIgnoreFlavor: i32 = 32i32;
pub const wbemConnectFlagUseMaxWait: i32 = 128i32;
pub const WbemContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1732994712, data2: 61074, data3: 4560, data4: [173, 113, 0, 192, 79, 216, 253, 255] };
pub const WbemDCOMTransport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4157484563, data2: 35984, data3: 4561, data4: [158, 123, 0, 192, 79, 195, 36, 168] };
pub const WbemDecoupledBasicEventProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4126627639,
    data2: 10307,
    data3: 20258,
    data4: [147, 61, 199, 106, 151, 205, 166, 47],
};
pub const WbemDecoupledRegistrar: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1291614514,
    data2: 3997,
    data3: 19439,
    data4: [156, 50, 142, 162, 166, 181, 111, 203],
};
pub const WbemDefPath: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3477914629,
    data2: 58053,
    data3: 19933,
    data4: [179, 206, 94, 117, 130, 216, 201, 250],
};
pub const wbemNoErr: i32 = 0i32;
pub const wbemErrFailed: i32 = -2147217407i32;
pub const wbemErrNotFound: i32 = -2147217406i32;
pub const wbemErrAccessDenied: i32 = -2147217405i32;
pub const wbemErrProviderFailure: i32 = -2147217404i32;
pub const wbemErrTypeMismatch: i32 = -2147217403i32;
pub const wbemErrOutOfMemory: i32 = -2147217402i32;
pub const wbemErrInvalidContext: i32 = -2147217401i32;
pub const wbemErrInvalidParameter: i32 = -2147217400i32;
pub const wbemErrNotAvailable: i32 = -2147217399i32;
pub const wbemErrCriticalError: i32 = -2147217398i32;
pub const wbemErrInvalidStream: i32 = -2147217397i32;
pub const wbemErrNotSupported: i32 = -2147217396i32;
pub const wbemErrInvalidSuperclass: i32 = -2147217395i32;
pub const wbemErrInvalidNamespace: i32 = -2147217394i32;
pub const wbemErrInvalidObject: i32 = -2147217393i32;
pub const wbemErrInvalidClass: i32 = -2147217392i32;
pub const wbemErrProviderNotFound: i32 = -2147217391i32;
pub const wbemErrInvalidProviderRegistration: i32 = -2147217390i32;
pub const wbemErrProviderLoadFailure: i32 = -2147217389i32;
pub const wbemErrInitializationFailure: i32 = -2147217388i32;
pub const wbemErrTransportFailure: i32 = -2147217387i32;
pub const wbemErrInvalidOperation: i32 = -2147217386i32;
pub const wbemErrInvalidQuery: i32 = -2147217385i32;
pub const wbemErrInvalidQueryType: i32 = -2147217384i32;
pub const wbemErrAlreadyExists: i32 = -2147217383i32;
pub const wbemErrOverrideNotAllowed: i32 = -2147217382i32;
pub const wbemErrPropagatedQualifier: i32 = -2147217381i32;
pub const wbemErrPropagatedProperty: i32 = -2147217380i32;
pub const wbemErrUnexpected: i32 = -2147217379i32;
pub const wbemErrIllegalOperation: i32 = -2147217378i32;
pub const wbemErrCannotBeKey: i32 = -2147217377i32;
pub const wbemErrIncompleteClass: i32 = -2147217376i32;
pub const wbemErrInvalidSyntax: i32 = -2147217375i32;
pub const wbemErrNondecoratedObject: i32 = -2147217374i32;
pub const wbemErrReadOnly: i32 = -2147217373i32;
pub const wbemErrProviderNotCapable: i32 = -2147217372i32;
pub const wbemErrClassHasChildren: i32 = -2147217371i32;
pub const wbemErrClassHasInstances: i32 = -2147217370i32;
pub const wbemErrQueryNotImplemented: i32 = -2147217369i32;
pub const wbemErrIllegalNull: i32 = -2147217368i32;
pub const wbemErrInvalidQualifierType: i32 = -2147217367i32;
pub const wbemErrInvalidPropertyType: i32 = -2147217366i32;
pub const wbemErrValueOutOfRange: i32 = -2147217365i32;
pub const wbemErrCannotBeSingleton: i32 = -2147217364i32;
pub const wbemErrInvalidCimType: i32 = -2147217363i32;
pub const wbemErrInvalidMethod: i32 = -2147217362i32;
pub const wbemErrInvalidMethodParameters: i32 = -2147217361i32;
pub const wbemErrSystemProperty: i32 = -2147217360i32;
pub const wbemErrInvalidProperty: i32 = -2147217359i32;
pub const wbemErrCallCancelled: i32 = -2147217358i32;
pub const wbemErrShuttingDown: i32 = -2147217357i32;
pub const wbemErrPropagatedMethod: i32 = -2147217356i32;
pub const wbemErrUnsupportedParameter: i32 = -2147217355i32;
pub const wbemErrMissingParameter: i32 = -2147217354i32;
pub const wbemErrInvalidParameterId: i32 = -2147217353i32;
pub const wbemErrNonConsecutiveParameterIds: i32 = -2147217352i32;
pub const wbemErrParameterIdOnRetval: i32 = -2147217351i32;
pub const wbemErrInvalidObjectPath: i32 = -2147217350i32;
pub const wbemErrOutOfDiskSpace: i32 = -2147217349i32;
pub const wbemErrBufferTooSmall: i32 = -2147217348i32;
pub const wbemErrUnsupportedPutExtension: i32 = -2147217347i32;
pub const wbemErrUnknownObjectType: i32 = -2147217346i32;
pub const wbemErrUnknownPacketType: i32 = -2147217345i32;
pub const wbemErrMarshalVersionMismatch: i32 = -2147217344i32;
pub const wbemErrMarshalInvalidSignature: i32 = -2147217343i32;
pub const wbemErrInvalidQualifier: i32 = -2147217342i32;
pub const wbemErrInvalidDuplicateParameter: i32 = -2147217341i32;
pub const wbemErrTooMuchData: i32 = -2147217340i32;
pub const wbemErrServerTooBusy: i32 = -2147217339i32;
pub const wbemErrInvalidFlavor: i32 = -2147217338i32;
pub const wbemErrCircularReference: i32 = -2147217337i32;
pub const wbemErrUnsupportedClassUpdate: i32 = -2147217336i32;
pub const wbemErrCannotChangeKeyInheritance: i32 = -2147217335i32;
pub const wbemErrCannotChangeIndexInheritance: i32 = -2147217328i32;
pub const wbemErrTooManyProperties: i32 = -2147217327i32;
pub const wbemErrUpdateTypeMismatch: i32 = -2147217326i32;
pub const wbemErrUpdateOverrideNotAllowed: i32 = -2147217325i32;
pub const wbemErrUpdatePropagatedMethod: i32 = -2147217324i32;
pub const wbemErrMethodNotImplemented: i32 = -2147217323i32;
pub const wbemErrMethodDisabled: i32 = -2147217322i32;
pub const wbemErrRefresherBusy: i32 = -2147217321i32;
pub const wbemErrUnparsableQuery: i32 = -2147217320i32;
pub const wbemErrNotEventClass: i32 = -2147217319i32;
pub const wbemErrMissingGroupWithin: i32 = -2147217318i32;
pub const wbemErrMissingAggregationList: i32 = -2147217317i32;
pub const wbemErrPropertyNotAnObject: i32 = -2147217316i32;
pub const wbemErrAggregatingByObject: i32 = -2147217315i32;
pub const wbemErrUninterpretableProviderQuery: i32 = -2147217313i32;
pub const wbemErrBackupRestoreWinmgmtRunning: i32 = -2147217312i32;
pub const wbemErrQueueOverflow: i32 = -2147217311i32;
pub const wbemErrPrivilegeNotHeld: i32 = -2147217310i32;
pub const wbemErrInvalidOperator: i32 = -2147217309i32;
pub const wbemErrLocalCredentials: i32 = -2147217308i32;
pub const wbemErrCannotBeAbstract: i32 = -2147217307i32;
pub const wbemErrAmendedObject: i32 = -2147217306i32;
pub const wbemErrClientTooSlow: i32 = -2147217305i32;
pub const wbemErrNullSecurityDescriptor: i32 = -2147217304i32;
pub const wbemErrTimeout: i32 = -2147217303i32;
pub const wbemErrInvalidAssociation: i32 = -2147217302i32;
pub const wbemErrAmbiguousOperation: i32 = -2147217301i32;
pub const wbemErrQuotaViolation: i32 = -2147217300i32;
pub const wbemErrTransactionConflict: i32 = -2147217299i32;
pub const wbemErrForcedRollback: i32 = -2147217298i32;
pub const wbemErrUnsupportedLocale: i32 = -2147217297i32;
pub const wbemErrHandleOutOfDate: i32 = -2147217296i32;
pub const wbemErrConnectionFailed: i32 = -2147217295i32;
pub const wbemErrInvalidHandleRequest: i32 = -2147217294i32;
pub const wbemErrPropertyNameTooWide: i32 = -2147217293i32;
pub const wbemErrClassNameTooWide: i32 = -2147217292i32;
pub const wbemErrMethodNameTooWide: i32 = -2147217291i32;
pub const wbemErrQualifierNameTooWide: i32 = -2147217290i32;
pub const wbemErrRerunCommand: i32 = -2147217289i32;
pub const wbemErrDatabaseVerMismatch: i32 = -2147217288i32;
pub const wbemErrVetoPut: i32 = -2147217287i32;
pub const wbemErrVetoDelete: i32 = -2147217286i32;
pub const wbemErrInvalidLocale: i32 = -2147217280i32;
pub const wbemErrProviderSuspended: i32 = -2147217279i32;
pub const wbemErrSynchronizationRequired: i32 = -2147217278i32;
pub const wbemErrNoSchema: i32 = -2147217277i32;
pub const wbemErrProviderAlreadyRegistered: i32 = -2147217276i32;
pub const wbemErrProviderNotRegistered: i32 = -2147217275i32;
pub const wbemErrFatalTransportError: i32 = -2147217274i32;
pub const wbemErrEncryptedConnectionRequired: i32 = -2147217273i32;
pub const wbemErrRegistrationTooBroad: i32 = -2147213311i32;
pub const wbemErrRegistrationTooPrecise: i32 = -2147213310i32;
pub const wbemErrTimedout: i32 = -2147209215i32;
pub const wbemErrResetToDefault: i32 = -2147209214i32;
pub const wbemFlagReturnImmediately: i32 = 16i32;
pub const wbemFlagReturnWhenComplete: i32 = 0i32;
pub const wbemFlagBidirectional: i32 = 0i32;
pub const wbemFlagForwardOnly: i32 = 32i32;
pub const wbemFlagNoErrorObject: i32 = 64i32;
pub const wbemFlagReturnErrorObject: i32 = 0i32;
pub const wbemFlagSendStatus: i32 = 128i32;
pub const wbemFlagDontSendStatus: i32 = 0i32;
pub const wbemFlagEnsureLocatable: i32 = 256i32;
pub const wbemFlagDirectRead: i32 = 512i32;
pub const wbemFlagSendOnlySelected: i32 = 0i32;
pub const wbemFlagUseAmendedQualifiers: i32 = 131072i32;
pub const wbemFlagGetDefault: i32 = 0i32;
pub const wbemFlagSpawnInstance: i32 = 1i32;
pub const wbemFlagUseCurrentTime: i32 = 1i32;
pub const wbemImpersonationLevelAnonymous: i32 = 1i32;
pub const wbemImpersonationLevelIdentify: i32 = 2i32;
pub const wbemImpersonationLevelImpersonate: i32 = 3i32;
pub const wbemImpersonationLevelDelegate: i32 = 4i32;
pub const WbemLevel1Login: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2344874078, data2: 55403, data3: 4560, data4: [160, 117, 0, 192, 79, 182, 136, 32] };
pub const WbemLocalAddrRes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2701412353, data2: 36734, data3: 4561, data4: [158, 124, 0, 192, 79, 195, 36, 168] };
pub const WbemLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1167128593, data2: 7482, data3: 4560, data4: [137, 31, 0, 170, 0, 75, 46, 36] };
pub const wbemObjectTextFormatCIMDTD20: i32 = 1i32;
pub const wbemObjectTextFormatWMIDTD20: i32 = 2i32;
pub const WbemObjectTextSrc: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2367444381,
    data2: 34032,
    data3: 19379,
    data4: [167, 213, 86, 167, 67, 90, 155, 166],
};
pub const wbemPrivilegeCreateToken: i32 = 1i32;
pub const wbemPrivilegePrimaryToken: i32 = 2i32;
pub const wbemPrivilegeLockMemory: i32 = 3i32;
pub const wbemPrivilegeIncreaseQuota: i32 = 4i32;
pub const wbemPrivilegeMachineAccount: i32 = 5i32;
pub const wbemPrivilegeTcb: i32 = 6i32;
pub const wbemPrivilegeSecurity: i32 = 7i32;
pub const wbemPrivilegeTakeOwnership: i32 = 8i32;
pub const wbemPrivilegeLoadDriver: i32 = 9i32;
pub const wbemPrivilegeSystemProfile: i32 = 10i32;
pub const wbemPrivilegeSystemtime: i32 = 11i32;
pub const wbemPrivilegeProfileSingleProcess: i32 = 12i32;
pub const wbemPrivilegeIncreaseBasePriority: i32 = 13i32;
pub const wbemPrivilegeCreatePagefile: i32 = 14i32;
pub const wbemPrivilegeCreatePermanent: i32 = 15i32;
pub const wbemPrivilegeBackup: i32 = 16i32;
pub const wbemPrivilegeRestore: i32 = 17i32;
pub const wbemPrivilegeShutdown: i32 = 18i32;
pub const wbemPrivilegeDebug: i32 = 19i32;
pub const wbemPrivilegeAudit: i32 = 20i32;
pub const wbemPrivilegeSystemEnvironment: i32 = 21i32;
pub const wbemPrivilegeChangeNotify: i32 = 22i32;
pub const wbemPrivilegeRemoteShutdown: i32 = 23i32;
pub const wbemPrivilegeUndock: i32 = 24i32;
pub const wbemPrivilegeSyncAgent: i32 = 25i32;
pub const wbemPrivilegeEnableDelegation: i32 = 26i32;
pub const wbemPrivilegeManageVolume: i32 = 27i32;
pub const WbemQuery: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3939016740,
    data2: 8674,
    data3: 17699,
    data4: [173, 115, 167, 26, 10, 162, 245, 106],
};
pub const wbemQueryFlagDeep: i32 = 0i32;
pub const wbemQueryFlagShallow: i32 = 1i32;
pub const wbemQueryFlagPrototype: i32 = 2i32;
pub const WbemRefresher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3340068594, data2: 22046, data3: 4561, data4: [173, 135, 0, 192, 79, 216, 253, 255] };
pub const WbemStatusCodeText: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3951550909, data2: 12851, data3: 4562, data4: [174, 201, 0, 192, 79, 182, 136, 32] };
pub const wbemTextFlagNoFlavors: i32 = 1i32;
pub const wbemTimeoutInfinite: i32 = -1i32;
pub const WbemUnauthenticatedLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1144945529, data2: 56881, data3: 4562, data4: [179, 64, 0, 16, 75, 204, 75, 74] };
pub const WbemUninitializedClassObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046961654, data2: 28936, data3: 4561, data4: [173, 144, 0, 192, 79, 216, 253, 255] };
pub const WBEM_FLAG_INPROC_LOGIN: i32 = 0i32;
pub const WBEM_FLAG_LOCAL_LOGIN: i32 = 1i32;
pub const WBEM_FLAG_REMOTE_LOGIN: i32 = 2i32;
pub const WBEM_AUTHENTICATION_METHOD_MASK: i32 = 15i32;
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: i32 = 16i32;
