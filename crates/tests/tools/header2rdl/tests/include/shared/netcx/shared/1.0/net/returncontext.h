// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#include <net/returncontexttypes.h>

EXTERN_C_START

inline
NET_FRAGMENT_RETURN_CONTEXT *
NetExtensionGetFragmentReturnContext(
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    return (NET_FRAGMENT_RETURN_CONTEXT *)NetExtensionGetData(Extension, Index);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

