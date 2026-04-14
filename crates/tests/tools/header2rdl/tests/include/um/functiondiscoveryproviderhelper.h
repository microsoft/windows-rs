//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


class CFDProviderHelper
{
public:
    CFDProviderHelper() : m_hToken(NULL), m_dwSessionId((DWORD) -1)
    {
        InitializeSRWLock(&srwTokenLock);
    }
    ~CFDProviderHelper()
    {
        if ( NULL != m_hToken )
            CloseHandle( m_hToken );
    }

// Implementation
public:
    /////////////////////////////////////////////////////////////////////////////
    // CoSetProxyBlanket
    //
    // Purpose:
    //    If the provider is running in a process that doesn't have permissions
    //    to call back into the client process by default then CoSetProxyBlanket
    //    must be called to enable impersonation.
    //    This function wraps the necessary calls and provides the correct CoSetProxyBlanket
    //    parameters in a single function.
    //
    // Arguments:
    //    pIUnk: Interface proxy to enable callbacks on
    //
    // Returns: S_OK on success, appropriate HRESULT otherwise
    static HRESULT CoSetProxyBlanket( IUnknown *pIUnk )
    {
        HRESULT hr = S_OK;
        IUnknown* pRealUnknown; 

        // Try to see if it's a COM proxy.  If so, see if it's local to the process or a remote COM proxy.
        // If it's remote, set the proxy blanket
        ULONG_PTR ulRpcOptions = GetRpcOptions( pIUnk );
        
        if ( SERVER_LOCALITY_PROCESS_LOCAL != ulRpcOptions )
        {
            hr = CoImpersonateClient();
            if ( S_OK == hr )
            {
                hr = pIUnk->QueryInterface(IID_IUnknown, (LPVOID*)&pRealUnknown );
                if ( S_OK == hr )
                {
                    hr = ::CoSetProxyBlanket( pRealUnknown, RPC_C_AUTHN_DEFAULT, RPC_C_AUTHZ_NONE, COLE_DEFAULT_PRINCIPAL, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, NULL, EOAC_STATIC_CLOAKING );
                    if ( S_OK == hr )
                    {
                        hr = ::CoSetProxyBlanket( pIUnk, RPC_C_AUTHN_DEFAULT, RPC_C_AUTHZ_NONE, COLE_DEFAULT_PRINCIPAL,
                                              RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, NULL, EOAC_STATIC_CLOAKING );
                    }
                    if ( E_NOINTERFACE == hr )
                        hr = S_OK;  // Provider can end up being inproc if we are in the fdPHost process.  Ignore error and try to proceed.

                    pRealUnknown->Release();
                }
                if ( S_OK == hr )
                    hr = CoRevertToSelf();
                else
                    CoRevertToSelf();   // Don't want the return code
            }
        }

        return hr;
    }

    /////////////////////////////////////////////////////////////////////////////
    // CoSetProxyBlanketWithThreadToken
    //
    // Purpose:
    //    If the provider is running in a process that doesn't have permissions
    //    to call back into the client process by default then CoSetProxyBlanket
    //    must be called to enable impersonation.
    //    This function wraps the necessary calls to CoSetProxyBlanket if the
    //    provider thread is from a thread pool instead of a thread called directly
    //    by FD.
    //    If you want to use this method you must call Initialize on the class instance.
    //    This is best done in IFunctionDiscoveryProvider::Initialize.
    //    Initialize must be called from one of the methods called by the client process.
    //
    // Arguments:
    //    pIUnk: Interface proxy to enable callbacks on
    //
    // Returns: S_OK on success, appropriate HRESULT otherwise
    HRESULT CoSetProxyBlanketWithThreadToken( IUnknown *pIUnk )
    {
        if ( NULL == m_hToken )
            return S_OK;
        IUnknown* pRealUnknown;

        HRESULT hr = S_OK;

        AcquireSRWLockShared(&srwTokenLock);
        if ( !SetThreadToken( NULL, m_hToken ))
            hr = HRESULT_FROM_WIN32( GetLastError() );
        ReleaseSRWLockShared(&srwTokenLock);
        
        if ( S_OK == hr )
        {
            hr = pIUnk->QueryInterface(IID_IUnknown, (LPVOID*)&pRealUnknown );
            if ( S_OK == hr )
            {
                hr = ::CoSetProxyBlanket( pRealUnknown, RPC_C_AUTHN_DEFAULT, RPC_C_AUTHZ_NONE, COLE_DEFAULT_PRINCIPAL, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, NULL, EOAC_STATIC_CLOAKING );
                if (S_OK == hr)
                    hr = ::CoSetProxyBlanket( pIUnk, RPC_C_AUTHN_DEFAULT, RPC_C_AUTHZ_NONE, COLE_DEFAULT_PRINCIPAL, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, NULL, EOAC_STATIC_CLOAKING );
                if ( E_NOINTERFACE == hr )
                    hr = S_OK;  // Provider can end up being inproc if we are in the fdPHost process.  Ignore error and try to proceed.

                pRealUnknown->Release();
            }
            if ( !SetThreadToken( NULL, NULL ))
            {
                if ( S_OK == hr )
                    hr = HRESULT_FROM_WIN32( GetLastError() );
            }
        }

        return hr;
    }

