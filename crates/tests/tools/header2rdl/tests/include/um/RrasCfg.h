

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

#ifndef __rrascfg_h__
#define __rrascfg_h__

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

#ifndef __IRouterProtocolConfig_FWD_DEFINED__
#define __IRouterProtocolConfig_FWD_DEFINED__
typedef interface IRouterProtocolConfig IRouterProtocolConfig;

#endif 	/* __IRouterProtocolConfig_FWD_DEFINED__ */


#ifndef __IAuthenticationProviderConfig_FWD_DEFINED__
#define __IAuthenticationProviderConfig_FWD_DEFINED__
typedef interface IAuthenticationProviderConfig IAuthenticationProviderConfig;

#endif 	/* __IAuthenticationProviderConfig_FWD_DEFINED__ */


#ifndef __IAccountingProviderConfig_FWD_DEFINED__
#define __IAccountingProviderConfig_FWD_DEFINED__
typedef interface IAccountingProviderConfig IAccountingProviderConfig;

#endif 	/* __IAccountingProviderConfig_FWD_DEFINED__ */


#ifndef __IEAPProviderConfig_FWD_DEFINED__
#define __IEAPProviderConfig_FWD_DEFINED__
typedef interface IEAPProviderConfig IEAPProviderConfig;

#endif 	/* __IEAPProviderConfig_FWD_DEFINED__ */


#ifndef __IEAPProviderConfig2_FWD_DEFINED__
#define __IEAPProviderConfig2_FWD_DEFINED__
typedef interface IEAPProviderConfig2 IEAPProviderConfig2;

#endif 	/* __IEAPProviderConfig2_FWD_DEFINED__ */


#ifndef __IEAPProviderConfig3_FWD_DEFINED__
#define __IEAPProviderConfig3_FWD_DEFINED__
typedef interface IEAPProviderConfig3 IEAPProviderConfig3;

#endif 	/* __IEAPProviderConfig3_FWD_DEFINED__ */


/* header files for imported files */
#include "basetsd.h"
#include "wtypes.h"
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rrascfg_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// RRasCfg.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1998 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
 
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef BYTE *PBYTE;



extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0000_v0_0_s_ifspec;

#ifndef __IRouterProtocolConfig_INTERFACE_DEFINED__
#define __IRouterProtocolConfig_INTERFACE_DEFINED__

