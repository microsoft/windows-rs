pub const ADDRESS_FAMILY_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AddressFamily");
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
pub const REMOTE_ADDRESS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RemoteAddress");
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = 8i32;
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = 4i32;
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = 0i32;
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = 1i32;
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = 2i32;
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = 0i32;
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = 2i32;
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = 1i32;
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
pub const UPNP_E_ACTION_REQUEST_FAILED: windows_core::HRESULT = 0x80040210_u32 as _;
pub const UPNP_E_ACTION_SPECIFIC_BASE: windows_core::HRESULT = 0x80040300_u32 as _;
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: windows_core::HRESULT = 0x80040201_u32 as _;
pub const UPNP_E_DEVICE_ERROR: windows_core::HRESULT = 0x80040214_u32 as _;
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: windows_core::HRESULT = 0x80040204_u32 as _;
pub const UPNP_E_DEVICE_NOTREGISTERED: windows_core::HRESULT = 0x8004A032_u32 as _;
pub const UPNP_E_DEVICE_RUNNING: windows_core::HRESULT = 0x8004A031_u32 as _;
pub const UPNP_E_DEVICE_TIMEOUT: windows_core::HRESULT = 0x80040217_u32 as _;
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: windows_core::HRESULT = 0x8004A021_u32 as _;
pub const UPNP_E_DUPLICATE_SERVICE_ID: windows_core::HRESULT = 0x8004A022_u32 as _;
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: windows_core::HRESULT = 0x80040216_u32 as _;
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: windows_core::HRESULT = 0x80040501_u32 as _;
pub const UPNP_E_ICON_ELEMENT_EXPECTED: windows_core::HRESULT = 0x80040205_u32 as _;
pub const UPNP_E_ICON_NODE_INCOMPLETE: windows_core::HRESULT = 0x80040206_u32 as _;
pub const UPNP_E_INVALID_ACTION: windows_core::HRESULT = 0x80040207_u32 as _;
pub const UPNP_E_INVALID_ARGUMENTS: windows_core::HRESULT = 0x80040208_u32 as _;
pub const UPNP_E_INVALID_DESCRIPTION: windows_core::HRESULT = 0x8004A023_u32 as _;
pub const UPNP_E_INVALID_DOCUMENT: windows_core::HRESULT = 0x80040500_u32 as _;
pub const UPNP_E_INVALID_ICON: windows_core::HRESULT = 0x8004A025_u32 as _;
pub const UPNP_E_INVALID_ROOT_NAMESPACE: windows_core::HRESULT = 0x8004A027_u32 as _;
pub const UPNP_E_INVALID_SERVICE: windows_core::HRESULT = 0x8004A024_u32 as _;
pub const UPNP_E_INVALID_VARIABLE: windows_core::HRESULT = 0x80040213_u32 as _;
pub const UPNP_E_INVALID_XML: windows_core::HRESULT = 0x8004A026_u32 as _;
pub const UPNP_E_OUT_OF_SYNC: windows_core::HRESULT = 0x80040209_u32 as _;
pub const UPNP_E_PROTOCOL_ERROR: windows_core::HRESULT = 0x80040215_u32 as _;
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: windows_core::HRESULT = 0x8004A020_u32 as _;
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: windows_core::HRESULT = 0x80040200_u32 as _;
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: windows_core::HRESULT = 0x80040202_u32 as _;
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: windows_core::HRESULT = 0x80040203_u32 as _;
pub const UPNP_E_SUFFIX_TOO_LONG: windows_core::HRESULT = 0x8004A028_u32 as _;
pub const UPNP_E_TRANSPORT_ERROR: windows_core::HRESULT = 0x80040211_u32 as _;
pub const UPNP_E_URLBASE_PRESENT: windows_core::HRESULT = 0x8004A029_u32 as _;
pub const UPNP_E_VALUE_TOO_LONG: windows_core::HRESULT = 0x8004A030_u32 as _;
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: windows_core::HRESULT = 0x80040212_u32 as _;
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
impl windows_core::TypeKind for SW_DEVICE_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SW_DEVICE_LIFETIME(pub i32);
impl windows_core::TypeKind for SW_DEVICE_LIFETIME {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: windows_core::PCWSTR,
    pub pszzHardwareIds: windows_core::PCWSTR,
    pub pszzCompatibleIds: windows_core::PCWSTR,
    pub pContainerId: *const windows_core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: windows_core::PCWSTR,
    pub pszDeviceLocation: windows_core::PCWSTR,
    pub pSecurityDescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SW_DEVICE_CREATE_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const UPnPDescriptionDocument: windows_core::GUID = windows_core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
pub const UPnPDescriptionDocumentEx: windows_core::GUID = windows_core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
pub const UPnPDevice: windows_core::GUID = windows_core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
pub const UPnPDeviceFinder: windows_core::GUID = windows_core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
pub const UPnPDeviceFinderEx: windows_core::GUID = windows_core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
pub const UPnPDevices: windows_core::GUID = windows_core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
pub const UPnPRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
pub const UPnPRemoteEndpointInfo: windows_core::GUID = windows_core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
pub const UPnPService: windows_core::GUID = windows_core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
pub const UPnPServices: windows_core::GUID = windows_core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
pub type SW_DEVICE_CREATE_CALLBACK = Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: windows_core::HRESULT, pcontext: *const core::ffi::c_void, pszdeviceinstanceid: windows_core::PCWSTR)>;
