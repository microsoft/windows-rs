/*++

Copyright (c) 1986-2003  Microsoft Corporation

Module Name:

    wiaintfc.h

Abstract:

    This module contains interface class GUID for WIA.

Revision History:


--*/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#ifndef _WIAINTFC_H_
#define _WIAINTFC_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Set packing
//

#include <pshpack8.h>
#include <guiddef.h>

//
// GUID for Image class device interface.
//

DEFINE_GUID(GUID_DEVINTERFACE_IMAGE, 0x6bdd1fc6L, 0x810f, 0x11d0, 0xbe, 0xc7, 0x08, 0x00, 0x2b, 0xe2, 0x09, 0x2f);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _WIAINTFC_H_

#endif //#if (_WIN32_WINNT >= 0x0501)
