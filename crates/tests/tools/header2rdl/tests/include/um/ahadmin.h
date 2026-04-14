

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

#ifndef __ahadmin_h__
#define __ahadmin_h__

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

#ifndef __IAppHostMethodExtension_FWD_DEFINED__
#define __IAppHostMethodExtension_FWD_DEFINED__
typedef interface IAppHostMethodExtension IAppHostMethodExtension;

#endif 	/* __IAppHostMethodExtension_FWD_DEFINED__ */


#ifndef __IAppHostPropertyExtension_FWD_DEFINED__
#define __IAppHostPropertyExtension_FWD_DEFINED__
typedef interface IAppHostPropertyExtension IAppHostPropertyExtension;

#endif 	/* __IAppHostPropertyExtension_FWD_DEFINED__ */


#ifndef __IAppHostElementExtension_FWD_DEFINED__
#define __IAppHostElementExtension_FWD_DEFINED__
typedef interface IAppHostElementExtension IAppHostElementExtension;

#endif 	/* __IAppHostElementExtension_FWD_DEFINED__ */


#ifndef __IAppHostMappingExtension_FWD_DEFINED__
#define __IAppHostMappingExtension_FWD_DEFINED__
typedef interface IAppHostMappingExtension IAppHostMappingExtension;

#endif 	/* __IAppHostMappingExtension_FWD_DEFINED__ */


#ifndef __IAppHostChildElementCollection_FWD_DEFINED__
#define __IAppHostChildElementCollection_FWD_DEFINED__
typedef interface IAppHostChildElementCollection IAppHostChildElementCollection;

#endif 	/* __IAppHostChildElementCollection_FWD_DEFINED__ */


#ifndef __IAppHostPropertyCollection_FWD_DEFINED__
#define __IAppHostPropertyCollection_FWD_DEFINED__
typedef interface IAppHostPropertyCollection IAppHostPropertyCollection;

#endif 	/* __IAppHostPropertyCollection_FWD_DEFINED__ */


#ifndef __IAppHostConfigLocationCollection_FWD_DEFINED__
#define __IAppHostConfigLocationCollection_FWD_DEFINED__
typedef interface IAppHostConfigLocationCollection IAppHostConfigLocationCollection;

#endif 	/* __IAppHostConfigLocationCollection_FWD_DEFINED__ */


#ifndef __IAppHostMethodCollection_FWD_DEFINED__
#define __IAppHostMethodCollection_FWD_DEFINED__
typedef interface IAppHostMethodCollection IAppHostMethodCollection;

#endif 	/* __IAppHostMethodCollection_FWD_DEFINED__ */


#ifndef __IAppHostElementSchemaCollection_FWD_DEFINED__
#define __IAppHostElementSchemaCollection_FWD_DEFINED__
typedef interface IAppHostElementSchemaCollection IAppHostElementSchemaCollection;

#endif 	/* __IAppHostElementSchemaCollection_FWD_DEFINED__ */


#ifndef __IAppHostPropertySchemaCollection_FWD_DEFINED__
#define __IAppHostPropertySchemaCollection_FWD_DEFINED__
typedef interface IAppHostPropertySchemaCollection IAppHostPropertySchemaCollection;

#endif 	/* __IAppHostPropertySchemaCollection_FWD_DEFINED__ */


#ifndef __IAppHostConstantValueCollection_FWD_DEFINED__
#define __IAppHostConstantValueCollection_FWD_DEFINED__
typedef interface IAppHostConstantValueCollection IAppHostConstantValueCollection;

#endif 	/* __IAppHostConstantValueCollection_FWD_DEFINED__ */


#ifndef __IAppHostConstantValue_FWD_DEFINED__
#define __IAppHostConstantValue_FWD_DEFINED__
typedef interface IAppHostConstantValue IAppHostConstantValue;

#endif 	/* __IAppHostConstantValue_FWD_DEFINED__ */


#ifndef __IAppHostPropertySchema_FWD_DEFINED__
#define __IAppHostPropertySchema_FWD_DEFINED__
typedef interface IAppHostPropertySchema IAppHostPropertySchema;

#endif 	/* __IAppHostPropertySchema_FWD_DEFINED__ */


#ifndef __IAppHostCollectionSchema_FWD_DEFINED__
#define __IAppHostCollectionSchema_FWD_DEFINED__
typedef interface IAppHostCollectionSchema IAppHostCollectionSchema;

#endif 	/* __IAppHostCollectionSchema_FWD_DEFINED__ */


#ifndef __IAppHostElementSchema_FWD_DEFINED__
#define __IAppHostElementSchema_FWD_DEFINED__
typedef interface IAppHostElementSchema IAppHostElementSchema;

#endif 	/* __IAppHostElementSchema_FWD_DEFINED__ */


#ifndef __IAppHostMethodSchema_FWD_DEFINED__
#define __IAppHostMethodSchema_FWD_DEFINED__
typedef interface IAppHostMethodSchema IAppHostMethodSchema;

#endif 	/* __IAppHostMethodSchema_FWD_DEFINED__ */


#ifndef __IAppHostMethodInstance_FWD_DEFINED__
#define __IAppHostMethodInstance_FWD_DEFINED__
typedef interface IAppHostMethodInstance IAppHostMethodInstance;

#endif 	/* __IAppHostMethodInstance_FWD_DEFINED__ */


#ifndef __IAppHostMethod_FWD_DEFINED__
#define __IAppHostMethod_FWD_DEFINED__
typedef interface IAppHostMethod IAppHostMethod;

#endif 	/* __IAppHostMethod_FWD_DEFINED__ */


#ifndef __IAppHostConfigException_FWD_DEFINED__
#define __IAppHostConfigException_FWD_DEFINED__
typedef interface IAppHostConfigException IAppHostConfigException;

#endif 	/* __IAppHostConfigException_FWD_DEFINED__ */


#ifndef __IAppHostPropertyException_FWD_DEFINED__
#define __IAppHostPropertyException_FWD_DEFINED__
typedef interface IAppHostPropertyException IAppHostPropertyException;

#endif 	/* __IAppHostPropertyException_FWD_DEFINED__ */


#ifndef __IAppHostElementCollection_FWD_DEFINED__
#define __IAppHostElementCollection_FWD_DEFINED__
typedef interface IAppHostElementCollection IAppHostElementCollection;

#endif 	/* __IAppHostElementCollection_FWD_DEFINED__ */


#ifndef __IAppHostElement_FWD_DEFINED__
#define __IAppHostElement_FWD_DEFINED__
typedef interface IAppHostElement IAppHostElement;

#endif 	/* __IAppHostElement_FWD_DEFINED__ */


#ifndef __IAppHostProperty_FWD_DEFINED__
#define __IAppHostProperty_FWD_DEFINED__
typedef interface IAppHostProperty IAppHostProperty;

#endif 	/* __IAppHostProperty_FWD_DEFINED__ */


#ifndef __IAppHostConfigLocation_FWD_DEFINED__
#define __IAppHostConfigLocation_FWD_DEFINED__
typedef interface IAppHostConfigLocation IAppHostConfigLocation;

#endif 	/* __IAppHostConfigLocation_FWD_DEFINED__ */


#ifndef __IAppHostSectionDefinition_FWD_DEFINED__
#define __IAppHostSectionDefinition_FWD_DEFINED__
typedef interface IAppHostSectionDefinition IAppHostSectionDefinition;

#endif 	/* __IAppHostSectionDefinition_FWD_DEFINED__ */


#ifndef __IAppHostSectionDefinitionCollection_FWD_DEFINED__
#define __IAppHostSectionDefinitionCollection_FWD_DEFINED__
typedef interface IAppHostSectionDefinitionCollection IAppHostSectionDefinitionCollection;

#endif 	/* __IAppHostSectionDefinitionCollection_FWD_DEFINED__ */


#ifndef __IAppHostSectionGroup_FWD_DEFINED__
#define __IAppHostSectionGroup_FWD_DEFINED__
typedef interface IAppHostSectionGroup IAppHostSectionGroup;

#endif 	/* __IAppHostSectionGroup_FWD_DEFINED__ */


#ifndef __IAppHostConfigFile_FWD_DEFINED__
#define __IAppHostConfigFile_FWD_DEFINED__
typedef interface IAppHostConfigFile IAppHostConfigFile;

#endif 	/* __IAppHostConfigFile_FWD_DEFINED__ */


#ifndef __IAppHostPathMapper_FWD_DEFINED__
#define __IAppHostPathMapper_FWD_DEFINED__
typedef interface IAppHostPathMapper IAppHostPathMapper;

#endif 	/* __IAppHostPathMapper_FWD_DEFINED__ */


#ifndef __IAppHostPathMapper2_FWD_DEFINED__
#define __IAppHostPathMapper2_FWD_DEFINED__
typedef interface IAppHostPathMapper2 IAppHostPathMapper2;

#endif 	/* __IAppHostPathMapper2_FWD_DEFINED__ */


#ifndef __IAppHostChangeHandler_FWD_DEFINED__
#define __IAppHostChangeHandler_FWD_DEFINED__
typedef interface IAppHostChangeHandler IAppHostChangeHandler;

#endif 	/* __IAppHostChangeHandler_FWD_DEFINED__ */


#ifndef __IAppHostAdminManager_FWD_DEFINED__
#define __IAppHostAdminManager_FWD_DEFINED__
typedef interface IAppHostAdminManager IAppHostAdminManager;

#endif 	/* __IAppHostAdminManager_FWD_DEFINED__ */


#ifndef __IAppHostWritableAdminManager_FWD_DEFINED__
#define __IAppHostWritableAdminManager_FWD_DEFINED__
typedef interface IAppHostWritableAdminManager IAppHostWritableAdminManager;

#endif 	/* __IAppHostWritableAdminManager_FWD_DEFINED__ */


#ifndef __IAppHostConfigManager_FWD_DEFINED__
#define __IAppHostConfigManager_FWD_DEFINED__
typedef interface IAppHostConfigManager IAppHostConfigManager;

#endif 	/* __IAppHostConfigManager_FWD_DEFINED__ */


#ifndef __AppHostAdminManager_FWD_DEFINED__
#define __AppHostAdminManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class AppHostAdminManager AppHostAdminManager;
#else
typedef struct AppHostAdminManager AppHostAdminManager;
#endif /* __cplusplus */

#endif 	/* __AppHostAdminManager_FWD_DEFINED__ */


