

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

#ifndef __amsi_h__
#define __amsi_h__

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

#ifndef __IAmsiStream_FWD_DEFINED__
#define __IAmsiStream_FWD_DEFINED__
typedef interface IAmsiStream IAmsiStream;

#endif 	/* __IAmsiStream_FWD_DEFINED__ */


#ifndef __IAntimalwareProvider_FWD_DEFINED__
#define __IAntimalwareProvider_FWD_DEFINED__
typedef interface IAntimalwareProvider IAntimalwareProvider;

#endif 	/* __IAntimalwareProvider_FWD_DEFINED__ */


#ifndef __IAntimalwareUacProvider_FWD_DEFINED__
#define __IAntimalwareUacProvider_FWD_DEFINED__
typedef interface IAntimalwareUacProvider IAntimalwareUacProvider;

#endif 	/* __IAntimalwareUacProvider_FWD_DEFINED__ */


#ifndef __IAntimalwareProvider2_FWD_DEFINED__
#define __IAntimalwareProvider2_FWD_DEFINED__
typedef interface IAntimalwareProvider2 IAntimalwareProvider2;

#endif 	/* __IAntimalwareProvider2_FWD_DEFINED__ */


#ifndef __IAntimalware_FWD_DEFINED__
#define __IAntimalware_FWD_DEFINED__
typedef interface IAntimalware IAntimalware;

#endif 	/* __IAntimalware_FWD_DEFINED__ */


#ifndef __IAntimalware2_FWD_DEFINED__
#define __IAntimalware2_FWD_DEFINED__
typedef interface IAntimalware2 IAntimalware2;

#endif 	/* __IAntimalware2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_amsi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum AMSI_RESULT
    {
        AMSI_RESULT_CLEAN	= 0,
        AMSI_RESULT_NOT_DETECTED	= 1,
        AMSI_RESULT_BLOCKED_BY_ADMIN_START	= 0x4000,
        AMSI_RESULT_BLOCKED_BY_ADMIN_END	= 0x4fff,
        AMSI_RESULT_DETECTED	= 32768
    } 	AMSI_RESULT;

#define AmsiResultIsMalware(r) ((r) >= AMSI_RESULT_DETECTED)
#define AmsiResultIsBlockedByAdmin(r) ((r) >= AMSI_RESULT_BLOCKED_BY_ADMIN_START) && (r) <= AMSI_RESULT_BLOCKED_BY_ADMIN_END
typedef /* [v1_enum] */ 
enum AMSI_ATTRIBUTE
    {
        AMSI_ATTRIBUTE_APP_NAME	= 0,
        AMSI_ATTRIBUTE_CONTENT_NAME	= 1,
        AMSI_ATTRIBUTE_CONTENT_SIZE	= 2,
        AMSI_ATTRIBUTE_CONTENT_ADDRESS	= 3,
        AMSI_ATTRIBUTE_SESSION	= 4,
        AMSI_ATTRIBUTE_REDIRECT_CHAIN_SIZE	= 5,
        AMSI_ATTRIBUTE_REDIRECT_CHAIN_ADDRESS	= 6,
        AMSI_ATTRIBUTE_ALL_SIZE	= 7,
        AMSI_ATTRIBUTE_ALL_ADDRESS	= 8,
        AMSI_ATTRIBUTE_QUIET	= 9
    } 	AMSI_ATTRIBUTE;

typedef /* [v1_enum] */ 
enum AMSI_UAC_REQUEST_TYPE
    {
        AMSI_UAC_REQUEST_TYPE_EXE	= 0,
        AMSI_UAC_REQUEST_TYPE_COM	= 1,
        AMSI_UAC_REQUEST_TYPE_MSI	= 2,
        AMSI_UAC_REQUEST_TYPE_AX	= 3,
        AMSI_UAC_REQUEST_TYPE_PACKAGED_APP	= 4,
        AMSI_UAC_REQUEST_TYPE_MAX	= 5
    } 	AMSI_UAC_REQUEST_TYPE;

typedef /* [v1_enum] */ 
enum AMSI_UAC_TRUST_STATE
    {
        AMSI_UAC_TRUST_STATE_TRUSTED	= 0,
        AMSI_UAC_TRUST_STATE_UNTRUSTED	= 1,
        AMSI_UAC_TRUST_STATE_BLOCKED	= 2,
        AMSI_UAC_TRUST_STATE_MAX	= 3
    } 	AMSI_UAC_TRUST_STATE;

