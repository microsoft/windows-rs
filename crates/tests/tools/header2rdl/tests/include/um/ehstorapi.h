

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __EhStorAPI_h__
#define __EhStorAPI_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IEnumEnhancedStorageACT_FWD_DEFINED__
#define __IEnumEnhancedStorageACT_FWD_DEFINED__
typedef interface IEnumEnhancedStorageACT IEnumEnhancedStorageACT;

#endif 	/* __IEnumEnhancedStorageACT_FWD_DEFINED__ */


#ifndef __IEnhancedStorageACT_FWD_DEFINED__
#define __IEnhancedStorageACT_FWD_DEFINED__
typedef interface IEnhancedStorageACT IEnhancedStorageACT;

#endif 	/* __IEnhancedStorageACT_FWD_DEFINED__ */


#ifndef __IEnhancedStorageACT2_FWD_DEFINED__
#define __IEnhancedStorageACT2_FWD_DEFINED__
typedef interface IEnhancedStorageACT2 IEnhancedStorageACT2;

#endif 	/* __IEnhancedStorageACT2_FWD_DEFINED__ */


#ifndef __IEnhancedStorageACT3_FWD_DEFINED__
#define __IEnhancedStorageACT3_FWD_DEFINED__
typedef interface IEnhancedStorageACT3 IEnhancedStorageACT3;

#endif 	/* __IEnhancedStorageACT3_FWD_DEFINED__ */


#ifndef __IEnhancedStorageSilo_FWD_DEFINED__
#define __IEnhancedStorageSilo_FWD_DEFINED__
typedef interface IEnhancedStorageSilo IEnhancedStorageSilo;

#endif 	/* __IEnhancedStorageSilo_FWD_DEFINED__ */


#ifndef __IEnhancedStorageSiloAction_FWD_DEFINED__
#define __IEnhancedStorageSiloAction_FWD_DEFINED__
typedef interface IEnhancedStorageSiloAction IEnhancedStorageSiloAction;

#endif 	/* __IEnhancedStorageSiloAction_FWD_DEFINED__ */


#ifndef __EnumEnhancedStorageACT_FWD_DEFINED__
#define __EnumEnhancedStorageACT_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnumEnhancedStorageACT EnumEnhancedStorageACT;
#else
typedef struct EnumEnhancedStorageACT EnumEnhancedStorageACT;
#endif /* __cplusplus */

#endif 	/* __EnumEnhancedStorageACT_FWD_DEFINED__ */


#ifndef __EnhancedStorageACT_FWD_DEFINED__
#define __EnhancedStorageACT_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnhancedStorageACT EnhancedStorageACT;
#else
typedef struct EnhancedStorageACT EnhancedStorageACT;
#endif /* __cplusplus */

#endif 	/* __EnhancedStorageACT_FWD_DEFINED__ */


#ifndef __EnhancedStorageSilo_FWD_DEFINED__
#define __EnhancedStorageSilo_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnhancedStorageSilo EnhancedStorageSilo;
#else
typedef struct EnhancedStorageSilo EnhancedStorageSilo;
#endif /* __cplusplus */

#endif 	/* __EnhancedStorageSilo_FWD_DEFINED__ */


#ifndef __EnhancedStorageSiloAction_FWD_DEFINED__
#define __EnhancedStorageSiloAction_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnhancedStorageSiloAction EnhancedStorageSiloAction;
#else
typedef struct EnhancedStorageSiloAction EnhancedStorageSiloAction;
#endif /* __cplusplus */

#endif 	/* __EnhancedStorageSiloAction_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "ocidl.h"
#include "propidl.h"
#include "portabledeviceapi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_EhStorAPI_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (_WIN32_WINNT >= 0x0501) // XP and later






typedef struct _ACT_AUTHORIZATION_STATE
    {
    ULONG ulState;
    } 	ACT_AUTHORIZATION_STATE;

