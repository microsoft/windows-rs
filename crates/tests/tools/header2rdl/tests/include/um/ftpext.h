

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

#ifndef __ftpext_h__
#define __ftpext_h__

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

#ifndef __IFtpProviderConstruct_FWD_DEFINED__
#define __IFtpProviderConstruct_FWD_DEFINED__
typedef interface IFtpProviderConstruct IFtpProviderConstruct;

#endif 	/* __IFtpProviderConstruct_FWD_DEFINED__ */


#ifndef __IFtpAuthenticationProvider_FWD_DEFINED__
#define __IFtpAuthenticationProvider_FWD_DEFINED__
typedef interface IFtpAuthenticationProvider IFtpAuthenticationProvider;

#endif 	/* __IFtpAuthenticationProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpAuthenticationProvider_FWD_DEFINED__
#define __AsyncIFtpAuthenticationProvider_FWD_DEFINED__
typedef interface AsyncIFtpAuthenticationProvider AsyncIFtpAuthenticationProvider;

#endif 	/* __AsyncIFtpAuthenticationProvider_FWD_DEFINED__ */


#ifndef __IFtpRoleProvider_FWD_DEFINED__
#define __IFtpRoleProvider_FWD_DEFINED__
typedef interface IFtpRoleProvider IFtpRoleProvider;

#endif 	/* __IFtpRoleProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpRoleProvider_FWD_DEFINED__
#define __AsyncIFtpRoleProvider_FWD_DEFINED__
typedef interface AsyncIFtpRoleProvider AsyncIFtpRoleProvider;

#endif 	/* __AsyncIFtpRoleProvider_FWD_DEFINED__ */


#ifndef __IFtpHomeDirectoryProvider_FWD_DEFINED__
#define __IFtpHomeDirectoryProvider_FWD_DEFINED__
typedef interface IFtpHomeDirectoryProvider IFtpHomeDirectoryProvider;

#endif 	/* __IFtpHomeDirectoryProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpHomeDirectoryProvider_FWD_DEFINED__
#define __AsyncIFtpHomeDirectoryProvider_FWD_DEFINED__
typedef interface AsyncIFtpHomeDirectoryProvider AsyncIFtpHomeDirectoryProvider;

#endif 	/* __AsyncIFtpHomeDirectoryProvider_FWD_DEFINED__ */


#ifndef __IFtpLogProvider_FWD_DEFINED__
#define __IFtpLogProvider_FWD_DEFINED__
typedef interface IFtpLogProvider IFtpLogProvider;

#endif 	/* __IFtpLogProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpLogProvider_FWD_DEFINED__
#define __AsyncIFtpLogProvider_FWD_DEFINED__
typedef interface AsyncIFtpLogProvider AsyncIFtpLogProvider;

#endif 	/* __AsyncIFtpLogProvider_FWD_DEFINED__ */


#ifndef __IFtpAuthorizationProvider_FWD_DEFINED__
#define __IFtpAuthorizationProvider_FWD_DEFINED__
typedef interface IFtpAuthorizationProvider IFtpAuthorizationProvider;

#endif 	/* __IFtpAuthorizationProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpAuthorizationProvider_FWD_DEFINED__
#define __AsyncIFtpAuthorizationProvider_FWD_DEFINED__
typedef interface AsyncIFtpAuthorizationProvider AsyncIFtpAuthorizationProvider;

#endif 	/* __AsyncIFtpAuthorizationProvider_FWD_DEFINED__ */


#ifndef __IFtpPreprocessProvider_FWD_DEFINED__
#define __IFtpPreprocessProvider_FWD_DEFINED__
typedef interface IFtpPreprocessProvider IFtpPreprocessProvider;

#endif 	/* __IFtpPreprocessProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpPreprocessProvider_FWD_DEFINED__
#define __AsyncIFtpPreprocessProvider_FWD_DEFINED__
typedef interface AsyncIFtpPreprocessProvider AsyncIFtpPreprocessProvider;

#endif 	/* __AsyncIFtpPreprocessProvider_FWD_DEFINED__ */


#ifndef __IFtpPostprocessProvider_FWD_DEFINED__
#define __IFtpPostprocessProvider_FWD_DEFINED__
typedef interface IFtpPostprocessProvider IFtpPostprocessProvider;