typedef /* [v1_enum] */ 
enum AMSI_UAC_MSI_ACTION
    {
        AMSI_UAC_MSI_ACTION_INSTALL	= 0,
        AMSI_UAC_MSI_ACTION_UNINSTALL	= 1,
        AMSI_UAC_MSI_ACTION_UPDATE	= 2,
        AMSI_UAC_MSI_ACTION_MAINTENANCE	= 3,
        AMSI_UAC_MSI_ACTION_MAX	= 4
    } 	AMSI_UAC_MSI_ACTION;

typedef struct AMSI_UAC_REQUEST_EXE_INFO
    {
    ULONG ulLength;
    /* [string] */ LPWSTR lpwszApplicationName;
    /* [string] */ LPWSTR lpwszCommandLine;
    /* [string] */ LPWSTR lpwszDLLParameter;
    } 	AMSI_UAC_REQUEST_EXE_INFO;

typedef struct AMSI_UAC_REQUEST_EXE_INFO *LPAMSI_UAC_REQUEST_EXE_INFO;

typedef struct AMSI_UAC_REQUEST_COM_INFO
    {
    ULONG ulLength;
    /* [string] */ LPWSTR lpwszServerBinary;
    /* [string] */ LPWSTR lpwszRequestor;
    GUID Clsid;
    } 	AMSI_UAC_REQUEST_COM_INFO;

typedef struct AMSI_UAC_REQUEST_COM_INFO *LPAMSI_UAC_REQUEST_COM_INFO;

typedef struct AMSI_UAC_REQUEST_MSI_INFO
    {
    ULONG ulLength;
    AMSI_UAC_MSI_ACTION MsiAction;
    /* [string] */ LPWSTR lpwszProductName;
    /* [string] */ LPWSTR lpwszVersion;
    /* [string] */ LPWSTR lpwszLanguage;
    /* [string] */ LPWSTR lpwszManufacturer;
    /* [string] */ LPWSTR lpwszPackagePath;
    /* [string] */ LPWSTR lpwszPackageSource;
    ULONG ulUpdates;
    /* [size_is][size_is][string] */ LPWSTR *ppwszUpdates;
    /* [size_is][size_is][string] */ LPWSTR *ppwszUpdateSources;
    } 	AMSI_UAC_REQUEST_MSI_INFO;

typedef struct AMSI_UAC_REQUEST_MSI_INFO *LPAMSI_UAC_REQUEST_MSI_INFO;

typedef struct AMSI_UAC_REQUEST_AX_INFO
    {
    ULONG ulLength;
    /* [string] */ LPWSTR lpwszLocalInstallPath;
    /* [string] */ LPWSTR lpwszSourceURL;
    } 	AMSI_UAC_REQUEST_AX_INFO;

typedef struct AMSI_UAC_REQUEST_AX_INFO *LPAMSI_UAC_REQUEST_AX_INFO;

typedef struct AMSI_UAC_REQUEST_PACKAGED_APP_INFO
    {
    ULONG ulLength;
    /* [string] */ LPWSTR lpwszApplicationName;
    /* [string] */ LPWSTR lpwszCommandLine;
    /* [string] */ LPWSTR lpPackageFamilyName;
    /* [string] */ LPWSTR lpApplicationId;
    } 	AMSI_UAC_REQUEST_PACKAGED_APP_INFO;

typedef struct AMSI_UAC_REQUEST_PACKAGED_APP_INFO *LPAMSI_UAC_REQUEST_PACKAGED_APP_INFO;

typedef struct AMSI_UAC_REQUEST_CONTEXT
    {
    ULONG ulLength;
    ULONG ulRequestorProcessId;
    AMSI_UAC_TRUST_STATE UACTrustState;
    AMSI_UAC_REQUEST_TYPE Type;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ AMSI_UAC_REQUEST_EXE_INFO ExeInfo;
        /* [case()] */ AMSI_UAC_REQUEST_COM_INFO ComInfo;
        /* [case()] */ AMSI_UAC_REQUEST_MSI_INFO MsiInfo;
        /* [case()] */ AMSI_UAC_REQUEST_AX_INFO ActiveXInfo;
        /* [case()] */ AMSI_UAC_REQUEST_PACKAGED_APP_INFO PackagedAppInfo;
        } 	RequestType;
    BOOL bAutoElevateRequest;
    } 	AMSI_UAC_REQUEST_CONTEXT;

typedef struct AMSI_UAC_REQUEST_CONTEXT *LPAMSI_UAC_REQUEST_CONTEXT;



extern RPC_IF_HANDLE __MIDL_itf_amsi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_amsi_0000_0000_v0_0_s_ifspec;

