

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

#ifndef __openservice_h__
#define __openservice_h__

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

#ifndef __IOpenServiceActivityInput_FWD_DEFINED__
#define __IOpenServiceActivityInput_FWD_DEFINED__
typedef interface IOpenServiceActivityInput IOpenServiceActivityInput;

#endif 	/* __IOpenServiceActivityInput_FWD_DEFINED__ */


#ifndef __IOpenServiceActivityOutputContext_FWD_DEFINED__
#define __IOpenServiceActivityOutputContext_FWD_DEFINED__
typedef interface IOpenServiceActivityOutputContext IOpenServiceActivityOutputContext;

#endif 	/* __IOpenServiceActivityOutputContext_FWD_DEFINED__ */


#ifndef __IOpenService_FWD_DEFINED__
#define __IOpenService_FWD_DEFINED__
typedef interface IOpenService IOpenService;

#endif 	/* __IOpenService_FWD_DEFINED__ */


#ifndef __IOpenServiceManager_FWD_DEFINED__
#define __IOpenServiceManager_FWD_DEFINED__
typedef interface IOpenServiceManager IOpenServiceManager;

#endif 	/* __IOpenServiceManager_FWD_DEFINED__ */


#ifndef __OpenServiceManager_FWD_DEFINED__
#define __OpenServiceManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class OpenServiceManager OpenServiceManager;
#else
typedef struct OpenServiceManager OpenServiceManager;
#endif /* __cplusplus */

#endif 	/* __OpenServiceManager_FWD_DEFINED__ */


#ifndef __OpenServiceActivityManager_FWD_DEFINED__
#define __OpenServiceActivityManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class OpenServiceActivityManager OpenServiceActivityManager;
#else
typedef struct OpenServiceActivityManager OpenServiceActivityManager;
#endif /* __cplusplus */

#endif 	/* __OpenServiceActivityManager_FWD_DEFINED__ */


#ifndef __IOpenServiceActivity_FWD_DEFINED__
#define __IOpenServiceActivity_FWD_DEFINED__
typedef interface IOpenServiceActivity IOpenServiceActivity;

#endif 	/* __IOpenServiceActivity_FWD_DEFINED__ */


#ifndef __IEnumOpenServiceActivity_FWD_DEFINED__
#define __IEnumOpenServiceActivity_FWD_DEFINED__
typedef interface IEnumOpenServiceActivity IEnumOpenServiceActivity;

#endif 	/* __IEnumOpenServiceActivity_FWD_DEFINED__ */


#ifndef __IOpenServiceActivityCategory_FWD_DEFINED__
#define __IOpenServiceActivityCategory_FWD_DEFINED__
typedef interface IOpenServiceActivityCategory IOpenServiceActivityCategory;

#endif 	/* __IOpenServiceActivityCategory_FWD_DEFINED__ */


#ifndef __IEnumOpenServiceActivityCategory_FWD_DEFINED__
#define __IEnumOpenServiceActivityCategory_FWD_DEFINED__
typedef interface IEnumOpenServiceActivityCategory IEnumOpenServiceActivityCategory;

#endif 	/* __IEnumOpenServiceActivityCategory_FWD_DEFINED__ */


#ifndef __IOpenServiceActivityManager_FWD_DEFINED__
#define __IOpenServiceActivityManager_FWD_DEFINED__
typedef interface IOpenServiceActivityManager IOpenServiceActivityManager;

#endif 	/* __IOpenServiceActivityManager_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"
#include "mshtml.h"
#include "exdisp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_openservice_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// openservice.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum OpenServiceErrors
    {
        OS_E_NOTFOUND	= 0x80030002,
        OS_E_NOTSUPPORTED	= 0x80004021,
        OS_E_CANCELLED	= 0x80002ef1,
        OS_E_GPDISABLED	= 0xc00d0bdc
    } 	OpenServiceErrors;

