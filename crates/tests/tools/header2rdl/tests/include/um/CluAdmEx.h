

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

#ifndef __cluadmex_h__
#define __cluadmex_h__

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

#ifndef __IGetClusterUIInfo_FWD_DEFINED__
#define __IGetClusterUIInfo_FWD_DEFINED__
typedef interface IGetClusterUIInfo IGetClusterUIInfo;

#endif 	/* __IGetClusterUIInfo_FWD_DEFINED__ */


#ifndef __IGetClusterDataInfo_FWD_DEFINED__
#define __IGetClusterDataInfo_FWD_DEFINED__
typedef interface IGetClusterDataInfo IGetClusterDataInfo;

#endif 	/* __IGetClusterDataInfo_FWD_DEFINED__ */


#ifndef __IGetClusterObjectInfo_FWD_DEFINED__
#define __IGetClusterObjectInfo_FWD_DEFINED__
typedef interface IGetClusterObjectInfo IGetClusterObjectInfo;

#endif 	/* __IGetClusterObjectInfo_FWD_DEFINED__ */


#ifndef __IGetClusterNodeInfo_FWD_DEFINED__
#define __IGetClusterNodeInfo_FWD_DEFINED__
typedef interface IGetClusterNodeInfo IGetClusterNodeInfo;

#endif 	/* __IGetClusterNodeInfo_FWD_DEFINED__ */


#ifndef __IGetClusterGroupInfo_FWD_DEFINED__
#define __IGetClusterGroupInfo_FWD_DEFINED__
typedef interface IGetClusterGroupInfo IGetClusterGroupInfo;

#endif 	/* __IGetClusterGroupInfo_FWD_DEFINED__ */


#ifndef __IGetClusterResourceInfo_FWD_DEFINED__
#define __IGetClusterResourceInfo_FWD_DEFINED__
typedef interface IGetClusterResourceInfo IGetClusterResourceInfo;

#endif 	/* __IGetClusterResourceInfo_FWD_DEFINED__ */


#ifndef __IGetClusterNetworkInfo_FWD_DEFINED__
#define __IGetClusterNetworkInfo_FWD_DEFINED__
typedef interface IGetClusterNetworkInfo IGetClusterNetworkInfo;

#endif 	/* __IGetClusterNetworkInfo_FWD_DEFINED__ */


#ifndef __IGetClusterNetInterfaceInfo_FWD_DEFINED__
#define __IGetClusterNetInterfaceInfo_FWD_DEFINED__
typedef interface IGetClusterNetInterfaceInfo IGetClusterNetInterfaceInfo;

#endif 	/* __IGetClusterNetInterfaceInfo_FWD_DEFINED__ */


#ifndef __IWCPropertySheetCallback_FWD_DEFINED__
#define __IWCPropertySheetCallback_FWD_DEFINED__
typedef interface IWCPropertySheetCallback IWCPropertySheetCallback;

#endif 	/* __IWCPropertySheetCallback_FWD_DEFINED__ */


#ifndef __IWEExtendPropertySheet_FWD_DEFINED__
#define __IWEExtendPropertySheet_FWD_DEFINED__
typedef interface IWEExtendPropertySheet IWEExtendPropertySheet;

#endif 	/* __IWEExtendPropertySheet_FWD_DEFINED__ */


#ifndef __IWCWizardCallback_FWD_DEFINED__
#define __IWCWizardCallback_FWD_DEFINED__
typedef interface IWCWizardCallback IWCWizardCallback;

#endif 	/* __IWCWizardCallback_FWD_DEFINED__ */


#ifndef __IWEExtendWizard_FWD_DEFINED__
#define __IWEExtendWizard_FWD_DEFINED__
typedef interface IWEExtendWizard IWEExtendWizard;

#endif 	/* __IWEExtendWizard_FWD_DEFINED__ */


#ifndef __IWCContextMenuCallback_FWD_DEFINED__
#define __IWCContextMenuCallback_FWD_DEFINED__
typedef interface IWCContextMenuCallback IWCContextMenuCallback;

