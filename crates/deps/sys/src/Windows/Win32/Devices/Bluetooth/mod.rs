#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BluetoothAuthenticateDevice();
    fn BluetoothAuthenticateDeviceEx();
    fn BluetoothAuthenticateMultipleDevices();
    fn BluetoothDisplayDeviceProperties();
    fn BluetoothEnableDiscovery();
    fn BluetoothEnableIncomingConnections();
    fn BluetoothEnumerateInstalledServices();
    fn BluetoothFindDeviceClose();
    fn BluetoothFindFirstDevice();
    fn BluetoothFindFirstRadio();
    fn BluetoothFindNextDevice();
    fn BluetoothFindNextRadio();
    fn BluetoothFindRadioClose();
    fn BluetoothGetDeviceInfo();
    fn BluetoothGetRadioInfo();
    fn BluetoothIsConnectable();
    fn BluetoothIsDiscoverable();
    fn BluetoothIsVersionAvailable();
    fn BluetoothRegisterForAuthentication();
    fn BluetoothRegisterForAuthenticationEx();
    fn BluetoothRemoveDevice();
    fn BluetoothSdpEnumAttributes();
    fn BluetoothSdpGetAttributeValue();
    fn BluetoothSdpGetContainerElementData();
    fn BluetoothSdpGetElementData();
    fn BluetoothSdpGetString();
    fn BluetoothSelectDevices();
    fn BluetoothSelectDevicesFree();
    fn BluetoothSendAuthenticationResponse();
    fn BluetoothSendAuthenticationResponseEx();
    fn BluetoothSetLocalServiceInfo();
    fn BluetoothSetServiceState();
    fn BluetoothUnregisterAuthentication();
    fn BluetoothUpdateDeviceRecord();
}
