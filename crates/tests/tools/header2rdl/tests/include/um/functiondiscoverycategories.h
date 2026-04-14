//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//

// *****************************************************************************
// Important:  Anything added here should also be added to FunctionDiscoveryManagedKeys.h
// *****************************************************************************

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define FD_SUBKEY                               L"SOFTWARE\\Microsoft\\Function Discovery\\"
#define FD_SUBKEY_CATEGORIES                    FD_SUBKEY L"Categories\\"

// *****************************************************************************
// Function Discovery Categories
// *****************************************************************************
// Important:  Anything added here should also be added to FunctionDiscoveryManagedKeys.h
// *****************************************************************************

// Provider Categories
// Windows Vista
#define FCTN_CATEGORY_PNP                       L"Provider\\Microsoft.Base.PnP"
#define FCTN_CATEGORY_REGISTRY                  L"Provider\\Microsoft.Base.Registry"
#define FCTN_CATEGORY_SSDP                      L"Provider\\Microsoft.Networking.SSDP"
#define FCTN_CATEGORY_WSDISCOVERY               L"Provider\\Microsoft.Networking.WSD"
#define FCTN_CATEGORY_NETBIOS                   L"Provider\\Microsoft.Networking.Netbios"
#define FCTN_CATEGORY_WCN                       L"Provider\\Microsoft.Networking.WCN"
#define FCTN_CATEGORY_PUBLICATION               L"Provider\\Microsoft.Base.Publication"
#define FCTN_CATEGORY_PNPXASSOCIATION           L"Provider\\Microsoft.PnPX.Association"
// Wireless Update Release
#define FCTN_CATEGORY_BT                        L"Provider\\Microsoft.Devices.Bluetooth"
#define FCTN_CATEGORY_WUSB                      L"Provider\\Microsoft.Devices.WirelessUSB"
#define FCTN_CATEGORY_DEVICEDISPLAYOBJECTS      L"Provider\\Microsoft.Base.DeviceDisplayObjects"
#define FCTN_CATEGORY_DEVQUERYOBJECTS           L"Provider\\Microsoft.Base.DevQueryObjects"

// Layered Categories
// Windows Vista
#define FCTN_CATEGORY_NETWORKDEVICES            L"Layered\\Microsoft.Networking.Devices"
#define FCTN_CATEGORY_DEVICES                   L"Layered\\Microsoft.Base.Devices"
#define FCTN_CATEGORY_DEVICEFUNCTIONENUMERATORS L"Layered\\Microsoft.Devices.FunctionEnumerators"
#define FCTN_CATEGORY_DEVICEPAIRING             L"Layered\\Microsoft.Base.DevicePairing"

// *****************************************************************************
// Function Discovery SubCategories
// *****************************************************************************
// Important:  Anything added here should also be added to FunctionDiscoveryManagedKeys.h
// *****************************************************************************

// Subcategories of Devices FCTN_CATEGORY_DEVICES
#define FCTN_SUBCAT_DEVICES_WSDPRINTERS         L"WSDPrinters"

// Subcategories of Devices FCTN_CATEGORY_NETWORKDEVICES
#define FCTN_SUBCAT_NETWORKDEVICES_SSDP         L"SSDP"
#define FCTN_SUBCAT_NETWORKDEVICES_WSD          L"WSD"

// Subcategories of Registry
#define FCTN_SUBCAT_REG_PUBLICATION             L"Publication"
#define FCTN_SUBCAT_REG_DIRECTED                L"Directed"

// *****************************************************************************
// Important:  Anything added here should also be added to FunctionDiscoveryManagedKeys.h
// *****************************************************************************


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

