/*++

Copyright (c) 1986-1997  Microsoft Corporation

Module Name:

    stireg.h

Abstract:

    This module contains the STI registry entries

Author:


Revision History:


--*/

#ifndef _STIREG_
#define _STIREG_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Registry keys and values
//
#define REGSTR_VAL_TYPE_W            L"Type"
#define REGSTR_VAL_VENDOR_NAME_W     L"Vendor"
#define REGSTR_VAL_DEVICETYPE_W      L"DeviceType"
#define REGSTR_VAL_DEVICESUBTYPE_W   L"DeviceSubType"
#define REGSTR_VAL_DEV_NAME_W        L"DeviceName"
#define REGSTR_VAL_DRIVER_DESC_W     L"DriverDesc"
#define REGSTR_VAL_FRIENDLY_NAME_W   L"FriendlyName"
#define REGSTR_VAL_GENERIC_CAPS_W    L"Capabilities"
#define REGSTR_VAL_HARDWARE_W        L"HardwareConfig"
#define REGSTR_VAL_HARDWARE          TEXT("HardwareConfig")
#define REGSTR_VAL_DEVICE_NAME_W     L"DriverDesc"
#define REGSTR_VAL_DATA_W            L"DeviceData"
#define REGSTR_VAL_GUID_W            L"GUID"
#define REGSTR_VAL_GUID               TEXT("GUID")
#define REGSTR_VAL_LAUNCH_APPS_W     L"LaunchApplications"
#define REGSTR_VAL_LAUNCH_APPS        TEXT("LaunchApplications")
#define REGSTR_VAL_LAUNCHABLE_W      L"Launchable"
#define REGSTR_VAL_LAUNCHABLE         TEXT("Launchable")
#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later
#define REGSTR_VAL_SHUTDOWNDELAY_W   L"ShutdownIfUnusedDelay"
#define REGSTR_VAL_SHUTDOWNDELAY      TEXT("ShutdownIfUnusedDelay")
#endif //#if (_WIN32_WINNT >= 0x0600)

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later
//
// CustomDeviceProperty names and values
//
#define IS_DIGITAL_CAMERA_STR   L"IsDigitalCamera"
#define IS_DIGITAL_CAMERA_VAL   1
#define SUPPORTS_MSCPLUS_STR    L"SupportsMSCPlus"
#define SUPPORTS_MSCPLUS_VAL    1
#endif //#if (_WIN32_WINNT >= 0x0501)

//
// Device instance value names
//
#define STI_DEVICE_VALUE_TWAIN_NAME    L"TwainDS"
#define STI_DEVICE_VALUE_ISIS_NAME     L"ISISDriverName"
#define STI_DEVICE_VALUE_ICM_PROFILE   L"ICMProfile"
#define STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP  L"DefaultLaunchApp"
#define STI_DEVICE_VALUE_TIMEOUT       L"PollTimeout"
#define STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS  L"DisableNotifications"
#define REGSTR_VAL_BAUDRATE            L"BaudRate"

#define STI_DEVICE_VALUE_TWAIN_NAME_A  "TwainDS"
#define STI_DEVICE_VALUE_ISIS_NAME_A   "ISISDriverName"
#define STI_DEVICE_VALUE_ICM_PROFILE_A   "ICMProfile"
#define STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A  "DefaultLaunchApp"
#define STI_DEVICE_VALUE_TIMEOUT_A       "PollTimeout"
#define STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A  "DisableNotifications"
#define REGSTR_VAL_BAUDRATE_A            "BaudRate"

//
// DEVPKEY_WIA_DeviceType
//
// The property GUID is the same as for the Imaging device class: {6BDD1FC6-810F-11D0-BEC7-08002BE2092F}
//
// Namespace: System.Devices.WiaDeviceType
//
// Type: DEVPROP_TYPE_UINT32
//
// This property can be set to any of the STI_DEVICE_TYPE enumeration values (see sti.h), including:
//
// 1 - StiDeviceTypeScanner (still image scanner: STI, WIA 1.0 or WIA 2.0)
// 2 - StiDeviceTypeDigitalCamera (still digital camera, WIA 1.0)
// 3 - StiDeviceTypeStreamingVideo (video streaming device such as webcam, still of the Imaging class,
//     formerly serviced by WIA Video but no longer STI/WIA compliant)
//
// For example a modern app can use this property to enumerate WIA scanner devices:
//
// System.Devices.InterfaceClassGuid:="{6bdd1fc6-810f-11d0-bec7-08002be2092f}" AND System.Devices.WiaDeviceType:=1
//
// ... where {6bdd1fc6-810f-11d0-bec7-08002be2092f} is the GUID of the Imaging device class and 1 is StiDeviceTypeScanner.
// Also note in this example that the app accesses the property via its namespace (System.Devices.WiaDeviceType), not via
// its internal DEVPKEY definition.
//

#include <devpropdef.h>

DEFINE_DEVPROPKEY(DEVPKEY_WIA_DeviceType, 0x6bdd1fc6, 0x810f, 0x11d0, 0xbe, 0xc7, 0x08, 0x00, 0x2b, 0xe2, 0x09, 0x2f, 2);

//
// DEVPKEY_WIA_USDClassId
//
// Type: DEVPROP_TYPE_STRING
//
// This property specifies the CLASSID of a WIA minidriver DLL.
// With DEVPKEY_WIA_USDClassId, a scan class monitor can create a devnode and register its interface with this property, allowing the WIA service to find the associated
// minidriver without a kernel mode device driver and INF file.
// Previously, a scan class monitor had to use a combination of a kernel mode device driver plus an INF file to set the device registry value USDClass in order to point
// the WIA service to the associated minidriver DLL.
//
DEFINE_DEVPROPKEY(DEVPKEY_WIA_USDClassId, 0x6bdd1fc6, 0x810f, 0x11d0, 0xbe, 0xc7, 0x08, 0x00, 0x2b, 0xe2, 0x09, 0x2f, 3);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _STIREG_

