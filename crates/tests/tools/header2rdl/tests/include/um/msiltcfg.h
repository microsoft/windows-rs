/*****************************************************************************\
*                                                                             *
* msiltcfg.h - - Interface for external access to Installer Service           *
*                                                                             *
* Version 4.0                                                                 *
*                                                                             *
* NOTES:  All buffers sizes are TCHAR count, null included only on input      *
*         Return argument pointers may be null if not interested in value     *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/

#ifndef _MSILTCFG_H_
#define _MSILTCFG_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

// Send shutdown msi.dll message to all the processes

UINT WINAPI ShutdownMsi(DWORD dwReserved);

// Send restart msi.dll message to all the processes

UINT WINAPI RestartMsi(DWORD dwReserved);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSILTCFG_H_
