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
#[repr(transparent)]
pub struct BluetoothAdapter(pub *mut ::core::ffi::c_void);
pub struct BluetoothAddressType(i32);
pub struct BluetoothCacheMode(i32);
#[repr(transparent)]
pub struct BluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
pub struct BluetoothConnectionStatus(i32);
#[repr(transparent)]
pub struct BluetoothDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothDeviceId(pub *mut ::core::ffi::c_void);
pub struct BluetoothError(i32);
#[repr(transparent)]
pub struct BluetoothLEAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAppearanceCategories(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAppearanceSubcategories(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(i32);
pub struct BluetoothMajorClass(i32);
pub struct BluetoothMinorClass(i32);
pub struct BluetoothServiceCapabilities(i32);
#[repr(transparent)]
pub struct BluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothUuidHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothClassOfDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceIdStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceCategoriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothUuidHelperStatics(pub *mut ::core::ffi::c_void);
