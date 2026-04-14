// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_FRAGMENT_VIRTUAL_ADDRESS
{

    void *
        VirtualAddress;

} NET_FRAGMENT_VIRTUAL_ADDRESS;

#ifndef WDFAPI
DECLARE_HANDLE(WDFMEMORY);
#endif

typedef struct _NET_FRAGMENT_WDFMEMORY
{
    WDFMEMORY
        WdfMemory;

} NET_FRAGMENT_WDFMEMORY;

C_ASSERT(sizeof(NET_FRAGMENT_VIRTUAL_ADDRESS) == sizeof(void *));

#pragma warning(pop)

EXTERN_C_END


#define NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_NAME L"ms_fragment_virtualaddress"
#define NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_VERSION_1 1U

#define NET_FRAGMENT_EXTENSION_WDFMEMORY_NAME L"ms_fragment_wdfmemory"
#define NET_FRAGMENT_EXTENSION_WDFMEMORY_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

