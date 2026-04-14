// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#include <net/mdltypes.h>

EXTERN_C_START

inline
NET_FRAGMENT_MDL *
NetExtensionGetFragmentMdl(
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    return (NET_FRAGMENT_MDL *)NetExtensionGetData(Extension, Index);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

