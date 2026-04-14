//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  WBEMGLUE.H
//
//  Purpose: Implementation of WBEM Glue classes
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _WbemGlue_H_Included
#define _WbemGlue_H_Included

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#undef PURE
#define PURE {return (ULONG)E_NOTIMPL;}
typedef LPVOID *PPVOID;

#include <wbemidl.h>
#include <map>
#include <set>
#include <provider.h>

#define DEFAULT_NAMESPACE       L"Root\\CimV2"

typedef std::map<CHString, LPVOID> STRING2LPVOID;
typedef std::map<LPCVOID, PLONG> PTR2PLONG;
typedef std::set<LPVOID> PROVIDERPTRS;

class CWbemGlueFactory;
class CWbemGlueImpersonation;

#ifdef _PREFAST_
#pragma prefast (push)  
#pragma prefast (disable: 26135) //We are using locks correctly.
#endif /* _PREFAST_ */

class POLARITY CWbemProviderGlue : public IWbemServices, public IWbemProviderInit
{

	friend class CWbemGlueImpersonation;

	public:
    // Public Static functions
    // these are for use by implementors of Framework based providers

    static HRESULT WINAPI GetAllInstances( LPCWSTR pszClassName, 
                                           TRefPointerCollection<CInstance> *pList, 
                                           LPCWSTR pszNamespace, 
                                           MethodContext *pMethodContext );

    static HRESULT WINAPI GetAllInstancesAsynch( LPCWSTR pszClassName, 
                                                 Provider *pRequester, 
                                                 LPProviderInstanceCallback pCallback, 
                                                 LPCWSTR pszNamespace, 
                                                 MethodContext *pMethodContext, 
                                                 void *pUserData );

    static HRESULT WINAPI GetAllDerivedInstancesAsynch( LPCWSTR pszBaseClassName, 
                                                        Provider *pRequester, 
                                                        LPProviderInstanceCallback pCallback, 
                                                        LPCWSTR pszNamespace, 
                                                        MethodContext *pMethodContext, 
                                                        void *pUserData );

    static HRESULT WINAPI GetAllDerivedInstances( LPCWSTR pszBaseClassName, 
                                                  TRefPointerCollection<CInstance> *pList, 
                                                  MethodContext *pMethodContext, 
#ifdef FRAMEWORK_ALLOW_DEPRECATED
                                                  LPCWSTR pszNamespace = NULL);
#else
                                                  LPCWSTR pszNamespace);
#endif

    static HRESULT WINAPI GetInstanceByPath( LPCWSTR pszObjectPath, 
                                             CInstance **ppInstance, 
#ifdef FRAMEWORK_ALLOW_DEPRECATED
                                             MethodContext *pMethodContext = NULL );
#else
                                             MethodContext *pMethodContext);
#endif

    static HRESULT WINAPI GetInstanceKeysByPath( LPCWSTR pszInstancePath,
                                                 CInstance **ppInstance,
                                                 MethodContext *pMethodContext);

    static HRESULT WINAPI GetInstancePropertiesByPath( LPCWSTR pszInstancePath,
                                                 CInstance **ppInstance,
                                                 MethodContext *pMethodContext,
                                                 CHStringArray &csaProperties);

    static HRESULT WINAPI GetInstancesByQuery( LPCWSTR query, 
                                               TRefPointerCollection<CInstance> *pList, 
                                               MethodContext *pMethodContext,  
#ifdef FRAMEWORK_ALLOW_DEPRECATED
                                               LPCWSTR pszNamespace = NULL);
#else
                                               LPCWSTR pszNamespace);
#endif

    static HRESULT WINAPI GetInstancesByQueryAsynch( LPCWSTR query, 
                                                     Provider *pRequester, 
                                                     LPProviderInstanceCallback pCallback, 
                                                     LPCWSTR pszNamespace, 
                                                     MethodContext *pMethodContext, 
                                                     void *pUserData );

