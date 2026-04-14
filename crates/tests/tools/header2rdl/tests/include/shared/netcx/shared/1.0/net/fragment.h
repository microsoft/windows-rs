// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef NETCX_ADAPTER_2
#error include netadaptercx.h
#endif

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding
#pragma warning(disable:4214) // nonstandard extension used: bit field types other than int

EXTERN_C_START

typedef struct _NET_FRAGMENT
{
    UINT64
        ValidLength : 26;

    UINT64
        Capacity : 26;

    UINT64
        Offset : 10;

    UINT64
        Scratch : 1;

    UINT64
        OsReserved_Bounced : 1;

} NET_FRAGMENT;

EXTERN_C_END

C_ASSERT(sizeof(NET_FRAGMENT) == 8);

#pragma warning(pop)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

