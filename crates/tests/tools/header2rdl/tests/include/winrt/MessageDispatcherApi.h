//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <IMessageDispatcher.h>

typedef IMessageDispatcher *PMessageDispatcher;

#ifdef __cplusplus
extern "C"
{
#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

WINOLEAPI_(void) CoSetMessageDispatcher(
    _In_opt_ PMessageDispatcher pMessageDispatcher);

WINOLEAPI_(void) CoHandlePriorityEventsFromMessagePump();

#endif

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
