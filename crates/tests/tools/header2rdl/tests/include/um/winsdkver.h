/*

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    WinSDKVer.h

Abstract:

    Master include file for versioning content that ships in the Windows SDK.

*/

#ifndef _INC_WINSDKVER
#define _INC_WINSDKVER

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// This list contains the highest version constants supported by content in the Windows SDK.

#define _WIN32_MAXVER           0x0A00
#define _WIN32_WINDOWS_MAXVER   0x0A00
#define NTDDI_MAXVER            0x0A00
#define _WIN32_IE_MAXVER        0x0A00
#define _WIN32_WINNT_MAXVER     0x0A00
#define WINVER_MAXVER           0x0A00


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