typedef 
enum OpenServiceActivityContentType
    {
        ActivityContentNone	= -1,
        ActivityContentDocument	= ( ActivityContentNone + 1 ) ,
        ActivityContentSelection	= ( ActivityContentDocument + 1 ) ,
        ActivityContentLink	= ( ActivityContentSelection + 1 ) ,
        ActivityContentCount	= ( ActivityContentLink + 1 ) 
    } 	OpenServiceActivityContentType;



extern RPC_IF_HANDLE __MIDL_itf_openservice_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_openservice_0000_0000_v0_0_s_ifspec;

#ifndef __IOpenServiceActivityInput_INTERFACE_DEFINED__
#define __IOpenServiceActivityInput_INTERFACE_DEFINED__

/* interface IOpenServiceActivityInput */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceActivityInput;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75cb4db9-6da0-4da3-83ce-422b6a433346")
    IOpenServiceActivityInput : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVariable( 
            /* [in] */ __RPC__in LPCWSTR pwzVariableName,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzVariableType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrVariableContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasVariable( 
            /* [in] */ __RPC__in LPCWSTR pwzVariableName,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzVariableType,
            /* [out] */ __RPC__out BOOL *pfHasVariable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out OpenServiceActivityContentType *pType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceActivityInputVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceActivityInput * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceActivityInput * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceActivityInput * This);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityInput, GetVariable)
        HRESULT ( STDMETHODCALLTYPE *GetVariable )( 
            __RPC__in IOpenServiceActivityInput * This,
            /* [in] */ __RPC__in LPCWSTR pwzVariableName,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzVariableType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrVariableContent);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityInput, HasVariable)
        HRESULT ( STDMETHODCALLTYPE *HasVariable )( 
            __RPC__in IOpenServiceActivityInput * This,
            /* [in] */ __RPC__in LPCWSTR pwzVariableName,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzVariableType,
            /* [out] */ __RPC__out BOOL *pfHasVariable);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityInput, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IOpenServiceActivityInput * This,
            /* [out] */ __RPC__out OpenServiceActivityContentType *pType);
        
        END_INTERFACE
    } IOpenServiceActivityInputVtbl;

    interface IOpenServiceActivityInput
    {
        CONST_VTBL struct IOpenServiceActivityInputVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceActivityInput_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceActivityInput_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceActivityInput_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceActivityInput_GetVariable(This,pwzVariableName,pwzVariableType,pbstrVariableContent)	\
    ( (This)->lpVtbl -> GetVariable(This,pwzVariableName,pwzVariableType,pbstrVariableContent) ) 

#define IOpenServiceActivityInput_HasVariable(This,pwzVariableName,pwzVariableType,pfHasVariable)	\
    ( (This)->lpVtbl -> HasVariable(This,pwzVariableName,pwzVariableType,pfHasVariable) ) 

#define IOpenServiceActivityInput_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceActivityInput_INTERFACE_DEFINED__ */


#ifndef __IOpenServiceActivityOutputContext_INTERFACE_DEFINED__
#define __IOpenServiceActivityOutputContext_INTERFACE_DEFINED__

/* interface IOpenServiceActivityOutputContext */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceActivityOutputContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e289deab-f709-49a9-b99e-282364074571")
    IOpenServiceActivityOutputContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Navigate( 
            /* [in] */ __RPC__in LPCWSTR pwzUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzMethod,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzHeaders,
            /* [unique][in] */ __RPC__in_opt IStream *pPostData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanNavigate( 
            /* [in] */ __RPC__in LPCWSTR pwzUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzMethod,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzHeaders,
            /* [unique][in] */ __RPC__in_opt IStream *pPostData,
            /* [out] */ __RPC__out BOOL *pfCanNavigate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceActivityOutputContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceActivityOutputContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceActivityOutputContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceActivityOutputContext * This);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityOutputContext, Navigate)
        HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in IOpenServiceActivityOutputContext * This,
            /* [in] */ __RPC__in LPCWSTR pwzUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzMethod,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzHeaders,
            /* [unique][in] */ __RPC__in_opt IStream *pPostData);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityOutputContext, CanNavigate)
        HRESULT ( STDMETHODCALLTYPE *CanNavigate )( 
            __RPC__in IOpenServiceActivityOutputContext * This,
            /* [in] */ __RPC__in LPCWSTR pwzUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzMethod,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwzHeaders,
            /* [unique][in] */ __RPC__in_opt IStream *pPostData,
            /* [out] */ __RPC__out BOOL *pfCanNavigate);
        
        END_INTERFACE
    } IOpenServiceActivityOutputContextVtbl;

    interface IOpenServiceActivityOutputContext
    {
        CONST_VTBL struct IOpenServiceActivityOutputContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceActivityOutputContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceActivityOutputContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceActivityOutputContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceActivityOutputContext_Navigate(This,pwzUri,pwzMethod,pwzHeaders,pPostData)	\
    ( (This)->lpVtbl -> Navigate(This,pwzUri,pwzMethod,pwzHeaders,pPostData) ) 

#define IOpenServiceActivityOutputContext_CanNavigate(This,pwzUri,pwzMethod,pwzHeaders,pPostData,pfCanNavigate)	\
    ( (This)->lpVtbl -> CanNavigate(This,pwzUri,pwzMethod,pwzHeaders,pPostData,pfCanNavigate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceActivityOutputContext_INTERFACE_DEFINED__ */


#ifndef __IOpenService_INTERFACE_DEFINED__
#define __IOpenService_INTERFACE_DEFINED__

/* interface IOpenService */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C2952ED1-6A89-4606-925F-1ED8B4BE0630")
    IOpenService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsDefault( 
            /* [out] */ __RPC__out BOOL *pfIsDefault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefault( 
            /* [in] */ BOOL fDefault,
            /* [in] */ __RPC__in HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetID( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenService * This);
        
        DECLSPEC_XFGVIRT(IOpenService, IsDefault)
        HRESULT ( STDMETHODCALLTYPE *IsDefault )( 
            __RPC__in IOpenService * This,
            /* [out] */ __RPC__out BOOL *pfIsDefault);
        
        DECLSPEC_XFGVIRT(IOpenService, SetDefault)
        HRESULT ( STDMETHODCALLTYPE *SetDefault )( 
            __RPC__in IOpenService * This,
            /* [in] */ BOOL fDefault,
            /* [in] */ __RPC__in HWND hwnd);
        
        DECLSPEC_XFGVIRT(IOpenService, GetID)
        HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in IOpenService * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrID);
        
        END_INTERFACE
    } IOpenServiceVtbl;

    interface IOpenService
    {
        CONST_VTBL struct IOpenServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenService_IsDefault(This,pfIsDefault)	\
    ( (This)->lpVtbl -> IsDefault(This,pfIsDefault) ) 

#define IOpenService_SetDefault(This,fDefault,hwnd)	\
    ( (This)->lpVtbl -> SetDefault(This,fDefault,hwnd) ) 

#define IOpenService_GetID(This,pbstrID)	\
    ( (This)->lpVtbl -> GetID(This,pbstrID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenService_INTERFACE_DEFINED__ */


#ifndef __IOpenServiceManager_INTERFACE_DEFINED__
#define __IOpenServiceManager_INTERFACE_DEFINED__

/* interface IOpenServiceManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5664125f-4e10-4e90-98e4-e4513d955a14")
    IOpenServiceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InstallService( 
            /* [in] */ __RPC__in LPCWSTR pwzServiceUrl,
            /* [out] */ __RPC__deref_out_opt IOpenService **ppService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UninstallService( 
            /* [in] */ __RPC__in_opt IOpenService *pService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceByID( 
            /* [in] */ __RPC__in LPCWSTR pwzID,
            /* [out] */ __RPC__deref_out_opt IOpenService **ppService) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceManager * This);
        
        DECLSPEC_XFGVIRT(IOpenServiceManager, InstallService)
        HRESULT ( STDMETHODCALLTYPE *InstallService )( 
            __RPC__in IOpenServiceManager * This,
            /* [in] */ __RPC__in LPCWSTR pwzServiceUrl,
            /* [out] */ __RPC__deref_out_opt IOpenService **ppService);
        
        DECLSPEC_XFGVIRT(IOpenServiceManager, UninstallService)
        HRESULT ( STDMETHODCALLTYPE *UninstallService )( 
            __RPC__in IOpenServiceManager * This,
            /* [in] */ __RPC__in_opt IOpenService *pService);
        
        DECLSPEC_XFGVIRT(IOpenServiceManager, GetServiceByID)
        HRESULT ( STDMETHODCALLTYPE *GetServiceByID )( 
            __RPC__in IOpenServiceManager * This,
            /* [in] */ __RPC__in LPCWSTR pwzID,
            /* [out] */ __RPC__deref_out_opt IOpenService **ppService);
        
        END_INTERFACE
    } IOpenServiceManagerVtbl;

    interface IOpenServiceManager
    {
        CONST_VTBL struct IOpenServiceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceManager_InstallService(This,pwzServiceUrl,ppService)	\
    ( (This)->lpVtbl -> InstallService(This,pwzServiceUrl,ppService) ) 

#define IOpenServiceManager_UninstallService(This,pService)	\
    ( (This)->lpVtbl -> UninstallService(This,pService) ) 

#define IOpenServiceManager_GetServiceByID(This,pwzID,ppService)	\
    ( (This)->lpVtbl -> GetServiceByID(This,pwzID,ppService) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceManager_INTERFACE_DEFINED__ */



#ifndef __IEOpenServiceObjects_LIBRARY_DEFINED__
#define __IEOpenServiceObjects_LIBRARY_DEFINED__

/* library IEOpenServiceObjects */
/* [uuid] */ 


EXTERN_C const IID LIBID_IEOpenServiceObjects;

EXTERN_C const CLSID CLSID_OpenServiceManager;

#ifdef __cplusplus

class DECLSPEC_UUID("098870b6-39ea-480b-b8b5-dd0167c4db59")
OpenServiceManager;
#endif

EXTERN_C const CLSID CLSID_OpenServiceActivityManager;

#ifdef __cplusplus

class DECLSPEC_UUID("c5efd803-50f8-43cd-9ab8-aafc1394c9e0")
OpenServiceActivityManager;
#endif
#endif /* __IEOpenServiceObjects_LIBRARY_DEFINED__ */

#ifndef __IOpenServiceActivity_INTERFACE_DEFINED__
#define __IOpenServiceActivity_INTERFACE_DEFINED__

/* interface IOpenServiceActivity */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13645c88-221a-4905-8ed1-4f5112cfc108")
    IOpenServiceActivity : public IOpenService
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Execute( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanExecute( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__out BOOL *pfCanExecute) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanExecuteType( 
            /* [in] */ OpenServiceActivityContentType type,
            /* [out] */ __RPC__out BOOL *pfCanExecute) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Preview( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanPreview( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__out BOOL *pfCanPreview) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanPreviewType( 
            /* [in] */ OpenServiceActivityContentType type,
            /* [out] */ __RPC__out BOOL *pfCanPreview) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatusText( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHomepageUrl( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHomepageUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCategoryName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCategoryName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIconPath( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrIconPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIcon( 
            /* [in] */ BOOL fSmallIcon,
            /* [out] */ __RPC__deref_out_opt HICON *phIcon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionFilePath( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXmlPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDownloadUrl( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXmlUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstallUrl( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrInstallUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEnabled( 
            /* [out] */ __RPC__out BOOL *pfIsEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEnabled( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IOpenService, IsDefault)
        HRESULT ( STDMETHODCALLTYPE *IsDefault )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__out BOOL *pfIsDefault);
        
        DECLSPEC_XFGVIRT(IOpenService, SetDefault)
        HRESULT ( STDMETHODCALLTYPE *SetDefault )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ BOOL fDefault,
            /* [in] */ __RPC__in HWND hwnd);
        
        DECLSPEC_XFGVIRT(IOpenService, GetID)
        HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrID);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, Execute)
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, CanExecute)
        HRESULT ( STDMETHODCALLTYPE *CanExecute )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__out BOOL *pfCanExecute);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, CanExecuteType)
        HRESULT ( STDMETHODCALLTYPE *CanExecuteType )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ OpenServiceActivityContentType type,
            /* [out] */ __RPC__out BOOL *pfCanExecute);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, Preview)
        HRESULT ( STDMETHODCALLTYPE *Preview )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, CanPreview)
        HRESULT ( STDMETHODCALLTYPE *CanPreview )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__out BOOL *pfCanPreview);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, CanPreviewType)
        HRESULT ( STDMETHODCALLTYPE *CanPreviewType )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ OpenServiceActivityContentType type,
            /* [out] */ __RPC__out BOOL *pfCanPreview);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetStatusText)
        HRESULT ( STDMETHODCALLTYPE *GetStatusText )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetHomepageUrl)
        HRESULT ( STDMETHODCALLTYPE *GetHomepageUrl )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHomepageUrl);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetCategoryName)
        HRESULT ( STDMETHODCALLTYPE *GetCategoryName )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCategoryName);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetIconPath)
        HRESULT ( STDMETHODCALLTYPE *GetIconPath )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrIconPath);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetIcon)
        HRESULT ( STDMETHODCALLTYPE *GetIcon )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ BOOL fSmallIcon,
            /* [out] */ __RPC__deref_out_opt HICON *phIcon);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetDescriptionFilePath)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionFilePath )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXmlPath);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetDownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *GetDownloadUrl )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXmlUri);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, GetInstallUrl)
        HRESULT ( STDMETHODCALLTYPE *GetInstallUrl )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrInstallUri);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, IsEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsEnabled )( 
            __RPC__in IOpenServiceActivity * This,
            /* [out] */ __RPC__out BOOL *pfIsEnabled);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivity, SetEnabled)
        HRESULT ( STDMETHODCALLTYPE *SetEnabled )( 
            __RPC__in IOpenServiceActivity * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IOpenServiceActivityVtbl;

    interface IOpenServiceActivity
    {
        CONST_VTBL struct IOpenServiceActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceActivity_IsDefault(This,pfIsDefault)	\
    ( (This)->lpVtbl -> IsDefault(This,pfIsDefault) ) 

#define IOpenServiceActivity_SetDefault(This,fDefault,hwnd)	\
    ( (This)->lpVtbl -> SetDefault(This,fDefault,hwnd) ) 

#define IOpenServiceActivity_GetID(This,pbstrID)	\
    ( (This)->lpVtbl -> GetID(This,pbstrID) ) 


#define IOpenServiceActivity_Execute(This,pInput,pOutput)	\
    ( (This)->lpVtbl -> Execute(This,pInput,pOutput) ) 

#define IOpenServiceActivity_CanExecute(This,pInput,pOutput,pfCanExecute)	\
    ( (This)->lpVtbl -> CanExecute(This,pInput,pOutput,pfCanExecute) ) 

#define IOpenServiceActivity_CanExecuteType(This,type,pfCanExecute)	\
    ( (This)->lpVtbl -> CanExecuteType(This,type,pfCanExecute) ) 

#define IOpenServiceActivity_Preview(This,pInput,pOutput)	\
    ( (This)->lpVtbl -> Preview(This,pInput,pOutput) ) 

#define IOpenServiceActivity_CanPreview(This,pInput,pOutput,pfCanPreview)	\
    ( (This)->lpVtbl -> CanPreview(This,pInput,pOutput,pfCanPreview) ) 

#define IOpenServiceActivity_CanPreviewType(This,type,pfCanPreview)	\
    ( (This)->lpVtbl -> CanPreviewType(This,type,pfCanPreview) ) 

#define IOpenServiceActivity_GetStatusText(This,pInput,pbstrStatusText)	\
    ( (This)->lpVtbl -> GetStatusText(This,pInput,pbstrStatusText) ) 

#define IOpenServiceActivity_GetHomepageUrl(This,pbstrHomepageUrl)	\
    ( (This)->lpVtbl -> GetHomepageUrl(This,pbstrHomepageUrl) ) 

#define IOpenServiceActivity_GetDisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pbstrDisplayName) ) 

#define IOpenServiceActivity_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define IOpenServiceActivity_GetCategoryName(This,pbstrCategoryName)	\
    ( (This)->lpVtbl -> GetCategoryName(This,pbstrCategoryName) ) 

#define IOpenServiceActivity_GetIconPath(This,pbstrIconPath)	\
    ( (This)->lpVtbl -> GetIconPath(This,pbstrIconPath) ) 

#define IOpenServiceActivity_GetIcon(This,fSmallIcon,phIcon)	\
    ( (This)->lpVtbl -> GetIcon(This,fSmallIcon,phIcon) ) 

#define IOpenServiceActivity_GetDescriptionFilePath(This,pbstrXmlPath)	\
    ( (This)->lpVtbl -> GetDescriptionFilePath(This,pbstrXmlPath) ) 

#define IOpenServiceActivity_GetDownloadUrl(This,pbstrXmlUri)	\
    ( (This)->lpVtbl -> GetDownloadUrl(This,pbstrXmlUri) ) 

#define IOpenServiceActivity_GetInstallUrl(This,pbstrInstallUri)	\
    ( (This)->lpVtbl -> GetInstallUrl(This,pbstrInstallUri) ) 

#define IOpenServiceActivity_IsEnabled(This,pfIsEnabled)	\
    ( (This)->lpVtbl -> IsEnabled(This,pfIsEnabled) ) 

#define IOpenServiceActivity_SetEnabled(This,fEnable)	\
    ( (This)->lpVtbl -> SetEnabled(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceActivity_INTERFACE_DEFINED__ */


#ifndef __IEnumOpenServiceActivity_INTERFACE_DEFINED__
#define __IEnumOpenServiceActivity_INTERFACE_DEFINED__

/* interface IEnumOpenServiceActivity */
/* [unique][uuid][object] */ 

typedef IOpenServiceActivity *EnumActivityType;


EXTERN_C const IID IID_IEnumOpenServiceActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a436d7d2-17c3-4ef4-a1e8-5c86faff26c0")
    IEnumOpenServiceActivity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) EnumActivityType *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivity **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumOpenServiceActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumOpenServiceActivity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumOpenServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumOpenServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivity, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumOpenServiceActivity * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) EnumActivityType *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivity, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumOpenServiceActivity * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivity, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumOpenServiceActivity * This);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivity, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumOpenServiceActivity * This,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivity **ppenum);
        
        END_INTERFACE
    } IEnumOpenServiceActivityVtbl;

    interface IEnumOpenServiceActivity
    {
        CONST_VTBL struct IEnumOpenServiceActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumOpenServiceActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumOpenServiceActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumOpenServiceActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumOpenServiceActivity_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumOpenServiceActivity_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumOpenServiceActivity_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumOpenServiceActivity_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumOpenServiceActivity_INTERFACE_DEFINED__ */


#ifndef __IOpenServiceActivityCategory_INTERFACE_DEFINED__
#define __IOpenServiceActivityCategory_INTERFACE_DEFINED__

/* interface IOpenServiceActivityCategory */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceActivityCategory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("850AF9D6-7309-40b5-BDB8-786C106B2153")
    IOpenServiceActivityCategory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HasDefaultActivity( 
            /* [out] */ __RPC__out BOOL *pfHasDefaultActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultActivity( 
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppDefaultActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultActivity( 
            /* [in] */ __RPC__in_opt IOpenServiceActivity *pActivity,
            /* [in] */ __RPC__in HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityEnumerator( 
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivity **ppEnumActivity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceActivityCategoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceActivityCategory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceActivityCategory * This);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityCategory, HasDefaultActivity)
        HRESULT ( STDMETHODCALLTYPE *HasDefaultActivity )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [out] */ __RPC__out BOOL *pfHasDefaultActivity);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityCategory, GetDefaultActivity)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultActivity )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppDefaultActivity);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityCategory, SetDefaultActivity)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultActivity )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivity *pActivity,
            /* [in] */ __RPC__in HWND hwnd);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityCategory, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityCategory, GetActivityEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetActivityEnumerator )( 
            __RPC__in IOpenServiceActivityCategory * This,
            /* [in] */ __RPC__in_opt IOpenServiceActivityInput *pInput,
            /* [in] */ __RPC__in_opt IOpenServiceActivityOutputContext *pOutput,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivity **ppEnumActivity);
        
        END_INTERFACE
    } IOpenServiceActivityCategoryVtbl;

    interface IOpenServiceActivityCategory
    {
        CONST_VTBL struct IOpenServiceActivityCategoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceActivityCategory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceActivityCategory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceActivityCategory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceActivityCategory_HasDefaultActivity(This,pfHasDefaultActivity)	\
    ( (This)->lpVtbl -> HasDefaultActivity(This,pfHasDefaultActivity) ) 

#define IOpenServiceActivityCategory_GetDefaultActivity(This,ppDefaultActivity)	\
    ( (This)->lpVtbl -> GetDefaultActivity(This,ppDefaultActivity) ) 

#define IOpenServiceActivityCategory_SetDefaultActivity(This,pActivity,hwnd)	\
    ( (This)->lpVtbl -> SetDefaultActivity(This,pActivity,hwnd) ) 

#define IOpenServiceActivityCategory_GetName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,pbstrName) ) 

#define IOpenServiceActivityCategory_GetActivityEnumerator(This,pInput,pOutput,ppEnumActivity)	\
    ( (This)->lpVtbl -> GetActivityEnumerator(This,pInput,pOutput,ppEnumActivity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceActivityCategory_INTERFACE_DEFINED__ */


#ifndef __IEnumOpenServiceActivityCategory_INTERFACE_DEFINED__
#define __IEnumOpenServiceActivityCategory_INTERFACE_DEFINED__

/* interface IEnumOpenServiceActivityCategory */
/* [unique][uuid][object] */ 

typedef IOpenServiceActivityCategory *EnumActivityVerbType;


EXTERN_C const IID IID_IEnumOpenServiceActivityCategory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33627a56-8c9a-4430-8fd1-b5f5c771afb6")
    IEnumOpenServiceActivityCategory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) EnumActivityVerbType *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivityCategory **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumOpenServiceActivityCategoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumOpenServiceActivityCategory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumOpenServiceActivityCategory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumOpenServiceActivityCategory * This);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivityCategory, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumOpenServiceActivityCategory * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) EnumActivityVerbType *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivityCategory, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumOpenServiceActivityCategory * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivityCategory, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumOpenServiceActivityCategory * This);
        
        DECLSPEC_XFGVIRT(IEnumOpenServiceActivityCategory, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumOpenServiceActivityCategory * This,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivityCategory **ppenum);
        
        END_INTERFACE
    } IEnumOpenServiceActivityCategoryVtbl;

    interface IEnumOpenServiceActivityCategory
    {
        CONST_VTBL struct IEnumOpenServiceActivityCategoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumOpenServiceActivityCategory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumOpenServiceActivityCategory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumOpenServiceActivityCategory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumOpenServiceActivityCategory_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumOpenServiceActivityCategory_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumOpenServiceActivityCategory_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumOpenServiceActivityCategory_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumOpenServiceActivityCategory_INTERFACE_DEFINED__ */


#ifndef __IOpenServiceActivityManager_INTERFACE_DEFINED__
#define __IOpenServiceActivityManager_INTERFACE_DEFINED__

/* interface IOpenServiceActivityManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOpenServiceActivityManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8a2d0a9d-e920-4bdc-a291-d30f650bc4f1")
    IOpenServiceActivityManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCategoryEnumerator( 
            /* [in] */ OpenServiceActivityContentType eType,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivityCategory **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityByID( 
            /* [in] */ __RPC__in LPCWSTR pwzActivityID,
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityByHomepageAndCategory( 
            /* [in] */ __RPC__in LPCWSTR pwzHomepage,
            /* [in] */ __RPC__in LPCWSTR pwzCategory,
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppActivity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionCookie( 
            /* [out] */ __RPC__out DWORD *pdwVersionCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpenServiceActivityManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpenServiceActivityManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpenServiceActivityManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpenServiceActivityManager * This);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityManager, GetCategoryEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCategoryEnumerator )( 
            __RPC__in IOpenServiceActivityManager * This,
            /* [in] */ OpenServiceActivityContentType eType,
            /* [out] */ __RPC__deref_out_opt IEnumOpenServiceActivityCategory **ppEnum);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityManager, GetActivityByID)
        HRESULT ( STDMETHODCALLTYPE *GetActivityByID )( 
            __RPC__in IOpenServiceActivityManager * This,
            /* [in] */ __RPC__in LPCWSTR pwzActivityID,
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppActivity);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityManager, GetActivityByHomepageAndCategory)
        HRESULT ( STDMETHODCALLTYPE *GetActivityByHomepageAndCategory )( 
            __RPC__in IOpenServiceActivityManager * This,
            /* [in] */ __RPC__in LPCWSTR pwzHomepage,
            /* [in] */ __RPC__in LPCWSTR pwzCategory,
            /* [out] */ __RPC__deref_out_opt IOpenServiceActivity **ppActivity);
        
        DECLSPEC_XFGVIRT(IOpenServiceActivityManager, GetVersionCookie)
        HRESULT ( STDMETHODCALLTYPE *GetVersionCookie )( 
            __RPC__in IOpenServiceActivityManager * This,
            /* [out] */ __RPC__out DWORD *pdwVersionCookie);
        
        END_INTERFACE
    } IOpenServiceActivityManagerVtbl;

    interface IOpenServiceActivityManager
    {
        CONST_VTBL struct IOpenServiceActivityManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenServiceActivityManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenServiceActivityManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenServiceActivityManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenServiceActivityManager_GetCategoryEnumerator(This,eType,ppEnum)	\
    ( (This)->lpVtbl -> GetCategoryEnumerator(This,eType,ppEnum) ) 

#define IOpenServiceActivityManager_GetActivityByID(This,pwzActivityID,ppActivity)	\
    ( (This)->lpVtbl -> GetActivityByID(This,pwzActivityID,ppActivity) ) 

#define IOpenServiceActivityManager_GetActivityByHomepageAndCategory(This,pwzHomepage,pwzCategory,ppActivity)	\
    ( (This)->lpVtbl -> GetActivityByHomepageAndCategory(This,pwzHomepage,pwzCategory,ppActivity) ) 

#define IOpenServiceActivityManager_GetVersionCookie(This,pdwVersionCookie)	\
    ( (This)->lpVtbl -> GetVersionCookie(This,pdwVersionCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpenServiceActivityManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_openservice_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_openservice_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_openservice_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


