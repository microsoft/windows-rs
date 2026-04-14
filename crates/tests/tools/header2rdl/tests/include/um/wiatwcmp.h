/****************************************************************************
*
*  (C) COPYRIGHT 1998-2001, MICROSOFT CORP.
*
*  FILE:        wiatwcmp.h
*
*  VERSION:     1.0
*
*  DATE:        6/01/2001
*
*  DESCRIPTION:
*    Defines TWAIN Compatibility Layer - Capability pass-through constants.
*    To support existing TWAIN applications that have private capabilities,
*    WIA drivers can utilize the Pass-through functionality.
*
*****************************************************************************/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#ifndef _WIATWCMP_H_
#define _WIATWCMP_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define WiaItemTypeTwainCapabilityPassThrough   0x00020000

//**************************************************************************
//
// TWAIN capability pass-through
//
//**************************************************************************

//
// Escape code ranges 2001 - 3000 are reserved for future ESC_ commands
//

#define ESC_TWAIN_CAPABILITY                2001    // private TWAIN capability negotiation
#define ESC_TWAIN_PRIVATE_SUPPORTED_CAPS    2002    // query for supported private capabilities

typedef struct _TWAIN_CAPABILITY {
    LONG  lSize;    // size of TWAIN_CAPABILITY structure
    LONG  lMSG;     // TWAIN Message, MSG_GET, MSG_GETCURRENT, MSG_SET, etc..
    LONG  lCapID;   // id of capability to set or get
    LONG  lConType; // container type of capability
    LONG  lRC;      // TWAIN return code, TWRC_SUCCESS, TWRC_FAILURE, etc..
    LONG  lCC;      // TWAIN condition code, TWCC_SUCCESS, TWCC_BUMMER, etc..
    LONG  lDataSize;// data size
    BYTE  Data[1];  // first BYTE of data
}TWAIN_CAPABILITY,*PTWAIN_CAPABILITY;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

#endif //#if (_WIN32_WINNT >= 0x0501)
