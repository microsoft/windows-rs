

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

#ifndef __editionupgradehelper_h__
#define __editionupgradehelper_h__

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

#ifndef __IEditionUpgradeHelper_FWD_DEFINED__
#define __IEditionUpgradeHelper_FWD_DEFINED__
typedef interface IEditionUpgradeHelper IEditionUpgradeHelper;

#endif 	/* __IEditionUpgradeHelper_FWD_DEFINED__ */


#ifndef __IWindowsLockModeHelper_FWD_DEFINED__
#define __IWindowsLockModeHelper_FWD_DEFINED__
typedef interface IWindowsLockModeHelper IWindowsLockModeHelper;

#endif 	/* __IWindowsLockModeHelper_FWD_DEFINED__ */


#ifndef __IEditionUpgradeBroker_FWD_DEFINED__
#define __IEditionUpgradeBroker_FWD_DEFINED__
typedef interface IEditionUpgradeBroker IEditionUpgradeBroker;

#endif 	/* __IEditionUpgradeBroker_FWD_DEFINED__ */


#ifndef __IContainerActivationHelper_FWD_DEFINED__
#define __IContainerActivationHelper_FWD_DEFINED__
typedef interface IContainerActivationHelper IContainerActivationHelper;

#endif 	/* __IContainerActivationHelper_FWD_DEFINED__ */


#ifndef __IClipServiceNotificationHelper_FWD_DEFINED__
#define __IClipServiceNotificationHelper_FWD_DEFINED__
typedef interface IClipServiceNotificationHelper IClipServiceNotificationHelper;

#endif 	/* __IClipServiceNotificationHelper_FWD_DEFINED__ */


#ifndef __IFClipNotificationHelper_FWD_DEFINED__
#define __IFClipNotificationHelper_FWD_DEFINED__
typedef interface IFClipNotificationHelper IFClipNotificationHelper;

#endif 	/* __IFClipNotificationHelper_FWD_DEFINED__ */


#ifndef __EditionUpgradeHelper_FWD_DEFINED__
#define __EditionUpgradeHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class EditionUpgradeHelper EditionUpgradeHelper;
#else
typedef struct EditionUpgradeHelper EditionUpgradeHelper;
#endif /* __cplusplus */

#endif 	/* __EditionUpgradeHelper_FWD_DEFINED__ */


#ifndef __EditionUpgradeBroker_FWD_DEFINED__
#define __EditionUpgradeBroker_FWD_DEFINED__

#ifdef __cplusplus
typedef class EditionUpgradeBroker EditionUpgradeBroker;
#else
typedef struct EditionUpgradeBroker EditionUpgradeBroker;
#endif /* __cplusplus */

#endif 	/* __EditionUpgradeBroker_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_editionupgradehelper_0000_0000 */
/* [local] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_editionupgradehelper_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_editionupgradehelper_0000_0000_v0_0_s_ifspec;

#ifndef __IEditionUpgradeHelper_INTERFACE_DEFINED__
#define __IEditionUpgradeHelper_INTERFACE_DEFINED__

/* interface IEditionUpgradeHelper */
/* [uuid][object] */ 


