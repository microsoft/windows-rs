//*@@@+++@@@@******************************************************************
//
// Microsoft Windows
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//*@@@---@@@@******************************************************************

/******************************Module*Header*******************************\
*
* Module Name: DLNADeviceInterfaceIds.h
*
* DLNA Device Interface Ids
*
\**************************************************************************/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if NTDDI_VERSION >= NTDDI_WIN8

#ifndef INITGUID
#define INITGUID
#endif
#include <guiddef.h>

// {D0875FB4-2196-4c7a-A63D-E416ADDD60A1}
#ifndef GUID_DEVINTERFACE_DMR
DEFINE_GUID( GUID_DEVINTERFACE_DMR,  0xD0875FB4, 0x2196, 0x4c7a, 0xA6, 0x3D, 0xE4, 0x16, 0xAD, 0xDD, 0x60, 0xA1 );
#endif

// {25B4E268-2A05-496e-803B-266837FBDA4B}
#ifndef GUID_DEVINTERFACE_DMP
DEFINE_GUID( GUID_DEVINTERFACE_DMP,  0x25B4E268, 0x2A05, 0x496e, 0x80, 0x3B, 0x26, 0x68, 0x37, 0xFB, 0xDA, 0x4B );
#endif

// {C96037AE-A558-4470-B432-115A31B85553}
#ifndef GUID_DEVINTERFACE_DMS
DEFINE_GUID( GUID_DEVINTERFACE_DMS,  0xC96037AE, 0xA558, 0x4470, 0xB4, 0x32, 0x11, 0x5A, 0x31, 0xB8, 0x55, 0x53 );
#endif

#endif // NTDDI_VERSION >= NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

