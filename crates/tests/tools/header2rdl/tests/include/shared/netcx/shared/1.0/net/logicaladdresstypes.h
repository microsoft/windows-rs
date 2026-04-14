// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_FRAGMENT_LOGICAL_ADDRESS
{

    UINT64
        LogicalAddress;

} NET_FRAGMENT_LOGICAL_ADDRESS;

C_ASSERT(sizeof(NET_FRAGMENT_LOGICAL_ADDRESS) == sizeof(UINT64));

#pragma warning(pop)

EXTERN_C_END


#define NET_FRAGMENT_EXTENSION_LOGICAL_ADDRESS_NAME L"ms_fragment_logicaladdress"
#define NET_FRAGMENT_EXTENSION_LOGICAL_ADDRESS_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

