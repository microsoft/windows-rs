//
//    Copyright (C) Microsoft.  All rights reserved.
//
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#include <wiamindr_lh.h>
#elif (NTDDI_VERSION >= NTDDI_WINXP)
#include <wiamindr_xp.h>
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