EXTERN_C const IID IID_IEditionUpgradeHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D3E9E342-5DEB-43B6-849E-6913B85D503A")
    IEditionUpgradeHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CanUpgrade( 
            /* [out] */ __RPC__out BOOL *isAllowed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateOperatingSystem( 
            /* [string][in] */ __RPC__in_string LPCWSTR contentId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowProductKeyUI( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOsProductContentId( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *contentId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGenuineLocalStatus( 
            /* [out] */ __RPC__out BOOL *isGenuine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEditionUpgradeHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEditionUpgradeHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEditionUpgradeHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEditionUpgradeHelper * This);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeHelper, CanUpgrade)
        HRESULT ( STDMETHODCALLTYPE *CanUpgrade )( 
            __RPC__in IEditionUpgradeHelper * This,
            /* [out] */ __RPC__out BOOL *isAllowed);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeHelper, UpdateOperatingSystem)
        HRESULT ( STDMETHODCALLTYPE *UpdateOperatingSystem )( 
            __RPC__in IEditionUpgradeHelper * This,
            /* [string][in] */ __RPC__in_string LPCWSTR contentId);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeHelper, ShowProductKeyUI)
        HRESULT ( STDMETHODCALLTYPE *ShowProductKeyUI )( 
            __RPC__in IEditionUpgradeHelper * This);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeHelper, GetOsProductContentId)
        HRESULT ( STDMETHODCALLTYPE *GetOsProductContentId )( 
            __RPC__in IEditionUpgradeHelper * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *contentId);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeHelper, GetGenuineLocalStatus)
        HRESULT ( STDMETHODCALLTYPE *GetGenuineLocalStatus )( 
            __RPC__in IEditionUpgradeHelper * This,
            /* [out] */ __RPC__out BOOL *isGenuine);
        
        END_INTERFACE
    } IEditionUpgradeHelperVtbl;

    interface IEditionUpgradeHelper
    {
        CONST_VTBL struct IEditionUpgradeHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEditionUpgradeHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEditionUpgradeHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEditionUpgradeHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEditionUpgradeHelper_CanUpgrade(This,isAllowed)	\
    ( (This)->lpVtbl -> CanUpgrade(This,isAllowed) ) 

#define IEditionUpgradeHelper_UpdateOperatingSystem(This,contentId)	\
    ( (This)->lpVtbl -> UpdateOperatingSystem(This,contentId) ) 

#define IEditionUpgradeHelper_ShowProductKeyUI(This)	\
    ( (This)->lpVtbl -> ShowProductKeyUI(This) ) 

#define IEditionUpgradeHelper_GetOsProductContentId(This,contentId)	\
    ( (This)->lpVtbl -> GetOsProductContentId(This,contentId) ) 

#define IEditionUpgradeHelper_GetGenuineLocalStatus(This,isGenuine)	\
    ( (This)->lpVtbl -> GetGenuineLocalStatus(This,isGenuine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEditionUpgradeHelper_INTERFACE_DEFINED__ */


#ifndef __IWindowsLockModeHelper_INTERFACE_DEFINED__
#define __IWindowsLockModeHelper_INTERFACE_DEFINED__

/* interface IWindowsLockModeHelper */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWindowsLockModeHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F342D19E-CC22-4648-BB5D-03CCF75B47C5")
    IWindowsLockModeHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSMode( 
            /* [out] */ __RPC__out BOOL *isSmode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsLockModeHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWindowsLockModeHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWindowsLockModeHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWindowsLockModeHelper * This);
        
        DECLSPEC_XFGVIRT(IWindowsLockModeHelper, GetSMode)
        HRESULT ( STDMETHODCALLTYPE *GetSMode )( 
            __RPC__in IWindowsLockModeHelper * This,
            /* [out] */ __RPC__out BOOL *isSmode);
        
        END_INTERFACE
    } IWindowsLockModeHelperVtbl;

    interface IWindowsLockModeHelper
    {
        CONST_VTBL struct IWindowsLockModeHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsLockModeHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsLockModeHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsLockModeHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsLockModeHelper_GetSMode(This,isSmode)	\
    ( (This)->lpVtbl -> GetSMode(This,isSmode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsLockModeHelper_INTERFACE_DEFINED__ */


#ifndef __IEditionUpgradeBroker_INTERFACE_DEFINED__
#define __IEditionUpgradeBroker_INTERFACE_DEFINED__

/* interface IEditionUpgradeBroker */
/* [unique][helpstring][oleautomation][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IEditionUpgradeBroker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FF19CBCF-9455-4937-B872-6B7929A460AF")
    IEditionUpgradeBroker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeParentWindow( 
            /* [in] */ OLE_HANDLE parentHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateOperatingSystem( 
            /* [string][in] */ __RPC__in_string BSTR parameter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowProductKeyUI( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanUpgrade( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEditionUpgradeBrokerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEditionUpgradeBroker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEditionUpgradeBroker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEditionUpgradeBroker * This);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeBroker, InitializeParentWindow)
        HRESULT ( STDMETHODCALLTYPE *InitializeParentWindow )( 
            __RPC__in IEditionUpgradeBroker * This,
            /* [in] */ OLE_HANDLE parentHandle);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeBroker, UpdateOperatingSystem)
        HRESULT ( STDMETHODCALLTYPE *UpdateOperatingSystem )( 
            __RPC__in IEditionUpgradeBroker * This,
            /* [string][in] */ __RPC__in_string BSTR parameter);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeBroker, ShowProductKeyUI)
        HRESULT ( STDMETHODCALLTYPE *ShowProductKeyUI )( 
            __RPC__in IEditionUpgradeBroker * This);
        
        DECLSPEC_XFGVIRT(IEditionUpgradeBroker, CanUpgrade)
        HRESULT ( STDMETHODCALLTYPE *CanUpgrade )( 
            __RPC__in IEditionUpgradeBroker * This);
        
        END_INTERFACE
    } IEditionUpgradeBrokerVtbl;

    interface IEditionUpgradeBroker
    {
        CONST_VTBL struct IEditionUpgradeBrokerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEditionUpgradeBroker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEditionUpgradeBroker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEditionUpgradeBroker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEditionUpgradeBroker_InitializeParentWindow(This,parentHandle)	\
    ( (This)->lpVtbl -> InitializeParentWindow(This,parentHandle) ) 

#define IEditionUpgradeBroker_UpdateOperatingSystem(This,parameter)	\
    ( (This)->lpVtbl -> UpdateOperatingSystem(This,parameter) ) 

#define IEditionUpgradeBroker_ShowProductKeyUI(This)	\
    ( (This)->lpVtbl -> ShowProductKeyUI(This) ) 

#define IEditionUpgradeBroker_CanUpgrade(This)	\
    ( (This)->lpVtbl -> CanUpgrade(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEditionUpgradeBroker_INTERFACE_DEFINED__ */


#ifndef __IContainerActivationHelper_INTERFACE_DEFINED__
#define __IContainerActivationHelper_INTERFACE_DEFINED__

/* interface IContainerActivationHelper */
/* [uuid][nonextensible][oleautomation][dual][object] */ 


EXTERN_C const IID IID_IContainerActivationHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B524F93F-80D5-4EC7-AE9E-D66E93ADE1FA")
    IContainerActivationHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CanActivateClientVM( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isAllowed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContainerActivationHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContainerActivationHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContainerActivationHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContainerActivationHelper * This);
        
        DECLSPEC_XFGVIRT(IContainerActivationHelper, CanActivateClientVM)
        HRESULT ( STDMETHODCALLTYPE *CanActivateClientVM )( 
            __RPC__in IContainerActivationHelper * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isAllowed);
        
        END_INTERFACE
    } IContainerActivationHelperVtbl;

    interface IContainerActivationHelper
    {
        CONST_VTBL struct IContainerActivationHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContainerActivationHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContainerActivationHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContainerActivationHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContainerActivationHelper_CanActivateClientVM(This,isAllowed)	\
    ( (This)->lpVtbl -> CanActivateClientVM(This,isAllowed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContainerActivationHelper_INTERFACE_DEFINED__ */


#ifndef __IClipServiceNotificationHelper_INTERFACE_DEFINED__
#define __IClipServiceNotificationHelper_INTERFACE_DEFINED__

/* interface IClipServiceNotificationHelper */
/* [uuid][nonextensible][oleautomation][dual][object] */ 


EXTERN_C const IID IID_IClipServiceNotificationHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C39948F0-6142-44FD-98CA-E1681A8D68B5")
    IClipServiceNotificationHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowToast( 
            /* [string][in] */ __RPC__in_string BSTR titleText,
            /* [string][in] */ __RPC__in_string BSTR bodyText,
            /* [string][in] */ __RPC__in_string BSTR packageName,
            /* [string][in] */ __RPC__in_string BSTR appId,
            /* [string][in] */ __RPC__in_string BSTR launchCommand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClipServiceNotificationHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IClipServiceNotificationHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IClipServiceNotificationHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IClipServiceNotificationHelper * This);
        
        DECLSPEC_XFGVIRT(IClipServiceNotificationHelper, ShowToast)
        HRESULT ( STDMETHODCALLTYPE *ShowToast )( 
            __RPC__in IClipServiceNotificationHelper * This,
            /* [string][in] */ __RPC__in_string BSTR titleText,
            /* [string][in] */ __RPC__in_string BSTR bodyText,
            /* [string][in] */ __RPC__in_string BSTR packageName,
            /* [string][in] */ __RPC__in_string BSTR appId,
            /* [string][in] */ __RPC__in_string BSTR launchCommand);
        
        END_INTERFACE
    } IClipServiceNotificationHelperVtbl;

    interface IClipServiceNotificationHelper
    {
        CONST_VTBL struct IClipServiceNotificationHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClipServiceNotificationHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClipServiceNotificationHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClipServiceNotificationHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClipServiceNotificationHelper_ShowToast(This,titleText,bodyText,packageName,appId,launchCommand)	\
    ( (This)->lpVtbl -> ShowToast(This,titleText,bodyText,packageName,appId,launchCommand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClipServiceNotificationHelper_INTERFACE_DEFINED__ */


#ifndef __IFClipNotificationHelper_INTERFACE_DEFINED__
#define __IFClipNotificationHelper_INTERFACE_DEFINED__

/* interface IFClipNotificationHelper */
/* [uuid][nonextensible][oleautomation][dual][object] */ 


EXTERN_C const IID IID_IFClipNotificationHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D5E3D21-BD41-4C2A-A669-B17CE87FB50B")
    IFClipNotificationHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowSystemDialog( 
            /* [string][in] */ __RPC__in_string BSTR titleText,
            /* [string][in] */ __RPC__in_string BSTR bodyText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFClipNotificationHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFClipNotificationHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFClipNotificationHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFClipNotificationHelper * This);
        
        DECLSPEC_XFGVIRT(IFClipNotificationHelper, ShowSystemDialog)
        HRESULT ( STDMETHODCALLTYPE *ShowSystemDialog )( 
            __RPC__in IFClipNotificationHelper * This,
            /* [string][in] */ __RPC__in_string BSTR titleText,
            /* [string][in] */ __RPC__in_string BSTR bodyText);
        
        END_INTERFACE
    } IFClipNotificationHelperVtbl;

    interface IFClipNotificationHelper
    {
        CONST_VTBL struct IFClipNotificationHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFClipNotificationHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFClipNotificationHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFClipNotificationHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFClipNotificationHelper_ShowSystemDialog(This,titleText,bodyText)	\
    ( (This)->lpVtbl -> ShowSystemDialog(This,titleText,bodyText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFClipNotificationHelper_INTERFACE_DEFINED__ */



#ifndef __EditionUpgradeHelperLib_LIBRARY_DEFINED__
#define __EditionUpgradeHelperLib_LIBRARY_DEFINED__

/* library EditionUpgradeHelperLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_EditionUpgradeHelperLib;

EXTERN_C const CLSID CLSID_EditionUpgradeHelper;

#ifdef __cplusplus

class DECLSPEC_UUID("01776DF3-B9AF-4E50-9B1C-56E93116D704")
EditionUpgradeHelper;
#endif

EXTERN_C const CLSID CLSID_EditionUpgradeBroker;

#ifdef __cplusplus

class DECLSPEC_UUID("C4270827-4F39-45DF-9288-12FF6B85A921")
EditionUpgradeBroker;
#endif
#endif /* __EditionUpgradeHelperLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_editionupgradehelper_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_editionupgradehelper_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_editionupgradehelper_0000_0007_v0_0_s_ifspec;

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


