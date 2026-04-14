/*++

Copyright (c) Microsoft Corporation

Module Name:

    WcnApi.h

Abstract:

    Central header for WCN API

--*/


#ifndef _wcnapi_h_
#define _wcnapi_h_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if !defined(NTDDI_VERSION) || NTDDI_VERSION < NTDDI_WIN7
#  error WcnApi.h is only available when targeting Windows 7 and higher
#endif // NTDDI_VERSION < NTDDI_WIN7


#include <WcnTypes.h>
#include <WcnDevice.h>

#ifndef NO_WCN_PKEYS
#  include <WcnFunctionDiscoveryKeys.h>
#endif // !NO_WCN_KEYS


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _wcnapi_h_

