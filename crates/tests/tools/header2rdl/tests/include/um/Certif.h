

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __certif_h__
#define __certif_h__

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

#ifndef __ICertServerPolicy_FWD_DEFINED__
#define __ICertServerPolicy_FWD_DEFINED__
typedef interface ICertServerPolicy ICertServerPolicy;

#endif 	/* __ICertServerPolicy_FWD_DEFINED__ */


#ifndef __ICertServerExit_FWD_DEFINED__
#define __ICertServerExit_FWD_DEFINED__
typedef interface ICertServerExit ICertServerExit;

#endif 	/* __ICertServerExit_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certif_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define	ENUMEXT_OBJECTID	( 0x1 )



extern RPC_IF_HANDLE __MIDL_itf_certif_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certif_0000_0000_v0_0_s_ifspec;

#ifndef __ICertServerPolicy_INTERFACE_DEFINED__
#define __ICertServerPolicy_INTERFACE_DEFINED__

/* interface ICertServerPolicy */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertServerPolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa000922-ffbe-11cf-8800-00a0c903b83c")
    ICertServerPolicy : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetContext( 
            /* [in] */ LONG Context) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestProperty( 
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestAttribute( 
            /* [in] */ __RPC__in const BSTR strAttributeName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateProperty( 
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCertificateProperty( 
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [in] */ __RPC__in const VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateExtension( 
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateExtensionFlags( 
            /* [retval][out] */ __RPC__out LONG *pExtFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCertificateExtension( 
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [in] */ LONG ExtFlags,
            /* [in] */ __RPC__in const VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensionsSetup( 
            /* [in] */ LONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensions( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrExtensionName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensionsClose( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributesSetup( 
            /* [in] */ LONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributesClose( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertServerPolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertServerPolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertServerPolicy * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertServerPolicy * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertServerPolicy * This,
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
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, SetContext)
        HRESULT ( STDMETHODCALLTYPE *SetContext )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ LONG Context);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, GetRequestProperty)
        HRESULT ( STDMETHODCALLTYPE *GetRequestProperty )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, GetRequestAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetRequestAttribute )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strAttributeName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, GetCertificateProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateProperty )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, SetCertificateProperty)
        HRESULT ( STDMETHODCALLTYPE *SetCertificateProperty )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [in] */ __RPC__in const VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, GetCertificateExtension)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateExtension )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, GetCertificateExtensionFlags)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateExtensionFlags )( 
            __RPC__in ICertServerPolicy * This,
            /* [retval][out] */ __RPC__out LONG *pExtFlags);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, SetCertificateExtension)
        HRESULT ( STDMETHODCALLTYPE *SetCertificateExtension )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [in] */ LONG ExtFlags,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateExtensionsSetup)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensionsSetup )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ LONG Flags);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateExtensions)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensions )( 
            __RPC__in ICertServerPolicy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrExtensionName);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateExtensionsClose)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensionsClose )( 
            __RPC__in ICertServerPolicy * This);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateAttributesSetup)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributesSetup )( 
            __RPC__in ICertServerPolicy * This,
            /* [in] */ LONG Flags);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateAttributes)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributes )( 
            __RPC__in ICertServerPolicy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeName);
        
        DECLSPEC_XFGVIRT(ICertServerPolicy, EnumerateAttributesClose)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributesClose )( 
            __RPC__in ICertServerPolicy * This);
        
        END_INTERFACE
    } ICertServerPolicyVtbl;

    interface ICertServerPolicy
    {
        CONST_VTBL struct ICertServerPolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertServerPolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertServerPolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertServerPolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertServerPolicy_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertServerPolicy_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertServerPolicy_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertServerPolicy_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertServerPolicy_SetContext(This,Context)	\
    ( (This)->lpVtbl -> SetContext(This,Context) ) 

#define ICertServerPolicy_GetRequestProperty(This,strPropertyName,PropertyType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetRequestProperty(This,strPropertyName,PropertyType,pvarPropertyValue) ) 

#define ICertServerPolicy_GetRequestAttribute(This,strAttributeName,pstrAttributeValue)	\
    ( (This)->lpVtbl -> GetRequestAttribute(This,strAttributeName,pstrAttributeValue) ) 

#define ICertServerPolicy_GetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue) ) 

#define ICertServerPolicy_SetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> SetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue) ) 

#define ICertServerPolicy_GetCertificateExtension(This,strExtensionName,Type,pvarValue)	\
    ( (This)->lpVtbl -> GetCertificateExtension(This,strExtensionName,Type,pvarValue) ) 

#define ICertServerPolicy_GetCertificateExtensionFlags(This,pExtFlags)	\
    ( (This)->lpVtbl -> GetCertificateExtensionFlags(This,pExtFlags) ) 

#define ICertServerPolicy_SetCertificateExtension(This,strExtensionName,Type,ExtFlags,pvarValue)	\
    ( (This)->lpVtbl -> SetCertificateExtension(This,strExtensionName,Type,ExtFlags,pvarValue) ) 

#define ICertServerPolicy_EnumerateExtensionsSetup(This,Flags)	\
    ( (This)->lpVtbl -> EnumerateExtensionsSetup(This,Flags) ) 

#define ICertServerPolicy_EnumerateExtensions(This,pstrExtensionName)	\
    ( (This)->lpVtbl -> EnumerateExtensions(This,pstrExtensionName) ) 

#define ICertServerPolicy_EnumerateExtensionsClose(This)	\
    ( (This)->lpVtbl -> EnumerateExtensionsClose(This) ) 

#define ICertServerPolicy_EnumerateAttributesSetup(This,Flags)	\
    ( (This)->lpVtbl -> EnumerateAttributesSetup(This,Flags) ) 

#define ICertServerPolicy_EnumerateAttributes(This,pstrAttributeName)	\
    ( (This)->lpVtbl -> EnumerateAttributes(This,pstrAttributeName) ) 

#define ICertServerPolicy_EnumerateAttributesClose(This)	\
    ( (This)->lpVtbl -> EnumerateAttributesClose(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertServerPolicy_INTERFACE_DEFINED__ */


#ifndef __ICertServerExit_INTERFACE_DEFINED__
#define __ICertServerExit_INTERFACE_DEFINED__

/* interface ICertServerExit */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertServerExit;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ba9eb90-732c-11d0-8816-00a0c903b83c")
    ICertServerExit : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetContext( 
            /* [in] */ LONG Context) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestProperty( 
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestAttribute( 
            /* [in] */ __RPC__in const BSTR strAttributeName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateProperty( 
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateExtension( 
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateExtensionFlags( 
            /* [retval][out] */ __RPC__out LONG *pExtFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensionsSetup( 
            /* [in] */ LONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensions( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrExtensionName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateExtensionsClose( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributesSetup( 
            /* [in] */ LONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateAttributesClose( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertServerExitVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertServerExit * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertServerExit * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertServerExit * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertServerExit * This,
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
        
        DECLSPEC_XFGVIRT(ICertServerExit, SetContext)
        HRESULT ( STDMETHODCALLTYPE *SetContext )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ LONG Context);
        
        DECLSPEC_XFGVIRT(ICertServerExit, GetRequestProperty)
        HRESULT ( STDMETHODCALLTYPE *GetRequestProperty )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertServerExit, GetRequestAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetRequestAttribute )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in const BSTR strAttributeName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeValue);
        
        DECLSPEC_XFGVIRT(ICertServerExit, GetCertificateProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateProperty )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in const BSTR strPropertyName,
            /* [in] */ LONG PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertServerExit, GetCertificateExtension)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateExtension )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertServerExit, GetCertificateExtensionFlags)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateExtensionFlags )( 
            __RPC__in ICertServerExit * This,
            /* [retval][out] */ __RPC__out LONG *pExtFlags);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateExtensionsSetup)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensionsSetup )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ LONG Flags);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateExtensions)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensions )( 
            __RPC__in ICertServerExit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrExtensionName);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateExtensionsClose)
        HRESULT ( STDMETHODCALLTYPE *EnumerateExtensionsClose )( 
            __RPC__in ICertServerExit * This);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateAttributesSetup)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributesSetup )( 
            __RPC__in ICertServerExit * This,
            /* [in] */ LONG Flags);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateAttributes)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributes )( 
            __RPC__in ICertServerExit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrAttributeName);
        
        DECLSPEC_XFGVIRT(ICertServerExit, EnumerateAttributesClose)
        HRESULT ( STDMETHODCALLTYPE *EnumerateAttributesClose )( 
            __RPC__in ICertServerExit * This);
        
        END_INTERFACE
    } ICertServerExitVtbl;

    interface ICertServerExit
    {
        CONST_VTBL struct ICertServerExitVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertServerExit_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertServerExit_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertServerExit_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertServerExit_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertServerExit_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertServerExit_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertServerExit_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertServerExit_SetContext(This,Context)	\
    ( (This)->lpVtbl -> SetContext(This,Context) ) 

#define ICertServerExit_GetRequestProperty(This,strPropertyName,PropertyType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetRequestProperty(This,strPropertyName,PropertyType,pvarPropertyValue) ) 

#define ICertServerExit_GetRequestAttribute(This,strAttributeName,pstrAttributeValue)	\
    ( (This)->lpVtbl -> GetRequestAttribute(This,strAttributeName,pstrAttributeValue) ) 

#define ICertServerExit_GetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetCertificateProperty(This,strPropertyName,PropertyType,pvarPropertyValue) ) 

#define ICertServerExit_GetCertificateExtension(This,strExtensionName,Type,pvarValue)	\
    ( (This)->lpVtbl -> GetCertificateExtension(This,strExtensionName,Type,pvarValue) ) 

#define ICertServerExit_GetCertificateExtensionFlags(This,pExtFlags)	\
    ( (This)->lpVtbl -> GetCertificateExtensionFlags(This,pExtFlags) ) 

#define ICertServerExit_EnumerateExtensionsSetup(This,Flags)	\
    ( (This)->lpVtbl -> EnumerateExtensionsSetup(This,Flags) ) 

#define ICertServerExit_EnumerateExtensions(This,pstrExtensionName)	\
    ( (This)->lpVtbl -> EnumerateExtensions(This,pstrExtensionName) ) 

#define ICertServerExit_EnumerateExtensionsClose(This)	\
    ( (This)->lpVtbl -> EnumerateExtensionsClose(This) ) 

#define ICertServerExit_EnumerateAttributesSetup(This,Flags)	\
    ( (This)->lpVtbl -> EnumerateAttributesSetup(This,Flags) ) 

#define ICertServerExit_EnumerateAttributes(This,pstrAttributeName)	\
    ( (This)->lpVtbl -> EnumerateAttributes(This,pstrAttributeName) ) 

#define ICertServerExit_EnumerateAttributesClose(This)	\
    ( (This)->lpVtbl -> EnumerateAttributesClose(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertServerExit_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certif_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certif_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certif_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


