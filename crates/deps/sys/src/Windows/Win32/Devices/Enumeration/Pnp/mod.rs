#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn SwDeviceClose(hswdevice: HSWDEVICE);
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SwDeviceCreate(pszenumeratorname: super::super::super::Foundation::PWSTR, pszparentdeviceinstance: super::super::super::Foundation::PWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows_sys::core::HRESULT;
    pub fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows_sys::core::GUID, pszreferencestring: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, fenabled: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    pub fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows_sys::core::HRESULT;
    pub fn SwMemFree(pmem: *const ::core::ffi::c_void);
}
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
pub struct HSWDEVICE(i32);
#[repr(transparent)]
pub struct IUPnPAddressFamilyControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPAsyncResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDescriptionDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDescriptionDocumentCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceControlHttpHeaders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccessEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceFinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceFinderCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDeviceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPHttpHeaderControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPRegistrar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPRemoteEndpointInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPReregistrar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPServiceAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPServiceCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPServiceDocumentAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPServiceEnumProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPServices(pub *mut ::core::ffi::c_void);
pub struct SW_DEVICE_CAPABILITIES(i32);
pub struct SW_DEVICE_CREATE_CALLBACK(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO(i32);
pub struct SW_DEVICE_LIFETIME(i32);
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220976i32 as _);
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220736i32 as _);
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const UPNP_E_DEVICE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220972i32 as _);
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220988i32 as _);
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180494i32 as _);
pub const UPNP_E_DEVICE_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180495i32 as _);
pub const UPNP_E_DEVICE_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220969i32 as _);
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180511i32 as _);
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180510i32 as _);
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220970i32 as _);
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220223i32 as _);
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220987i32 as _);
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220986i32 as _);
pub const UPNP_E_INVALID_ACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220985i32 as _);
pub const UPNP_E_INVALID_ARGUMENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220984i32 as _);
pub const UPNP_E_INVALID_DESCRIPTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180509i32 as _);
pub const UPNP_E_INVALID_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220224i32 as _);
pub const UPNP_E_INVALID_ICON: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180507i32 as _);
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180505i32 as _);
pub const UPNP_E_INVALID_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180508i32 as _);
pub const UPNP_E_INVALID_VARIABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220973i32 as _);
pub const UPNP_E_INVALID_XML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180506i32 as _);
pub const UPNP_E_OUT_OF_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220983i32 as _);
pub const UPNP_E_PROTOCOL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220971i32 as _);
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180512i32 as _);
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220989i32 as _);
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180504i32 as _);
pub const UPNP_E_TRANSPORT_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220975i32 as _);
pub const UPNP_E_URLBASE_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180503i32 as _);
pub const UPNP_E_VALUE_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180496i32 as _);
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220974i32 as _);
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
pub struct UPnPDescriptionDocument(i32);
pub struct UPnPDescriptionDocumentEx(i32);
pub struct UPnPDevice(i32);
pub struct UPnPDeviceFinder(i32);
pub struct UPnPDeviceFinderEx(i32);
pub struct UPnPDevices(i32);
pub struct UPnPRegistrar(i32);
pub struct UPnPRemoteEndpointInfo(i32);
pub struct UPnPService(i32);
pub struct UPnPServices(i32);
