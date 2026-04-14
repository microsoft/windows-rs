

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

#ifndef __wsmandisp_h__
#define __wsmandisp_h__

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

#ifndef __IWSMan_FWD_DEFINED__
#define __IWSMan_FWD_DEFINED__
typedef interface IWSMan IWSMan;

#endif 	/* __IWSMan_FWD_DEFINED__ */


#ifndef __IWSManEx_FWD_DEFINED__
#define __IWSManEx_FWD_DEFINED__
typedef interface IWSManEx IWSManEx;

#endif 	/* __IWSManEx_FWD_DEFINED__ */


#ifndef __IWSManEx2_FWD_DEFINED__
#define __IWSManEx2_FWD_DEFINED__
typedef interface IWSManEx2 IWSManEx2;

#endif 	/* __IWSManEx2_FWD_DEFINED__ */


#ifndef __IWSManEx3_FWD_DEFINED__
#define __IWSManEx3_FWD_DEFINED__
typedef interface IWSManEx3 IWSManEx3;

#endif 	/* __IWSManEx3_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptions_FWD_DEFINED__
#define __IWSManConnectionOptions_FWD_DEFINED__
typedef interface IWSManConnectionOptions IWSManConnectionOptions;

#endif 	/* __IWSManConnectionOptions_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx_FWD_DEFINED__
#define __IWSManConnectionOptionsEx_FWD_DEFINED__
typedef interface IWSManConnectionOptionsEx IWSManConnectionOptionsEx;

#endif 	/* __IWSManConnectionOptionsEx_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx2_FWD_DEFINED__
#define __IWSManConnectionOptionsEx2_FWD_DEFINED__
typedef interface IWSManConnectionOptionsEx2 IWSManConnectionOptionsEx2;

#endif 	/* __IWSManConnectionOptionsEx2_FWD_DEFINED__ */


#ifndef __IWSManSession_FWD_DEFINED__
#define __IWSManSession_FWD_DEFINED__
typedef interface IWSManSession IWSManSession;

#endif 	/* __IWSManSession_FWD_DEFINED__ */


#ifndef __IWSManEnumerator_FWD_DEFINED__
#define __IWSManEnumerator_FWD_DEFINED__
typedef interface IWSManEnumerator IWSManEnumerator;

#endif 	/* __IWSManEnumerator_FWD_DEFINED__ */


#ifndef __IWSManResourceLocator_FWD_DEFINED__
#define __IWSManResourceLocator_FWD_DEFINED__
typedef interface IWSManResourceLocator IWSManResourceLocator;

#endif 	/* __IWSManResourceLocator_FWD_DEFINED__ */


#ifndef __IWSManResourceLocatorInternal_FWD_DEFINED__
#define __IWSManResourceLocatorInternal_FWD_DEFINED__
typedef interface IWSManResourceLocatorInternal IWSManResourceLocatorInternal;

#endif 	/* __IWSManResourceLocatorInternal_FWD_DEFINED__ */


#ifndef __WSMan_FWD_DEFINED__
#define __WSMan_FWD_DEFINED__

#ifdef __cplusplus
typedef class WSMan WSMan;
#else
typedef struct WSMan WSMan;
#endif /* __cplusplus */

#endif 	/* __WSMan_FWD_DEFINED__ */


#ifndef __IWSManInternal_FWD_DEFINED__
#define __IWSManInternal_FWD_DEFINED__
typedef interface IWSManInternal IWSManInternal;

#endif 	/* __IWSManInternal_FWD_DEFINED__ */


#ifndef __WSManInternal_FWD_DEFINED__
#define __WSManInternal_FWD_DEFINED__

#ifdef __cplusplus
typedef class WSManInternal WSManInternal;
#else
typedef struct WSManInternal WSManInternal;
#endif /* __cplusplus */

#endif 	/* __WSManInternal_FWD_DEFINED__ */


#ifndef __IWSMan_FWD_DEFINED__
#define __IWSMan_FWD_DEFINED__
typedef interface IWSMan IWSMan;

#endif 	/* __IWSMan_FWD_DEFINED__ */


#ifndef __IWSManEx_FWD_DEFINED__
#define __IWSManEx_FWD_DEFINED__
typedef interface IWSManEx IWSManEx;

#endif 	/* __IWSManEx_FWD_DEFINED__ */


#ifndef __IWSManEx2_FWD_DEFINED__
#define __IWSManEx2_FWD_DEFINED__
typedef interface IWSManEx2 IWSManEx2;

#endif 	/* __IWSManEx2_FWD_DEFINED__ */


#ifndef __IWSManEx3_FWD_DEFINED__
#define __IWSManEx3_FWD_DEFINED__
typedef interface IWSManEx3 IWSManEx3;

#endif 	/* __IWSManEx3_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptions_FWD_DEFINED__
#define __IWSManConnectionOptions_FWD_DEFINED__
typedef interface IWSManConnectionOptions IWSManConnectionOptions;

#endif 	/* __IWSManConnectionOptions_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx_FWD_DEFINED__
#define __IWSManConnectionOptionsEx_FWD_DEFINED__
typedef interface IWSManConnectionOptionsEx IWSManConnectionOptionsEx;

#endif 	/* __IWSManConnectionOptionsEx_FWD_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx2_FWD_DEFINED__
#define __IWSManConnectionOptionsEx2_FWD_DEFINED__
typedef interface IWSManConnectionOptionsEx2 IWSManConnectionOptionsEx2;

#endif 	/* __IWSManConnectionOptionsEx2_FWD_DEFINED__ */


#ifndef __IWSManSession_FWD_DEFINED__
#define __IWSManSession_FWD_DEFINED__
typedef interface IWSManSession IWSManSession;

#endif 	/* __IWSManSession_FWD_DEFINED__ */


#ifndef __IWSManEnumerator_FWD_DEFINED__
#define __IWSManEnumerator_FWD_DEFINED__
typedef interface IWSManEnumerator IWSManEnumerator;

#endif 	/* __IWSManEnumerator_FWD_DEFINED__ */


#ifndef __IWSManResourceLocator_FWD_DEFINED__
#define __IWSManResourceLocator_FWD_DEFINED__
typedef interface IWSManResourceLocator IWSManResourceLocator;

#endif 	/* __IWSManResourceLocator_FWD_DEFINED__ */


#ifndef __IWSManResourceLocatorInternal_FWD_DEFINED__
#define __IWSManResourceLocatorInternal_FWD_DEFINED__
typedef interface IWSManResourceLocatorInternal IWSManResourceLocatorInternal;

#endif 	/* __IWSManResourceLocatorInternal_FWD_DEFINED__ */


#ifndef __IWSManInternal_FWD_DEFINED__
#define __IWSManInternal_FWD_DEFINED__
typedef interface IWSManInternal IWSManInternal;

