#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[link(name = "windows")]
extern "system" {
    fn BluetoothAdapter();
    fn BluetoothAddressType();
    fn BluetoothCacheMode();
    fn BluetoothClassOfDevice();
    fn BluetoothConnectionStatus();
    fn BluetoothDevice();
    fn BluetoothDeviceId();
    fn BluetoothError();
    fn BluetoothLEAppearance();
    fn BluetoothLEAppearanceCategories();
    fn BluetoothLEAppearanceSubcategories();
    fn BluetoothLEConnectionParameters();
    fn BluetoothLEConnectionPhy();
    fn BluetoothLEConnectionPhyInfo();
    fn BluetoothLEDevice();
    fn BluetoothLEPreferredConnectionParameters();
    fn BluetoothLEPreferredConnectionParametersRequest();
    fn BluetoothLEPreferredConnectionParametersRequestStatus();
    fn BluetoothMajorClass();
    fn BluetoothMinorClass();
    fn BluetoothServiceCapabilities();
    fn BluetoothSignalStrengthFilter();
    fn BluetoothUuidHelper();
    fn IBluetoothAdapter();
    fn IBluetoothAdapter2();
    fn IBluetoothAdapter3();
    fn IBluetoothAdapterStatics();
    fn IBluetoothClassOfDevice();
    fn IBluetoothClassOfDeviceStatics();
    fn IBluetoothDevice();
    fn IBluetoothDevice2();
    fn IBluetoothDevice3();
    fn IBluetoothDevice4();
    fn IBluetoothDevice5();
    fn IBluetoothDeviceId();
    fn IBluetoothDeviceIdStatics();
    fn IBluetoothDeviceStatics();
    fn IBluetoothDeviceStatics2();
    fn IBluetoothLEAppearance();
    fn IBluetoothLEAppearanceCategoriesStatics();
    fn IBluetoothLEAppearanceStatics();
    fn IBluetoothLEAppearanceSubcategoriesStatics();
    fn IBluetoothLEConnectionParameters();
    fn IBluetoothLEConnectionPhy();
    fn IBluetoothLEConnectionPhyInfo();
    fn IBluetoothLEDevice();
    fn IBluetoothLEDevice2();
    fn IBluetoothLEDevice3();
    fn IBluetoothLEDevice4();
    fn IBluetoothLEDevice5();
    fn IBluetoothLEDevice6();
    fn IBluetoothLEDeviceStatics();
    fn IBluetoothLEDeviceStatics2();
    fn IBluetoothLEPreferredConnectionParameters();
    fn IBluetoothLEPreferredConnectionParametersRequest();
    fn IBluetoothLEPreferredConnectionParametersStatics();
    fn IBluetoothSignalStrengthFilter();
    fn IBluetoothUuidHelperStatics();
}
