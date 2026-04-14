/*++

Copyright (C) Microsoft Corporation, 2006

Module Name:

    sliddefs.h

Abstract:

    Software Licensing GUID definitions
   
--*/
#pragma once

#ifndef _SLIDDEFS_H_
#define _SLIDDEFS_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

#ifndef _WINDOWS_SLID_
#define _WINDOWS_SLID_
DEFINE_GUID(WINDOWS_SLID, 0x55c92734, 0xd682, 0x4d71, 0x98, 0x3e, 0xd6, 0xec, 0x3f, 0x16, 0x05, 0x9f);
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
