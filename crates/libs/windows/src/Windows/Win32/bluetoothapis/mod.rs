#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateDevice(hwndparent: Option<super::windef::HWND>, hradio: Option<super::winnt::HANDLE>, pbtbi: *mut BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey: Option<&[u16]>) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDevice(hwndparent : super::windef::HWND, hradio : super::winnt::HANDLE, pbtbi : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_core::PCWSTR, ulpasskeylength : u32) -> u32);
    unsafe { BluetoothAuthenticateDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, hradio.unwrap_or(core::mem::zeroed()) as _, pbtbi as _, core::mem::transmute(pszpasskey.map_or(core::ptr::null(), |slice| slice.as_ptr())), pszpasskey.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateDeviceEx(hwndparentin: Option<super::windef::HWND>, hradioin: Option<super::winnt::HANDLE>, pbtdiinout: *mut BLUETOOTH_DEVICE_INFO_STRUCT, pbtoobdata: Option<*const BLUETOOTH_OOB_DATA_INFO>, authenticationrequirement: super::bthdef::AUTHENTICATION_REQUIREMENTS) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDeviceEx(hwndparentin : super::windef::HWND, hradioin : super::winnt::HANDLE, pbtdiinout : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pbtoobdata : *const BLUETOOTH_OOB_DATA_INFO, authenticationrequirement : super::bthdef::AUTHENTICATION_REQUIREMENTS) -> u32);
    unsafe { BluetoothAuthenticateDeviceEx(hwndparentin.unwrap_or(core::mem::zeroed()) as _, hradioin.unwrap_or(core::mem::zeroed()) as _, pbtdiinout as _, pbtoobdata.unwrap_or(core::mem::zeroed()) as _, authenticationrequirement) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateMultipleDevices(hwndparent: Option<super::windef::HWND>, hradio: Option<super::winnt::HANDLE>, rgbtdi: &mut [BLUETOOTH_DEVICE_INFO_STRUCT]) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateMultipleDevices(hwndparent : super::windef::HWND, hradio : super::winnt::HANDLE, cdevices : u32, rgbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
    unsafe { BluetoothAuthenticateMultipleDevices(hwndparent.unwrap_or(core::mem::zeroed()) as _, hradio.unwrap_or(core::mem::zeroed()) as _, rgbtdi.len().try_into().unwrap(), core::mem::transmute(rgbtdi.as_ptr())) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef"))]
#[inline]
pub unsafe fn BluetoothDisplayDeviceProperties(hwndparent: Option<super::windef::HWND>, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothDisplayDeviceProperties(hwndparent : super::windef::HWND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL);
    unsafe { BluetoothDisplayDeviceProperties(hwndparent.unwrap_or(core::mem::zeroed()) as _, pbtdi as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothEnableDiscovery(hradio: Option<super::winnt::HANDLE>, fenabled: bool) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnableDiscovery(hradio : super::winnt::HANDLE, fenabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { BluetoothEnableDiscovery(hradio.unwrap_or(core::mem::zeroed()) as _, fenabled.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothEnableIncomingConnections(hradio: Option<super::winnt::HANDLE>, fenabled: bool) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnableIncomingConnections(hradio : super::winnt::HANDLE, fenabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { BluetoothEnableIncomingConnections(hradio.unwrap_or(core::mem::zeroed()) as _, fenabled.into()) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothEnumerateInstalledServices(hradio: Option<super::winnt::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pcserviceinout: *mut u32, pguidservices: Option<*mut windows_core::GUID>) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnumerateInstalledServices(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pcserviceinout : *mut u32, pguidservices : *mut windows_core::GUID) -> u32);
    unsafe { BluetoothEnumerateInstalledServices(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pcserviceinout as _, pguidservices.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothFindDeviceClose(hfind: HBLUETOOTH_DEVICE_FIND) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindDeviceClose(hfind : HBLUETOOTH_DEVICE_FIND) -> windows_core::BOOL);
    unsafe { BluetoothFindDeviceClose(hfind) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothFindFirstDevice(pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> HBLUETOOTH_DEVICE_FIND {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindFirstDevice(pbtsp : *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> HBLUETOOTH_DEVICE_FIND);
    unsafe { BluetoothFindFirstDevice(pbtsp, pbtdi as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothFindFirstRadio(pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS, phradio: *mut super::winnt::HANDLE) -> HBLUETOOTH_RADIO_FIND {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindFirstRadio(pbtfrp : *const BLUETOOTH_FIND_RADIO_PARAMS, phradio : *mut super::winnt::HANDLE) -> HBLUETOOTH_RADIO_FIND);
    unsafe { BluetoothFindFirstRadio(pbtfrp, phradio as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothFindNextDevice(hfind: HBLUETOOTH_DEVICE_FIND, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindNextDevice(hfind : HBLUETOOTH_DEVICE_FIND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL);
    unsafe { BluetoothFindNextDevice(hfind, pbtdi as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothFindNextRadio(hfind: HBLUETOOTH_RADIO_FIND, phradio: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindNextRadio(hfind : HBLUETOOTH_RADIO_FIND, phradio : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothFindNextRadio(hfind, phradio as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothFindRadioClose(hfind: HBLUETOOTH_RADIO_FIND) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindRadioClose(hfind : HBLUETOOTH_RADIO_FIND) -> windows_core::BOOL);
    unsafe { BluetoothFindRadioClose(hfind) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothGetDeviceInfo(hradio: Option<super::winnt::HANDLE>, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothGetDeviceInfo(hradio : super::winnt::HANDLE, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
    unsafe { BluetoothGetDeviceInfo(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothGetRadioInfo(hradio: super::winnt::HANDLE, pradioinfo: *mut BLUETOOTH_RADIO_INFO) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothGetRadioInfo(hradio : super::winnt::HANDLE, pradioinfo : *mut BLUETOOTH_RADIO_INFO) -> u32);
    unsafe { BluetoothGetRadioInfo(hradio, pradioinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothIsConnectable(hradio: Option<super::winnt::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsConnectable(hradio : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothIsConnectable(hradio.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothIsDiscoverable(hradio: Option<super::winnt::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsDiscoverable(hradio : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothIsDiscoverable(hradio.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BluetoothIsVersionAvailable(majorversion: u8, minorversion: u8) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsVersionAvailable(majorversion : u8, minorversion : u8) -> windows_core::BOOL);
    unsafe { BluetoothIsVersionAvailable(majorversion, minorversion) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothRegisterForAuthentication(pbtdi: Option<*const BLUETOOTH_DEVICE_INFO_STRUCT>, phreghandle: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallback: PFN_AUTHENTICATION_CALLBACK, pvparam: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthentication(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandle : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallback : PFN_AUTHENTICATION_CALLBACK, pvparam : *const core::ffi::c_void) -> u32);
    unsafe { BluetoothRegisterForAuthentication(pbtdi.unwrap_or(core::mem::zeroed()) as _, phreghandle as _, pfncallback, pvparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothRegisterForAuthenticationEx(pbtdiin: Option<*const BLUETOOTH_DEVICE_INFO_STRUCT>, phreghandleout: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallbackin: PFN_AUTHENTICATION_CALLBACK_EX, pvparam: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthenticationEx(pbtdiin : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandleout : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallbackin : PFN_AUTHENTICATION_CALLBACK_EX, pvparam : *const core::ffi::c_void) -> u32);
    unsafe { BluetoothRegisterForAuthenticationEx(pbtdiin.unwrap_or(core::mem::zeroed()) as _, phreghandleout as _, pfncallbackin, pvparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_bthdef")]
#[inline]
pub unsafe fn BluetoothRemoveDevice(paddress: *const BLUETOOTH_ADDRESS_STRUCT) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRemoveDevice(paddress : *const BLUETOOTH_ADDRESS_STRUCT) -> u32);
    unsafe { BluetoothRemoveDevice(paddress) }
}
#[inline]
pub unsafe fn BluetoothSdpEnumAttributes(psdpstream: &[u8], pfncallback: PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpEnumAttributes(psdpstream : *const u8, cbstreamsize : u32, pfncallback : PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { BluetoothSdpEnumAttributes(core::mem::transmute(psdpstream.as_ptr()), psdpstream.len().try_into().unwrap(), pfncallback, pvparam) }
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn BluetoothSdpGetAttributeValue(precordstream: &[u8], usattributeid: u16, pattributedata: *mut SDP_ELEMENT_DATA) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetAttributeValue(precordstream : *const u8, cbrecordlength : u32, usattributeid : u16, pattributedata : *mut SDP_ELEMENT_DATA) -> u32);
    unsafe { BluetoothSdpGetAttributeValue(core::mem::transmute(precordstream.as_ptr()), precordstream.len().try_into().unwrap(), usattributeid, pattributedata as _) }
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothSdpGetContainerElementData(pcontainerstream: &[u8], pelement: *mut HBLUETOOTH_CONTAINER_ELEMENT, pdata: *mut SDP_ELEMENT_DATA) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetContainerElementData(pcontainerstream : *const u8, cbcontainerlength : u32, pelement : *mut HBLUETOOTH_CONTAINER_ELEMENT, pdata : *mut SDP_ELEMENT_DATA) -> u32);
    unsafe { BluetoothSdpGetContainerElementData(core::mem::transmute(pcontainerstream.as_ptr()), pcontainerstream.len().try_into().unwrap(), pelement as _, pdata as _) }
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn BluetoothSdpGetElementData(psdpstream: &[u8], pdata: *mut SDP_ELEMENT_DATA) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetElementData(psdpstream : *const u8, cbsdpstreamlength : u32, pdata : *mut SDP_ELEMENT_DATA) -> u32);
    unsafe { BluetoothSdpGetElementData(core::mem::transmute(psdpstream.as_ptr()), psdpstream.len().try_into().unwrap(), pdata as _) }
}
#[inline]
pub unsafe fn BluetoothSdpGetString(precordstream: &[u8], pstringdata: Option<*const SDP_STRING_TYPE_DATA>, usstringoffset: u16, pszstring: windows_core::PWSTR, pcchstringlength: *mut u32) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetString(precordstream : *const u8, cbrecordlength : u32, pstringdata : *const SDP_STRING_TYPE_DATA, usstringoffset : u16, pszstring : windows_core::PWSTR, pcchstringlength : *mut u32) -> u32);
    unsafe { BluetoothSdpGetString(core::mem::transmute(precordstream.as_ptr()), precordstream.len().try_into().unwrap(), pstringdata.unwrap_or(core::mem::zeroed()) as _, usstringoffset, core::mem::transmute(pszstring), pcchstringlength as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef"))]
#[inline]
pub unsafe fn BluetoothSelectDevices(pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSelectDevices(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL);
    unsafe { BluetoothSelectDevices(pbtsdp as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef"))]
#[inline]
pub unsafe fn BluetoothSelectDevicesFree(pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSelectDevicesFree(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL);
    unsafe { BluetoothSelectDevicesFree(pbtsdp as _) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothSendAuthenticationResponse<P2>(hradio: Option<super::winnt::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponse(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_core::PCWSTR) -> u32);
    unsafe { BluetoothSendAuthenticationResponse(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pszpasskey.param().abi()) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothSendAuthenticationResponseEx(hradioin: Option<super::winnt::HANDLE>, pauthresponse: *const BLUETOOTH_AUTHENTICATE_RESPONSE) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponseEx(hradioin : super::winnt::HANDLE, pauthresponse : *const BLUETOOTH_AUTHENTICATE_RESPONSE) -> u32);
    unsafe { BluetoothSendAuthenticationResponseEx(hradioin.unwrap_or(core::mem::zeroed()) as _, pauthresponse) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothSetLocalServiceInfo(hradioin: Option<super::winnt::HANDLE>, pclassguid: *const windows_core::GUID, ulinstance: u32, pserviceinfoin: *const BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSetLocalServiceInfo(hradioin : super::winnt::HANDLE, pclassguid : *const windows_core::GUID, ulinstance : u32, pserviceinfoin : *const BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT) -> u32);
    unsafe { BluetoothSetLocalServiceInfo(hradioin.unwrap_or(core::mem::zeroed()) as _, pclassguid, ulinstance, pserviceinfoin) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn BluetoothSetServiceState(hradio: Option<super::winnt::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pguidservice: *const windows_core::GUID, dwserviceflags: u32) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSetServiceState(hradio : super::winnt::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pguidservice : *const windows_core::GUID, dwserviceflags : u32) -> u32);
    unsafe { BluetoothSetServiceState(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pguidservice, dwserviceflags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn BluetoothUnregisterAuthentication(hreghandle: HBLUETOOTH_AUTHENTICATION_REGISTRATION) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothUnregisterAuthentication(hreghandle : HBLUETOOTH_AUTHENTICATION_REGISTRATION) -> windows_core::BOOL);
    unsafe { BluetoothUnregisterAuthentication(hreghandle) }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[inline]
pub unsafe fn BluetoothUpdateDeviceRecord(pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT) -> u32 {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothUpdateDeviceRecord(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT) -> u32);
    unsafe { BluetoothUpdateDeviceRecord(pbtdi) }
}
#[repr(C)]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_ADDRESS_STRUCT {
    pub Anonymous: BLUETOOTH_ADDRESS_STRUCT_0,
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BLUETOOTH_ADDRESS_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_ADDRESS_STRUCT_0 {
    pub ullLong: super::bthdef::BTH_ADDR,
    pub rgBytes: [u8; 6],
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BLUETOOTH_ADDRESS_STRUCT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATE_RESPONSE {
    pub bthAddressRemote: BLUETOOTH_ADDRESS_STRUCT,
    pub authMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub Anonymous: BLUETOOTH_AUTHENTICATE_RESPONSE_0,
    pub negativeResponse: u8,
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    pub pinInfo: BLUETOOTH_PIN_INFO,
    pub oobInfo: BLUETOOTH_OOB_DATA_INFO,
    pub numericCompInfo: BLUETOOTH_NUMERIC_COMPARISON_INFO,
    pub passkeyInfo: BLUETOOTH_PASSKEY_INFO,
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    pub deviceInfo: BLUETOOTH_DEVICE_INFO_STRUCT,
    pub authenticationMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub ioCapability: BLUETOOTH_IO_CAPABILITY,
    pub authenticationRequirements: BLUETOOTH_AUTHENTICATION_REQUIREMENTS,
    pub Anonymous: BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0,
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl Default for BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0 {
    pub Numeric_Value: u32,
    pub Passkey: u32,
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLUETOOTH_COD_PAIRS {
    pub ulCODMask: u32,
    pub pcszDescription: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_DEVICE_INFO_STRUCT {
    pub dwSize: u32,
    pub Address: BLUETOOTH_ADDRESS_STRUCT,
    pub ulClassofDevice: u32,
    pub fConnected: windows_core::BOOL,
    pub fRemembered: windows_core::BOOL,
    pub fAuthenticated: windows_core::BOOL,
    pub stLastSeen: super::minwinbase::SYSTEMTIME,
    pub stLastUsed: super::minwinbase::SYSTEMTIME,
    pub szName: [u16; 248],
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl Default for BLUETOOTH_DEVICE_INFO_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_DEVICE_NAME_SIZE: u32 = 256;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
    pub dwSize: u32,
    pub fReturnAuthenticated: windows_core::BOOL,
    pub fReturnRemembered: windows_core::BOOL,
    pub fReturnUnknown: windows_core::BOOL,
    pub fReturnConnected: windows_core::BOOL,
    pub fIssueInquiry: windows_core::BOOL,
    pub cTimeoutMultiplier: u8,
    pub hRadio: super::winnt::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT {
    pub Enabled: windows_core::BOOL,
    pub btAddr: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [u16; 256],
    pub szDeviceString: [u16; 256],
}
#[cfg(feature = "Win32_bthdef")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLUETOOTH_NUMERIC_COMPARISON_INFO {
    pub NumericValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLUETOOTH_PASSKEY_INFO {
    pub passkey: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_RADIO_INFO {
    pub dwSize: u32,
    pub address: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [u16; 248],
    pub ulClassofDevice: u32,
    pub lmpSubversion: u16,
    pub manufacturer: u16,
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BLUETOOTH_RADIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug)]
pub struct BLUETOOTH_SELECT_DEVICE_PARAMS {
    pub dwSize: u32,
    pub cNumOfClasses: u32,
    pub prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
    pub pszInfo: windows_core::PWSTR,
    pub hwndParent: super::windef::HWND,
    pub fForceAuthentication: windows_core::BOOL,
    pub fShowAuthenticated: windows_core::BOOL,
    pub fShowRemembered: windows_core::BOOL,
    pub fShowUnknown: windows_core::BOOL,
    pub fAddNewDeviceWizard: windows_core::BOOL,
    pub fSkipServicesPage: windows_core::BOOL,
    pub pfnDeviceCallback: PFN_DEVICE_CALLBACK,
    pub pvParam: *mut core::ffi::c_void,
    pub cNumDevices: u32,
    pub pDevices: PBLUETOOTH_DEVICE_INFO,
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase", feature = "Win32_windef"))]
impl Default for BLUETOOTH_SELECT_DEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_SERVICE_DISABLE: u32 = 0;
pub const BLUETOOTH_SERVICE_ENABLE: u32 = 1;
pub const BLUETOOTH_SERVICE_MASK: u32 = 1;
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBLUETOOTH_AUTHENTICATION_REGISTRATION(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBLUETOOTH_CONTAINER_ELEMENT(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBLUETOOTH_DEVICE_FIND(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBLUETOOTH_RADIO_FIND(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_bthdef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_AUTHENTICATE_RESPONSE(pub *mut BLUETOOTH_AUTHENTICATE_RESPONSE);
#[cfg(feature = "Win32_bthdef")]
impl PBLUETOOTH_AUTHENTICATE_RESPONSE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_bthdef")]
impl Default for PBLUETOOTH_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS(pub *mut BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS);
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl Default for PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_AUTHENTICATION_METHOD(pub *mut BLUETOOTH_AUTHENTICATION_METHOD);
impl PBLUETOOTH_AUTHENTICATION_METHOD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLUETOOTH_AUTHENTICATION_METHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_DEVICE_INFO(pub *mut BLUETOOTH_DEVICE_INFO_STRUCT);
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl PBLUETOOTH_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
impl Default for PBLUETOOTH_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_bthdef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_LOCAL_SERVICE_INFO(pub *mut BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT);
#[cfg(feature = "Win32_bthdef")]
impl PBLUETOOTH_LOCAL_SERVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_bthdef")]
impl Default for PBLUETOOTH_LOCAL_SERVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_NUMERIC_COMPARISON_INFO(pub *mut BLUETOOTH_NUMERIC_COMPARISON_INFO);
impl PBLUETOOTH_NUMERIC_COMPARISON_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLUETOOTH_NUMERIC_COMPARISON_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_OOB_DATA_INFO(pub *mut BLUETOOTH_OOB_DATA_INFO);
impl PBLUETOOTH_OOB_DATA_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLUETOOTH_OOB_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_PASSKEY_INFO(pub *mut BLUETOOTH_PASSKEY_INFO);
impl PBLUETOOTH_PASSKEY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLUETOOTH_PASSKEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_PIN_INFO(pub *mut BLUETOOTH_PIN_INFO);
impl PBLUETOOTH_PIN_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLUETOOTH_PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_bthdef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLUETOOTH_RADIO_INFO(pub *mut BLUETOOTH_RADIO_INFO);
#[cfg(feature = "Win32_bthdef")]
impl PBLUETOOTH_RADIO_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_bthdef")]
impl Default for PBLUETOOTH_RADIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
pub type PFN_AUTHENTICATION_CALLBACK = Option<unsafe extern "system" fn(pvparam: *mut core::ffi::c_void, pdevice: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
pub type PFN_AUTHENTICATION_CALLBACK_EX = Option<unsafe extern "system" fn(pvparam: *const core::ffi::c_void, pauthcallbackparams: *const BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS) -> windows_core::BOOL>;
pub type PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK = Option<unsafe extern "system" fn(uattribid: u32, pvaluestream: *const u8, cbstreamsize: u32, pvparam: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_bthdef", feature = "Win32_minwinbase"))]
pub type PFN_DEVICE_CALLBACK = Option<unsafe extern "system" fn(pvparam: *mut core::ffi::c_void, pdevice: *const BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSDP_ELEMENT_DATA(pub *mut SDP_ELEMENT_DATA);
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
impl PSDP_ELEMENT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
impl Default for PSDP_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSDP_STRING_TYPE_DATA(pub *mut SDP_STRING_TYPE_DATA);
impl PSDP_STRING_TYPE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSDP_STRING_TYPE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA {
    pub r#type: super::bthsdpdef::SDP_TYPE,
    pub specificType: super::bthsdpdef::SDP_SPECIFICTYPE,
    pub data: SDP_ELEMENT_DATA_0,
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
impl Default for SDP_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
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
    pub uuid128: windows_core::GUID,
    pub uuid32: u32,
    pub uuid16: u16,
    pub string: SDP_ELEMENT_DATA_0_0,
    pub url: SDP_ELEMENT_DATA_0_1,
    pub sequence: SDP_ELEMENT_DATA_0_2,
    pub alternative: SDP_ELEMENT_DATA_0_3,
}
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
impl Default for SDP_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_0 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_1 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_2 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_bthsdpdef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_3 {
    pub value: super::minwindef::LPBYTE,
    pub length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_STRING_TYPE_DATA {
    pub encoding: u16,
    pub mibeNum: u16,
    pub attributeId: u16,
}
