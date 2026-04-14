// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

DECLARE_HANDLE(NET_FRAGMENT_RETURN_CONTEXT_HANDLE);

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_FRAGMENT_RETURN_CONTEXT
{

    NET_FRAGMENT_RETURN_CONTEXT_HANDLE
        Handle;

} NET_FRAGMENT_RETURN_CONTEXT;

C_ASSERT(sizeof(NET_FRAGMENT_RETURN_CONTEXT) == sizeof(NET_FRAGMENT_RETURN_CONTEXT_HANDLE));

#pragma warning(pop)

EXTERN_C_END


#define NET_FRAGMENT_EXTENSION_RETURN_CONTEXT_NAME L"ms_fragment_returncontext"
#define NET_FRAGMENT_EXTENSION_RETURN_CONTEXT_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