#ifdef FRAMEWORK_ALLOW_DEPRECATED
    // This version of GetEmptyInstance is deprecated.  Use the next one.
    static HRESULT WINAPI GetEmptyInstance( LPCWSTR pszClassName, 
                                            CInstance **ppInstance, 
                                            LPCWSTR pszNamespace = NULL);
#endif

    static HRESULT WINAPI GetEmptyInstance( MethodContext *pMethodContext, 
                                            LPCWSTR pszClassName, 
                                            CInstance **ppInstance, 
#ifdef FRAMEWORK_ALLOW_DEPRECATED
                                            LPCWSTR pszNamespace = NULL);
#else
                                            LPCWSTR pszNamespace);
#endif

    // Both of these FillInstance calls are deprecated.  Use GetInstanceByPath, or the even
    // more performant functions GetInstanceKeysByPath or GetInstancePropertiesByPath.
#ifdef FRAMEWORK_ALLOW_DEPRECATED
    static HRESULT WINAPI FillInstance( CInstance *pInstance, 
                                        LPCWSTR pszNamespace = NULL );

    static HRESULT WINAPI FillInstance( MethodContext *pMethodContext, 
                                        CInstance *pInstance );
#endif

    // determine whether one class is derived from another
    static bool  WINAPI IsDerivedFrom( LPCWSTR pszBaseClassName, 
                                       LPCWSTR pszDerivedClassName, 
                                       MethodContext *pMethodContext, 
#ifdef FRAMEWORK_ALLOW_DEPRECATED
                                       LPCWSTR pszNamespace = NULL );
#else
                                       LPCWSTR pszNamespace);
#endif

    // logging in and out of framework
    // each framework based DLL must log in and out with these functions

     // Deprecated in favor of the version that takes a PLONG.
#ifdef FRAMEWORK_ALLOW_DEPRECATED
    static BOOL WINAPI FrameworkLoginDLL(LPCWSTR name);
    static BOOL WINAPI FrameworkLogoffDLL(LPCWSTR name);
#endif

    // You must pass the *same* PLONG to all three of FrameworkLoginDLL, 
    // FrameworkLogoffDLL, and the CWbemGlueFactory constructor.
    static BOOL WINAPI FrameworkLoginDLL(LPCWSTR name, PLONG plRefCount);
    static BOOL WINAPI FrameworkLogoffDLL(LPCWSTR name, PLONG plRefCount);

    static bool WINAPI SetStatusObject(MethodContext *pContext, LPCWSTR pNamespace, 
                                        LPCWSTR pDescription, HRESULT hr, 
                                        const SAFEARRAY *pPrivilegesNotHeld = NULL,
                                        const SAFEARRAY *pPrivilegesRequired = NULL);

    ////////////////////////////////////////////////////////////////////////////////////
    // note: the following public functions are necessary to allow COM communication  //
    //       with CIMOM.  Provider Implementors will not need to call these.          //
    ////////////////////////////////////////////////////////////////////////////////////

#ifndef NO_BASEINTERFACE_FUNCS

    /* IUnknown methods */
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID FAR *ppvObj) ;
    STDMETHOD_(ULONG, AddRef)(THIS) ;
    STDMETHOD_(ULONG, Release)(THIS) ;