#ifndef __AppHostWritableAdminManager_FWD_DEFINED__
#define __AppHostWritableAdminManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class AppHostWritableAdminManager AppHostWritableAdminManager;
#else
typedef struct AppHostWritableAdminManager AppHostWritableAdminManager;
#endif /* __cplusplus */

#endif 	/* __AppHostWritableAdminManager_FWD_DEFINED__ */


#ifndef __IAppHostConfigException_FWD_DEFINED__
#define __IAppHostConfigException_FWD_DEFINED__
typedef interface IAppHostConfigException IAppHostConfigException;

#endif 	/* __IAppHostConfigException_FWD_DEFINED__ */


#ifndef __IAppHostPropertyException_FWD_DEFINED__
#define __IAppHostPropertyException_FWD_DEFINED__
typedef interface IAppHostPropertyException IAppHostPropertyException;

#endif 	/* __IAppHostPropertyException_FWD_DEFINED__ */


#ifndef __IAppHostMappingExtension_FWD_DEFINED__
#define __IAppHostMappingExtension_FWD_DEFINED__
typedef interface IAppHostMappingExtension IAppHostMappingExtension;

#endif 	/* __IAppHostMappingExtension_FWD_DEFINED__ */


#ifndef __IAppHostPathMapper_FWD_DEFINED__
#define __IAppHostPathMapper_FWD_DEFINED__
typedef interface IAppHostPathMapper IAppHostPathMapper;

#endif 	/* __IAppHostPathMapper_FWD_DEFINED__ */


#ifndef __IAppHostChangeHandler_FWD_DEFINED__
#define __IAppHostChangeHandler_FWD_DEFINED__
typedef interface IAppHostChangeHandler IAppHostChangeHandler;

#endif 	/* __IAppHostChangeHandler_FWD_DEFINED__ */


#ifndef __IAppHostPropertyExtension_FWD_DEFINED__
#define __IAppHostPropertyExtension_FWD_DEFINED__
typedef interface IAppHostPropertyExtension IAppHostPropertyExtension;

#endif 	/* __IAppHostPropertyExtension_FWD_DEFINED__ */


#ifndef __IAppHostElementExtension_FWD_DEFINED__
#define __IAppHostElementExtension_FWD_DEFINED__
typedef interface IAppHostElementExtension IAppHostElementExtension;

#endif 	/* __IAppHostElementExtension_FWD_DEFINED__ */


#ifndef __IAppHostMethodExtension_FWD_DEFINED__
#define __IAppHostMethodExtension_FWD_DEFINED__
typedef interface IAppHostMethodExtension IAppHostMethodExtension;

#endif 	/* __IAppHostMethodExtension_FWD_DEFINED__ */


#ifndef __IAppHostPathMapper2_FWD_DEFINED__
#define __IAppHostPathMapper2_FWD_DEFINED__
typedef interface IAppHostPathMapper2 IAppHostPathMapper2;

#endif 	/* __IAppHostPathMapper2_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ahadmin_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



























extern RPC_IF_HANDLE __MIDL_itf_ahadmin_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ahadmin_0000_0000_v0_0_s_ifspec;

#ifndef __IAppHostMethodExtension_INTERFACE_DEFINED__
#define __IAppHostMethodExtension_INTERFACE_DEFINED__

/* interface IAppHostMethodExtension */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMethodExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70184ac9-7673-4770-96b1-445ce035cf70")
    IAppHostMethodExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProvideMethod( 
            /* [in] */ __RPC__in_opt IAppHostMethod *pMethod,
            /* [in] */ __RPC__in_opt IAppHostMethodInstance *pMethodInstance,
            /* [in] */ __RPC__in_opt IAppHostElement *pElement) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMethodExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMethodExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMethodExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMethodExtension * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethodExtension, ProvideMethod)
        HRESULT ( STDMETHODCALLTYPE *ProvideMethod )( 
            __RPC__in IAppHostMethodExtension * This,
            /* [in] */ __RPC__in_opt IAppHostMethod *pMethod,
            /* [in] */ __RPC__in_opt IAppHostMethodInstance *pMethodInstance,
            /* [in] */ __RPC__in_opt IAppHostElement *pElement);
        
        END_INTERFACE
    } IAppHostMethodExtensionVtbl;

    interface IAppHostMethodExtension
    {
        CONST_VTBL struct IAppHostMethodExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMethodExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMethodExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMethodExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMethodExtension_ProvideMethod(This,pMethod,pMethodInstance,pElement)	\
    ( (This)->lpVtbl -> ProvideMethod(This,pMethod,pMethodInstance,pElement) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMethodExtension_INTERFACE_DEFINED__ */


#ifndef __IAppHostPropertyExtension_INTERFACE_DEFINED__
#define __IAppHostPropertyExtension_INTERFACE_DEFINED__

/* interface IAppHostPropertyExtension */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPropertyExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88e220f3-33e8-4534-afac-b4a98eccf9ae")
    IAppHostPropertyExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProvideGetProperty( 
            /* [in] */ __RPC__in_opt IAppHostElement *pElement,
            /* [in] */ __RPC__in_opt IAppHostProperty *pProperty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertyExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPropertyExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPropertyExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPropertyExtension * This);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyExtension, ProvideGetProperty)
        HRESULT ( STDMETHODCALLTYPE *ProvideGetProperty )( 
            __RPC__in IAppHostPropertyExtension * This,
            /* [in] */ __RPC__in_opt IAppHostElement *pElement,
            /* [in] */ __RPC__in_opt IAppHostProperty *pProperty);
        
        END_INTERFACE
    } IAppHostPropertyExtensionVtbl;

    interface IAppHostPropertyExtension
    {
        CONST_VTBL struct IAppHostPropertyExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPropertyExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPropertyExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPropertyExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPropertyExtension_ProvideGetProperty(This,pElement,pProperty)	\
    ( (This)->lpVtbl -> ProvideGetProperty(This,pElement,pProperty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPropertyExtension_INTERFACE_DEFINED__ */


#ifndef __IAppHostElementExtension_INTERFACE_DEFINED__
#define __IAppHostElementExtension_INTERFACE_DEFINED__

/* interface IAppHostElementExtension */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostElementExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("518eb37d-1ff4-42dd-86c3-3140bc35b823")
    IAppHostElementExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProvideElement( 
            /* [in] */ __RPC__in_opt IAppHostElement *pNewElement) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostElementExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostElementExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostElementExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostElementExtension * This);
        
        DECLSPEC_XFGVIRT(IAppHostElementExtension, ProvideElement)
        HRESULT ( STDMETHODCALLTYPE *ProvideElement )( 
            __RPC__in IAppHostElementExtension * This,
            /* [in] */ __RPC__in_opt IAppHostElement *pNewElement);
        
        END_INTERFACE
    } IAppHostElementExtensionVtbl;

    interface IAppHostElementExtension
    {
        CONST_VTBL struct IAppHostElementExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostElementExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostElementExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostElementExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostElementExtension_ProvideElement(This,pNewElement)	\
    ( (This)->lpVtbl -> ProvideElement(This,pNewElement) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostElementExtension_INTERFACE_DEFINED__ */


#ifndef __IAppHostMappingExtension_INTERFACE_DEFINED__
#define __IAppHostMappingExtension_INTERFACE_DEFINED__

/* interface IAppHostMappingExtension */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMappingExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31a83ea0-c0e4-4a2c-8a01-353cc2a4c60a")
    IAppHostMappingExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSiteNameFromSiteId( 
            /* [in] */ DWORD dwSiteId,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrSiteName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSiteIdFromSiteName( 
            /* [string][in] */ __RPC__in_string BSTR bstrSiteName,
            /* [retval][out] */ __RPC__out DWORD *pdwSiteId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSiteElementFromSiteId( 
            /* [in] */ DWORD dwSiteId,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppSiteElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MapPath( 
            /* [string][in] */ __RPC__in_string BSTR bstrSiteName,
            /* [string][in] */ __RPC__in_string BSTR bstrVirtualPath,
            /* [string][out] */ __RPC__deref_out_opt_string BSTR *pbstrPhysicalPath,
            /* [out] */ __RPC__deref_out_opt IAppHostElement **ppVirtualDirectoryElement,
            /* [out] */ __RPC__deref_out_opt IAppHostElement **ppApplicationElement) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMappingExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMappingExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMappingExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMappingExtension * This);
        
        DECLSPEC_XFGVIRT(IAppHostMappingExtension, GetSiteNameFromSiteId)
        HRESULT ( STDMETHODCALLTYPE *GetSiteNameFromSiteId )( 
            __RPC__in IAppHostMappingExtension * This,
            /* [in] */ DWORD dwSiteId,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrSiteName);
        
        DECLSPEC_XFGVIRT(IAppHostMappingExtension, GetSiteIdFromSiteName)
        HRESULT ( STDMETHODCALLTYPE *GetSiteIdFromSiteName )( 
            __RPC__in IAppHostMappingExtension * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSiteName,
            /* [retval][out] */ __RPC__out DWORD *pdwSiteId);
        
        DECLSPEC_XFGVIRT(IAppHostMappingExtension, GetSiteElementFromSiteId)
        HRESULT ( STDMETHODCALLTYPE *GetSiteElementFromSiteId )( 
            __RPC__in IAppHostMappingExtension * This,
            /* [in] */ DWORD dwSiteId,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppSiteElement);
        
        DECLSPEC_XFGVIRT(IAppHostMappingExtension, MapPath)
        HRESULT ( STDMETHODCALLTYPE *MapPath )( 
            __RPC__in IAppHostMappingExtension * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSiteName,
            /* [string][in] */ __RPC__in_string BSTR bstrVirtualPath,
            /* [string][out] */ __RPC__deref_out_opt_string BSTR *pbstrPhysicalPath,
            /* [out] */ __RPC__deref_out_opt IAppHostElement **ppVirtualDirectoryElement,
            /* [out] */ __RPC__deref_out_opt IAppHostElement **ppApplicationElement);
        
        END_INTERFACE
    } IAppHostMappingExtensionVtbl;

    interface IAppHostMappingExtension
    {
        CONST_VTBL struct IAppHostMappingExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMappingExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMappingExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMappingExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMappingExtension_GetSiteNameFromSiteId(This,dwSiteId,pbstrSiteName)	\
    ( (This)->lpVtbl -> GetSiteNameFromSiteId(This,dwSiteId,pbstrSiteName) ) 

#define IAppHostMappingExtension_GetSiteIdFromSiteName(This,bstrSiteName,pdwSiteId)	\
    ( (This)->lpVtbl -> GetSiteIdFromSiteName(This,bstrSiteName,pdwSiteId) ) 

#define IAppHostMappingExtension_GetSiteElementFromSiteId(This,dwSiteId,ppSiteElement)	\
    ( (This)->lpVtbl -> GetSiteElementFromSiteId(This,dwSiteId,ppSiteElement) ) 

#define IAppHostMappingExtension_MapPath(This,bstrSiteName,bstrVirtualPath,pbstrPhysicalPath,ppVirtualDirectoryElement,ppApplicationElement)	\
    ( (This)->lpVtbl -> MapPath(This,bstrSiteName,bstrVirtualPath,pbstrPhysicalPath,ppVirtualDirectoryElement,ppApplicationElement) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMappingExtension_INTERFACE_DEFINED__ */


#ifndef __IAppHostChildElementCollection_INTERFACE_DEFINED__
#define __IAppHostChildElementCollection_INTERFACE_DEFINED__

/* interface IAppHostChildElementCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostChildElementCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08a90f5f-0702-48d6-b45f-02a9885a9768")
    IAppHostChildElementCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostChildElementCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostChildElementCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostChildElementCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostChildElementCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostChildElementCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostChildElementCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostChildElementCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostChildElementCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement);
        
        END_INTERFACE
    } IAppHostChildElementCollectionVtbl;

    interface IAppHostChildElementCollection
    {
        CONST_VTBL struct IAppHostChildElementCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostChildElementCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostChildElementCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostChildElementCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostChildElementCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostChildElementCollection_get_Item(This,cIndex,ppElement)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppElement) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostChildElementCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostPropertyCollection_INTERFACE_DEFINED__
#define __IAppHostPropertyCollection_INTERFACE_DEFINED__

/* interface IAppHostPropertyCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPropertyCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0191775e-bcff-445a-b4f4-3bdda54e2816")
    IAppHostPropertyCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostProperty **ppProperty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertyCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPropertyCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostPropertyCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostPropertyCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostProperty **ppProperty);
        
        END_INTERFACE
    } IAppHostPropertyCollectionVtbl;

    interface IAppHostPropertyCollection
    {
        CONST_VTBL struct IAppHostPropertyCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPropertyCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPropertyCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPropertyCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPropertyCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostPropertyCollection_get_Item(This,cIndex,ppProperty)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppProperty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPropertyCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostConfigLocationCollection_INTERFACE_DEFINED__
#define __IAppHostConfigLocationCollection_INTERFACE_DEFINED__

/* interface IAppHostConfigLocationCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConfigLocationCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("832a32f7-b3ea-4b8c-b260-9a2923001184")
    IAppHostConfigLocationCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocation **ppLocation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddLocation( 
            /* [string][in] */ __RPC__in_string BSTR bstrLocationPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocation **ppNewLocation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteLocation( 
            /* [in] */ VARIANT cIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenameLocation( 
            /* [in] */ VARIANT varIndex,
            /* [string][in] */ __RPC__in_string BSTR bstrLocationPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConfigLocationCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConfigLocationCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConfigLocationCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocationCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocationCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocation **ppLocation);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocationCollection, AddLocation)
        HRESULT ( STDMETHODCALLTYPE *AddLocation )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [string][in] */ __RPC__in_string BSTR bstrLocationPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocation **ppNewLocation);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocationCollection, DeleteLocation)
        HRESULT ( STDMETHODCALLTYPE *DeleteLocation )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [in] */ VARIANT cIndex);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocationCollection, RenameLocation)
        HRESULT ( STDMETHODCALLTYPE *RenameLocation )( 
            __RPC__in IAppHostConfigLocationCollection * This,
            /* [in] */ VARIANT varIndex,
            /* [string][in] */ __RPC__in_string BSTR bstrLocationPath);
        
        END_INTERFACE
    } IAppHostConfigLocationCollectionVtbl;

    interface IAppHostConfigLocationCollection
    {
        CONST_VTBL struct IAppHostConfigLocationCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConfigLocationCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConfigLocationCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConfigLocationCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConfigLocationCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostConfigLocationCollection_get_Item(This,varIndex,ppLocation)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppLocation) ) 