/* interface IRouterProtocolConfig */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IRouterProtocolConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66A2DB16-D706-11D0-A37B-00C04FC9DA04")
    IRouterProtocolConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddProtocol( 
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwTransportId,
            /* [in] */ DWORD dwProtocolId,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pRouter,
            /* [in] */ ULONG_PTR uReserved1) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveProtocol( 
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwTransportId,
            /* [in] */ DWORD dwProtocolId,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pRouter,
            /* [in] */ ULONG_PTR uReserved1) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRouterProtocolConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRouterProtocolConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRouterProtocolConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRouterProtocolConfig * This);
        
        DECLSPEC_XFGVIRT(IRouterProtocolConfig, AddProtocol)
        HRESULT ( STDMETHODCALLTYPE *AddProtocol )( 
            IRouterProtocolConfig * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwTransportId,
            /* [in] */ DWORD dwProtocolId,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pRouter,
            /* [in] */ ULONG_PTR uReserved1);
        
        DECLSPEC_XFGVIRT(IRouterProtocolConfig, RemoveProtocol)
        HRESULT ( STDMETHODCALLTYPE *RemoveProtocol )( 
            IRouterProtocolConfig * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwTransportId,
            /* [in] */ DWORD dwProtocolId,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pRouter,
            /* [in] */ ULONG_PTR uReserved1);
        
        END_INTERFACE
    } IRouterProtocolConfigVtbl;

    interface IRouterProtocolConfig
    {
        CONST_VTBL struct IRouterProtocolConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRouterProtocolConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRouterProtocolConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRouterProtocolConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRouterProtocolConfig_AddProtocol(This,pszMachineName,dwTransportId,dwProtocolId,hWnd,dwFlags,pRouter,uReserved1)	\
    ( (This)->lpVtbl -> AddProtocol(This,pszMachineName,dwTransportId,dwProtocolId,hWnd,dwFlags,pRouter,uReserved1) ) 

#define IRouterProtocolConfig_RemoveProtocol(This,pszMachineName,dwTransportId,dwProtocolId,hWnd,dwFlags,pRouter,uReserved1)	\
    ( (This)->lpVtbl -> RemoveProtocol(This,pszMachineName,dwTransportId,dwProtocolId,hWnd,dwFlags,pRouter,uReserved1) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRouterProtocolConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rrascfg_0000_0001 */
/* [local] */ 

#define DeclareIRouterProtocolConfigMembers(IPURE) \
	STDMETHOD(AddProtocol)(THIS_ LPCOLESTR pszMachineName,\
					   DWORD dwTransportId,\
					   DWORD dwProtocolId,\
					   HWND hWnd,\
					   DWORD dwFlags,\
					   IUnknown *pRouter,\
					   ULONG_PTR uReserved1) IPURE;\
	STDMETHOD(RemoveProtocol)(THIS_ LPCOLESTR pszMachineName,\
						 DWORD dwTransportId,\
						 DWORD dwProtocolId,\
						 HWND hWnd,\
						 DWORD dwFlags,\
						 IUnknown *pRouter,\
						 ULONG_PTR uReserved2) IPURE;\
 


extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0001_v0_0_s_ifspec;

#ifndef __IAuthenticationProviderConfig_INTERFACE_DEFINED__
#define __IAuthenticationProviderConfig_INTERFACE_DEFINED__

/* interface IAuthenticationProviderConfig */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IAuthenticationProviderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66A2DB17-D706-11D0-A37B-00C04FC9DA04")
    IAuthenticationProviderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [out] */ ULONG_PTR *puConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( 
            /* [in] */ ULONG_PTR uConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Configure( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAuthenticationProviderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAuthenticationProviderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAuthenticationProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAuthenticationProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IAuthenticationProviderConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IAuthenticationProviderConfig * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [out] */ ULONG_PTR *puConnectionParam);
        
        DECLSPEC_XFGVIRT(IAuthenticationProviderConfig, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            IAuthenticationProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam);
        
        DECLSPEC_XFGVIRT(IAuthenticationProviderConfig, Configure)
        HRESULT ( STDMETHODCALLTYPE *Configure )( 
            IAuthenticationProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IAuthenticationProviderConfig, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IAuthenticationProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IAuthenticationProviderConfig, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            IAuthenticationProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        END_INTERFACE
    } IAuthenticationProviderConfigVtbl;

    interface IAuthenticationProviderConfig
    {
        CONST_VTBL struct IAuthenticationProviderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAuthenticationProviderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAuthenticationProviderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAuthenticationProviderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAuthenticationProviderConfig_Initialize(This,pszMachineName,puConnectionParam)	\
    ( (This)->lpVtbl -> Initialize(This,pszMachineName,puConnectionParam) ) 

#define IAuthenticationProviderConfig_Uninitialize(This,uConnectionParam)	\
    ( (This)->lpVtbl -> Uninitialize(This,uConnectionParam) ) 

#define IAuthenticationProviderConfig_Configure(This,uConnectionParam,hWnd,dwFlags,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Configure(This,uConnectionParam,hWnd,dwFlags,uReserved1,uReserved2) ) 

#define IAuthenticationProviderConfig_Activate(This,uConnectionParam,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Activate(This,uConnectionParam,uReserved1,uReserved2) ) 

#define IAuthenticationProviderConfig_Deactivate(This,uConnectionParam,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Deactivate(This,uConnectionParam,uReserved1,uReserved2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAuthenticationProviderConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rrascfg_0000_0002 */
/* [local] */ 

#define DeclareIAuthenticationProviderConfigMembers(IPURE) \
	STDMETHOD(Initialize)(THIS_ \
							LPCOLESTR pszMachineName, \
							ULONG_PTR *puConnectionParam) IPURE; \
	STDMETHOD(Uninitialize)(THIS_ \
							ULONG_PTR uConnectionParam) IPURE; \
	 \
	STDMETHOD(Configure)(THIS_ \
							ULONG_PTR uConnectionParam, \
							HWND hWnd, \
						  DWORD dwFlags, \
						  ULONG_PTR uReserved1, \
						  ULONG_PTR uReserved2) IPURE; \
 \
	STDMETHOD(Activate)(THIS_ \
						ULONG_PTR uConnectionParam, \
						 ULONG_PTR uReserved1, \
						 ULONG_PTR uReserved2) IPURE; \
 \
	STDMETHOD(Deactivate)(THIS_ \
						ULONG_PTR uConnectionParam, \
						   ULONG_PTR uReserved1, \
						   ULONG_PTR uReserved2) IPURE; \
 


extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0002_v0_0_s_ifspec;

#ifndef __IAccountingProviderConfig_INTERFACE_DEFINED__
#define __IAccountingProviderConfig_INTERFACE_DEFINED__

/* interface IAccountingProviderConfig */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IAccountingProviderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66A2DB18-D706-11D0-A37B-00C04FC9DA04")
    IAccountingProviderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [out] */ ULONG_PTR *puConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( 
            /* [in] */ ULONG_PTR uConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Configure( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( 
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccountingProviderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAccountingProviderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAccountingProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAccountingProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IAccountingProviderConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IAccountingProviderConfig * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [out] */ ULONG_PTR *puConnectionParam);
        
        DECLSPEC_XFGVIRT(IAccountingProviderConfig, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            IAccountingProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam);
        
        DECLSPEC_XFGVIRT(IAccountingProviderConfig, Configure)
        HRESULT ( STDMETHODCALLTYPE *Configure )( 
            IAccountingProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IAccountingProviderConfig, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IAccountingProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IAccountingProviderConfig, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            IAccountingProviderConfig * This,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        END_INTERFACE
    } IAccountingProviderConfigVtbl;

    interface IAccountingProviderConfig
    {
        CONST_VTBL struct IAccountingProviderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccountingProviderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccountingProviderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccountingProviderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccountingProviderConfig_Initialize(This,pszMachineName,puConnectionParam)	\
    ( (This)->lpVtbl -> Initialize(This,pszMachineName,puConnectionParam) ) 

#define IAccountingProviderConfig_Uninitialize(This,uConnectionParam)	\
    ( (This)->lpVtbl -> Uninitialize(This,uConnectionParam) ) 

#define IAccountingProviderConfig_Configure(This,uConnectionParam,hWnd,dwFlags,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Configure(This,uConnectionParam,hWnd,dwFlags,uReserved1,uReserved2) ) 

#define IAccountingProviderConfig_Activate(This,uConnectionParam,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Activate(This,uConnectionParam,uReserved1,uReserved2) ) 

#define IAccountingProviderConfig_Deactivate(This,uConnectionParam,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> Deactivate(This,uConnectionParam,uReserved1,uReserved2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccountingProviderConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rrascfg_0000_0003 */
/* [local] */ 

#define DeclareIAccountingProviderConfigMembers(IPURE) \
	STDMETHOD(Initialize)(THIS_ \
							LPCOLESTR pszMachineName, \
							ULONG_PTR *puConnectionParam) IPURE; \
	STDMETHOD(Uninitialize)(THIS_ \
							ULONG_PTR uConnectionParam) IPURE; \
	STDMETHOD(Configure)(THIS_ \
						ULONG_PTR uConnectionParam, \
						HWND hWnd, \
						  DWORD dwFlags, \
						  ULONG_PTR uReserved1, \
						  ULONG_PTR uReserved2) IPURE; \
 \
	STDMETHOD(Activate)(THIS_ \
						ULONG_PTR uConnectionParam, \
						 ULONG_PTR uReserved1, \
						 ULONG_PTR uReserved2) IPURE; \
 \
	STDMETHOD(Deactivate)(THIS_ \
						ULONG_PTR uConnectionParam, \
						   ULONG_PTR uReserved1, \
						   ULONG_PTR uReserved2) IPURE; \
 


extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0003_v0_0_s_ifspec;

#ifndef __IEAPProviderConfig_INTERFACE_DEFINED__
#define __IEAPProviderConfig_INTERFACE_DEFINED__

/* interface IEAPProviderConfig */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IEAPProviderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66A2DB19-D706-11D0-A37B-00C04FC9DA04")
    IEAPProviderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwEapTypeId,
            /* [out] */ ULONG_PTR *puConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( 
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ServerInvokeConfigUI( 
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RouterInvokeConfigUI( 
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][size_is][out] */ BYTE **ppConnectionDataOut,
            /* [out] */ DWORD *pdwSizeOfConnectionDataOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RouterInvokeCredentialsUI( 
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][in] */ BYTE *pUserDataIn,
            /* [in] */ DWORD dwSizeOfUserDataIn,
            /* [size_is][size_is][out] */ BYTE **ppUserDataOut,
            /* [out] */ DWORD *pdwSizeOfUserDataOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEAPProviderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEAPProviderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEAPProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEAPProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IEAPProviderConfig * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwEapTypeId,
            /* [out] */ ULONG_PTR *puConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            IEAPProviderConfig * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, ServerInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeConfigUI )( 
            IEAPProviderConfig * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeConfigUI )( 
            IEAPProviderConfig * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][size_is][out] */ BYTE **ppConnectionDataOut,
            /* [out] */ DWORD *pdwSizeOfConnectionDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeCredentialsUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeCredentialsUI )( 
            IEAPProviderConfig * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][in] */ BYTE *pUserDataIn,
            /* [in] */ DWORD dwSizeOfUserDataIn,
            /* [size_is][size_is][out] */ BYTE **ppUserDataOut,
            /* [out] */ DWORD *pdwSizeOfUserDataOut);
        
        END_INTERFACE
    } IEAPProviderConfigVtbl;

    interface IEAPProviderConfig
    {
        CONST_VTBL struct IEAPProviderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEAPProviderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEAPProviderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEAPProviderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEAPProviderConfig_Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam)	\
    ( (This)->lpVtbl -> Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam) ) 

#define IEAPProviderConfig_Uninitialize(This,dwEapTypeId,uConnectionParam)	\
    ( (This)->lpVtbl -> Uninitialize(This,dwEapTypeId,uConnectionParam) ) 

#define IEAPProviderConfig_ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2) ) 

#define IEAPProviderConfig_RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut) ) 

#define IEAPProviderConfig_RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEAPProviderConfig_INTERFACE_DEFINED__ */


#ifndef __IEAPProviderConfig2_INTERFACE_DEFINED__
#define __IEAPProviderConfig2_INTERFACE_DEFINED__

/* interface IEAPProviderConfig2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IEAPProviderConfig2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D565917A-85C4-4466-856E-671C3742EA9A")
    IEAPProviderConfig2 : public IEAPProviderConfig
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ServerInvokeConfigUI2( 
            DWORD dwEapTypeId,
            ULONG_PTR uConnectionParam,
            HWND hWnd,
            const BYTE *pConfigDataIn,
            DWORD dwSizeOfConfigDataIn,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlobalConfig( 
            DWORD dwEapTypeId,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEAPProviderConfig2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEAPProviderConfig2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEAPProviderConfig2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEAPProviderConfig2 * This);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IEAPProviderConfig2 * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwEapTypeId,
            /* [out] */ ULONG_PTR *puConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            IEAPProviderConfig2 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, ServerInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeConfigUI )( 
            IEAPProviderConfig2 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeConfigUI )( 
            IEAPProviderConfig2 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][size_is][out] */ BYTE **ppConnectionDataOut,
            /* [out] */ DWORD *pdwSizeOfConnectionDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeCredentialsUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeCredentialsUI )( 
            IEAPProviderConfig2 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][in] */ BYTE *pUserDataIn,
            /* [in] */ DWORD dwSizeOfUserDataIn,
            /* [size_is][size_is][out] */ BYTE **ppUserDataOut,
            /* [out] */ DWORD *pdwSizeOfUserDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig2, ServerInvokeConfigUI2)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeConfigUI2 )( 
            IEAPProviderConfig2 * This,
            DWORD dwEapTypeId,
            ULONG_PTR uConnectionParam,
            HWND hWnd,
            const BYTE *pConfigDataIn,
            DWORD dwSizeOfConfigDataIn,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig2, GetGlobalConfig)
        HRESULT ( STDMETHODCALLTYPE *GetGlobalConfig )( 
            IEAPProviderConfig2 * This,
            DWORD dwEapTypeId,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut);
        
        END_INTERFACE
    } IEAPProviderConfig2Vtbl;

    interface IEAPProviderConfig2
    {
        CONST_VTBL struct IEAPProviderConfig2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEAPProviderConfig2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEAPProviderConfig2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEAPProviderConfig2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEAPProviderConfig2_Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam)	\
    ( (This)->lpVtbl -> Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam) ) 

