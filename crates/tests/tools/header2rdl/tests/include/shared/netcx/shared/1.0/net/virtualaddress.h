// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#include <net/virtualaddresstypes.h>

EXTERN_C_START

inline
NET_FRAGMENT_VIRTUAL_ADDRESS *
NetExtensionGetFragmentVirtualAddress(
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    return (NET_FRAGMENT_VIRTUAL_ADDRESS *)NetExtensionGetData(Extension, Index);
}

inline
void *
NetExtensionGetFragmentVirtualAddressOffset(
    NET_FRAGMENT const * Fragment,
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    NET_FRAGMENT_VIRTUAL_ADDRESS const * virtualAddress =
        NetExtensionGetFragmentVirtualAddress(Extension, Index);

    return (UINT8 *)virtualAddress->VirtualAddress + Fragment->Offset;
}

inline
NET_FRAGMENT_WDFMEMORY *
NetExtensionGetFragmentWdfMemory(
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    return (NET_FRAGMENT_WDFMEMORY *)NetExtensionGetData(Extension, Index);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

