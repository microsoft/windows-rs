/*++

Copyright (c) Microsoft Corporation

Module Name:

    transportsettingcommon.h

Environment:

    User mode or kernel mode

--*/

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifndef _TRANSPORT_SETTING_COMMON_
#define _TRANSPORT_SETTING_COMMON_

#ifdef _MSC_VER

#pragma once

#endif  // _MSC_VER

#ifdef __cplusplus
extern  "C" {
#endif // __cplusplus

//
// This structure represents the key used in query and apply transport setting
// APIs.
//
typedef struct TRANSPORT_SETTING_ID
{
    GUID Guid;
} TRANSPORT_SETTING_ID, *PTRANSPORT_SETTING_ID;


#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _TRANSPORT_SETTING_COMMON_
#endif // (NTDDI_VERSION >= NTDDI_WIN8)