#define IEAPProviderConfig2_Uninitialize(This,dwEapTypeId,uConnectionParam)	\
    ( (This)->lpVtbl -> Uninitialize(This,dwEapTypeId,uConnectionParam) ) 

#define IEAPProviderConfig2_ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2) ) 

#define IEAPProviderConfig2_RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut) ) 

#define IEAPProviderConfig2_RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut) ) 


#define IEAPProviderConfig2_ServerInvokeConfigUI2(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut)	\
    ( (This)->lpVtbl -> ServerInvokeConfigUI2(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut) ) 

#define IEAPProviderConfig2_GetGlobalConfig(This,dwEapTypeId,ppConfigDataOut,pdwSizeOfConfigDataOut)	\
    ( (This)->lpVtbl -> GetGlobalConfig(This,dwEapTypeId,ppConfigDataOut,pdwSizeOfConfigDataOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEAPProviderConfig2_INTERFACE_DEFINED__ */


#ifndef __IEAPProviderConfig3_INTERFACE_DEFINED__
#define __IEAPProviderConfig3_INTERFACE_DEFINED__

/* interface IEAPProviderConfig3 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IEAPProviderConfig3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B78ECD12-68BB-4f86-9BF0-8438DD3BE982")
    IEAPProviderConfig3 : public IEAPProviderConfig2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ServerInvokeCertificateConfigUI( 
            DWORD dwEapTypeId,
            ULONG_PTR uConnectionParam,
            HWND hWnd,
            const BYTE *pConfigDataIn,
            DWORD dwSizeOfConfigDataIn,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut,
            ULONG_PTR uReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEAPProviderConfig3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEAPProviderConfig3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEAPProviderConfig3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEAPProviderConfig3 * This);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IEAPProviderConfig3 * This,
            /* [string][in] */ LPCOLESTR pszMachineName,
            /* [in] */ DWORD dwEapTypeId,
            /* [out] */ ULONG_PTR *puConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            IEAPProviderConfig3 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, ServerInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeConfigUI )( 
            IEAPProviderConfig3 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hWnd,
            /* [in] */ ULONG_PTR uReserved1,
            /* [in] */ ULONG_PTR uReserved2);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeConfigUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeConfigUI )( 
            IEAPProviderConfig3 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][size_is][out] */ BYTE **ppConnectionDataOut,
            /* [out] */ DWORD *pdwSizeOfConnectionDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig, RouterInvokeCredentialsUI)
        HRESULT ( STDMETHODCALLTYPE *RouterInvokeCredentialsUI )( 
            IEAPProviderConfig3 * This,
            /* [in] */ DWORD dwEapTypeId,
            /* [in] */ ULONG_PTR uConnectionParam,
            /* [in] */ HWND hwndParent,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ BYTE *pConnectionDataIn,
            /* [in] */ DWORD dwSizeOfConnectionDataIn,
            /* [size_is][in] */ BYTE *pUserDataIn,
            /* [in] */ DWORD dwSizeOfUserDataIn,
            /* [size_is][size_is][out] */ BYTE **ppUserDataOut,
            /* [out] */ DWORD *pdwSizeOfUserDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig2, ServerInvokeConfigUI2)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeConfigUI2 )( 
            IEAPProviderConfig3 * This,
            DWORD dwEapTypeId,
            ULONG_PTR uConnectionParam,
            HWND hWnd,
            const BYTE *pConfigDataIn,
            DWORD dwSizeOfConfigDataIn,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig2, GetGlobalConfig)
        HRESULT ( STDMETHODCALLTYPE *GetGlobalConfig )( 
            IEAPProviderConfig3 * This,
            DWORD dwEapTypeId,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut);
        
        DECLSPEC_XFGVIRT(IEAPProviderConfig3, ServerInvokeCertificateConfigUI)
        HRESULT ( STDMETHODCALLTYPE *ServerInvokeCertificateConfigUI )( 
            IEAPProviderConfig3 * This,
            DWORD dwEapTypeId,
            ULONG_PTR uConnectionParam,
            HWND hWnd,
            const BYTE *pConfigDataIn,
            DWORD dwSizeOfConfigDataIn,
            BYTE **ppConfigDataOut,
            DWORD *pdwSizeOfConfigDataOut,
            ULONG_PTR uReserved);
        
        END_INTERFACE
    } IEAPProviderConfig3Vtbl;

    interface IEAPProviderConfig3
    {
        CONST_VTBL struct IEAPProviderConfig3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEAPProviderConfig3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEAPProviderConfig3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEAPProviderConfig3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEAPProviderConfig3_Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam)	\
    ( (This)->lpVtbl -> Initialize(This,pszMachineName,dwEapTypeId,puConnectionParam) ) 

