//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright 1996-1998 Microsoft Corporation. All Rights Reserved.
//
//  File: scrptids.h
//
//--------------------------------------------------------------------------

// Invent DISPIDs for the things that don't have built-in dispid's
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define DISPID_LOAD		1
#define DISPID_UNLOAD	2
#define DISPID_SUBMIT	3
#define DISPID_FOCUS	4
#define DISPID_CHANGE	5
#define DISPID_BLUR		6
#define DISPID_SELECT	7
#define DISPID_MOUSEOVER 8
#define DISPID_PARSECOMPLETE 9
#define DISPID_ABORT	10
#define DISPID_ERROR	11
#define DISPID_MOUSEOUT 12

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