#endif 	/* __IWCContextMenuCallback_FWD_DEFINED__ */


#ifndef __IWEExtendContextMenu_FWD_DEFINED__
#define __IWEExtendContextMenu_FWD_DEFINED__
typedef interface IWEExtendContextMenu IWEExtendContextMenu;

#endif 	/* __IWEExtendContextMenu_FWD_DEFINED__ */


#ifndef __IWEInvokeCommand_FWD_DEFINED__
#define __IWEInvokeCommand_FWD_DEFINED__
typedef interface IWEInvokeCommand IWEInvokeCommand;

#endif 	/* __IWEInvokeCommand_FWD_DEFINED__ */


#ifndef __IWCWizard97Callback_FWD_DEFINED__
#define __IWCWizard97Callback_FWD_DEFINED__
typedef interface IWCWizard97Callback IWCWizard97Callback;

#endif 	/* __IWCWizard97Callback_FWD_DEFINED__ */


#ifndef __IWEExtendWizard97_FWD_DEFINED__
#define __IWEExtendWizard97_FWD_DEFINED__
typedef interface IWEExtendWizard97 IWEExtendWizard97;

#endif 	/* __IWEExtendWizard97_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "clusapi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_cluadmex_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum _CLUADMEX_OBJECT_TYPE
    {
        CLUADMEX_OT_NONE	= 0,
        CLUADMEX_OT_CLUSTER	= ( CLUADMEX_OT_NONE + 1 ) ,
        CLUADMEX_OT_NODE	= ( CLUADMEX_OT_CLUSTER + 1 ) ,
        CLUADMEX_OT_GROUP	= ( CLUADMEX_OT_NODE + 1 ) ,
        CLUADMEX_OT_RESOURCE	= ( CLUADMEX_OT_GROUP + 1 ) ,
        CLUADMEX_OT_RESOURCETYPE	= ( CLUADMEX_OT_RESOURCE + 1 ) ,
        CLUADMEX_OT_NETWORK	= ( CLUADMEX_OT_RESOURCETYPE + 1 ) ,
        CLUADMEX_OT_NETINTERFACE	= ( CLUADMEX_OT_NETWORK + 1 ) 
    } 	CLUADMEX_OBJECT_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_cluadmex_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cluadmex_0000_0000_v0_0_s_ifspec;

#ifndef __IGetClusterUIInfo_INTERFACE_DEFINED__
#define __IGetClusterUIInfo_INTERFACE_DEFINED__