#endif 	/* __IWSManInternal_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wsmandisp_0000_0000 */
/* [local] */ 

/***************************************************************/
/*                                                             */
/*    Copyright (C) Microsoft Corporation.  All rights reserved. */
/*                                                             */
/*    wsmandisp.idl                                              */
/*                                                             */
/*    WSMAN IDispatch Compatible Access for Automation clients.  */
/*                                                             */
/***************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


extern RPC_IF_HANDLE __MIDL_itf_wsmandisp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsmandisp_0000_0000_v0_0_s_ifspec;


#ifndef __WSManAutomation_LIBRARY_DEFINED__
#define __WSManAutomation_LIBRARY_DEFINED__

/* library WSManAutomation */
/* [helpstring][version][lcid][uuid] */ 

typedef /* [v1_enum] */ 
enum _WSManSessionFlags
    {
        WSManFlagUTF8	= 0x1,
        WSManFlagCredUsernamePassword	= 0x1000,
        WSManFlagSkipCACheck	= 0x2000,
        WSManFlagSkipCNCheck	= 0x4000,
        WSManFlagUseNoAuthentication	= 0x8000,
        WSManFlagUseDigest	= 0x10000,
        WSManFlagUseNegotiate	= 0x20000,
        WSManFlagUseBasic	= 0x40000,
        WSManFlagUseKerberos	= 0x80000,
        WSManFlagNoEncryption	= 0x100000,
        WSManFlagUseClientCertificate	= 0x200000,
        WSManFlagEnableSPNServerPort	= 0x400000,
        WSManFlagUTF16	= 0x800000,
        WSManFlagUseCredSsp	= 0x1000000,
        WSManFlagSkipRevocationCheck	= 0x2000000,
        WSManFlagAllowNegotiateImplicitCredentials	= 0x4000000,
        WSManFlagUseSsl	= 0x8000000
    } 	WSManSessionFlags;

typedef /* [v1_enum] */ 
enum _WSManEnumFlags
    {
        WSManFlagNonXmlText	= 0x1,
        WSManFlagReturnObject	= 0,
        WSManFlagReturnEPR	= 0x2,
        WSManFlagReturnObjectAndEPR	= 0x4,
        WSManFlagHierarchyDeep	= 0,
        WSManFlagHierarchyShallow	= 0x20,
        WSManFlagHierarchyDeepBasePropsOnly	= 0x40,
        WSManFlagAssociatedInstance	= 0,
        WSManFlagAssociationInstance	= 0x80
    } 	WSManEnumFlags;

typedef /* [v1_enum] */ 
enum _WSManProxyAccessTypeFlags
    {
        WSManProxyIEConfig	= 0x1,
        WSManProxyWinHttpConfig	= 0x2,
        WSManProxyAutoDetect	= 0x4,
        WSManProxyNoProxyServer	= 0x8
    } 	WSManProxyAccessTypeFlags;

typedef /* [v1_enum] */ 
enum _WSManProxyAuthenticationFlags
    {
        WSManFlagProxyAuthenticationUseNegotiate	= 0x1,
        WSManFlagProxyAuthenticationUseBasic	= 0x2,
        WSManFlagProxyAuthenticationUseDigest	= 0x4
    } 	WSManProxyAuthenticationFlags;















EXTERN_C const IID LIBID_WSManAutomation;

#ifndef __IWSMan_INTERFACE_DEFINED__
#define __IWSMan_INTERFACE_DEFINED__

