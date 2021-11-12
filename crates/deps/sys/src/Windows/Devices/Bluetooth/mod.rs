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
extern "system" {}
pub struct BluetoothAdapter(i32);
pub struct BluetoothAddressType(i32);
pub struct BluetoothCacheMode(i32);
pub struct BluetoothClassOfDevice(i32);
pub struct BluetoothConnectionStatus(i32);
pub struct BluetoothDevice(i32);
pub struct BluetoothDeviceId(i32);
pub struct BluetoothError(i32);
pub struct BluetoothLEAppearance(i32);
pub struct BluetoothLEAppearanceCategories(i32);
pub struct BluetoothLEAppearanceSubcategories(i32);
pub struct BluetoothLEConnectionParameters(i32);
pub struct BluetoothLEConnectionPhy(i32);
pub struct BluetoothLEConnectionPhyInfo(i32);
pub struct BluetoothLEDevice(i32);
pub struct BluetoothLEPreferredConnectionParameters(i32);
pub struct BluetoothLEPreferredConnectionParametersRequest(i32);
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(i32);
pub struct BluetoothMajorClass(i32);
pub struct BluetoothMinorClass(i32);
pub struct BluetoothServiceCapabilities(i32);
pub struct BluetoothSignalStrengthFilter(i32);
pub struct BluetoothUuidHelper(i32);
pub struct IBluetoothAdapter(pub *mut ::core::ffi::c_void);
pub struct IBluetoothAdapter2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothAdapter3(pub *mut ::core::ffi::c_void);
pub struct IBluetoothAdapterStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
pub struct IBluetoothClassOfDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDevice(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDevice2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDevice3(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDevice4(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDevice5(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDeviceId(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDeviceIdStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothDeviceStatics2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAppearance(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAppearanceCategoriesStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAppearanceStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAppearanceSubcategoriesStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice3(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice4(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice5(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDevice6(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEDeviceStatics2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEPreferredConnectionParametersStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
pub struct IBluetoothUuidHelperStatics(pub *mut ::core::ffi::c_void);
