#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateDevice(hwndparent: super::super::Foundation::HWND, hradio: super::super::Foundation::HANDLE, pbtbi: *mut BLUETOOTH_DEVICE_INFO, pszpasskey: super::super::Foundation::PWSTR, ulpasskeylength: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateDeviceEx(hwndparentin: super::super::Foundation::HWND, hradioin: super::super::Foundation::HANDLE, pbtdiinout: *mut BLUETOOTH_DEVICE_INFO, pbtoobdata: *const BLUETOOTH_OOB_DATA_INFO, authenticationrequirement: AUTHENTICATION_REQUIREMENTS) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateMultipleDevices(hwndparent: super::super::Foundation::HWND, hradio: super::super::Foundation::HANDLE, cdevices: u32, rgbtdi: *mut BLUETOOTH_DEVICE_INFO) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothDisplayDeviceProperties(hwndparent: super::super::Foundation::HWND, pbtdi: *mut BLUETOOTH_DEVICE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnableDiscovery(hradio: super::super::Foundation::HANDLE, fenabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnableIncomingConnections(hradio: super::super::Foundation::HANDLE, fenabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnumerateInstalledServices(hradio: super::super::Foundation::HANDLE, pbtdi: *const BLUETOOTH_DEVICE_INFO, pcserviceinout: *mut u32, pguidservices: *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindDeviceClose(hfind: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindFirstDevice(pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS, pbtdi: *mut BLUETOOTH_DEVICE_INFO) -> isize;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindFirstRadio(pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS, phradio: *mut super::super::Foundation::HANDLE) -> isize;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindNextDevice(hfind: isize, pbtdi: *mut BLUETOOTH_DEVICE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindNextRadio(hfind: isize, phradio: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindRadioClose(hfind: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothGetDeviceInfo(hradio: super::super::Foundation::HANDLE, pbtdi: *mut BLUETOOTH_DEVICE_INFO) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothGetRadioInfo(hradio: super::super::Foundation::HANDLE, pradioinfo: *mut BLUETOOTH_RADIO_INFO) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsConnectable(hradio: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsDiscoverable(hradio: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsVersionAvailable(majorversion: u8, minorversion: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothRegisterForAuthentication(pbtdi: *const BLUETOOTH_DEVICE_INFO, phreghandle: *mut isize, pfncallback: ::windows::runtime::RawPtr, pvparam: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothRegisterForAuthenticationEx(pbtdiin: *const BLUETOOTH_DEVICE_INFO, phreghandleout: *mut isize, pfncallbackin: ::windows::runtime::RawPtr, pvparam: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`*"]
    pub fn BluetoothRemoveDevice(paddress: *const BLUETOOTH_ADDRESS) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpEnumAttributes(psdpstream: *const u8, cbstreamsize: u32, pfncallback: ::windows::runtime::RawPtr, pvparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetAttributeValue(precordstream: *const u8, cbrecordlength: u32, usattributeid: u16, pattributedata: *mut SDP_ELEMENT_DATA) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetContainerElementData(pcontainerstream: *const u8, cbcontainerlength: u32, pelement: *mut isize, pdata: *mut SDP_ELEMENT_DATA) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetElementData(psdpstream: *const u8, cbsdpstreamlength: u32, pdata: *mut SDP_ELEMENT_DATA) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetString(precordstream: *const u8, cbrecordlength: u32, pstringdata: *const SDP_STRING_TYPE_DATA, usstringoffset: u16, pszstring: super::super::Foundation::PWSTR, pcchstringlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSelectDevices(pbtsdp: *mut ::core::mem::ManuallyDrop<BLUETOOTH_SELECT_DEVICE_PARAMS>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSelectDevicesFree(pbtsdp: *mut ::core::mem::ManuallyDrop<BLUETOOTH_SELECT_DEVICE_PARAMS>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSendAuthenticationResponse(hradio: super::super::Foundation::HANDLE, pbtdi: *const BLUETOOTH_DEVICE_INFO, pszpasskey: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSendAuthenticationResponseEx(hradioin: super::super::Foundation::HANDLE, pauthresponse: *const BLUETOOTH_AUTHENTICATE_RESPONSE) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSetLocalServiceInfo(hradioin: super::super::Foundation::HANDLE, pclassguid: *const ::windows::runtime::GUID, ulinstance: u32, pserviceinfoin: *const BLUETOOTH_LOCAL_SERVICE_INFO) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSetServiceState(hradio: super::super::Foundation::HANDLE, pbtdi: *const BLUETOOTH_DEVICE_INFO, pguidservice: *const ::windows::runtime::GUID, dwserviceflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothUnregisterAuthentication(hreghandle: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothUpdateDeviceRecord(pbtdi: *const BLUETOOTH_DEVICE_INFO) -> u32;
}
