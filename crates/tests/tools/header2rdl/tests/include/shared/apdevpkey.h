
/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    apdevpkey.h

Abstract:

    Defines Plug and Play Device property keys used for AutoPlay behavior.

Environment:

    User and Kernel modes.


--*/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#include "devpropdef.h"

//
// Indicate that AutoPlay should not be displayed for this device interface.
//
DEFINE_DEVPROPKEY(DEVPKEY_DeviceInterface_Autoplay_Silent, 0x434dd28f, 0x9e75, 0x450a, 0x9a, 0xb9, 0xff, 0x61, 0xe6, 0x18, 0xba, 0xd0, 2);  // DEVPROP_TYPE_BOOLEAN
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