/* interface IWSMan */
/* [hidden][nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSMan;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("190D8637-5CD3-496d-AD24-69636BB5A3B5")
    IWSMan : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateSession( 
            /* [defaultvalue][in] */ BSTR connection,
            /* [defaultvalue][in] */ long flags,
            /* [defaultvalue][in] */ IDispatch *connectionOptions,
            /* [retval][out] */ IDispatch **session) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateConnectionOptions( 
            /* [retval][out] */ IDispatch **connectionOptions) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_CommandLine( 
            /* [retval][out] */ BSTR *value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ BSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSMan * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSMan * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSMan * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSMan * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSMan * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSMan * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSMan * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateSession)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IWSMan * This,
            /* [defaultvalue][in] */ BSTR connection,
            /* [defaultvalue][in] */ long flags,
            /* [defaultvalue][in] */ IDispatch *connectionOptions,
            /* [retval][out] */ IDispatch **session);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateConnectionOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateConnectionOptions )( 
            IWSMan * This,
            /* [retval][out] */ IDispatch **connectionOptions);
        
        DECLSPEC_XFGVIRT(IWSMan, get_CommandLine)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandLine )( 
            IWSMan * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSMan, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSMan * This,
            /* [retval][out] */ BSTR *value);
        
        END_INTERFACE
    } IWSManVtbl;

    interface IWSMan
    {
        CONST_VTBL struct IWSManVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSMan_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSMan_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSMan_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSMan_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSMan_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSMan_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSMan_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSMan_CreateSession(This,connection,flags,connectionOptions,session)	\
    ( (This)->lpVtbl -> CreateSession(This,connection,flags,connectionOptions,session) ) 

#define IWSMan_CreateConnectionOptions(This,connectionOptions)	\
    ( (This)->lpVtbl -> CreateConnectionOptions(This,connectionOptions) ) 

#define IWSMan_get_CommandLine(This,value)	\
    ( (This)->lpVtbl -> get_CommandLine(This,value) ) 

#define IWSMan_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSMan_INTERFACE_DEFINED__ */


#ifndef __IWSManEx_INTERFACE_DEFINED__
#define __IWSManEx_INTERFACE_DEFINED__

/* interface IWSManEx */
/* [hidden][nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D53BDAA-798E-49e6-A1AA-74D01256F411")
    IWSManEx : public IWSMan
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateResourceLocator( 
            /* [defaultvalue][in] */ BSTR strResourceLocator,
            /* [retval][out] */ IDispatch **newResourceLocator) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUTF8( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagCredUsernamePassword( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagSkipCACheck( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagSkipCNCheck( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseDigest( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseNegotiate( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseBasic( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseKerberos( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagNoEncryption( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagEnableSPNServerPort( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseNoAuthentication( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagNonXmlText( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagReturnEPR( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagReturnObjectAndEPR( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetErrorMessage( 
            /* [in] */ DWORD errorNumber,
            /* [retval][out] */ BSTR *errorMessage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagHierarchyDeep( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagHierarchyShallow( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagHierarchyDeepBasePropsOnly( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagReturnObject( 
            /* [retval][out] */ long *flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManEx * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManEx * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManEx * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateSession)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IWSManEx * This,
            /* [defaultvalue][in] */ BSTR connection,
            /* [defaultvalue][in] */ long flags,
            /* [defaultvalue][in] */ IDispatch *connectionOptions,
            /* [retval][out] */ IDispatch **session);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateConnectionOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateConnectionOptions )( 
            IWSManEx * This,
            /* [retval][out] */ IDispatch **connectionOptions);
        
        DECLSPEC_XFGVIRT(IWSMan, get_CommandLine)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandLine )( 
            IWSManEx * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSMan, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManEx * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSManEx, CreateResourceLocator)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateResourceLocator )( 
            IWSManEx * This,
            /* [defaultvalue][in] */ BSTR strResourceLocator,
            /* [retval][out] */ IDispatch **newResourceLocator);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUTF8)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUTF8 )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagCredUsernamePassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagCredUsernamePassword )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCACheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCACheck )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCNCheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCNCheck )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseDigest)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseDigest )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNegotiate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNegotiate )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseBasic)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseBasic )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseKerberos)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseKerberos )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagNoEncryption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagNoEncryption )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagEnableSPNServerPort)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagEnableSPNServerPort )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNoAuthentication)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNoAuthentication )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagNonXmlText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagNonXmlText )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnEPR )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObjectAndEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObjectAndEPR )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, GetErrorMessage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetErrorMessage )( 
            IWSManEx * This,
            /* [in] */ DWORD errorNumber,
            /* [retval][out] */ BSTR *errorMessage);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeep)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeep )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyShallow)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyShallow )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeepBasePropsOnly)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeepBasePropsOnly )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObject )( 
            IWSManEx * This,
            /* [retval][out] */ long *flags);
        
        END_INTERFACE
    } IWSManExVtbl;

    interface IWSManEx
    {
        CONST_VTBL struct IWSManExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManEx_CreateSession(This,connection,flags,connectionOptions,session)	\
    ( (This)->lpVtbl -> CreateSession(This,connection,flags,connectionOptions,session) ) 

#define IWSManEx_CreateConnectionOptions(This,connectionOptions)	\
    ( (This)->lpVtbl -> CreateConnectionOptions(This,connectionOptions) ) 

#define IWSManEx_get_CommandLine(This,value)	\
    ( (This)->lpVtbl -> get_CommandLine(This,value) ) 

#define IWSManEx_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 


#define IWSManEx_CreateResourceLocator(This,strResourceLocator,newResourceLocator)	\
    ( (This)->lpVtbl -> CreateResourceLocator(This,strResourceLocator,newResourceLocator) ) 

#define IWSManEx_SessionFlagUTF8(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUTF8(This,flags) ) 

#define IWSManEx_SessionFlagCredUsernamePassword(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagCredUsernamePassword(This,flags) ) 

#define IWSManEx_SessionFlagSkipCACheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCACheck(This,flags) ) 

#define IWSManEx_SessionFlagSkipCNCheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCNCheck(This,flags) ) 

#define IWSManEx_SessionFlagUseDigest(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseDigest(This,flags) ) 

#define IWSManEx_SessionFlagUseNegotiate(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNegotiate(This,flags) ) 

#define IWSManEx_SessionFlagUseBasic(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseBasic(This,flags) ) 

#define IWSManEx_SessionFlagUseKerberos(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseKerberos(This,flags) ) 

#define IWSManEx_SessionFlagNoEncryption(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagNoEncryption(This,flags) ) 

#define IWSManEx_SessionFlagEnableSPNServerPort(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagEnableSPNServerPort(This,flags) ) 

#define IWSManEx_SessionFlagUseNoAuthentication(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNoAuthentication(This,flags) ) 

#define IWSManEx_EnumerationFlagNonXmlText(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagNonXmlText(This,flags) ) 

#define IWSManEx_EnumerationFlagReturnEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnEPR(This,flags) ) 

#define IWSManEx_EnumerationFlagReturnObjectAndEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObjectAndEPR(This,flags) ) 

#define IWSManEx_GetErrorMessage(This,errorNumber,errorMessage)	\
    ( (This)->lpVtbl -> GetErrorMessage(This,errorNumber,errorMessage) ) 

#define IWSManEx_EnumerationFlagHierarchyDeep(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeep(This,flags) ) 

#define IWSManEx_EnumerationFlagHierarchyShallow(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyShallow(This,flags) ) 

#define IWSManEx_EnumerationFlagHierarchyDeepBasePropsOnly(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeepBasePropsOnly(This,flags) ) 

#define IWSManEx_EnumerationFlagReturnObject(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObject(This,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManEx_INTERFACE_DEFINED__ */


#ifndef __IWSManEx2_INTERFACE_DEFINED__
#define __IWSManEx2_INTERFACE_DEFINED__

/* interface IWSManEx2 */
/* [hidden][nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManEx2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1D1B5AE0-42D9-4021-8261-3987619512E9")
    IWSManEx2 : public IWSManEx
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseClientCertificate( 
            /* [retval][out] */ long *flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManEx2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManEx2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManEx2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManEx2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManEx2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManEx2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManEx2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManEx2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateSession)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IWSManEx2 * This,
            /* [defaultvalue][in] */ BSTR connection,
            /* [defaultvalue][in] */ long flags,
            /* [defaultvalue][in] */ IDispatch *connectionOptions,
            /* [retval][out] */ IDispatch **session);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateConnectionOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateConnectionOptions )( 
            IWSManEx2 * This,
            /* [retval][out] */ IDispatch **connectionOptions);
        
        DECLSPEC_XFGVIRT(IWSMan, get_CommandLine)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandLine )( 
            IWSManEx2 * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSMan, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManEx2 * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSManEx, CreateResourceLocator)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateResourceLocator )( 
            IWSManEx2 * This,
            /* [defaultvalue][in] */ BSTR strResourceLocator,
            /* [retval][out] */ IDispatch **newResourceLocator);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUTF8)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUTF8 )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagCredUsernamePassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagCredUsernamePassword )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCACheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCACheck )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCNCheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCNCheck )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseDigest)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseDigest )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNegotiate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNegotiate )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseBasic)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseBasic )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseKerberos)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseKerberos )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagNoEncryption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagNoEncryption )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagEnableSPNServerPort)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagEnableSPNServerPort )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNoAuthentication)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNoAuthentication )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagNonXmlText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagNonXmlText )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnEPR )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObjectAndEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObjectAndEPR )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, GetErrorMessage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetErrorMessage )( 
            IWSManEx2 * This,
            /* [in] */ DWORD errorNumber,
            /* [retval][out] */ BSTR *errorMessage);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeep)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeep )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyShallow)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyShallow )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeepBasePropsOnly)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeepBasePropsOnly )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObject )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx2, SessionFlagUseClientCertificate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseClientCertificate )( 
            IWSManEx2 * This,
            /* [retval][out] */ long *flags);
        
        END_INTERFACE
    } IWSManEx2Vtbl;

    interface IWSManEx2
    {
        CONST_VTBL struct IWSManEx2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManEx2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManEx2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManEx2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManEx2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManEx2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManEx2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManEx2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManEx2_CreateSession(This,connection,flags,connectionOptions,session)	\
    ( (This)->lpVtbl -> CreateSession(This,connection,flags,connectionOptions,session) ) 

#define IWSManEx2_CreateConnectionOptions(This,connectionOptions)	\
    ( (This)->lpVtbl -> CreateConnectionOptions(This,connectionOptions) ) 

#define IWSManEx2_get_CommandLine(This,value)	\
    ( (This)->lpVtbl -> get_CommandLine(This,value) ) 

#define IWSManEx2_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 


#define IWSManEx2_CreateResourceLocator(This,strResourceLocator,newResourceLocator)	\
    ( (This)->lpVtbl -> CreateResourceLocator(This,strResourceLocator,newResourceLocator) ) 

#define IWSManEx2_SessionFlagUTF8(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUTF8(This,flags) ) 

#define IWSManEx2_SessionFlagCredUsernamePassword(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagCredUsernamePassword(This,flags) ) 

#define IWSManEx2_SessionFlagSkipCACheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCACheck(This,flags) ) 

#define IWSManEx2_SessionFlagSkipCNCheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCNCheck(This,flags) ) 

#define IWSManEx2_SessionFlagUseDigest(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseDigest(This,flags) ) 

#define IWSManEx2_SessionFlagUseNegotiate(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNegotiate(This,flags) ) 

#define IWSManEx2_SessionFlagUseBasic(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseBasic(This,flags) ) 

#define IWSManEx2_SessionFlagUseKerberos(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseKerberos(This,flags) ) 

#define IWSManEx2_SessionFlagNoEncryption(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagNoEncryption(This,flags) ) 

#define IWSManEx2_SessionFlagEnableSPNServerPort(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagEnableSPNServerPort(This,flags) ) 

#define IWSManEx2_SessionFlagUseNoAuthentication(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNoAuthentication(This,flags) ) 

#define IWSManEx2_EnumerationFlagNonXmlText(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagNonXmlText(This,flags) ) 

#define IWSManEx2_EnumerationFlagReturnEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnEPR(This,flags) ) 

#define IWSManEx2_EnumerationFlagReturnObjectAndEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObjectAndEPR(This,flags) ) 

#define IWSManEx2_GetErrorMessage(This,errorNumber,errorMessage)	\
    ( (This)->lpVtbl -> GetErrorMessage(This,errorNumber,errorMessage) ) 

#define IWSManEx2_EnumerationFlagHierarchyDeep(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeep(This,flags) ) 

#define IWSManEx2_EnumerationFlagHierarchyShallow(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyShallow(This,flags) ) 

#define IWSManEx2_EnumerationFlagHierarchyDeepBasePropsOnly(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeepBasePropsOnly(This,flags) ) 

#define IWSManEx2_EnumerationFlagReturnObject(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObject(This,flags) ) 


#define IWSManEx2_SessionFlagUseClientCertificate(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseClientCertificate(This,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManEx2_INTERFACE_DEFINED__ */


#ifndef __IWSManEx3_INTERFACE_DEFINED__
#define __IWSManEx3_INTERFACE_DEFINED__

/* interface IWSManEx3 */
/* [hidden][nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManEx3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6400E966-011D-4eac-8474-049E0848AFAD")
    IWSManEx3 : public IWSManEx2
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUTF16( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseCredSsp( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagAssociationInstance( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumerationFlagAssociatedInstance( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagSkipRevocationCheck( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagAllowNegotiateImplicitCredentials( 
            /* [retval][out] */ long *flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionFlagUseSsl( 
            /* [retval][out] */ long *flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManEx3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManEx3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManEx3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManEx3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManEx3 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManEx3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManEx3 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManEx3 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateSession)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IWSManEx3 * This,
            /* [defaultvalue][in] */ BSTR connection,
            /* [defaultvalue][in] */ long flags,
            /* [defaultvalue][in] */ IDispatch *connectionOptions,
            /* [retval][out] */ IDispatch **session);
        
        DECLSPEC_XFGVIRT(IWSMan, CreateConnectionOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateConnectionOptions )( 
            IWSManEx3 * This,
            /* [retval][out] */ IDispatch **connectionOptions);
        
        DECLSPEC_XFGVIRT(IWSMan, get_CommandLine)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandLine )( 
            IWSManEx3 * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSMan, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManEx3 * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSManEx, CreateResourceLocator)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateResourceLocator )( 
            IWSManEx3 * This,
            /* [defaultvalue][in] */ BSTR strResourceLocator,
            /* [retval][out] */ IDispatch **newResourceLocator);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUTF8)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUTF8 )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagCredUsernamePassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagCredUsernamePassword )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCACheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCACheck )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagSkipCNCheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipCNCheck )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseDigest)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseDigest )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNegotiate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNegotiate )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseBasic)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseBasic )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseKerberos)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseKerberos )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagNoEncryption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagNoEncryption )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagEnableSPNServerPort)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagEnableSPNServerPort )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, SessionFlagUseNoAuthentication)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseNoAuthentication )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagNonXmlText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagNonXmlText )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnEPR )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObjectAndEPR)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObjectAndEPR )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, GetErrorMessage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetErrorMessage )( 
            IWSManEx3 * This,
            /* [in] */ DWORD errorNumber,
            /* [retval][out] */ BSTR *errorMessage);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeep)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeep )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyShallow)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyShallow )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagHierarchyDeepBasePropsOnly)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagHierarchyDeepBasePropsOnly )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx, EnumerationFlagReturnObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagReturnObject )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx2, SessionFlagUseClientCertificate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseClientCertificate )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, SessionFlagUTF16)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUTF16 )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, SessionFlagUseCredSsp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseCredSsp )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, EnumerationFlagAssociationInstance)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagAssociationInstance )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, EnumerationFlagAssociatedInstance)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumerationFlagAssociatedInstance )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, SessionFlagSkipRevocationCheck)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagSkipRevocationCheck )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, SessionFlagAllowNegotiateImplicitCredentials)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagAllowNegotiateImplicitCredentials )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        DECLSPEC_XFGVIRT(IWSManEx3, SessionFlagUseSsl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionFlagUseSsl )( 
            IWSManEx3 * This,
            /* [retval][out] */ long *flags);
        
        END_INTERFACE
    } IWSManEx3Vtbl;

    interface IWSManEx3
    {
        CONST_VTBL struct IWSManEx3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManEx3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManEx3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManEx3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManEx3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManEx3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManEx3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManEx3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManEx3_CreateSession(This,connection,flags,connectionOptions,session)	\
    ( (This)->lpVtbl -> CreateSession(This,connection,flags,connectionOptions,session) ) 

#define IWSManEx3_CreateConnectionOptions(This,connectionOptions)	\
    ( (This)->lpVtbl -> CreateConnectionOptions(This,connectionOptions) ) 

#define IWSManEx3_get_CommandLine(This,value)	\
    ( (This)->lpVtbl -> get_CommandLine(This,value) ) 

#define IWSManEx3_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 


#define IWSManEx3_CreateResourceLocator(This,strResourceLocator,newResourceLocator)	\
    ( (This)->lpVtbl -> CreateResourceLocator(This,strResourceLocator,newResourceLocator) ) 

#define IWSManEx3_SessionFlagUTF8(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUTF8(This,flags) ) 

#define IWSManEx3_SessionFlagCredUsernamePassword(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagCredUsernamePassword(This,flags) ) 

#define IWSManEx3_SessionFlagSkipCACheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCACheck(This,flags) ) 

#define IWSManEx3_SessionFlagSkipCNCheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipCNCheck(This,flags) ) 

#define IWSManEx3_SessionFlagUseDigest(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseDigest(This,flags) ) 

#define IWSManEx3_SessionFlagUseNegotiate(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNegotiate(This,flags) ) 

#define IWSManEx3_SessionFlagUseBasic(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseBasic(This,flags) ) 

#define IWSManEx3_SessionFlagUseKerberos(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseKerberos(This,flags) ) 

#define IWSManEx3_SessionFlagNoEncryption(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagNoEncryption(This,flags) ) 

#define IWSManEx3_SessionFlagEnableSPNServerPort(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagEnableSPNServerPort(This,flags) ) 

#define IWSManEx3_SessionFlagUseNoAuthentication(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseNoAuthentication(This,flags) ) 

#define IWSManEx3_EnumerationFlagNonXmlText(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagNonXmlText(This,flags) ) 

#define IWSManEx3_EnumerationFlagReturnEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnEPR(This,flags) ) 

#define IWSManEx3_EnumerationFlagReturnObjectAndEPR(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObjectAndEPR(This,flags) ) 

#define IWSManEx3_GetErrorMessage(This,errorNumber,errorMessage)	\
    ( (This)->lpVtbl -> GetErrorMessage(This,errorNumber,errorMessage) ) 

#define IWSManEx3_EnumerationFlagHierarchyDeep(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeep(This,flags) ) 

#define IWSManEx3_EnumerationFlagHierarchyShallow(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyShallow(This,flags) ) 

#define IWSManEx3_EnumerationFlagHierarchyDeepBasePropsOnly(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagHierarchyDeepBasePropsOnly(This,flags) ) 

#define IWSManEx3_EnumerationFlagReturnObject(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagReturnObject(This,flags) ) 


#define IWSManEx3_SessionFlagUseClientCertificate(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseClientCertificate(This,flags) ) 


#define IWSManEx3_SessionFlagUTF16(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUTF16(This,flags) ) 

#define IWSManEx3_SessionFlagUseCredSsp(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseCredSsp(This,flags) ) 

#define IWSManEx3_EnumerationFlagAssociationInstance(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagAssociationInstance(This,flags) ) 

#define IWSManEx3_EnumerationFlagAssociatedInstance(This,flags)	\
    ( (This)->lpVtbl -> EnumerationFlagAssociatedInstance(This,flags) ) 

#define IWSManEx3_SessionFlagSkipRevocationCheck(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagSkipRevocationCheck(This,flags) ) 

#define IWSManEx3_SessionFlagAllowNegotiateImplicitCredentials(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagAllowNegotiateImplicitCredentials(This,flags) ) 

#define IWSManEx3_SessionFlagUseSsl(This,flags)	\
    ( (This)->lpVtbl -> SessionFlagUseSsl(This,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManEx3_INTERFACE_DEFINED__ */


#ifndef __IWSManConnectionOptions_INTERFACE_DEFINED__
#define __IWSManConnectionOptions_INTERFACE_DEFINED__

/* interface IWSManConnectionOptions */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManConnectionOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F704E861-9E52-464f-B786-DA5EB2320FDD")
    IWSManConnectionOptions : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_UserName( 
            /* [in] */ BSTR name) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Password( 
            /* [in] */ BSTR password) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManConnectionOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManConnectionOptions * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManConnectionOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManConnectionOptions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManConnectionOptions * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManConnectionOptions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManConnectionOptions * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManConnectionOptions * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, get_UserName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            IWSManConnectionOptions * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_UserName)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserName )( 
            IWSManConnectionOptions * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_Password)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Password )( 
            IWSManConnectionOptions * This,
            /* [in] */ BSTR password);
        
        END_INTERFACE
    } IWSManConnectionOptionsVtbl;

    interface IWSManConnectionOptions
    {
        CONST_VTBL struct IWSManConnectionOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManConnectionOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManConnectionOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManConnectionOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManConnectionOptions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManConnectionOptions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManConnectionOptions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManConnectionOptions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManConnectionOptions_get_UserName(This,name)	\
    ( (This)->lpVtbl -> get_UserName(This,name) ) 

#define IWSManConnectionOptions_put_UserName(This,name)	\
    ( (This)->lpVtbl -> put_UserName(This,name) ) 

#define IWSManConnectionOptions_put_Password(This,password)	\
    ( (This)->lpVtbl -> put_Password(This,password) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManConnectionOptions_INTERFACE_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx_INTERFACE_DEFINED__
#define __IWSManConnectionOptionsEx_INTERFACE_DEFINED__

/* interface IWSManConnectionOptionsEx */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManConnectionOptionsEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF43EDF7-2A48-4d93-9526-8BD6AB6D4A6B")
    IWSManConnectionOptionsEx : public IWSManConnectionOptions
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_CertificateThumbprint( 
            /* [retval][out] */ BSTR *thumbprint) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_CertificateThumbprint( 
            /* [in] */ BSTR thumbprint) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManConnectionOptionsExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManConnectionOptionsEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManConnectionOptionsEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManConnectionOptionsEx * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManConnectionOptionsEx * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, get_UserName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            IWSManConnectionOptionsEx * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_UserName)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserName )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_Password)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Password )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ BSTR password);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx, get_CertificateThumbprint)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CertificateThumbprint )( 
            IWSManConnectionOptionsEx * This,
            /* [retval][out] */ BSTR *thumbprint);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx, put_CertificateThumbprint)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CertificateThumbprint )( 
            IWSManConnectionOptionsEx * This,
            /* [in] */ BSTR thumbprint);
        
        END_INTERFACE
    } IWSManConnectionOptionsExVtbl;

    interface IWSManConnectionOptionsEx
    {
        CONST_VTBL struct IWSManConnectionOptionsExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManConnectionOptionsEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManConnectionOptionsEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManConnectionOptionsEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManConnectionOptionsEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManConnectionOptionsEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManConnectionOptionsEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManConnectionOptionsEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManConnectionOptionsEx_get_UserName(This,name)	\
    ( (This)->lpVtbl -> get_UserName(This,name) ) 

#define IWSManConnectionOptionsEx_put_UserName(This,name)	\
    ( (This)->lpVtbl -> put_UserName(This,name) ) 

#define IWSManConnectionOptionsEx_put_Password(This,password)	\
    ( (This)->lpVtbl -> put_Password(This,password) ) 


#define IWSManConnectionOptionsEx_get_CertificateThumbprint(This,thumbprint)	\
    ( (This)->lpVtbl -> get_CertificateThumbprint(This,thumbprint) ) 

#define IWSManConnectionOptionsEx_put_CertificateThumbprint(This,thumbprint)	\
    ( (This)->lpVtbl -> put_CertificateThumbprint(This,thumbprint) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManConnectionOptionsEx_INTERFACE_DEFINED__ */


#ifndef __IWSManConnectionOptionsEx2_INTERFACE_DEFINED__
#define __IWSManConnectionOptionsEx2_INTERFACE_DEFINED__

/* interface IWSManConnectionOptionsEx2 */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManConnectionOptionsEx2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F500C9EC-24EE-48ab-B38D-FC9A164C658E")
    IWSManConnectionOptionsEx2 : public IWSManConnectionOptionsEx
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProxy( 
            /* [defaultvalue][in] */ long accessType = 0,
            /* [defaultvalue][in] */ long authenticationMechanism = 0,
            /* [defaultvalue][in] */ BSTR userName = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR password = (BSTR)L"") = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyIEConfig( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyWinHttpConfig( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyAutoDetect( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyNoProxyServer( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyAuthenticationUseNegotiate( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyAuthenticationUseBasic( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ProxyAuthenticationUseDigest( 
            /* [retval][out] */ long *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManConnectionOptionsEx2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManConnectionOptionsEx2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManConnectionOptionsEx2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManConnectionOptionsEx2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManConnectionOptionsEx2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, get_UserName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_UserName)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserName )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptions, put_Password)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Password )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ BSTR password);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx, get_CertificateThumbprint)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CertificateThumbprint )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ BSTR *thumbprint);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx, put_CertificateThumbprint)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CertificateThumbprint )( 
            IWSManConnectionOptionsEx2 * This,
            /* [in] */ BSTR thumbprint);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, SetProxy)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProxy )( 
            IWSManConnectionOptionsEx2 * This,
            /* [defaultvalue][in] */ long accessType,
            /* [defaultvalue][in] */ long authenticationMechanism,
            /* [defaultvalue][in] */ BSTR userName,
            /* [defaultvalue][in] */ BSTR password);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyIEConfig)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyIEConfig )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyWinHttpConfig)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyWinHttpConfig )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyAutoDetect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyAutoDetect )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyNoProxyServer)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyNoProxyServer )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyAuthenticationUseNegotiate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyAuthenticationUseNegotiate )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyAuthenticationUseBasic)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyAuthenticationUseBasic )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManConnectionOptionsEx2, ProxyAuthenticationUseDigest)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ProxyAuthenticationUseDigest )( 
            IWSManConnectionOptionsEx2 * This,
            /* [retval][out] */ long *value);
        
        END_INTERFACE
    } IWSManConnectionOptionsEx2Vtbl;

    interface IWSManConnectionOptionsEx2
    {
        CONST_VTBL struct IWSManConnectionOptionsEx2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManConnectionOptionsEx2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManConnectionOptionsEx2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManConnectionOptionsEx2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManConnectionOptionsEx2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManConnectionOptionsEx2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManConnectionOptionsEx2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManConnectionOptionsEx2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManConnectionOptionsEx2_get_UserName(This,name)	\
    ( (This)->lpVtbl -> get_UserName(This,name) ) 

#define IWSManConnectionOptionsEx2_put_UserName(This,name)	\
    ( (This)->lpVtbl -> put_UserName(This,name) ) 

#define IWSManConnectionOptionsEx2_put_Password(This,password)	\
    ( (This)->lpVtbl -> put_Password(This,password) ) 


#define IWSManConnectionOptionsEx2_get_CertificateThumbprint(This,thumbprint)	\
    ( (This)->lpVtbl -> get_CertificateThumbprint(This,thumbprint) ) 

#define IWSManConnectionOptionsEx2_put_CertificateThumbprint(This,thumbprint)	\
    ( (This)->lpVtbl -> put_CertificateThumbprint(This,thumbprint) ) 


#define IWSManConnectionOptionsEx2_SetProxy(This,accessType,authenticationMechanism,userName,password)	\
    ( (This)->lpVtbl -> SetProxy(This,accessType,authenticationMechanism,userName,password) ) 

#define IWSManConnectionOptionsEx2_ProxyIEConfig(This,value)	\
    ( (This)->lpVtbl -> ProxyIEConfig(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyWinHttpConfig(This,value)	\
    ( (This)->lpVtbl -> ProxyWinHttpConfig(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyAutoDetect(This,value)	\
    ( (This)->lpVtbl -> ProxyAutoDetect(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyNoProxyServer(This,value)	\
    ( (This)->lpVtbl -> ProxyNoProxyServer(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyAuthenticationUseNegotiate(This,value)	\
    ( (This)->lpVtbl -> ProxyAuthenticationUseNegotiate(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyAuthenticationUseBasic(This,value)	\
    ( (This)->lpVtbl -> ProxyAuthenticationUseBasic(This,value) ) 

#define IWSManConnectionOptionsEx2_ProxyAuthenticationUseDigest(This,value)	\
    ( (This)->lpVtbl -> ProxyAuthenticationUseDigest(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManConnectionOptionsEx2_INTERFACE_DEFINED__ */


#ifndef __IWSManSession_INTERFACE_DEFINED__
#define __IWSManSession_INTERFACE_DEFINED__

/* interface IWSManSession */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FC84FC58-1286-40c4-9DA0-C8EF6EC241E0")
    IWSManSession : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resource) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Put( 
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR resource,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resultResource) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR resource,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *newUri) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags = 0) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ BSTR actionUri,
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR parameters,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *result) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Enumerate( 
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ BSTR filter,
            /* [defaultvalue][in] */ BSTR dialect,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ IDispatch **resultSet) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Identify( 
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *result) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ BSTR *value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_BatchItems( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BatchItems( 
            /* [in] */ long value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Timeout( 
            /* [retval][out] */ long *value) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Timeout( 
            /* [in] */ long value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManSession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManSession * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManSession * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManSession * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManSession * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManSession * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManSession, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IWSManSession * This,
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resource);
        
        DECLSPEC_XFGVIRT(IWSManSession, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IWSManSession * This,
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR resource,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resultResource);
        
        DECLSPEC_XFGVIRT(IWSManSession, Create)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            IWSManSession * This,
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR resource,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *newUri);
        
        DECLSPEC_XFGVIRT(IWSManSession, Delete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IWSManSession * This,
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags);
        
        DECLSPEC_XFGVIRT(IWSManSession, Invoke)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManSession * This,
            /* [in] */ BSTR actionUri,
            /* [in] */ VARIANT resourceUri,
            /* [in] */ BSTR parameters,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *result);
        
        DECLSPEC_XFGVIRT(IWSManSession, Enumerate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Enumerate )( 
            IWSManSession * This,
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ BSTR filter,
            /* [defaultvalue][in] */ BSTR dialect,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ IDispatch **resultSet);
        
        DECLSPEC_XFGVIRT(IWSManSession, Identify)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Identify )( 
            IWSManSession * This,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *result);
        
        DECLSPEC_XFGVIRT(IWSManSession, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManSession * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IWSManSession, get_BatchItems)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BatchItems )( 
            IWSManSession * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManSession, put_BatchItems)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BatchItems )( 
            IWSManSession * This,
            /* [in] */ long value);
        
        DECLSPEC_XFGVIRT(IWSManSession, get_Timeout)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Timeout )( 
            IWSManSession * This,
            /* [retval][out] */ long *value);
        
        DECLSPEC_XFGVIRT(IWSManSession, put_Timeout)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Timeout )( 
            IWSManSession * This,
            /* [in] */ long value);
        
        END_INTERFACE
    } IWSManSessionVtbl;

    interface IWSManSession
    {
        CONST_VTBL struct IWSManSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManSession_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManSession_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManSession_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManSession_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManSession_Get(This,resourceUri,flags,resource)	\
    ( (This)->lpVtbl -> Get(This,resourceUri,flags,resource) ) 

#define IWSManSession_Put(This,resourceUri,resource,flags,resultResource)	\
    ( (This)->lpVtbl -> Put(This,resourceUri,resource,flags,resultResource) ) 

#define IWSManSession_Create(This,resourceUri,resource,flags,newUri)	\
    ( (This)->lpVtbl -> Create(This,resourceUri,resource,flags,newUri) ) 

#define IWSManSession_Delete(This,resourceUri,flags)	\
    ( (This)->lpVtbl -> Delete(This,resourceUri,flags) ) 

#define IWSManSession_Invoke(This,actionUri,resourceUri,parameters,flags,result)	\
    ( (This)->lpVtbl -> Invoke(This,actionUri,resourceUri,parameters,flags,result) ) 

#define IWSManSession_Enumerate(This,resourceUri,filter,dialect,flags,resultSet)	\
    ( (This)->lpVtbl -> Enumerate(This,resourceUri,filter,dialect,flags,resultSet) ) 

#define IWSManSession_Identify(This,flags,result)	\
    ( (This)->lpVtbl -> Identify(This,flags,result) ) 

#define IWSManSession_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 

#define IWSManSession_get_BatchItems(This,value)	\
    ( (This)->lpVtbl -> get_BatchItems(This,value) ) 

#define IWSManSession_put_BatchItems(This,value)	\
    ( (This)->lpVtbl -> put_BatchItems(This,value) ) 

#define IWSManSession_get_Timeout(This,value)	\
    ( (This)->lpVtbl -> get_Timeout(This,value) ) 

#define IWSManSession_put_Timeout(This,value)	\
    ( (This)->lpVtbl -> put_Timeout(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManSession_INTERFACE_DEFINED__ */


#ifndef __IWSManEnumerator_INTERFACE_DEFINED__
#define __IWSManEnumerator_INTERFACE_DEFINED__

/* interface IWSManEnumerator */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F3457CA9-ABB9-4fa5-B850-90E8CA300E7F")
    IWSManEnumerator : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ReadItem( 
            /* [retval][out] */ BSTR *resource) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_AtEndOfStream( 
            /* [retval][out] */ VARIANT_BOOL *eos) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ BSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManEnumerator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManEnumerator * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManEnumerator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManEnumerator * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManEnumerator * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManEnumerator, ReadItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ReadItem )( 
            IWSManEnumerator * This,
            /* [retval][out] */ BSTR *resource);
        
        DECLSPEC_XFGVIRT(IWSManEnumerator, get_AtEndOfStream)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AtEndOfStream )( 
            IWSManEnumerator * This,
            /* [retval][out] */ VARIANT_BOOL *eos);
        
        DECLSPEC_XFGVIRT(IWSManEnumerator, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManEnumerator * This,
            /* [retval][out] */ BSTR *value);
        
        END_INTERFACE
    } IWSManEnumeratorVtbl;

    interface IWSManEnumerator
    {
        CONST_VTBL struct IWSManEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManEnumerator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManEnumerator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManEnumerator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManEnumerator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManEnumerator_ReadItem(This,resource)	\
    ( (This)->lpVtbl -> ReadItem(This,resource) ) 

#define IWSManEnumerator_get_AtEndOfStream(This,eos)	\
    ( (This)->lpVtbl -> get_AtEndOfStream(This,eos) ) 

#define IWSManEnumerator_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManEnumerator_INTERFACE_DEFINED__ */


#ifndef __IWSManResourceLocator_INTERFACE_DEFINED__
#define __IWSManResourceLocator_INTERFACE_DEFINED__

/* interface IWSManResourceLocator */
/* [nonextensible][local][oleautomation][uuid][object][dual] */ 


EXTERN_C const IID IID_IWSManResourceLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A7A1BA28-DE41-466a-AD0A-C4059EAD7428")
    IWSManResourceLocator : public IDispatch
    {
    public:
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ResourceURI( 
            /* [in] */ BSTR uri) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ResourceURI( 
            /* [retval][out] */ BSTR *uri) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddSelector( 
            /* [in] */ BSTR resourceSelName,
            /* [in] */ VARIANT selValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearSelectors( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FragmentPath( 
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_FragmentPath( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FragmentDialect( 
            /* [retval][out] */ BSTR *text) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_FragmentDialect( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddOption( 
            /* [in] */ BSTR OptionName,
            /* [in] */ VARIANT OptionValue,
            /* [defaultvalue][in] */ BOOL mustComply = 0) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MustUnderstandOptions( 
            /* [in] */ BOOL mustUnderstand) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MustUnderstandOptions( 
            /* [retval][out] */ BOOL *mustUnderstand) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearOptions( void) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ BSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManResourceLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManResourceLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManResourceLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManResourceLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManResourceLocator * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManResourceLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManResourceLocator * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManResourceLocator * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, put_ResourceURI)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ResourceURI )( 
            IWSManResourceLocator * This,
            /* [in] */ BSTR uri);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, get_ResourceURI)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ResourceURI )( 
            IWSManResourceLocator * This,
            /* [retval][out] */ BSTR *uri);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, AddSelector)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddSelector )( 
            IWSManResourceLocator * This,
            /* [in] */ BSTR resourceSelName,
            /* [in] */ VARIANT selValue);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, ClearSelectors)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearSelectors )( 
            IWSManResourceLocator * This);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, get_FragmentPath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FragmentPath )( 
            IWSManResourceLocator * This,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, put_FragmentPath)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FragmentPath )( 
            IWSManResourceLocator * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, get_FragmentDialect)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FragmentDialect )( 
            IWSManResourceLocator * This,
            /* [retval][out] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, put_FragmentDialect)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FragmentDialect )( 
            IWSManResourceLocator * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, AddOption)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddOption )( 
            IWSManResourceLocator * This,
            /* [in] */ BSTR OptionName,
            /* [in] */ VARIANT OptionValue,
            /* [defaultvalue][in] */ BOOL mustComply);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, put_MustUnderstandOptions)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MustUnderstandOptions )( 
            IWSManResourceLocator * This,
            /* [in] */ BOOL mustUnderstand);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, get_MustUnderstandOptions)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MustUnderstandOptions )( 
            IWSManResourceLocator * This,
            /* [retval][out] */ BOOL *mustUnderstand);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, ClearOptions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearOptions )( 
            IWSManResourceLocator * This);
        
        DECLSPEC_XFGVIRT(IWSManResourceLocator, get_Error)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            IWSManResourceLocator * This,
            /* [retval][out] */ BSTR *value);
        
        END_INTERFACE
    } IWSManResourceLocatorVtbl;

    interface IWSManResourceLocator
    {
        CONST_VTBL struct IWSManResourceLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManResourceLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManResourceLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManResourceLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManResourceLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManResourceLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManResourceLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManResourceLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManResourceLocator_put_ResourceURI(This,uri)	\
    ( (This)->lpVtbl -> put_ResourceURI(This,uri) ) 

#define IWSManResourceLocator_get_ResourceURI(This,uri)	\
    ( (This)->lpVtbl -> get_ResourceURI(This,uri) ) 

#define IWSManResourceLocator_AddSelector(This,resourceSelName,selValue)	\
    ( (This)->lpVtbl -> AddSelector(This,resourceSelName,selValue) ) 

#define IWSManResourceLocator_ClearSelectors(This)	\
    ( (This)->lpVtbl -> ClearSelectors(This) ) 

#define IWSManResourceLocator_get_FragmentPath(This,text)	\
    ( (This)->lpVtbl -> get_FragmentPath(This,text) ) 

#define IWSManResourceLocator_put_FragmentPath(This,text)	\
    ( (This)->lpVtbl -> put_FragmentPath(This,text) ) 

#define IWSManResourceLocator_get_FragmentDialect(This,text)	\
    ( (This)->lpVtbl -> get_FragmentDialect(This,text) ) 

#define IWSManResourceLocator_put_FragmentDialect(This,text)	\
    ( (This)->lpVtbl -> put_FragmentDialect(This,text) ) 

#define IWSManResourceLocator_AddOption(This,OptionName,OptionValue,mustComply)	\
    ( (This)->lpVtbl -> AddOption(This,OptionName,OptionValue,mustComply) ) 

#define IWSManResourceLocator_put_MustUnderstandOptions(This,mustUnderstand)	\
    ( (This)->lpVtbl -> put_MustUnderstandOptions(This,mustUnderstand) ) 

#define IWSManResourceLocator_get_MustUnderstandOptions(This,mustUnderstand)	\
    ( (This)->lpVtbl -> get_MustUnderstandOptions(This,mustUnderstand) ) 

#define IWSManResourceLocator_ClearOptions(This)	\
    ( (This)->lpVtbl -> ClearOptions(This) ) 

#define IWSManResourceLocator_get_Error(This,value)	\
    ( (This)->lpVtbl -> get_Error(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManResourceLocator_INTERFACE_DEFINED__ */


#ifndef __IWSManResourceLocatorInternal_INTERFACE_DEFINED__
#define __IWSManResourceLocatorInternal_INTERFACE_DEFINED__

/* interface IWSManResourceLocatorInternal */
/* [hidden][nonextensible][local][oleautomation][uuid][object] */ 


EXTERN_C const IID IID_IWSManResourceLocatorInternal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EFFAEAD7-7EC8-4716-B9BE-F2E7E9FB4ADB")
    IWSManResourceLocatorInternal : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManResourceLocatorInternalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManResourceLocatorInternal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManResourceLocatorInternal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManResourceLocatorInternal * This);
        
        END_INTERFACE
    } IWSManResourceLocatorInternalVtbl;

    interface IWSManResourceLocatorInternal
    {
        CONST_VTBL struct IWSManResourceLocatorInternalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManResourceLocatorInternal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManResourceLocatorInternal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManResourceLocatorInternal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManResourceLocatorInternal_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WSMan;

#ifdef __cplusplus

class DECLSPEC_UUID("BCED617B-EC03-420b-8508-977DC7A686BD")
WSMan;
#endif

#ifndef __IWSManInternal_INTERFACE_DEFINED__
#define __IWSManInternal_INTERFACE_DEFINED__

/* interface IWSManInternal */
/* [hidden][nonextensible][local][oleautomation][uuid][object] */ 


EXTERN_C const IID IID_IWSManInternal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("04AE2B1D-9954-4D99-94A9-A961E72C3A13")
    IWSManInternal : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ConfigSDDL( 
            /* [in] */ IDispatch *session,
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSManInternalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSManInternal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSManInternal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSManInternal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWSManInternal * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWSManInternal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWSManInternal * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSManInternal * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWSManInternal, ConfigSDDL)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ConfigSDDL )( 
            IWSManInternal * This,
            /* [in] */ IDispatch *session,
            /* [in] */ VARIANT resourceUri,
            /* [defaultvalue][in] */ long flags,
            /* [retval][out] */ BSTR *resource);
        
        END_INTERFACE
    } IWSManInternalVtbl;

    interface IWSManInternal
    {
        CONST_VTBL struct IWSManInternalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSManInternal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSManInternal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSManInternal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSManInternal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSManInternal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSManInternal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSManInternal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSManInternal_ConfigSDDL(This,session,resourceUri,flags,resource)	\
    ( (This)->lpVtbl -> ConfigSDDL(This,session,resourceUri,flags,resource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSManInternal_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WSManInternal;

#ifdef __cplusplus

class DECLSPEC_UUID("7DE087A5-5DCB-4df7-BB12-0924AD8FBD9A")
WSManInternal;
#endif
#endif /* __WSManAutomation_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wsmandisp_0000_0013 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wsmandisp_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsmandisp_0000_0013_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


