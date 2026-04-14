

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

#ifndef __mmcobj_h__
#define __mmcobj_h__

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

#ifndef __ISnapinProperties_FWD_DEFINED__
#define __ISnapinProperties_FWD_DEFINED__
typedef interface ISnapinProperties ISnapinProperties;

#endif 	/* __ISnapinProperties_FWD_DEFINED__ */


#ifndef __ISnapinPropertiesCallback_FWD_DEFINED__
#define __ISnapinPropertiesCallback_FWD_DEFINED__
typedef interface ISnapinPropertiesCallback ISnapinPropertiesCallback;

#endif 	/* __ISnapinPropertiesCallback_FWD_DEFINED__ */


#ifndef ___Application_FWD_DEFINED__
#define ___Application_FWD_DEFINED__
typedef interface _Application _Application;

#endif 	/* ___Application_FWD_DEFINED__ */


#ifndef ___AppEvents_FWD_DEFINED__
#define ___AppEvents_FWD_DEFINED__
typedef interface _AppEvents _AppEvents;

#endif 	/* ___AppEvents_FWD_DEFINED__ */


#ifndef __AppEvents_FWD_DEFINED__
#define __AppEvents_FWD_DEFINED__
typedef interface AppEvents AppEvents;

#endif 	/* __AppEvents_FWD_DEFINED__ */


#ifndef __Application_FWD_DEFINED__
#define __Application_FWD_DEFINED__

#ifdef __cplusplus
typedef class Application Application;
#else
typedef struct Application Application;
#endif /* __cplusplus */

#endif 	/* __Application_FWD_DEFINED__ */


#ifndef ___EventConnector_FWD_DEFINED__
#define ___EventConnector_FWD_DEFINED__
typedef interface _EventConnector _EventConnector;

#endif 	/* ___EventConnector_FWD_DEFINED__ */


#ifndef __AppEventsDHTMLConnector_FWD_DEFINED__
#define __AppEventsDHTMLConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class AppEventsDHTMLConnector AppEventsDHTMLConnector;
#else
typedef struct AppEventsDHTMLConnector AppEventsDHTMLConnector;
#endif /* __cplusplus */

#endif 	/* __AppEventsDHTMLConnector_FWD_DEFINED__ */


#ifndef __Frame_FWD_DEFINED__
#define __Frame_FWD_DEFINED__
typedef interface Frame Frame;

#endif 	/* __Frame_FWD_DEFINED__ */


#ifndef __Node_FWD_DEFINED__
#define __Node_FWD_DEFINED__
typedef interface Node Node;

#endif 	/* __Node_FWD_DEFINED__ */


#ifndef __ScopeNamespace_FWD_DEFINED__
#define __ScopeNamespace_FWD_DEFINED__
typedef interface ScopeNamespace ScopeNamespace;

#endif 	/* __ScopeNamespace_FWD_DEFINED__ */


#ifndef __Document_FWD_DEFINED__
#define __Document_FWD_DEFINED__
typedef interface Document Document;

#endif 	/* __Document_FWD_DEFINED__ */


#ifndef __SnapIn_FWD_DEFINED__
#define __SnapIn_FWD_DEFINED__
typedef interface SnapIn SnapIn;

#endif 	/* __SnapIn_FWD_DEFINED__ */


#ifndef __SnapIns_FWD_DEFINED__
#define __SnapIns_FWD_DEFINED__
typedef interface SnapIns SnapIns;

#endif 	/* __SnapIns_FWD_DEFINED__ */


#ifndef __Extension_FWD_DEFINED__
#define __Extension_FWD_DEFINED__
typedef interface Extension Extension;

#endif 	/* __Extension_FWD_DEFINED__ */


#ifndef __Extensions_FWD_DEFINED__
#define __Extensions_FWD_DEFINED__
typedef interface Extensions Extensions;

#endif 	/* __Extensions_FWD_DEFINED__ */


#ifndef __Columns_FWD_DEFINED__
#define __Columns_FWD_DEFINED__
typedef interface Columns Columns;

#endif 	/* __Columns_FWD_DEFINED__ */


#ifndef __Column_FWD_DEFINED__
#define __Column_FWD_DEFINED__
typedef interface Column Column;

#endif 	/* __Column_FWD_DEFINED__ */


#ifndef __Views_FWD_DEFINED__
#define __Views_FWD_DEFINED__
typedef interface Views Views;

#endif 	/* __Views_FWD_DEFINED__ */


#ifndef __View_FWD_DEFINED__
#define __View_FWD_DEFINED__
typedef interface View View;

#endif 	/* __View_FWD_DEFINED__ */


#ifndef __Nodes_FWD_DEFINED__
#define __Nodes_FWD_DEFINED__
typedef interface Nodes Nodes;

#endif 	/* __Nodes_FWD_DEFINED__ */


#ifndef __ContextMenu_FWD_DEFINED__
#define __ContextMenu_FWD_DEFINED__
typedef interface ContextMenu ContextMenu;

#endif 	/* __ContextMenu_FWD_DEFINED__ */


#ifndef __MenuItem_FWD_DEFINED__
#define __MenuItem_FWD_DEFINED__
typedef interface MenuItem MenuItem;

#endif 	/* __MenuItem_FWD_DEFINED__ */


#ifndef __Properties_FWD_DEFINED__
#define __Properties_FWD_DEFINED__
typedef interface Properties Properties;

#endif 	/* __Properties_FWD_DEFINED__ */


#ifndef __Property_FWD_DEFINED__
#define __Property_FWD_DEFINED__
typedef interface Property Property;

#endif 	/* __Property_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mmcobj_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef MMC_VER
#define MMC_VER 0x0200
#endif
#if (MMC_VER >= 0x0200)




















typedef _Application *PAPPLICATION;

typedef _Application **PPAPPLICATION;

typedef Column *PCOLUMN;

typedef Column **PPCOLUMN;

typedef Columns *PCOLUMNS;

typedef Columns **PPCOLUMNS;

typedef ContextMenu *PCONTEXTMENU;

typedef ContextMenu **PPCONTEXTMENU;

typedef Document *PDOCUMENT;

typedef Document **PPDOCUMENT;

typedef Frame *PFRAME;

typedef Frame **PPFRAME;

typedef MenuItem *PMENUITEM;

typedef MenuItem **PPMENUITEM;

typedef Node *PNODE;

typedef Node **PPNODE;

typedef Nodes *PNODES;

typedef Nodes **PPNODES;

typedef Properties *PPROPERTIES;

typedef Properties **PPPROPERTIES;

typedef Property *PPROPERTY;

typedef Property **PPPROPERTY;

typedef ScopeNamespace *PSCOPENAMESPACE;

typedef ScopeNamespace **PPSCOPENAMESPACE;

typedef SnapIn *PSNAPIN;

typedef SnapIn **PPSNAPIN;

typedef SnapIns *PSNAPINS;

typedef SnapIns **PPSNAPINS;

typedef Extension *PEXTENSION;

typedef Extension **PPEXTENSION;

typedef Extensions *PEXTENSIONS;

typedef Extensions **PPEXTENSIONS;

typedef View *PVIEW;

typedef View **PPVIEW;

typedef Views *PVIEWS;

typedef Views **PPVIEWS;

typedef ISnapinProperties *LPSNAPINPROPERTIES;

typedef ISnapinPropertiesCallback *LPSNAPINPROPERTIESCALLBACK;

typedef BOOL *PBOOL;

typedef int *PINT;

typedef BSTR *PBSTR;

typedef VARIANT *PVARIANT;

typedef long *PLONG;

typedef IDispatch *PDISPATCH;

typedef IDispatch **PPDISPATCH;



extern RPC_IF_HANDLE __MIDL_itf_mmcobj_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmcobj_0000_0000_v0_0_s_ifspec;

#ifndef __ISnapinProperties_INTERFACE_DEFINED__
#define __ISnapinProperties_INTERFACE_DEFINED__

/* interface ISnapinProperties */
/* [unique][helpstring][uuid][object] */ 

typedef 
enum _MMC_PROPERTY_ACTION
    {
        MMC_PROPACT_DELETING	= 1,
        MMC_PROPACT_CHANGING	= ( MMC_PROPACT_DELETING + 1 ) ,
        MMC_PROPACT_INITIALIZED	= ( MMC_PROPACT_CHANGING + 1 ) 
    } 	MMC_PROPERTY_ACTION;

typedef struct _MMC_SNAPIN_PROPERTY
    {
    LPCOLESTR pszPropName;
    VARIANT varValue;
    MMC_PROPERTY_ACTION eAction;
    } 	MMC_SNAPIN_PROPERTY;


