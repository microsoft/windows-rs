//
//    Copyright (C) Microsoft.  All rights reserved.
//
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later
#include <wia_lh.h>
#elif (_WIN32_WINNT >= 0x0501) // Windows XP and later
#include <wia_xp.h>
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

