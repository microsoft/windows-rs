#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateDevice(hwndparent: Option<super::HWND>, hradio: Option<super::HANDLE>, pbtbi: *mut BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey: Option<&[u16]>) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDevice(hwndparent : super::HWND, hradio : super::HANDLE, pbtbi : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_core::PCWSTR, ulpasskeylength : super::ULONG) -> super::DWORD);
    unsafe { BluetoothAuthenticateDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, hradio.unwrap_or(core::mem::zeroed()) as _, pbtbi as _, core::mem::transmute(pszpasskey.map_or(core::ptr::null(), |slice| slice.as_ptr())), pszpasskey.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[must_use]
#[cfg(all(feature = "bthdef", feature = "minwinbase", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateDeviceEx(hwndparentin: Option<super::HWND>, hradioin: Option<super::HANDLE>, pbtdiinout: *mut BLUETOOTH_DEVICE_INFO_STRUCT, pbtoobdata: Option<PBLUETOOTH_OOB_DATA_INFO>, authenticationrequirement: super::AUTHENTICATION_REQUIREMENTS) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateDeviceEx(hwndparentin : super::HWND, hradioin : super::HANDLE, pbtdiinout : *mut BLUETOOTH_DEVICE_INFO_STRUCT, pbtoobdata : PBLUETOOTH_OOB_DATA_INFO, authenticationrequirement : super::AUTHENTICATION_REQUIREMENTS) -> super::DWORD);
    unsafe { BluetoothAuthenticateDeviceEx(hwndparentin.unwrap_or(core::mem::zeroed()) as _, hradioin.unwrap_or(core::mem::zeroed()) as _, pbtdiinout as _, pbtoobdata.unwrap_or(core::mem::zeroed()) as _, authenticationrequirement) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothAuthenticateMultipleDevices(hwndparent: Option<super::HWND>, hradio: Option<super::HANDLE>, rgbtdi: &mut [BLUETOOTH_DEVICE_INFO_STRUCT]) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothAuthenticateMultipleDevices(hwndparent : super::HWND, hradio : super::HANDLE, cdevices : super::DWORD, rgbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> super::DWORD);
    unsafe { BluetoothAuthenticateMultipleDevices(hwndparent.unwrap_or(core::mem::zeroed()) as _, hradio.unwrap_or(core::mem::zeroed()) as _, rgbtdi.len().try_into().unwrap(), rgbtdi.as_mut_ptr()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothDisplayDeviceProperties(hwndparent: Option<super::HWND>, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothDisplayDeviceProperties(hwndparent : super::HWND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL);
    unsafe { BluetoothDisplayDeviceProperties(hwndparent.unwrap_or(core::mem::zeroed()) as _, pbtdi as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BluetoothEnableDiscovery(hradio: Option<super::HANDLE>, fenabled: bool) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnableDiscovery(hradio : super::HANDLE, fenabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { BluetoothEnableDiscovery(hradio.unwrap_or(core::mem::zeroed()) as _, fenabled.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BluetoothEnableIncomingConnections(hradio: Option<super::HANDLE>, fenabled: bool) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnableIncomingConnections(hradio : super::HANDLE, fenabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { BluetoothEnableIncomingConnections(hradio.unwrap_or(core::mem::zeroed()) as _, fenabled.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothEnumAttributes(psdpstream: &[u8], pfncallback: PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" "BluetoothSdpEnumAttributes" fn BluetoothEnumAttributes(psdpstream : super::LPBYTE, cbstreamsize : super::ULONG, pfncallback : PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam : super::LPVOID) -> windows_core::BOOL);
    unsafe { BluetoothEnumAttributes(core::mem::transmute(psdpstream.as_ptr()), psdpstream.len().try_into().unwrap(), pfncallback, pvparam) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothEnumerateInstalledServices(hradio: Option<super::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pcserviceinout: *mut super::DWORD, pguidservices: Option<*mut windows_core::GUID>) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothEnumerateInstalledServices(hradio : super::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pcserviceinout : *mut super::DWORD, pguidservices : *mut windows_core::GUID) -> super::DWORD);
    unsafe { BluetoothEnumerateInstalledServices(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pcserviceinout as _, pguidservices.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BluetoothFindDeviceClose(hfind: HBLUETOOTH_DEVICE_FIND) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindDeviceClose(hfind : HBLUETOOTH_DEVICE_FIND) -> windows_core::BOOL);
    unsafe { BluetoothFindDeviceClose(hfind) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothFindFirstDevice(pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> HBLUETOOTH_DEVICE_FIND {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindFirstDevice(pbtsp : *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> HBLUETOOTH_DEVICE_FIND);
    unsafe { BluetoothFindFirstDevice(pbtsp, pbtdi as _) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothFindFirstRadio(pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS, phradio: *mut super::HANDLE) -> HBLUETOOTH_RADIO_FIND {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindFirstRadio(pbtfrp : *const BLUETOOTH_FIND_RADIO_PARAMS, phradio : *mut super::HANDLE) -> HBLUETOOTH_RADIO_FIND);
    unsafe { BluetoothFindFirstRadio(pbtfrp, phradio as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothFindNextDevice(hfind: HBLUETOOTH_DEVICE_FIND, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindNextDevice(hfind : HBLUETOOTH_DEVICE_FIND, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL);
    unsafe { BluetoothFindNextDevice(hfind, pbtdi as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BluetoothFindNextRadio(hfind: HBLUETOOTH_RADIO_FIND, phradio: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindNextRadio(hfind : HBLUETOOTH_RADIO_FIND, phradio : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothFindNextRadio(hfind, phradio as _) }
}
#[inline]
pub unsafe fn BluetoothFindRadioClose(hfind: HBLUETOOTH_RADIO_FIND) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothFindRadioClose(hfind : HBLUETOOTH_RADIO_FIND) -> windows_core::BOOL);
    unsafe { BluetoothFindRadioClose(hfind) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothGetDeviceInfo(hradio: Option<super::HANDLE>, pbtdi: *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothGetDeviceInfo(hradio : super::HANDLE, pbtdi : *mut BLUETOOTH_DEVICE_INFO_STRUCT) -> super::DWORD);
    unsafe { BluetoothGetDeviceInfo(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi as _) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothGetRadioInfo(hradio: super::HANDLE, pradioinfo: PBLUETOOTH_RADIO_INFO) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothGetRadioInfo(hradio : super::HANDLE, pradioinfo : PBLUETOOTH_RADIO_INFO) -> super::DWORD);
    unsafe { BluetoothGetRadioInfo(hradio, pradioinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BluetoothIsConnectable(hradio: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsConnectable(hradio : super::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothIsConnectable(hradio.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BluetoothIsDiscoverable(hradio: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsDiscoverable(hradio : super::HANDLE) -> windows_core::BOOL);
    unsafe { BluetoothIsDiscoverable(hradio.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BluetoothIsVersionAvailable(majorversion: super::UCHAR, minorversion: super::UCHAR) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothIsVersionAvailable(majorversion : super::UCHAR, minorversion : super::UCHAR) -> windows_core::BOOL);
    unsafe { BluetoothIsVersionAvailable(majorversion, minorversion) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothRegisterForAuthentication(pbtdi: Option<*const BLUETOOTH_DEVICE_INFO_STRUCT>, phreghandle: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallback: PFN_AUTHENTICATION_CALLBACK, pvparam: Option<super::PVOID>) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthentication(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandle : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallback : PFN_AUTHENTICATION_CALLBACK, pvparam : super::PVOID) -> super::DWORD);
    unsafe { BluetoothRegisterForAuthentication(pbtdi.unwrap_or(core::mem::zeroed()) as _, phreghandle as _, pfncallback, pvparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothRegisterForAuthenticationEx(pbtdiin: Option<*const BLUETOOTH_DEVICE_INFO_STRUCT>, phreghandleout: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallbackin: PFN_AUTHENTICATION_CALLBACK_EX, pvparam: Option<super::PVOID>) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRegisterForAuthenticationEx(pbtdiin : *const BLUETOOTH_DEVICE_INFO_STRUCT, phreghandleout : *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION, pfncallbackin : PFN_AUTHENTICATION_CALLBACK_EX, pvparam : super::PVOID) -> super::DWORD);
    unsafe { BluetoothRegisterForAuthenticationEx(pbtdiin.unwrap_or(core::mem::zeroed()) as _, phreghandleout as _, pfncallbackin, pvparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BluetoothRemoveDevice(paddress: *const BLUETOOTH_ADDRESS_STRUCT) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothRemoveDevice(paddress : *const BLUETOOTH_ADDRESS_STRUCT) -> super::DWORD);
    unsafe { BluetoothRemoveDevice(paddress) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothSdpEnumAttributes(psdpstream: &[u8], pfncallback: PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpEnumAttributes(psdpstream : super::LPBYTE, cbstreamsize : super::ULONG, pfncallback : PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK, pvparam : super::LPVOID) -> windows_core::BOOL);
    unsafe { BluetoothSdpEnumAttributes(core::mem::transmute(psdpstream.as_ptr()), psdpstream.len().try_into().unwrap(), pfncallback, pvparam) }
}
#[must_use]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothSdpGetAttributeValue(precordstream: &[u8], usattributeid: super::USHORT, pattributedata: PSDP_ELEMENT_DATA) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetAttributeValue(precordstream : super::LPBYTE, cbrecordlength : super::ULONG, usattributeid : super::USHORT, pattributedata : PSDP_ELEMENT_DATA) -> super::DWORD);
    unsafe { BluetoothSdpGetAttributeValue(core::mem::transmute(precordstream.as_ptr()), precordstream.len().try_into().unwrap(), usattributeid, pattributedata as _) }
}
#[must_use]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothSdpGetContainerElementData(pcontainerstream: &[u8], pelement: *mut HBLUETOOTH_CONTAINER_ELEMENT, pdata: PSDP_ELEMENT_DATA) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetContainerElementData(pcontainerstream : super::LPBYTE, cbcontainerlength : super::ULONG, pelement : *mut HBLUETOOTH_CONTAINER_ELEMENT, pdata : PSDP_ELEMENT_DATA) -> super::DWORD);
    unsafe { BluetoothSdpGetContainerElementData(core::mem::transmute(pcontainerstream.as_ptr()), pcontainerstream.len().try_into().unwrap(), pelement as _, pdata as _) }
}
#[must_use]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothSdpGetElementData(psdpstream: &[u8], pdata: PSDP_ELEMENT_DATA) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetElementData(psdpstream : super::LPBYTE, cbsdpstreamlength : super::ULONG, pdata : PSDP_ELEMENT_DATA) -> super::DWORD);
    unsafe { BluetoothSdpGetElementData(core::mem::transmute(psdpstream.as_ptr()), psdpstream.len().try_into().unwrap(), pdata as _) }
}
#[must_use]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BluetoothSdpGetString(precordstream: &[u8], pstringdata: Option<PSDP_STRING_TYPE_DATA>, usstringoffset: super::USHORT, pszstring: windows_core::PWSTR, pcchstringlength: super::PULONG) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSdpGetString(precordstream : super::LPBYTE, cbrecordlength : super::ULONG, pstringdata : PSDP_STRING_TYPE_DATA, usstringoffset : super::USHORT, pszstring : windows_core::PWSTR, pcchstringlength : super::PULONG) -> super::DWORD);
    unsafe { BluetoothSdpGetString(core::mem::transmute(precordstream.as_ptr()), precordstream.len().try_into().unwrap(), pstringdata.unwrap_or(core::mem::zeroed()) as _, usstringoffset, pszstring, pcchstringlength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSelectDevices(pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSelectDevices(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL);
    unsafe { BluetoothSelectDevices(pbtsdp as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSelectDevicesFree(pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSelectDevicesFree(pbtsdp : *mut BLUETOOTH_SELECT_DEVICE_PARAMS) -> windows_core::BOOL);
    unsafe { BluetoothSelectDevicesFree(pbtsdp as _) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSendAuthenticationResponse<P2>(hradio: Option<super::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponse(hradio : super::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pszpasskey : windows_core::PCWSTR) -> super::DWORD);
    unsafe { BluetoothSendAuthenticationResponse(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pszpasskey.param().abi()) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSendAuthenticationResponseEx(hradioin: Option<super::HANDLE>, pauthresponse: PBLUETOOTH_AUTHENTICATE_RESPONSE) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSendAuthenticationResponseEx(hradioin : super::HANDLE, pauthresponse : PBLUETOOTH_AUTHENTICATE_RESPONSE) -> super::DWORD);
    unsafe { BluetoothSendAuthenticationResponseEx(hradioin.unwrap_or(core::mem::zeroed()) as _, pauthresponse) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSetLocalServiceInfo(hradioin: Option<super::HANDLE>, pclassguid: *const windows_core::GUID, ulinstance: super::ULONG, pserviceinfoin: *const BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSetLocalServiceInfo(hradioin : super::HANDLE, pclassguid : *const windows_core::GUID, ulinstance : super::ULONG, pserviceinfoin : *const BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT) -> super::DWORD);
    unsafe { BluetoothSetLocalServiceInfo(hradioin.unwrap_or(core::mem::zeroed()) as _, pclassguid, ulinstance, pserviceinfoin) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothSetServiceState(hradio: Option<super::HANDLE>, pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT, pguidservice: *const windows_core::GUID, dwserviceflags: super::DWORD) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothSetServiceState(hradio : super::HANDLE, pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT, pguidservice : *const windows_core::GUID, dwserviceflags : super::DWORD) -> super::DWORD);
    unsafe { BluetoothSetServiceState(hradio.unwrap_or(core::mem::zeroed()) as _, pbtdi, pguidservice, dwserviceflags) }
}
#[inline]
pub unsafe fn BluetoothUnregisterAuthentication(hreghandle: HBLUETOOTH_AUTHENTICATION_REGISTRATION) -> windows_core::BOOL {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothUnregisterAuthentication(hreghandle : HBLUETOOTH_AUTHENTICATION_REGISTRATION) -> windows_core::BOOL);
    unsafe { BluetoothUnregisterAuthentication(hreghandle) }
}
#[must_use]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BluetoothUpdateDeviceRecord(pbtdi: *const BLUETOOTH_DEVICE_INFO_STRUCT) -> super::DWORD {
    windows_core::link!("bthprops.cpl" "system" fn BluetoothUpdateDeviceRecord(pbtdi : *const BLUETOOTH_DEVICE_INFO_STRUCT) -> super::DWORD);
    unsafe { BluetoothUpdateDeviceRecord(pbtdi) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_ADDRESS_STRUCT {
    pub Anonymous: BLUETOOTH_ADDRESS_STRUCT_0,
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_ADDRESS_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_ADDRESS_STRUCT_0 {
    pub ullLong: super::BTH_ADDR,
    pub rgBytes: [super::BYTE; 6],
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_ADDRESS_STRUCT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATE_RESPONSE {
    pub bthAddressRemote: BLUETOOTH_ADDRESS_STRUCT,
    pub authMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub Anonymous: BLUETOOTH_AUTHENTICATE_RESPONSE_0,
    pub negativeResponse: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    pub pinInfo: BLUETOOTH_PIN_INFO,
    pub oobInfo: BLUETOOTH_OOB_DATA_INFO,
    pub numericCompInfo: BLUETOOTH_NUMERIC_COMPARISON_INFO,
    pub passkeyInfo: BLUETOOTH_PASSKEY_INFO,
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_AUTHENTICATE_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    pub deviceInfo: BLUETOOTH_DEVICE_INFO_STRUCT,
    pub authenticationMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    pub ioCapability: BLUETOOTH_IO_CAPABILITY,
    pub authenticationRequirements: BLUETOOTH_AUTHENTICATION_REQUIREMENTS,
    pub Anonymous: BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_0 {
    pub Numeric_Value: super::ULONG,
    pub Passkey: super::ULONG,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLUETOOTH_COD_PAIRS {
    pub ulCODMask: super::ULONG,
    pub pcszDescription: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_DEVICE_INFO_STRUCT {
    pub dwSize: super::DWORD,
    pub Address: BLUETOOTH_ADDRESS_STRUCT,
    pub ulClassofDevice: super::ULONG,
    pub fConnected: windows_core::BOOL,
    pub fRemembered: windows_core::BOOL,
    pub fAuthenticated: windows_core::BOOL,
    pub stLastSeen: super::SYSTEMTIME,
    pub stLastUsed: super::SYSTEMTIME,
    pub szName: [super::WCHAR; 248],
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for BLUETOOTH_DEVICE_INFO_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_DEVICE_NAME_SIZE: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
    pub dwSize: super::DWORD,
    pub fReturnAuthenticated: windows_core::BOOL,
    pub fReturnRemembered: windows_core::BOOL,
    pub fReturnUnknown: windows_core::BOOL,
    pub fReturnConnected: windows_core::BOOL,
    pub fIssueInquiry: windows_core::BOOL,
    pub cTimeoutMultiplier: super::UCHAR,
    pub hRadio: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLUETOOTH_FIND_RADIO_PARAMS {
    pub dwSize: super::DWORD,
}
pub type BLUETOOTH_IO_CAPABILITY = i32;
pub const BLUETOOTH_IO_CAPABILITY_DISPLAYONLY: BLUETOOTH_IO_CAPABILITY = 0;
pub const BLUETOOTH_IO_CAPABILITY_DISPLAYYESNO: BLUETOOTH_IO_CAPABILITY = 1;
pub const BLUETOOTH_IO_CAPABILITY_KEYBOARDONLY: BLUETOOTH_IO_CAPABILITY = 2;
pub const BLUETOOTH_IO_CAPABILITY_NOINPUTNOOUTPUT: BLUETOOTH_IO_CAPABILITY = 3;
pub const BLUETOOTH_IO_CAPABILITY_UNDEFINED: BLUETOOTH_IO_CAPABILITY = 255;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT {
    pub Enabled: windows_core::BOOL,
    pub btAddr: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [super::WCHAR; 256],
    pub szDeviceString: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BLUETOOTH_MAX_NAME_SIZE: i32 = 248;
pub const BLUETOOTH_MAX_PASSKEY_BUFFER_SIZE: i32 = 17;
pub const BLUETOOTH_MAX_PASSKEY_SIZE: i32 = 16;
pub const BLUETOOTH_MAX_SERVICE_NAME_SIZE: i32 = 256;
pub const BLUETOOTH_MITM_ProtectionNotDefined: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 255;
pub const BLUETOOTH_MITM_ProtectionNotRequired: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 0;
pub const BLUETOOTH_MITM_ProtectionNotRequiredBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 2;
pub const BLUETOOTH_MITM_ProtectionNotRequiredGeneralBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 4;
pub const BLUETOOTH_MITM_ProtectionRequired: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 1;
pub const BLUETOOTH_MITM_ProtectionRequiredBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 3;
pub const BLUETOOTH_MITM_ProtectionRequiredGeneralBonding: BLUETOOTH_AUTHENTICATION_REQUIREMENTS = 5;
#[cfg(feature = "shared")]
pub const BLUETOOTH_NULL_ADDRESS: super::ULONGLONG = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLUETOOTH_NUMERIC_COMPARISON_INFO {
    pub NumericValue: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BLUETOOTH_OOB_DATA_INFO {
    pub C: [super::UCHAR; 16],
    pub R: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_OOB_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLUETOOTH_PASSKEY_INFO {
    pub passkey: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BLUETOOTH_PIN_INFO {
    pub pin: [super::UCHAR; 16],
    pub pinLength: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for BLUETOOTH_PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BLUETOOTH_RADIO_INFO {
    pub dwSize: super::DWORD,
    pub address: BLUETOOTH_ADDRESS_STRUCT,
    pub szName: [super::WCHAR; 248],
    pub ulClassofDevice: super::ULONG,
    pub lmpSubversion: super::USHORT,
    pub manufacturer: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BLUETOOTH_RADIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct BLUETOOTH_SELECT_DEVICE_PARAMS {
    pub dwSize: super::DWORD,
    pub cNumOfClasses: super::ULONG,
    pub prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
    pub pszInfo: windows_core::PWSTR,
    pub hwndParent: super::HWND,
    pub fForceAuthentication: windows_core::BOOL,
    pub fShowAuthenticated: windows_core::BOOL,
    pub fShowRemembered: windows_core::BOOL,
    pub fShowUnknown: windows_core::BOOL,
    pub fAddNewDeviceWizard: windows_core::BOOL,
    pub fSkipServicesPage: windows_core::BOOL,
    pub pfnDeviceCallback: PFN_DEVICE_CALLBACK,
    pub pvParam: super::LPVOID,
    pub cNumDevices: super::DWORD,
    pub pDevices: PBLUETOOTH_DEVICE_INFO,
}
pub const BLUETOOTH_SERVICE_DISABLE: i32 = 0;
pub const BLUETOOTH_SERVICE_ENABLE: i32 = 1;
pub const BLUETOOTH_SERVICE_MASK: i32 = 1;
pub type HBLUETOOTH_AUTHENTICATION_REGISTRATION = *mut core::ffi::c_void;
pub type HBLUETOOTH_CONTAINER_ELEMENT = *mut core::ffi::c_void;
pub type HBLUETOOTH_DEVICE_FIND = *mut core::ffi::c_void;
pub type HBLUETOOTH_RADIO_FIND = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PBLUETOOTH_AUTHENTICATE_RESPONSE = *mut BLUETOOTH_AUTHENTICATE_RESPONSE;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS = *mut BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS;
pub type PBLUETOOTH_AUTHENTICATION_METHOD = *mut BLUETOOTH_AUTHENTICATION_METHOD;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PBLUETOOTH_DEVICE_INFO = *mut BLUETOOTH_DEVICE_INFO_STRUCT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBLUETOOTH_LOCAL_SERVICE_INFO = *mut BLUETOOTH_LOCAL_SERVICE_INFO_STRUCT;
#[cfg(feature = "shared")]
pub type PBLUETOOTH_NUMERIC_COMPARISON_INFO = *mut BLUETOOTH_NUMERIC_COMPARISON_INFO;
#[cfg(feature = "shared")]
pub type PBLUETOOTH_OOB_DATA_INFO = *mut BLUETOOTH_OOB_DATA_INFO;
#[cfg(feature = "shared")]
pub type PBLUETOOTH_PASSKEY_INFO = *mut BLUETOOTH_PASSKEY_INFO;
#[cfg(feature = "shared")]
pub type PBLUETOOTH_PIN_INFO = *mut BLUETOOTH_PIN_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBLUETOOTH_RADIO_INFO = *mut BLUETOOTH_RADIO_INFO;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_AUTHENTICATION_CALLBACK = Option<unsafe extern "system" fn(pvparam: super::LPVOID, pdevice: PBLUETOOTH_DEVICE_INFO) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_AUTHENTICATION_CALLBACK_EX = Option<unsafe extern "system" fn(pvparam: super::LPVOID, pauthcallbackparams: PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK = Option<unsafe extern "system" fn(uattribid: super::ULONG, pvaluestream: super::LPBYTE, cbstreamsize: super::ULONG, pvparam: super::LPVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_DEVICE_CALLBACK = Option<unsafe extern "system" fn(pvparam: super::LPVOID, pdevice: *const BLUETOOTH_DEVICE_INFO_STRUCT) -> windows_core::BOOL>;
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
pub type PSDP_ELEMENT_DATA = *mut SDP_ELEMENT_DATA;
#[cfg(feature = "shared")]
pub type PSDP_STRING_TYPE_DATA = *mut SDP_STRING_TYPE_DATA;
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SDP_ELEMENT_DATA {
    pub r#type: super::SDP_TYPE,
    pub specificType: super::SDP_SPECIFICTYPE,
    pub data: SDP_ELEMENT_DATA_0,
}
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
impl Default for SDP_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union SDP_ELEMENT_DATA_0 {
    pub int128: super::SDP_LARGE_INTEGER_16,
    pub int64: super::LONGLONG,
    pub int32: super::LONG,
    pub int16: super::SHORT,
    pub int8: super::CHAR,
    pub uint128: super::SDP_ULARGE_INTEGER_16,
    pub uint64: super::ULONGLONG,
    pub uint32: super::ULONG,
    pub uint16: super::USHORT,
    pub uint8: super::UCHAR,
    pub booleanVal: super::UCHAR,
    pub uuid128: windows_core::GUID,
    pub uuid32: super::ULONG,
    pub uuid16: super::USHORT,
    pub string: SDP_ELEMENT_DATA_0_0,
    pub url: SDP_ELEMENT_DATA_0_1,
    pub sequence: SDP_ELEMENT_DATA_0_2,
    pub alternative: SDP_ELEMENT_DATA_0_3,
}
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
impl Default for SDP_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_0 {
    pub value: super::LPBYTE,
    pub length: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_1 {
    pub value: super::LPBYTE,
    pub length: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_2 {
    pub value: super::LPBYTE,
    pub length: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "bthsdpdef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SDP_ELEMENT_DATA_0_3 {
    pub value: super::LPBYTE,
    pub length: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SDP_STRING_TYPE_DATA {
    pub encoding: super::USHORT,
    pub mibeNum: super::USHORT,
    pub attributeId: super::USHORT,
}
