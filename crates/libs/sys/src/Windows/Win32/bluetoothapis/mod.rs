#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDevice(hwndparent : super::windef::HWND, hradio : super::winnt::HANDLE, pbtbi : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_sys::core::PCWSTR, ulpasskeylength : u32) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDeviceEx(hwndparentin : super::windef::HWND, hradioin : super::winnt::HANDLE, pbtdiinout : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pbtoobdata : *const BLUETOOTH_OOB_DATA_INFO, authenticationrequirement : super::bthdef::AUTHENTICATION_REQUIREMENTS) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothAuthenticateMultipleDevices(hwndparent : super::windef::HWND, hradio : super::winnt::HANDLE, cdevices : u32, rgbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothDisplayDeviceProperties(hwndparent : super::windef::HWND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothEnableDiscovery(hradio : super::winnt::HANDLE, fenabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothEnableIncomingConnections(hradio : super::winnt::HANDLE, fenabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothEnumerateInstalledServices(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pcserviceinout : *mut u32, pguidservices : *mut windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindDeviceClose(hfind : HBLUETOOTH_DEVICE_FIND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindFirstDevice(pbtsp : *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> HBLUETOOTH_DEVICE_FIND);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindFirstRadio(pbtfrp : *const BLUETOOTH_FIND_RADIO_PARAMS, phradio : *mut super::winnt::HANDLE) -> HBLUETOOTH_RADIO_FIND);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindNextDevice(hfind : HBLUETOOTH_DEVICE_FIND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindNextRadio(hfind : HBLUETOOTH_RADIO_FIND, phradio : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothFindRadioClose(hfind : HBLUETOOTH_RADIO_FIND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothGetDeviceInfo(hradio : super::winnt::HANDLE, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
#[cfg(all(feature = "bthdef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothGetRadioInfo(hradio : super::winnt::HANDLE, pradioinfo : *mut BLUETOOTH_RADIO_INFO) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothIsConnectable(hradio : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothIsDiscoverable(hradio : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("bthprops.cpl" "system" fn BluetoothIsVersionAvailable(majorversion : u8, minorversion : u8) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthentication(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandle : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallback : PFN_AUTHENTICATION_CALLBACK, pvparam : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthenticationEx(pbtdiin : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandleout : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallbackin : PFN_AUTHENTICATION_CALLBACK_EX, pvparam : *const core::ffi::c_void) -> u32);
#[cfg(feature = "bthdef")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothRemoveDevice(paddress : *const BLUETOOTH_ADDRESS_STRUCT) -> u32);
windows_link::link!("bthprops.cpl" "system" fn BluetoothSdpEnumAttributes(psdpstream : *const u8, cbstreamsize : u32, pfncallback : PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSdpGetAttributeValue(precordstream : *const u8, cbrecordlength : u32, usattributeid : u16, pattributedata : *mut SDP_ELEMENT_DATA) -> u32);
#[cfg(all(feature = "bthsdpdef", feature = "minwindef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSdpGetContainerElementData(pcontainerstream : *const u8, cbcontainerlength : u32, pelement : *mut HBLUETOOTH_CONTAINER_ELEMENT, pdata : *mut SDP_ELEMENT_DATA) -> u32);
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSdpGetElementData(psdpstream : *const u8, cbsdpstreamlength : u32, pdata : *mut SDP_ELEMENT_DATA) -> u32);
windows_link::link!("bthprops.cpl" "system" fn BluetoothSdpGetString(precordstream : *const u8, cbrecordlength : u32, pstringdata : *const SDP_STRING_TYPE_DATA, usstringoffset : u16, pszstring : windows_sys::core::PWSTR, pcchstringlength : *mut u32) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSelectDevices(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSelectDevicesFree(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponse(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_sys::core::PCWSTR) -> u32);
#[cfg(all(feature = "bthdef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponseEx(hradioin : super::winnt::HANDLE, pauthresponse : *const BLUETOOTH_AUTHENTICATE_RESPONSE) -> u32);
#[cfg(all(feature = "bthdef", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSetLocalServiceInfo(hradioin : super::winnt::HANDLE, pclassguid : *const windows_sys::core::GUID, ulinstance : u32, pserviceinfoin : *const BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT) -> u32);
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothSetServiceState(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pguidservice : *const windows_sys::core::GUID, dwserviceflags : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("bthprops.cpl" "system" fn BluetoothUnregisterAuthentication(hreghandle : HBLUETOOTH_AUTHENTICATION_REGISTRATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
windows_link::link!("bthprops.cpl" "system" fn BluetoothUpdateDeviceRecord(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_ADDRESS_STRUCT {
    pub Anonymous: BLUETOOTH_ADDRESS_STRUCT_0,
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_ADDRESS_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_ADDRESS_STRUCT_0 {
    pub ullLong: super::bthdef::BTH_ADDR,
    pub rgBytes: [u8; 6],
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_ADDRESS_STRUCT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATE_RESPONSE {
    pub bthAddressRemote: BLUETOOTH_ADDRESS_STRUCT,
    pub authMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub Anonymous: BLUETOOTH_AUTHENTICATE_RESPONSE_0,
    pub negativeResponse: u8,
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    pub pinInfo: BLUETOOTH_PIN_INFO,
    pub oobInfo: BLUETOOTH_OOB_DATA_INFO,
    pub numericCompInfo: BLUETOOTH_NUMERIC_COMPARISON_INFO,
    pub passkeyInfo: BLUETOOTH_PASSKEY_INFO,
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    pub deviceInfo: BLUETOOTH_DEVICE_INFO_STRUCT,
    pub authenticationMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub ioCapability: BLUETOOTH_IO_CAPABILITY,
    pub authenticationRequirements: BLUETOOTH_AUTHENTICATION_REQUIREMENTS,
    pub Anonymous: BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0,
}
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
impl Default for BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0 {
    pub Numeric_Value: u32,
    pub Passkey: u32,
}
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
impl Default for BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BLUETOOTH_AUTHENTICATION_METHOD = i32;
pub const BLUETOOTH_AUTHENTICATION_METHOD_LEGACY: BLUETOOTH_AUTHENTICATION_METHOD = 1;
pub const BLUETOOTH_AUTHENTICATION_METHOD_NUMERIC_COMPARISON: BLUETOOTH_AUTHENTICATION_METHOD = 3;
pub const BLUETOOTH_AUTHENTICATION_METHOD_OOB: BLUETOOTH_AUTHENTICATION_METHOD = 2;
pub const BLUETOOTH_AUTHENTICATION_METHOD_PASSKEY: BLUETOOTH_AUTHENTICATION_METHOD = 5;
pub const BLUETOOTH_AUTHENTICATION_METHOD_PASSKEY_NOTIFICATION: BLUETOOTH_AUTHENTICATION_METHOD = 4;
pub type BLUETOOTH_AUTHENTICATION_REQUIREMENTS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_COD_PAIRS {
    pub ulCODMask: u32,
    pub pcszDescription: windows_sys::core::PCWSTR,
}
impl Default for BLUETOOTH_COD_PAIRS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_DEVICE_INFO_STRUCT {
    pub dwSize: u32,
    pub Address: BLUETOOTH_ADDRESS_STRUCT,
    pub ulClassofDevice: u32,
    pub fConnected: windows_sys::core::BOOL,
    pub fRemembered: windows_sys::core::BOOL,
    pub fAuthenticated: windows_sys::core::BOOL,
    pub stLastSeen: super::minwinbase::SYSTEMTIME,
    pub stLastUsed: super::minwinbase::SYSTEMTIME,
    pub szName: [u16; 248],
}
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
impl Default for BLUETOOTH_DEVICE_INFO_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_DEVICE_NAME_SIZE: u32 = 256;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
    pub dwSize: u32,
    pub fReturnAuthenticated: windows_sys::core::BOOL,
    pub fReturnRemembered: windows_sys::core::BOOL,
    pub fReturnUnknown: windows_sys::core::BOOL,
    pub fReturnConnected: windows_sys::core::BOOL,
    pub fIssueInquiry: windows_sys::core::BOOL,
    pub cTimeoutMultiplier: u8,
    pub hRadio: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for BLUETOOTH_DEVICE_SEARCH_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLUETOOTH_FIND_RADIO_PARAMS {
    pub dwSize: u32,
}
pub type BLUETOOTH_IO_CAPABILITY = i32;
pub const BLUETOOTH_IO_CAPABILITY_DISPLAYONLY: BLUETOOTH_IO_CAPABILITY = 0;
pub const BLUETOOTH_IO_CAPABILITY_DISPLAYYESNO: BLUETOOTH_IO_CAPABILITY = 1;
pub const BLUETOOTH_IO_CAPABILITY_KEYBOARDONLY: BLUETOOTH_IO_CAPABILITY = 2;
pub const BLUETOOTH_IO_CAPABILITY_NOINPUTNOOUTPUT: BLUETOOTH_IO_CAPABILITY = 3;
pub const BLUETOOTH_IO_CAPABILITY_UNDEFINED: BLUETOOTH_IO_CAPABILITY = 255;
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT {
    pub Enabled: windows_sys::core::BOOL,
    pub btAddr: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [u16; 256],
    pub szDeviceString: [u16; 256],
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_MAX_NAME_SIZE: u32 = 248;
pub const BLUETOOTH_MAX_PASSKEY_BUFFER_SIZE: u32 = 17;
pub const BLUETOOTH_MAX_PASSKEY_SIZE: u32 = 16;
pub const BLUETOOTH_MAX_SERVICE_NAME_SIZE: u32 = 256;
pub const BLUETOOTH_MITM_ProtectionNotDefined: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 255;
pub const BLUETOOTH_MITM_ProtectionNotRequired: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 0;
pub const BLUETOOTH_MITM_ProtectionNotRequiredBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 2;
pub const BLUETOOTH_MITM_ProtectionNotRequiredGeneralBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 4;
pub const BLUETOOTH_MITM_ProtectionRequired: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 1;
pub const BLUETOOTH_MITM_ProtectionRequiredBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 3;
pub const BLUETOOTH_MITM_ProtectionRequiredGeneralBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 5;
pub const BLUETOOTH_NULL_ADDRESS: u64 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLUETOOTH_NUMERIC_COMPARISON_INFO {
    pub NumericValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_OOB_DATA_INFO {
    pub C: [u8; 16],
    pub R: [u8; 16],
}
impl Default for BLUETOOTH_OOB_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLUETOOTH_PASSKEY_INFO {
    pub passkey: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_PIN_INFO {
    pub pin: [u8; 16],
    pub pinLength: u8,
}
impl Default for BLUETOOTH_PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_RADIO_INFO {
    pub dwSize: u32,
    pub address: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [u16; 248],
    pub ulClassofDevice: u32,
    pub lmpSubversion: u16,
    pub manufacturer: u16,
}
#[cfg(feature = "bthdef")]
impl Default for BLUETOOTH_RADIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_SELECT_DEVICE_PARAMS {
    pub dwSize: u32,
    pub cNumOfClasses: u32,
    pub prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
    pub pszInfo: windows_sys::core::PWSTR,
    pub hwndParent: super::windef::HWND,
    pub fForceAuthentication: windows_sys::core::BOOL,
    pub fShowAuthenticated: windows_sys::core::BOOL,
    pub fShowRemembered: windows_sys::core::BOOL,
    pub fShowUnknown: windows_sys::core::BOOL,
    pub fAddNewDeviceWizard: windows_sys::core::BOOL,
    pub fSkipServicesPage: windows_sys::core::BOOL,
    pub pfnDeviceCallback: PFN_DEVICE_CALLBACK,
    pub pvParam: *mut core::ffi::c_void,
    pub cNumDevices: u32,
    pub pDevices: PBLUETOOTH_DEVICE_INFO,
}
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "windef"))]
impl Default for BLUETOOTH_SELECT_DEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_SERVICE_DISABLE: u32 = 0;
pub const BLUETOOTH_SERVICE_ENABLE: u32 = 1;
pub const BLUETOOTH_SERVICE_MASK: u32 = 1;
#[cfg(feature = "winnt")]
pub type HBLUETOOTH_AUTHENTICATION_REGISTRATION = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HBLUETOOTH_CONTAINER_ELEMENT = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HBLUETOOTH_DEVICE_FIND = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HBLUETOOTH_RADIO_FIND = super::winnt::HANDLE;
#[cfg(feature = "bthdef")]
pub type PBLUETOOTH_AUTHENTICATE_RESPONSE = *mut BLUETOOTH_AUTHENTICATE_RESPONSE;
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
pub type PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS = *mut BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS;
pub type PBLUETOOTH_AUTHENTICATION_METHOD = *mut BLUETOOTH_AUTHENTICATION_METHOD;
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
pub type PBLUETOOTH_DEVICE_INFO = *mut BLUETOOTH_DEVICE_INFO_STRUCT;
#[cfg(feature = "bthdef")]
pub type PBLUETOOTH_LOCAL_SERVICE_INFO = *mut BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT;
pub type PBLUETOOTH_NUMERIC_COMPARISON_INFO = *mut BLUETOOTH_NUMERIC_COMPARISON_INFO;
pub type PBLUETOOTH_OOB_DATA_INFO = *mut BLUETOOTH_OOB_DATA_INFO;
pub type PBLUETOOTH_PASSKEY_INFO = *mut BLUETOOTH_PASSKEY_INFO;
pub type PBLUETOOTH_PIN_INFO = *mut BLUETOOTH_PIN_INFO;
#[cfg(feature = "bthdef")]
pub type PBLUETOOTH_RADIO_INFO = *mut BLUETOOTH_RADIO_INFO;
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
pub type PFN_AUTHENTICATION_CALLBACK = Option<unsafe extern "system" fn(pvparam: *mut core::ffi::c_void, pdevice: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
pub type PFN_AUTHENTICATION_CALLBACK_EX = Option<unsafe extern "system" fn(pvparam: *const core::ffi::c_void, pauthcallbackparams: *const BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS) -> windows_sys::core::BOOL>;
pub type PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK = Option<unsafe extern "system" fn(uattribid: u32, pvaluestream: *const u8, cbstreamsize: u32, pvparam: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bthdef", feature = "minwinbase"))]
pub type PFN_DEVICE_CALLBACK = Option<unsafe extern "system" fn(pvparam: *mut core::ffi::c_void, pdevice: *const BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
pub type PSDP_ELEMENT_DATA = *mut SDP_ELEMENT_DATA;
pub type PSDP_STRING_TYPE_DATA = *mut SDP_STRING_TYPE_DATA;
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA {
    pub r#type: super::bthsdpdef::SDP_TYPE,
    pub specificType: super::bthsdpdef::SDP_SPECIFICTYPE,
    pub data: SDP_ELEMENT_DATA_0,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub union SDP_ELEMENT_DATA_0 {
    pub int128: super::bthsdpdef::SDP_LARGE_INTEGER_16,
    pub int64: i64,
    pub int32: i32,
    pub int16: i16,
    pub int8: i8,
    pub uint128: super::bthsdpdef::SDP_ULARGE_INTEGER_16,
    pub uint64: u64,
    pub uint32: u32,
    pub uint16: u16,
    pub uint8: u8,
    pub booleanVal: u8,
    pub uuid128: windows_sys::core::GUID,
    pub uuid32: u32,
    pub uuid16: u16,
    pub string: SDP_ELEMENT_DATA_0_0,
    pub url: SDP_ELEMENT_DATA_0_1,
    pub sequence: SDP_ELEMENT_DATA_0_2,
    pub alternative: SDP_ELEMENT_DATA_0_3,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA_0_0 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA_0_1 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA_0_2 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA_0_3 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[cfg(all(feature = "bthsdpdef", feature = "minwindef"))]
impl Default for SDP_ELEMENT_DATA_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDP_STRING_TYPE_DATA {
    pub encoding: u16,
    pub mibeNum: u16,
    pub attributeId: u16,
}
