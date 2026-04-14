//=--------------------------------------------------------------------------=
// MsCtfMonitorApi.h
//
//
// MsCtfMonitorApi declarations.

//=--------------------------------------------------------------------------=
// (C) Copyright 1995-2003 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR TFPLIED, INCLUDING BUT NOT LIMITED TO
// THE TFPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



////////////////////////////////////////////////////////////////////////////
//
// DoMsCtfMonitor for ctfmon.exe
//
////////////////////////////////////////////////////////////////////////////

#define DCM_FLAGS_TASKENG         0x00000001
#define DCM_FLAGS_CTFMON          0x00000002
#define DCM_FLAGS_LOCALTHREADTSF  0x00000004
EXTERN_C BOOL WINAPI DoMsCtfMonitor(DWORD dwFlags, HANDLE hEventForServiceStop);

////////////////////////////////////////////////////////////////////////////
//
// Flags for InitLocalMsCtfMonitor()
//
// ILMCM_CHECKLAYOUTANDTIPENABLED
//
//    With this flag, InitLocalMsCtfMonitor() checkes the keyboard layout or 
//    tips are enabled. If there is only one keyboard layout, it does not 
//    initialize Text Services Framework or start the language bar.
//
////////////////////////////////////////////////////////////////////////////
#define ILMCM_CHECKLAYOUTANDTIPENABLED    0x00001    
////////////////////////////////////////////////////////////////////////////
//
// ILMCM_LANGUAGEBAROFF
//
//    With this flag set, InitLocalMsCtfMonitor() won't start the default 
//    system language bar for the desktop.
//
////////////////////////////////////////////////////////////////////////////
#define ILMCM_LANGUAGEBAROFF              0x00002    

////////////////////////////////////////////////////////////////////////////
//
// InitLocalMsCtfMonitor
//
////////////////////////////////////////////////////////////////////////////
EXTERN_C HRESULT WINAPI InitLocalMsCtfMonitor(DWORD dwFlags);

////////////////////////////////////////////////////////////////////////////
//
// UninitLocalMsCtfMonitor
//
////////////////////////////////////////////////////////////////////////////
EXTERN_C HRESULT WINAPI UninitLocalMsCtfMonitor();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

