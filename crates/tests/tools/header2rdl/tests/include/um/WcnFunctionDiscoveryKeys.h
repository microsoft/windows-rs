/*++

Copyright (c) Microsoft Corporation

Module Name:

    WcnFunctionDiscoveryKeys.h

Abstract:

    Function Discovery PKEYs used by WCN API

--*/


#ifndef _wcnfunctiondiscoverykeys_h_
#define _wcnfunctiondiscoverykeys_h_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if NTDDI_VERSION >= NTDDI_WIN7

#include <PropKeyDef.h>

// C100BECA-D33A-4a4b-BF23-BBEF4663D017
DEFINE_GUID( SID_WcnProvider, 0xC100BECA,0xD33A,0x4A4B,0xBF,0x23,0xBB,0xEF,0x46,0x63,0xD0,0x17);


DEFINE_PROPERTYKEY(PKEY_WCN_DeviceType_Category,       0x88190b8b, 0x4684, 0x11da, 0xa2, 0x6a, 0x00, 0x02, 0xb3, 0x98, 0x8e, 0x81, 0x00000010); // VT_UINT
DEFINE_PROPERTYKEY(PKEY_WCN_DeviceType_SubCategoryOUI, 0x88190b8b, 0x4684, 0x11da, 0xa2, 0x6a, 0x00, 0x02, 0xb3, 0x98, 0x8e, 0x81, 0x00000011); // VT_UINT
DEFINE_PROPERTYKEY(PKEY_WCN_DeviceType_SubCategory,    0x88190b8b, 0x4684, 0x11da, 0xa2, 0x6a, 0x00, 0x02, 0xb3, 0x98, 0x8e, 0x81, 0x00000012); // VT_UINT
DEFINE_PROPERTYKEY(PKEY_WCN_SSID,                      0x88190b8b, 0x4684, 0x11da, 0xa2, 0x6a, 0x00, 0x02, 0xb3, 0x98, 0x8e, 0x81, 0x00000020); // VT_LPWSTR (should be VT_UI1|VT_VECTOR, but that's not supported by IFunctionInstanceCollectionQuery::AddPropertyConstraint)


#endif // NTDDI_VERSION >= NTDDI_WIN7

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _wcnfunctiondiscoverykeys_h_