#define IEAPProviderConfig3_Uninitialize(This,dwEapTypeId,uConnectionParam)	\
    ( (This)->lpVtbl -> Uninitialize(This,dwEapTypeId,uConnectionParam) ) 

#define IEAPProviderConfig3_ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2)	\
    ( (This)->lpVtbl -> ServerInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,uReserved1,uReserved2) ) 

#define IEAPProviderConfig3_RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeConfigUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,ppConnectionDataOut,pdwSizeOfConnectionDataOut) ) 

#define IEAPProviderConfig3_RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut)	\
    ( (This)->lpVtbl -> RouterInvokeCredentialsUI(This,dwEapTypeId,uConnectionParam,hwndParent,dwFlags,pConnectionDataIn,dwSizeOfConnectionDataIn,pUserDataIn,dwSizeOfUserDataIn,ppUserDataOut,pdwSizeOfUserDataOut) ) 


#define IEAPProviderConfig3_ServerInvokeConfigUI2(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut)	\
    ( (This)->lpVtbl -> ServerInvokeConfigUI2(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut) ) 

#define IEAPProviderConfig3_GetGlobalConfig(This,dwEapTypeId,ppConfigDataOut,pdwSizeOfConfigDataOut)	\
    ( (This)->lpVtbl -> GetGlobalConfig(This,dwEapTypeId,ppConfigDataOut,pdwSizeOfConfigDataOut) ) 