#endif
    
   virtual HRESULT STDMETHODCALLTYPE Initialize( 
           _In_  LPWSTR pszUser,
            /* [in] */ LONG lFlags,
            _In_ LPWSTR pszNamespace,
            _In_ LPWSTR pszLocale,
            _In_ IWbemServices __RPC_FAR *pNamespace,
            _In_ IWbemContext __RPC_FAR *pCtx,
            _In_ IWbemProviderInitSink __RPC_FAR *pInitSink
    );

    STDMETHOD(CreateInstanceEnumAsync)(THIS_
            /* [in] */ const BSTR Class,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    );
    
    STDMETHOD(GetObjectAsync)( THIS_
            const BSTR ObjectPath, 
            long lFlags, 
            IWbemContext __RPC_FAR *pCtx, 
            IWbemObjectSink __RPC_FAR *pResponseHandler
    );
    
    STDMETHOD(ExecQueryAsync)(THIS_
            /* [in] */ const BSTR QueryLanguage,
            /* [in] */ const BSTR Query,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    );
    
    STDMETHOD(PutInstanceAsync)( THIS_
            /* [in] */ IWbemClassObject __RPC_FAR *pInst,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    );
    
    STDMETHOD(DeleteInstanceAsync)(
            /* [in] */ const BSTR ObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    );

    STDMETHOD(ExecMethodAsync)( const BSTR, 
                                const BSTR, 
                                long, 
                                IWbemContext*, 
                                IWbemClassObject*,
                                IWbemObjectSink*
    );


 // Unsupported service methods
 // ===========================

    STDMETHOD(OpenNamespace)(THIS_
            const BSTR Namespace, 
            long lFlags,  
            IWbemContext __RPC_FAR *pCtx,
            IWbemServices __RPC_FAR *__RPC_FAR *ppWorkingNamespace, 
            IWbemCallResult __RPC_FAR *__RPC_FAR *ppResult
    )
        { return E_NOTIMPL; }
    

    STDMETHOD(PutClass)(IWbemClassObject __RPC_FAR *pObject, 
                        long lFlags, 
                        IWbemContext __RPC_FAR *pCtx, 
                        IWbemCallResult __RPC_FAR *__RPC_FAR *ppCallResult
    )
         {return E_NOTIMPL;}

    STDMETHOD(PutClassAsync)( 
            /* [in] */ IWbemClassObject __RPC_FAR *pObject,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    )
         {return E_NOTIMPL;}

    STDMETHOD(DeleteClass)(  
            /* [in] */ const BSTR Class,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [unique][in][out] */ IWbemCallResult __RPC_FAR *__RPC_FAR *ppCallResult
    )
         {return E_NOTIMPL;}

    STDMETHOD(DeleteClassAsync)( 
            /* [in] */ const BSTR Class,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    )
         {return E_NOTIMPL;}

    STDMETHOD(CreateClassEnum)(
            /* [in] */ const BSTR Superclass,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [out] */ IEnumWbemClassObject __RPC_FAR *__RPC_FAR *ppEnum
    )
         {return E_NOTIMPL;}

    STDMETHOD(CreateClassEnumAsync)(
            /* [in] */ const BSTR Superclass,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    )
         {return E_NOTIMPL;}

    STDMETHOD(PutInstance)(
            /* [in] */ IWbemClassObject __RPC_FAR *pInst,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [unique][in][out] */ IWbemCallResult __RPC_FAR *__RPC_FAR *ppCallResult
    )
         {return E_NOTIMPL;}

    STDMETHOD(DeleteInstance)(
            /* [in] */ const BSTR ObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [unique][in][out] */ IWbemCallResult __RPC_FAR *__RPC_FAR *ppCallResult
    )
         {return E_NOTIMPL;}

    STDMETHOD(CancelAsyncRequest)(THIS_ long lAsyncRequestHandle
    )
         {return E_NOTIMPL;}

    STDMETHOD(CancelAsyncCall)(IWbemObjectSink __RPC_FAR *pSink
    )
         {return E_NOTIMPL;}

    STDMETHOD(CreateInstanceEnum)(
            /* [in] */ const BSTR Class,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [out] */ IEnumWbemClassObject __RPC_FAR *__RPC_FAR *ppEnum
    )
         {return E_NOTIMPL;}
  
    STDMETHOD(ExecQuery)(
            /* [in] */ const BSTR QueryLanguage,
            /* [in] */ const BSTR Query,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [out] */ IEnumWbemClassObject __RPC_FAR *__RPC_FAR *ppEnum
    )
         {return E_NOTIMPL;}

    STDMETHOD(QueryObjectSink)(long lFlags, 
                               IWbemObjectSink __RPC_FAR *__RPC_FAR *ppResponseHandler
    )
         {return E_NOTIMPL;}


    STDMETHOD(GetObject)( const BSTR ObjectPath, 
                          long lFlags, 
                          IWbemContext __RPC_FAR *pCtx, 
                          IWbemClassObject __RPC_FAR *__RPC_FAR *ppObject, 
                          IWbemCallResult __RPC_FAR *__RPC_FAR *ppCallResult
    )

         {return E_NOTIMPL;}

    STDMETHOD(ExecNotificationQuery)( 
            /* [in] */ const BSTR QueryLanguage,
            /* [in] */ const BSTR Query,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [out] */ IEnumWbemClassObject __RPC_FAR *__RPC_FAR *ppEnum
    )
       {return E_NOTIMPL;}
        
        
    STDMETHOD(ExecNotificationQueryAsync)( 
            /* [in] */ const BSTR QueryLanguage,
            /* [in] */ const BSTR Query,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext __RPC_FAR *pCtx,
            /* [in] */ IWbemObjectSink __RPC_FAR *pResponseHandler
    )
       {return E_NOTIMPL;}
        
    STDMETHOD(ExecMethod)(const BSTR, 
                          const BSTR, 
                          long, 
                          IWbemContext*, 
                          IWbemClassObject*,
                          IWbemClassObject**, 
                          IWbemCallResult**
    )
       {return E_NOTIMPL;}

    CWbemProviderGlue();
    CWbemProviderGlue(PLONG pCount);
    ~CWbemProviderGlue(); // Destructor

    // used by the provider base class - you probably will never need to call this directly
    static IWbemServices *WINAPI GetNamespaceConnection( LPCWSTR NameSpace );
    static IWbemServices *WINAPI GetNamespaceConnection( LPCWSTR NameSpace, MethodContext *pMethodContext );

    // each provider class must log in and out with these in their constructors and destructors
    // this is done for you in the Provider base class
    static void WINAPI FrameworkLogin( LPCWSTR a_pszName, 
                                       Provider *a_pProvider, 
                                       LPCWSTR a_pszNameSpace );

    static void WINAPI FrameworkLogoff( LPCWSTR a_pszName, 
                                        LPCWSTR a_pszNameSpace );

    static void WINAPI IncrementObjectCount(void);

    static LONG WINAPI DecrementObjectCount(void);

    static DWORD WINAPI GetOSMajorVersion() { return s_dwMajorVersion; }

    static DWORD WINAPI GetPlatform() { return s_dwPlatform; }

    static LPCWSTR WINAPI GetCSDVersion() { return s_wstrCSDVersion; }

 private:

    IWbemServices *WINAPI InternalGetNamespaceConnection( LPCWSTR NameSpace );

    static HRESULT WINAPI GetInstanceFromCIMOM( LPCWSTR pszObjectPath,
                                        LPCWSTR pszNameSpace,
                                        MethodContext *pMethodContext,
                                        CInstance **ppInstance );   

    void FlushAll(void);

    static Provider *WINAPI SearchMapForProvider( LPCWSTR a_pszProviderName, 
                                                  LPCWSTR a_pszNamespace );

    static Provider *WINAPI AddProviderToMap( LPCWSTR a_pszProviderName, 
                                              LPCWSTR a_strNamespace, 
                                              Provider *a_pProvider );

    static void WINAPI LockProviderMap( void );

    static void WINAPI UnlockProviderMap( void );

    static void WINAPI LockFactoryMap( void );

    static void WINAPI UnlockFactoryMap( void );

    static IWbemClassObject *WINAPI GetStatusObject( MethodContext *pContext, 
                                                     LPCWSTR pNamespace);
    static void WINAPI Init( void );

    static void WINAPI UnInit( void );

    static void WINAPI GetComputerName( CHString& strComputerName );
    
    static HRESULT WINAPI CheckImpersonationLevel();

    HRESULT PreProcessPutInstanceParms(IWbemClassObject __RPC_FAR *pInstIn, 
                                       IWbemClassObject __RPC_FAR **pInstOut, 
                                       IWbemContext __RPC_FAR *pCtx);    

    HRESULT NullOutUnsetProperties(IWbemClassObject __RPC_FAR *pInstIn, 
                                   IWbemClassObject __RPC_FAR **pInstOut, 
                                   const VARIANT& vValue);

    void AddFlushPtr(LPVOID pVoid);

    long    m_lRefCount; 
    CHString    m_strNamespace;
    IWbemServices *m_pServices;
    static long s_lObjects; // number of objects out there - class factories & WbemGlues
    PLONG m_pCount;
    static PROVIDERPTRS	m_FlushPtrs;
    static CCritSec		m_csFlushPtrs;

    static STRING2LPVOID    s_providersmap;
    static CCritSec s_csProviderMap;
    static CCritSec m_csStatusObject;
    static BOOL s_bInitted;
    static DWORD s_dwPlatform;
    static DWORD s_dwMajorVersion;
    static WCHAR s_wstrCSDVersion[_MAX_PATH];
    static IWbemClassObject *m_pStatusObject;
    static PTR2PLONG        s_factorymap;
    static CCritSec         s_csFactoryMap;

