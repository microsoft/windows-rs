// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_FRAGMENT_MDL
{

    MDL *
        Mdl;

} NET_FRAGMENT_MDL;

C_ASSERT(sizeof(NET_FRAGMENT_MDL) == sizeof(void *));

#pragma warning(pop)

EXTERN_C_END


#define NET_FRAGMENT_EXTENSION_MDL_NAME L"ms_fragment_mdl"
#define NET_FRAGMENT_EXTENSION_MDL_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