#define IAppHostConfigLocationCollection_AddLocation(This,bstrLocationPath,ppNewLocation)	\
    ( (This)->lpVtbl -> AddLocation(This,bstrLocationPath,ppNewLocation) ) 

#define IAppHostConfigLocationCollection_DeleteLocation(This,cIndex)	\
    ( (This)->lpVtbl -> DeleteLocation(This,cIndex) ) 

#define IAppHostConfigLocationCollection_RenameLocation(This,varIndex,bstrLocationPath)	\
    ( (This)->lpVtbl -> RenameLocation(This,varIndex,bstrLocationPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConfigLocationCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostMethodCollection_INTERFACE_DEFINED__
#define __IAppHostMethodCollection_INTERFACE_DEFINED__

/* interface IAppHostMethodCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMethodCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d6c7cd8f-bb8d-4f96-b591-d3a5f1320269")
    IAppHostMethodCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethod **ppMethod) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMethodCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMethodCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMethodCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMethodCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethodCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostMethodCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostMethodCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostMethodCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethod **ppMethod);
        
        END_INTERFACE
    } IAppHostMethodCollectionVtbl;

    interface IAppHostMethodCollection
    {
        CONST_VTBL struct IAppHostMethodCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMethodCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMethodCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMethodCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMethodCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostMethodCollection_get_Item(This,cIndex,ppMethod)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppMethod) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMethodCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostElementSchemaCollection_INTERFACE_DEFINED__
#define __IAppHostElementSchemaCollection_INTERFACE_DEFINED__

/* interface IAppHostElementSchemaCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostElementSchemaCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0344cdda-151e-4cbf-82da-66ae61e97754")
    IAppHostElementSchemaCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppElementSchema) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostElementSchemaCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostElementSchemaCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostElementSchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostElementSchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchemaCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostElementSchemaCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchemaCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostElementSchemaCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppElementSchema);
        
        END_INTERFACE
    } IAppHostElementSchemaCollectionVtbl;

    interface IAppHostElementSchemaCollection
    {
        CONST_VTBL struct IAppHostElementSchemaCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostElementSchemaCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostElementSchemaCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostElementSchemaCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostElementSchemaCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostElementSchemaCollection_get_Item(This,cIndex,ppElementSchema)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppElementSchema) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostElementSchemaCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostPropertySchemaCollection_INTERFACE_DEFINED__
#define __IAppHostPropertySchemaCollection_INTERFACE_DEFINED__

/* interface IAppHostPropertySchemaCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPropertySchemaCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8bed2c68-a5fb-4b28-8581-a0dc5267419f")
    IAppHostPropertySchemaCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchema **ppPropertySchema) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertySchemaCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPropertySchemaCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPropertySchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPropertySchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchemaCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostPropertySchemaCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchemaCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostPropertySchemaCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchema **ppPropertySchema);
        
        END_INTERFACE
    } IAppHostPropertySchemaCollectionVtbl;

    interface IAppHostPropertySchemaCollection
    {
        CONST_VTBL struct IAppHostPropertySchemaCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPropertySchemaCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPropertySchemaCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPropertySchemaCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPropertySchemaCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostPropertySchemaCollection_get_Item(This,cIndex,ppPropertySchema)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppPropertySchema) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPropertySchemaCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostConstantValueCollection_INTERFACE_DEFINED__
#define __IAppHostConstantValueCollection_INTERFACE_DEFINED__

/* interface IAppHostConstantValueCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConstantValueCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5b5a68e6-8b9f-45e1-8199-a95ffccdffff")
    IAppHostConstantValueCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConstantValue **ppConstantValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConstantValueCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConstantValueCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConstantValueCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConstantValueCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostConstantValueCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostConstantValueCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostConstantValueCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostConstantValueCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConstantValue **ppConstantValue);
        
        END_INTERFACE
    } IAppHostConstantValueCollectionVtbl;

    interface IAppHostConstantValueCollection
    {
        CONST_VTBL struct IAppHostConstantValueCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConstantValueCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConstantValueCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConstantValueCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConstantValueCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostConstantValueCollection_get_Item(This,cIndex,ppConstantValue)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppConstantValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConstantValueCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostConstantValue_INTERFACE_DEFINED__
#define __IAppHostConstantValue_INTERFACE_DEFINED__

/* interface IAppHostConstantValue */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConstantValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0716caf8-7d05-4a46-8099-77594be91394")
    IAppHostConstantValue : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out DWORD *pdwValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConstantValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConstantValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConstantValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConstantValue * This);
        
        DECLSPEC_XFGVIRT(IAppHostConstantValue, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostConstantValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostConstantValue, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IAppHostConstantValue * This,
            /* [retval][out] */ __RPC__out DWORD *pdwValue);
        
        END_INTERFACE
    } IAppHostConstantValueVtbl;

    interface IAppHostConstantValue
    {
        CONST_VTBL struct IAppHostConstantValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConstantValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConstantValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConstantValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConstantValue_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostConstantValue_get_Value(This,pdwValue)	\
    ( (This)->lpVtbl -> get_Value(This,pdwValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConstantValue_INTERFACE_DEFINED__ */


#ifndef __IAppHostPropertySchema_INTERFACE_DEFINED__
#define __IAppHostPropertySchema_INTERFACE_DEFINED__

/* interface IAppHostPropertySchema */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPropertySchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("450386db-7409-4667-935e-384dbbee2a9e")
    IAppHostPropertySchema : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DefaultValue( 
            /* [retval][out] */ __RPC__out VARIANT *pDefaultValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsRequired( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsRequired) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsUniqueKey( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsUniqueKey) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsCombinedKey( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCombinedKey) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsExpanded( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsExpanded) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValidationType( 
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValidationParameter( 
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationParameter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsCaseSensitive( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCaseSensitive) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PossibleValues( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConstantValueCollection **ppValues) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DoesAllowInfinite( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowInfinite) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsEncrypted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsEncrypted) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TimeSpanFormat( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTimeSpanFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertySchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPropertySchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPropertySchema * This);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_DefaultValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultValue )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT *pDefaultValue);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsRequired)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsRequired )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsRequired);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsUniqueKey)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsUniqueKey )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsUniqueKey);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsCombinedKey)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsCombinedKey )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCombinedKey);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsExpanded)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsExpanded )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsExpanded);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_ValidationType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValidationType )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationType);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_ValidationParameter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValidationParameter )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationParameter);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsCaseSensitive)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsCaseSensitive )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCaseSensitive);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_PossibleValues)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PossibleValues )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConstantValueCollection **ppValues);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_DoesAllowInfinite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoesAllowInfinite )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowInfinite);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_IsEncrypted)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsEncrypted )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsEncrypted);
        
        DECLSPEC_XFGVIRT(IAppHostPropertySchema, get_TimeSpanFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TimeSpanFormat )( 
            __RPC__in IAppHostPropertySchema * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTimeSpanFormat);
        
        END_INTERFACE
    } IAppHostPropertySchemaVtbl;

    interface IAppHostPropertySchema
    {
        CONST_VTBL struct IAppHostPropertySchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPropertySchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPropertySchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPropertySchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPropertySchema_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostPropertySchema_get_Type(This,pbstrType)	\
    ( (This)->lpVtbl -> get_Type(This,pbstrType) ) 

#define IAppHostPropertySchema_get_DefaultValue(This,pDefaultValue)	\
    ( (This)->lpVtbl -> get_DefaultValue(This,pDefaultValue) ) 

#define IAppHostPropertySchema_get_IsRequired(This,pfIsRequired)	\
    ( (This)->lpVtbl -> get_IsRequired(This,pfIsRequired) ) 

#define IAppHostPropertySchema_get_IsUniqueKey(This,pfIsUniqueKey)	\
    ( (This)->lpVtbl -> get_IsUniqueKey(This,pfIsUniqueKey) ) 

#define IAppHostPropertySchema_get_IsCombinedKey(This,pfIsCombinedKey)	\
    ( (This)->lpVtbl -> get_IsCombinedKey(This,pfIsCombinedKey) ) 

#define IAppHostPropertySchema_get_IsExpanded(This,pfIsExpanded)	\
    ( (This)->lpVtbl -> get_IsExpanded(This,pfIsExpanded) ) 

#define IAppHostPropertySchema_get_ValidationType(This,pbstrValidationType)	\
    ( (This)->lpVtbl -> get_ValidationType(This,pbstrValidationType) ) 

#define IAppHostPropertySchema_get_ValidationParameter(This,pbstrValidationParameter)	\
    ( (This)->lpVtbl -> get_ValidationParameter(This,pbstrValidationParameter) ) 

#define IAppHostPropertySchema_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostPropertySchema_get_IsCaseSensitive(This,pfIsCaseSensitive)	\
    ( (This)->lpVtbl -> get_IsCaseSensitive(This,pfIsCaseSensitive) ) 

#define IAppHostPropertySchema_get_PossibleValues(This,ppValues)	\
    ( (This)->lpVtbl -> get_PossibleValues(This,ppValues) ) 

#define IAppHostPropertySchema_get_DoesAllowInfinite(This,pfAllowInfinite)	\
    ( (This)->lpVtbl -> get_DoesAllowInfinite(This,pfAllowInfinite) ) 

#define IAppHostPropertySchema_get_IsEncrypted(This,pfIsEncrypted)	\
    ( (This)->lpVtbl -> get_IsEncrypted(This,pfIsEncrypted) ) 

#define IAppHostPropertySchema_get_TimeSpanFormat(This,pbstrTimeSpanFormat)	\
    ( (This)->lpVtbl -> get_TimeSpanFormat(This,pbstrTimeSpanFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPropertySchema_INTERFACE_DEFINED__ */


#ifndef __IAppHostCollectionSchema_INTERFACE_DEFINED__
#define __IAppHostCollectionSchema_INTERFACE_DEFINED__

/* interface IAppHostCollectionSchema */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostCollectionSchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("de095db1-5368-4d11-81f6-efef619b7bcf")
    IAppHostCollectionSchema : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AddElementNames( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrElementName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAddElementSchema( 
            /* [string][in] */ __RPC__in_string BSTR bstrElementName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RemoveElementSchema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClearElementSchema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsMergeAppend( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsMergeAppend) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DoesAllowDuplicates( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowDuplicates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostCollectionSchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostCollectionSchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostCollectionSchema * This);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, get_AddElementNames)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddElementNames )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrElementName);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, GetAddElementSchema)
        HRESULT ( STDMETHODCALLTYPE *GetAddElementSchema )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [string][in] */ __RPC__in_string BSTR bstrElementName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, get_RemoveElementSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoveElementSchema )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, get_ClearElementSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClearElementSchema )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, get_IsMergeAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsMergeAppend )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsMergeAppend);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostCollectionSchema, get_DoesAllowDuplicates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoesAllowDuplicates )( 
            __RPC__in IAppHostCollectionSchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowDuplicates);
        
        END_INTERFACE
    } IAppHostCollectionSchemaVtbl;

    interface IAppHostCollectionSchema
    {
        CONST_VTBL struct IAppHostCollectionSchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostCollectionSchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostCollectionSchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostCollectionSchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostCollectionSchema_get_AddElementNames(This,pbstrElementName)	\
    ( (This)->lpVtbl -> get_AddElementNames(This,pbstrElementName) ) 

#define IAppHostCollectionSchema_GetAddElementSchema(This,bstrElementName,ppSchema)	\
    ( (This)->lpVtbl -> GetAddElementSchema(This,bstrElementName,ppSchema) ) 

#define IAppHostCollectionSchema_get_RemoveElementSchema(This,ppSchema)	\
    ( (This)->lpVtbl -> get_RemoveElementSchema(This,ppSchema) ) 

#define IAppHostCollectionSchema_get_ClearElementSchema(This,ppSchema)	\
    ( (This)->lpVtbl -> get_ClearElementSchema(This,ppSchema) ) 

#define IAppHostCollectionSchema_get_IsMergeAppend(This,pfIsMergeAppend)	\
    ( (This)->lpVtbl -> get_IsMergeAppend(This,pfIsMergeAppend) ) 

#define IAppHostCollectionSchema_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostCollectionSchema_get_DoesAllowDuplicates(This,pfAllowDuplicates)	\
    ( (This)->lpVtbl -> get_DoesAllowDuplicates(This,pfAllowDuplicates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostCollectionSchema_INTERFACE_DEFINED__ */


#ifndef __IAppHostElementSchema_INTERFACE_DEFINED__
#define __IAppHostElementSchema_INTERFACE_DEFINED__

/* interface IAppHostElementSchema */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostElementSchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ef13d885-642c-4709-99ec-b89561c6bc69")
    IAppHostElementSchema : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DoesAllowUnschematizedProperties( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowUnschematized) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CollectionSchema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostCollectionSchema **ppCollectionSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ChildElementSchemas( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchemaCollection **ppChildSchemas) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PropertySchemas( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchemaCollection **ppPropertySchemas) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsCollectionDefault( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCollectionDefault) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostElementSchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostElementSchema * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostElementSchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostElementSchema * This);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_DoesAllowUnschematizedProperties)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoesAllowUnschematizedProperties )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfAllowUnschematized);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostElementSchema * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_CollectionSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectionSchema )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostCollectionSchema **ppCollectionSchema);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_ChildElementSchemas)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChildElementSchemas )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchemaCollection **ppChildSchemas);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_PropertySchemas)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PropertySchemas )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchemaCollection **ppPropertySchemas);
        
        DECLSPEC_XFGVIRT(IAppHostElementSchema, get_IsCollectionDefault)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsCollectionDefault )( 
            __RPC__in IAppHostElementSchema * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsCollectionDefault);
        
        END_INTERFACE
    } IAppHostElementSchemaVtbl;

    interface IAppHostElementSchema
    {
        CONST_VTBL struct IAppHostElementSchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostElementSchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostElementSchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostElementSchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostElementSchema_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostElementSchema_get_DoesAllowUnschematizedProperties(This,pfAllowUnschematized)	\
    ( (This)->lpVtbl -> get_DoesAllowUnschematizedProperties(This,pfAllowUnschematized) ) 

#define IAppHostElementSchema_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostElementSchema_get_CollectionSchema(This,ppCollectionSchema)	\
    ( (This)->lpVtbl -> get_CollectionSchema(This,ppCollectionSchema) ) 

#define IAppHostElementSchema_get_ChildElementSchemas(This,ppChildSchemas)	\
    ( (This)->lpVtbl -> get_ChildElementSchemas(This,ppChildSchemas) ) 

#define IAppHostElementSchema_get_PropertySchemas(This,ppPropertySchemas)	\
    ( (This)->lpVtbl -> get_PropertySchemas(This,ppPropertySchemas) ) 

#define IAppHostElementSchema_get_IsCollectionDefault(This,pfIsCollectionDefault)	\
    ( (This)->lpVtbl -> get_IsCollectionDefault(This,pfIsCollectionDefault) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostElementSchema_INTERFACE_DEFINED__ */


#ifndef __IAppHostMethodSchema_INTERFACE_DEFINED__
#define __IAppHostMethodSchema_INTERFACE_DEFINED__

/* interface IAppHostMethodSchema */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMethodSchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2d9915fb-9d42-4328-b782-1b46819fab9e")
    IAppHostMethodSchema : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_InputSchema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppInputSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OutputSchema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppOutputSchema) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMethodSchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMethodSchema * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMethodSchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMethodSchema * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethodSchema, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostMethodSchema * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostMethodSchema, get_InputSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_InputSchema )( 
            __RPC__in IAppHostMethodSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppInputSchema);
        
        DECLSPEC_XFGVIRT(IAppHostMethodSchema, get_OutputSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputSchema )( 
            __RPC__in IAppHostMethodSchema * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppOutputSchema);
        
        DECLSPEC_XFGVIRT(IAppHostMethodSchema, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostMethodSchema * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        END_INTERFACE
    } IAppHostMethodSchemaVtbl;

    interface IAppHostMethodSchema
    {
        CONST_VTBL struct IAppHostMethodSchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMethodSchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMethodSchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMethodSchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMethodSchema_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostMethodSchema_get_InputSchema(This,ppInputSchema)	\
    ( (This)->lpVtbl -> get_InputSchema(This,ppInputSchema) ) 

#define IAppHostMethodSchema_get_OutputSchema(This,ppOutputSchema)	\
    ( (This)->lpVtbl -> get_OutputSchema(This,ppOutputSchema) ) 

#define IAppHostMethodSchema_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMethodSchema_INTERFACE_DEFINED__ */


#ifndef __IAppHostMethodInstance_INTERFACE_DEFINED__
#define __IAppHostMethodInstance_INTERFACE_DEFINED__

/* interface IAppHostMethodInstance */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMethodInstance;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b80f3c42-60e0-4ae0-9007-f52852d3dbed")
    IAppHostMethodInstance : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Input( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppInputElement) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Output( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppOutputElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Execute( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMethodInstanceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMethodInstance * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMethodInstance * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMethodInstance * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethodInstance, get_Input)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Input )( 
            __RPC__in IAppHostMethodInstance * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppInputElement);
        
        DECLSPEC_XFGVIRT(IAppHostMethodInstance, get_Output)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Output )( 
            __RPC__in IAppHostMethodInstance * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppOutputElement);
        
        DECLSPEC_XFGVIRT(IAppHostMethodInstance, Execute)
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in IAppHostMethodInstance * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethodInstance, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostMethodInstance * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostMethodInstance, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostMethodInstance * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        END_INTERFACE
    } IAppHostMethodInstanceVtbl;

    interface IAppHostMethodInstance
    {
        CONST_VTBL struct IAppHostMethodInstanceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMethodInstance_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMethodInstance_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMethodInstance_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMethodInstance_get_Input(This,ppInputElement)	\
    ( (This)->lpVtbl -> get_Input(This,ppInputElement) ) 

#define IAppHostMethodInstance_get_Output(This,ppOutputElement)	\
    ( (This)->lpVtbl -> get_Output(This,ppOutputElement) ) 

#define IAppHostMethodInstance_Execute(This)	\
    ( (This)->lpVtbl -> Execute(This) ) 

#define IAppHostMethodInstance_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostMethodInstance_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMethodInstance_INTERFACE_DEFINED__ */


#ifndef __IAppHostMethod_INTERFACE_DEFINED__
#define __IAppHostMethod_INTERFACE_DEFINED__

/* interface IAppHostMethod */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostMethod;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7883ca1c-1112-4447-84c3-52fbeb38069d")
    IAppHostMethod : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Schema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodSchema **ppMethodSchema) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodInstance **ppMethodInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostMethodVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostMethod * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostMethod * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostMethod * This);
        
        DECLSPEC_XFGVIRT(IAppHostMethod, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostMethod * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostMethod, get_Schema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IAppHostMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodSchema **ppMethodSchema);
        
        DECLSPEC_XFGVIRT(IAppHostMethod, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in IAppHostMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodInstance **ppMethodInstance);
        
        END_INTERFACE
    } IAppHostMethodVtbl;

    interface IAppHostMethod
    {
        CONST_VTBL struct IAppHostMethodVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostMethod_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostMethod_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostMethod_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostMethod_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostMethod_get_Schema(This,ppMethodSchema)	\
    ( (This)->lpVtbl -> get_Schema(This,ppMethodSchema) ) 

#define IAppHostMethod_CreateInstance(This,ppMethodInstance)	\
    ( (This)->lpVtbl -> CreateInstance(This,ppMethodInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostMethod_INTERFACE_DEFINED__ */


#ifndef __IAppHostConfigException_INTERFACE_DEFINED__
#define __IAppHostConfigException_INTERFACE_DEFINED__

/* interface IAppHostConfigException */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConfigException;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4dfa1df3-8900-4bc7-bbb5-d1a458c52410")
    IAppHostConfigException : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LineNumber( 
            /* [retval][out] */ __RPC__out ULONG *pcLineNumber) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ConfigPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrConfigPath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ErrorLine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorLine) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PreErrorLine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPreErrorLine) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PostErrorLine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPostErrorLine) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ErrorString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConfigExceptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConfigException * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConfigException * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConfigException * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_LineNumber)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineNumber )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__out ULONG *pcLineNumber);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ConfigPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigPath )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrConfigPath);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorLine )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_PreErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreErrorLine )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPreErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_PostErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostErrorLine )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPostErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ErrorString)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorString )( 
            __RPC__in IAppHostConfigException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorString);
        
        END_INTERFACE
    } IAppHostConfigExceptionVtbl;

    interface IAppHostConfigException
    {
        CONST_VTBL struct IAppHostConfigExceptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConfigException_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConfigException_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConfigException_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConfigException_get_LineNumber(This,pcLineNumber)	\
    ( (This)->lpVtbl -> get_LineNumber(This,pcLineNumber) ) 

#define IAppHostConfigException_get_FileName(This,pbstrFileName)	\
    ( (This)->lpVtbl -> get_FileName(This,pbstrFileName) ) 

#define IAppHostConfigException_get_ConfigPath(This,pbstrConfigPath)	\
    ( (This)->lpVtbl -> get_ConfigPath(This,pbstrConfigPath) ) 

#define IAppHostConfigException_get_ErrorLine(This,pbstrErrorLine)	\
    ( (This)->lpVtbl -> get_ErrorLine(This,pbstrErrorLine) ) 

#define IAppHostConfigException_get_PreErrorLine(This,pbstrPreErrorLine)	\
    ( (This)->lpVtbl -> get_PreErrorLine(This,pbstrPreErrorLine) ) 

#define IAppHostConfigException_get_PostErrorLine(This,pbstrPostErrorLine)	\
    ( (This)->lpVtbl -> get_PostErrorLine(This,pbstrPostErrorLine) ) 

#define IAppHostConfigException_get_ErrorString(This,pbstrErrorString)	\
    ( (This)->lpVtbl -> get_ErrorString(This,pbstrErrorString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConfigException_INTERFACE_DEFINED__ */


#ifndef __IAppHostPropertyException_INTERFACE_DEFINED__
#define __IAppHostPropertyException_INTERFACE_DEFINED__

/* interface IAppHostPropertyException */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPropertyException;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eafe4895-a929-41ea-b14d-613e23f62b71")
    IAppHostPropertyException : public IAppHostConfigException
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_InvalidValue( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValidationFailureReason( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationReason) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValidationFailureParameters( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pParameterArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertyExceptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPropertyException * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPropertyException * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPropertyException * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_LineNumber)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineNumber )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__out ULONG *pcLineNumber);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ConfigPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigPath )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrConfigPath);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorLine )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_PreErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreErrorLine )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPreErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_PostErrorLine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostErrorLine )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPostErrorLine);
        
        DECLSPEC_XFGVIRT(IAppHostConfigException, get_ErrorString)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorString )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrErrorString);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyException, get_InvalidValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_InvalidValue )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyException, get_ValidationFailureReason)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValidationFailureReason )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrValidationReason);
        
        DECLSPEC_XFGVIRT(IAppHostPropertyException, get_ValidationFailureParameters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValidationFailureParameters )( 
            __RPC__in IAppHostPropertyException * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pParameterArray);
        
        END_INTERFACE
    } IAppHostPropertyExceptionVtbl;

    interface IAppHostPropertyException
    {
        CONST_VTBL struct IAppHostPropertyExceptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPropertyException_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPropertyException_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPropertyException_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPropertyException_get_LineNumber(This,pcLineNumber)	\
    ( (This)->lpVtbl -> get_LineNumber(This,pcLineNumber) ) 

#define IAppHostPropertyException_get_FileName(This,pbstrFileName)	\
    ( (This)->lpVtbl -> get_FileName(This,pbstrFileName) ) 

#define IAppHostPropertyException_get_ConfigPath(This,pbstrConfigPath)	\
    ( (This)->lpVtbl -> get_ConfigPath(This,pbstrConfigPath) ) 

#define IAppHostPropertyException_get_ErrorLine(This,pbstrErrorLine)	\
    ( (This)->lpVtbl -> get_ErrorLine(This,pbstrErrorLine) ) 

#define IAppHostPropertyException_get_PreErrorLine(This,pbstrPreErrorLine)	\
    ( (This)->lpVtbl -> get_PreErrorLine(This,pbstrPreErrorLine) ) 

#define IAppHostPropertyException_get_PostErrorLine(This,pbstrPostErrorLine)	\
    ( (This)->lpVtbl -> get_PostErrorLine(This,pbstrPostErrorLine) ) 

#define IAppHostPropertyException_get_ErrorString(This,pbstrErrorString)	\
    ( (This)->lpVtbl -> get_ErrorString(This,pbstrErrorString) ) 


#define IAppHostPropertyException_get_InvalidValue(This,pbstrValue)	\
    ( (This)->lpVtbl -> get_InvalidValue(This,pbstrValue) ) 

#define IAppHostPropertyException_get_ValidationFailureReason(This,pbstrValidationReason)	\
    ( (This)->lpVtbl -> get_ValidationFailureReason(This,pbstrValidationReason) ) 

#define IAppHostPropertyException_get_ValidationFailureParameters(This,pParameterArray)	\
    ( (This)->lpVtbl -> get_ValidationFailureParameters(This,pParameterArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPropertyException_INTERFACE_DEFINED__ */


#ifndef __IAppHostElementCollection_INTERFACE_DEFINED__
#define __IAppHostElementCollection_INTERFACE_DEFINED__

/* interface IAppHostElementCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostElementCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8550bff-5281-4b1e-ac34-99b6fa38464d")
    IAppHostElementCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcElementCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddElement( 
            /* [in] */ __RPC__in_opt IAppHostElement *pElement,
            /* [defaultvalue][in] */ INT cPosition = -1) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteElement( 
            /* [in] */ VARIANT cIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateNewElement( 
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR bstrElementName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Schema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostCollectionSchema **ppSchema) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostElementCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostElementCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostElementCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostElementCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostElementCollection * This,
            /* [retval][out] */ __RPC__out DWORD *pcElementCount);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostElementCollection * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, AddElement)
        HRESULT ( STDMETHODCALLTYPE *AddElement )( 
            __RPC__in IAppHostElementCollection * This,
            /* [in] */ __RPC__in_opt IAppHostElement *pElement,
            /* [defaultvalue][in] */ INT cPosition);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, DeleteElement)
        HRESULT ( STDMETHODCALLTYPE *DeleteElement )( 
            __RPC__in IAppHostElementCollection * This,
            /* [in] */ VARIANT cIndex);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IAppHostElementCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, CreateNewElement)
        HRESULT ( STDMETHODCALLTYPE *CreateNewElement )( 
            __RPC__in IAppHostElementCollection * This,
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR bstrElementName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement);
        
        DECLSPEC_XFGVIRT(IAppHostElementCollection, get_Schema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IAppHostElementCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostCollectionSchema **ppSchema);
        
        END_INTERFACE
    } IAppHostElementCollectionVtbl;

    interface IAppHostElementCollection
    {
        CONST_VTBL struct IAppHostElementCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostElementCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostElementCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostElementCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostElementCollection_get_Count(This,pcElementCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcElementCount) ) 