    /////////////////////////////////////////////////////////////////////////////
    // Initialize
    //
    // Purpose:
    //    If the provider is running in a process that doesn't have permissions
    //    to call back into the client process by default then CoSetProxyBlanket
    //    must be called to enable impersonation.
    //    CoSetProxyBlanketWithThreadToken wraps the calls necessary to do this.
    //    Before using that function you must call Initialize on the class instance.
    //    This is best done in IFunctionDiscoveryProvider::Initialize.
    //    Initialize must be called from one of the methods called by the client process.
    //
    //    Note:  There are three cases we we will not impersonate
    //    1. If the caller is Local System we will not impersonate and subsequent
    //       calls to CoSetProxyBlanketWithThreadToken will effectively noop
    //    2. Similary if the provider is actually running in-proc no proxy blanket
    //       is required and CoImpersonateClient will fail with:
    //          Code=0x80010117 (2147549463): RPC_E_CALL_COMPLETE: "Call context cannot be accessed after call completed."
    //          Severity=FAILURE; Code=279 (0x117); Facility=1 (0x1) (FACILITY_RPC)
    //       Handle this case gracefully and effectively noop on the
    //       CoSetProxyBlanketWithThreadToken as well.
    //    3. If we get an inproc proxy to the provider we detect that and subsequent
    //       calls to CoSetProxyBlanketWithThreadToken will effectively noop.
    //
    // Returns: S_OK on success, appropriate HRESULT otherwise
    HRESULT Initialize( IUnknown *pIUnk )
    {
        if ( NULL != m_hToken )
            return S_FALSE;

        const SID LOCAL_SYSTEM_SID = { SID_REVISION, 1, {0,0,0,0,0,5}, SECURITY_LOCAL_SYSTEM_RID };

        HRESULT hr = CoImpersonateClient();

        if ( S_OK == hr )
        {
            BOOL isLocalSystem = FALSE;
            BOOL isLocalCall = (GetRpcOptions( pIUnk ) == SERVER_LOCALITY_PROCESS_LOCAL);

            if (!isLocalCall)
            {
                if ( !CheckTokenMembership( NULL, (PSID)&LOCAL_SYSTEM_SID, &isLocalSystem ))
                {
                    hr = HRESULT_FROM_WIN32( GetLastError() );
                }
            }

            // Only save the impersonation info if 
            // this is not a local call, and the caller is not System.
            if ( (S_OK == hr) && (!isLocalCall) && (!isLocalSystem) )
            {
                AcquireSRWLockExclusive(&srwTokenLock);
                if ( !OpenThreadToken( GetCurrentThread(), TOKEN_IMPERSONATE | TOKEN_QUERY, FALSE, &m_hToken ))
                    hr = HRESULT_FROM_WIN32( GetLastError() );
                if ( S_OK == hr )
                {
                    DWORD dwSizeReturned;
                    if ( !GetTokenInformation(m_hToken, TokenSessionId, &m_dwSessionId, sizeof(m_dwSessionId), &dwSizeReturned ))
                        hr = HRESULT_FROM_WIN32( GetLastError() );
                }
                ReleaseSRWLockExclusive(&srwTokenLock);
            }
            if ( S_OK == hr )
                hr = CoRevertToSelf();
            else
                CoRevertToSelf();   // Don't want the return code
        }
        else if ( RPC_E_CALL_COMPLETE == hr )
        {
            hr = S_OK;  // InProc
        }

        return hr;
    }

    BOOL ReleaseToken(DWORD dwSessionId)
    {
        BOOL bMatch = FALSE;
        
        AcquireSRWLockExclusive( &srwTokenLock );
        if ( NULL != m_hToken && dwSessionId == m_dwSessionId )
        {
            CloseHandle( m_hToken );
            m_hToken = NULL;
            bMatch = TRUE;
        }
        ReleaseSRWLockExclusive( &srwTokenLock );

        return bMatch;
    }
    VOID ReleaseToken()
    {
        AcquireSRWLockExclusive( &srwTokenLock );
        if ( NULL != m_hToken)
        {
            CloseHandle( m_hToken );
            m_hToken = NULL;
            m_dwSessionId = (DWORD) -1;
        }
        ReleaseSRWLockExclusive( &srwTokenLock );
    }

// Implementation
protected:
    /////////////////////////////////////////////////////////////////////////////
    // GetRpcOptions
    //
    // Purpose:
    //    Get the rpc options if this is a proxy.
    //
    // Arguments:
    //    pIUnk: Interface proxy.
    //
    // Returns: ULONG_PTR rpc options on success, assume it's inproc otherwise (SERVER_LOCALITY_PROCESS_LOCAL)
    static ULONG_PTR GetRpcOptions( IUnknown *pIUnk )
    {
        ULONG_PTR ulOptions = SERVER_LOCALITY_PROCESS_LOCAL;
        IRpcOptions* pRpcOptions;
        
        HRESULT hr = pIUnk->QueryInterface( __uuidof( IRpcOptions ), reinterpret_cast<LPVOID*>( &pRpcOptions ) );
        if ( S_OK == hr )
        {
            hr = pRpcOptions->Query( pIUnk, COMBND_SERVER_LOCALITY, &ulOptions );
            pRpcOptions->Release();
        }
        return ulOptions;
    }

// Attributes
protected:
    DWORD   m_dwSessionId;
    HANDLE  m_hToken;
    SRWLOCK srwTokenLock;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