typedef struct _SILO_INFO
    {
    ULONG ulSTID;
    UCHAR SpecificationMajor;
    UCHAR SpecificationMinor;
    UCHAR ImplementationMajor;
    UCHAR ImplementationMinor;
    UCHAR type;
    UCHAR capabilities;
    } 	SILO_INFO;

typedef /* [public][helpstring][v1_enum] */ 
enum _ACT_AUTHORIZATION_STATE_VALUE
    {
        ACT_UNAUTHORIZED	= 0,
        ACT_AUTHORIZED	= 0x1
    } 	ACT_AUTHORIZATION_STATE_VALUE;

typedef /* [public][helpstring][v1_enum] */ enum _ACT_AUTHORIZATION_STATE_VALUE *PACT_AUTHORIZATION_STATE_VALUE;



extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumEnhancedStorageACT_INTERFACE_DEFINED__
#define __IEnumEnhancedStorageACT_INTERFACE_DEFINED__

/* interface IEnumEnhancedStorageACT */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnumEnhancedStorageACT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09b224bd-1335-4631-a7ff-cfd3a92646d7")
    IEnumEnhancedStorageACT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetACTs( 
            /* [size_is][size_is][out] */ IEnhancedStorageACT ***pppIEnhancedStorageACTs,
            /* [out] */ ULONG *pcEnhancedStorageACTs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMatchingACT( 
            /* [in] */ LPCWSTR szVolume,
            /* [out] */ IEnhancedStorageACT **ppIEnhancedStorageACT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumEnhancedStorageACTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumEnhancedStorageACT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumEnhancedStorageACT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumEnhancedStorageACT * This);
        
        DECLSPEC_XFGVIRT(IEnumEnhancedStorageACT, GetACTs)
        HRESULT ( STDMETHODCALLTYPE *GetACTs )( 
            IEnumEnhancedStorageACT * This,
            /* [size_is][size_is][out] */ IEnhancedStorageACT ***pppIEnhancedStorageACTs,
            /* [out] */ ULONG *pcEnhancedStorageACTs);
        
        DECLSPEC_XFGVIRT(IEnumEnhancedStorageACT, GetMatchingACT)
        HRESULT ( STDMETHODCALLTYPE *GetMatchingACT )( 
            IEnumEnhancedStorageACT * This,
            /* [in] */ LPCWSTR szVolume,
            /* [out] */ IEnhancedStorageACT **ppIEnhancedStorageACT);
        
        END_INTERFACE
    } IEnumEnhancedStorageACTVtbl;

    interface IEnumEnhancedStorageACT
    {
        CONST_VTBL struct IEnumEnhancedStorageACTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumEnhancedStorageACT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumEnhancedStorageACT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumEnhancedStorageACT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumEnhancedStorageACT_GetACTs(This,pppIEnhancedStorageACTs,pcEnhancedStorageACTs)	\
    ( (This)->lpVtbl -> GetACTs(This,pppIEnhancedStorageACTs,pcEnhancedStorageACTs) ) 

#define IEnumEnhancedStorageACT_GetMatchingACT(This,szVolume,ppIEnhancedStorageACT)	\
    ( (This)->lpVtbl -> GetMatchingACT(This,szVolume,ppIEnhancedStorageACT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumEnhancedStorageACT_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_EhStorAPI_0000_0001 */
/* [local] */ 

#define ACT_AUTHORIZE_ON_RESUME               0x00000001
#define ACT_AUTHORIZE_ON_SESSION_UNLOCK       0x00000002


extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0001_v0_0_s_ifspec;

#ifndef __IEnhancedStorageACT_INTERFACE_DEFINED__
#define __IEnhancedStorageACT_INTERFACE_DEFINED__

/* interface IEnhancedStorageACT */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnhancedStorageACT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6e7781f4-e0f2-4239-b976-a01abab52930")
    IEnhancedStorageACT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Authorize( 
            /* [in] */ DWORD hwndParent,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unauthorize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAuthorizationState( 
            /* [out] */ ACT_AUTHORIZATION_STATE *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMatchingVolume( 
            /* [out] */ LPWSTR *ppwszVolume) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUniqueIdentity( 
            /* [out] */ LPWSTR *ppwszIdentity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSilos( 
            /* [size_is][size_is][size_is][out] */ IEnhancedStorageSilo ***pppIEnhancedStorageSilos,
            /* [out] */ ULONG *pcEnhancedStorageSilos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnhancedStorageACTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnhancedStorageACT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnhancedStorageACT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnhancedStorageACT * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Authorize)
        HRESULT ( STDMETHODCALLTYPE *Authorize )( 
            IEnhancedStorageACT * This,
            /* [in] */ DWORD hwndParent,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Unauthorize)
        HRESULT ( STDMETHODCALLTYPE *Unauthorize )( 
            IEnhancedStorageACT * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetAuthorizationState)
        HRESULT ( STDMETHODCALLTYPE *GetAuthorizationState )( 
            IEnhancedStorageACT * This,
            /* [out] */ ACT_AUTHORIZATION_STATE *pState);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetMatchingVolume)
        HRESULT ( STDMETHODCALLTYPE *GetMatchingVolume )( 
            IEnhancedStorageACT * This,
            /* [out] */ LPWSTR *ppwszVolume);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetUniqueIdentity)
        HRESULT ( STDMETHODCALLTYPE *GetUniqueIdentity )( 
            IEnhancedStorageACT * This,
            /* [out] */ LPWSTR *ppwszIdentity);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetSilos)
        HRESULT ( STDMETHODCALLTYPE *GetSilos )( 
            IEnhancedStorageACT * This,
            /* [size_is][size_is][size_is][out] */ IEnhancedStorageSilo ***pppIEnhancedStorageSilos,
            /* [out] */ ULONG *pcEnhancedStorageSilos);
        
        END_INTERFACE
    } IEnhancedStorageACTVtbl;

    interface IEnhancedStorageACT
    {
        CONST_VTBL struct IEnhancedStorageACTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnhancedStorageACT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnhancedStorageACT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnhancedStorageACT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnhancedStorageACT_Authorize(This,hwndParent,dwFlags)	\
    ( (This)->lpVtbl -> Authorize(This,hwndParent,dwFlags) ) 

#define IEnhancedStorageACT_Unauthorize(This)	\
    ( (This)->lpVtbl -> Unauthorize(This) ) 

#define IEnhancedStorageACT_GetAuthorizationState(This,pState)	\
    ( (This)->lpVtbl -> GetAuthorizationState(This,pState) ) 

#define IEnhancedStorageACT_GetMatchingVolume(This,ppwszVolume)	\
    ( (This)->lpVtbl -> GetMatchingVolume(This,ppwszVolume) ) 

#define IEnhancedStorageACT_GetUniqueIdentity(This,ppwszIdentity)	\
    ( (This)->lpVtbl -> GetUniqueIdentity(This,ppwszIdentity) ) 

#define IEnhancedStorageACT_GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos)	\
    ( (This)->lpVtbl -> GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnhancedStorageACT_INTERFACE_DEFINED__ */


#ifndef __IEnhancedStorageACT2_INTERFACE_DEFINED__
#define __IEnhancedStorageACT2_INTERFACE_DEFINED__

/* interface IEnhancedStorageACT2 */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnhancedStorageACT2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4DA57D2E-8EB3-41f6-A07E-98B52B88242B")
    IEnhancedStorageACT2 : public IEnhancedStorageACT
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceName( 
            /* [out] */ LPWSTR *ppwszDeviceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDeviceRemovable( 
            /* [out] */ BOOL *pIsDeviceRemovable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnhancedStorageACT2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnhancedStorageACT2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnhancedStorageACT2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnhancedStorageACT2 * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Authorize)
        HRESULT ( STDMETHODCALLTYPE *Authorize )( 
            IEnhancedStorageACT2 * This,
            /* [in] */ DWORD hwndParent,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Unauthorize)
        HRESULT ( STDMETHODCALLTYPE *Unauthorize )( 
            IEnhancedStorageACT2 * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetAuthorizationState)
        HRESULT ( STDMETHODCALLTYPE *GetAuthorizationState )( 
            IEnhancedStorageACT2 * This,
            /* [out] */ ACT_AUTHORIZATION_STATE *pState);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetMatchingVolume)
        HRESULT ( STDMETHODCALLTYPE *GetMatchingVolume )( 
            IEnhancedStorageACT2 * This,
            /* [out] */ LPWSTR *ppwszVolume);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetUniqueIdentity)
        HRESULT ( STDMETHODCALLTYPE *GetUniqueIdentity )( 
            IEnhancedStorageACT2 * This,
            /* [out] */ LPWSTR *ppwszIdentity);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetSilos)
        HRESULT ( STDMETHODCALLTYPE *GetSilos )( 
            IEnhancedStorageACT2 * This,
            /* [size_is][size_is][size_is][out] */ IEnhancedStorageSilo ***pppIEnhancedStorageSilos,
            /* [out] */ ULONG *pcEnhancedStorageSilos);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT2, GetDeviceName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceName )( 
            IEnhancedStorageACT2 * This,
            /* [out] */ LPWSTR *ppwszDeviceName);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT2, IsDeviceRemovable)
        HRESULT ( STDMETHODCALLTYPE *IsDeviceRemovable )( 
            IEnhancedStorageACT2 * This,
            /* [out] */ BOOL *pIsDeviceRemovable);
        
        END_INTERFACE
    } IEnhancedStorageACT2Vtbl;

    interface IEnhancedStorageACT2
    {
        CONST_VTBL struct IEnhancedStorageACT2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnhancedStorageACT2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnhancedStorageACT2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnhancedStorageACT2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnhancedStorageACT2_Authorize(This,hwndParent,dwFlags)	\
    ( (This)->lpVtbl -> Authorize(This,hwndParent,dwFlags) ) 

#define IEnhancedStorageACT2_Unauthorize(This)	\
    ( (This)->lpVtbl -> Unauthorize(This) ) 

#define IEnhancedStorageACT2_GetAuthorizationState(This,pState)	\
    ( (This)->lpVtbl -> GetAuthorizationState(This,pState) ) 

#define IEnhancedStorageACT2_GetMatchingVolume(This,ppwszVolume)	\
    ( (This)->lpVtbl -> GetMatchingVolume(This,ppwszVolume) ) 

#define IEnhancedStorageACT2_GetUniqueIdentity(This,ppwszIdentity)	\
    ( (This)->lpVtbl -> GetUniqueIdentity(This,ppwszIdentity) ) 

#define IEnhancedStorageACT2_GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos)	\
    ( (This)->lpVtbl -> GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos) ) 