#define IAppHostElementCollection_get_Item(This,cIndex,ppElement)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppElement) ) 

#define IAppHostElementCollection_AddElement(This,pElement,cPosition)	\
    ( (This)->lpVtbl -> AddElement(This,pElement,cPosition) ) 

#define IAppHostElementCollection_DeleteElement(This,cIndex)	\
    ( (This)->lpVtbl -> DeleteElement(This,cIndex) ) 

#define IAppHostElementCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IAppHostElementCollection_CreateNewElement(This,bstrElementName,ppElement)	\
    ( (This)->lpVtbl -> CreateNewElement(This,bstrElementName,ppElement) ) 

#define IAppHostElementCollection_get_Schema(This,ppSchema)	\
    ( (This)->lpVtbl -> get_Schema(This,ppSchema) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostElementCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostElement_INTERFACE_DEFINED__
#define __IAppHostElement_INTERFACE_DEFINED__

/* interface IAppHostElement */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64ff8ccc-b287-4dae-b08a-a72cbf45f453")
    IAppHostElement : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Collection( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementCollection **ppCollection) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertyCollection **ppProperties) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ChildElements( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostChildElementCollection **ppElements) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Schema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetElementByName( 
            /* [string][in] */ __RPC__in_string BSTR bstrSubName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyByName( 
            /* [string][in] */ __RPC__in_string BSTR bstrSubName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostProperty **ppProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Methods( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodCollection **ppMethods) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostElement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostElement * This);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_Collection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Collection )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_Properties)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertyCollection **ppProperties);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_ChildElements)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChildElements )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostChildElementCollection **ppElements);
        
        DECLSPEC_XFGVIRT(IAppHostElement, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostElement * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostElement, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostElement * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_Schema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElementSchema **ppSchema);
        
        DECLSPEC_XFGVIRT(IAppHostElement, GetElementByName)
        HRESULT ( STDMETHODCALLTYPE *GetElementByName )( 
            __RPC__in IAppHostElement * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSubName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppElement);
        
        DECLSPEC_XFGVIRT(IAppHostElement, GetPropertyByName)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyByName )( 
            __RPC__in IAppHostElement * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSubName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostProperty **ppProperty);
        
        DECLSPEC_XFGVIRT(IAppHostElement, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IAppHostElement * This);
        
        DECLSPEC_XFGVIRT(IAppHostElement, get_Methods)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Methods )( 
            __RPC__in IAppHostElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostMethodCollection **ppMethods);
        
        END_INTERFACE
    } IAppHostElementVtbl;

    interface IAppHostElement
    {
        CONST_VTBL struct IAppHostElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostElement_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostElement_get_Collection(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Collection(This,ppCollection) ) 

#define IAppHostElement_get_Properties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppProperties) ) 

#define IAppHostElement_get_ChildElements(This,ppElements)	\
    ( (This)->lpVtbl -> get_ChildElements(This,ppElements) ) 

#define IAppHostElement_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostElement_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#define IAppHostElement_get_Schema(This,ppSchema)	\
    ( (This)->lpVtbl -> get_Schema(This,ppSchema) ) 

#define IAppHostElement_GetElementByName(This,bstrSubName,ppElement)	\
    ( (This)->lpVtbl -> GetElementByName(This,bstrSubName,ppElement) ) 

#define IAppHostElement_GetPropertyByName(This,bstrSubName,ppProperty)	\
    ( (This)->lpVtbl -> GetPropertyByName(This,bstrSubName,ppProperty) ) 

#define IAppHostElement_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IAppHostElement_get_Methods(This,ppMethods)	\
    ( (This)->lpVtbl -> get_Methods(This,ppMethods) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostElement_INTERFACE_DEFINED__ */


#ifndef __IAppHostProperty_INTERFACE_DEFINED__
#define __IAppHostProperty_INTERFACE_DEFINED__

/* interface IAppHostProperty */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ed35f7a1-5024-4e7b-a44d-07ddaf4b524d")
    IAppHostProperty : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StringValue( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Exception( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertyException **ppException) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Schema( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchema **ppSchema) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostProperty * This);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IAppHostProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, put_Value)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IAppHostProperty * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IAppHostProperty * This);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, get_StringValue)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StringValue )( 
            __RPC__in IAppHostProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, get_Exception)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Exception )( 
            __RPC__in IAppHostProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertyException **ppException);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostProperty * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostProperty * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostProperty, get_Schema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IAppHostProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostPropertySchema **ppSchema);
        
        END_INTERFACE
    } IAppHostPropertyVtbl;

    interface IAppHostProperty
    {
        CONST_VTBL struct IAppHostPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostProperty_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostProperty_get_Value(This,pVariant)	\
    ( (This)->lpVtbl -> get_Value(This,pVariant) ) 

#define IAppHostProperty_put_Value(This,value)	\
    ( (This)->lpVtbl -> put_Value(This,value) ) 

#define IAppHostProperty_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IAppHostProperty_get_StringValue(This,pbstrValue)	\
    ( (This)->lpVtbl -> get_StringValue(This,pbstrValue) ) 

#define IAppHostProperty_get_Exception(This,ppException)	\
    ( (This)->lpVtbl -> get_Exception(This,ppException) ) 

#define IAppHostProperty_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostProperty_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#define IAppHostProperty_get_Schema(This,ppSchema)	\
    ( (This)->lpVtbl -> get_Schema(This,ppSchema) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostProperty_INTERFACE_DEFINED__ */


#ifndef __IAppHostConfigLocation_INTERFACE_DEFINED__
#define __IAppHostConfigLocation_INTERFACE_DEFINED__

/* interface IAppHostConfigLocation */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConfigLocation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("370af178-7758-4dad-8146-7391f6e18585")
    IAppHostConfigLocation : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrLocationPath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out DWORD *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddConfigSection( 
            /* [in] */ __RPC__in BSTR bstrSectionName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteConfigSection( 
            /* [in] */ VARIANT cIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConfigLocationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConfigLocation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConfigLocation * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocation, get_Path)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrLocationPath);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocation, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [retval][out] */ __RPC__out DWORD *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocation, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [in] */ VARIANT cIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppSection);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocation, AddConfigSection)
        HRESULT ( STDMETHODCALLTYPE *AddConfigSection )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [in] */ __RPC__in BSTR bstrSectionName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminElement);
        
        DECLSPEC_XFGVIRT(IAppHostConfigLocation, DeleteConfigSection)
        HRESULT ( STDMETHODCALLTYPE *DeleteConfigSection )( 
            __RPC__in IAppHostConfigLocation * This,
            /* [in] */ VARIANT cIndex);
        
        END_INTERFACE
    } IAppHostConfigLocationVtbl;

    interface IAppHostConfigLocation
    {
        CONST_VTBL struct IAppHostConfigLocationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConfigLocation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConfigLocation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConfigLocation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConfigLocation_get_Path(This,pbstrLocationPath)	\
    ( (This)->lpVtbl -> get_Path(This,pbstrLocationPath) ) 

#define IAppHostConfigLocation_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostConfigLocation_get_Item(This,cIndex,ppSection)	\
    ( (This)->lpVtbl -> get_Item(This,cIndex,ppSection) ) 

#define IAppHostConfigLocation_AddConfigSection(This,bstrSectionName,ppAdminElement)	\
    ( (This)->lpVtbl -> AddConfigSection(This,bstrSectionName,ppAdminElement) ) 

#define IAppHostConfigLocation_DeleteConfigSection(This,cIndex)	\
    ( (This)->lpVtbl -> DeleteConfigSection(This,cIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConfigLocation_INTERFACE_DEFINED__ */


#ifndef __IAppHostSectionDefinition_INTERFACE_DEFINED__
#define __IAppHostSectionDefinition_INTERFACE_DEFINED__

/* interface IAppHostSectionDefinition */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostSectionDefinition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c5c04795-321c-4014-8fd6-d44658799393")
    IAppHostSectionDefinition : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OverrideModeDefault( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOverrideModeDefault) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_OverrideModeDefault( 
            /* [in] */ __RPC__in BSTR bstrOverrideModeDefault) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AllowDefinition( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAllowDefinition) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AllowDefinition( 
            /* [in] */ __RPC__in BSTR bstrAllowDefinition) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AllowLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAllowLocation) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AllowLocation( 
            /* [in] */ __RPC__in BSTR bstrAllowLocation) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequirePermission( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRequirePermission) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequirePermission( 
            /* [in] */ VARIANT_BOOL pfRequirePermission) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostSectionDefinitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostSectionDefinition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostSectionDefinition * This);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, put_Type)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_OverrideModeDefault)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OverrideModeDefault )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOverrideModeDefault);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, put_OverrideModeDefault)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_OverrideModeDefault )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ __RPC__in BSTR bstrOverrideModeDefault);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_AllowDefinition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowDefinition )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAllowDefinition);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, put_AllowDefinition)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowDefinition )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ __RPC__in BSTR bstrAllowDefinition);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_AllowLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowLocation )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAllowLocation);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, put_AllowLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowLocation )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ __RPC__in BSTR bstrAllowLocation);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, get_RequirePermission)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequirePermission )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRequirePermission);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinition, put_RequirePermission)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequirePermission )( 
            __RPC__in IAppHostSectionDefinition * This,
            /* [in] */ VARIANT_BOOL pfRequirePermission);
        
        END_INTERFACE
    } IAppHostSectionDefinitionVtbl;

    interface IAppHostSectionDefinition
    {
        CONST_VTBL struct IAppHostSectionDefinitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostSectionDefinition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostSectionDefinition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostSectionDefinition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostSectionDefinition_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostSectionDefinition_get_Type(This,pbstrType)	\
    ( (This)->lpVtbl -> get_Type(This,pbstrType) ) 

#define IAppHostSectionDefinition_put_Type(This,bstrType)	\
    ( (This)->lpVtbl -> put_Type(This,bstrType) ) 

#define IAppHostSectionDefinition_get_OverrideModeDefault(This,pbstrOverrideModeDefault)	\
    ( (This)->lpVtbl -> get_OverrideModeDefault(This,pbstrOverrideModeDefault) ) 

#define IAppHostSectionDefinition_put_OverrideModeDefault(This,bstrOverrideModeDefault)	\
    ( (This)->lpVtbl -> put_OverrideModeDefault(This,bstrOverrideModeDefault) ) 

#define IAppHostSectionDefinition_get_AllowDefinition(This,pbstrAllowDefinition)	\
    ( (This)->lpVtbl -> get_AllowDefinition(This,pbstrAllowDefinition) ) 

#define IAppHostSectionDefinition_put_AllowDefinition(This,bstrAllowDefinition)	\
    ( (This)->lpVtbl -> put_AllowDefinition(This,bstrAllowDefinition) ) 

#define IAppHostSectionDefinition_get_AllowLocation(This,pbstrAllowLocation)	\
    ( (This)->lpVtbl -> get_AllowLocation(This,pbstrAllowLocation) ) 

#define IAppHostSectionDefinition_put_AllowLocation(This,bstrAllowLocation)	\
    ( (This)->lpVtbl -> put_AllowLocation(This,bstrAllowLocation) ) 

#define IAppHostSectionDefinition_get_RequirePermission(This,pfRequirePermission)	\
    ( (This)->lpVtbl -> get_RequirePermission(This,pfRequirePermission) ) 

#define IAppHostSectionDefinition_put_RequirePermission(This,pfRequirePermission)	\
    ( (This)->lpVtbl -> put_RequirePermission(This,pfRequirePermission) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostSectionDefinition_INTERFACE_DEFINED__ */


#ifndef __IAppHostSectionDefinitionCollection_INTERFACE_DEFINED__
#define __IAppHostSectionDefinitionCollection_INTERFACE_DEFINED__

/* interface IAppHostSectionDefinitionCollection */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostSectionDefinitionCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b7d381ee-8860-47a1-8af4-1f33b2b1f325")
    IAppHostSectionDefinitionCollection : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *pcCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinition **ppConfigSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSection( 
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinition **ppConfigSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteSection( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostSectionDefinitionCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostSectionDefinitionCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostSectionDefinitionCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostSectionDefinitionCollection * This);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinitionCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostSectionDefinitionCollection * This,
            /* [retval][out] */ __RPC__out ULONG *pcCount);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinitionCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostSectionDefinitionCollection * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinition **ppConfigSection);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinitionCollection, AddSection)
        HRESULT ( STDMETHODCALLTYPE *AddSection )( 
            __RPC__in IAppHostSectionDefinitionCollection * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinition **ppConfigSection);
        
        DECLSPEC_XFGVIRT(IAppHostSectionDefinitionCollection, DeleteSection)
        HRESULT ( STDMETHODCALLTYPE *DeleteSection )( 
            __RPC__in IAppHostSectionDefinitionCollection * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } IAppHostSectionDefinitionCollectionVtbl;

    interface IAppHostSectionDefinitionCollection
    {
        CONST_VTBL struct IAppHostSectionDefinitionCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostSectionDefinitionCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostSectionDefinitionCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostSectionDefinitionCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostSectionDefinitionCollection_get_Count(This,pcCount)	\
    ( (This)->lpVtbl -> get_Count(This,pcCount) ) 

#define IAppHostSectionDefinitionCollection_get_Item(This,varIndex,ppConfigSection)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppConfigSection) ) 

#define IAppHostSectionDefinitionCollection_AddSection(This,bstrSectionName,ppConfigSection)	\
    ( (This)->lpVtbl -> AddSection(This,bstrSectionName,ppConfigSection) ) 

#define IAppHostSectionDefinitionCollection_DeleteSection(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteSection(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostSectionDefinitionCollection_INTERFACE_DEFINED__ */


#ifndef __IAppHostSectionGroup_INTERFACE_DEFINED__
#define __IAppHostSectionGroup_INTERFACE_DEFINED__

/* interface IAppHostSectionGroup */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostSectionGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0dd8a158-ebe6-4008-a1d9-b7ecc8f1104b")
    IAppHostSectionGroup : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *pcSectionGroup) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroup) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Sections( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinitionCollection **ppSections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSectionGroup( 
            /* [string][in] */ __RPC__in_string BSTR bstrSectionGroupName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteSectionGroup( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostSectionGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostSectionGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostSectionGroup * This);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [retval][out] */ __RPC__out ULONG *pcSectionGroup);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroup);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, get_Sections)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sections )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionDefinitionCollection **ppSections);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, AddSectionGroup)
        HRESULT ( STDMETHODCALLTYPE *AddSectionGroup )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionGroupName,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroup);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, DeleteSectionGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteSectionGroup )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IAppHostSectionGroup, put_Type)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IAppHostSectionGroup * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        END_INTERFACE
    } IAppHostSectionGroupVtbl;

    interface IAppHostSectionGroup
    {
        CONST_VTBL struct IAppHostSectionGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostSectionGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostSectionGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostSectionGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostSectionGroup_get_Count(This,pcSectionGroup)	\
    ( (This)->lpVtbl -> get_Count(This,pcSectionGroup) ) 

#define IAppHostSectionGroup_get_Item(This,varIndex,ppSectionGroup)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppSectionGroup) ) 

#define IAppHostSectionGroup_get_Sections(This,ppSections)	\
    ( (This)->lpVtbl -> get_Sections(This,ppSections) ) 

#define IAppHostSectionGroup_AddSectionGroup(This,bstrSectionGroupName,ppSectionGroup)	\
    ( (This)->lpVtbl -> AddSectionGroup(This,bstrSectionGroupName,ppSectionGroup) ) 

#define IAppHostSectionGroup_DeleteSectionGroup(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteSectionGroup(This,varIndex) ) 

#define IAppHostSectionGroup_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAppHostSectionGroup_get_Type(This,pbstrType)	\
    ( (This)->lpVtbl -> get_Type(This,pbstrType) ) 

#define IAppHostSectionGroup_put_Type(This,bstrType)	\
    ( (This)->lpVtbl -> put_Type(This,bstrType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostSectionGroup_INTERFACE_DEFINED__ */


#ifndef __IAppHostConfigFile_INTERFACE_DEFINED__
#define __IAppHostConfigFile_INTERFACE_DEFINED__

/* interface IAppHostConfigFile */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConfigFile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ada4e6fb-e025-401e-a5d0-c3134a281f07")
    IAppHostConfigFile : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ConfigPath( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrConfigPath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilePath( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrFilePath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Locations( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocationCollection **ppLocations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAdminSection( 
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearInvalidSections( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootSectionGroup( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroups) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConfigFileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConfigFile * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConfigFile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConfigFile * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, get_ConfigPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigPath )( 
            __RPC__in IAppHostConfigFile * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrConfigPath);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, get_FilePath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilePath )( 
            __RPC__in IAppHostConfigFile * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrFilePath);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, get_Locations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locations )( 
            __RPC__in IAppHostConfigFile * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigLocationCollection **ppLocations);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, GetAdminSection)
        HRESULT ( STDMETHODCALLTYPE *GetAdminSection )( 
            __RPC__in IAppHostConfigFile * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminSection);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostConfigFile * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostConfigFile * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, ClearInvalidSections)
        HRESULT ( STDMETHODCALLTYPE *ClearInvalidSections )( 
            __RPC__in IAppHostConfigFile * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigFile, get_RootSectionGroup)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootSectionGroup )( 
            __RPC__in IAppHostConfigFile * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostSectionGroup **ppSectionGroups);
        
        END_INTERFACE
    } IAppHostConfigFileVtbl;

    interface IAppHostConfigFile
    {
        CONST_VTBL struct IAppHostConfigFileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConfigFile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConfigFile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConfigFile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConfigFile_get_ConfigPath(This,pbstrConfigPath)	\
    ( (This)->lpVtbl -> get_ConfigPath(This,pbstrConfigPath) ) 

#define IAppHostConfigFile_get_FilePath(This,pbstrFilePath)	\
    ( (This)->lpVtbl -> get_FilePath(This,pbstrFilePath) ) 

#define IAppHostConfigFile_get_Locations(This,ppLocations)	\
    ( (This)->lpVtbl -> get_Locations(This,ppLocations) ) 

#define IAppHostConfigFile_GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection)	\
    ( (This)->lpVtbl -> GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection) ) 

#define IAppHostConfigFile_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostConfigFile_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#define IAppHostConfigFile_ClearInvalidSections(This)	\
    ( (This)->lpVtbl -> ClearInvalidSections(This) ) 

#define IAppHostConfigFile_get_RootSectionGroup(This,ppSectionGroups)	\
    ( (This)->lpVtbl -> get_RootSectionGroup(This,ppSectionGroups) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConfigFile_INTERFACE_DEFINED__ */


#ifndef __IAppHostPathMapper_INTERFACE_DEFINED__
#define __IAppHostPathMapper_INTERFACE_DEFINED__

/* interface IAppHostPathMapper */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPathMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7927575-5cc3-403b-822e-328a6b904bee")
    IAppHostPathMapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapPath( 
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [string][in] */ __RPC__in_string BSTR bstrMappedPhysicalPath,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrNewPhysicalPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPathMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostPathMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostPathMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostPathMapper * This);
        
        DECLSPEC_XFGVIRT(IAppHostPathMapper, MapPath)
        HRESULT ( STDMETHODCALLTYPE *MapPath )( 
            __RPC__in IAppHostPathMapper * This,
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [string][in] */ __RPC__in_string BSTR bstrMappedPhysicalPath,
            /* [retval][string][out] */ __RPC__deref_out_opt_string BSTR *pbstrNewPhysicalPath);
        
        END_INTERFACE
    } IAppHostPathMapperVtbl;

    interface IAppHostPathMapper
    {
        CONST_VTBL struct IAppHostPathMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPathMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPathMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPathMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPathMapper_MapPath(This,bstrConfigPath,bstrMappedPhysicalPath,pbstrNewPhysicalPath)	\
    ( (This)->lpVtbl -> MapPath(This,bstrConfigPath,bstrMappedPhysicalPath,pbstrNewPhysicalPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPathMapper_INTERFACE_DEFINED__ */


#ifndef __IAppHostPathMapper2_INTERFACE_DEFINED__
#define __IAppHostPathMapper2_INTERFACE_DEFINED__

/* interface IAppHostPathMapper2 */
/* [helpstring][local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostPathMapper2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0f80e901-8f4c-449a-bf90-13d5d082f187")
    IAppHostPathMapper2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapPath( 
            /* [string][in] */ BSTR bstrConfigPath,
            /* [string][in] */ BSTR bstrMappedPhysicalPath,
            /* [string][out] */ BSTR *pbstrNewPhysicalPath,
            /* [out] */ HANDLE *phImpersonationToken) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostPathMapper2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAppHostPathMapper2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAppHostPathMapper2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAppHostPathMapper2 * This);
        
        DECLSPEC_XFGVIRT(IAppHostPathMapper2, MapPath)
        HRESULT ( STDMETHODCALLTYPE *MapPath )( 
            IAppHostPathMapper2 * This,
            /* [string][in] */ BSTR bstrConfigPath,
            /* [string][in] */ BSTR bstrMappedPhysicalPath,
            /* [string][out] */ BSTR *pbstrNewPhysicalPath,
            /* [out] */ HANDLE *phImpersonationToken);
        
        END_INTERFACE
    } IAppHostPathMapper2Vtbl;

    interface IAppHostPathMapper2
    {
        CONST_VTBL struct IAppHostPathMapper2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostPathMapper2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostPathMapper2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostPathMapper2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostPathMapper2_MapPath(This,bstrConfigPath,bstrMappedPhysicalPath,pbstrNewPhysicalPath,phImpersonationToken)	\
    ( (This)->lpVtbl -> MapPath(This,bstrConfigPath,bstrMappedPhysicalPath,pbstrNewPhysicalPath,phImpersonationToken) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostPathMapper2_INTERFACE_DEFINED__ */


#ifndef __IAppHostChangeHandler_INTERFACE_DEFINED__
#define __IAppHostChangeHandler_INTERFACE_DEFINED__

/* interface IAppHostChangeHandler */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostChangeHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09829352-87c2-418d-8d79-4133969a489d")
    IAppHostChangeHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSectionChanges( 
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostChangeHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostChangeHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IAppHostChangeHandler, OnSectionChanges)
        HRESULT ( STDMETHODCALLTYPE *OnSectionChanges )( 
            __RPC__in IAppHostChangeHandler * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath);
        
        END_INTERFACE
    } IAppHostChangeHandlerVtbl;

    interface IAppHostChangeHandler
    {
        CONST_VTBL struct IAppHostChangeHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostChangeHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostChangeHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostChangeHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostChangeHandler_OnSectionChanges(This,bstrSectionName,bstrConfigPath)	\
    ( (This)->lpVtbl -> OnSectionChanges(This,bstrSectionName,bstrConfigPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostChangeHandler_INTERFACE_DEFINED__ */


#ifndef __IAppHostAdminManager_INTERFACE_DEFINED__
#define __IAppHostAdminManager_INTERFACE_DEFINED__

/* interface IAppHostAdminManager */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostAdminManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9be77978-73ed-4a9a-87fd-13f09fec1b13")
    IAppHostAdminManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAdminSection( 
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ConfigManager( 
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigManager **ppConfigManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostAdminManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostAdminManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostAdminManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostAdminManager * This);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, GetAdminSection)
        HRESULT ( STDMETHODCALLTYPE *GetAdminSection )( 
            __RPC__in IAppHostAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminSection);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, get_ConfigManager)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigManager )( 
            __RPC__in IAppHostAdminManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigManager **ppConfigManager);
        
        END_INTERFACE
    } IAppHostAdminManagerVtbl;

    interface IAppHostAdminManager
    {
        CONST_VTBL struct IAppHostAdminManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostAdminManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostAdminManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostAdminManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostAdminManager_GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection)	\
    ( (This)->lpVtbl -> GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection) ) 

#define IAppHostAdminManager_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostAdminManager_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#define IAppHostAdminManager_get_ConfigManager(This,ppConfigManager)	\
    ( (This)->lpVtbl -> get_ConfigManager(This,ppConfigManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostAdminManager_INTERFACE_DEFINED__ */


#ifndef __IAppHostWritableAdminManager_INTERFACE_DEFINED__
#define __IAppHostWritableAdminManager_INTERFACE_DEFINED__

/* interface IAppHostWritableAdminManager */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostWritableAdminManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa7660f6-7b3f-4237-a8bf-ed0ad0dcbbd9")
    IAppHostWritableAdminManager : public IAppHostAdminManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CommitChanges( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommitPath( 
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrCommitPath) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CommitPath( 
            /* [string][in] */ __RPC__in_string BSTR bstrCommitPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostWritableAdminManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostWritableAdminManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostWritableAdminManager * This);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, GetAdminSection)
        HRESULT ( STDMETHODCALLTYPE *GetAdminSection )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrSectionName,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostElement **ppAdminSection);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrMetadataType,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IAppHostAdminManager, get_ConfigManager)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigManager )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigManager **ppConfigManager);
        
        DECLSPEC_XFGVIRT(IAppHostWritableAdminManager, CommitChanges)
        HRESULT ( STDMETHODCALLTYPE *CommitChanges )( 
            __RPC__in IAppHostWritableAdminManager * This);
        
        DECLSPEC_XFGVIRT(IAppHostWritableAdminManager, get_CommitPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommitPath )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrCommitPath);
        
        DECLSPEC_XFGVIRT(IAppHostWritableAdminManager, put_CommitPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CommitPath )( 
            __RPC__in IAppHostWritableAdminManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrCommitPath);
        
        END_INTERFACE
    } IAppHostWritableAdminManagerVtbl;

    interface IAppHostWritableAdminManager
    {
        CONST_VTBL struct IAppHostWritableAdminManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostWritableAdminManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostWritableAdminManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostWritableAdminManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostWritableAdminManager_GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection)	\
    ( (This)->lpVtbl -> GetAdminSection(This,bstrSectionName,bstrPath,ppAdminSection) ) 

