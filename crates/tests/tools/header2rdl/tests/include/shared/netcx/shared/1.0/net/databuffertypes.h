// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#include <net/databuffertypes.h>

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef void * NET_MEMORY_ID;

typedef struct _NET_FRAGMENT_NET_MEMORY
{
    NET_MEMORY_ID
        NetMemoryId;

} NET_FRAGMENT_NET_MEMORY;

C_ASSERT(sizeof(NET_FRAGMENT_NET_MEMORY) == sizeof(NET_MEMORY_ID));

#pragma warning(pop)

EXTERN_C_END


#define NET_FRAGMENT_EXTENSION_NET_MEMORY_NAME L"ms_fragment_netmemory"
#define NET_FRAGMENT_EXTENSION_NET_MEMORY_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