#define IEnhancedStorageACT2_GetDeviceName(This,ppwszDeviceName)	\
    ( (This)->lpVtbl -> GetDeviceName(This,ppwszDeviceName) ) 

#define IEnhancedStorageACT2_IsDeviceRemovable(This,pIsDeviceRemovable)	\
    ( (This)->lpVtbl -> IsDeviceRemovable(This,pIsDeviceRemovable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnhancedStorageACT2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_EhStorAPI_0000_0003 */
/* [local] */ 

#define ACT_UNAUTHORIZE_ON_SUSPEND            0x00000001
#define ACT_UNAUTHORIZE_ON_SESSION_LOCK       0x00000002


extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0003_v0_0_s_ifspec;

#ifndef __IEnhancedStorageACT3_INTERFACE_DEFINED__
#define __IEnhancedStorageACT3_INTERFACE_DEFINED__

/* interface IEnhancedStorageACT3 */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnhancedStorageACT3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("022150A1-113D-11DF-BB61-001AA01BBC58")
    IEnhancedStorageACT3 : public IEnhancedStorageACT2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UnauthorizeEx( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsQueueFrozen( 
            /* [out] */ BOOL *pIsQueueFrozen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetShellExtSupport( 
            /* [out] */ BOOL *pShellExtSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnhancedStorageACT3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnhancedStorageACT3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnhancedStorageACT3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnhancedStorageACT3 * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Authorize)
        HRESULT ( STDMETHODCALLTYPE *Authorize )( 
            IEnhancedStorageACT3 * This,
            /* [in] */ DWORD hwndParent,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, Unauthorize)
        HRESULT ( STDMETHODCALLTYPE *Unauthorize )( 
            IEnhancedStorageACT3 * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetAuthorizationState)
        HRESULT ( STDMETHODCALLTYPE *GetAuthorizationState )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ ACT_AUTHORIZATION_STATE *pState);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetMatchingVolume)
        HRESULT ( STDMETHODCALLTYPE *GetMatchingVolume )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ LPWSTR *ppwszVolume);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetUniqueIdentity)
        HRESULT ( STDMETHODCALLTYPE *GetUniqueIdentity )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ LPWSTR *ppwszIdentity);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT, GetSilos)
        HRESULT ( STDMETHODCALLTYPE *GetSilos )( 
            IEnhancedStorageACT3 * This,
            /* [size_is][size_is][size_is][out] */ IEnhancedStorageSilo ***pppIEnhancedStorageSilos,
            /* [out] */ ULONG *pcEnhancedStorageSilos);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT2, GetDeviceName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceName )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ LPWSTR *ppwszDeviceName);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT2, IsDeviceRemovable)
        HRESULT ( STDMETHODCALLTYPE *IsDeviceRemovable )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ BOOL *pIsDeviceRemovable);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT3, UnauthorizeEx)
        HRESULT ( STDMETHODCALLTYPE *UnauthorizeEx )( 
            IEnhancedStorageACT3 * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT3, IsQueueFrozen)
        HRESULT ( STDMETHODCALLTYPE *IsQueueFrozen )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ BOOL *pIsQueueFrozen);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageACT3, GetShellExtSupport)
        HRESULT ( STDMETHODCALLTYPE *GetShellExtSupport )( 
            IEnhancedStorageACT3 * This,
            /* [out] */ BOOL *pShellExtSupport);
        
        END_INTERFACE
    } IEnhancedStorageACT3Vtbl;

    interface IEnhancedStorageACT3
    {
        CONST_VTBL struct IEnhancedStorageACT3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnhancedStorageACT3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnhancedStorageACT3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnhancedStorageACT3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnhancedStorageACT3_Authorize(This,hwndParent,dwFlags)	\
    ( (This)->lpVtbl -> Authorize(This,hwndParent,dwFlags) ) 

#define IEnhancedStorageACT3_Unauthorize(This)	\
    ( (This)->lpVtbl -> Unauthorize(This) ) 

#define IEnhancedStorageACT3_GetAuthorizationState(This,pState)	\
    ( (This)->lpVtbl -> GetAuthorizationState(This,pState) ) 

#define IEnhancedStorageACT3_GetMatchingVolume(This,ppwszVolume)	\
    ( (This)->lpVtbl -> GetMatchingVolume(This,ppwszVolume) ) 

#define IEnhancedStorageACT3_GetUniqueIdentity(This,ppwszIdentity)	\
    ( (This)->lpVtbl -> GetUniqueIdentity(This,ppwszIdentity) ) 

#define IEnhancedStorageACT3_GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos)	\
    ( (This)->lpVtbl -> GetSilos(This,pppIEnhancedStorageSilos,pcEnhancedStorageSilos) ) 


#define IEnhancedStorageACT3_GetDeviceName(This,ppwszDeviceName)	\
    ( (This)->lpVtbl -> GetDeviceName(This,ppwszDeviceName) ) 

#define IEnhancedStorageACT3_IsDeviceRemovable(This,pIsDeviceRemovable)	\
    ( (This)->lpVtbl -> IsDeviceRemovable(This,pIsDeviceRemovable) ) 


#define IEnhancedStorageACT3_UnauthorizeEx(This,dwFlags)	\
    ( (This)->lpVtbl -> UnauthorizeEx(This,dwFlags) ) 

#define IEnhancedStorageACT3_IsQueueFrozen(This,pIsQueueFrozen)	\
    ( (This)->lpVtbl -> IsQueueFrozen(This,pIsQueueFrozen) ) 

#define IEnhancedStorageACT3_GetShellExtSupport(This,pShellExtSupport)	\
    ( (This)->lpVtbl -> GetShellExtSupport(This,pShellExtSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnhancedStorageACT3_INTERFACE_DEFINED__ */


#ifndef __IEnhancedStorageSilo_INTERFACE_DEFINED__
#define __IEnhancedStorageSilo_INTERFACE_DEFINED__

/* interface IEnhancedStorageSilo */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnhancedStorageSilo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5aef78c6-2242-4703-bf49-44b29357a359")
    IEnhancedStorageSilo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [out] */ SILO_INFO *pSiloInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActions( 
            /* [size_is][size_is][out] */ IEnhancedStorageSiloAction ***pppIEnhancedStorageSiloActions,
            /* [out] */ ULONG *pcEnhancedStorageSiloActions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendCommand( 
            /* [in] */ UCHAR Command,
            /* [size_is][in] */ BYTE *pbCommandBuffer,
            /* [in] */ ULONG cbCommandBuffer,
            /* [size_is][out] */ BYTE *pbResponseBuffer,
            /* [out][in] */ ULONG *pcbResponseBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPortableDevice( 
            /* [out] */ IPortableDevice **ppIPortableDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevicePath( 
            /* [out] */ LPWSTR *ppwszSiloDevicePath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnhancedStorageSiloVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnhancedStorageSilo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnhancedStorageSilo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnhancedStorageSilo * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSilo, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IEnhancedStorageSilo * This,
            /* [out] */ SILO_INFO *pSiloInfo);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSilo, GetActions)
        HRESULT ( STDMETHODCALLTYPE *GetActions )( 
            IEnhancedStorageSilo * This,
            /* [size_is][size_is][out] */ IEnhancedStorageSiloAction ***pppIEnhancedStorageSiloActions,
            /* [out] */ ULONG *pcEnhancedStorageSiloActions);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSilo, SendCommand)
        HRESULT ( STDMETHODCALLTYPE *SendCommand )( 
            IEnhancedStorageSilo * This,
            /* [in] */ UCHAR Command,
            /* [size_is][in] */ BYTE *pbCommandBuffer,
            /* [in] */ ULONG cbCommandBuffer,
            /* [size_is][out] */ BYTE *pbResponseBuffer,
            /* [out][in] */ ULONG *pcbResponseBuffer);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSilo, GetPortableDevice)
        HRESULT ( STDMETHODCALLTYPE *GetPortableDevice )( 
            IEnhancedStorageSilo * This,
            /* [out] */ IPortableDevice **ppIPortableDevice);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSilo, GetDevicePath)
        HRESULT ( STDMETHODCALLTYPE *GetDevicePath )( 
            IEnhancedStorageSilo * This,
            /* [out] */ LPWSTR *ppwszSiloDevicePath);
        
        END_INTERFACE
    } IEnhancedStorageSiloVtbl;

    interface IEnhancedStorageSilo
    {
        CONST_VTBL struct IEnhancedStorageSiloVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnhancedStorageSilo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnhancedStorageSilo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnhancedStorageSilo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnhancedStorageSilo_GetInfo(This,pSiloInfo)	\
    ( (This)->lpVtbl -> GetInfo(This,pSiloInfo) ) 

#define IEnhancedStorageSilo_GetActions(This,pppIEnhancedStorageSiloActions,pcEnhancedStorageSiloActions)	\
    ( (This)->lpVtbl -> GetActions(This,pppIEnhancedStorageSiloActions,pcEnhancedStorageSiloActions) ) 

#define IEnhancedStorageSilo_SendCommand(This,Command,pbCommandBuffer,cbCommandBuffer,pbResponseBuffer,pcbResponseBuffer)	\
    ( (This)->lpVtbl -> SendCommand(This,Command,pbCommandBuffer,cbCommandBuffer,pbResponseBuffer,pcbResponseBuffer) ) 

#define IEnhancedStorageSilo_GetPortableDevice(This,ppIPortableDevice)	\
    ( (This)->lpVtbl -> GetPortableDevice(This,ppIPortableDevice) ) 

#define IEnhancedStorageSilo_GetDevicePath(This,ppwszSiloDevicePath)	\
    ( (This)->lpVtbl -> GetDevicePath(This,ppwszSiloDevicePath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnhancedStorageSilo_INTERFACE_DEFINED__ */


#ifndef __IEnhancedStorageSiloAction_INTERFACE_DEFINED__
#define __IEnhancedStorageSiloAction_INTERFACE_DEFINED__

/* interface IEnhancedStorageSiloAction */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnhancedStorageSiloAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b6f7f311-206f-4ff8-9c4b-27efee77a86f")
    IEnhancedStorageSiloAction : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [out] */ LPWSTR *ppwszActionName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ LPWSTR *ppwszActionDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Invoke( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnhancedStorageSiloActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnhancedStorageSiloAction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnhancedStorageSiloAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnhancedStorageSiloAction * This);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSiloAction, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IEnhancedStorageSiloAction * This,
            /* [out] */ LPWSTR *ppwszActionName);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSiloAction, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            IEnhancedStorageSiloAction * This,
            /* [out] */ LPWSTR *ppwszActionDescription);
        
        DECLSPEC_XFGVIRT(IEnhancedStorageSiloAction, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEnhancedStorageSiloAction * This);
        
        END_INTERFACE
    } IEnhancedStorageSiloActionVtbl;

    interface IEnhancedStorageSiloAction
    {
        CONST_VTBL struct IEnhancedStorageSiloActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnhancedStorageSiloAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnhancedStorageSiloAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnhancedStorageSiloAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnhancedStorageSiloAction_GetName(This,ppwszActionName)	\
    ( (This)->lpVtbl -> GetName(This,ppwszActionName) ) 

#define IEnhancedStorageSiloAction_GetDescription(This,ppwszActionDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,ppwszActionDescription) ) 

#define IEnhancedStorageSiloAction_Invoke(This)	\
    ( (This)->lpVtbl -> Invoke(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnhancedStorageSiloAction_INTERFACE_DEFINED__ */



#ifndef __EnhancedStorageAPILib_LIBRARY_DEFINED__
#define __EnhancedStorageAPILib_LIBRARY_DEFINED__

/* library EnhancedStorageAPILib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_EnhancedStorageAPILib;

EXTERN_C const CLSID CLSID_EnumEnhancedStorageACT;

#ifdef __cplusplus

class DECLSPEC_UUID("fe841493-835c-4fa3-b6cc-b4b2d4719848")
EnumEnhancedStorageACT;
#endif

EXTERN_C const CLSID CLSID_EnhancedStorageACT;

#ifdef __cplusplus

class DECLSPEC_UUID("af076a15-2ece-4ad4-bb21-29f040e176d8")
EnhancedStorageACT;
#endif

EXTERN_C const CLSID CLSID_EnhancedStorageSilo;

#ifdef __cplusplus

class DECLSPEC_UUID("cb25220c-76c7-4fee-842b-f3383cd022bc")
EnhancedStorageSilo;
#endif

EXTERN_C const CLSID CLSID_EnhancedStorageSiloAction;

#ifdef __cplusplus

class DECLSPEC_UUID("886D29DD-B506-466B-9FBF-B44FF383FB3F")
EnhancedStorageSiloAction;
#endif
#endif /* __EnhancedStorageAPILib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_EhStorAPI_0000_0007 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_EhStorAPI_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