#define IEAPProviderConfig3_ServerInvokeCertificateConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut,uReserved)	\
    ( (This)->lpVtbl -> ServerInvokeCertificateConfigUI(This,dwEapTypeId,uConnectionParam,hWnd,pConfigDataIn,dwSizeOfConfigDataIn,ppConfigDataOut,pdwSizeOfConfigDataOut,uReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEAPProviderConfig3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rrascfg_0000_0006 */
/* [local] */ 

#define DeclareIEAPProviderConfigMembers(IPURE) \
	STDMETHOD(Initialize)(THIS_ \
		LPCOLESTR 	pszMachineName, \
 	DWORD       dwEapTypeId, \
		ULONG_PTR*	puConnectionParam) IPURE; \
	STDMETHOD(Uninitialize)(THIS_ \
 	DWORD       dwEapTypeId, \
		ULONG_PTR 	uConnectionParam) IPURE; \
	STDMETHOD(ServerInvokeConfigUI)(THIS_ \
 	DWORD       dwEapTypeId, \
		ULONG_PTR 	uConnectionParam, \
		HWND 		hWnd, \
		ULONG_PTR 	dwRes1, \
		ULONG_PTR 	dwRes2) IPURE; \
 STDMETHOD(RouterInvokeConfigUI)(THIS_ \
 	DWORD       dwEapTypeId, \
		ULONG_PTR 	uConnectionParam, \
 	HWND        hwndParent, \
 	DWORD       dwFlags, \
 	BYTE* 		pConnectionDataIn, \
 	DWORD		dwSizeOfConnectionDataIn, \
 	BYTE**		ppConnectionDataOut, \
 	DWORD*		pdwSizeOfConnectionDataOut) IPURE; \
 STDMETHOD(RouterInvokeCredentialsUI)(THIS_  \
 	DWORD   	dwEapTypeId, \
		ULONG_PTR 	uConnectionParam, \
 	HWND    	hwndParent, \
 	DWORD   	dwFlags, \
 	BYTE*   	pConnectionDataIn, \
 	DWORD   	dwSizeOfConnectionDataIn, \
 	BYTE*   	pUserDataIn, \
 	DWORD   	dwSizeOfUserDataIn, \
 	BYTE**  	ppUserDataOut, \
 	DWORD*  	pdwSizeOfUserDataOut) IPURE; \
 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rrascfg_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