EXTERN_C const IID IID_ISnapinProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F7889DA9-4A02-4837-BF89-1A6F2A021010")
    ISnapinProperties : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt Properties *pProperties) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryPropertyNames( 
            /* [in] */ __RPC__in_opt ISnapinPropertiesCallback *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PropertiesChanged( 
            /* [in] */ long cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) MMC_SNAPIN_PROPERTY *pProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISnapinPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISnapinProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISnapinProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISnapinProperties * This);
        
        DECLSPEC_XFGVIRT(ISnapinProperties, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ISnapinProperties * This,
            /* [in] */ __RPC__in_opt Properties *pProperties);
        
        DECLSPEC_XFGVIRT(ISnapinProperties, QueryPropertyNames)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPropertyNames )( 
            __RPC__in ISnapinProperties * This,
            /* [in] */ __RPC__in_opt ISnapinPropertiesCallback *pCallback);
        
        DECLSPEC_XFGVIRT(ISnapinProperties, PropertiesChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PropertiesChanged )( 
            __RPC__in ISnapinProperties * This,
            /* [in] */ long cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) MMC_SNAPIN_PROPERTY *pProperties);
        
        END_INTERFACE
    } ISnapinPropertiesVtbl;

    interface ISnapinProperties
    {
        CONST_VTBL struct ISnapinPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISnapinProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISnapinProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISnapinProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISnapinProperties_Initialize(This,pProperties)	\
    ( (This)->lpVtbl -> Initialize(This,pProperties) ) 

#define ISnapinProperties_QueryPropertyNames(This,pCallback)	\
    ( (This)->lpVtbl -> QueryPropertyNames(This,pCallback) ) 

#define ISnapinProperties_PropertiesChanged(This,cProperties,pProperties)	\
    ( (This)->lpVtbl -> PropertiesChanged(This,cProperties,pProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISnapinProperties_INTERFACE_DEFINED__ */


#ifndef __ISnapinPropertiesCallback_INTERFACE_DEFINED__
#define __ISnapinPropertiesCallback_INTERFACE_DEFINED__

/* interface ISnapinPropertiesCallback */
/* [unique][helpstring][uuid][object] */ 

#define	MMC_PROP_CHANGEAFFECTSUI	( 0x1 )

#define	MMC_PROP_MODIFIABLE	( 0x2 )

#define	MMC_PROP_REMOVABLE	( 0x4 )

#define	MMC_PROP_PERSIST	( 0x8 )


EXTERN_C const IID IID_ISnapinPropertiesCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A50FA2E5-7E61-45EB-A8D4-9A07B3E851A8")
    ISnapinPropertiesCallback : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPropertyName( 
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISnapinPropertiesCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISnapinPropertiesCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISnapinPropertiesCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISnapinPropertiesCallback * This);
        
        DECLSPEC_XFGVIRT(ISnapinPropertiesCallback, AddPropertyName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPropertyName )( 
            __RPC__in ISnapinPropertiesCallback * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } ISnapinPropertiesCallbackVtbl;

    interface ISnapinPropertiesCallback
    {
        CONST_VTBL struct ISnapinPropertiesCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISnapinPropertiesCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISnapinPropertiesCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISnapinPropertiesCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISnapinPropertiesCallback_AddPropertyName(This,pszPropName,dwFlags)	\
    ( (This)->lpVtbl -> AddPropertyName(This,pszPropName,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISnapinPropertiesCallback_INTERFACE_DEFINED__ */



#ifndef __MMC20_LIBRARY_DEFINED__
#define __MMC20_LIBRARY_DEFINED__

/* library MMC20 */
/* [version][helpstring][uuid] */ 

typedef /* [helpstring][v1_enum] */ 
enum DocumentMode
    {
        DocumentMode_Author	= 0,
        DocumentMode_User	= ( DocumentMode_Author + 1 ) ,
        DocumentMode_User_MDI	= ( DocumentMode_User + 1 ) ,
        DocumentMode_User_SDI	= ( DocumentMode_User_MDI + 1 ) 
    } 	_DocumentMode;

typedef enum DocumentMode DOCUMENTMODE;

typedef enum DocumentMode *PDOCUMENTMODE;

typedef enum DocumentMode **PPDOCUMENTMODE;

typedef /* [helpstring][v1_enum] */ 
enum ListViewMode
    {
        ListMode_Small_Icons	= 0,
        ListMode_Large_Icons	= ( ListMode_Small_Icons + 1 ) ,
        ListMode_List	= ( ListMode_Large_Icons + 1 ) ,
        ListMode_Detail	= ( ListMode_List + 1 ) ,
        ListMode_Filtered	= ( ListMode_Detail + 1 ) 
    } 	_ListViewMode;

typedef enum ListViewMode LISTVIEWMODE;

typedef enum ListViewMode *PLISTVIEWMODE;

typedef enum ListViewMode **PPLISTVIEWMODE;

typedef /* [helpstring][v1_enum] */ 
enum ViewOptions
    {
        ViewOption_Default	= 0,
        ViewOption_ScopeTreeHidden	= 0x1,
        ViewOption_NoToolBars	= 0x2,
        ViewOption_NotPersistable	= 0x4,
        ViewOption_ActionPaneHidden	= 0x8
    } 	_ViewOptions;

typedef enum ViewOptions VIEWOPTIONS;

typedef enum ViewOptions *PVIEWOPTIONS;

typedef enum ViewOptions **PPVIEWOPTIONS;

typedef /* [helpstring][v1_enum] */ 
enum ExportListOptions
    {
        ExportListOptions_Default	= 0,
        ExportListOptions_Unicode	= 0x1,
        ExportListOptions_TabDelimited	= 0x2,
        ExportListOptions_SelectedItemsOnly	= 0x4
    } 	_ExportListOptions;

typedef enum ExportListOptions EXPORTLISTOPTIONS;


EXTERN_C const IID LIBID_MMC20;

#ifndef ___Application_INTERFACE_DEFINED__
#define ___Application_INTERFACE_DEFINED__

/* interface _Application */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID__Application;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3AFB9CC-B653-4741-86AB-F0470EC1384C")
    _Application : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE Help( void) = 0;
        
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE Quit( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Document( 
            /* [retval][out] */ __RPC__deref_out_opt PPDOCUMENT Document) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ __RPC__in BSTR Filename) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Frame( 
            /* [retval][out] */ __RPC__deref_out_opt PPFRAME Frame) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out PBOOL Visible) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Show( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Hide( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UserControl( 
            /* [retval][out] */ __RPC__out PBOOL UserControl) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UserControl( 
            /* [in] */ BOOL UserControl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_VersionMajor( 
            /* [retval][out] */ __RPC__out PLONG VersionMajor) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_VersionMinor( 
            /* [retval][out] */ __RPC__out PLONG VersionMinor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _ApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _Application * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _Application * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _Application * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _Application * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _Application * This,
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
        
        DECLSPEC_XFGVIRT(_Application, Help)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *Help )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(_Application, Quit)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *Quit )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(_Application, get_Document)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__deref_out_opt PPDOCUMENT Document);
        
        DECLSPEC_XFGVIRT(_Application, Load)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in _Application * This,
            /* [in] */ __RPC__in BSTR Filename);
        
        DECLSPEC_XFGVIRT(_Application, get_Frame)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Frame )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__deref_out_opt PPFRAME Frame);
        
        DECLSPEC_XFGVIRT(_Application, get_Visible)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__out PBOOL Visible);
        
        DECLSPEC_XFGVIRT(_Application, Show)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(_Application, Hide)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Hide )( 
            __RPC__in _Application * This);
        
        DECLSPEC_XFGVIRT(_Application, get_UserControl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UserControl )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__out PBOOL UserControl);
        
        DECLSPEC_XFGVIRT(_Application, put_UserControl)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UserControl )( 
            __RPC__in _Application * This,
            /* [in] */ BOOL UserControl);
        
        DECLSPEC_XFGVIRT(_Application, get_VersionMajor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_VersionMajor )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__out PLONG VersionMajor);
        
        DECLSPEC_XFGVIRT(_Application, get_VersionMinor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_VersionMinor )( 
            __RPC__in _Application * This,
            /* [retval][out] */ __RPC__out PLONG VersionMinor);
        
        END_INTERFACE
    } _ApplicationVtbl;

    interface _Application
    {
        CONST_VTBL struct _ApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _Application_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _Application_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _Application_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _Application_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _Application_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _Application_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _Application_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define _Application_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define _Application_Quit(This)	\
    ( (This)->lpVtbl -> Quit(This) ) 

#define _Application_get_Document(This,Document)	\
    ( (This)->lpVtbl -> get_Document(This,Document) ) 

#define _Application_Load(This,Filename)	\
    ( (This)->lpVtbl -> Load(This,Filename) ) 

#define _Application_get_Frame(This,Frame)	\
    ( (This)->lpVtbl -> get_Frame(This,Frame) ) 

#define _Application_get_Visible(This,Visible)	\
    ( (This)->lpVtbl -> get_Visible(This,Visible) ) 

#define _Application_Show(This)	\
    ( (This)->lpVtbl -> Show(This) ) 

#define _Application_Hide(This)	\
    ( (This)->lpVtbl -> Hide(This) ) 

#define _Application_get_UserControl(This,UserControl)	\
    ( (This)->lpVtbl -> get_UserControl(This,UserControl) ) 

#define _Application_put_UserControl(This,UserControl)	\
    ( (This)->lpVtbl -> put_UserControl(This,UserControl) ) 

#define _Application_get_VersionMajor(This,VersionMajor)	\
    ( (This)->lpVtbl -> get_VersionMajor(This,VersionMajor) ) 

#define _Application_get_VersionMinor(This,VersionMinor)	\
    ( (This)->lpVtbl -> get_VersionMinor(This,VersionMinor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___Application_INTERFACE_DEFINED__ */


#ifndef ___AppEvents_INTERFACE_DEFINED__
#define ___AppEvents_INTERFACE_DEFINED__

/* interface _AppEvents */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID__AppEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DE46CBDD-53F5-4635-AF54-4FE71E923D3F")
    _AppEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnQuit( 
            /* [in] */ __RPC__in_opt PAPPLICATION Application) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDocumentOpen( 
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ BOOL New) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDocumentClose( 
            /* [in] */ __RPC__in_opt PDOCUMENT Document) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSnapInAdded( 
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSnapInRemoved( 
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnNewView( 
            /* [in] */ __RPC__in_opt PVIEW View) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnViewClose( 
            /* [in] */ __RPC__in_opt PVIEW View) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnViewChange( 
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [in] */ __RPC__in_opt PNODE NewOwnerNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSelectionChange( 
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [in] */ __RPC__in_opt PNODES NewNodes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnContextMenuExecuted( 
            /* [in] */ __RPC__in_opt PMENUITEM MenuItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnToolbarButtonClicked( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnListUpdated( 
            /* [in] */ __RPC__in_opt PVIEW View) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _AppEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _AppEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _AppEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _AppEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _AppEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _AppEvents * This,
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
        
        DECLSPEC_XFGVIRT(_AppEvents, OnQuit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnQuit )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PAPPLICATION Application);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnDocumentOpen)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDocumentOpen )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ BOOL New);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnDocumentClose)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDocumentClose )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PDOCUMENT Document);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnSnapInAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSnapInAdded )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnSnapInRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSnapInRemoved )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PDOCUMENT Document,
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnNewView)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnNewView )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PVIEW View);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnViewClose)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnViewClose )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PVIEW View);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnViewChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnViewChange )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [in] */ __RPC__in_opt PNODE NewOwnerNode);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnSelectionChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSelectionChange )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [in] */ __RPC__in_opt PNODES NewNodes);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnContextMenuExecuted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnContextMenuExecuted )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PMENUITEM MenuItem);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnToolbarButtonClicked)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnToolbarButtonClicked )( 
            __RPC__in _AppEvents * This);
        
        DECLSPEC_XFGVIRT(_AppEvents, OnListUpdated)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnListUpdated )( 
            __RPC__in _AppEvents * This,
            /* [in] */ __RPC__in_opt PVIEW View);
        
        END_INTERFACE
    } _AppEventsVtbl;

    interface _AppEvents
    {
        CONST_VTBL struct _AppEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _AppEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _AppEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _AppEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _AppEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _AppEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _AppEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _AppEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define _AppEvents_OnQuit(This,Application)	\
    ( (This)->lpVtbl -> OnQuit(This,Application) ) 

#define _AppEvents_OnDocumentOpen(This,Document,New)	\
    ( (This)->lpVtbl -> OnDocumentOpen(This,Document,New) ) 

#define _AppEvents_OnDocumentClose(This,Document)	\
    ( (This)->lpVtbl -> OnDocumentClose(This,Document) ) 

#define _AppEvents_OnSnapInAdded(This,Document,SnapIn)	\
    ( (This)->lpVtbl -> OnSnapInAdded(This,Document,SnapIn) ) 

#define _AppEvents_OnSnapInRemoved(This,Document,SnapIn)	\
    ( (This)->lpVtbl -> OnSnapInRemoved(This,Document,SnapIn) ) 

#define _AppEvents_OnNewView(This,View)	\
    ( (This)->lpVtbl -> OnNewView(This,View) ) 

#define _AppEvents_OnViewClose(This,View)	\
    ( (This)->lpVtbl -> OnViewClose(This,View) ) 

#define _AppEvents_OnViewChange(This,View,NewOwnerNode)	\
    ( (This)->lpVtbl -> OnViewChange(This,View,NewOwnerNode) ) 

#define _AppEvents_OnSelectionChange(This,View,NewNodes)	\
    ( (This)->lpVtbl -> OnSelectionChange(This,View,NewNodes) ) 

#define _AppEvents_OnContextMenuExecuted(This,MenuItem)	\
    ( (This)->lpVtbl -> OnContextMenuExecuted(This,MenuItem) ) 

#define _AppEvents_OnToolbarButtonClicked(This)	\
    ( (This)->lpVtbl -> OnToolbarButtonClicked(This) ) 

#define _AppEvents_OnListUpdated(This,View)	\
    ( (This)->lpVtbl -> OnListUpdated(This,View) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___AppEvents_INTERFACE_DEFINED__ */


#ifndef __AppEvents_DISPINTERFACE_DEFINED__
#define __AppEvents_DISPINTERFACE_DEFINED__

/* dispinterface AppEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_AppEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("FC7A4252-78AC-4532-8C5A-563CFE138863")
    AppEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct AppEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AppEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AppEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AppEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in AppEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in AppEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in AppEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            AppEvents * This,
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
        
        END_INTERFACE
    } AppEventsVtbl;

    interface AppEvents
    {
        CONST_VTBL struct AppEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AppEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AppEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AppEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AppEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define AppEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define AppEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define AppEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __AppEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Application;

#ifdef __cplusplus

class DECLSPEC_UUID("49B2791A-B1AE-4C90-9B8E-E860BA07F889")
Application;
#endif

#ifndef ___EventConnector_INTERFACE_DEFINED__
#define ___EventConnector_INTERFACE_DEFINED__

/* interface _EventConnector */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID__EventConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0BCCD30-DE44-4528-8403-A05A6A1CC8EA")
    _EventConnector : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ConnectTo( 
            /* [in] */ __RPC__in_opt PAPPLICATION Application) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _EventConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _EventConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _EventConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _EventConnector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _EventConnector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _EventConnector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _EventConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _EventConnector * This,
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
        
        DECLSPEC_XFGVIRT(_EventConnector, ConnectTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectTo )( 
            __RPC__in _EventConnector * This,
            /* [in] */ __RPC__in_opt PAPPLICATION Application);
        
        DECLSPEC_XFGVIRT(_EventConnector, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in _EventConnector * This);
        
        END_INTERFACE
    } _EventConnectorVtbl;

    interface _EventConnector
    {
        CONST_VTBL struct _EventConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _EventConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _EventConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _EventConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _EventConnector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _EventConnector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _EventConnector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _EventConnector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define _EventConnector_ConnectTo(This,Application)	\
    ( (This)->lpVtbl -> ConnectTo(This,Application) ) 

#define _EventConnector_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___EventConnector_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_AppEventsDHTMLConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("ADE6444B-C91F-4e37-92A4-5BB430A33340")
AppEventsDHTMLConnector;
#endif

#ifndef __Frame_INTERFACE_DEFINED__
#define __Frame_INTERFACE_DEFINED__

/* interface Frame */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Frame;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E5E2D970-5BB3-4306-8804-B0968A31C8E6")
    Frame : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Maximize( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Minimize( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Restore( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Top( 
            /* [retval][out] */ __RPC__out PINT Top) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Top( 
            /* [in] */ int top) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Bottom( 
            /* [retval][out] */ __RPC__out PINT Bottom) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Bottom( 
            /* [in] */ int bottom) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Left( 
            /* [retval][out] */ __RPC__out PINT Left) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Left( 
            /* [in] */ int left) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Right( 
            /* [retval][out] */ __RPC__out PINT Right) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Right( 
            /* [in] */ int right) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FrameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Frame * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Frame * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Frame * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Frame * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Frame * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Frame * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Frame * This,
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
        
        DECLSPEC_XFGVIRT(Frame, Maximize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Maximize )( 
            __RPC__in Frame * This);
        
        DECLSPEC_XFGVIRT(Frame, Minimize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Minimize )( 
            __RPC__in Frame * This);
        
        DECLSPEC_XFGVIRT(Frame, Restore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in Frame * This);
        
        DECLSPEC_XFGVIRT(Frame, get_Top)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in Frame * This,
            /* [retval][out] */ __RPC__out PINT Top);
        
        DECLSPEC_XFGVIRT(Frame, put_Top)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in Frame * This,
            /* [in] */ int top);
        
        DECLSPEC_XFGVIRT(Frame, get_Bottom)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bottom )( 
            __RPC__in Frame * This,
            /* [retval][out] */ __RPC__out PINT Bottom);
        
        DECLSPEC_XFGVIRT(Frame, put_Bottom)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bottom )( 
            __RPC__in Frame * This,
            /* [in] */ int bottom);
        
        DECLSPEC_XFGVIRT(Frame, get_Left)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in Frame * This,
            /* [retval][out] */ __RPC__out PINT Left);
        
        DECLSPEC_XFGVIRT(Frame, put_Left)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in Frame * This,
            /* [in] */ int left);
        
        DECLSPEC_XFGVIRT(Frame, get_Right)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Right )( 
            __RPC__in Frame * This,
            /* [retval][out] */ __RPC__out PINT Right);
        
        DECLSPEC_XFGVIRT(Frame, put_Right)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Right )( 
            __RPC__in Frame * This,
            /* [in] */ int right);
        
        END_INTERFACE
    } FrameVtbl;

    interface Frame
    {
        CONST_VTBL struct FrameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Frame_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Frame_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Frame_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Frame_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Frame_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Frame_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Frame_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Frame_Maximize(This)	\
    ( (This)->lpVtbl -> Maximize(This) ) 

#define Frame_Minimize(This)	\
    ( (This)->lpVtbl -> Minimize(This) ) 

#define Frame_Restore(This)	\
    ( (This)->lpVtbl -> Restore(This) ) 

#define Frame_get_Top(This,Top)	\
    ( (This)->lpVtbl -> get_Top(This,Top) ) 

#define Frame_put_Top(This,top)	\
    ( (This)->lpVtbl -> put_Top(This,top) ) 

#define Frame_get_Bottom(This,Bottom)	\
    ( (This)->lpVtbl -> get_Bottom(This,Bottom) ) 

#define Frame_put_Bottom(This,bottom)	\
    ( (This)->lpVtbl -> put_Bottom(This,bottom) ) 

#define Frame_get_Left(This,Left)	\
    ( (This)->lpVtbl -> get_Left(This,Left) ) 

#define Frame_put_Left(This,left)	\
    ( (This)->lpVtbl -> put_Left(This,left) ) 

#define Frame_get_Right(This,Right)	\
    ( (This)->lpVtbl -> get_Right(This,Right) ) 

#define Frame_put_Right(This,right)	\
    ( (This)->lpVtbl -> put_Right(This,right) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Frame_INTERFACE_DEFINED__ */


#ifndef __Node_INTERFACE_DEFINED__
#define __Node_INTERFACE_DEFINED__

/* interface Node */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Node;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F81ED800-7839-4447-945D-8E15DA59CA55")
    Node : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Property( 
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR PropertyValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Bookmark( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Bookmark) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsScopeNode( 
            /* [retval][out] */ __RPC__out PBOOL IsScopeNode) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Nodetype( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Nodetype) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct NodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Node * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Node * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Node * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Node * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Node * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Node * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Node * This,
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
        
        DECLSPEC_XFGVIRT(Node, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Node * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name);
        
        DECLSPEC_XFGVIRT(Node, get_Property)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Property )( 
            __RPC__in Node * This,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR PropertyValue);
        
        DECLSPEC_XFGVIRT(Node, get_Bookmark)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bookmark )( 
            __RPC__in Node * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Bookmark);
        
        DECLSPEC_XFGVIRT(Node, IsScopeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsScopeNode )( 
            __RPC__in Node * This,
            /* [retval][out] */ __RPC__out PBOOL IsScopeNode);
        
        DECLSPEC_XFGVIRT(Node, get_Nodetype)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Nodetype )( 
            __RPC__in Node * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Nodetype);
        
        END_INTERFACE
    } NodeVtbl;

    interface Node
    {
        CONST_VTBL struct NodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Node_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Node_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Node_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Node_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Node_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Node_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Node_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Node_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define Node_get_Property(This,PropertyName,PropertyValue)	\
    ( (This)->lpVtbl -> get_Property(This,PropertyName,PropertyValue) ) 

#define Node_get_Bookmark(This,Bookmark)	\
    ( (This)->lpVtbl -> get_Bookmark(This,Bookmark) ) 

#define Node_IsScopeNode(This,IsScopeNode)	\
    ( (This)->lpVtbl -> IsScopeNode(This,IsScopeNode) ) 

#define Node_get_Nodetype(This,Nodetype)	\
    ( (This)->lpVtbl -> get_Nodetype(This,Nodetype) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Node_INTERFACE_DEFINED__ */


#ifndef __ScopeNamespace_INTERFACE_DEFINED__
#define __ScopeNamespace_INTERFACE_DEFINED__

/* interface ScopeNamespace */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_ScopeNamespace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EBBB48DC-1A3B-4D86-B786-C21B28389012")
    ScopeNamespace : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetParent( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Parent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetChild( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Child) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetNext( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Next) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRoot( 
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Root) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Expand( 
            /* [in] */ __RPC__in_opt PNODE Node) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ScopeNamespaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ScopeNamespace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ScopeNamespace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ScopeNamespace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ScopeNamespace * This,
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
        
        DECLSPEC_XFGVIRT(ScopeNamespace, GetParent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Parent);
        
        DECLSPEC_XFGVIRT(ScopeNamespace, GetChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetChild )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Child);
        
        DECLSPEC_XFGVIRT(ScopeNamespace, GetNext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetNext )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Next);
        
        DECLSPEC_XFGVIRT(ScopeNamespace, GetRoot)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRoot )( 
            __RPC__in ScopeNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Root);
        
        DECLSPEC_XFGVIRT(ScopeNamespace, Expand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in ScopeNamespace * This,
            /* [in] */ __RPC__in_opt PNODE Node);
        
        END_INTERFACE
    } ScopeNamespaceVtbl;

    interface ScopeNamespace
    {
        CONST_VTBL struct ScopeNamespaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ScopeNamespace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ScopeNamespace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ScopeNamespace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ScopeNamespace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ScopeNamespace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ScopeNamespace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ScopeNamespace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ScopeNamespace_GetParent(This,Node,Parent)	\
    ( (This)->lpVtbl -> GetParent(This,Node,Parent) ) 

#define ScopeNamespace_GetChild(This,Node,Child)	\
    ( (This)->lpVtbl -> GetChild(This,Node,Child) ) 

#define ScopeNamespace_GetNext(This,Node,Next)	\
    ( (This)->lpVtbl -> GetNext(This,Node,Next) ) 

#define ScopeNamespace_GetRoot(This,Root)	\
    ( (This)->lpVtbl -> GetRoot(This,Root) ) 

#define ScopeNamespace_Expand(This,Node)	\
    ( (This)->lpVtbl -> Expand(This,Node) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ScopeNamespace_INTERFACE_DEFINED__ */


#ifndef __Document_INTERFACE_DEFINED__
#define __Document_INTERFACE_DEFINED__

/* interface Document */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Document;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("225120D6-1E0F-40A3-93FE-1079E6A8017B")
    Document : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveAs( 
            /* [in] */ __RPC__in BSTR Filename) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Close( 
            /* [in] */ BOOL SaveChanges) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Views( 
            /* [retval][out] */ __RPC__deref_out_opt PPVIEWS Views) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SnapIns( 
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPINS SnapIns) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveView( 
            /* [retval][out] */ __RPC__deref_out_opt PPVIEW View) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR Name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Location( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Location) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsSaved( 
            /* [retval][out] */ __RPC__out PBOOL IsSaved) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out PDOCUMENTMODE Mode) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ DOCUMENTMODE Mode) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RootNode( 
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ScopeNamespace( 
            /* [retval][out] */ __RPC__deref_out_opt PPSCOPENAMESPACE ScopeNamespace) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTIES Properties) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt PPAPPLICATION Application) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Document * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Document * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Document * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Document * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Document * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Document * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Document * This,
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
        
        DECLSPEC_XFGVIRT(Document, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in Document * This);
        
        DECLSPEC_XFGVIRT(Document, SaveAs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveAs )( 
            __RPC__in Document * This,
            /* [in] */ __RPC__in BSTR Filename);
        
        DECLSPEC_XFGVIRT(Document, Close)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in Document * This,
            /* [in] */ BOOL SaveChanges);
        
        DECLSPEC_XFGVIRT(Document, get_Views)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Views )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPVIEWS Views);
        
        DECLSPEC_XFGVIRT(Document, get_SnapIns)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SnapIns )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPINS SnapIns);
        
        DECLSPEC_XFGVIRT(Document, get_ActiveView)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveView )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPVIEW View);
        
        DECLSPEC_XFGVIRT(Document, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name);
        
        DECLSPEC_XFGVIRT(Document, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in Document * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(Document, get_Location)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Location )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Location);
        
        DECLSPEC_XFGVIRT(Document, get_IsSaved)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsSaved )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__out PBOOL IsSaved);
        
        DECLSPEC_XFGVIRT(Document, get_Mode)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__out PDOCUMENTMODE Mode);
        
        DECLSPEC_XFGVIRT(Document, put_Mode)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in Document * This,
            /* [in] */ DOCUMENTMODE Mode);
        
        DECLSPEC_XFGVIRT(Document, get_RootNode)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RootNode )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node);
        
        DECLSPEC_XFGVIRT(Document, get_ScopeNamespace)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScopeNamespace )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPSCOPENAMESPACE ScopeNamespace);
        
        DECLSPEC_XFGVIRT(Document, CreateProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateProperties )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTIES Properties);
        
        DECLSPEC_XFGVIRT(Document, get_Application)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in Document * This,
            /* [retval][out] */ __RPC__deref_out_opt PPAPPLICATION Application);
        
        END_INTERFACE
    } DocumentVtbl;

    interface Document
    {
        CONST_VTBL struct DocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Document_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Document_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Document_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Document_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Document_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Document_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Document_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Document_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define Document_SaveAs(This,Filename)	\
    ( (This)->lpVtbl -> SaveAs(This,Filename) ) 

#define Document_Close(This,SaveChanges)	\
    ( (This)->lpVtbl -> Close(This,SaveChanges) ) 

#define Document_get_Views(This,Views)	\
    ( (This)->lpVtbl -> get_Views(This,Views) ) 

#define Document_get_SnapIns(This,SnapIns)	\
    ( (This)->lpVtbl -> get_SnapIns(This,SnapIns) ) 

#define Document_get_ActiveView(This,View)	\
    ( (This)->lpVtbl -> get_ActiveView(This,View) ) 

#define Document_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define Document_put_Name(This,Name)	\
    ( (This)->lpVtbl -> put_Name(This,Name) ) 

#define Document_get_Location(This,Location)	\
    ( (This)->lpVtbl -> get_Location(This,Location) ) 

#define Document_get_IsSaved(This,IsSaved)	\
    ( (This)->lpVtbl -> get_IsSaved(This,IsSaved) ) 

#define Document_get_Mode(This,Mode)	\
    ( (This)->lpVtbl -> get_Mode(This,Mode) ) 

#define Document_put_Mode(This,Mode)	\
    ( (This)->lpVtbl -> put_Mode(This,Mode) ) 

#define Document_get_RootNode(This,Node)	\
    ( (This)->lpVtbl -> get_RootNode(This,Node) ) 

#define Document_get_ScopeNamespace(This,ScopeNamespace)	\
    ( (This)->lpVtbl -> get_ScopeNamespace(This,ScopeNamespace) ) 

#define Document_CreateProperties(This,Properties)	\
    ( (This)->lpVtbl -> CreateProperties(This,Properties) ) 

#define Document_get_Application(This,Application)	\
    ( (This)->lpVtbl -> get_Application(This,Application) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Document_INTERFACE_DEFINED__ */


#ifndef __SnapIn_INTERFACE_DEFINED__
#define __SnapIn_INTERFACE_DEFINED__

/* interface SnapIn */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_SnapIn;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3BE910F6-3459-49C6-A1BB-41E6BE9DF3EA")
    SnapIn : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Vendor( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Vendor) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Version) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Extensions( 
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSIONS Extensions) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SnapinCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR SnapinCLSID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTIES Properties) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableAllExtensions( 
            /* [in] */ BOOL Enable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct SnapInVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in SnapIn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in SnapIn * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in SnapIn * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in SnapIn * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in SnapIn * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in SnapIn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            SnapIn * This,
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
        
        DECLSPEC_XFGVIRT(SnapIn, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name);
        
        DECLSPEC_XFGVIRT(SnapIn, get_Vendor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Vendor )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Vendor);
        
        DECLSPEC_XFGVIRT(SnapIn, get_Version)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Version);
        
        DECLSPEC_XFGVIRT(SnapIn, get_Extensions)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Extensions )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSIONS Extensions);
        
        DECLSPEC_XFGVIRT(SnapIn, get_SnapinCLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SnapinCLSID )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR SnapinCLSID);
        
        DECLSPEC_XFGVIRT(SnapIn, get_Properties)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in SnapIn * This,
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTIES Properties);
        
        DECLSPEC_XFGVIRT(SnapIn, EnableAllExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableAllExtensions )( 
            __RPC__in SnapIn * This,
            /* [in] */ BOOL Enable);
        
        END_INTERFACE
    } SnapInVtbl;

    interface SnapIn
    {
        CONST_VTBL struct SnapInVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define SnapIn_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define SnapIn_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define SnapIn_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define SnapIn_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define SnapIn_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define SnapIn_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define SnapIn_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define SnapIn_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define SnapIn_get_Vendor(This,Vendor)	\
    ( (This)->lpVtbl -> get_Vendor(This,Vendor) ) 

#define SnapIn_get_Version(This,Version)	\
    ( (This)->lpVtbl -> get_Version(This,Version) ) 

#define SnapIn_get_Extensions(This,Extensions)	\
    ( (This)->lpVtbl -> get_Extensions(This,Extensions) ) 

#define SnapIn_get_SnapinCLSID(This,SnapinCLSID)	\
    ( (This)->lpVtbl -> get_SnapinCLSID(This,SnapinCLSID) ) 

#define SnapIn_get_Properties(This,Properties)	\
    ( (This)->lpVtbl -> get_Properties(This,Properties) ) 

#define SnapIn_EnableAllExtensions(This,Enable)	\
    ( (This)->lpVtbl -> EnableAllExtensions(This,Enable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __SnapIn_INTERFACE_DEFINED__ */


#ifndef __SnapIns_INTERFACE_DEFINED__
#define __SnapIns_INTERFACE_DEFINED__

/* interface SnapIns */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_SnapIns;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2EF3DE1D-B12A-49D1-92C5-0B00798768F1")
    SnapIns : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPIN SnapIn) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR SnapinNameOrCLSID,
            /* [optional][in] */ VARIANT ParentSnapin,
            /* [optional][in] */ VARIANT Properties,
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPIN SnapIn) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct SnapInsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in SnapIns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in SnapIns * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in SnapIns * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in SnapIns * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in SnapIns * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in SnapIns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            SnapIns * This,
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
        
        DECLSPEC_XFGVIRT(SnapIns, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in SnapIns * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(SnapIns, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in SnapIns * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPIN SnapIn);
        
        DECLSPEC_XFGVIRT(SnapIns, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in SnapIns * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        DECLSPEC_XFGVIRT(SnapIns, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in SnapIns * This,
            /* [in] */ __RPC__in BSTR SnapinNameOrCLSID,
            /* [optional][in] */ VARIANT ParentSnapin,
            /* [optional][in] */ VARIANT Properties,
            /* [retval][out] */ __RPC__deref_out_opt PPSNAPIN SnapIn);
        
        DECLSPEC_XFGVIRT(SnapIns, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in SnapIns * This,
            /* [in] */ __RPC__in_opt PSNAPIN SnapIn);
        
        END_INTERFACE
    } SnapInsVtbl;

    interface SnapIns
    {
        CONST_VTBL struct SnapInsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define SnapIns_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define SnapIns_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define SnapIns_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define SnapIns_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define SnapIns_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define SnapIns_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define SnapIns_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define SnapIns_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define SnapIns_Item(This,Index,SnapIn)	\
    ( (This)->lpVtbl -> Item(This,Index,SnapIn) ) 

#define SnapIns_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define SnapIns_Add(This,SnapinNameOrCLSID,ParentSnapin,Properties,SnapIn)	\
    ( (This)->lpVtbl -> Add(This,SnapinNameOrCLSID,ParentSnapin,Properties,SnapIn) ) 

#define SnapIns_Remove(This,SnapIn)	\
    ( (This)->lpVtbl -> Remove(This,SnapIn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __SnapIns_INTERFACE_DEFINED__ */


#ifndef __Extension_INTERFACE_DEFINED__
#define __Extension_INTERFACE_DEFINED__

/* interface Extension */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Extension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD4D6CA6-912F-409b-A26E-7FD234AEF542")
    Extension : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Vendor( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Vendor) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Version) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Extensions( 
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSIONS Extensions) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SnapinCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR SnapinCLSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableAllExtensions( 
            /* [in] */ BOOL Enable) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Enable( 
            /* [in] */ BOOL Enable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Extension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Extension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Extension * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Extension * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Extension * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Extension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Extension * This,
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
        
        DECLSPEC_XFGVIRT(Extension, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Extension * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name);
        
        DECLSPEC_XFGVIRT(Extension, get_Vendor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Vendor )( 
            __RPC__in Extension * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Vendor);
        
        DECLSPEC_XFGVIRT(Extension, get_Version)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in Extension * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Version);
        
        DECLSPEC_XFGVIRT(Extension, get_Extensions)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Extensions )( 
            __RPC__in Extension * This,
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSIONS Extensions);
        
        DECLSPEC_XFGVIRT(Extension, get_SnapinCLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SnapinCLSID )( 
            __RPC__in Extension * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR SnapinCLSID);
        
        DECLSPEC_XFGVIRT(Extension, EnableAllExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableAllExtensions )( 
            __RPC__in Extension * This,
            /* [in] */ BOOL Enable);
        
        DECLSPEC_XFGVIRT(Extension, Enable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in Extension * This,
            /* [in] */ BOOL Enable);
        
        END_INTERFACE
    } ExtensionVtbl;

    interface Extension
    {
        CONST_VTBL struct ExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Extension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Extension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Extension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Extension_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Extension_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Extension_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Extension_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Extension_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define Extension_get_Vendor(This,Vendor)	\
    ( (This)->lpVtbl -> get_Vendor(This,Vendor) ) 

#define Extension_get_Version(This,Version)	\
    ( (This)->lpVtbl -> get_Version(This,Version) ) 

#define Extension_get_Extensions(This,Extensions)	\
    ( (This)->lpVtbl -> get_Extensions(This,Extensions) ) 

#define Extension_get_SnapinCLSID(This,SnapinCLSID)	\
    ( (This)->lpVtbl -> get_SnapinCLSID(This,SnapinCLSID) ) 

#define Extension_EnableAllExtensions(This,Enable)	\
    ( (This)->lpVtbl -> EnableAllExtensions(This,Enable) ) 

#define Extension_Enable(This,Enable)	\
    ( (This)->lpVtbl -> Enable(This,Enable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Extension_INTERFACE_DEFINED__ */


#ifndef __Extensions_INTERFACE_DEFINED__
#define __Extensions_INTERFACE_DEFINED__

/* interface Extensions */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Extensions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82DBEA43-8CA4-44bc-A2CA-D18741059EC8")
    Extensions : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSION Extension) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ExtensionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Extensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Extensions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Extensions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Extensions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Extensions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Extensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Extensions * This,
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
        
        DECLSPEC_XFGVIRT(Extensions, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in Extensions * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(Extensions, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in Extensions * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPEXTENSION Extension);
        
        DECLSPEC_XFGVIRT(Extensions, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Extensions * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        END_INTERFACE
    } ExtensionsVtbl;

    interface Extensions
    {
        CONST_VTBL struct ExtensionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Extensions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Extensions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Extensions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Extensions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Extensions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Extensions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Extensions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Extensions_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define Extensions_Item(This,Index,Extension)	\
    ( (This)->lpVtbl -> Item(This,Index,Extension) ) 

#define Extensions_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Extensions_INTERFACE_DEFINED__ */


#ifndef __Columns_INTERFACE_DEFINED__
#define __Columns_INTERFACE_DEFINED__

/* interface Columns */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Columns;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("383D4D97-FC44-478B-B139-6323DC48611C")
    Columns : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPCOLUMN Column) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ColumnsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Columns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Columns * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Columns * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Columns * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Columns * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Columns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Columns * This,
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
        
        DECLSPEC_XFGVIRT(Columns, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in Columns * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPCOLUMN Column);
        
        DECLSPEC_XFGVIRT(Columns, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Columns * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        DECLSPEC_XFGVIRT(Columns, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in Columns * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        END_INTERFACE
    } ColumnsVtbl;

    interface Columns
    {
        CONST_VTBL struct ColumnsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Columns_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Columns_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Columns_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Columns_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Columns_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Columns_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Columns_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Columns_Item(This,Index,Column)	\
    ( (This)->lpVtbl -> Item(This,Index,Column) ) 

#define Columns_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define Columns_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Columns_INTERFACE_DEFINED__ */


#ifndef __Column_INTERFACE_DEFINED__
#define __Column_INTERFACE_DEFINED__

/* interface Column */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 

typedef 
enum ColumnSortOrder
    {
        SortOrder_Ascending	= 0,
        SortOrder_Descending	= ( SortOrder_Ascending + 1 ) 
    } 	_ColumnSortOrder;

typedef enum ColumnSortOrder COLUMNSORTORDER;


EXTERN_C const IID IID_Column;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD1C5F63-2B16-4D06-9AB3-F45350B940AB")
    Column : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out PLONG Width) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ long Width) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayPosition( 
            /* [retval][out] */ __RPC__out PLONG DisplayPosition) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayPosition( 
            /* [in] */ long Index) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Hidden( 
            /* [retval][out] */ __RPC__out PBOOL Hidden) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Hidden( 
            /* [in] */ BOOL Hidden) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAsSortColumn( 
            /* [in] */ COLUMNSORTORDER SortOrder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsSortColumn( 
            /* [retval][out] */ __RPC__out PBOOL IsSortColumn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ColumnVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Column * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Column * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Column * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Column * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Column * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Column * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Column * This,
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
        
        DECLSPEC_XFGVIRT(Column, Name)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Name )( 
            __RPC__in Column * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(Column, get_Width)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in Column * This,
            /* [retval][out] */ __RPC__out PLONG Width);
        
        DECLSPEC_XFGVIRT(Column, put_Width)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in Column * This,
            /* [in] */ long Width);
        
        DECLSPEC_XFGVIRT(Column, get_DisplayPosition)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayPosition )( 
            __RPC__in Column * This,
            /* [retval][out] */ __RPC__out PLONG DisplayPosition);
        
        DECLSPEC_XFGVIRT(Column, put_DisplayPosition)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayPosition )( 
            __RPC__in Column * This,
            /* [in] */ long Index);
        
        DECLSPEC_XFGVIRT(Column, get_Hidden)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Hidden )( 
            __RPC__in Column * This,
            /* [retval][out] */ __RPC__out PBOOL Hidden);
        
        DECLSPEC_XFGVIRT(Column, put_Hidden)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Hidden )( 
            __RPC__in Column * This,
            /* [in] */ BOOL Hidden);
        
        DECLSPEC_XFGVIRT(Column, SetAsSortColumn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAsSortColumn )( 
            __RPC__in Column * This,
            /* [in] */ COLUMNSORTORDER SortOrder);
        
        DECLSPEC_XFGVIRT(Column, IsSortColumn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsSortColumn )( 
            __RPC__in Column * This,
            /* [retval][out] */ __RPC__out PBOOL IsSortColumn);
        
        END_INTERFACE
    } ColumnVtbl;

    interface Column
    {
        CONST_VTBL struct ColumnVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Column_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Column_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Column_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Column_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Column_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Column_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Column_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Column_Name(This,Name)	\
    ( (This)->lpVtbl -> Name(This,Name) ) 

#define Column_get_Width(This,Width)	\
    ( (This)->lpVtbl -> get_Width(This,Width) ) 

#define Column_put_Width(This,Width)	\
    ( (This)->lpVtbl -> put_Width(This,Width) ) 

#define Column_get_DisplayPosition(This,DisplayPosition)	\
    ( (This)->lpVtbl -> get_DisplayPosition(This,DisplayPosition) ) 

#define Column_put_DisplayPosition(This,Index)	\
    ( (This)->lpVtbl -> put_DisplayPosition(This,Index) ) 

#define Column_get_Hidden(This,Hidden)	\
    ( (This)->lpVtbl -> get_Hidden(This,Hidden) ) 

#define Column_put_Hidden(This,Hidden)	\
    ( (This)->lpVtbl -> put_Hidden(This,Hidden) ) 

#define Column_SetAsSortColumn(This,SortOrder)	\
    ( (This)->lpVtbl -> SetAsSortColumn(This,SortOrder) ) 

#define Column_IsSortColumn(This,IsSortColumn)	\
    ( (This)->lpVtbl -> IsSortColumn(This,IsSortColumn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Column_INTERFACE_DEFINED__ */


#ifndef __Views_INTERFACE_DEFINED__
#define __Views_INTERFACE_DEFINED__

/* interface Views */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Views;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6B8C29D-A1FF-4D72-AAB0-E381E9B9338D")
    Views : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPVIEW View) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [defaultvalue][in] */ VIEWOPTIONS viewOptions = ViewOption_Default) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ViewsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Views * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Views * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Views * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Views * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Views * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Views * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Views * This,
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
        
        DECLSPEC_XFGVIRT(Views, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in Views * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPVIEW View);
        
        DECLSPEC_XFGVIRT(Views, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Views * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        DECLSPEC_XFGVIRT(Views, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in Views * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [defaultvalue][in] */ VIEWOPTIONS viewOptions);
        
        DECLSPEC_XFGVIRT(Views, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in Views * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        END_INTERFACE
    } ViewsVtbl;

    interface Views
    {
        CONST_VTBL struct ViewsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Views_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Views_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Views_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Views_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Views_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Views_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Views_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Views_Item(This,Index,View)	\
    ( (This)->lpVtbl -> Item(This,Index,View) ) 

#define Views_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define Views_Add(This,Node,viewOptions)	\
    ( (This)->lpVtbl -> Add(This,Node,viewOptions) ) 

#define Views_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Views_INTERFACE_DEFINED__ */


#ifndef __View_INTERFACE_DEFINED__
#define __View_INTERFACE_DEFINED__

/* interface View */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_View;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6EFC2DA2-B38C-457E-9ABB-ED2D189B8C38")
    View : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveScopeNode( 
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ActiveScopeNode( 
            /* [in] */ __RPC__in_opt PNODE Node) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Selection( 
            /* [retval][out] */ __RPC__deref_out_opt PPNODES Nodes) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ListItems( 
            /* [retval][out] */ __RPC__deref_out_opt PPNODES Nodes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SnapinScopeObject( 
            /* [optional][in] */ VARIANT ScopeNode,
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH ScopeNodeObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SnapinSelectionObject( 
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH SelectionObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Is( 
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *TheSame) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Document( 
            /* [retval][out] */ __RPC__deref_out_opt PPDOCUMENT Document) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectAll( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Select( 
            /* [in] */ __RPC__in_opt PNODE Node) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Deselect( 
            /* [in] */ __RPC__in_opt PNODE Node) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsSelected( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__out PBOOL IsSelected) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisplayScopeNodePropertySheet( 
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisplaySelectionPropertySheet( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyScopeNode( 
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopySelection( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteScopeNode( 
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteSelection( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RenameScopeNode( 
            /* [in] */ __RPC__in BSTR NewName,
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RenameSelectedItem( 
            /* [in] */ __RPC__in BSTR NewName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ScopeNodeContextMenu( 
            /* [optional][in] */ VARIANT ScopeNode,
            /* [retval][out] */ __RPC__deref_out_opt PPCONTEXTMENU ContextMenu) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelectionContextMenu( 
            /* [retval][out] */ __RPC__deref_out_opt PPCONTEXTMENU ContextMenu) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RefreshScopeNode( 
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RefreshSelection( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecuteSelectionMenuItem( 
            /* [in] */ __RPC__in BSTR MenuItemPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecuteScopeNodeMenuItem( 
            /* [in] */ __RPC__in BSTR MenuItemPath,
            /* [optional][in] */ VARIANT ScopeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecuteShellCommand( 
            /* [in] */ __RPC__in BSTR Command,
            /* [in] */ __RPC__in BSTR Directory,
            /* [in] */ __RPC__in BSTR Parameters,
            /* [in] */ __RPC__in BSTR WindowState) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Frame( 
            /* [retval][out] */ __RPC__deref_out_opt PPFRAME Frame) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ScopeTreeVisible( 
            /* [retval][out] */ __RPC__out PBOOL Visible) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ScopeTreeVisible( 
            /* [in] */ BOOL Visible) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Back( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Forward( void) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StatusBarText( 
            /* [in] */ __RPC__in BSTR StatusBarText) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Memento( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Memento) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ViewMemento( 
            /* [in] */ __RPC__in BSTR Memento) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__deref_out_opt PPCOLUMNS Columns) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CellContents( 
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [in] */ long Column,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR CellContents) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportList( 
            /* [in] */ __RPC__in BSTR File,
            /* [defaultvalue][in] */ EXPORTLISTOPTIONS exportoptions = ExportListOptions_Default) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ListViewMode( 
            /* [retval][out] */ __RPC__out PLISTVIEWMODE Mode) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ListViewMode( 
            /* [in] */ LISTVIEWMODE mode) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ControlObject( 
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH Control) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in View * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in View * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            View * This,
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
        
        DECLSPEC_XFGVIRT(View, get_ActiveScopeNode)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveScopeNode )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node);
        
        DECLSPEC_XFGVIRT(View, put_ActiveScopeNode)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveScopeNode )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PNODE Node);
        
        DECLSPEC_XFGVIRT(View, get_Selection)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Selection )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPNODES Nodes);
        
        DECLSPEC_XFGVIRT(View, get_ListItems)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ListItems )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPNODES Nodes);
        
        DECLSPEC_XFGVIRT(View, SnapinScopeObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SnapinScopeObject )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode,
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH ScopeNodeObject);
        
        DECLSPEC_XFGVIRT(View, SnapinSelectionObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SnapinSelectionObject )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH SelectionObject);
        
        DECLSPEC_XFGVIRT(View, Is)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Is )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PVIEW View,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *TheSame);
        
        DECLSPEC_XFGVIRT(View, get_Document)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPDOCUMENT Document);
        
        DECLSPEC_XFGVIRT(View, SelectAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectAll )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, Select)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PNODE Node);
        
        DECLSPEC_XFGVIRT(View, Deselect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deselect )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PNODE Node);
        
        DECLSPEC_XFGVIRT(View, IsSelected)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsSelected )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [retval][out] */ __RPC__out PBOOL IsSelected);
        
        DECLSPEC_XFGVIRT(View, DisplayScopeNodePropertySheet)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisplayScopeNodePropertySheet )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, DisplaySelectionPropertySheet)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisplaySelectionPropertySheet )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, CopyScopeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyScopeNode )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, CopySelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopySelection )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, DeleteScopeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteScopeNode )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, DeleteSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSelection )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, RenameScopeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RenameScopeNode )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR NewName,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, RenameSelectedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RenameSelectedItem )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR NewName);
        
        DECLSPEC_XFGVIRT(View, get_ScopeNodeContextMenu)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScopeNodeContextMenu )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode,
            /* [retval][out] */ __RPC__deref_out_opt PPCONTEXTMENU ContextMenu);
        
        DECLSPEC_XFGVIRT(View, get_SelectionContextMenu)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelectionContextMenu )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPCONTEXTMENU ContextMenu);
        
        DECLSPEC_XFGVIRT(View, RefreshScopeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshScopeNode )( 
            __RPC__in View * This,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, RefreshSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshSelection )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, ExecuteSelectionMenuItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteSelectionMenuItem )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR MenuItemPath);
        
        DECLSPEC_XFGVIRT(View, ExecuteScopeNodeMenuItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteScopeNodeMenuItem )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR MenuItemPath,
            /* [optional][in] */ VARIANT ScopeNode);
        
        DECLSPEC_XFGVIRT(View, ExecuteShellCommand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteShellCommand )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR Command,
            /* [in] */ __RPC__in BSTR Directory,
            /* [in] */ __RPC__in BSTR Parameters,
            /* [in] */ __RPC__in BSTR WindowState);
        
        DECLSPEC_XFGVIRT(View, get_Frame)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Frame )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPFRAME Frame);
        
        DECLSPEC_XFGVIRT(View, Close)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, get_ScopeTreeVisible)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScopeTreeVisible )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__out PBOOL Visible);
        
        DECLSPEC_XFGVIRT(View, put_ScopeTreeVisible)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ScopeTreeVisible )( 
            __RPC__in View * This,
            /* [in] */ BOOL Visible);
        
        DECLSPEC_XFGVIRT(View, Back)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Back )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, Forward)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Forward )( 
            __RPC__in View * This);
        
        DECLSPEC_XFGVIRT(View, put_StatusBarText)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StatusBarText )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR StatusBarText);
        
        DECLSPEC_XFGVIRT(View, get_Memento)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Memento )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Memento);
        
        DECLSPEC_XFGVIRT(View, ViewMemento)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ViewMemento )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR Memento);
        
        DECLSPEC_XFGVIRT(View, get_Columns)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPCOLUMNS Columns);
        
        DECLSPEC_XFGVIRT(View, get_CellContents)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CellContents )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in_opt PNODE Node,
            /* [in] */ long Column,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR CellContents);
        
        DECLSPEC_XFGVIRT(View, ExportList)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportList )( 
            __RPC__in View * This,
            /* [in] */ __RPC__in BSTR File,
            /* [defaultvalue][in] */ EXPORTLISTOPTIONS exportoptions);
        
        DECLSPEC_XFGVIRT(View, get_ListViewMode)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ListViewMode )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__out PLISTVIEWMODE Mode);
        
        DECLSPEC_XFGVIRT(View, put_ListViewMode)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ListViewMode )( 
            __RPC__in View * This,
            /* [in] */ LISTVIEWMODE mode);
        
        DECLSPEC_XFGVIRT(View, get_ControlObject)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ControlObject )( 
            __RPC__in View * This,
            /* [retval][out] */ __RPC__deref_out_opt PPDISPATCH Control);
        
        END_INTERFACE
    } ViewVtbl;

    interface View
    {
        CONST_VTBL struct ViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define View_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define View_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define View_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define View_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define View_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define View_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define View_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define View_get_ActiveScopeNode(This,Node)	\
    ( (This)->lpVtbl -> get_ActiveScopeNode(This,Node) ) 

#define View_put_ActiveScopeNode(This,Node)	\
    ( (This)->lpVtbl -> put_ActiveScopeNode(This,Node) ) 

#define View_get_Selection(This,Nodes)	\
    ( (This)->lpVtbl -> get_Selection(This,Nodes) ) 

#define View_get_ListItems(This,Nodes)	\
    ( (This)->lpVtbl -> get_ListItems(This,Nodes) ) 

#define View_SnapinScopeObject(This,ScopeNode,ScopeNodeObject)	\
    ( (This)->lpVtbl -> SnapinScopeObject(This,ScopeNode,ScopeNodeObject) ) 

#define View_SnapinSelectionObject(This,SelectionObject)	\
    ( (This)->lpVtbl -> SnapinSelectionObject(This,SelectionObject) ) 

#define View_Is(This,View,TheSame)	\
    ( (This)->lpVtbl -> Is(This,View,TheSame) ) 

#define View_get_Document(This,Document)	\
    ( (This)->lpVtbl -> get_Document(This,Document) ) 

#define View_SelectAll(This)	\
    ( (This)->lpVtbl -> SelectAll(This) ) 

#define View_Select(This,Node)	\
    ( (This)->lpVtbl -> Select(This,Node) ) 

#define View_Deselect(This,Node)	\
    ( (This)->lpVtbl -> Deselect(This,Node) ) 

#define View_IsSelected(This,Node,IsSelected)	\
    ( (This)->lpVtbl -> IsSelected(This,Node,IsSelected) ) 

#define View_DisplayScopeNodePropertySheet(This,ScopeNode)	\
    ( (This)->lpVtbl -> DisplayScopeNodePropertySheet(This,ScopeNode) ) 

#define View_DisplaySelectionPropertySheet(This)	\
    ( (This)->lpVtbl -> DisplaySelectionPropertySheet(This) ) 

#define View_CopyScopeNode(This,ScopeNode)	\
    ( (This)->lpVtbl -> CopyScopeNode(This,ScopeNode) ) 

#define View_CopySelection(This)	\
    ( (This)->lpVtbl -> CopySelection(This) ) 

#define View_DeleteScopeNode(This,ScopeNode)	\
    ( (This)->lpVtbl -> DeleteScopeNode(This,ScopeNode) ) 

#define View_DeleteSelection(This)	\
    ( (This)->lpVtbl -> DeleteSelection(This) ) 

#define View_RenameScopeNode(This,NewName,ScopeNode)	\
    ( (This)->lpVtbl -> RenameScopeNode(This,NewName,ScopeNode) ) 

#define View_RenameSelectedItem(This,NewName)	\
    ( (This)->lpVtbl -> RenameSelectedItem(This,NewName) ) 

#define View_get_ScopeNodeContextMenu(This,ScopeNode,ContextMenu)	\
    ( (This)->lpVtbl -> get_ScopeNodeContextMenu(This,ScopeNode,ContextMenu) ) 

#define View_get_SelectionContextMenu(This,ContextMenu)	\
    ( (This)->lpVtbl -> get_SelectionContextMenu(This,ContextMenu) ) 

#define View_RefreshScopeNode(This,ScopeNode)	\
    ( (This)->lpVtbl -> RefreshScopeNode(This,ScopeNode) ) 

#define View_RefreshSelection(This)	\
    ( (This)->lpVtbl -> RefreshSelection(This) ) 

#define View_ExecuteSelectionMenuItem(This,MenuItemPath)	\
    ( (This)->lpVtbl -> ExecuteSelectionMenuItem(This,MenuItemPath) ) 

#define View_ExecuteScopeNodeMenuItem(This,MenuItemPath,ScopeNode)	\
    ( (This)->lpVtbl -> ExecuteScopeNodeMenuItem(This,MenuItemPath,ScopeNode) ) 

#define View_ExecuteShellCommand(This,Command,Directory,Parameters,WindowState)	\
    ( (This)->lpVtbl -> ExecuteShellCommand(This,Command,Directory,Parameters,WindowState) ) 

#define View_get_Frame(This,Frame)	\
    ( (This)->lpVtbl -> get_Frame(This,Frame) ) 

#define View_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define View_get_ScopeTreeVisible(This,Visible)	\
    ( (This)->lpVtbl -> get_ScopeTreeVisible(This,Visible) ) 

#define View_put_ScopeTreeVisible(This,Visible)	\
    ( (This)->lpVtbl -> put_ScopeTreeVisible(This,Visible) ) 

#define View_Back(This)	\
    ( (This)->lpVtbl -> Back(This) ) 

#define View_Forward(This)	\
    ( (This)->lpVtbl -> Forward(This) ) 

#define View_put_StatusBarText(This,StatusBarText)	\
    ( (This)->lpVtbl -> put_StatusBarText(This,StatusBarText) ) 

#define View_get_Memento(This,Memento)	\
    ( (This)->lpVtbl -> get_Memento(This,Memento) ) 

#define View_ViewMemento(This,Memento)	\
    ( (This)->lpVtbl -> ViewMemento(This,Memento) ) 

#define View_get_Columns(This,Columns)	\
    ( (This)->lpVtbl -> get_Columns(This,Columns) ) 

#define View_get_CellContents(This,Node,Column,CellContents)	\
    ( (This)->lpVtbl -> get_CellContents(This,Node,Column,CellContents) ) 

#define View_ExportList(This,File,exportoptions)	\
    ( (This)->lpVtbl -> ExportList(This,File,exportoptions) ) 

#define View_get_ListViewMode(This,Mode)	\
    ( (This)->lpVtbl -> get_ListViewMode(This,Mode) ) 

#define View_put_ListViewMode(This,mode)	\
    ( (This)->lpVtbl -> put_ListViewMode(This,mode) ) 

#define View_get_ControlObject(This,Control)	\
    ( (This)->lpVtbl -> get_ControlObject(This,Control) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __View_INTERFACE_DEFINED__ */


#ifndef __Nodes_INTERFACE_DEFINED__
#define __Nodes_INTERFACE_DEFINED__

/* interface Nodes */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Nodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("313B01DF-B22F-4D42-B1B8-483CDCF51D35")
    Nodes : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct NodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Nodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Nodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Nodes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Nodes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Nodes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Nodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Nodes * This,
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
        
        DECLSPEC_XFGVIRT(Nodes, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in Nodes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(Nodes, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in Nodes * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt PPNODE Node);
        
        DECLSPEC_XFGVIRT(Nodes, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Nodes * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        END_INTERFACE
    } NodesVtbl;

    interface Nodes
    {
        CONST_VTBL struct NodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Nodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Nodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Nodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Nodes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Nodes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Nodes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Nodes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Nodes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define Nodes_Item(This,Index,Node)	\
    ( (This)->lpVtbl -> Item(This,Index,Node) ) 

#define Nodes_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Nodes_INTERFACE_DEFINED__ */


#ifndef __ContextMenu_INTERFACE_DEFINED__
#define __ContextMenu_INTERFACE_DEFINED__

/* interface ContextMenu */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_ContextMenu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DAB39CE0-25E6-4E07-8362-BA9C95706545")
    ContextMenu : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT IndexOrPath,
            /* [retval][out] */ __RPC__deref_out_opt PPMENUITEM MenuItem) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ContextMenuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ContextMenu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ContextMenu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ContextMenu * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ContextMenu * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ContextMenu * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ContextMenu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ContextMenu * This,
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
        
        DECLSPEC_XFGVIRT(ContextMenu, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ContextMenu * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ContextMenu, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ContextMenu * This,
            /* [in] */ VARIANT IndexOrPath,
            /* [retval][out] */ __RPC__deref_out_opt PPMENUITEM MenuItem);
        
        DECLSPEC_XFGVIRT(ContextMenu, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ContextMenu * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        END_INTERFACE
    } ContextMenuVtbl;

    interface ContextMenu
    {
        CONST_VTBL struct ContextMenuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ContextMenu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ContextMenu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ContextMenu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ContextMenu_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ContextMenu_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ContextMenu_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ContextMenu_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ContextMenu_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ContextMenu_get_Item(This,IndexOrPath,MenuItem)	\
    ( (This)->lpVtbl -> get_Item(This,IndexOrPath,MenuItem) ) 

#define ContextMenu_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ContextMenu_INTERFACE_DEFINED__ */


#ifndef __MenuItem_INTERFACE_DEFINED__
#define __MenuItem_INTERFACE_DEFINED__

/* interface MenuItem */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_MenuItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0178FAD1-B361-4B27-96AD-67C57EBF2E1D")
    MenuItem : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR DisplayName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LanguageIndependentName( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR LanguageIndependentName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Path) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LanguageIndependentPath( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR LanguageIndependentPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Execute( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out PBOOL Enabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct MenuItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in MenuItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in MenuItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in MenuItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in MenuItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in MenuItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in MenuItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            MenuItem * This,
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
        
        DECLSPEC_XFGVIRT(MenuItem, get_DisplayName)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in MenuItem * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR DisplayName);
        
        DECLSPEC_XFGVIRT(MenuItem, get_LanguageIndependentName)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LanguageIndependentName )( 
            __RPC__in MenuItem * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR LanguageIndependentName);
        
        DECLSPEC_XFGVIRT(MenuItem, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in MenuItem * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Path);
        
        DECLSPEC_XFGVIRT(MenuItem, get_LanguageIndependentPath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LanguageIndependentPath )( 
            __RPC__in MenuItem * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR LanguageIndependentPath);
        
        DECLSPEC_XFGVIRT(MenuItem, Execute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in MenuItem * This);
        
        DECLSPEC_XFGVIRT(MenuItem, get_Enabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in MenuItem * This,
            /* [retval][out] */ __RPC__out PBOOL Enabled);
        
        END_INTERFACE
    } MenuItemVtbl;

    interface MenuItem
    {
        CONST_VTBL struct MenuItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define MenuItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define MenuItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define MenuItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define MenuItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define MenuItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define MenuItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define MenuItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define MenuItem_get_DisplayName(This,DisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,DisplayName) ) 

#define MenuItem_get_LanguageIndependentName(This,LanguageIndependentName)	\
    ( (This)->lpVtbl -> get_LanguageIndependentName(This,LanguageIndependentName) ) 

#define MenuItem_get_Path(This,Path)	\
    ( (This)->lpVtbl -> get_Path(This,Path) ) 

#define MenuItem_get_LanguageIndependentPath(This,LanguageIndependentPath)	\
    ( (This)->lpVtbl -> get_LanguageIndependentPath(This,LanguageIndependentPath) ) 

#define MenuItem_Execute(This)	\
    ( (This)->lpVtbl -> Execute(This) ) 

#define MenuItem_get_Enabled(This,Enabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,Enabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __MenuItem_INTERFACE_DEFINED__ */


#ifndef __Properties_INTERFACE_DEFINED__
#define __Properties_INTERFACE_DEFINED__

/* interface Properties */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Properties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2886ABC2-A425-42b2-91C6-E25C0E04581C")
    Properties : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTY Property) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out PLONG Count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR Name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct PropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Properties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Properties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Properties * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Properties * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Properties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Properties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Properties * This,
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
        
        DECLSPEC_XFGVIRT(Properties, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in Properties * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(Properties, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in Properties * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt PPPROPERTY Property);
        
        DECLSPEC_XFGVIRT(Properties, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Properties * This,
            /* [retval][out] */ __RPC__out PLONG Count);
        
        DECLSPEC_XFGVIRT(Properties, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in Properties * This,
            /* [in] */ __RPC__in BSTR Name);
        
        END_INTERFACE
    } PropertiesVtbl;

    interface Properties
    {
        CONST_VTBL struct PropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Properties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Properties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Properties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Properties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Properties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Properties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Properties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Properties_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define Properties_Item(This,Name,Property)	\
    ( (This)->lpVtbl -> Item(This,Name,Property) ) 

#define Properties_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define Properties_Remove(This,Name)	\
    ( (This)->lpVtbl -> Remove(This,Name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Properties_INTERFACE_DEFINED__ */


#ifndef __Property_INTERFACE_DEFINED__
#define __Property_INTERFACE_DEFINED__

/* interface Property */
/* [object][helpstring][dual][uuid][nonextensible][dual][oleautomation] */ 


EXTERN_C const IID IID_Property;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4600C3A5-E301-41d8-B6D0-EF2E4212E0CA")
    Property : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out PVARIANT Value) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct PropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Property * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Property * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Property * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Property * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Property * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Property * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Property * This,
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
        
        DECLSPEC_XFGVIRT(Property, get_Value)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in Property * This,
            /* [retval][out] */ __RPC__out PVARIANT Value);
        
        DECLSPEC_XFGVIRT(Property, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in Property * This,
            /* [in] */ VARIANT Value);
        
        DECLSPEC_XFGVIRT(Property, get_Name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Property * This,
            /* [retval][out] */ __RPC__deref_out_opt PBSTR Name);
        
        END_INTERFACE
    } PropertyVtbl;

    interface Property
    {
        CONST_VTBL struct PropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Property_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Property_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Property_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Property_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Property_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Property_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Property_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Property_get_Value(This,Value)	\
    ( (This)->lpVtbl -> get_Value(This,Value) ) 

#define Property_put_Value(This,Value)	\
    ( (This)->lpVtbl -> put_Value(This,Value) ) 

#define Property_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Property_INTERFACE_DEFINED__ */

#endif /* __MMC20_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mmcobj_0000_0003 */
/* [local] */ 

#endif // MMC_VER >= 0x0200
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mmcobj_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmcobj_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