#ifndef __IAmsiStream_INTERFACE_DEFINED__
#define __IAmsiStream_INTERFACE_DEFINED__

/* interface IAmsiStream */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAmsiStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e47f2e5-81d4-4d3b-897f-545096770373")
    IAmsiStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [in] */ AMSI_ATTRIBUTE attribute,
            /* [range][in] */ ULONG dataSize,
            /* [length_is][size_is][out] */ unsigned char *data,
            /* [out] */ ULONG *retData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [in] */ ULONGLONG position,
            /* [range][in] */ ULONG size,
            /* [length_is][size_is][out] */ unsigned char *buffer,
            /* [out] */ ULONG *readSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAmsiStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAmsiStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAmsiStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAmsiStream * This);
        
        DECLSPEC_XFGVIRT(IAmsiStream, GetAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            IAmsiStream * This,
            /* [in] */ AMSI_ATTRIBUTE attribute,
            /* [range][in] */ ULONG dataSize,
            /* [length_is][size_is][out] */ unsigned char *data,
            /* [out] */ ULONG *retData);
        
        DECLSPEC_XFGVIRT(IAmsiStream, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            IAmsiStream * This,
            /* [in] */ ULONGLONG position,
            /* [range][in] */ ULONG size,
            /* [length_is][size_is][out] */ unsigned char *buffer,
            /* [out] */ ULONG *readSize);
        
        END_INTERFACE
    } IAmsiStreamVtbl;

    interface IAmsiStream
    {
        CONST_VTBL struct IAmsiStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAmsiStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAmsiStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAmsiStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAmsiStream_GetAttribute(This,attribute,dataSize,data,retData)	\
    ( (This)->lpVtbl -> GetAttribute(This,attribute,dataSize,data,retData) ) 

#define IAmsiStream_Read(This,position,size,buffer,readSize)	\
    ( (This)->lpVtbl -> Read(This,position,size,buffer,readSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAmsiStream_INTERFACE_DEFINED__ */


#ifndef __IAntimalwareProvider_INTERFACE_DEFINED__
#define __IAntimalwareProvider_INTERFACE_DEFINED__

/* interface IAntimalwareProvider */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAntimalwareProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b2cabfe3-fe04-42b1-a5df-08d483d4d125")
    IAntimalwareProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Scan( 
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result) = 0;
        
        virtual void STDMETHODCALLTYPE CloseSession( 
            /* [in] */ ULONGLONG session) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayName( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *displayName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAntimalwareProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAntimalwareProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAntimalwareProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAntimalwareProvider * This);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, Scan)
        HRESULT ( STDMETHODCALLTYPE *Scan )( 
            IAntimalwareProvider * This,
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, CloseSession)
        void ( STDMETHODCALLTYPE *CloseSession )( 
            IAntimalwareProvider * This,
            /* [in] */ ULONGLONG session);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, DisplayName)
        HRESULT ( STDMETHODCALLTYPE *DisplayName )( 
            IAntimalwareProvider * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *displayName);
        
        END_INTERFACE
    } IAntimalwareProviderVtbl;

    interface IAntimalwareProvider
    {
        CONST_VTBL struct IAntimalwareProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAntimalwareProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAntimalwareProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAntimalwareProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAntimalwareProvider_Scan(This,stream,result)	\
    ( (This)->lpVtbl -> Scan(This,stream,result) ) 

#define IAntimalwareProvider_CloseSession(This,session)	\
    ( (This)->lpVtbl -> CloseSession(This,session) ) 

#define IAntimalwareProvider_DisplayName(This,displayName)	\
    ( (This)->lpVtbl -> DisplayName(This,displayName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAntimalwareProvider_INTERFACE_DEFINED__ */


#ifndef __IAntimalwareUacProvider_INTERFACE_DEFINED__
#define __IAntimalwareUacProvider_INTERFACE_DEFINED__

/* interface IAntimalwareUacProvider */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IAntimalwareUacProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b2cabfe4-fe04-42b1-a5df-08d483d4d125")
    IAntimalwareUacProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UacScan( 
            /* [in] */ __RPC__in LPAMSI_UAC_REQUEST_CONTEXT context,
            /* [out] */ __RPC__out AMSI_RESULT *result) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayName( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *displayName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAntimalwareUacProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAntimalwareUacProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAntimalwareUacProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAntimalwareUacProvider * This);
        
        DECLSPEC_XFGVIRT(IAntimalwareUacProvider, UacScan)
        HRESULT ( STDMETHODCALLTYPE *UacScan )( 
            __RPC__in IAntimalwareUacProvider * This,
            /* [in] */ __RPC__in LPAMSI_UAC_REQUEST_CONTEXT context,
            /* [out] */ __RPC__out AMSI_RESULT *result);
        
        DECLSPEC_XFGVIRT(IAntimalwareUacProvider, DisplayName)
        HRESULT ( STDMETHODCALLTYPE *DisplayName )( 
            __RPC__in IAntimalwareUacProvider * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *displayName);
        
        END_INTERFACE
    } IAntimalwareUacProviderVtbl;

    interface IAntimalwareUacProvider
    {
        CONST_VTBL struct IAntimalwareUacProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAntimalwareUacProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAntimalwareUacProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAntimalwareUacProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAntimalwareUacProvider_UacScan(This,context,result)	\
    ( (This)->lpVtbl -> UacScan(This,context,result) ) 

#define IAntimalwareUacProvider_DisplayName(This,displayName)	\
    ( (This)->lpVtbl -> DisplayName(This,displayName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAntimalwareUacProvider_INTERFACE_DEFINED__ */


#ifndef __IAntimalwareProvider2_INTERFACE_DEFINED__
#define __IAntimalwareProvider2_INTERFACE_DEFINED__

/* interface IAntimalwareProvider2 */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAntimalwareProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7c1e6570-3f73-4e0f-8ad4-98b94cd3290f")
    IAntimalwareProvider2 : public IAntimalwareProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ PVOID buffer,
            /* [in] */ ULONG length,
            /* [in] */ LPCWSTR contentName,
            /* [in] */ LPCWSTR appName,
            /* [out] */ AMSI_RESULT *pResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAntimalwareProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAntimalwareProvider2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAntimalwareProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAntimalwareProvider2 * This);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, Scan)
        HRESULT ( STDMETHODCALLTYPE *Scan )( 
            IAntimalwareProvider2 * This,
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, CloseSession)
        void ( STDMETHODCALLTYPE *CloseSession )( 
            IAntimalwareProvider2 * This,
            /* [in] */ ULONGLONG session);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider, DisplayName)
        HRESULT ( STDMETHODCALLTYPE *DisplayName )( 
            IAntimalwareProvider2 * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *displayName);
        
        DECLSPEC_XFGVIRT(IAntimalwareProvider2, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            IAntimalwareProvider2 * This,
            /* [in] */ PVOID buffer,
            /* [in] */ ULONG length,
            /* [in] */ LPCWSTR contentName,
            /* [in] */ LPCWSTR appName,
            /* [out] */ AMSI_RESULT *pResult);
        
        END_INTERFACE
    } IAntimalwareProvider2Vtbl;

    interface IAntimalwareProvider2
    {
        CONST_VTBL struct IAntimalwareProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAntimalwareProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAntimalwareProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAntimalwareProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAntimalwareProvider2_Scan(This,stream,result)	\
    ( (This)->lpVtbl -> Scan(This,stream,result) ) 

#define IAntimalwareProvider2_CloseSession(This,session)	\
    ( (This)->lpVtbl -> CloseSession(This,session) ) 

#define IAntimalwareProvider2_DisplayName(This,displayName)	\
    ( (This)->lpVtbl -> DisplayName(This,displayName) ) 


#define IAntimalwareProvider2_Notify(This,buffer,length,contentName,appName,pResult)	\
    ( (This)->lpVtbl -> Notify(This,buffer,length,contentName,appName,pResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAntimalwareProvider2_INTERFACE_DEFINED__ */


#ifndef __IAntimalware_INTERFACE_DEFINED__
#define __IAntimalware_INTERFACE_DEFINED__

/* interface IAntimalware */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAntimalware;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82d29c2e-f062-44e6-b5c9-3d9a2f24a2df")
    IAntimalware : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Scan( 
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result,
            /* [out] */ IAntimalwareProvider **provider) = 0;
        
        virtual void STDMETHODCALLTYPE CloseSession( 
            /* [in] */ ULONGLONG session) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAntimalwareVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAntimalware * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAntimalware * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAntimalware * This);
        
        DECLSPEC_XFGVIRT(IAntimalware, Scan)
        HRESULT ( STDMETHODCALLTYPE *Scan )( 
            IAntimalware * This,
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result,
            /* [out] */ IAntimalwareProvider **provider);
        
        DECLSPEC_XFGVIRT(IAntimalware, CloseSession)
        void ( STDMETHODCALLTYPE *CloseSession )( 
            IAntimalware * This,
            /* [in] */ ULONGLONG session);
        
        END_INTERFACE
    } IAntimalwareVtbl;

    interface IAntimalware
    {
        CONST_VTBL struct IAntimalwareVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAntimalware_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAntimalware_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAntimalware_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAntimalware_Scan(This,stream,result,provider)	\
    ( (This)->lpVtbl -> Scan(This,stream,result,provider) ) 

#define IAntimalware_CloseSession(This,session)	\
    ( (This)->lpVtbl -> CloseSession(This,session) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAntimalware_INTERFACE_DEFINED__ */


#ifndef __IAntimalware2_INTERFACE_DEFINED__
#define __IAntimalware2_INTERFACE_DEFINED__

/* interface IAntimalware2 */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IAntimalware2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("301035b5-2d42-4f56-8c65-2dcaa7fb3cdc")
    IAntimalware2 : public IAntimalware
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ PVOID buffer,
            /* [in] */ ULONG length,
            /* [in] */ LPCWSTR contentName,
            /* [in] */ LPCWSTR appName,
            /* [out] */ AMSI_RESULT *pResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAntimalware2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAntimalware2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAntimalware2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAntimalware2 * This);
        
        DECLSPEC_XFGVIRT(IAntimalware, Scan)
        HRESULT ( STDMETHODCALLTYPE *Scan )( 
            IAntimalware2 * This,
            /* [in] */ IAmsiStream *stream,
            /* [out] */ AMSI_RESULT *result,
            /* [out] */ IAntimalwareProvider **provider);
        
        DECLSPEC_XFGVIRT(IAntimalware, CloseSession)
        void ( STDMETHODCALLTYPE *CloseSession )( 
            IAntimalware2 * This,
            /* [in] */ ULONGLONG session);
        
        DECLSPEC_XFGVIRT(IAntimalware2, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            IAntimalware2 * This,
            /* [in] */ PVOID buffer,
            /* [in] */ ULONG length,
            /* [in] */ LPCWSTR contentName,
            /* [in] */ LPCWSTR appName,
            /* [out] */ AMSI_RESULT *pResult);
        
        END_INTERFACE
    } IAntimalware2Vtbl;

    interface IAntimalware2
    {
        CONST_VTBL struct IAntimalware2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAntimalware2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAntimalware2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAntimalware2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAntimalware2_Scan(This,stream,result,provider)	\
    ( (This)->lpVtbl -> Scan(This,stream,result,provider) ) 

#define IAntimalware2_CloseSession(This,session)	\
    ( (This)->lpVtbl -> CloseSession(This,session) ) 


#define IAntimalware2_Notify(This,buffer,length,contentName,appName,pResult)	\
    ( (This)->lpVtbl -> Notify(This,buffer,length,contentName,appName,pResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAntimalware2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_amsi_0000_0006 */
/* [local] */ 

#ifdef __cplusplus
class DECLSPEC_UUID("fdb00e52-a214-4aa1-8fba-4357bb0072ec") CAntimalware;
#endif
EXTERN_C const CLSID CLSID_Antimalware;

DECLARE_HANDLE(HAMSICONTEXT);
DECLARE_HANDLE(HAMSISESSION);

STDAPI AmsiInitialize(
    _In_  LPCWSTR appName,
    _Outptr_ HAMSICONTEXT* amsiContext);

STDAPI_(VOID) AmsiUninitialize(
    _In_  HAMSICONTEXT amsiContext);

STDAPI AmsiOpenSession(
    _In_  HAMSICONTEXT amsiContext,
    _Out_ HAMSISESSION* amsiSession);

STDAPI_(VOID) AmsiCloseSession(
    _In_  HAMSICONTEXT amsiContext,
    _In_  HAMSISESSION amsiSession);

STDAPI AmsiScanBuffer(
    _In_  HAMSICONTEXT amsiContext,
    _In_reads_bytes_(length) PVOID buffer,
    _In_  ULONG length,
    _In_opt_  LPCWSTR contentName,
    _In_opt_  HAMSISESSION amsiSession,
    _Out_ AMSI_RESULT* result);

STDAPI AmsiNotifyOperation(
    _In_  HAMSICONTEXT amsiContext,
    _In_reads_bytes_(length) PVOID buffer,
    _In_  ULONG length,
    _In_opt_  LPCWSTR contentName,
    _Out_ AMSI_RESULT* result);

STDAPI AmsiScanString(
    _In_  HAMSICONTEXT amsiContext,
    _In_  LPCWSTR string,
    _In_opt_  LPCWSTR contentName,
    _In_opt_  HAMSISESSION amsiSession,
    _Out_ AMSI_RESULT* result);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_amsi_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_amsi_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