/* interface IGetClusterUIInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterUIInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE50-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterUIInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetClusterName( 
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName) = 0;
        
        virtual /* [local] */ LCID STDMETHODCALLTYPE GetLocale( void) = 0;
        
        virtual /* [local] */ HFONT STDMETHODCALLTYPE GetFont( void) = 0;
        
        virtual /* [local] */ HICON STDMETHODCALLTYPE GetIcon( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterUIInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterUIInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterUIInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterUIInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterUIInfo, GetClusterName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetClusterName )( 
            IGetClusterUIInfo * This,
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName);
        
        DECLSPEC_XFGVIRT(IGetClusterUIInfo, GetLocale)
        /* [local] */ LCID ( STDMETHODCALLTYPE *GetLocale )( 
            IGetClusterUIInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterUIInfo, GetFont)
        /* [local] */ HFONT ( STDMETHODCALLTYPE *GetFont )( 
            IGetClusterUIInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterUIInfo, GetIcon)
        /* [local] */ HICON ( STDMETHODCALLTYPE *GetIcon )( 
            IGetClusterUIInfo * This);
        
        END_INTERFACE
    } IGetClusterUIInfoVtbl;

    interface IGetClusterUIInfo
    {
        CONST_VTBL struct IGetClusterUIInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterUIInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterUIInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterUIInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterUIInfo_GetClusterName(This,lpszName,pcchName)	\
    ( (This)->lpVtbl -> GetClusterName(This,lpszName,pcchName) ) 

#define IGetClusterUIInfo_GetLocale(This)	\
    ( (This)->lpVtbl -> GetLocale(This) ) 

#define IGetClusterUIInfo_GetFont(This)	\
    ( (This)->lpVtbl -> GetFont(This) ) 

#define IGetClusterUIInfo_GetIcon(This)	\
    ( (This)->lpVtbl -> GetIcon(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterUIInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterDataInfo_INTERFACE_DEFINED__
#define __IGetClusterDataInfo_INTERFACE_DEFINED__

/* interface IGetClusterDataInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterDataInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE51-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterDataInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetClusterName( 
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName) = 0;
        
        virtual /* [local] */ HCLUSTER STDMETHODCALLTYPE GetClusterHandle( void) = 0;
        
        virtual /* [local] */ LONG STDMETHODCALLTYPE GetObjectCount( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterDataInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterDataInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterDataInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterDataInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterDataInfo, GetClusterName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetClusterName )( 
            IGetClusterDataInfo * This,
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName);
        
        DECLSPEC_XFGVIRT(IGetClusterDataInfo, GetClusterHandle)
        /* [local] */ HCLUSTER ( STDMETHODCALLTYPE *GetClusterHandle )( 
            IGetClusterDataInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterDataInfo, GetObjectCount)
        /* [local] */ LONG ( STDMETHODCALLTYPE *GetObjectCount )( 
            IGetClusterDataInfo * This);
        
        END_INTERFACE
    } IGetClusterDataInfoVtbl;

    interface IGetClusterDataInfo
    {
        CONST_VTBL struct IGetClusterDataInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterDataInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterDataInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterDataInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterDataInfo_GetClusterName(This,lpszName,pcchName)	\
    ( (This)->lpVtbl -> GetClusterName(This,lpszName,pcchName) ) 

#define IGetClusterDataInfo_GetClusterHandle(This)	\
    ( (This)->lpVtbl -> GetClusterHandle(This) ) 

#define IGetClusterDataInfo_GetObjectCount(This)	\
    ( (This)->lpVtbl -> GetObjectCount(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterDataInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterObjectInfo_INTERFACE_DEFINED__
#define __IGetClusterObjectInfo_INTERFACE_DEFINED__

/* interface IGetClusterObjectInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterObjectInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE52-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterObjectInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetObjectName( 
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName) = 0;
        
        virtual /* [local] */ CLUADMEX_OBJECT_TYPE STDMETHODCALLTYPE GetObjectType( 
            /* [in] */ LONG lObjIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterObjectInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterObjectInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterObjectInfo, GetObjectName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetObjectName )( 
            IGetClusterObjectInfo * This,
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszName,
            /* [out][in] */ LONG *pcchName);
        
        DECLSPEC_XFGVIRT(IGetClusterObjectInfo, GetObjectType)
        /* [local] */ CLUADMEX_OBJECT_TYPE ( STDMETHODCALLTYPE *GetObjectType )( 
            IGetClusterObjectInfo * This,
            /* [in] */ LONG lObjIndex);
        
        END_INTERFACE
    } IGetClusterObjectInfoVtbl;

    interface IGetClusterObjectInfo
    {
        CONST_VTBL struct IGetClusterObjectInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterObjectInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterObjectInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterObjectInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterObjectInfo_GetObjectName(This,lObjIndex,lpszName,pcchName)	\
    ( (This)->lpVtbl -> GetObjectName(This,lObjIndex,lpszName,pcchName) ) 

#define IGetClusterObjectInfo_GetObjectType(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetObjectType(This,lObjIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterObjectInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterNodeInfo_INTERFACE_DEFINED__
#define __IGetClusterNodeInfo_INTERFACE_DEFINED__

/* interface IGetClusterNodeInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterNodeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE53-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterNodeInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HNODE STDMETHODCALLTYPE GetNodeHandle( 
            /* [in] */ LONG lObjIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterNodeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterNodeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterNodeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterNodeInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterNodeInfo, GetNodeHandle)
        /* [local] */ HNODE ( STDMETHODCALLTYPE *GetNodeHandle )( 
            IGetClusterNodeInfo * This,
            /* [in] */ LONG lObjIndex);
        
        END_INTERFACE
    } IGetClusterNodeInfoVtbl;

    interface IGetClusterNodeInfo
    {
        CONST_VTBL struct IGetClusterNodeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterNodeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterNodeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterNodeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterNodeInfo_GetNodeHandle(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetNodeHandle(This,lObjIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterNodeInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterGroupInfo_INTERFACE_DEFINED__
#define __IGetClusterGroupInfo_INTERFACE_DEFINED__

/* interface IGetClusterGroupInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterGroupInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE54-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterGroupInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HGROUP STDMETHODCALLTYPE GetGroupHandle( 
            /* [in] */ LONG lObjIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterGroupInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterGroupInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterGroupInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterGroupInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterGroupInfo, GetGroupHandle)
        /* [local] */ HGROUP ( STDMETHODCALLTYPE *GetGroupHandle )( 
            IGetClusterGroupInfo * This,
            /* [in] */ LONG lObjIndex);
        
        END_INTERFACE
    } IGetClusterGroupInfoVtbl;

    interface IGetClusterGroupInfo
    {
        CONST_VTBL struct IGetClusterGroupInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterGroupInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterGroupInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterGroupInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterGroupInfo_GetGroupHandle(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetGroupHandle(This,lObjIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterGroupInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterResourceInfo_INTERFACE_DEFINED__
#define __IGetClusterResourceInfo_INTERFACE_DEFINED__

/* interface IGetClusterResourceInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterResourceInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE55-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterResourceInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESOURCE STDMETHODCALLTYPE GetResourceHandle( 
            /* [in] */ LONG lObjIndex) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetResourceTypeName( 
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszResTypeName,
            /* [out][in] */ LONG *pcchResTypeName) = 0;
        
        virtual /* [local] */ BOOL STDMETHODCALLTYPE GetResourceNetworkName( 
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszNetName,
            /* [out][in] */ ULONG *pcchNetName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterResourceInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterResourceInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterResourceInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterResourceInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterResourceInfo, GetResourceHandle)
        /* [local] */ HRESOURCE ( STDMETHODCALLTYPE *GetResourceHandle )( 
            IGetClusterResourceInfo * This,
            /* [in] */ LONG lObjIndex);
        
        DECLSPEC_XFGVIRT(IGetClusterResourceInfo, GetResourceTypeName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetResourceTypeName )( 
            IGetClusterResourceInfo * This,
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszResTypeName,
            /* [out][in] */ LONG *pcchResTypeName);
        
        DECLSPEC_XFGVIRT(IGetClusterResourceInfo, GetResourceNetworkName)
        /* [local] */ BOOL ( STDMETHODCALLTYPE *GetResourceNetworkName )( 
            IGetClusterResourceInfo * This,
            /* [in] */ LONG lObjIndex,
            /* [out] */ BSTR lpszNetName,
            /* [out][in] */ ULONG *pcchNetName);
        
        END_INTERFACE
    } IGetClusterResourceInfoVtbl;

    interface IGetClusterResourceInfo
    {
        CONST_VTBL struct IGetClusterResourceInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterResourceInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterResourceInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterResourceInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterResourceInfo_GetResourceHandle(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetResourceHandle(This,lObjIndex) ) 

#define IGetClusterResourceInfo_GetResourceTypeName(This,lObjIndex,lpszResTypeName,pcchResTypeName)	\
    ( (This)->lpVtbl -> GetResourceTypeName(This,lObjIndex,lpszResTypeName,pcchResTypeName) ) 

#define IGetClusterResourceInfo_GetResourceNetworkName(This,lObjIndex,lpszNetName,pcchNetName)	\
    ( (This)->lpVtbl -> GetResourceNetworkName(This,lObjIndex,lpszNetName,pcchNetName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterResourceInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterNetworkInfo_INTERFACE_DEFINED__
#define __IGetClusterNetworkInfo_INTERFACE_DEFINED__

/* interface IGetClusterNetworkInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterNetworkInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE56-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterNetworkInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HNETWORK STDMETHODCALLTYPE GetNetworkHandle( 
            /* [in] */ LONG lObjIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterNetworkInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterNetworkInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterNetworkInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterNetworkInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterNetworkInfo, GetNetworkHandle)
        /* [local] */ HNETWORK ( STDMETHODCALLTYPE *GetNetworkHandle )( 
            IGetClusterNetworkInfo * This,
            /* [in] */ LONG lObjIndex);
        
        END_INTERFACE
    } IGetClusterNetworkInfoVtbl;

    interface IGetClusterNetworkInfo
    {
        CONST_VTBL struct IGetClusterNetworkInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterNetworkInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterNetworkInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterNetworkInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterNetworkInfo_GetNetworkHandle(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetNetworkHandle(This,lObjIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterNetworkInfo_INTERFACE_DEFINED__ */


#ifndef __IGetClusterNetInterfaceInfo_INTERFACE_DEFINED__
#define __IGetClusterNetInterfaceInfo_INTERFACE_DEFINED__

/* interface IGetClusterNetInterfaceInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGetClusterNetInterfaceInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE57-FC6B-11CF-B5F5-00A0C90AB505")
    IGetClusterNetInterfaceInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HNETINTERFACE STDMETHODCALLTYPE GetNetInterfaceHandle( 
            /* [in] */ LONG lObjIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetClusterNetInterfaceInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetClusterNetInterfaceInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetClusterNetInterfaceInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetClusterNetInterfaceInfo * This);
        
        DECLSPEC_XFGVIRT(IGetClusterNetInterfaceInfo, GetNetInterfaceHandle)
        /* [local] */ HNETINTERFACE ( STDMETHODCALLTYPE *GetNetInterfaceHandle )( 
            IGetClusterNetInterfaceInfo * This,
            /* [in] */ LONG lObjIndex);
        
        END_INTERFACE
    } IGetClusterNetInterfaceInfoVtbl;

    interface IGetClusterNetInterfaceInfo
    {
        CONST_VTBL struct IGetClusterNetInterfaceInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetClusterNetInterfaceInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetClusterNetInterfaceInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetClusterNetInterfaceInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetClusterNetInterfaceInfo_GetNetInterfaceHandle(This,lObjIndex)	\
    ( (This)->lpVtbl -> GetNetInterfaceHandle(This,lObjIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetClusterNetInterfaceInfo_INTERFACE_DEFINED__ */


#ifndef __IWCPropertySheetCallback_INTERFACE_DEFINED__
#define __IWCPropertySheetCallback_INTERFACE_DEFINED__

/* interface IWCPropertySheetCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWCPropertySheetCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE60-FC6B-11CF-B5F5-00A0C90AB505")
    IWCPropertySheetCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddPropertySheetPage( 
            /* [in] */ __RPC__in LONG *hpage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCPropertySheetCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCPropertySheetCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCPropertySheetCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCPropertySheetCallback * This);
        
        DECLSPEC_XFGVIRT(IWCPropertySheetCallback, AddPropertySheetPage)
        HRESULT ( STDMETHODCALLTYPE *AddPropertySheetPage )( 
            __RPC__in IWCPropertySheetCallback * This,
            /* [in] */ __RPC__in LONG *hpage);
        
        END_INTERFACE
    } IWCPropertySheetCallbackVtbl;

    interface IWCPropertySheetCallback
    {
        CONST_VTBL struct IWCPropertySheetCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCPropertySheetCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCPropertySheetCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCPropertySheetCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCPropertySheetCallback_AddPropertySheetPage(This,hpage)	\
    ( (This)->lpVtbl -> AddPropertySheetPage(This,hpage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCPropertySheetCallback_INTERFACE_DEFINED__ */


#ifndef __IWEExtendPropertySheet_INTERFACE_DEFINED__
#define __IWEExtendPropertySheet_INTERFACE_DEFINED__

/* interface IWEExtendPropertySheet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWEExtendPropertySheet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE61-FC6B-11CF-B5F5-00A0C90AB505")
    IWEExtendPropertySheet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePropertySheetPages( 
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCPropertySheetCallback *piCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWEExtendPropertySheetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWEExtendPropertySheet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWEExtendPropertySheet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWEExtendPropertySheet * This);
        
        DECLSPEC_XFGVIRT(IWEExtendPropertySheet, CreatePropertySheetPages)
        HRESULT ( STDMETHODCALLTYPE *CreatePropertySheetPages )( 
            __RPC__in IWEExtendPropertySheet * This,
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCPropertySheetCallback *piCallback);
        
        END_INTERFACE
    } IWEExtendPropertySheetVtbl;

    interface IWEExtendPropertySheet
    {
        CONST_VTBL struct IWEExtendPropertySheetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWEExtendPropertySheet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWEExtendPropertySheet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWEExtendPropertySheet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWEExtendPropertySheet_CreatePropertySheetPages(This,piData,piCallback)	\
    ( (This)->lpVtbl -> CreatePropertySheetPages(This,piData,piCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWEExtendPropertySheet_INTERFACE_DEFINED__ */


#ifndef __IWCWizardCallback_INTERFACE_DEFINED__
#define __IWCWizardCallback_INTERFACE_DEFINED__

/* interface IWCWizardCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWCWizardCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE62-FC6B-11CF-B5F5-00A0C90AB505")
    IWCWizardCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddWizardPage( 
            /* [in] */ __RPC__in LONG *hpage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableNext( 
            /* [in] */ __RPC__in LONG *hpage,
            /* [in] */ BOOL bEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCWizardCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCWizardCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCWizardCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCWizardCallback * This);
        
        DECLSPEC_XFGVIRT(IWCWizardCallback, AddWizardPage)
        HRESULT ( STDMETHODCALLTYPE *AddWizardPage )( 
            __RPC__in IWCWizardCallback * This,
            /* [in] */ __RPC__in LONG *hpage);
        
        DECLSPEC_XFGVIRT(IWCWizardCallback, EnableNext)
        HRESULT ( STDMETHODCALLTYPE *EnableNext )( 
            __RPC__in IWCWizardCallback * This,
            /* [in] */ __RPC__in LONG *hpage,
            /* [in] */ BOOL bEnable);
        
        END_INTERFACE
    } IWCWizardCallbackVtbl;

    interface IWCWizardCallback
    {
        CONST_VTBL struct IWCWizardCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCWizardCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCWizardCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCWizardCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCWizardCallback_AddWizardPage(This,hpage)	\
    ( (This)->lpVtbl -> AddWizardPage(This,hpage) ) 

#define IWCWizardCallback_EnableNext(This,hpage,bEnable)	\
    ( (This)->lpVtbl -> EnableNext(This,hpage,bEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCWizardCallback_INTERFACE_DEFINED__ */


#ifndef __IWEExtendWizard_INTERFACE_DEFINED__
#define __IWEExtendWizard_INTERFACE_DEFINED__

/* interface IWEExtendWizard */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWEExtendWizard;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE63-FC6B-11CF-B5F5-00A0C90AB505")
    IWEExtendWizard : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateWizardPages( 
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCWizardCallback *piCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWEExtendWizardVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWEExtendWizard * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWEExtendWizard * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWEExtendWizard * This);
        
        DECLSPEC_XFGVIRT(IWEExtendWizard, CreateWizardPages)
        HRESULT ( STDMETHODCALLTYPE *CreateWizardPages )( 
            __RPC__in IWEExtendWizard * This,
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCWizardCallback *piCallback);
        
        END_INTERFACE
    } IWEExtendWizardVtbl;

    interface IWEExtendWizard
    {
        CONST_VTBL struct IWEExtendWizardVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWEExtendWizard_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWEExtendWizard_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWEExtendWizard_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWEExtendWizard_CreateWizardPages(This,piData,piCallback)	\
    ( (This)->lpVtbl -> CreateWizardPages(This,piData,piCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWEExtendWizard_INTERFACE_DEFINED__ */


#ifndef __IWCContextMenuCallback_INTERFACE_DEFINED__
#define __IWCContextMenuCallback_INTERFACE_DEFINED__

/* interface IWCContextMenuCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWCContextMenuCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE64-FC6B-11CF-B5F5-00A0C90AB505")
    IWCContextMenuCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddExtensionMenuItem( 
            /* [in] */ __RPC__in BSTR lpszName,
            /* [in] */ __RPC__in BSTR lpszStatusBarText,
            /* [in] */ ULONG nCommandID,
            /* [in] */ ULONG nSubmenuCommandID,
            /* [in] */ ULONG uFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCContextMenuCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCContextMenuCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCContextMenuCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCContextMenuCallback * This);
        
        DECLSPEC_XFGVIRT(IWCContextMenuCallback, AddExtensionMenuItem)
        HRESULT ( STDMETHODCALLTYPE *AddExtensionMenuItem )( 
            __RPC__in IWCContextMenuCallback * This,
            /* [in] */ __RPC__in BSTR lpszName,
            /* [in] */ __RPC__in BSTR lpszStatusBarText,
            /* [in] */ ULONG nCommandID,
            /* [in] */ ULONG nSubmenuCommandID,
            /* [in] */ ULONG uFlags);
        
        END_INTERFACE
    } IWCContextMenuCallbackVtbl;

    interface IWCContextMenuCallback
    {
        CONST_VTBL struct IWCContextMenuCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCContextMenuCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCContextMenuCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCContextMenuCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCContextMenuCallback_AddExtensionMenuItem(This,lpszName,lpszStatusBarText,nCommandID,nSubmenuCommandID,uFlags)	\
    ( (This)->lpVtbl -> AddExtensionMenuItem(This,lpszName,lpszStatusBarText,nCommandID,nSubmenuCommandID,uFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCContextMenuCallback_INTERFACE_DEFINED__ */


#ifndef __IWEExtendContextMenu_INTERFACE_DEFINED__
#define __IWEExtendContextMenu_INTERFACE_DEFINED__

/* interface IWEExtendContextMenu */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWEExtendContextMenu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE65-FC6B-11CF-B5F5-00A0C90AB505")
    IWEExtendContextMenu : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddContextMenuItems( 
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCContextMenuCallback *piCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWEExtendContextMenuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWEExtendContextMenu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWEExtendContextMenu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWEExtendContextMenu * This);
        
        DECLSPEC_XFGVIRT(IWEExtendContextMenu, AddContextMenuItems)
        HRESULT ( STDMETHODCALLTYPE *AddContextMenuItems )( 
            __RPC__in IWEExtendContextMenu * This,
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCContextMenuCallback *piCallback);
        
        END_INTERFACE
    } IWEExtendContextMenuVtbl;

    interface IWEExtendContextMenu
    {
        CONST_VTBL struct IWEExtendContextMenuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWEExtendContextMenu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWEExtendContextMenu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWEExtendContextMenu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWEExtendContextMenu_AddContextMenuItems(This,piData,piCallback)	\
    ( (This)->lpVtbl -> AddContextMenuItems(This,piData,piCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWEExtendContextMenu_INTERFACE_DEFINED__ */


#ifndef __IWEInvokeCommand_INTERFACE_DEFINED__
#define __IWEInvokeCommand_INTERFACE_DEFINED__

/* interface IWEInvokeCommand */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWEInvokeCommand;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE66-FC6B-11CF-B5F5-00A0C90AB505")
    IWEInvokeCommand : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InvokeCommand( 
            /* [in] */ ULONG nCommandID,
            /* [in] */ __RPC__in_opt IUnknown *piData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWEInvokeCommandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWEInvokeCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWEInvokeCommand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWEInvokeCommand * This);
        
        DECLSPEC_XFGVIRT(IWEInvokeCommand, InvokeCommand)
        HRESULT ( STDMETHODCALLTYPE *InvokeCommand )( 
            __RPC__in IWEInvokeCommand * This,
            /* [in] */ ULONG nCommandID,
            /* [in] */ __RPC__in_opt IUnknown *piData);
        
        END_INTERFACE
    } IWEInvokeCommandVtbl;

    interface IWEInvokeCommand
    {
        CONST_VTBL struct IWEInvokeCommandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWEInvokeCommand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWEInvokeCommand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWEInvokeCommand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWEInvokeCommand_InvokeCommand(This,nCommandID,piData)	\
    ( (This)->lpVtbl -> InvokeCommand(This,nCommandID,piData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWEInvokeCommand_INTERFACE_DEFINED__ */


#ifndef __IWCWizard97Callback_INTERFACE_DEFINED__
#define __IWCWizard97Callback_INTERFACE_DEFINED__

/* interface IWCWizard97Callback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWCWizard97Callback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE67-FC6B-11CF-B5F5-00A0C90AB505")
    IWCWizard97Callback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddWizard97Page( 
            /* [in] */ __RPC__in LONG *hpage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableNext( 
            /* [in] */ __RPC__in LONG *hpage,
            /* [in] */ BOOL bEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCWizard97CallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCWizard97Callback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCWizard97Callback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCWizard97Callback * This);
        
        DECLSPEC_XFGVIRT(IWCWizard97Callback, AddWizard97Page)
        HRESULT ( STDMETHODCALLTYPE *AddWizard97Page )( 
            __RPC__in IWCWizard97Callback * This,
            /* [in] */ __RPC__in LONG *hpage);
        
        DECLSPEC_XFGVIRT(IWCWizard97Callback, EnableNext)
        HRESULT ( STDMETHODCALLTYPE *EnableNext )( 
            __RPC__in IWCWizard97Callback * This,
            /* [in] */ __RPC__in LONG *hpage,
            /* [in] */ BOOL bEnable);
        
        END_INTERFACE
    } IWCWizard97CallbackVtbl;

    interface IWCWizard97Callback
    {
        CONST_VTBL struct IWCWizard97CallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCWizard97Callback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCWizard97Callback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCWizard97Callback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCWizard97Callback_AddWizard97Page(This,hpage)	\
    ( (This)->lpVtbl -> AddWizard97Page(This,hpage) ) 

#define IWCWizard97Callback_EnableNext(This,hpage,bEnable)	\
    ( (This)->lpVtbl -> EnableNext(This,hpage,bEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCWizard97Callback_INTERFACE_DEFINED__ */


#ifndef __IWEExtendWizard97_INTERFACE_DEFINED__
#define __IWEExtendWizard97_INTERFACE_DEFINED__

/* interface IWEExtendWizard97 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWEExtendWizard97;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97DEDE68-FC6B-11CF-B5F5-00A0C90AB505")
    IWEExtendWizard97 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateWizard97Pages( 
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCWizard97Callback *piCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWEExtendWizard97Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWEExtendWizard97 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWEExtendWizard97 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWEExtendWizard97 * This);
        
        DECLSPEC_XFGVIRT(IWEExtendWizard97, CreateWizard97Pages)
        HRESULT ( STDMETHODCALLTYPE *CreateWizard97Pages )( 
            __RPC__in IWEExtendWizard97 * This,
            /* [in] */ __RPC__in_opt IUnknown *piData,
            /* [in] */ __RPC__in_opt IWCWizard97Callback *piCallback);
        
        END_INTERFACE
    } IWEExtendWizard97Vtbl;

    interface IWEExtendWizard97
    {
        CONST_VTBL struct IWEExtendWizard97Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWEExtendWizard97_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWEExtendWizard97_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWEExtendWizard97_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWEExtendWizard97_CreateWizard97Pages(This,piData,piCallback)	\
    ( (This)->lpVtbl -> CreateWizard97Pages(This,piData,piCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWEExtendWizard97_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_cluadmex_0000_0017 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_cluadmex_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cluadmex_0000_0017_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


