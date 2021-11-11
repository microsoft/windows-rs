#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateDevice();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateDeviceEx();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothAuthenticateMultipleDevices();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothDisplayDeviceProperties();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnableDiscovery();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnableIncomingConnections();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothEnumerateInstalledServices();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindDeviceClose();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindFirstDevice();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindFirstRadio();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindNextDevice();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindNextRadio();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothFindRadioClose();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothGetDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothGetRadioInfo();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsConnectable();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsDiscoverable();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothIsVersionAvailable();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothRegisterForAuthentication();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothRegisterForAuthenticationEx();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`*"]
    pub fn BluetoothRemoveDevice();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpEnumAttributes();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetAttributeValue();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetContainerElementData();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetElementData();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSdpGetString();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSelectDevices();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSelectDevicesFree();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSendAuthenticationResponse();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSendAuthenticationResponseEx();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSetLocalServiceInfo();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothSetServiceState();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothUnregisterAuthentication();
    #[doc = "*Required features: `Win32_Devices_Bluetooth`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BluetoothUpdateDeviceRecord();
}
