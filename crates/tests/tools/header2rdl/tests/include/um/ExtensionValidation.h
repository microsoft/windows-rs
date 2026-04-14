

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

#ifndef __extensionvalidation_h__
#define __extensionvalidation_h__

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

#ifndef __IExtensionValidation_FWD_DEFINED__
#define __IExtensionValidation_FWD_DEFINED__
typedef interface IExtensionValidation IExtensionValidation;

#endif 	/* __IExtensionValidation_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "Mshtml.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_extensionvalidation_0000_0000 */
/* [local] */ 

/*******************************************************
 *                                                     *
 *     Copyright (C) Microsoft. All rights reserved.   *
 *                                                     *
 *******************************************************/
#ifdef _MSC_VER
#pragma once
#endif

#pragma comment(lib,"uuid.lib")

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum ExtensionValidationContexts
    {
        ExtensionValidationContextNone	= 0,
        ExtensionValidationContextDynamic	= 0x1,
        ExtensionValidationContextParsed	= 0x2
    } 	ExtensionValidationContexts;

typedef 
enum ExtensionValidationResults
    {
        ExtensionValidationResultNone	= 0,
        ExtensionValidationResultDoNotInstantiate	= 0x1,
        ExtensionValidationResultArrestPageLoad	= 0x2
    } 	ExtensionValidationResults;



extern RPC_IF_HANDLE __MIDL_itf_extensionvalidation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_extensionvalidation_0000_0000_v0_0_s_ifspec;

#ifndef __IExtensionValidation_INTERFACE_DEFINED__
#define __IExtensionValidation_INTERFACE_DEFINED__

/* interface IExtensionValidation */
/* [uuid][unique][object][local] */ 


EXTERN_C const IID IID_IExtensionValidation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7d33f73d-8525-4e0f-87db-830288baff44")
    IExtensionValidation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Validate( 
            /* [in] */ REFGUID extensionGuid,
            /* [string][in] */ LPWSTR extensionModulePath,
            /* [in] */ DWORD extensionFileVersionMS,
            /* [in] */ DWORD extensionFileVersionLS,
            /* [in] */ IHTMLDocument2 *htmlDocumentTop,
            /* [in] */ IHTMLDocument2 *htmlDocumentSubframe,
            /* [in] */ IHTMLElement *htmlElement,
            /* [in] */ ExtensionValidationContexts contexts,
            /* [out] */ ExtensionValidationResults *results) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayName( 
            /* [string][out] */ LPWSTR *displayName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtensionValidationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExtensionValidation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExtensionValidation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExtensionValidation * This);
        
        DECLSPEC_XFGVIRT(IExtensionValidation, Validate)
        HRESULT ( STDMETHODCALLTYPE *Validate )( 
            IExtensionValidation * This,
            /* [in] */ REFGUID extensionGuid,
            /* [string][in] */ LPWSTR extensionModulePath,
            /* [in] */ DWORD extensionFileVersionMS,
            /* [in] */ DWORD extensionFileVersionLS,
            /* [in] */ IHTMLDocument2 *htmlDocumentTop,
            /* [in] */ IHTMLDocument2 *htmlDocumentSubframe,
            /* [in] */ IHTMLElement *htmlElement,
            /* [in] */ ExtensionValidationContexts contexts,
            /* [out] */ ExtensionValidationResults *results);
        
        DECLSPEC_XFGVIRT(IExtensionValidation, DisplayName)
        HRESULT ( STDMETHODCALLTYPE *DisplayName )( 
            IExtensionValidation * This,
            /* [string][out] */ LPWSTR *displayName);
        
        END_INTERFACE
    } IExtensionValidationVtbl;

    interface IExtensionValidation
    {
        CONST_VTBL struct IExtensionValidationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtensionValidation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtensionValidation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtensionValidation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtensionValidation_Validate(This,extensionGuid,extensionModulePath,extensionFileVersionMS,extensionFileVersionLS,htmlDocumentTop,htmlDocumentSubframe,htmlElement,contexts,results)	\
    ( (This)->lpVtbl -> Validate(This,extensionGuid,extensionModulePath,extensionFileVersionMS,extensionFileVersionLS,htmlDocumentTop,htmlDocumentSubframe,htmlElement,contexts,results) ) 

#define IExtensionValidation_DisplayName(This,displayName)	\
    ( (This)->lpVtbl -> DisplayName(This,displayName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtensionValidation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_extensionvalidation_0000_0001 */
/* [local] */ 

EXTERN_C GUID CATID_ExtensionValidation;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_extensionvalidation_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_extensionvalidation_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