#define IAppHostWritableAdminManager_GetMetadata(This,bstrMetadataType,pValue)	\
    ( (This)->lpVtbl -> GetMetadata(This,bstrMetadataType,pValue) ) 

#define IAppHostWritableAdminManager_SetMetadata(This,bstrMetadataType,value)	\
    ( (This)->lpVtbl -> SetMetadata(This,bstrMetadataType,value) ) 

#define IAppHostWritableAdminManager_get_ConfigManager(This,ppConfigManager)	\
    ( (This)->lpVtbl -> get_ConfigManager(This,ppConfigManager) ) 


#define IAppHostWritableAdminManager_CommitChanges(This)	\
    ( (This)->lpVtbl -> CommitChanges(This) ) 

#define IAppHostWritableAdminManager_get_CommitPath(This,pbstrCommitPath)	\
    ( (This)->lpVtbl -> get_CommitPath(This,pbstrCommitPath) ) 

#define IAppHostWritableAdminManager_put_CommitPath(This,bstrCommitPath)	\
    ( (This)->lpVtbl -> put_CommitPath(This,bstrCommitPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostWritableAdminManager_INTERFACE_DEFINED__ */


#ifndef __IAppHostConfigManager_INTERFACE_DEFINED__
#define __IAppHostConfigManager_INTERFACE_DEFINED__

/* interface IAppHostConfigManager */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IAppHostConfigManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f6d760f-f0cb-4d69-b5f6-848b33e9bdc6")
    IAppHostConfigManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetConfigFile( 
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigFile **ppConfigFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUniqueConfigPath( 
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrUniquePath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppHostConfigManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppHostConfigManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppHostConfigManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppHostConfigManager * This);
        
        DECLSPEC_XFGVIRT(IAppHostConfigManager, GetConfigFile)
        HRESULT ( STDMETHODCALLTYPE *GetConfigFile )( 
            __RPC__in IAppHostConfigManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [retval][out] */ __RPC__deref_out_opt IAppHostConfigFile **ppConfigFile);
        
        DECLSPEC_XFGVIRT(IAppHostConfigManager, GetUniqueConfigPath)
        HRESULT ( STDMETHODCALLTYPE *GetUniqueConfigPath )( 
            __RPC__in IAppHostConfigManager * This,
            /* [string][in] */ __RPC__in_string BSTR bstrConfigPath,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *pbstrUniquePath);
        
        END_INTERFACE
    } IAppHostConfigManagerVtbl;

    interface IAppHostConfigManager
    {
        CONST_VTBL struct IAppHostConfigManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppHostConfigManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppHostConfigManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppHostConfigManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppHostConfigManager_GetConfigFile(This,bstrConfigPath,ppConfigFile)	\
    ( (This)->lpVtbl -> GetConfigFile(This,bstrConfigPath,ppConfigFile) ) 

#define IAppHostConfigManager_GetUniqueConfigPath(This,bstrConfigPath,pbstrUniquePath)	\
    ( (This)->lpVtbl -> GetUniqueConfigPath(This,bstrConfigPath,pbstrUniquePath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppHostConfigManager_INTERFACE_DEFINED__ */



#ifndef __AppHostAdminLibrary_LIBRARY_DEFINED__
#define __AppHostAdminLibrary_LIBRARY_DEFINED__

/* library AppHostAdminLibrary */
/* [helpstring][version][uuid] */ 











EXTERN_C const IID LIBID_AppHostAdminLibrary;

EXTERN_C const CLSID CLSID_AppHostAdminManager;

#ifdef __cplusplus

class DECLSPEC_UUID("228fb8f7-fb53-4fd5-8c7b-ff59de606c5b")
AppHostAdminManager;
#endif

EXTERN_C const CLSID CLSID_AppHostWritableAdminManager;

#ifdef __cplusplus

class DECLSPEC_UUID("2b72133b-3f5b-4602-8952-803546ce3344")
AppHostWritableAdminManager;
#endif
#endif /* __AppHostAdminLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_ahadmin_0000_0035 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ahadmin_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ahadmin_0000_0035_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


