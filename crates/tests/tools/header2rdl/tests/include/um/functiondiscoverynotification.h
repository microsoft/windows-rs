#pragma once

//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//

//  CFunctionDiscoveryNotificationWrapper class.
//
//  All OS components implementing IFunctionDiscoveryNotification should derive
//  from this class rather then directly from IFunctionDiscoveryNotification
//  so that a default implementation can be provided here in case of future
//  breaking changes to the IFunctionDiscoveryNotification interface

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

interface IFunctionDiscoveryNotification;   // forward declaration

#ifdef __cplusplus
class CFunctionDiscoveryNotificationWrapper : public IFunctionDiscoveryNotification
{
public:
        virtual HRESULT STDMETHODCALLTYPE OnUpdate(
            /* [in] */ QueryUpdateAction,
            /* [in] */ FDQUERYCONTEXT,
            /* [in] */ __RPC__in_opt IFunctionInstance*)
        { return S_OK; }
        virtual HRESULT STDMETHODCALLTYPE OnError(
            /* [in] */ HRESULT,
            /* [in] */ FDQUERYCONTEXT,
            /* [string][in] */ __RPC__in_string const WCHAR*)
        { return S_OK; }
        virtual HRESULT STDMETHODCALLTYPE OnEvent(
            /* [in] */ DWORD,
            /* [in] */ FDQUERYCONTEXT,
            /* [string][in] */ __RPC__in_string const WCHAR*)
        { return S_OK; }
};
#endif

// Internal Event ids
#define FD_EVENTID_PRIVATE          100

// Event ids
#define FD_EVENTID                  1000
#define FD_EVENTID_SEARCHCOMPLETE   FD_EVENTID
#define FD_EVENTID_ASYNCTHREADEXIT  FD_EVENTID + 1
#define FD_EVENTID_SEARCHSTART      FD_EVENTID + 2
#define FD_EVENTID_IPADDRESSCHANGE  FD_EVENTID + 3
#define FD_EVENTID_QUERYREFRESH     FD_EVENTID + 4

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