protected:
    friend CWbemGlueFactory;

    static LONG IncrementMapCount(const CWbemGlueFactory *pGlue);
    static LONG IncrementMapCount(PLONG pCount);
    static LONG DecrementMapCount(const CWbemGlueFactory *pGlue);
    static LONG DecrementMapCount(PLONG pCount);
    static PLONG GetMapCountPtr(const CWbemGlueFactory *pGlue);
    static VOID AddToFactoryMap(const CWbemGlueFactory *pGlue, PLONG pdwRefCount);
    static VOID RemoveFromFactoryMap(const CWbemGlueFactory *pGlue);
};

inline void CWbemProviderGlue::LockProviderMap( void )
{
    EnterCriticalSection( &s_csProviderMap );
}

inline void CWbemProviderGlue::UnlockProviderMap( void )
{
    LeaveCriticalSection( &s_csProviderMap );
}

inline void CWbemProviderGlue::LockFactoryMap( void )
{
    EnterCriticalSection( &s_csFactoryMap );
}

inline void CWbemProviderGlue::UnlockFactoryMap( void )
{
    LeaveCriticalSection( &s_csFactoryMap );
}

class POLARITY CWbemGlueFactory : public IClassFactory
{
 protected:
    long m_lRefCount;

 public:
     // Deprecated in favor of the constructor that takes a PLONG.  This
     // must be the same PLONG that is passed to FrameworkLoginDLL &
     // FrameworkLogoffDLL.
#ifdef FRAMEWORK_ALLOW_DEPRECATED
        CWbemGlueFactory(void);
        static CWbemGlueFactory * Create();
#endif
        CWbemGlueFactory(PLONG plRefCount);
        ~CWbemGlueFactory(void);

        static CWbemGlueFactory * Create(PLONG plRefCount);
        void Destroy();

        //IUnknown members
        STDMETHODIMP         QueryInterface(REFIID, PPVOID);
        STDMETHODIMP_(ULONG) AddRef(void);
        STDMETHODIMP_(ULONG) Release(void);

        //IClassFactory members
        STDMETHODIMP         CreateInstance(LPUNKNOWN, REFIID, PPVOID);
        STDMETHODIMP         LockServer(BOOL);
    };

typedef CWbemGlueFactory *PCWbemGlueFactory;

#ifdef _PREFAST_
#pragma prefast (pop)
#endif /* _PREFAST_ */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion



#endif