#endif 	/* __IFtpPostprocessProvider_FWD_DEFINED__ */


#ifndef __AsyncIFtpPostprocessProvider_FWD_DEFINED__
#define __AsyncIFtpPostprocessProvider_FWD_DEFINED__
typedef interface AsyncIFtpPostprocessProvider AsyncIFtpPostprocessProvider;

#endif 	/* __AsyncIFtpPostprocessProvider_FWD_DEFINED__ */


#ifndef __FtpProvider_FWD_DEFINED__
#define __FtpProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class FtpProvider FtpProvider;
#else
typedef struct FtpProvider FtpProvider;
#endif /* __cplusplus */

#endif 	/* __FtpProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ftpext_0000_0000 */
/* [local] */ 

/*++

Copyright (c) 2008 Microsoft Corporation

Module Name: ftpext.h


 FTP extensibility API for FTP server (version 7.0 and higher)
 It can be used to implement

 - custom FTP authentication
 - custom FTP roles
 - custom FTP user isolation
 - custom FTP logging


--*/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
/* [uuid] */ struct  DECLSPEC_UUID("9e04226f-e38c-419e-a448-62de3b3a8f43") CONFIGURATION_ENTRY
    {
    BSTR bstrKey;
    BSTR bstrValue;
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0000_v0_0_s_ifspec;

#ifndef __IFtpProviderConstruct_INTERFACE_DEFINED__
#define __IFtpProviderConstruct_INTERFACE_DEFINED__

/* interface IFtpProviderConstruct */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IFtpProviderConstruct;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4d1a3f7b-412d-447c-b199-64f967e9a2da")
    IFtpProviderConstruct : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Construct( 
            /* [in] */ __RPC__in SAFEARRAY * configurationEntries) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpProviderConstructVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpProviderConstruct * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpProviderConstruct * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpProviderConstruct * This);
        
        DECLSPEC_XFGVIRT(IFtpProviderConstruct, Construct)
        HRESULT ( STDMETHODCALLTYPE *Construct )( 
            __RPC__in IFtpProviderConstruct * This,
            /* [in] */ __RPC__in SAFEARRAY * configurationEntries);
        
        END_INTERFACE
    } IFtpProviderConstructVtbl;

    interface IFtpProviderConstruct
    {
        CONST_VTBL struct IFtpProviderConstructVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpProviderConstruct_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpProviderConstruct_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpProviderConstruct_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpProviderConstruct_Construct(This,configurationEntries)	\
    ( (This)->lpVtbl -> Construct(This,configurationEntries) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpProviderConstruct_INTERFACE_DEFINED__ */


#ifndef __IFtpAuthenticationProvider_INTERFACE_DEFINED__
#define __IFtpAuthenticationProvider_INTERFACE_DEFINED__

/* interface IFtpAuthenticationProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpAuthenticationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4659f95c-d5a8-4707-b2fc-6fd5794246cf")
    IFtpAuthenticationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AuthenticateUser( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPassword,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszCanonicalUserName,
            /* [retval][out] */ __RPC__out BOOL *pfAuthenticated) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpAuthenticationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpAuthenticationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpAuthenticationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpAuthenticationProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpAuthenticationProvider, AuthenticateUser)
        HRESULT ( STDMETHODCALLTYPE *AuthenticateUser )( 
            __RPC__in IFtpAuthenticationProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPassword,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszCanonicalUserName,
            /* [retval][out] */ __RPC__out BOOL *pfAuthenticated);
        
        END_INTERFACE
    } IFtpAuthenticationProviderVtbl;

    interface IFtpAuthenticationProvider
    {
        CONST_VTBL struct IFtpAuthenticationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpAuthenticationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpAuthenticationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpAuthenticationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpAuthenticationProvider_AuthenticateUser(This,pszSessionId,pszSiteName,pszUserName,pszPassword,ppszCanonicalUserName,pfAuthenticated)	\
    ( (This)->lpVtbl -> AuthenticateUser(This,pszSessionId,pszSiteName,pszUserName,pszPassword,ppszCanonicalUserName,pfAuthenticated) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpAuthenticationProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpAuthenticationProvider_INTERFACE_DEFINED__
#define __AsyncIFtpAuthenticationProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpAuthenticationProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpAuthenticationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c24efb65-9f3e-4996-8fb1-ce166916bab5")
    AsyncIFtpAuthenticationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_AuthenticateUser( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_AuthenticateUser( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszCanonicalUserName,
            /* [retval][out] */ __RPC__out BOOL *pfAuthenticated) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpAuthenticationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpAuthenticationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpAuthenticationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpAuthenticationProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpAuthenticationProvider, Begin_AuthenticateUser)
        HRESULT ( STDMETHODCALLTYPE *Begin_AuthenticateUser )( 
            __RPC__in AsyncIFtpAuthenticationProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPassword);
        
        DECLSPEC_XFGVIRT(AsyncIFtpAuthenticationProvider, Finish_AuthenticateUser)
        HRESULT ( STDMETHODCALLTYPE *Finish_AuthenticateUser )( 
            __RPC__in AsyncIFtpAuthenticationProvider * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszCanonicalUserName,
            /* [retval][out] */ __RPC__out BOOL *pfAuthenticated);
        
        END_INTERFACE
    } AsyncIFtpAuthenticationProviderVtbl;

    interface AsyncIFtpAuthenticationProvider
    {
        CONST_VTBL struct AsyncIFtpAuthenticationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpAuthenticationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpAuthenticationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpAuthenticationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpAuthenticationProvider_Begin_AuthenticateUser(This,pszSessionId,pszSiteName,pszUserName,pszPassword)	\
    ( (This)->lpVtbl -> Begin_AuthenticateUser(This,pszSessionId,pszSiteName,pszUserName,pszPassword) ) 

#define AsyncIFtpAuthenticationProvider_Finish_AuthenticateUser(This,ppszCanonicalUserName,pfAuthenticated)	\
    ( (This)->lpVtbl -> Finish_AuthenticateUser(This,ppszCanonicalUserName,pfAuthenticated) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpAuthenticationProvider_INTERFACE_DEFINED__ */


#ifndef __IFtpRoleProvider_INTERFACE_DEFINED__
#define __IFtpRoleProvider_INTERFACE_DEFINED__

/* interface IFtpRoleProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpRoleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("909c850d-8ca0-4674-96b8-cc2941535725")
    IFtpRoleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsUserInRole( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszRole,
            /* [retval][out] */ __RPC__out BOOL *pfIsInRole) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpRoleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpRoleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpRoleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpRoleProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpRoleProvider, IsUserInRole)
        HRESULT ( STDMETHODCALLTYPE *IsUserInRole )( 
            __RPC__in IFtpRoleProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszRole,
            /* [retval][out] */ __RPC__out BOOL *pfIsInRole);
        
        END_INTERFACE
    } IFtpRoleProviderVtbl;

    interface IFtpRoleProvider
    {
        CONST_VTBL struct IFtpRoleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpRoleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpRoleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpRoleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpRoleProvider_IsUserInRole(This,pszSessionId,pszSiteName,pszUserName,pszRole,pfIsInRole)	\
    ( (This)->lpVtbl -> IsUserInRole(This,pszSessionId,pszSiteName,pszUserName,pszRole,pfIsInRole) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpRoleProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpRoleProvider_INTERFACE_DEFINED__
#define __AsyncIFtpRoleProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpRoleProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpRoleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e83bf99-70ec-41ca-84b6-aca7c7a62caf")
    AsyncIFtpRoleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_IsUserInRole( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_IsUserInRole( 
            /* [retval][out] */ __RPC__out BOOL *pfIsInRole) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpRoleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpRoleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpRoleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpRoleProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpRoleProvider, Begin_IsUserInRole)
        HRESULT ( STDMETHODCALLTYPE *Begin_IsUserInRole )( 
            __RPC__in AsyncIFtpRoleProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszRole);
        
        DECLSPEC_XFGVIRT(AsyncIFtpRoleProvider, Finish_IsUserInRole)
        HRESULT ( STDMETHODCALLTYPE *Finish_IsUserInRole )( 
            __RPC__in AsyncIFtpRoleProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pfIsInRole);
        
        END_INTERFACE
    } AsyncIFtpRoleProviderVtbl;

    interface AsyncIFtpRoleProvider
    {
        CONST_VTBL struct AsyncIFtpRoleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpRoleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpRoleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpRoleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpRoleProvider_Begin_IsUserInRole(This,pszSessionId,pszSiteName,pszUserName,pszRole)	\
    ( (This)->lpVtbl -> Begin_IsUserInRole(This,pszSessionId,pszSiteName,pszUserName,pszRole) ) 

#define AsyncIFtpRoleProvider_Finish_IsUserInRole(This,pfIsInRole)	\
    ( (This)->lpVtbl -> Finish_IsUserInRole(This,pfIsInRole) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpRoleProvider_INTERFACE_DEFINED__ */


#ifndef __IFtpHomeDirectoryProvider_INTERFACE_DEFINED__
#define __IFtpHomeDirectoryProvider_INTERFACE_DEFINED__

/* interface IFtpHomeDirectoryProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpHomeDirectoryProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0933b392-18dd-4097-8b9c-83325c35d9a6")
    IFtpHomeDirectoryProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUserHomeDirectoryData( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszHomeDirectoryData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpHomeDirectoryProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpHomeDirectoryProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpHomeDirectoryProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpHomeDirectoryProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpHomeDirectoryProvider, GetUserHomeDirectoryData)
        HRESULT ( STDMETHODCALLTYPE *GetUserHomeDirectoryData )( 
            __RPC__in IFtpHomeDirectoryProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszHomeDirectoryData);
        
        END_INTERFACE
    } IFtpHomeDirectoryProviderVtbl;

    interface IFtpHomeDirectoryProvider
    {
        CONST_VTBL struct IFtpHomeDirectoryProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpHomeDirectoryProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpHomeDirectoryProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpHomeDirectoryProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpHomeDirectoryProvider_GetUserHomeDirectoryData(This,pszSessionId,pszSiteName,pszUserName,ppszHomeDirectoryData)	\
    ( (This)->lpVtbl -> GetUserHomeDirectoryData(This,pszSessionId,pszSiteName,pszUserName,ppszHomeDirectoryData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpHomeDirectoryProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpHomeDirectoryProvider_INTERFACE_DEFINED__
#define __AsyncIFtpHomeDirectoryProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpHomeDirectoryProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpHomeDirectoryProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73f81638-6295-42bd-a2be-4a657f7c479c")
    AsyncIFtpHomeDirectoryProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_GetUserHomeDirectoryData( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetUserHomeDirectoryData( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszHomeDirectoryData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpHomeDirectoryProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpHomeDirectoryProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpHomeDirectoryProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpHomeDirectoryProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpHomeDirectoryProvider, Begin_GetUserHomeDirectoryData)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetUserHomeDirectoryData )( 
            __RPC__in AsyncIFtpHomeDirectoryProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName);
        
        DECLSPEC_XFGVIRT(AsyncIFtpHomeDirectoryProvider, Finish_GetUserHomeDirectoryData)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetUserHomeDirectoryData )( 
            __RPC__in AsyncIFtpHomeDirectoryProvider * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszHomeDirectoryData);
        
        END_INTERFACE
    } AsyncIFtpHomeDirectoryProviderVtbl;

    interface AsyncIFtpHomeDirectoryProvider
    {
        CONST_VTBL struct AsyncIFtpHomeDirectoryProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpHomeDirectoryProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpHomeDirectoryProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpHomeDirectoryProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpHomeDirectoryProvider_Begin_GetUserHomeDirectoryData(This,pszSessionId,pszSiteName,pszUserName)	\
    ( (This)->lpVtbl -> Begin_GetUserHomeDirectoryData(This,pszSessionId,pszSiteName,pszUserName) ) 

#define AsyncIFtpHomeDirectoryProvider_Finish_GetUserHomeDirectoryData(This,ppszHomeDirectoryData)	\
    ( (This)->lpVtbl -> Finish_GetUserHomeDirectoryData(This,ppszHomeDirectoryData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpHomeDirectoryProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ftpext_0000_0004 */
/* [local] */ 

/* [uuid] */ struct  DECLSPEC_UUID("6c678262-fc37-406e-84e8-e9c6a5757cdc") LOGGING_PARAMETERS
    {
    LPCWSTR pszSessionId;
    LPCWSTR pszSiteName;
    LPCWSTR pszUserName;
    LPCWSTR pszHostName;
    LPCWSTR pszRemoteIpAddress;
    DWORD dwRemoteIpPort;
    LPCWSTR pszLocalIpAddress;
    DWORD dwLocalIpPort;
    ULONGLONG BytesSent;
    ULONGLONG BytesReceived;
    LPCWSTR pszCommand;
    LPCWSTR pszCommandParameters;
    LPCWSTR pszFullPath;
    DWORD dwElapsedMilliseconds;
    DWORD FtpStatus;
    DWORD FtpSubStatus;
    HRESULT hrStatus;
    LPCWSTR pszInformation;
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0004_v0_0_s_ifspec;

#ifndef __IFtpLogProvider_INTERFACE_DEFINED__
#define __IFtpLogProvider_INTERFACE_DEFINED__

/* interface IFtpLogProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpLogProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a18a94cc-8299-4408-816c-7c3baca1a40e")
    IFtpLogProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Log( 
            /* [in] */ __RPC__in const struct LOGGING_PARAMETERS *pLoggingParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpLogProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpLogProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpLogProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpLogProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpLogProvider, Log)
        HRESULT ( STDMETHODCALLTYPE *Log )( 
            __RPC__in IFtpLogProvider * This,
            /* [in] */ __RPC__in const struct LOGGING_PARAMETERS *pLoggingParameters);
        
        END_INTERFACE
    } IFtpLogProviderVtbl;

    interface IFtpLogProvider
    {
        CONST_VTBL struct IFtpLogProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpLogProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpLogProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpLogProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpLogProvider_Log(This,pLoggingParameters)	\
    ( (This)->lpVtbl -> Log(This,pLoggingParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpLogProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpLogProvider_INTERFACE_DEFINED__
#define __AsyncIFtpLogProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpLogProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpLogProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00a0ae46-2498-48b2-95e6-df678ed7d49f")
    AsyncIFtpLogProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_Log( 
            /* [in] */ __RPC__in const struct LOGGING_PARAMETERS *pLoggingParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Log( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpLogProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpLogProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpLogProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpLogProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpLogProvider, Begin_Log)
        HRESULT ( STDMETHODCALLTYPE *Begin_Log )( 
            __RPC__in AsyncIFtpLogProvider * This,
            /* [in] */ __RPC__in const struct LOGGING_PARAMETERS *pLoggingParameters);
        
        DECLSPEC_XFGVIRT(AsyncIFtpLogProvider, Finish_Log)
        HRESULT ( STDMETHODCALLTYPE *Finish_Log )( 
            __RPC__in AsyncIFtpLogProvider * This);
        
        END_INTERFACE
    } AsyncIFtpLogProviderVtbl;

    interface AsyncIFtpLogProvider
    {
        CONST_VTBL struct AsyncIFtpLogProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpLogProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpLogProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpLogProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpLogProvider_Begin_Log(This,pLoggingParameters)	\
    ( (This)->lpVtbl -> Begin_Log(This,pLoggingParameters) ) 

#define AsyncIFtpLogProvider_Finish_Log(This)	\
    ( (This)->lpVtbl -> Finish_Log(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpLogProvider_INTERFACE_DEFINED__ */


#ifndef __IFtpAuthorizationProvider_INTERFACE_DEFINED__
#define __IFtpAuthorizationProvider_INTERFACE_DEFINED__

/* interface IFtpAuthorizationProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 

typedef 
enum _FTP_ACCESS
    {
        FTP_ACCESS_NONE	= 0,
        FTP_ACCESS_READ	= ( FTP_ACCESS_NONE + 1 ) ,
        FTP_ACCESS_WRITE	= ( FTP_ACCESS_READ + 1 ) ,
        FTP_ACCESS_READ_WRITE	= ( FTP_ACCESS_WRITE + 1 ) 
    } 	FTP_ACCESS;


EXTERN_C const IID IID_IFtpAuthorizationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a50ae7a1-a35a-42b4-a4f3-f4f7057a05d1")
    IFtpAuthorizationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUserAccessPermission( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszVirtualPath,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [retval][out] */ __RPC__out FTP_ACCESS *pFtpAccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpAuthorizationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpAuthorizationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpAuthorizationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpAuthorizationProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpAuthorizationProvider, GetUserAccessPermission)
        HRESULT ( STDMETHODCALLTYPE *GetUserAccessPermission )( 
            __RPC__in IFtpAuthorizationProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszVirtualPath,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName,
            /* [retval][out] */ __RPC__out FTP_ACCESS *pFtpAccess);
        
        END_INTERFACE
    } IFtpAuthorizationProviderVtbl;

    interface IFtpAuthorizationProvider
    {
        CONST_VTBL struct IFtpAuthorizationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpAuthorizationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpAuthorizationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpAuthorizationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpAuthorizationProvider_GetUserAccessPermission(This,pszSessionId,pszSiteName,pszVirtualPath,pszUserName,pFtpAccess)	\
    ( (This)->lpVtbl -> GetUserAccessPermission(This,pszSessionId,pszSiteName,pszVirtualPath,pszUserName,pFtpAccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpAuthorizationProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpAuthorizationProvider_INTERFACE_DEFINED__
#define __AsyncIFtpAuthorizationProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpAuthorizationProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpAuthorizationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("860dc339-07e5-4a5c-9c61-8820cea012bc")
    AsyncIFtpAuthorizationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_GetUserAccessPermission( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszVirtualPath,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetUserAccessPermission( 
            /* [retval][out] */ __RPC__out FTP_ACCESS *pFtpAccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpAuthorizationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpAuthorizationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpAuthorizationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpAuthorizationProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpAuthorizationProvider, Begin_GetUserAccessPermission)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetUserAccessPermission )( 
            __RPC__in AsyncIFtpAuthorizationProvider * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSessionId,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSiteName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszVirtualPath,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName);
        
        DECLSPEC_XFGVIRT(AsyncIFtpAuthorizationProvider, Finish_GetUserAccessPermission)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetUserAccessPermission )( 
            __RPC__in AsyncIFtpAuthorizationProvider * This,
            /* [retval][out] */ __RPC__out FTP_ACCESS *pFtpAccess);
        
        END_INTERFACE
    } AsyncIFtpAuthorizationProviderVtbl;

    interface AsyncIFtpAuthorizationProvider
    {
        CONST_VTBL struct AsyncIFtpAuthorizationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpAuthorizationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpAuthorizationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpAuthorizationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpAuthorizationProvider_Begin_GetUserAccessPermission(This,pszSessionId,pszSiteName,pszVirtualPath,pszUserName)	\
    ( (This)->lpVtbl -> Begin_GetUserAccessPermission(This,pszSessionId,pszSiteName,pszVirtualPath,pszUserName) ) 

#define AsyncIFtpAuthorizationProvider_Finish_GetUserAccessPermission(This,pFtpAccess)	\
    ( (This)->lpVtbl -> Finish_GetUserAccessPermission(This,pFtpAccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpAuthorizationProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ftpext_0000_0006 */
/* [local] */ 

typedef /* [v1_enum][uuid] */  DECLSPEC_UUID("e95e698b-7505-4e49-8fb1-cb06dc26c096") 
enum _FTP_PROCESS_STATUS
    {
        FTP_PROCESS_CONTINUE	= 0,
        FTP_PROCESS_CLOSE_SESSION	= ( FTP_PROCESS_CONTINUE + 1 ) ,
        FTP_PROCESS_TERMINATE_SESSION	= ( FTP_PROCESS_CLOSE_SESSION + 1 ) ,
        FTP_PROCESS_REJECT_COMMAND	= ( FTP_PROCESS_TERMINATE_SESSION + 1 ) 
    } 	FTP_PROCESS_STATUS;

/* [uuid] */ struct  DECLSPEC_UUID("07c3d591-cead-4702-abab-a70886af38a3") PRE_PROCESS_PARAMETERS
    {
    LPCWSTR pszSessionId;
    LPCWSTR pszSiteName;
    LPCWSTR pszUserName;
    LPCWSTR pszHostName;
    LPCWSTR pszRemoteIpAddress;
    DWORD dwRemoteIpPort;
    LPCWSTR pszLocalIpAddress;
    DWORD dwLocalIpPort;
    LPCWSTR pszCommand;
    LPCWSTR pszCommandParameters;
    FILETIME SessionStartTime;
    ULONGLONG BytesSentPerSession;
    ULONGLONG BytesReceivedPerSession;
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0006_v0_0_s_ifspec;

#ifndef __IFtpPreprocessProvider_INTERFACE_DEFINED__
#define __IFtpPreprocessProvider_INTERFACE_DEFINED__

/* interface IFtpPreprocessProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpPreprocessProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a3c19b60-5a28-471a-8f93-ab30411cee82")
    IFtpPreprocessProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandlePreprocess( 
            /* [in] */ __RPC__in const struct PRE_PROCESS_PARAMETERS *pPreProcessParameters,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpPreprocessProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpPreprocessProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpPreprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpPreprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpPreprocessProvider, HandlePreprocess)
        HRESULT ( STDMETHODCALLTYPE *HandlePreprocess )( 
            __RPC__in IFtpPreprocessProvider * This,
            /* [in] */ __RPC__in const struct PRE_PROCESS_PARAMETERS *pPreProcessParameters,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus);
        
        END_INTERFACE
    } IFtpPreprocessProviderVtbl;

    interface IFtpPreprocessProvider
    {
        CONST_VTBL struct IFtpPreprocessProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpPreprocessProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpPreprocessProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpPreprocessProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpPreprocessProvider_HandlePreprocess(This,pPreProcessParameters,pFtpProcessStatus)	\
    ( (This)->lpVtbl -> HandlePreprocess(This,pPreProcessParameters,pFtpProcessStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpPreprocessProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpPreprocessProvider_INTERFACE_DEFINED__
#define __AsyncIFtpPreprocessProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpPreprocessProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpPreprocessProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ff5fd8f-fd8e-48b1-a3e0-bf7073db4db5")
    AsyncIFtpPreprocessProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_HandlePreprocess( 
            /* [in] */ __RPC__in const struct PRE_PROCESS_PARAMETERS *pPreProcessParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_HandlePreprocess( 
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpPreprocessProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpPreprocessProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpPreprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpPreprocessProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpPreprocessProvider, Begin_HandlePreprocess)
        HRESULT ( STDMETHODCALLTYPE *Begin_HandlePreprocess )( 
            __RPC__in AsyncIFtpPreprocessProvider * This,
            /* [in] */ __RPC__in const struct PRE_PROCESS_PARAMETERS *pPreProcessParameters);
        
        DECLSPEC_XFGVIRT(AsyncIFtpPreprocessProvider, Finish_HandlePreprocess)
        HRESULT ( STDMETHODCALLTYPE *Finish_HandlePreprocess )( 
            __RPC__in AsyncIFtpPreprocessProvider * This,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus);
        
        END_INTERFACE
    } AsyncIFtpPreprocessProviderVtbl;

    interface AsyncIFtpPreprocessProvider
    {
        CONST_VTBL struct AsyncIFtpPreprocessProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpPreprocessProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpPreprocessProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpPreprocessProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpPreprocessProvider_Begin_HandlePreprocess(This,pPreProcessParameters)	\
    ( (This)->lpVtbl -> Begin_HandlePreprocess(This,pPreProcessParameters) ) 

#define AsyncIFtpPreprocessProvider_Finish_HandlePreprocess(This,pFtpProcessStatus)	\
    ( (This)->lpVtbl -> Finish_HandlePreprocess(This,pFtpProcessStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpPreprocessProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ftpext_0000_0007 */
/* [local] */ 

/* [uuid] */ struct  DECLSPEC_UUID("53337595-9165-4a8b-a202-7d5dbf7e4b8b") POST_PROCESS_PARAMETERS
    {
    LPCWSTR pszSessionId;
    LPCWSTR pszSiteName;
    LPCWSTR pszUserName;
    LPCWSTR pszHostName;
    LPCWSTR pszRemoteIpAddress;
    DWORD dwRemoteIpPort;
    LPCWSTR pszLocalIpAddress;
    DWORD dwLocalIpPort;
    ULONGLONG BytesSent;
    ULONGLONG BytesReceived;
    LPCWSTR pszCommand;
    LPCWSTR pszCommandParameters;
    LPCWSTR pszFullPath;
    LPCWSTR pszPhysicalPath;
    DWORD FtpStatus;
    DWORD FtpSubStatus;
    HRESULT hrStatus;
    FILETIME SessionStartTime;
    ULONGLONG BytesSentPerSession;
    ULONGLONG BytesReceivedPerSession;
    } ;


extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0007_v0_0_s_ifspec;

#ifndef __IFtpPostprocessProvider_INTERFACE_DEFINED__
#define __IFtpPostprocessProvider_INTERFACE_DEFINED__

/* interface IFtpPostprocessProvider */
/* [helpstring][unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IFtpPostprocessProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4522cbc6-16cd-49ad-8653-9a2c579e4280")
    IFtpPostprocessProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandlePostprocess( 
            /* [in] */ __RPC__in const struct POST_PROCESS_PARAMETERS *pPostProcessParameters,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFtpPostprocessProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFtpPostprocessProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFtpPostprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFtpPostprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IFtpPostprocessProvider, HandlePostprocess)
        HRESULT ( STDMETHODCALLTYPE *HandlePostprocess )( 
            __RPC__in IFtpPostprocessProvider * This,
            /* [in] */ __RPC__in const struct POST_PROCESS_PARAMETERS *pPostProcessParameters,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus);
        
        END_INTERFACE
    } IFtpPostprocessProviderVtbl;

    interface IFtpPostprocessProvider
    {
        CONST_VTBL struct IFtpPostprocessProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFtpPostprocessProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFtpPostprocessProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFtpPostprocessProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFtpPostprocessProvider_HandlePostprocess(This,pPostProcessParameters,pFtpProcessStatus)	\
    ( (This)->lpVtbl -> HandlePostprocess(This,pPostProcessParameters,pFtpProcessStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFtpPostprocessProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIFtpPostprocessProvider_INTERFACE_DEFINED__
#define __AsyncIFtpPostprocessProvider_INTERFACE_DEFINED__

/* interface AsyncIFtpPostprocessProvider */
/* [uuid][helpstring][unique][object] */ 


EXTERN_C const IID IID_AsyncIFtpPostprocessProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a16b2542-9694-4eb1-a564-6c2e91fdc133")
    AsyncIFtpPostprocessProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_HandlePostprocess( 
            /* [in] */ __RPC__in const struct POST_PROCESS_PARAMETERS *pPostProcessParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_HandlePostprocess( 
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIFtpPostprocessProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIFtpPostprocessProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIFtpPostprocessProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIFtpPostprocessProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIFtpPostprocessProvider, Begin_HandlePostprocess)
        HRESULT ( STDMETHODCALLTYPE *Begin_HandlePostprocess )( 
            __RPC__in AsyncIFtpPostprocessProvider * This,
            /* [in] */ __RPC__in const struct POST_PROCESS_PARAMETERS *pPostProcessParameters);
        
        DECLSPEC_XFGVIRT(AsyncIFtpPostprocessProvider, Finish_HandlePostprocess)
        HRESULT ( STDMETHODCALLTYPE *Finish_HandlePostprocess )( 
            __RPC__in AsyncIFtpPostprocessProvider * This,
            /* [retval][out] */ __RPC__out FTP_PROCESS_STATUS *pFtpProcessStatus);
        
        END_INTERFACE
    } AsyncIFtpPostprocessProviderVtbl;

    interface AsyncIFtpPostprocessProvider
    {
        CONST_VTBL struct AsyncIFtpPostprocessProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIFtpPostprocessProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIFtpPostprocessProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIFtpPostprocessProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIFtpPostprocessProvider_Begin_HandlePostprocess(This,pPostProcessParameters)	\
    ( (This)->lpVtbl -> Begin_HandlePostprocess(This,pPostProcessParameters) ) 

#define AsyncIFtpPostprocessProvider_Finish_HandlePostprocess(This,pFtpProcessStatus)	\
    ( (This)->lpVtbl -> Finish_HandlePostprocess(This,pFtpProcessStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIFtpPostprocessProvider_INTERFACE_DEFINED__ */



#ifndef __FtpProviderLibrary_LIBRARY_DEFINED__
#define __FtpProviderLibrary_LIBRARY_DEFINED__

/* library FtpProviderLibrary */
/* [helpstring][version][uuid] */ 






EXTERN_C const IID LIBID_FtpProviderLibrary;

EXTERN_C const CLSID CLSID_FtpProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("70bdc667-33b2-45f0-ac52-c3ca46f7a656")
FtpProvider;
#endif
#endif /* __FtpProviderLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_ftpext_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ftpext_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


