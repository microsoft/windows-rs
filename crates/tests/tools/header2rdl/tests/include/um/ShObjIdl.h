

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

#ifndef __shobjidl_h__
#define __shobjidl_h__

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

#ifndef __IQueryCodePage_FWD_DEFINED__
#define __IQueryCodePage_FWD_DEFINED__
typedef interface IQueryCodePage IQueryCodePage;

#endif 	/* __IQueryCodePage_FWD_DEFINED__ */


#ifndef __IFolderViewOptions_FWD_DEFINED__
#define __IFolderViewOptions_FWD_DEFINED__
typedef interface IFolderViewOptions IFolderViewOptions;

#endif 	/* __IFolderViewOptions_FWD_DEFINED__ */


#ifndef __IShellView3_FWD_DEFINED__
#define __IShellView3_FWD_DEFINED__
typedef interface IShellView3 IShellView3;

#endif 	/* __IShellView3_FWD_DEFINED__ */


#ifndef __ISearchBoxInfo_FWD_DEFINED__
#define __ISearchBoxInfo_FWD_DEFINED__
typedef interface ISearchBoxInfo ISearchBoxInfo;

#endif 	/* __ISearchBoxInfo_FWD_DEFINED__ */


#ifndef __IVisualProperties_FWD_DEFINED__
#define __IVisualProperties_FWD_DEFINED__
typedef interface IVisualProperties IVisualProperties;

#endif 	/* __IVisualProperties_FWD_DEFINED__ */


#ifndef __ICommDlgBrowser3_FWD_DEFINED__
#define __ICommDlgBrowser3_FWD_DEFINED__
typedef interface ICommDlgBrowser3 ICommDlgBrowser3;

#endif 	/* __ICommDlgBrowser3_FWD_DEFINED__ */


#ifndef __IUserAccountChangeCallback_FWD_DEFINED__
#define __IUserAccountChangeCallback_FWD_DEFINED__
typedef interface IUserAccountChangeCallback IUserAccountChangeCallback;

#endif 	/* __IUserAccountChangeCallback_FWD_DEFINED__ */


#ifndef __IStreamAsync_FWD_DEFINED__
#define __IStreamAsync_FWD_DEFINED__
typedef interface IStreamAsync IStreamAsync;

#endif 	/* __IStreamAsync_FWD_DEFINED__ */


#ifndef __IStreamUnbufferedInfo_FWD_DEFINED__
#define __IStreamUnbufferedInfo_FWD_DEFINED__
typedef interface IStreamUnbufferedInfo IStreamUnbufferedInfo;

#endif 	/* __IStreamUnbufferedInfo_FWD_DEFINED__ */


#ifndef __IDragSourceHelper2_FWD_DEFINED__
#define __IDragSourceHelper2_FWD_DEFINED__
typedef interface IDragSourceHelper2 IDragSourceHelper2;

#endif 	/* __IDragSourceHelper2_FWD_DEFINED__ */


#ifndef __IHWEventHandler_FWD_DEFINED__
#define __IHWEventHandler_FWD_DEFINED__
typedef interface IHWEventHandler IHWEventHandler;

#endif 	/* __IHWEventHandler_FWD_DEFINED__ */


#ifndef __IHWEventHandler2_FWD_DEFINED__
#define __IHWEventHandler2_FWD_DEFINED__
typedef interface IHWEventHandler2 IHWEventHandler2;

#endif 	/* __IHWEventHandler2_FWD_DEFINED__ */


#ifndef __IQueryCancelAutoPlay_FWD_DEFINED__
#define __IQueryCancelAutoPlay_FWD_DEFINED__
typedef interface IQueryCancelAutoPlay IQueryCancelAutoPlay;

#endif 	/* __IQueryCancelAutoPlay_FWD_DEFINED__ */


#ifndef __IDynamicHWHandler_FWD_DEFINED__
#define __IDynamicHWHandler_FWD_DEFINED__
typedef interface IDynamicHWHandler IDynamicHWHandler;

#endif 	/* __IDynamicHWHandler_FWD_DEFINED__ */


#ifndef __IUserNotificationCallback_FWD_DEFINED__
#define __IUserNotificationCallback_FWD_DEFINED__
typedef interface IUserNotificationCallback IUserNotificationCallback;

#endif 	/* __IUserNotificationCallback_FWD_DEFINED__ */


#ifndef __IUserNotification2_FWD_DEFINED__
#define __IUserNotification2_FWD_DEFINED__
typedef interface IUserNotification2 IUserNotification2;

#endif 	/* __IUserNotification2_FWD_DEFINED__ */


#ifndef __IDeskBand2_FWD_DEFINED__
#define __IDeskBand2_FWD_DEFINED__
typedef interface IDeskBand2 IDeskBand2;

#endif 	/* __IDeskBand2_FWD_DEFINED__ */


#ifndef __IStartMenuPinnedList_FWD_DEFINED__
#define __IStartMenuPinnedList_FWD_DEFINED__
typedef interface IStartMenuPinnedList IStartMenuPinnedList;

#endif 	/* __IStartMenuPinnedList_FWD_DEFINED__ */


#ifndef __ICDBurn_FWD_DEFINED__
#define __ICDBurn_FWD_DEFINED__
typedef interface ICDBurn ICDBurn;

#endif 	/* __ICDBurn_FWD_DEFINED__ */


#ifndef __IWizardSite_FWD_DEFINED__
#define __IWizardSite_FWD_DEFINED__
typedef interface IWizardSite IWizardSite;

#endif 	/* __IWizardSite_FWD_DEFINED__ */


#ifndef __IWizardExtension_FWD_DEFINED__
#define __IWizardExtension_FWD_DEFINED__
typedef interface IWizardExtension IWizardExtension;

#endif 	/* __IWizardExtension_FWD_DEFINED__ */


#ifndef __IWebWizardExtension_FWD_DEFINED__
#define __IWebWizardExtension_FWD_DEFINED__
typedef interface IWebWizardExtension IWebWizardExtension;

#endif 	/* __IWebWizardExtension_FWD_DEFINED__ */


#ifndef __IPublishingWizard_FWD_DEFINED__
#define __IPublishingWizard_FWD_DEFINED__
typedef interface IPublishingWizard IPublishingWizard;

#endif 	/* __IPublishingWizard_FWD_DEFINED__ */


#ifndef __IFolderViewHost_FWD_DEFINED__
#define __IFolderViewHost_FWD_DEFINED__
typedef interface IFolderViewHost IFolderViewHost;

#endif 	/* __IFolderViewHost_FWD_DEFINED__ */


#ifndef __IAccessibleObject_FWD_DEFINED__
#define __IAccessibleObject_FWD_DEFINED__
typedef interface IAccessibleObject IAccessibleObject;

#endif 	/* __IAccessibleObject_FWD_DEFINED__ */


#ifndef __IResultsFolder_FWD_DEFINED__
#define __IResultsFolder_FWD_DEFINED__
typedef interface IResultsFolder IResultsFolder;

#endif 	/* __IResultsFolder_FWD_DEFINED__ */


#ifndef __IAutoCompleteDropDown_FWD_DEFINED__
#define __IAutoCompleteDropDown_FWD_DEFINED__
typedef interface IAutoCompleteDropDown IAutoCompleteDropDown;

#endif 	/* __IAutoCompleteDropDown_FWD_DEFINED__ */


#ifndef __ICDBurnExt_FWD_DEFINED__
#define __ICDBurnExt_FWD_DEFINED__
typedef interface ICDBurnExt ICDBurnExt;

#endif 	/* __ICDBurnExt_FWD_DEFINED__ */


#ifndef __IEnumReadyCallback_FWD_DEFINED__
#define __IEnumReadyCallback_FWD_DEFINED__
typedef interface IEnumReadyCallback IEnumReadyCallback;

#endif 	/* __IEnumReadyCallback_FWD_DEFINED__ */


#ifndef __IEnumerableView_FWD_DEFINED__
#define __IEnumerableView_FWD_DEFINED__
typedef interface IEnumerableView IEnumerableView;

#endif 	/* __IEnumerableView_FWD_DEFINED__ */


#ifndef __IInsertItem_FWD_DEFINED__
#define __IInsertItem_FWD_DEFINED__
typedef interface IInsertItem IInsertItem;

#endif 	/* __IInsertItem_FWD_DEFINED__ */


#ifndef __IFolderBandPriv_FWD_DEFINED__
#define __IFolderBandPriv_FWD_DEFINED__
typedef interface IFolderBandPriv IFolderBandPriv;

#endif 	/* __IFolderBandPriv_FWD_DEFINED__ */


#ifndef __IImageRecompress_FWD_DEFINED__
#define __IImageRecompress_FWD_DEFINED__
typedef interface IImageRecompress IImageRecompress;

#endif 	/* __IImageRecompress_FWD_DEFINED__ */


#ifndef __IFileDialogControlEvents_FWD_DEFINED__
#define __IFileDialogControlEvents_FWD_DEFINED__
typedef interface IFileDialogControlEvents IFileDialogControlEvents;

#endif 	/* __IFileDialogControlEvents_FWD_DEFINED__ */


#ifndef __IFileDialog2_FWD_DEFINED__
#define __IFileDialog2_FWD_DEFINED__
typedef interface IFileDialog2 IFileDialog2;

#endif 	/* __IFileDialog2_FWD_DEFINED__ */


#ifndef __IApplicationAssociationRegistrationUI_FWD_DEFINED__
#define __IApplicationAssociationRegistrationUI_FWD_DEFINED__
typedef interface IApplicationAssociationRegistrationUI IApplicationAssociationRegistrationUI;

#endif 	/* __IApplicationAssociationRegistrationUI_FWD_DEFINED__ */


#ifndef __IShellRunDll_FWD_DEFINED__
#define __IShellRunDll_FWD_DEFINED__
typedef interface IShellRunDll IShellRunDll;

#endif 	/* __IShellRunDll_FWD_DEFINED__ */


#ifndef __IPreviousVersionsInfo_FWD_DEFINED__
#define __IPreviousVersionsInfo_FWD_DEFINED__
typedef interface IPreviousVersionsInfo IPreviousVersionsInfo;

#endif 	/* __IPreviousVersionsInfo_FWD_DEFINED__ */


#ifndef __IUseToBrowseItem_FWD_DEFINED__
#define __IUseToBrowseItem_FWD_DEFINED__
typedef interface IUseToBrowseItem IUseToBrowseItem;

#endif 	/* __IUseToBrowseItem_FWD_DEFINED__ */


#ifndef __INameSpaceTreeControl2_FWD_DEFINED__
#define __INameSpaceTreeControl2_FWD_DEFINED__
typedef interface INameSpaceTreeControl2 INameSpaceTreeControl2;

#endif 	/* __INameSpaceTreeControl2_FWD_DEFINED__ */


#ifndef __INameSpaceTreeControlEvents_FWD_DEFINED__
#define __INameSpaceTreeControlEvents_FWD_DEFINED__
typedef interface INameSpaceTreeControlEvents INameSpaceTreeControlEvents;

#endif 	/* __INameSpaceTreeControlEvents_FWD_DEFINED__ */


#ifndef __INameSpaceTreeControlDropHandler_FWD_DEFINED__
#define __INameSpaceTreeControlDropHandler_FWD_DEFINED__
typedef interface INameSpaceTreeControlDropHandler INameSpaceTreeControlDropHandler;

#endif 	/* __INameSpaceTreeControlDropHandler_FWD_DEFINED__ */


#ifndef __INameSpaceTreeAccessible_FWD_DEFINED__
#define __INameSpaceTreeAccessible_FWD_DEFINED__
typedef interface INameSpaceTreeAccessible INameSpaceTreeAccessible;

#endif 	/* __INameSpaceTreeAccessible_FWD_DEFINED__ */


#ifndef __INameSpaceTreeControlCustomDraw_FWD_DEFINED__
#define __INameSpaceTreeControlCustomDraw_FWD_DEFINED__
typedef interface INameSpaceTreeControlCustomDraw INameSpaceTreeControlCustomDraw;

#endif 	/* __INameSpaceTreeControlCustomDraw_FWD_DEFINED__ */


#ifndef __ITrayDeskBand_FWD_DEFINED__
#define __ITrayDeskBand_FWD_DEFINED__
typedef interface ITrayDeskBand ITrayDeskBand;

#endif 	/* __ITrayDeskBand_FWD_DEFINED__ */


#ifndef __IBandHost_FWD_DEFINED__
#define __IBandHost_FWD_DEFINED__
typedef interface IBandHost IBandHost;

#endif 	/* __IBandHost_FWD_DEFINED__ */


#ifndef __IMarkupCallback_FWD_DEFINED__
#define __IMarkupCallback_FWD_DEFINED__
typedef interface IMarkupCallback IMarkupCallback;

#endif 	/* __IMarkupCallback_FWD_DEFINED__ */


#ifndef __IControlMarkup_FWD_DEFINED__
#define __IControlMarkup_FWD_DEFINED__
typedef interface IControlMarkup IControlMarkup;

#endif 	/* __IControlMarkup_FWD_DEFINED__ */


#ifndef __IComputerInfoChangeNotify_FWD_DEFINED__
#define __IComputerInfoChangeNotify_FWD_DEFINED__
typedef interface IComputerInfoChangeNotify IComputerInfoChangeNotify;

#endif 	/* __IComputerInfoChangeNotify_FWD_DEFINED__ */


#ifndef __IDesktopGadget_FWD_DEFINED__
#define __IDesktopGadget_FWD_DEFINED__
typedef interface IDesktopGadget IDesktopGadget;

#endif 	/* __IDesktopGadget_FWD_DEFINED__ */


#ifndef __QueryCancelAutoPlay_FWD_DEFINED__
#define __QueryCancelAutoPlay_FWD_DEFINED__

#ifdef __cplusplus
typedef class QueryCancelAutoPlay QueryCancelAutoPlay;
#else
typedef struct QueryCancelAutoPlay QueryCancelAutoPlay;
#endif /* __cplusplus */

#endif 	/* __QueryCancelAutoPlay_FWD_DEFINED__ */


#ifndef __TimeCategorizer_FWD_DEFINED__
#define __TimeCategorizer_FWD_DEFINED__

#ifdef __cplusplus
typedef class TimeCategorizer TimeCategorizer;
#else
typedef struct TimeCategorizer TimeCategorizer;
#endif /* __cplusplus */

#endif 	/* __TimeCategorizer_FWD_DEFINED__ */


#ifndef __AlphabeticalCategorizer_FWD_DEFINED__
#define __AlphabeticalCategorizer_FWD_DEFINED__

#ifdef __cplusplus
typedef class AlphabeticalCategorizer AlphabeticalCategorizer;
#else
typedef struct AlphabeticalCategorizer AlphabeticalCategorizer;
#endif /* __cplusplus */

#endif 	/* __AlphabeticalCategorizer_FWD_DEFINED__ */


#ifndef __MergedCategorizer_FWD_DEFINED__
#define __MergedCategorizer_FWD_DEFINED__

#ifdef __cplusplus
typedef class MergedCategorizer MergedCategorizer;
#else
typedef struct MergedCategorizer MergedCategorizer;
#endif /* __cplusplus */

#endif 	/* __MergedCategorizer_FWD_DEFINED__ */


#ifndef __ImageProperties_FWD_DEFINED__
#define __ImageProperties_FWD_DEFINED__

#ifdef __cplusplus
typedef class ImageProperties ImageProperties;
#else
typedef struct ImageProperties ImageProperties;
#endif /* __cplusplus */

#endif 	/* __ImageProperties_FWD_DEFINED__ */


#ifndef __CDBurn_FWD_DEFINED__
#define __CDBurn_FWD_DEFINED__

#ifdef __cplusplus
typedef class CDBurn CDBurn;
#else
typedef struct CDBurn CDBurn;
#endif /* __cplusplus */

#endif 	/* __CDBurn_FWD_DEFINED__ */


#ifndef __StartMenuPin_FWD_DEFINED__
#define __StartMenuPin_FWD_DEFINED__

#ifdef __cplusplus
typedef class StartMenuPin StartMenuPin;
#else
typedef struct StartMenuPin StartMenuPin;
#endif /* __cplusplus */

#endif 	/* __StartMenuPin_FWD_DEFINED__ */


#ifndef __WebWizardHost_FWD_DEFINED__
#define __WebWizardHost_FWD_DEFINED__

#ifdef __cplusplus
typedef class WebWizardHost WebWizardHost;
#else
typedef struct WebWizardHost WebWizardHost;
#endif /* __cplusplus */

#endif 	/* __WebWizardHost_FWD_DEFINED__ */


#ifndef __PublishDropTarget_FWD_DEFINED__
#define __PublishDropTarget_FWD_DEFINED__

#ifdef __cplusplus
typedef class PublishDropTarget PublishDropTarget;
#else
typedef struct PublishDropTarget PublishDropTarget;
#endif /* __cplusplus */

#endif 	/* __PublishDropTarget_FWD_DEFINED__ */


#ifndef __PublishingWizard_FWD_DEFINED__
#define __PublishingWizard_FWD_DEFINED__

#ifdef __cplusplus
typedef class PublishingWizard PublishingWizard;
#else
typedef struct PublishingWizard PublishingWizard;
#endif /* __cplusplus */

#endif 	/* __PublishingWizard_FWD_DEFINED__ */


#ifndef __InternetPrintOrdering_FWD_DEFINED__
#define __InternetPrintOrdering_FWD_DEFINED__

#ifdef __cplusplus
typedef class InternetPrintOrdering InternetPrintOrdering;
#else
typedef struct InternetPrintOrdering InternetPrintOrdering;
#endif /* __cplusplus */

#endif 	/* __InternetPrintOrdering_FWD_DEFINED__ */


#ifndef __FolderViewHost_FWD_DEFINED__
#define __FolderViewHost_FWD_DEFINED__

#ifdef __cplusplus
typedef class FolderViewHost FolderViewHost;
#else
typedef struct FolderViewHost FolderViewHost;
#endif /* __cplusplus */

#endif 	/* __FolderViewHost_FWD_DEFINED__ */


#ifndef __ExplorerBrowser_FWD_DEFINED__
#define __ExplorerBrowser_FWD_DEFINED__

#ifdef __cplusplus
typedef class ExplorerBrowser ExplorerBrowser;
#else
typedef struct ExplorerBrowser ExplorerBrowser;
#endif /* __cplusplus */

#endif 	/* __ExplorerBrowser_FWD_DEFINED__ */


#ifndef __ImageRecompress_FWD_DEFINED__
#define __ImageRecompress_FWD_DEFINED__

#ifdef __cplusplus
typedef class ImageRecompress ImageRecompress;
#else
typedef struct ImageRecompress ImageRecompress;
#endif /* __cplusplus */

#endif 	/* __ImageRecompress_FWD_DEFINED__ */


#ifndef __TrayBandSiteService_FWD_DEFINED__
#define __TrayBandSiteService_FWD_DEFINED__

#ifdef __cplusplus
typedef class TrayBandSiteService TrayBandSiteService;
#else
typedef struct TrayBandSiteService TrayBandSiteService;
#endif /* __cplusplus */

#endif 	/* __TrayBandSiteService_FWD_DEFINED__ */


#ifndef __TrayDeskBand_FWD_DEFINED__
#define __TrayDeskBand_FWD_DEFINED__

#ifdef __cplusplus
typedef class TrayDeskBand TrayDeskBand;
#else
typedef struct TrayDeskBand TrayDeskBand;
#endif /* __cplusplus */

#endif 	/* __TrayDeskBand_FWD_DEFINED__ */


#ifndef __AttachmentServices_FWD_DEFINED__
#define __AttachmentServices_FWD_DEFINED__

#ifdef __cplusplus
typedef class AttachmentServices AttachmentServices;
#else
typedef struct AttachmentServices AttachmentServices;
#endif /* __cplusplus */

#endif 	/* __AttachmentServices_FWD_DEFINED__ */


#ifndef __DocPropShellExtension_FWD_DEFINED__
#define __DocPropShellExtension_FWD_DEFINED__

#ifdef __cplusplus
typedef class DocPropShellExtension DocPropShellExtension;
#else
typedef struct DocPropShellExtension DocPropShellExtension;
#endif /* __cplusplus */

#endif 	/* __DocPropShellExtension_FWD_DEFINED__ */


#ifndef __FSCopyHandler_FWD_DEFINED__
#define __FSCopyHandler_FWD_DEFINED__

#ifdef __cplusplus
typedef class FSCopyHandler FSCopyHandler;
#else
typedef struct FSCopyHandler FSCopyHandler;
#endif /* __cplusplus */

#endif 	/* __FSCopyHandler_FWD_DEFINED__ */


#ifndef __PreviousVersions_FWD_DEFINED__
#define __PreviousVersions_FWD_DEFINED__

#ifdef __cplusplus
typedef class PreviousVersions PreviousVersions;
#else
typedef struct PreviousVersions PreviousVersions;
#endif /* __cplusplus */

#endif 	/* __PreviousVersions_FWD_DEFINED__ */


#ifndef __NamespaceTreeControl_FWD_DEFINED__
#define __NamespaceTreeControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class NamespaceTreeControl NamespaceTreeControl;
#else
typedef struct NamespaceTreeControl NamespaceTreeControl;
#endif /* __cplusplus */

#endif 	/* __NamespaceTreeControl_FWD_DEFINED__ */


#ifndef __IENamespaceTreeControl_FWD_DEFINED__
#define __IENamespaceTreeControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class IENamespaceTreeControl IENamespaceTreeControl;
#else
typedef struct IENamespaceTreeControl IENamespaceTreeControl;
#endif /* __cplusplus */

#endif 	/* __IENamespaceTreeControl_FWD_DEFINED__ */


#ifndef __ApplicationAssociationRegistrationUI_FWD_DEFINED__
#define __ApplicationAssociationRegistrationUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class ApplicationAssociationRegistrationUI ApplicationAssociationRegistrationUI;
#else
typedef struct ApplicationAssociationRegistrationUI ApplicationAssociationRegistrationUI;
#endif /* __cplusplus */

#endif 	/* __ApplicationAssociationRegistrationUI_FWD_DEFINED__ */


#ifndef __DesktopGadget_FWD_DEFINED__
#define __DesktopGadget_FWD_DEFINED__

#ifdef __cplusplus
typedef class DesktopGadget DesktopGadget;
#else
typedef struct DesktopGadget DesktopGadget;
#endif /* __cplusplus */

#endif 	/* __DesktopGadget_FWD_DEFINED__ */


#ifndef __AccessibilityDockingService_FWD_DEFINED__
#define __AccessibilityDockingService_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccessibilityDockingService AccessibilityDockingService;
#else
typedef struct AccessibilityDockingService AccessibilityDockingService;
#endif /* __cplusplus */

#endif 	/* __AccessibilityDockingService_FWD_DEFINED__ */


#ifndef __ExecuteFolder_FWD_DEFINED__
#define __ExecuteFolder_FWD_DEFINED__

#ifdef __cplusplus
typedef class ExecuteFolder ExecuteFolder;
#else
typedef struct ExecuteFolder ExecuteFolder;
#endif /* __cplusplus */

#endif 	/* __ExecuteFolder_FWD_DEFINED__ */


#ifndef __VirtualDesktopManager_FWD_DEFINED__
#define __VirtualDesktopManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class VirtualDesktopManager VirtualDesktopManager;
#else
typedef struct VirtualDesktopManager VirtualDesktopManager;
#endif /* __cplusplus */

#endif 	/* __VirtualDesktopManager_FWD_DEFINED__ */


#ifndef __StorageProviderBanners_FWD_DEFINED__
#define __StorageProviderBanners_FWD_DEFINED__

#ifdef __cplusplus
typedef class StorageProviderBanners StorageProviderBanners;
#else
typedef struct StorageProviderBanners StorageProviderBanners;
#endif /* __cplusplus */

#endif 	/* __StorageProviderBanners_FWD_DEFINED__ */


#ifndef __IAccessibilityDockingServiceCallback_FWD_DEFINED__
#define __IAccessibilityDockingServiceCallback_FWD_DEFINED__
typedef interface IAccessibilityDockingServiceCallback IAccessibilityDockingServiceCallback;

#endif 	/* __IAccessibilityDockingServiceCallback_FWD_DEFINED__ */


#ifndef __IAccessibilityDockingService_FWD_DEFINED__
#define __IAccessibilityDockingService_FWD_DEFINED__
typedef interface IAccessibilityDockingService IAccessibilityDockingService;

#endif 	/* __IAccessibilityDockingService_FWD_DEFINED__ */


#ifndef __IStorageProviderBanners_FWD_DEFINED__
#define __IStorageProviderBanners_FWD_DEFINED__
typedef interface IStorageProviderBanners IStorageProviderBanners;

#endif 	/* __IStorageProviderBanners_FWD_DEFINED__ */


#ifndef __IStorageProviderCopyHook_FWD_DEFINED__
#define __IStorageProviderCopyHook_FWD_DEFINED__
typedef interface IStorageProviderCopyHook IStorageProviderCopyHook;

#endif 	/* __IStorageProviderCopyHook_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"
#include "oaidl.h"
#include "docobj.h"
#include "shtypes.h"
#include "comcat.h"
#include "propidl.h"
#include "prsht.h"
#include "propsys.h"
#include "ObjectArray.h"
#include "shobjidl_core.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shobjidl_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#ifndef _MSC_EXTENSIONS
#pragma warning(disable:4309) /* truncation of constant value */
#endif
#endif
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <sherrors.h>


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0000_v0_0_s_ifspec;

#ifndef __IQueryCodePage_INTERFACE_DEFINED__
#define __IQueryCodePage_INTERFACE_DEFINED__

/* interface IQueryCodePage */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IQueryCodePage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C7B236CE-EE80-11D0-985F-006008059382")
    IQueryCodePage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCodePage( 
            /* [annotation][out] */ 
            _Out_  UINT *puiCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCodePage( 
            /* [annotation][in] */ 
            _In_  UINT uiCodePage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryCodePageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IQueryCodePage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IQueryCodePage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IQueryCodePage * This);
        
        DECLSPEC_XFGVIRT(IQueryCodePage, GetCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetCodePage )( 
            IQueryCodePage * This,
            /* [annotation][out] */ 
            _Out_  UINT *puiCodePage);
        
        DECLSPEC_XFGVIRT(IQueryCodePage, SetCodePage)
        HRESULT ( STDMETHODCALLTYPE *SetCodePage )( 
            IQueryCodePage * This,
            /* [annotation][in] */ 
            _In_  UINT uiCodePage);
        
        END_INTERFACE
    } IQueryCodePageVtbl;

    interface IQueryCodePage
    {
        CONST_VTBL struct IQueryCodePageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryCodePage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryCodePage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryCodePage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryCodePage_GetCodePage(This,puiCodePage)	\
    ( (This)->lpVtbl -> GetCodePage(This,puiCodePage) ) 

#define IQueryCodePage_SetCodePage(This,uiCodePage)	\
    ( (This)->lpVtbl -> SetCodePage(This,uiCodePage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryCodePage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0001 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum SYNC_ENGINE_STATE_FLAGS
    {
        SESF_NONE	= 0,
        SESF_SERVICE_QUOTA_NEARING_LIMIT	= 0x1,
        SESF_SERVICE_QUOTA_EXCEEDED_LIMIT	= 0x2,
        SESF_AUTHENTICATION_ERROR	= 0x4,
        SESF_PAUSED_DUE_TO_METERED_NETWORK	= 0x8,
        SESF_PAUSED_DUE_TO_DISK_SPACE_FULL	= 0x10,
        SESF_PAUSED_DUE_TO_CLIENT_POLICY	= 0x20,
        SESF_PAUSED_DUE_TO_SERVICE_POLICY	= 0x40,
        SESF_SERVICE_UNAVAILABLE	= 0x80,
        SESF_PAUSED_DUE_TO_USER_REQUEST	= 0x100,
        SESF_ALL_FLAGS	= ( ( ( ( ( ( ( ( ( SESF_NONE | SESF_SERVICE_QUOTA_NEARING_LIMIT )  | SESF_SERVICE_QUOTA_EXCEEDED_LIMIT )  | SESF_AUTHENTICATION_ERROR )  | SESF_PAUSED_DUE_TO_METERED_NETWORK )  | SESF_PAUSED_DUE_TO_DISK_SPACE_FULL )  | SESF_PAUSED_DUE_TO_CLIENT_POLICY )  | SESF_PAUSED_DUE_TO_SERVICE_POLICY )  | SESF_SERVICE_UNAVAILABLE )  | SESF_PAUSED_DUE_TO_USER_REQUEST ) 
    } 	SYNC_ENGINE_STATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(SYNC_ENGINE_STATE_FLAGS)
typedef char *LPVIEWSETTINGS;



extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0001_v0_0_s_ifspec;

#ifndef __IFolderViewOptions_INTERFACE_DEFINED__
#define __IFolderViewOptions_INTERFACE_DEFINED__

/* interface IFolderViewOptions */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum FOLDERVIEWOPTIONS
    {
        FVO_DEFAULT	= 0,
        FVO_VISTALAYOUT	= 0x1,
        FVO_CUSTOMPOSITION	= 0x2,
        FVO_CUSTOMORDERING	= 0x4,
        FVO_SUPPORTHYPERLINKS	= 0x8,
        FVO_NOANIMATIONS	= 0x10,
        FVO_NOSCROLLTIPS	= 0x20
    } 	FOLDERVIEWOPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(FOLDERVIEWOPTIONS)

EXTERN_C const IID IID_IFolderViewOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3cc974d2-b302-4d36-ad3e-06d93f695d3f")
    IFolderViewOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFolderViewOptions( 
            /* [in] */ FOLDERVIEWOPTIONS fvoMask,
            /* [in] */ FOLDERVIEWOPTIONS fvoFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFolderViewOptions( 
            /* [out] */ __RPC__out FOLDERVIEWOPTIONS *pfvoFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderViewOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFolderViewOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFolderViewOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFolderViewOptions * This);
        
        DECLSPEC_XFGVIRT(IFolderViewOptions, SetFolderViewOptions)
        HRESULT ( STDMETHODCALLTYPE *SetFolderViewOptions )( 
            __RPC__in IFolderViewOptions * This,
            /* [in] */ FOLDERVIEWOPTIONS fvoMask,
            /* [in] */ FOLDERVIEWOPTIONS fvoFlags);
        
        DECLSPEC_XFGVIRT(IFolderViewOptions, GetFolderViewOptions)
        HRESULT ( STDMETHODCALLTYPE *GetFolderViewOptions )( 
            __RPC__in IFolderViewOptions * This,
            /* [out] */ __RPC__out FOLDERVIEWOPTIONS *pfvoFlags);
        
        END_INTERFACE
    } IFolderViewOptionsVtbl;

    interface IFolderViewOptions
    {
        CONST_VTBL struct IFolderViewOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderViewOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderViewOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderViewOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderViewOptions_SetFolderViewOptions(This,fvoMask,fvoFlags)	\
    ( (This)->lpVtbl -> SetFolderViewOptions(This,fvoMask,fvoFlags) ) 

#define IFolderViewOptions_GetFolderViewOptions(This,pfvoFlags)	\
    ( (This)->lpVtbl -> GetFolderViewOptions(This,pfvoFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderViewOptions_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0002 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0002_v0_0_s_ifspec;

#ifndef __IShellView3_INTERFACE_DEFINED__
#define __IShellView3_INTERFACE_DEFINED__

/* interface IShellView3 */
/* [unique][object][uuid] */ 

/* [v1_enum] */ 
enum _SV3CVW3_FLAGS
    {
        SV3CVW3_DEFAULT	= 0,
        SV3CVW3_NONINTERACTIVE	= 0x1,
        SV3CVW3_FORCEVIEWMODE	= 0x2,
        SV3CVW3_FORCEFOLDERFLAGS	= 0x4
    } ;
typedef DWORD SV3CVW3_FLAGS;


EXTERN_C const IID IID_IShellView3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ec39fa88-f8af-41c5-8421-38bed28f4673")
    IShellView3 : public IShellView2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateViewWindow3( 
            /* [in] */ __RPC__in_opt IShellBrowser *psbOwner,
            /* [unique][in] */ __RPC__in_opt IShellView *psvPrev,
            /* [in] */ SV3CVW3_FLAGS dwViewFlags,
            /* [in] */ FOLDERFLAGS dwMask,
            /* [in] */ FOLDERFLAGS dwFlags,
            /* [in] */ FOLDERVIEWMODE fvMode,
            /* [unique][in] */ __RPC__in_opt const SHELLVIEWID *pvid,
            /* [in] */ __RPC__in const RECT *prcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwndView) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellView3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellView3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellView3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellView3 * This);
        
        DECLSPEC_XFGVIRT(IOleWindow, GetWindow)
        /* [input_sync] */ HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in IShellView3 * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IOleWindow, ContextSensitiveHelp)
        HRESULT ( STDMETHODCALLTYPE *ContextSensitiveHelp )( 
            __RPC__in IShellView3 * This,
            /* [in] */ BOOL fEnterMode);
        
        DECLSPEC_XFGVIRT(IShellView, TranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *TranslateAccelerator )( 
            __RPC__in IShellView3 * This,
            /* [in] */ __RPC__in MSG *pmsg);
        
        DECLSPEC_XFGVIRT(IShellView, EnableModeless)
        HRESULT ( STDMETHODCALLTYPE *EnableModeless )( 
            __RPC__in IShellView3 * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellView, UIActivate)
        HRESULT ( STDMETHODCALLTYPE *UIActivate )( 
            __RPC__in IShellView3 * This,
            /* [in] */ UINT uState);
        
        DECLSPEC_XFGVIRT(IShellView, Refresh)
        HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IShellView3 * This);
        
        DECLSPEC_XFGVIRT(IShellView, CreateViewWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow )( 
            __RPC__in IShellView3 * This,
            /* [unique][in] */ __RPC__in_opt IShellView *psvPrevious,
            /* [in] */ __RPC__in LPCFOLDERSETTINGS pfs,
            /* [in] */ __RPC__in_opt IShellBrowser *psb,
            /* [in] */ __RPC__in RECT *prcView,
            /* [out] */ __RPC__deref_out_opt HWND *phWnd);
        
        DECLSPEC_XFGVIRT(IShellView, DestroyViewWindow)
        HRESULT ( STDMETHODCALLTYPE *DestroyViewWindow )( 
            __RPC__in IShellView3 * This);
        
        DECLSPEC_XFGVIRT(IShellView, GetCurrentInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentInfo )( 
            __RPC__in IShellView3 * This,
            /* [out] */ __RPC__out LPFOLDERSETTINGS pfs);
        
        DECLSPEC_XFGVIRT(IShellView, AddPropertySheetPages)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *AddPropertySheetPages )( 
            IShellView3 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwReserved,
            /* [annotation][in] */ 
            _In_  LPFNSVADDPROPSHEETPAGE pfn,
            /* [annotation][in] */ 
            _In_  LPARAM lparam);
        
        DECLSPEC_XFGVIRT(IShellView, SaveViewState)
        HRESULT ( STDMETHODCALLTYPE *SaveViewState )( 
            __RPC__in IShellView3 * This);
        
        DECLSPEC_XFGVIRT(IShellView, SelectItem)
        HRESULT ( STDMETHODCALLTYPE *SelectItem )( 
            __RPC__in IShellView3 * This,
            /* [unique][in] */ __RPC__in_opt PCUITEMID_CHILD pidlItem,
            /* [in] */ SVSIF uFlags);
        
        DECLSPEC_XFGVIRT(IShellView, GetItemObject)
        HRESULT ( STDMETHODCALLTYPE *GetItemObject )( 
            __RPC__in IShellView3 * This,
            /* [in] */ UINT uItem,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IShellView2, GetView)
        HRESULT ( STDMETHODCALLTYPE *GetView )( 
            __RPC__in IShellView3 * This,
            /* [out][in] */ __RPC__inout SHELLVIEWID *pvid,
            /* [in] */ ULONG uView);
        
        DECLSPEC_XFGVIRT(IShellView2, CreateViewWindow2)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow2 )( 
            __RPC__in IShellView3 * This,
            /* [in] */ __RPC__in LPSV2CVW2_PARAMS lpParams);
        
        DECLSPEC_XFGVIRT(IShellView2, HandleRename)
        HRESULT ( STDMETHODCALLTYPE *HandleRename )( 
            __RPC__in IShellView3 * This,
            /* [unique][in] */ __RPC__in_opt PCUITEMID_CHILD pidlNew);
        
        DECLSPEC_XFGVIRT(IShellView2, SelectAndPositionItem)
        HRESULT ( STDMETHODCALLTYPE *SelectAndPositionItem )( 
            __RPC__in IShellView3 * This,
            /* [unique][in] */ __RPC__in_opt PCUITEMID_CHILD pidlItem,
            /* [in] */ UINT uFlags,
            /* [unique][in] */ __RPC__in_opt POINT *ppt);
        
        DECLSPEC_XFGVIRT(IShellView3, CreateViewWindow3)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow3 )( 
            __RPC__in IShellView3 * This,
            /* [in] */ __RPC__in_opt IShellBrowser *psbOwner,
            /* [unique][in] */ __RPC__in_opt IShellView *psvPrev,
            /* [in] */ SV3CVW3_FLAGS dwViewFlags,
            /* [in] */ FOLDERFLAGS dwMask,
            /* [in] */ FOLDERFLAGS dwFlags,
            /* [in] */ FOLDERVIEWMODE fvMode,
            /* [unique][in] */ __RPC__in_opt const SHELLVIEWID *pvid,
            /* [in] */ __RPC__in const RECT *prcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwndView);
        
        END_INTERFACE
    } IShellView3Vtbl;

    interface IShellView3
    {
        CONST_VTBL struct IShellView3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellView3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellView3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellView3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellView3_GetWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,phwnd) ) 

#define IShellView3_ContextSensitiveHelp(This,fEnterMode)	\
    ( (This)->lpVtbl -> ContextSensitiveHelp(This,fEnterMode) ) 


#define IShellView3_TranslateAccelerator(This,pmsg)	\
    ( (This)->lpVtbl -> TranslateAccelerator(This,pmsg) ) 

#define IShellView3_EnableModeless(This,fEnable)	\
    ( (This)->lpVtbl -> EnableModeless(This,fEnable) ) 

#define IShellView3_UIActivate(This,uState)	\
    ( (This)->lpVtbl -> UIActivate(This,uState) ) 

#define IShellView3_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IShellView3_CreateViewWindow(This,psvPrevious,pfs,psb,prcView,phWnd)	\
    ( (This)->lpVtbl -> CreateViewWindow(This,psvPrevious,pfs,psb,prcView,phWnd) ) 

#define IShellView3_DestroyViewWindow(This)	\
    ( (This)->lpVtbl -> DestroyViewWindow(This) ) 

#define IShellView3_GetCurrentInfo(This,pfs)	\
    ( (This)->lpVtbl -> GetCurrentInfo(This,pfs) ) 

#define IShellView3_AddPropertySheetPages(This,dwReserved,pfn,lparam)	\
    ( (This)->lpVtbl -> AddPropertySheetPages(This,dwReserved,pfn,lparam) ) 

#define IShellView3_SaveViewState(This)	\
    ( (This)->lpVtbl -> SaveViewState(This) ) 

#define IShellView3_SelectItem(This,pidlItem,uFlags)	\
    ( (This)->lpVtbl -> SelectItem(This,pidlItem,uFlags) ) 

#define IShellView3_GetItemObject(This,uItem,riid,ppv)	\
    ( (This)->lpVtbl -> GetItemObject(This,uItem,riid,ppv) ) 


#define IShellView3_GetView(This,pvid,uView)	\
    ( (This)->lpVtbl -> GetView(This,pvid,uView) ) 

#define IShellView3_CreateViewWindow2(This,lpParams)	\
    ( (This)->lpVtbl -> CreateViewWindow2(This,lpParams) ) 

#define IShellView3_HandleRename(This,pidlNew)	\
    ( (This)->lpVtbl -> HandleRename(This,pidlNew) ) 

#define IShellView3_SelectAndPositionItem(This,pidlItem,uFlags,ppt)	\
    ( (This)->lpVtbl -> SelectAndPositionItem(This,pidlItem,uFlags,ppt) ) 


#define IShellView3_CreateViewWindow3(This,psbOwner,psvPrev,dwViewFlags,dwMask,dwFlags,fvMode,pvid,prcView,phwndView)	\
    ( (This)->lpVtbl -> CreateViewWindow3(This,psbOwner,psvPrev,dwViewFlags,dwMask,dwFlags,fvMode,pvid,prcView,phwndView) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellView3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0003 */
/* [local] */ 

#endif  // NTDDI_VISTA
#if (NTDDI_VERSION >= NTDDI_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0003_v0_0_s_ifspec;

#ifndef __ISearchBoxInfo_INTERFACE_DEFINED__
#define __ISearchBoxInfo_INTERFACE_DEFINED__

/* interface ISearchBoxInfo */
/* [object][uuid] */ 


EXTERN_C const IID IID_ISearchBoxInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6af6e03f-d664-4ef4-9626-f7e0ed36755e")
    ISearchBoxInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCondition( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppsz) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISearchBoxInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISearchBoxInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISearchBoxInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISearchBoxInfo * This);
        
        DECLSPEC_XFGVIRT(ISearchBoxInfo, GetCondition)
        HRESULT ( STDMETHODCALLTYPE *GetCondition )( 
            __RPC__in ISearchBoxInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(ISearchBoxInfo, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ISearchBoxInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppsz);
        
        END_INTERFACE
    } ISearchBoxInfoVtbl;

    interface ISearchBoxInfo
    {
        CONST_VTBL struct ISearchBoxInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISearchBoxInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISearchBoxInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISearchBoxInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISearchBoxInfo_GetCondition(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetCondition(This,riid,ppv) ) 

#define ISearchBoxInfo_GetText(This,ppsz)	\
    ( (This)->lpVtbl -> GetText(This,ppsz) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISearchBoxInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0004 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0004_v0_0_s_ifspec;

#ifndef __IVisualProperties_INTERFACE_DEFINED__
#define __IVisualProperties_INTERFACE_DEFINED__

/* interface IVisualProperties */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum VPWATERMARKFLAGS
    {
        VPWF_DEFAULT	= 0,
        VPWF_ALPHABLEND	= 0x1
    } 	VPWATERMARKFLAGS;

DEFINE_ENUM_FLAG_OPERATORS(VPWATERMARKFLAGS)
typedef /* [v1_enum] */ 
enum VPCOLORFLAGS
    {
        VPCF_TEXT	= 1,
        VPCF_BACKGROUND	= 2,
        VPCF_SORTCOLUMN	= 3,
        VPCF_SUBTEXT	= 4,
        VPCF_TEXTBACKGROUND	= 5
    } 	VPCOLORFLAGS;


EXTERN_C const IID IID_IVisualProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e693cf68-d967-4112-8763-99172aee5e5a")
    IVisualProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetWatermark( 
            /* [unique][in] */ __RPC__in_opt HBITMAP hbmp,
            /* [in] */ VPWATERMARKFLAGS vpwf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColor( 
            /* [in] */ VPCOLORFLAGS vpcf,
            /* [in] */ COLORREF cr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColor( 
            /* [in] */ VPCOLORFLAGS vpcf,
            /* [out] */ __RPC__out COLORREF *pcr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetItemHeight( 
            /* [in] */ int cyItemInPixels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemHeight( 
            /* [out] */ __RPC__out int *cyItemInPixels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFont( 
            /* [in] */ __RPC__in const LOGFONTW *plf,
            /* [in] */ BOOL bRedraw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFont( 
            /* [out] */ __RPC__out LOGFONTW *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTheme( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszSubAppName,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszSubIdList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVisualPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVisualProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVisualProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVisualProperties * This);
        
        DECLSPEC_XFGVIRT(IVisualProperties, SetWatermark)
        HRESULT ( STDMETHODCALLTYPE *SetWatermark )( 
            __RPC__in IVisualProperties * This,
            /* [unique][in] */ __RPC__in_opt HBITMAP hbmp,
            /* [in] */ VPWATERMARKFLAGS vpwf);
        
        DECLSPEC_XFGVIRT(IVisualProperties, SetColor)
        HRESULT ( STDMETHODCALLTYPE *SetColor )( 
            __RPC__in IVisualProperties * This,
            /* [in] */ VPCOLORFLAGS vpcf,
            /* [in] */ COLORREF cr);
        
        DECLSPEC_XFGVIRT(IVisualProperties, GetColor)
        HRESULT ( STDMETHODCALLTYPE *GetColor )( 
            __RPC__in IVisualProperties * This,
            /* [in] */ VPCOLORFLAGS vpcf,
            /* [out] */ __RPC__out COLORREF *pcr);
        
        DECLSPEC_XFGVIRT(IVisualProperties, SetItemHeight)
        HRESULT ( STDMETHODCALLTYPE *SetItemHeight )( 
            __RPC__in IVisualProperties * This,
            /* [in] */ int cyItemInPixels);
        
        DECLSPEC_XFGVIRT(IVisualProperties, GetItemHeight)
        HRESULT ( STDMETHODCALLTYPE *GetItemHeight )( 
            __RPC__in IVisualProperties * This,
            /* [out] */ __RPC__out int *cyItemInPixels);
        
        DECLSPEC_XFGVIRT(IVisualProperties, SetFont)
        HRESULT ( STDMETHODCALLTYPE *SetFont )( 
            __RPC__in IVisualProperties * This,
            /* [in] */ __RPC__in const LOGFONTW *plf,
            /* [in] */ BOOL bRedraw);
        
        DECLSPEC_XFGVIRT(IVisualProperties, GetFont)
        HRESULT ( STDMETHODCALLTYPE *GetFont )( 
            __RPC__in IVisualProperties * This,
            /* [out] */ __RPC__out LOGFONTW *plf);
        
        DECLSPEC_XFGVIRT(IVisualProperties, SetTheme)
        HRESULT ( STDMETHODCALLTYPE *SetTheme )( 
            __RPC__in IVisualProperties * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszSubAppName,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszSubIdList);
        
        END_INTERFACE
    } IVisualPropertiesVtbl;

    interface IVisualProperties
    {
        CONST_VTBL struct IVisualPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVisualProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVisualProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVisualProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVisualProperties_SetWatermark(This,hbmp,vpwf)	\
    ( (This)->lpVtbl -> SetWatermark(This,hbmp,vpwf) ) 

#define IVisualProperties_SetColor(This,vpcf,cr)	\
    ( (This)->lpVtbl -> SetColor(This,vpcf,cr) ) 

#define IVisualProperties_GetColor(This,vpcf,pcr)	\
    ( (This)->lpVtbl -> GetColor(This,vpcf,pcr) ) 

#define IVisualProperties_SetItemHeight(This,cyItemInPixels)	\
    ( (This)->lpVtbl -> SetItemHeight(This,cyItemInPixels) ) 

#define IVisualProperties_GetItemHeight(This,cyItemInPixels)	\
    ( (This)->lpVtbl -> GetItemHeight(This,cyItemInPixels) ) 

#define IVisualProperties_SetFont(This,plf,bRedraw)	\
    ( (This)->lpVtbl -> SetFont(This,plf,bRedraw) ) 

#define IVisualProperties_GetFont(This,plf)	\
    ( (This)->lpVtbl -> GetFont(This,plf) ) 

#define IVisualProperties_SetTheme(This,pszSubAppName,pszSubIdList)	\
    ( (This)->lpVtbl -> SetTheme(This,pszSubAppName,pszSubIdList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVisualProperties_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0005 */
/* [local] */ 

#endif  // _WIN32_IE_IE70
#if (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0005_v0_0_s_ifspec;

#ifndef __ICommDlgBrowser3_INTERFACE_DEFINED__
#define __ICommDlgBrowser3_INTERFACE_DEFINED__

/* interface ICommDlgBrowser3 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ICommDlgBrowser3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8ad25a1-3294-41ee-8165-71174bd01c57")
    ICommDlgBrowser3 : public ICommDlgBrowser2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnColumnClicked( 
            /* [in] */ __RPC__in_opt IShellView *ppshv,
            /* [in] */ int iColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentFilter( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchFileSpec) LPWSTR pszFileSpec,
            /* [in] */ int cchFileSpec) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPreViewCreated( 
            /* [in] */ __RPC__in_opt IShellView *ppshv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICommDlgBrowser3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICommDlgBrowser3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICommDlgBrowser3 * This);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser, OnDefaultCommand)
        HRESULT ( STDMETHODCALLTYPE *OnDefaultCommand )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser, OnStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnStateChange )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv,
            /* [in] */ ULONG uChange);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser, IncludeObject)
        HRESULT ( STDMETHODCALLTYPE *IncludeObject )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [unique][in] */ __RPC__in_opt IShellView *ppshv,
            /* [in] */ __RPC__in PCUITEMID_CHILD pidl);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser2, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv,
            /* [in] */ DWORD dwNotifyType);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser2, GetDefaultMenuText)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultMenuText )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchMax) LPWSTR pszText,
            /* [in] */ int cchMax);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser2, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser3, OnColumnClicked)
        HRESULT ( STDMETHODCALLTYPE *OnColumnClicked )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv,
            /* [in] */ int iColumn);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser3, GetCurrentFilter)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentFilter )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchFileSpec) LPWSTR pszFileSpec,
            /* [in] */ int cchFileSpec);
        
        DECLSPEC_XFGVIRT(ICommDlgBrowser3, OnPreViewCreated)
        HRESULT ( STDMETHODCALLTYPE *OnPreViewCreated )( 
            __RPC__in ICommDlgBrowser3 * This,
            /* [in] */ __RPC__in_opt IShellView *ppshv);
        
        END_INTERFACE
    } ICommDlgBrowser3Vtbl;

    interface ICommDlgBrowser3
    {
        CONST_VTBL struct ICommDlgBrowser3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICommDlgBrowser3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICommDlgBrowser3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICommDlgBrowser3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICommDlgBrowser3_OnDefaultCommand(This,ppshv)	\
    ( (This)->lpVtbl -> OnDefaultCommand(This,ppshv) ) 

#define ICommDlgBrowser3_OnStateChange(This,ppshv,uChange)	\
    ( (This)->lpVtbl -> OnStateChange(This,ppshv,uChange) ) 

#define ICommDlgBrowser3_IncludeObject(This,ppshv,pidl)	\
    ( (This)->lpVtbl -> IncludeObject(This,ppshv,pidl) ) 


#define ICommDlgBrowser3_Notify(This,ppshv,dwNotifyType)	\
    ( (This)->lpVtbl -> Notify(This,ppshv,dwNotifyType) ) 

#define ICommDlgBrowser3_GetDefaultMenuText(This,ppshv,pszText,cchMax)	\
    ( (This)->lpVtbl -> GetDefaultMenuText(This,ppshv,pszText,cchMax) ) 

#define ICommDlgBrowser3_GetViewFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,pdwFlags) ) 


#define ICommDlgBrowser3_OnColumnClicked(This,ppshv,iColumn)	\
    ( (This)->lpVtbl -> OnColumnClicked(This,ppshv,iColumn) ) 

#define ICommDlgBrowser3_GetCurrentFilter(This,pszFileSpec,cchFileSpec)	\
    ( (This)->lpVtbl -> GetCurrentFilter(This,pszFileSpec,cchFileSpec) ) 

#define ICommDlgBrowser3_OnPreViewCreated(This,ppshv)	\
    ( (This)->lpVtbl -> OnPreViewCreated(This,ppshv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICommDlgBrowser3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0006 */
/* [local] */ 

#endif  // (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0006_v0_0_s_ifspec;

#ifndef __IUserAccountChangeCallback_INTERFACE_DEFINED__
#define __IUserAccountChangeCallback_INTERFACE_DEFINED__

/* interface IUserAccountChangeCallback */
/* [version][uuid][object] */ 


EXTERN_C const IID IID_IUserAccountChangeCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a561e69a-b4b8-4113-91a5-64c6bcca3430")
    IUserAccountChangeCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnPictureChange( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserAccountChangeCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUserAccountChangeCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUserAccountChangeCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUserAccountChangeCallback * This);
        
        DECLSPEC_XFGVIRT(IUserAccountChangeCallback, OnPictureChange)
        HRESULT ( STDMETHODCALLTYPE *OnPictureChange )( 
            __RPC__in IUserAccountChangeCallback * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszUserName);
        
        END_INTERFACE
    } IUserAccountChangeCallbackVtbl;

    interface IUserAccountChangeCallback
    {
        CONST_VTBL struct IUserAccountChangeCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserAccountChangeCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserAccountChangeCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserAccountChangeCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserAccountChangeCallback_OnPictureChange(This,pszUserName)	\
    ( (This)->lpVtbl -> OnPictureChange(This,pszUserName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserAccountChangeCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0007 */
/* [local] */ 

#ifdef MIDL_PASS
typedef struct _OVERLAPPED
    {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    union 
        {
        struct 
            {
            DWORD Offset;
            DWORD OffsetHigh;
            } 	;
        PVOID Pointer;
        } 	;
    HANDLE hEvent;
    } 	OVERLAPPED;

typedef struct _OVERLAPPED *LPOVERLAPPED;

#endif // MIDL_PASS


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0007_v0_0_s_ifspec;

#ifndef __IStreamAsync_INTERFACE_DEFINED__
#define __IStreamAsync_INTERFACE_DEFINED__

/* interface IStreamAsync */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IStreamAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fe0b6665-e0ca-49b9-a178-2b5cb48d92a5")
    IStreamAsync : public IStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReadAsync( 
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_opt_ _Deref_out_range_(0, cb)  LPDWORD pcbRead,
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAsync( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *lpBuffer,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_opt_ _Deref_out_range_(0, cb)  LPDWORD pcbWritten,
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OverlappedResult( 
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped,
            /* [annotation][out] */ 
            _Out_  LPDWORD lpNumberOfBytesTransferred,
            /* [annotation][in] */ 
            _In_  BOOL bWait) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelIo( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStreamAsync * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStreamAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStreamAsync * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IStreamAsync * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IStreamAsync * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IStreamAsync * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            IStreamAsync * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStreamAsync * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            IStreamAsync * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            IStreamAsync * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            IStreamAsync * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            IStreamAsync * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            IStreamAsync * This,
            /* [out] */ STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IStreamAsync * This,
            /* [out] */ IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStreamAsync, ReadAsync)
        HRESULT ( STDMETHODCALLTYPE *ReadAsync )( 
            IStreamAsync * This,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_opt_ _Deref_out_range_(0, cb)  LPDWORD pcbRead,
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped);
        
        DECLSPEC_XFGVIRT(IStreamAsync, WriteAsync)
        HRESULT ( STDMETHODCALLTYPE *WriteAsync )( 
            IStreamAsync * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *lpBuffer,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_opt_ _Deref_out_range_(0, cb)  LPDWORD pcbWritten,
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped);
        
        DECLSPEC_XFGVIRT(IStreamAsync, OverlappedResult)
        HRESULT ( STDMETHODCALLTYPE *OverlappedResult )( 
            IStreamAsync * This,
            /* [annotation][in] */ 
            _In_  LPOVERLAPPED lpOverlapped,
            /* [annotation][out] */ 
            _Out_  LPDWORD lpNumberOfBytesTransferred,
            /* [annotation][in] */ 
            _In_  BOOL bWait);
        
        DECLSPEC_XFGVIRT(IStreamAsync, CancelIo)
        HRESULT ( STDMETHODCALLTYPE *CancelIo )( 
            IStreamAsync * This);
        
        END_INTERFACE
    } IStreamAsyncVtbl;

    interface IStreamAsync
    {
        CONST_VTBL struct IStreamAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamAsync_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IStreamAsync_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IStreamAsync_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IStreamAsync_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IStreamAsync_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IStreamAsync_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStreamAsync_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStreamAsync_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamAsync_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamAsync_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IStreamAsync_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IStreamAsync_ReadAsync(This,pv,cb,pcbRead,lpOverlapped)	\
    ( (This)->lpVtbl -> ReadAsync(This,pv,cb,pcbRead,lpOverlapped) ) 

#define IStreamAsync_WriteAsync(This,lpBuffer,cb,pcbWritten,lpOverlapped)	\
    ( (This)->lpVtbl -> WriteAsync(This,lpBuffer,cb,pcbWritten,lpOverlapped) ) 

#define IStreamAsync_OverlappedResult(This,lpOverlapped,lpNumberOfBytesTransferred,bWait)	\
    ( (This)->lpVtbl -> OverlappedResult(This,lpOverlapped,lpNumberOfBytesTransferred,bWait) ) 

#define IStreamAsync_CancelIo(This)	\
    ( (This)->lpVtbl -> CancelIo(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamAsync_INTERFACE_DEFINED__ */


#ifndef __IStreamUnbufferedInfo_INTERFACE_DEFINED__
#define __IStreamUnbufferedInfo_INTERFACE_DEFINED__

/* interface IStreamUnbufferedInfo */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IStreamUnbufferedInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8a68fdda-1fdc-4c20-8ceb-416643b5a625")
    IStreamUnbufferedInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSectorSize( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcbSectorSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamUnbufferedInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStreamUnbufferedInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStreamUnbufferedInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStreamUnbufferedInfo * This);
        
        DECLSPEC_XFGVIRT(IStreamUnbufferedInfo, GetSectorSize)
        HRESULT ( STDMETHODCALLTYPE *GetSectorSize )( 
            IStreamUnbufferedInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbSectorSize);
        
        END_INTERFACE
    } IStreamUnbufferedInfoVtbl;

    interface IStreamUnbufferedInfo
    {
        CONST_VTBL struct IStreamUnbufferedInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamUnbufferedInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamUnbufferedInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamUnbufferedInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamUnbufferedInfo_GetSectorSize(This,pcbSectorSize)	\
    ( (This)->lpVtbl -> GetSectorSize(This,pcbSectorSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamUnbufferedInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0009 */
/* [local] */ 

#if (_WIN32_IE >= _WIN32_IE_IE70)
SHSTDAPI SHRemovePersonalPropertyValues(_In_ IShellItemArray *psia);
SHSTDAPI SHAddDefaultPropertiesByExt(_In_ PCWSTR pszExt, _In_ IPropertyStore *pPropStore);
SHSTDAPI SHCreateDefaultPropertiesOp(_In_ IShellItem *psi, _Outptr_ IFileOperation **ppFileOp);
SHSTDAPI SHSetDefaultProperties(_In_opt_ HWND hwnd, _In_ IShellItem *psi, DWORD dwFileOpFlags, _In_opt_ IFileOperationProgressSink *pfops);
#endif  // (_WIN32_IE >= _WIN32_IE_IE70)
#if (NTDDI_VERSION >= NTDDI_WIN2K)
#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef /* [v1_enum] */ 
enum DSH_FLAGS
    {
        DSH_ALLOWDROPDESCRIPTIONTEXT	= 0x1
    } 	DSH_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(DSH_FLAGS)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0009_v0_0_s_ifspec;

#ifndef __IDragSourceHelper2_INTERFACE_DEFINED__
#define __IDragSourceHelper2_INTERFACE_DEFINED__

/* interface IDragSourceHelper2 */
/* [object][unique][local][uuid] */ 


EXTERN_C const IID IID_IDragSourceHelper2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83E07D0D-0C5F-4163-BF1A-60B274051E40")
    IDragSourceHelper2 : public IDragSourceHelper
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDragSourceHelper2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDragSourceHelper2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDragSourceHelper2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDragSourceHelper2 * This);
        
        DECLSPEC_XFGVIRT(IDragSourceHelper, InitializeFromBitmap)
        HRESULT ( STDMETHODCALLTYPE *InitializeFromBitmap )( 
            IDragSourceHelper2 * This,
            /* [annotation][in] */ 
            _In_  LPSHDRAGIMAGE pshdi,
            /* [annotation][in] */ 
            _In_  IDataObject *pDataObject);
        
        DECLSPEC_XFGVIRT(IDragSourceHelper, InitializeFromWindow)
        HRESULT ( STDMETHODCALLTYPE *InitializeFromWindow )( 
            IDragSourceHelper2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  HWND hwnd,
            /* [annotation][unique][in] */ 
            _In_opt_  POINT *ppt,
            /* [annotation][in] */ 
            _In_  IDataObject *pDataObject);
        
        DECLSPEC_XFGVIRT(IDragSourceHelper2, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IDragSourceHelper2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        END_INTERFACE
    } IDragSourceHelper2Vtbl;

    interface IDragSourceHelper2
    {
        CONST_VTBL struct IDragSourceHelper2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDragSourceHelper2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDragSourceHelper2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDragSourceHelper2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDragSourceHelper2_InitializeFromBitmap(This,pshdi,pDataObject)	\
    ( (This)->lpVtbl -> InitializeFromBitmap(This,pshdi,pDataObject) ) 

#define IDragSourceHelper2_InitializeFromWindow(This,hwnd,ppt,pDataObject)	\
    ( (This)->lpVtbl -> InitializeFromWindow(This,hwnd,ppt,pDataObject) ) 


#define IDragSourceHelper2_SetFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDragSourceHelper2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0010 */
/* [local] */ 

#endif  // NTDDI_VISTA
#endif  // NTDDI_WIN2K


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0010_v0_0_s_ifspec;

#ifndef __IHWEventHandler_INTERFACE_DEFINED__
#define __IHWEventHandler_INTERFACE_DEFINED__

/* interface IHWEventHandler */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IHWEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C1FB73D0-EC3A-4ba2-B512-8CDB9187B6D1")
    IHWEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleEvent( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleEventWithContent( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType,
            /* [string][in] */ __RPC__in_string LPCWSTR pszContentTypeHandler,
            /* [in] */ __RPC__in_opt IDataObject *pdataobject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHWEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHWEventHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHWEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHWEventHandler * This);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IHWEventHandler * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszParams);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, HandleEvent)
        HRESULT ( STDMETHODCALLTYPE *HandleEvent )( 
            __RPC__in IHWEventHandler * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, HandleEventWithContent)
        HRESULT ( STDMETHODCALLTYPE *HandleEventWithContent )( 
            __RPC__in IHWEventHandler * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType,
            /* [string][in] */ __RPC__in_string LPCWSTR pszContentTypeHandler,
            /* [in] */ __RPC__in_opt IDataObject *pdataobject);
        
        END_INTERFACE
    } IHWEventHandlerVtbl;

    interface IHWEventHandler
    {
        CONST_VTBL struct IHWEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHWEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHWEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHWEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHWEventHandler_Initialize(This,pszParams)	\
    ( (This)->lpVtbl -> Initialize(This,pszParams) ) 

#define IHWEventHandler_HandleEvent(This,pszDeviceID,pszAltDeviceID,pszEventType)	\
    ( (This)->lpVtbl -> HandleEvent(This,pszDeviceID,pszAltDeviceID,pszEventType) ) 

#define IHWEventHandler_HandleEventWithContent(This,pszDeviceID,pszAltDeviceID,pszEventType,pszContentTypeHandler,pdataobject)	\
    ( (This)->lpVtbl -> HandleEventWithContent(This,pszDeviceID,pszAltDeviceID,pszEventType,pszContentTypeHandler,pdataobject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHWEventHandler_INTERFACE_DEFINED__ */


#ifndef __IHWEventHandler2_INTERFACE_DEFINED__
#define __IHWEventHandler2_INTERFACE_DEFINED__

/* interface IHWEventHandler2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IHWEventHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CFCC809F-295D-42e8-9FFC-424B33C487E6")
    IHWEventHandler2 : public IHWEventHandler
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandleEventWithHWND( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType,
            /* [in] */ __RPC__in HWND hwndOwner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHWEventHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHWEventHandler2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHWEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHWEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IHWEventHandler2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszParams);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, HandleEvent)
        HRESULT ( STDMETHODCALLTYPE *HandleEvent )( 
            __RPC__in IHWEventHandler2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType);
        
        DECLSPEC_XFGVIRT(IHWEventHandler, HandleEventWithContent)
        HRESULT ( STDMETHODCALLTYPE *HandleEventWithContent )( 
            __RPC__in IHWEventHandler2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType,
            /* [string][in] */ __RPC__in_string LPCWSTR pszContentTypeHandler,
            /* [in] */ __RPC__in_opt IDataObject *pdataobject);
        
        DECLSPEC_XFGVIRT(IHWEventHandler2, HandleEventWithHWND)
        HRESULT ( STDMETHODCALLTYPE *HandleEventWithHWND )( 
            __RPC__in IHWEventHandler2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAltDeviceID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszEventType,
            /* [in] */ __RPC__in HWND hwndOwner);
        
        END_INTERFACE
    } IHWEventHandler2Vtbl;

    interface IHWEventHandler2
    {
        CONST_VTBL struct IHWEventHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHWEventHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHWEventHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHWEventHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHWEventHandler2_Initialize(This,pszParams)	\
    ( (This)->lpVtbl -> Initialize(This,pszParams) ) 

#define IHWEventHandler2_HandleEvent(This,pszDeviceID,pszAltDeviceID,pszEventType)	\
    ( (This)->lpVtbl -> HandleEvent(This,pszDeviceID,pszAltDeviceID,pszEventType) ) 

#define IHWEventHandler2_HandleEventWithContent(This,pszDeviceID,pszAltDeviceID,pszEventType,pszContentTypeHandler,pdataobject)	\
    ( (This)->lpVtbl -> HandleEventWithContent(This,pszDeviceID,pszAltDeviceID,pszEventType,pszContentTypeHandler,pdataobject) ) 


#define IHWEventHandler2_HandleEventWithHWND(This,pszDeviceID,pszAltDeviceID,pszEventType,hwndOwner)	\
    ( (This)->lpVtbl -> HandleEventWithHWND(This,pszDeviceID,pszAltDeviceID,pszEventType,hwndOwner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHWEventHandler2_INTERFACE_DEFINED__ */


#ifndef __IQueryCancelAutoPlay_INTERFACE_DEFINED__
#define __IQueryCancelAutoPlay_INTERFACE_DEFINED__

/* interface IQueryCancelAutoPlay */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IQueryCancelAutoPlay;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDEFE873-6997-4e68-BE26-39B633ADBE12")
    IQueryCancelAutoPlay : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AllowAutoPlay( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath,
            /* [in] */ DWORD dwContentType,
            /* [string][in] */ __RPC__in_string LPCWSTR pszLabel,
            /* [in] */ DWORD dwSerialNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryCancelAutoPlayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryCancelAutoPlay * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryCancelAutoPlay * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryCancelAutoPlay * This);
        
        DECLSPEC_XFGVIRT(IQueryCancelAutoPlay, AllowAutoPlay)
        HRESULT ( STDMETHODCALLTYPE *AllowAutoPlay )( 
            __RPC__in IQueryCancelAutoPlay * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath,
            /* [in] */ DWORD dwContentType,
            /* [string][in] */ __RPC__in_string LPCWSTR pszLabel,
            /* [in] */ DWORD dwSerialNumber);
        
        END_INTERFACE
    } IQueryCancelAutoPlayVtbl;

    interface IQueryCancelAutoPlay
    {
        CONST_VTBL struct IQueryCancelAutoPlayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryCancelAutoPlay_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryCancelAutoPlay_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryCancelAutoPlay_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryCancelAutoPlay_AllowAutoPlay(This,pszPath,dwContentType,pszLabel,dwSerialNumber)	\
    ( (This)->lpVtbl -> AllowAutoPlay(This,pszPath,dwContentType,pszLabel,dwSerialNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryCancelAutoPlay_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0013 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0013_v0_0_s_ifspec;

#ifndef __IDynamicHWHandler_INTERFACE_DEFINED__
#define __IDynamicHWHandler_INTERFACE_DEFINED__

/* interface IDynamicHWHandler */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDynamicHWHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DC2601D7-059E-42fc-A09D-2AFD21B6D5F7")
    IDynamicHWHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDynamicInfo( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [in] */ DWORD dwContentType,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDynamicHWHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDynamicHWHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDynamicHWHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDynamicHWHandler * This);
        
        DECLSPEC_XFGVIRT(IDynamicHWHandler, GetDynamicInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDynamicInfo )( 
            __RPC__in IDynamicHWHandler * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDeviceID,
            /* [in] */ DWORD dwContentType,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszAction);
        
        END_INTERFACE
    } IDynamicHWHandlerVtbl;

    interface IDynamicHWHandler
    {
        CONST_VTBL struct IDynamicHWHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDynamicHWHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDynamicHWHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDynamicHWHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDynamicHWHandler_GetDynamicInfo(This,pszDeviceID,dwContentType,ppszAction)	\
    ( (This)->lpVtbl -> GetDynamicInfo(This,pszDeviceID,dwContentType,ppszAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDynamicHWHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0014 */
/* [local] */ 

#endif  // NTDDI_VISTA


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0014_v0_0_s_ifspec;

#ifndef __IUserNotificationCallback_INTERFACE_DEFINED__
#define __IUserNotificationCallback_INTERFACE_DEFINED__

/* interface IUserNotificationCallback */
/* [object][uuid] */ 


EXTERN_C const IID IID_IUserNotificationCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19108294-0441-4AFF-8013-FA0A730B0BEA")
    IUserNotificationCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBalloonUserClick( 
            /* [in] */ __RPC__in POINT *pt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLeftClick( 
            /* [in] */ __RPC__in POINT *pt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnContextMenu( 
            /* [in] */ __RPC__in POINT *pt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserNotificationCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUserNotificationCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUserNotificationCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUserNotificationCallback * This);
        
        DECLSPEC_XFGVIRT(IUserNotificationCallback, OnBalloonUserClick)
        HRESULT ( STDMETHODCALLTYPE *OnBalloonUserClick )( 
            __RPC__in IUserNotificationCallback * This,
            /* [in] */ __RPC__in POINT *pt);
        
        DECLSPEC_XFGVIRT(IUserNotificationCallback, OnLeftClick)
        HRESULT ( STDMETHODCALLTYPE *OnLeftClick )( 
            __RPC__in IUserNotificationCallback * This,
            /* [in] */ __RPC__in POINT *pt);
        
        DECLSPEC_XFGVIRT(IUserNotificationCallback, OnContextMenu)
        HRESULT ( STDMETHODCALLTYPE *OnContextMenu )( 
            __RPC__in IUserNotificationCallback * This,
            /* [in] */ __RPC__in POINT *pt);
        
        END_INTERFACE
    } IUserNotificationCallbackVtbl;

    interface IUserNotificationCallback
    {
        CONST_VTBL struct IUserNotificationCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserNotificationCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserNotificationCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserNotificationCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserNotificationCallback_OnBalloonUserClick(This,pt)	\
    ( (This)->lpVtbl -> OnBalloonUserClick(This,pt) ) 

#define IUserNotificationCallback_OnLeftClick(This,pt)	\
    ( (This)->lpVtbl -> OnLeftClick(This,pt) ) 

#define IUserNotificationCallback_OnContextMenu(This,pt)	\
    ( (This)->lpVtbl -> OnContextMenu(This,pt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserNotificationCallback_INTERFACE_DEFINED__ */


#ifndef __IUserNotification2_INTERFACE_DEFINED__
#define __IUserNotification2_INTERFACE_DEFINED__

/* interface IUserNotification2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IUserNotification2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("215913CC-57EB-4FAB-AB5A-E5FA7BEA2A6C")
    IUserNotification2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBalloonInfo( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszTitle,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszText,
            /* [in] */ DWORD dwInfoFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBalloonRetry( 
            /* [in] */ DWORD dwShowTime,
            /* [in] */ DWORD dwInterval,
            /* [in] */ UINT cRetryCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIconInfo( 
            /* [unique][in] */ __RPC__in_opt HICON hIcon,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszToolTip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Show( 
            /* [unique][in] */ __RPC__in_opt IQueryContinue *pqc,
            /* [in] */ DWORD dwContinuePollInterval,
            /* [unique][in] */ __RPC__in_opt IUserNotificationCallback *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlaySound( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszSoundName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserNotification2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUserNotification2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUserNotification2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUserNotification2 * This);
        
        DECLSPEC_XFGVIRT(IUserNotification2, SetBalloonInfo)
        HRESULT ( STDMETHODCALLTYPE *SetBalloonInfo )( 
            __RPC__in IUserNotification2 * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszTitle,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszText,
            /* [in] */ DWORD dwInfoFlags);
        
        DECLSPEC_XFGVIRT(IUserNotification2, SetBalloonRetry)
        HRESULT ( STDMETHODCALLTYPE *SetBalloonRetry )( 
            __RPC__in IUserNotification2 * This,
            /* [in] */ DWORD dwShowTime,
            /* [in] */ DWORD dwInterval,
            /* [in] */ UINT cRetryCount);
        
        DECLSPEC_XFGVIRT(IUserNotification2, SetIconInfo)
        HRESULT ( STDMETHODCALLTYPE *SetIconInfo )( 
            __RPC__in IUserNotification2 * This,
            /* [unique][in] */ __RPC__in_opt HICON hIcon,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pszToolTip);
        
        DECLSPEC_XFGVIRT(IUserNotification2, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in IUserNotification2 * This,
            /* [unique][in] */ __RPC__in_opt IQueryContinue *pqc,
            /* [in] */ DWORD dwContinuePollInterval,
            /* [unique][in] */ __RPC__in_opt IUserNotificationCallback *pSink);
        
        DECLSPEC_XFGVIRT(IUserNotification2, PlaySound)
        HRESULT ( STDMETHODCALLTYPE *PlaySound )( 
            __RPC__in IUserNotification2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszSoundName);
        
        END_INTERFACE
    } IUserNotification2Vtbl;

    interface IUserNotification2
    {
        CONST_VTBL struct IUserNotification2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserNotification2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserNotification2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserNotification2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserNotification2_SetBalloonInfo(This,pszTitle,pszText,dwInfoFlags)	\
    ( (This)->lpVtbl -> SetBalloonInfo(This,pszTitle,pszText,dwInfoFlags) ) 

#define IUserNotification2_SetBalloonRetry(This,dwShowTime,dwInterval,cRetryCount)	\
    ( (This)->lpVtbl -> SetBalloonRetry(This,dwShowTime,dwInterval,cRetryCount) ) 

#define IUserNotification2_SetIconInfo(This,hIcon,pszToolTip)	\
    ( (This)->lpVtbl -> SetIconInfo(This,hIcon,pszToolTip) ) 

#define IUserNotification2_Show(This,pqc,dwContinuePollInterval,pSink)	\
    ( (This)->lpVtbl -> Show(This,pqc,dwContinuePollInterval,pSink) ) 

#define IUserNotification2_PlaySound(This,pszSoundName)	\
    ( (This)->lpVtbl -> PlaySound(This,pszSoundName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserNotification2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0016 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0016_v0_0_s_ifspec;

#ifndef __IDeskBand2_INTERFACE_DEFINED__
#define __IDeskBand2_INTERFACE_DEFINED__

/* interface IDeskBand2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDeskBand2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79D16DE4-ABEE-4021-8D9D-9169B261D657")
    IDeskBand2 : public IDeskBand
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CanRenderComposited( 
            /* [out] */ __RPC__out BOOL *pfCanRenderComposited) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionState( 
            /* [in] */ BOOL fCompositionEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionState( 
            /* [out] */ __RPC__out BOOL *pfCompositionEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeskBand2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeskBand2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeskBand2 * This);
        
        DECLSPEC_XFGVIRT(IOleWindow, GetWindow)
        /* [input_sync] */ HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in IDeskBand2 * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IOleWindow, ContextSensitiveHelp)
        HRESULT ( STDMETHODCALLTYPE *ContextSensitiveHelp )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ BOOL fEnterMode);
        
        DECLSPEC_XFGVIRT(IDockingWindow, ShowDW)
        HRESULT ( STDMETHODCALLTYPE *ShowDW )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ BOOL fShow);
        
        DECLSPEC_XFGVIRT(IDockingWindow, CloseDW)
        HRESULT ( STDMETHODCALLTYPE *CloseDW )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IDockingWindow, ResizeBorderDW)
        HRESULT ( STDMETHODCALLTYPE *ResizeBorderDW )( 
            __RPC__in IDeskBand2 * This,
            /* [unique][in] */ __RPC__in_opt LPCRECT prcBorder,
            /* [unique][in] */ __RPC__in_opt IUnknown *punkToolbarSite,
            /* [in] */ BOOL fReserved);
        
        DECLSPEC_XFGVIRT(IDeskBand, GetBandInfo)
        HRESULT ( STDMETHODCALLTYPE *GetBandInfo )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ DWORD dwBandID,
            /* [in] */ DWORD dwViewMode,
            /* [out][in] */ __RPC__inout DESKBANDINFO *pdbi);
        
        DECLSPEC_XFGVIRT(IDeskBand2, CanRenderComposited)
        HRESULT ( STDMETHODCALLTYPE *CanRenderComposited )( 
            __RPC__in IDeskBand2 * This,
            /* [out] */ __RPC__out BOOL *pfCanRenderComposited);
        
        DECLSPEC_XFGVIRT(IDeskBand2, SetCompositionState)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionState )( 
            __RPC__in IDeskBand2 * This,
            /* [in] */ BOOL fCompositionEnabled);
        
        DECLSPEC_XFGVIRT(IDeskBand2, GetCompositionState)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionState )( 
            __RPC__in IDeskBand2 * This,
            /* [out] */ __RPC__out BOOL *pfCompositionEnabled);
        
        END_INTERFACE
    } IDeskBand2Vtbl;

    interface IDeskBand2
    {
        CONST_VTBL struct IDeskBand2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeskBand2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeskBand2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeskBand2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeskBand2_GetWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,phwnd) ) 

#define IDeskBand2_ContextSensitiveHelp(This,fEnterMode)	\
    ( (This)->lpVtbl -> ContextSensitiveHelp(This,fEnterMode) ) 


#define IDeskBand2_ShowDW(This,fShow)	\
    ( (This)->lpVtbl -> ShowDW(This,fShow) ) 

#define IDeskBand2_CloseDW(This,dwReserved)	\
    ( (This)->lpVtbl -> CloseDW(This,dwReserved) ) 

#define IDeskBand2_ResizeBorderDW(This,prcBorder,punkToolbarSite,fReserved)	\
    ( (This)->lpVtbl -> ResizeBorderDW(This,prcBorder,punkToolbarSite,fReserved) ) 


#define IDeskBand2_GetBandInfo(This,dwBandID,dwViewMode,pdbi)	\
    ( (This)->lpVtbl -> GetBandInfo(This,dwBandID,dwViewMode,pdbi) ) 


#define IDeskBand2_CanRenderComposited(This,pfCanRenderComposited)	\
    ( (This)->lpVtbl -> CanRenderComposited(This,pfCanRenderComposited) ) 

#define IDeskBand2_SetCompositionState(This,fCompositionEnabled)	\
    ( (This)->lpVtbl -> SetCompositionState(This,fCompositionEnabled) ) 

#define IDeskBand2_GetCompositionState(This,pfCompositionEnabled)	\
    ( (This)->lpVtbl -> GetCompositionState(This,pfCompositionEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeskBand2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0017 */
/* [local] */ 

#endif  // NTDDI_VISTA


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0017_v0_0_s_ifspec;

#ifndef __IStartMenuPinnedList_INTERFACE_DEFINED__
#define __IStartMenuPinnedList_INTERFACE_DEFINED__

/* interface IStartMenuPinnedList */
/* [object][uuid] */ 


EXTERN_C const IID IID_IStartMenuPinnedList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4CD19ADA-25A5-4A32-B3B7-347BEE5BE36B")
    IStartMenuPinnedList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RemoveFromList( 
            /* [in] */ __RPC__in_opt IShellItem *pitem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStartMenuPinnedListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStartMenuPinnedList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStartMenuPinnedList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStartMenuPinnedList * This);
        
        DECLSPEC_XFGVIRT(IStartMenuPinnedList, RemoveFromList)
        HRESULT ( STDMETHODCALLTYPE *RemoveFromList )( 
            __RPC__in IStartMenuPinnedList * This,
            /* [in] */ __RPC__in_opt IShellItem *pitem);
        
        END_INTERFACE
    } IStartMenuPinnedListVtbl;

    interface IStartMenuPinnedList
    {
        CONST_VTBL struct IStartMenuPinnedListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStartMenuPinnedList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStartMenuPinnedList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStartMenuPinnedList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStartMenuPinnedList_RemoveFromList(This,pitem)	\
    ( (This)->lpVtbl -> RemoveFromList(This,pitem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStartMenuPinnedList_INTERFACE_DEFINED__ */


#ifndef __ICDBurn_INTERFACE_DEFINED__
#define __ICDBurn_INTERFACE_DEFINED__

/* interface ICDBurn */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICDBurn;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d73a659-e5d0-4d42-afc0-5121ba425c8d")
    ICDBurn : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRecorderDriveLetter( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cch) LPWSTR pszDrive,
            /* [in] */ UINT cch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Burn( 
            /* [in] */ __RPC__in HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasRecordableDrive( 
            /* [out] */ __RPC__out BOOL *pfHasRecorder) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICDBurnVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICDBurn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICDBurn * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICDBurn * This);
        
        DECLSPEC_XFGVIRT(ICDBurn, GetRecorderDriveLetter)
        HRESULT ( STDMETHODCALLTYPE *GetRecorderDriveLetter )( 
            __RPC__in ICDBurn * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cch) LPWSTR pszDrive,
            /* [in] */ UINT cch);
        
        DECLSPEC_XFGVIRT(ICDBurn, Burn)
        HRESULT ( STDMETHODCALLTYPE *Burn )( 
            __RPC__in ICDBurn * This,
            /* [in] */ __RPC__in HWND hwnd);
        
        DECLSPEC_XFGVIRT(ICDBurn, HasRecordableDrive)
        HRESULT ( STDMETHODCALLTYPE *HasRecordableDrive )( 
            __RPC__in ICDBurn * This,
            /* [out] */ __RPC__out BOOL *pfHasRecorder);
        
        END_INTERFACE
    } ICDBurnVtbl;

    interface ICDBurn
    {
        CONST_VTBL struct ICDBurnVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICDBurn_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICDBurn_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICDBurn_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICDBurn_GetRecorderDriveLetter(This,pszDrive,cch)	\
    ( (This)->lpVtbl -> GetRecorderDriveLetter(This,pszDrive,cch) ) 

#define ICDBurn_Burn(This,hwnd)	\
    ( (This)->lpVtbl -> Burn(This,hwnd) ) 

#define ICDBurn_HasRecordableDrive(This,pfHasRecorder)	\
    ( (This)->lpVtbl -> HasRecordableDrive(This,pfHasRecorder) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICDBurn_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0019 */
/* [local] */ 

#define IDD_WIZEXTN_FIRST    0x5000
#define IDD_WIZEXTN_LAST     0x5100


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0019_v0_0_s_ifspec;

#ifndef __IWizardSite_INTERFACE_DEFINED__
#define __IWizardSite_INTERFACE_DEFINED__

/* interface IWizardSite */
/* [object][local][uuid] */ 


EXTERN_C const IID IID_IWizardSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88960f5b-422f-4e7b-8013-73415381c3c3")
    IWizardSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreviousPage( 
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextPage( 
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCancelledPage( 
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWizardSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWizardSite * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWizardSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWizardSite * This);
        
        DECLSPEC_XFGVIRT(IWizardSite, GetPreviousPage)
        HRESULT ( STDMETHODCALLTYPE *GetPreviousPage )( 
            IWizardSite * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWizardSite, GetNextPage)
        HRESULT ( STDMETHODCALLTYPE *GetNextPage )( 
            IWizardSite * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWizardSite, GetCancelledPage)
        HRESULT ( STDMETHODCALLTYPE *GetCancelledPage )( 
            IWizardSite * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        END_INTERFACE
    } IWizardSiteVtbl;

    interface IWizardSite
    {
        CONST_VTBL struct IWizardSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWizardSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWizardSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWizardSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWizardSite_GetPreviousPage(This,phpage)	\
    ( (This)->lpVtbl -> GetPreviousPage(This,phpage) ) 

#define IWizardSite_GetNextPage(This,phpage)	\
    ( (This)->lpVtbl -> GetNextPage(This,phpage) ) 

#define IWizardSite_GetCancelledPage(This,phpage)	\
    ( (This)->lpVtbl -> GetCancelledPage(This,phpage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWizardSite_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0020 */
/* [local] */ 

#define SID_WizardSite IID_IWizardSite


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0020_v0_0_s_ifspec;

#ifndef __IWizardExtension_INTERFACE_DEFINED__
#define __IWizardExtension_INTERFACE_DEFINED__

/* interface IWizardExtension */
/* [object][local][uuid] */ 


EXTERN_C const IID IID_IWizardExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c02ea696-86cc-491e-9b23-74394a0444a8")
    IWizardExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddPages( 
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPages, *pnPagesAdded)  HPROPSHEETPAGE *aPages,
            /* [annotation][in] */ 
            _In_  UINT cPages,
            /* [annotation][out] */ 
            _Out_ _Deref_out_range_(0, cPages)  UINT *pnPagesAdded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFirstPage( 
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastPage( 
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWizardExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWizardExtension * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWizardExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWizardExtension * This);
        
        DECLSPEC_XFGVIRT(IWizardExtension, AddPages)
        HRESULT ( STDMETHODCALLTYPE *AddPages )( 
            IWizardExtension * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPages, *pnPagesAdded)  HPROPSHEETPAGE *aPages,
            /* [annotation][in] */ 
            _In_  UINT cPages,
            /* [annotation][out] */ 
            _Out_ _Deref_out_range_(0, cPages)  UINT *pnPagesAdded);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetFirstPage)
        HRESULT ( STDMETHODCALLTYPE *GetFirstPage )( 
            IWizardExtension * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetLastPage)
        HRESULT ( STDMETHODCALLTYPE *GetLastPage )( 
            IWizardExtension * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        END_INTERFACE
    } IWizardExtensionVtbl;

    interface IWizardExtension
    {
        CONST_VTBL struct IWizardExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWizardExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWizardExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWizardExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWizardExtension_AddPages(This,aPages,cPages,pnPagesAdded)	\
    ( (This)->lpVtbl -> AddPages(This,aPages,cPages,pnPagesAdded) ) 

#define IWizardExtension_GetFirstPage(This,phpage)	\
    ( (This)->lpVtbl -> GetFirstPage(This,phpage) ) 

#define IWizardExtension_GetLastPage(This,phpage)	\
    ( (This)->lpVtbl -> GetLastPage(This,phpage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWizardExtension_INTERFACE_DEFINED__ */


#ifndef __IWebWizardExtension_INTERFACE_DEFINED__
#define __IWebWizardExtension_INTERFACE_DEFINED__

/* interface IWebWizardExtension */
/* [object][unique][local][uuid] */ 


EXTERN_C const IID IID_IWebWizardExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e6b3f66-98d1-48c0-a222-fbde74e2fbc5")
    IWebWizardExtension : public IWizardExtension
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInitialURL( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetErrorURL( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszErrorURL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebWizardExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebWizardExtension * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebWizardExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebWizardExtension * This);
        
        DECLSPEC_XFGVIRT(IWizardExtension, AddPages)
        HRESULT ( STDMETHODCALLTYPE *AddPages )( 
            IWebWizardExtension * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPages, *pnPagesAdded)  HPROPSHEETPAGE *aPages,
            /* [annotation][in] */ 
            _In_  UINT cPages,
            /* [annotation][out] */ 
            _Out_ _Deref_out_range_(0, cPages)  UINT *pnPagesAdded);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetFirstPage)
        HRESULT ( STDMETHODCALLTYPE *GetFirstPage )( 
            IWebWizardExtension * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetLastPage)
        HRESULT ( STDMETHODCALLTYPE *GetLastPage )( 
            IWebWizardExtension * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWebWizardExtension, SetInitialURL)
        HRESULT ( STDMETHODCALLTYPE *SetInitialURL )( 
            IWebWizardExtension * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszURL);
        
        DECLSPEC_XFGVIRT(IWebWizardExtension, SetErrorURL)
        HRESULT ( STDMETHODCALLTYPE *SetErrorURL )( 
            IWebWizardExtension * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszErrorURL);
        
        END_INTERFACE
    } IWebWizardExtensionVtbl;

    interface IWebWizardExtension
    {
        CONST_VTBL struct IWebWizardExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebWizardExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebWizardExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebWizardExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebWizardExtension_AddPages(This,aPages,cPages,pnPagesAdded)	\
    ( (This)->lpVtbl -> AddPages(This,aPages,cPages,pnPagesAdded) ) 

#define IWebWizardExtension_GetFirstPage(This,phpage)	\
    ( (This)->lpVtbl -> GetFirstPage(This,phpage) ) 

#define IWebWizardExtension_GetLastPage(This,phpage)	\
    ( (This)->lpVtbl -> GetLastPage(This,phpage) ) 


#define IWebWizardExtension_SetInitialURL(This,pszURL)	\
    ( (This)->lpVtbl -> SetInitialURL(This,pszURL) ) 

#define IWebWizardExtension_SetErrorURL(This,pszErrorURL)	\
    ( (This)->lpVtbl -> SetErrorURL(This,pszErrorURL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebWizardExtension_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0022 */
/* [local] */ 

#define SID_WebWizardHost IID_IWebWizardExtension
#define SHPWHF_NORECOMPRESS             0x00000001  // don't allow/prompt for recompress of streams
#define SHPWHF_NONETPLACECREATE         0x00000002  // don't create a network place when transfer is complete
#define SHPWHF_NOFILESELECTOR           0x00000004  // don't show the file selector
#define SHPWHF_USEMRU                   0x00000008  // For OPW.  Use the Most-Recently-Used Print Provider
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define SHPWHF_ANYLOCATION              0x00000100  // allow publishing to any location
#endif  // NTDDI_VISTA
#define SHPWHF_VALIDATEVIAWEBFOLDERS    0x00010000  // enable web folders to validate network places (ANP support)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0022_v0_0_s_ifspec;

#ifndef __IPublishingWizard_INTERFACE_DEFINED__
#define __IPublishingWizard_INTERFACE_DEFINED__

/* interface IPublishingWizard */
/* [object][unique][local][uuid] */ 


EXTERN_C const IID IID_IPublishingWizard;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa9198bb-ccec-472d-beed-19a4f6733f7a")
    IPublishingWizard : public IWizardExtension
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][unique][in] */ 
            _In_opt_  IDataObject *pdo,
            /* [annotation][in] */ 
            _In_  DWORD dwOptions,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszServiceScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransferManifest( 
            /* [annotation][out] */ 
            _Out_opt_  HRESULT *phrFromTransfer,
            /* [annotation][out] */ 
            _Outptr_opt_  IXMLDOMDocument **pdocManifest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPublishingWizardVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPublishingWizard * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPublishingWizard * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPublishingWizard * This);
        
        DECLSPEC_XFGVIRT(IWizardExtension, AddPages)
        HRESULT ( STDMETHODCALLTYPE *AddPages )( 
            IPublishingWizard * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPages, *pnPagesAdded)  HPROPSHEETPAGE *aPages,
            /* [annotation][in] */ 
            _In_  UINT cPages,
            /* [annotation][out] */ 
            _Out_ _Deref_out_range_(0, cPages)  UINT *pnPagesAdded);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetFirstPage)
        HRESULT ( STDMETHODCALLTYPE *GetFirstPage )( 
            IPublishingWizard * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IWizardExtension, GetLastPage)
        HRESULT ( STDMETHODCALLTYPE *GetLastPage )( 
            IPublishingWizard * This,
            /* [annotation][out] */ 
            _Out_  HPROPSHEETPAGE *phpage);
        
        DECLSPEC_XFGVIRT(IPublishingWizard, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPublishingWizard * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IDataObject *pdo,
            /* [annotation][in] */ 
            _In_  DWORD dwOptions,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszServiceScope);
        
        DECLSPEC_XFGVIRT(IPublishingWizard, GetTransferManifest)
        HRESULT ( STDMETHODCALLTYPE *GetTransferManifest )( 
            IPublishingWizard * This,
            /* [annotation][out] */ 
            _Out_opt_  HRESULT *phrFromTransfer,
            /* [annotation][out] */ 
            _Outptr_opt_  IXMLDOMDocument **pdocManifest);
        
        END_INTERFACE
    } IPublishingWizardVtbl;

    interface IPublishingWizard
    {
        CONST_VTBL struct IPublishingWizardVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPublishingWizard_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPublishingWizard_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPublishingWizard_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPublishingWizard_AddPages(This,aPages,cPages,pnPagesAdded)	\
    ( (This)->lpVtbl -> AddPages(This,aPages,cPages,pnPagesAdded) ) 

#define IPublishingWizard_GetFirstPage(This,phpage)	\
    ( (This)->lpVtbl -> GetFirstPage(This,phpage) ) 

#define IPublishingWizard_GetLastPage(This,phpage)	\
    ( (This)->lpVtbl -> GetLastPage(This,phpage) ) 


#define IPublishingWizard_Initialize(This,pdo,dwOptions,pszServiceScope)	\
    ( (This)->lpVtbl -> Initialize(This,pdo,dwOptions,pszServiceScope) ) 

#define IPublishingWizard_GetTransferManifest(This,phrFromTransfer,pdocManifest)	\
    ( (This)->lpVtbl -> GetTransferManifest(This,phrFromTransfer,pdocManifest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPublishingWizard_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0023 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINXP) || (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0023_v0_0_s_ifspec;

#ifndef __IFolderViewHost_INTERFACE_DEFINED__
#define __IFolderViewHost_INTERFACE_DEFINED__

/* interface IFolderViewHost */
/* [object][local][uuid] */ 


EXTERN_C const IID IID_IFolderViewHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1ea58f02-d55a-411d-b09e-9e65ac21605b")
    IFolderViewHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_  IDataObject *pdo,
            /* [annotation][in] */ 
            _In_  RECT *prc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderViewHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFolderViewHost * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFolderViewHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFolderViewHost * This);
        
        DECLSPEC_XFGVIRT(IFolderViewHost, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IFolderViewHost * This,
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_  IDataObject *pdo,
            /* [annotation][in] */ 
            _In_  RECT *prc);
        
        END_INTERFACE
    } IFolderViewHostVtbl;

    interface IFolderViewHost
    {
        CONST_VTBL struct IFolderViewHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderViewHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderViewHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderViewHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderViewHost_Initialize(This,hwndParent,pdo,prc)	\
    ( (This)->lpVtbl -> Initialize(This,hwndParent,pdo,prc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderViewHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0024 */
/* [local] */ 

#if (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0024_v0_0_s_ifspec;

#ifndef __IAccessibleObject_INTERFACE_DEFINED__
#define __IAccessibleObject_INTERFACE_DEFINED__

/* interface IAccessibleObject */
/* [object][uuid] */ 


EXTERN_C const IID IID_IAccessibleObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95A391C5-9ED4-4c28-8401-AB9E06719E11")
    IAccessibleObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAccessibleName( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleObject * This);
        
        DECLSPEC_XFGVIRT(IAccessibleObject, SetAccessibleName)
        HRESULT ( STDMETHODCALLTYPE *SetAccessibleName )( 
            __RPC__in IAccessibleObject * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszName);
        
        END_INTERFACE
    } IAccessibleObjectVtbl;

    interface IAccessibleObject
    {
        CONST_VTBL struct IAccessibleObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleObject_SetAccessibleName(This,pszName)	\
    ( (This)->lpVtbl -> SetAccessibleName(This,pszName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleObject_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0025 */
/* [local] */ 

#endif  // (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0025_v0_0_s_ifspec;

#ifndef __IResultsFolder_INTERFACE_DEFINED__
#define __IResultsFolder_INTERFACE_DEFINED__

/* interface IResultsFolder */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IResultsFolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("96E5AE6D-6AE1-4b1c-900C-C6480EAA8828")
    IResultsFolder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in_opt IShellItem *psi) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE AddIDList( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Outptr_opt_  PITEMID_CHILD *ppidlAdded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ __RPC__in_opt IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveIDList( 
            /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultsFolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultsFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultsFolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultsFolder * This);
        
        DECLSPEC_XFGVIRT(IResultsFolder, AddItem)
        HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IResultsFolder * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        DECLSPEC_XFGVIRT(IResultsFolder, AddIDList)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *AddIDList )( 
            IResultsFolder * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Outptr_opt_  PITEMID_CHILD *ppidlAdded);
        
        DECLSPEC_XFGVIRT(IResultsFolder, RemoveItem)
        HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in IResultsFolder * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        DECLSPEC_XFGVIRT(IResultsFolder, RemoveIDList)
        HRESULT ( STDMETHODCALLTYPE *RemoveIDList )( 
            __RPC__in IResultsFolder * This,
            /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IResultsFolder, RemoveAll)
        HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            __RPC__in IResultsFolder * This);
        
        END_INTERFACE
    } IResultsFolderVtbl;

    interface IResultsFolder
    {
        CONST_VTBL struct IResultsFolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultsFolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultsFolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultsFolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultsFolder_AddItem(This,psi)	\
    ( (This)->lpVtbl -> AddItem(This,psi) ) 

#define IResultsFolder_AddIDList(This,pidl,ppidlAdded)	\
    ( (This)->lpVtbl -> AddIDList(This,pidl,ppidlAdded) ) 

#define IResultsFolder_RemoveItem(This,psi)	\
    ( (This)->lpVtbl -> RemoveItem(This,psi) ) 

#define IResultsFolder_RemoveIDList(This,pidl)	\
    ( (This)->lpVtbl -> RemoveIDList(This,pidl) ) 

#define IResultsFolder_RemoveAll(This)	\
    ( (This)->lpVtbl -> RemoveAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IResultsFolder_RemoteAddIDList_Proxy( 
    __RPC__in IResultsFolder * This,
    /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl,
    /* [out] */ __RPC__deref_out_opt PITEMID_CHILD *ppidlAdded);


void __RPC_STUB IResultsFolder_RemoteAddIDList_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IResultsFolder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0026 */
/* [local] */ 

#if (_WIN32_IE >= _WIN32_IE_IE70)
#endif  // (_WIN32_IE >= _WIN32_IE_IE70)
#endif  // NTDDI_WINXP || (_WIN32_IE >= _WIN32_IE_IE70)
#define ACDD_VISIBLE        0x0001


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0026_v0_0_s_ifspec;

#ifndef __IAutoCompleteDropDown_INTERFACE_DEFINED__
#define __IAutoCompleteDropDown_INTERFACE_DEFINED__

/* interface IAutoCompleteDropDown */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IAutoCompleteDropDown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3CD141F4-3C6A-11d2-BCAA-00C04FD929DB")
    IAutoCompleteDropDown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDropDownStatus( 
            /* [out] */ __RPC__out DWORD *pdwFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppwszString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetEnumerator( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAutoCompleteDropDownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAutoCompleteDropDown * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAutoCompleteDropDown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAutoCompleteDropDown * This);
        
        DECLSPEC_XFGVIRT(IAutoCompleteDropDown, GetDropDownStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDropDownStatus )( 
            __RPC__in IAutoCompleteDropDown * This,
            /* [out] */ __RPC__out DWORD *pdwFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppwszString);
        
        DECLSPEC_XFGVIRT(IAutoCompleteDropDown, ResetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *ResetEnumerator )( 
            __RPC__in IAutoCompleteDropDown * This);
        
        END_INTERFACE
    } IAutoCompleteDropDownVtbl;

    interface IAutoCompleteDropDown
    {
        CONST_VTBL struct IAutoCompleteDropDownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAutoCompleteDropDown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAutoCompleteDropDown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAutoCompleteDropDown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAutoCompleteDropDown_GetDropDownStatus(This,pdwFlags,ppwszString)	\
    ( (This)->lpVtbl -> GetDropDownStatus(This,pdwFlags,ppwszString) ) 

#define IAutoCompleteDropDown_ResetEnumerator(This)	\
    ( (This)->lpVtbl -> ResetEnumerator(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAutoCompleteDropDown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0027 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define PROPSTR_EXTENSIONCOMPLETIONSTATE L"ExtensionCompletionState"

enum tagCDBURNINGEXTENSIONRET
    {
        CDBE_RET_DEFAULT	= 0,
        CDBE_RET_DONTRUNOTHEREXTS	= 0x1,
        CDBE_RET_STOPWIZARD	= 0x2
    } ;
#define SID_CDWizardHost IID_ICDBurnExt
/* [v1_enum] */ 
enum _CDBE_ACTIONS
    {
        CDBE_TYPE_MUSIC	= 0x1,
        CDBE_TYPE_DATA	= 0x2,
        CDBE_TYPE_ALL	= ( int  )0xffffffff
    } ;
typedef DWORD CDBE_ACTIONS;



extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0027_v0_0_s_ifspec;

#ifndef __ICDBurnExt_INTERFACE_DEFINED__
#define __ICDBurnExt_INTERFACE_DEFINED__

/* interface ICDBurnExt */
/* [object][uuid] */ 


EXTERN_C const IID IID_ICDBurnExt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2271dcca-74fc-4414-8fb7-c56b05ace2d7")
    ICDBurnExt : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedActionTypes( 
            /* [out] */ __RPC__out CDBE_ACTIONS *pdwActions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICDBurnExtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICDBurnExt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICDBurnExt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICDBurnExt * This);
        
        DECLSPEC_XFGVIRT(ICDBurnExt, GetSupportedActionTypes)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedActionTypes )( 
            __RPC__in ICDBurnExt * This,
            /* [out] */ __RPC__out CDBE_ACTIONS *pdwActions);
        
        END_INTERFACE
    } ICDBurnExtVtbl;

    interface ICDBurnExt
    {
        CONST_VTBL struct ICDBurnExtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICDBurnExt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICDBurnExt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICDBurnExt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICDBurnExt_GetSupportedActionTypes(This,pdwActions)	\
    ( (This)->lpVtbl -> GetSupportedActionTypes(This,pdwActions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICDBurnExt_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0028 */
/* [local] */ 

#endif  // NTDDI_WINXP


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0028_v0_0_s_ifspec;

#ifndef __IEnumReadyCallback_INTERFACE_DEFINED__
#define __IEnumReadyCallback_INTERFACE_DEFINED__

/* interface IEnumReadyCallback */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_IEnumReadyCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61E00D45-8FFF-4e60-924E-6537B61612DD")
    IEnumReadyCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumReady( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumReadyCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumReadyCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumReadyCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumReadyCallback * This);
        
        DECLSPEC_XFGVIRT(IEnumReadyCallback, EnumReady)
        HRESULT ( STDMETHODCALLTYPE *EnumReady )( 
            IEnumReadyCallback * This);
        
        END_INTERFACE
    } IEnumReadyCallbackVtbl;

    interface IEnumReadyCallback
    {
        CONST_VTBL struct IEnumReadyCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumReadyCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumReadyCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumReadyCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumReadyCallback_EnumReady(This)	\
    ( (This)->lpVtbl -> EnumReady(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumReadyCallback_INTERFACE_DEFINED__ */


#ifndef __IEnumerableView_INTERFACE_DEFINED__
#define __IEnumerableView_INTERFACE_DEFINED__

/* interface IEnumerableView */
/* [object][local][uuid] */ 


EXTERN_C const IID IID_IEnumerableView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8C8BF236-1AEC-495f-9894-91D57C3C686F")
    IEnumerableView : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetEnumReadyCallback( 
            /* [annotation][in] */ 
            _In_  IEnumReadyCallback *percb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEnumIDListFromContents( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlFolder,
            /* [annotation][in] */ 
            _In_  DWORD dwEnumFlags,
            /* [annotation][out] */ 
            _Outptr_  IEnumIDList **ppEnumIDList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumerableViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumerableView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumerableView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumerableView * This);
        
        DECLSPEC_XFGVIRT(IEnumerableView, SetEnumReadyCallback)
        HRESULT ( STDMETHODCALLTYPE *SetEnumReadyCallback )( 
            IEnumerableView * This,
            /* [annotation][in] */ 
            _In_  IEnumReadyCallback *percb);
        
        DECLSPEC_XFGVIRT(IEnumerableView, CreateEnumIDListFromContents)
        HRESULT ( STDMETHODCALLTYPE *CreateEnumIDListFromContents )( 
            IEnumerableView * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlFolder,
            /* [annotation][in] */ 
            _In_  DWORD dwEnumFlags,
            /* [annotation][out] */ 
            _Outptr_  IEnumIDList **ppEnumIDList);
        
        END_INTERFACE
    } IEnumerableViewVtbl;

    interface IEnumerableView
    {
        CONST_VTBL struct IEnumerableViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumerableView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumerableView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumerableView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumerableView_SetEnumReadyCallback(This,percb)	\
    ( (This)->lpVtbl -> SetEnumReadyCallback(This,percb) ) 

#define IEnumerableView_CreateEnumIDListFromContents(This,pidlFolder,dwEnumFlags,ppEnumIDList)	\
    ( (This)->lpVtbl -> CreateEnumIDListFromContents(This,pidlFolder,dwEnumFlags,ppEnumIDList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumerableView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0030 */
/* [local] */ 

#define SID_EnumerableView IID_IEnumerableView
#if (NTDDI_VERSION >= NTDDI_WINXP) || (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0030_v0_0_s_ifspec;

#ifndef __IInsertItem_INTERFACE_DEFINED__
#define __IInsertItem_INTERFACE_DEFINED__

/* interface IInsertItem */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IInsertItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2B57227-3D23-4b95-93C0-492BD454C356")
    IInsertItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InsertItem( 
            /* [annotation][in] */ 
            _In_  PCUIDLIST_RELATIVE pidl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInsertItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInsertItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInsertItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInsertItem * This);
        
        DECLSPEC_XFGVIRT(IInsertItem, InsertItem)
        HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            IInsertItem * This,
            /* [annotation][in] */ 
            _In_  PCUIDLIST_RELATIVE pidl);
        
        END_INTERFACE
    } IInsertItemVtbl;

    interface IInsertItem
    {
        CONST_VTBL struct IInsertItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInsertItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInsertItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInsertItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInsertItem_InsertItem(This,pidl)	\
    ( (This)->lpVtbl -> InsertItem(This,pidl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInsertItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0031 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINXP)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0031_v0_0_s_ifspec;

#ifndef __IFolderBandPriv_INTERFACE_DEFINED__
#define __IFolderBandPriv_INTERFACE_DEFINED__

/* interface IFolderBandPriv */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IFolderBandPriv;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("47c01f95-e185-412c-b5c5-4f27df965aea")
    IFolderBandPriv : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCascade( 
            /* [in] */ BOOL fCascade) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccelerators( 
            /* [in] */ BOOL fAccelerators) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNoIcons( 
            /* [in] */ BOOL fNoIcons) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNoText( 
            /* [in] */ BOOL fNoText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderBandPrivVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFolderBandPriv * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFolderBandPriv * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFolderBandPriv * This);
        
        DECLSPEC_XFGVIRT(IFolderBandPriv, SetCascade)
        HRESULT ( STDMETHODCALLTYPE *SetCascade )( 
            __RPC__in IFolderBandPriv * This,
            /* [in] */ BOOL fCascade);
        
        DECLSPEC_XFGVIRT(IFolderBandPriv, SetAccelerators)
        HRESULT ( STDMETHODCALLTYPE *SetAccelerators )( 
            __RPC__in IFolderBandPriv * This,
            /* [in] */ BOOL fAccelerators);
        
        DECLSPEC_XFGVIRT(IFolderBandPriv, SetNoIcons)
        HRESULT ( STDMETHODCALLTYPE *SetNoIcons )( 
            __RPC__in IFolderBandPriv * This,
            /* [in] */ BOOL fNoIcons);
        
        DECLSPEC_XFGVIRT(IFolderBandPriv, SetNoText)
        HRESULT ( STDMETHODCALLTYPE *SetNoText )( 
            __RPC__in IFolderBandPriv * This,
            /* [in] */ BOOL fNoText);
        
        END_INTERFACE
    } IFolderBandPrivVtbl;

    interface IFolderBandPriv
    {
        CONST_VTBL struct IFolderBandPrivVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderBandPriv_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderBandPriv_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderBandPriv_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderBandPriv_SetCascade(This,fCascade)	\
    ( (This)->lpVtbl -> SetCascade(This,fCascade) ) 

#define IFolderBandPriv_SetAccelerators(This,fAccelerators)	\
    ( (This)->lpVtbl -> SetAccelerators(This,fAccelerators) ) 

#define IFolderBandPriv_SetNoIcons(This,fNoIcons)	\
    ( (This)->lpVtbl -> SetNoIcons(This,fNoIcons) ) 

#define IFolderBandPriv_SetNoText(This,fNoText)	\
    ( (This)->lpVtbl -> SetNoText(This,fNoText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderBandPriv_INTERFACE_DEFINED__ */


#ifndef __IImageRecompress_INTERFACE_DEFINED__
#define __IImageRecompress_INTERFACE_DEFINED__

/* interface IImageRecompress */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IImageRecompress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("505f1513-6b3e-4892-a272-59f8889a4d3e")
    IImageRecompress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RecompressImage( 
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ int cx,
            /* [in] */ int cy,
            /* [in] */ int iQuality,
            /* [in] */ __RPC__in_opt IStorage *pstg,
            /* [out] */ __RPC__deref_out_opt IStream **ppstrmOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImageRecompressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IImageRecompress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IImageRecompress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IImageRecompress * This);
        
        DECLSPEC_XFGVIRT(IImageRecompress, RecompressImage)
        HRESULT ( STDMETHODCALLTYPE *RecompressImage )( 
            __RPC__in IImageRecompress * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ int cx,
            /* [in] */ int cy,
            /* [in] */ int iQuality,
            /* [in] */ __RPC__in_opt IStorage *pstg,
            /* [out] */ __RPC__deref_out_opt IStream **ppstrmOut);
        
        END_INTERFACE
    } IImageRecompressVtbl;

    interface IImageRecompress
    {
        CONST_VTBL struct IImageRecompressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImageRecompress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImageRecompress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImageRecompress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImageRecompress_RecompressImage(This,psi,cx,cy,iQuality,pstg,ppstrmOut)	\
    ( (This)->lpVtbl -> RecompressImage(This,psi,cx,cy,iQuality,pstg,ppstrmOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImageRecompress_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0033 */
/* [local] */ 

#endif  // NTDDI_WINXP
#endif  // NTDDI_WINXP) || (_WIN32_IE >= _WIN32_IE_IE70)
#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0033_v0_0_s_ifspec;

#ifndef __IFileDialogControlEvents_INTERFACE_DEFINED__
#define __IFileDialogControlEvents_INTERFACE_DEFINED__

/* interface IFileDialogControlEvents */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IFileDialogControlEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36116642-D713-4b97-9B83-7484A9D00433")
    IFileDialogControlEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnItemSelected( 
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl,
            /* [in] */ DWORD dwIDItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnButtonClicked( 
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCheckButtonToggled( 
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl,
            /* [in] */ BOOL bChecked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnControlActivating( 
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileDialogControlEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileDialogControlEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileDialogControlEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileDialogControlEvents * This);
        
        DECLSPEC_XFGVIRT(IFileDialogControlEvents, OnItemSelected)
        HRESULT ( STDMETHODCALLTYPE *OnItemSelected )( 
            __RPC__in IFileDialogControlEvents * This,
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl,
            /* [in] */ DWORD dwIDItem);
        
        DECLSPEC_XFGVIRT(IFileDialogControlEvents, OnButtonClicked)
        HRESULT ( STDMETHODCALLTYPE *OnButtonClicked )( 
            __RPC__in IFileDialogControlEvents * This,
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl);
        
        DECLSPEC_XFGVIRT(IFileDialogControlEvents, OnCheckButtonToggled)
        HRESULT ( STDMETHODCALLTYPE *OnCheckButtonToggled )( 
            __RPC__in IFileDialogControlEvents * This,
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl,
            /* [in] */ BOOL bChecked);
        
        DECLSPEC_XFGVIRT(IFileDialogControlEvents, OnControlActivating)
        HRESULT ( STDMETHODCALLTYPE *OnControlActivating )( 
            __RPC__in IFileDialogControlEvents * This,
            /* [in] */ __RPC__in_opt IFileDialogCustomize *pfdc,
            /* [in] */ DWORD dwIDCtl);
        
        END_INTERFACE
    } IFileDialogControlEventsVtbl;

    interface IFileDialogControlEvents
    {
        CONST_VTBL struct IFileDialogControlEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileDialogControlEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileDialogControlEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileDialogControlEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileDialogControlEvents_OnItemSelected(This,pfdc,dwIDCtl,dwIDItem)	\
    ( (This)->lpVtbl -> OnItemSelected(This,pfdc,dwIDCtl,dwIDItem) ) 

#define IFileDialogControlEvents_OnButtonClicked(This,pfdc,dwIDCtl)	\
    ( (This)->lpVtbl -> OnButtonClicked(This,pfdc,dwIDCtl) ) 

#define IFileDialogControlEvents_OnCheckButtonToggled(This,pfdc,dwIDCtl,bChecked)	\
    ( (This)->lpVtbl -> OnCheckButtonToggled(This,pfdc,dwIDCtl,bChecked) ) 

#define IFileDialogControlEvents_OnControlActivating(This,pfdc,dwIDCtl)	\
    ( (This)->lpVtbl -> OnControlActivating(This,pfdc,dwIDCtl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileDialogControlEvents_INTERFACE_DEFINED__ */


#ifndef __IFileDialog2_INTERFACE_DEFINED__
#define __IFileDialog2_INTERFACE_DEFINED__

/* interface IFileDialog2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IFileDialog2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61744fc7-85b5-4791-a9b0-272276309b13")
    IFileDialog2 : public IFileDialog
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCancelButtonLabel( 
            /* [in] */ __RPC__in LPCWSTR pszLabel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNavigationRoot( 
            /* [in] */ __RPC__in_opt IShellItem *psi) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileDialog2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileDialog2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileDialog2 * This);
        
        DECLSPEC_XFGVIRT(IModalWindow, Show)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            IFileDialog2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  HWND hwndOwner);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFileTypes)
        HRESULT ( STDMETHODCALLTYPE *SetFileTypes )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ UINT cFileTypes,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileTypes) const COMDLG_FILTERSPEC *rgFilterSpec);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFileTypeIndex)
        HRESULT ( STDMETHODCALLTYPE *SetFileTypeIndex )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ UINT iFileType);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetFileTypeIndex)
        HRESULT ( STDMETHODCALLTYPE *GetFileTypeIndex )( 
            __RPC__in IFileDialog2 * This,
            /* [out] */ __RPC__out UINT *piFileType);
        
        DECLSPEC_XFGVIRT(IFileDialog, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IFileDialogEvents *pfde,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IFileDialog, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetOptions)
        HRESULT ( STDMETHODCALLTYPE *SetOptions )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ FILEOPENDIALOGOPTIONS fos);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetOptions)
        HRESULT ( STDMETHODCALLTYPE *GetOptions )( 
            __RPC__in IFileDialog2 * This,
            /* [out] */ __RPC__out FILEOPENDIALOGOPTIONS *pfos);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetDefaultFolder)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultFolder )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFolder)
        HRESULT ( STDMETHODCALLTYPE *SetFolder )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetFolder)
        HRESULT ( STDMETHODCALLTYPE *GetFolder )( 
            __RPC__in IFileDialog2 * This,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsi);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetCurrentSelection)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSelection )( 
            __RPC__in IFileDialog2 * This,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsi);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFileName)
        HRESULT ( STDMETHODCALLTYPE *SetFileName )( 
            __RPC__in IFileDialog2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetFileName)
        HRESULT ( STDMETHODCALLTYPE *GetFileName )( 
            __RPC__in IFileDialog2 * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *pszName);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetTitle)
        HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            __RPC__in IFileDialog2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszTitle);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetOkButtonLabel)
        HRESULT ( STDMETHODCALLTYPE *SetOkButtonLabel )( 
            __RPC__in IFileDialog2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszText);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFileNameLabel)
        HRESULT ( STDMETHODCALLTYPE *SetFileNameLabel )( 
            __RPC__in IFileDialog2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszLabel);
        
        DECLSPEC_XFGVIRT(IFileDialog, GetResult)
        HRESULT ( STDMETHODCALLTYPE *GetResult )( 
            __RPC__in IFileDialog2 * This,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsi);
        
        DECLSPEC_XFGVIRT(IFileDialog, AddPlace)
        HRESULT ( STDMETHODCALLTYPE *AddPlace )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ FDAP fdap);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetDefaultExtension)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultExtension )( 
            __RPC__in IFileDialog2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszDefaultExtension);
        
        DECLSPEC_XFGVIRT(IFileDialog, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetClientGuid)
        HRESULT ( STDMETHODCALLTYPE *SetClientGuid )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in REFGUID guid);
        
        DECLSPEC_XFGVIRT(IFileDialog, ClearClientData)
        HRESULT ( STDMETHODCALLTYPE *ClearClientData )( 
            __RPC__in IFileDialog2 * This);
        
        DECLSPEC_XFGVIRT(IFileDialog, SetFilter)
        HRESULT ( STDMETHODCALLTYPE *SetFilter )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IShellItemFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IFileDialog2, SetCancelButtonLabel)
        HRESULT ( STDMETHODCALLTYPE *SetCancelButtonLabel )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in LPCWSTR pszLabel);
        
        DECLSPEC_XFGVIRT(IFileDialog2, SetNavigationRoot)
        HRESULT ( STDMETHODCALLTYPE *SetNavigationRoot )( 
            __RPC__in IFileDialog2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        END_INTERFACE
    } IFileDialog2Vtbl;

    interface IFileDialog2
    {
        CONST_VTBL struct IFileDialog2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileDialog2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileDialog2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileDialog2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileDialog2_Show(This,hwndOwner)	\
    ( (This)->lpVtbl -> Show(This,hwndOwner) ) 


#define IFileDialog2_SetFileTypes(This,cFileTypes,rgFilterSpec)	\
    ( (This)->lpVtbl -> SetFileTypes(This,cFileTypes,rgFilterSpec) ) 

#define IFileDialog2_SetFileTypeIndex(This,iFileType)	\
    ( (This)->lpVtbl -> SetFileTypeIndex(This,iFileType) ) 

#define IFileDialog2_GetFileTypeIndex(This,piFileType)	\
    ( (This)->lpVtbl -> GetFileTypeIndex(This,piFileType) ) 

#define IFileDialog2_Advise(This,pfde,pdwCookie)	\
    ( (This)->lpVtbl -> Advise(This,pfde,pdwCookie) ) 

#define IFileDialog2_Unadvise(This,dwCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwCookie) ) 

#define IFileDialog2_SetOptions(This,fos)	\
    ( (This)->lpVtbl -> SetOptions(This,fos) ) 

#define IFileDialog2_GetOptions(This,pfos)	\
    ( (This)->lpVtbl -> GetOptions(This,pfos) ) 

#define IFileDialog2_SetDefaultFolder(This,psi)	\
    ( (This)->lpVtbl -> SetDefaultFolder(This,psi) ) 

#define IFileDialog2_SetFolder(This,psi)	\
    ( (This)->lpVtbl -> SetFolder(This,psi) ) 

#define IFileDialog2_GetFolder(This,ppsi)	\
    ( (This)->lpVtbl -> GetFolder(This,ppsi) ) 

#define IFileDialog2_GetCurrentSelection(This,ppsi)	\
    ( (This)->lpVtbl -> GetCurrentSelection(This,ppsi) ) 

#define IFileDialog2_SetFileName(This,pszName)	\
    ( (This)->lpVtbl -> SetFileName(This,pszName) ) 

#define IFileDialog2_GetFileName(This,pszName)	\
    ( (This)->lpVtbl -> GetFileName(This,pszName) ) 

#define IFileDialog2_SetTitle(This,pszTitle)	\
    ( (This)->lpVtbl -> SetTitle(This,pszTitle) ) 

#define IFileDialog2_SetOkButtonLabel(This,pszText)	\
    ( (This)->lpVtbl -> SetOkButtonLabel(This,pszText) ) 

#define IFileDialog2_SetFileNameLabel(This,pszLabel)	\
    ( (This)->lpVtbl -> SetFileNameLabel(This,pszLabel) ) 

#define IFileDialog2_GetResult(This,ppsi)	\
    ( (This)->lpVtbl -> GetResult(This,ppsi) ) 

#define IFileDialog2_AddPlace(This,psi,fdap)	\
    ( (This)->lpVtbl -> AddPlace(This,psi,fdap) ) 

#define IFileDialog2_SetDefaultExtension(This,pszDefaultExtension)	\
    ( (This)->lpVtbl -> SetDefaultExtension(This,pszDefaultExtension) ) 

#define IFileDialog2_Close(This,hr)	\
    ( (This)->lpVtbl -> Close(This,hr) ) 

#define IFileDialog2_SetClientGuid(This,guid)	\
    ( (This)->lpVtbl -> SetClientGuid(This,guid) ) 

#define IFileDialog2_ClearClientData(This)	\
    ( (This)->lpVtbl -> ClearClientData(This) ) 

#define IFileDialog2_SetFilter(This,pFilter)	\
    ( (This)->lpVtbl -> SetFilter(This,pFilter) ) 


#define IFileDialog2_SetCancelButtonLabel(This,pszLabel)	\
    ( (This)->lpVtbl -> SetCancelButtonLabel(This,pszLabel) ) 

#define IFileDialog2_SetNavigationRoot(This,psi)	\
    ( (This)->lpVtbl -> SetNavigationRoot(This,psi) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileDialog2_INTERFACE_DEFINED__ */


#ifndef __IApplicationAssociationRegistrationUI_INTERFACE_DEFINED__
#define __IApplicationAssociationRegistrationUI_INTERFACE_DEFINED__

/* interface IApplicationAssociationRegistrationUI */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IApplicationAssociationRegistrationUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f76a169-f994-40ac-8fc8-0959e8874710")
    IApplicationAssociationRegistrationUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LaunchAdvancedAssociationUI( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszAppRegistryName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApplicationAssociationRegistrationUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApplicationAssociationRegistrationUI * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApplicationAssociationRegistrationUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApplicationAssociationRegistrationUI * This);
        
        DECLSPEC_XFGVIRT(IApplicationAssociationRegistrationUI, LaunchAdvancedAssociationUI)
        HRESULT ( STDMETHODCALLTYPE *LaunchAdvancedAssociationUI )( 
            __RPC__in IApplicationAssociationRegistrationUI * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszAppRegistryName);
        
        END_INTERFACE
    } IApplicationAssociationRegistrationUIVtbl;

    interface IApplicationAssociationRegistrationUI
    {
        CONST_VTBL struct IApplicationAssociationRegistrationUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApplicationAssociationRegistrationUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApplicationAssociationRegistrationUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApplicationAssociationRegistrationUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApplicationAssociationRegistrationUI_LaunchAdvancedAssociationUI(This,pszAppRegistryName)	\
    ( (This)->lpVtbl -> LaunchAdvancedAssociationUI(This,pszAppRegistryName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApplicationAssociationRegistrationUI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0036 */
/* [local] */ 

#endif  // NTDDI_VISTA


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0036_v0_0_s_ifspec;

#ifndef __IShellRunDll_INTERFACE_DEFINED__
#define __IShellRunDll_INTERFACE_DEFINED__

/* interface IShellRunDll */
/* [local][ref][object][uuid] */ 


EXTERN_C const IID IID_IShellRunDll;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fce4bde0-4b68-4b80-8e9c-7426315a7388")
    IShellRunDll : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszArgs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellRunDllVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IShellRunDll * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IShellRunDll * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IShellRunDll * This);
        
        DECLSPEC_XFGVIRT(IShellRunDll, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IShellRunDll * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszArgs);
        
        END_INTERFACE
    } IShellRunDllVtbl;

    interface IShellRunDll
    {
        CONST_VTBL struct IShellRunDllVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellRunDll_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellRunDll_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellRunDll_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellRunDll_Run(This,pszArgs)	\
    ( (This)->lpVtbl -> Run(This,pszArgs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellRunDll_INTERFACE_DEFINED__ */


#ifndef __IPreviousVersionsInfo_INTERFACE_DEFINED__
#define __IPreviousVersionsInfo_INTERFACE_DEFINED__

/* interface IPreviousVersionsInfo */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IPreviousVersionsInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76e54780-ad74-48e3-a695-3ba9a0aff10d")
    IPreviousVersionsInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AreSnapshotsAvailable( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszPath,
            /* [annotation][in] */ 
            _In_  BOOL fOkToBeSlow,
            /* [annotation][out] */ 
            _Out_  BOOL *pfAvailable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPreviousVersionsInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPreviousVersionsInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPreviousVersionsInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPreviousVersionsInfo * This);
        
        DECLSPEC_XFGVIRT(IPreviousVersionsInfo, AreSnapshotsAvailable)
        HRESULT ( STDMETHODCALLTYPE *AreSnapshotsAvailable )( 
            IPreviousVersionsInfo * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszPath,
            /* [annotation][in] */ 
            _In_  BOOL fOkToBeSlow,
            /* [annotation][out] */ 
            _Out_  BOOL *pfAvailable);
        
        END_INTERFACE
    } IPreviousVersionsInfoVtbl;

    interface IPreviousVersionsInfo
    {
        CONST_VTBL struct IPreviousVersionsInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPreviousVersionsInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPreviousVersionsInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPreviousVersionsInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPreviousVersionsInfo_AreSnapshotsAvailable(This,pszPath,fOkToBeSlow,pfAvailable)	\
    ( (This)->lpVtbl -> AreSnapshotsAvailable(This,pszPath,fOkToBeSlow,pfAvailable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPreviousVersionsInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0038 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0038_v0_0_s_ifspec;

#ifndef __IUseToBrowseItem_INTERFACE_DEFINED__
#define __IUseToBrowseItem_INTERFACE_DEFINED__

/* interface IUseToBrowseItem */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IUseToBrowseItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05edda5c-98a3-4717-8adb-c5e7da991eb1")
    IUseToBrowseItem : public IRelatedItem
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IUseToBrowseItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUseToBrowseItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUseToBrowseItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUseToBrowseItem * This);
        
        DECLSPEC_XFGVIRT(IRelatedItem, GetItemIDList)
        HRESULT ( STDMETHODCALLTYPE *GetItemIDList )( 
            __RPC__in IUseToBrowseItem * This,
            /* [out] */ __RPC__deref_out_opt PIDLIST_ABSOLUTE *ppidl);
        
        DECLSPEC_XFGVIRT(IRelatedItem, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IUseToBrowseItem * This,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsi);
        
        END_INTERFACE
    } IUseToBrowseItemVtbl;

    interface IUseToBrowseItem
    {
        CONST_VTBL struct IUseToBrowseItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUseToBrowseItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUseToBrowseItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUseToBrowseItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUseToBrowseItem_GetItemIDList(This,ppidl)	\
    ( (This)->lpVtbl -> GetItemIDList(This,ppidl) ) 

#define IUseToBrowseItem_GetItem(This,ppsi)	\
    ( (This)->lpVtbl -> GetItem(This,ppsi) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUseToBrowseItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0039 */
/* [local] */ 

#endif  // NTDDI_VISTA
DEFINE_GUID(SID_SCommandBarState, 0xB99EAA5C, 0x3850, 0x4400, 0xBC, 0x33, 0x2C, 0xE5, 0x34, 0x04, 0x8B, 0xF8);


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0039_v0_0_s_ifspec;

#ifndef __INameSpaceTreeControl2_INTERFACE_DEFINED__
#define __INameSpaceTreeControl2_INTERFACE_DEFINED__

/* interface INameSpaceTreeControl2 */
/* [uuid][object] */ 

typedef /* [v1_enum] */ 
enum NSTCSTYLE2
    {
        NSTCS2_DEFAULT	= 0,
        NSTCS2_INTERRUPTNOTIFICATIONS	= 0x1,
        NSTCS2_SHOWNULLSPACEMENU	= 0x2,
        NSTCS2_DISPLAYPADDING	= 0x4,
        NSTCS2_DISPLAYPINNEDONLY	= 0x8,
        NTSCS2_NOSINGLETONAUTOEXPAND	= 0x10,
        NTSCS2_NEVERINSERTNONENUMERATED	= 0x20
    } 	NSTCSTYLE2;

DEFINE_ENUM_FLAG_OPERATORS(NSTCSTYLE2)

EXTERN_C const IID IID_INameSpaceTreeControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7cc7aed8-290e-49bc-8945-c1401cc9306c")
    INameSpaceTreeControl2 : public INameSpaceTreeControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetControlStyle( 
            /* [in] */ NSTCSTYLE nstcsMask,
            /* [in] */ NSTCSTYLE nstcsStyle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetControlStyle( 
            /* [in] */ NSTCSTYLE nstcsMask,
            /* [out] */ __RPC__out NSTCSTYLE *pnstcsStyle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetControlStyle2( 
            /* [in] */ NSTCSTYLE2 nstcsMask,
            /* [in] */ NSTCSTYLE2 nstcsStyle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetControlStyle2( 
            /* [in] */ NSTCSTYLE2 nstcsMask,
            /* [out] */ __RPC__out NSTCSTYLE2 *pnstcsStyle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INameSpaceTreeControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INameSpaceTreeControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INameSpaceTreeControl2 * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [unique][in] */ __RPC__in_opt RECT *prc,
            /* [in] */ NSTCSTYLE nsctsFlags);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, TreeAdvise)
        HRESULT ( STDMETHODCALLTYPE *TreeAdvise )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, TreeUnadvise)
        HRESULT ( STDMETHODCALLTYPE *TreeUnadvise )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, AppendRoot)
        HRESULT ( STDMETHODCALLTYPE *AppendRoot )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psiRoot,
            /* [in] */ SHCONTF grfEnumFlags,
            /* [in] */ NSTCROOTSTYLE grfRootStyle,
            /* [unique][in] */ __RPC__in_opt IShellItemFilter *pif);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, InsertRoot)
        HRESULT ( STDMETHODCALLTYPE *InsertRoot )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ int iIndex,
            /* [in] */ __RPC__in_opt IShellItem *psiRoot,
            /* [in] */ SHCONTF grfEnumFlags,
            /* [in] */ NSTCROOTSTYLE grfRootStyle,
            /* [unique][in] */ __RPC__in_opt IShellItemFilter *pif);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, RemoveRoot)
        HRESULT ( STDMETHODCALLTYPE *RemoveRoot )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psiRoot);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, RemoveAllRoots)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllRoots )( 
            __RPC__in INameSpaceTreeControl2 * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetRootItems)
        HRESULT ( STDMETHODCALLTYPE *GetRootItems )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [out] */ __RPC__deref_out_opt IShellItemArray **ppsiaRootItems);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, SetItemState)
        HRESULT ( STDMETHODCALLTYPE *SetItemState )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ NSTCITEMSTATE nstcisMask,
            /* [in] */ NSTCITEMSTATE nstcisFlags);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetItemState)
        HRESULT ( STDMETHODCALLTYPE *GetItemState )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ NSTCITEMSTATE nstcisMask,
            /* [out] */ __RPC__out NSTCITEMSTATE *pnstcisFlags);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetSelectedItems)
        HRESULT ( STDMETHODCALLTYPE *GetSelectedItems )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [out] */ __RPC__deref_out_opt IShellItemArray **psiaItems);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetItemCustomState)
        HRESULT ( STDMETHODCALLTYPE *GetItemCustomState )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [out] */ __RPC__out int *piStateNumber);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, SetItemCustomState)
        HRESULT ( STDMETHODCALLTYPE *SetItemCustomState )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ int iStateNumber);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, EnsureItemVisible)
        HRESULT ( STDMETHODCALLTYPE *EnsureItemVisible )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, SetTheme)
        HRESULT ( STDMETHODCALLTYPE *SetTheme )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszTheme);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetNextItem)
        HRESULT ( STDMETHODCALLTYPE *GetNextItem )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [unique][in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ NSTCGNI nstcgi,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsiNext);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, HitTest)
        HRESULT ( STDMETHODCALLTYPE *HitTest )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in POINT *ppt,
            /* [out] */ __RPC__deref_out_opt IShellItem **ppsiOut);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, GetItemRect)
        HRESULT ( STDMETHODCALLTYPE *GetItemRect )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [out] */ __RPC__out RECT *prect);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl, CollapseAll)
        HRESULT ( STDMETHODCALLTYPE *CollapseAll )( 
            __RPC__in INameSpaceTreeControl2 * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl2, SetControlStyle)
        HRESULT ( STDMETHODCALLTYPE *SetControlStyle )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ NSTCSTYLE nstcsMask,
            /* [in] */ NSTCSTYLE nstcsStyle);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl2, GetControlStyle)
        HRESULT ( STDMETHODCALLTYPE *GetControlStyle )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ NSTCSTYLE nstcsMask,
            /* [out] */ __RPC__out NSTCSTYLE *pnstcsStyle);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl2, SetControlStyle2)
        HRESULT ( STDMETHODCALLTYPE *SetControlStyle2 )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ NSTCSTYLE2 nstcsMask,
            /* [in] */ NSTCSTYLE2 nstcsStyle);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControl2, GetControlStyle2)
        HRESULT ( STDMETHODCALLTYPE *GetControlStyle2 )( 
            __RPC__in INameSpaceTreeControl2 * This,
            /* [in] */ NSTCSTYLE2 nstcsMask,
            /* [out] */ __RPC__out NSTCSTYLE2 *pnstcsStyle);
        
        END_INTERFACE
    } INameSpaceTreeControl2Vtbl;

    interface INameSpaceTreeControl2
    {
        CONST_VTBL struct INameSpaceTreeControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INameSpaceTreeControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INameSpaceTreeControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INameSpaceTreeControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INameSpaceTreeControl2_Initialize(This,hwndParent,prc,nsctsFlags)	\
    ( (This)->lpVtbl -> Initialize(This,hwndParent,prc,nsctsFlags) ) 

#define INameSpaceTreeControl2_TreeAdvise(This,punk,pdwCookie)	\
    ( (This)->lpVtbl -> TreeAdvise(This,punk,pdwCookie) ) 

#define INameSpaceTreeControl2_TreeUnadvise(This,dwCookie)	\
    ( (This)->lpVtbl -> TreeUnadvise(This,dwCookie) ) 

#define INameSpaceTreeControl2_AppendRoot(This,psiRoot,grfEnumFlags,grfRootStyle,pif)	\
    ( (This)->lpVtbl -> AppendRoot(This,psiRoot,grfEnumFlags,grfRootStyle,pif) ) 

#define INameSpaceTreeControl2_InsertRoot(This,iIndex,psiRoot,grfEnumFlags,grfRootStyle,pif)	\
    ( (This)->lpVtbl -> InsertRoot(This,iIndex,psiRoot,grfEnumFlags,grfRootStyle,pif) ) 

#define INameSpaceTreeControl2_RemoveRoot(This,psiRoot)	\
    ( (This)->lpVtbl -> RemoveRoot(This,psiRoot) ) 

#define INameSpaceTreeControl2_RemoveAllRoots(This)	\
    ( (This)->lpVtbl -> RemoveAllRoots(This) ) 

#define INameSpaceTreeControl2_GetRootItems(This,ppsiaRootItems)	\
    ( (This)->lpVtbl -> GetRootItems(This,ppsiaRootItems) ) 

#define INameSpaceTreeControl2_SetItemState(This,psi,nstcisMask,nstcisFlags)	\
    ( (This)->lpVtbl -> SetItemState(This,psi,nstcisMask,nstcisFlags) ) 

#define INameSpaceTreeControl2_GetItemState(This,psi,nstcisMask,pnstcisFlags)	\
    ( (This)->lpVtbl -> GetItemState(This,psi,nstcisMask,pnstcisFlags) ) 

#define INameSpaceTreeControl2_GetSelectedItems(This,psiaItems)	\
    ( (This)->lpVtbl -> GetSelectedItems(This,psiaItems) ) 

#define INameSpaceTreeControl2_GetItemCustomState(This,psi,piStateNumber)	\
    ( (This)->lpVtbl -> GetItemCustomState(This,psi,piStateNumber) ) 

#define INameSpaceTreeControl2_SetItemCustomState(This,psi,iStateNumber)	\
    ( (This)->lpVtbl -> SetItemCustomState(This,psi,iStateNumber) ) 

#define INameSpaceTreeControl2_EnsureItemVisible(This,psi)	\
    ( (This)->lpVtbl -> EnsureItemVisible(This,psi) ) 

#define INameSpaceTreeControl2_SetTheme(This,pszTheme)	\
    ( (This)->lpVtbl -> SetTheme(This,pszTheme) ) 

#define INameSpaceTreeControl2_GetNextItem(This,psi,nstcgi,ppsiNext)	\
    ( (This)->lpVtbl -> GetNextItem(This,psi,nstcgi,ppsiNext) ) 

#define INameSpaceTreeControl2_HitTest(This,ppt,ppsiOut)	\
    ( (This)->lpVtbl -> HitTest(This,ppt,ppsiOut) ) 

#define INameSpaceTreeControl2_GetItemRect(This,psi,prect)	\
    ( (This)->lpVtbl -> GetItemRect(This,psi,prect) ) 

#define INameSpaceTreeControl2_CollapseAll(This)	\
    ( (This)->lpVtbl -> CollapseAll(This) ) 


#define INameSpaceTreeControl2_SetControlStyle(This,nstcsMask,nstcsStyle)	\
    ( (This)->lpVtbl -> SetControlStyle(This,nstcsMask,nstcsStyle) ) 

#define INameSpaceTreeControl2_GetControlStyle(This,nstcsMask,pnstcsStyle)	\
    ( (This)->lpVtbl -> GetControlStyle(This,nstcsMask,pnstcsStyle) ) 

#define INameSpaceTreeControl2_SetControlStyle2(This,nstcsMask,nstcsStyle)	\
    ( (This)->lpVtbl -> SetControlStyle2(This,nstcsMask,nstcsStyle) ) 

#define INameSpaceTreeControl2_GetControlStyle2(This,nstcsMask,pnstcsStyle)	\
    ( (This)->lpVtbl -> GetControlStyle2(This,nstcsMask,pnstcsStyle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INameSpaceTreeControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0040 */
/* [local] */ 

#define NSTCS2_ALLMASK (NSTCS2_INTERRUPTNOTIFICATIONS | NSTCS2_SHOWNULLSPACEMENU | NSTCS2_DISPLAYPADDING)
#define ISLBUTTON(x) (NSTCECT_LBUTTON == ((x) & NSTCECT_BUTTON))
#define ISMBUTTON(x) (NSTCECT_MBUTTON == ((x) & NSTCECT_BUTTON))
#define ISRBUTTON(x) (NSTCECT_RBUTTON == ((x) & NSTCECT_BUTTON))
#define ISDBLCLICK(x) (NSTCECT_DBLCLICK == ((x) & NSTCECT_DBLCLICK))


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0040_v0_0_s_ifspec;

#ifndef __INameSpaceTreeControlEvents_INTERFACE_DEFINED__
#define __INameSpaceTreeControlEvents_INTERFACE_DEFINED__

/* interface INameSpaceTreeControlEvents */
/* [local][uuid][object] */ 

/* [v1_enum] */ 
enum _NSTCEHITTEST
    {
        NSTCEHT_NOWHERE	= 0x1,
        NSTCEHT_ONITEMICON	= 0x2,
        NSTCEHT_ONITEMLABEL	= 0x4,
        NSTCEHT_ONITEMINDENT	= 0x8,
        NSTCEHT_ONITEMBUTTON	= 0x10,
        NSTCEHT_ONITEMRIGHT	= 0x20,
        NSTCEHT_ONITEMSTATEICON	= 0x40,
        NSTCEHT_ONITEM	= 0x46,
        NSTCEHT_ONITEMTABBUTTON	= 0x1000
    } ;
typedef DWORD NSTCEHITTEST;

/* [v1_enum] */ 
enum _NSTCECLICKTYPE
    {
        NSTCECT_LBUTTON	= 0x1,
        NSTCECT_MBUTTON	= 0x2,
        NSTCECT_RBUTTON	= 0x3,
        NSTCECT_BUTTON	= 0x3,
        NSTCECT_DBLCLICK	= 0x4
    } ;
typedef DWORD NSTCECLICKTYPE;


EXTERN_C const IID IID_INameSpaceTreeControlEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("93D77985-B3D8-4484-8318-672CDDA002CE")
    INameSpaceTreeControlEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnItemClick( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCEHITTEST nstceHitTest,
            /* [annotation][in] */ 
            _In_  NSTCECLICKTYPE nstceClickType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPropertyItemCommit( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnItemStateChanging( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisMask,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnItemStateChanged( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisMask,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSelectionChanged( 
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKeyboardInput( 
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeforeExpand( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAfterExpand( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeginLabelEdit( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndLabelEdit( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnGetToolTip( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][size_is][string][out] */ 
            _Out_writes_(cchTip)  LPWSTR pszTip,
            /* [annotation][in] */ 
            _In_  int cchTip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeforeItemDelete( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnItemAdded( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  BOOL fIsRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnItemDeleted( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  BOOL fIsRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeforeContextMenu( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAfterContextMenu( 
            /* [annotation][in] */ 
            _In_opt_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  IContextMenu *pcmIn,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeforeStateImageChange( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnGetDefaultIconIndex( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Out_  int *piDefaultIcon,
            /* [annotation][out] */ 
            _Out_  int *piOpenIcon) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INameSpaceTreeControlEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INameSpaceTreeControlEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INameSpaceTreeControlEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INameSpaceTreeControlEvents * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnItemClick)
        HRESULT ( STDMETHODCALLTYPE *OnItemClick )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCEHITTEST nstceHitTest,
            /* [annotation][in] */ 
            _In_  NSTCECLICKTYPE nstceClickType);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnPropertyItemCommit)
        HRESULT ( STDMETHODCALLTYPE *OnPropertyItemCommit )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnItemStateChanging)
        HRESULT ( STDMETHODCALLTYPE *OnItemStateChanging )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisMask,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisState);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnItemStateChanged)
        HRESULT ( STDMETHODCALLTYPE *OnItemStateChanged )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisMask,
            /* [annotation][in] */ 
            _In_  NSTCITEMSTATE nstcisState);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnSelectionChanged)
        HRESULT ( STDMETHODCALLTYPE *OnSelectionChanged )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaSelection);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnKeyboardInput)
        HRESULT ( STDMETHODCALLTYPE *OnKeyboardInput )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnBeforeExpand)
        HRESULT ( STDMETHODCALLTYPE *OnBeforeExpand )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnAfterExpand)
        HRESULT ( STDMETHODCALLTYPE *OnAfterExpand )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnBeginLabelEdit)
        HRESULT ( STDMETHODCALLTYPE *OnBeginLabelEdit )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnEndLabelEdit)
        HRESULT ( STDMETHODCALLTYPE *OnEndLabelEdit )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnGetToolTip)
        HRESULT ( STDMETHODCALLTYPE *OnGetToolTip )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][size_is][string][out] */ 
            _Out_writes_(cchTip)  LPWSTR pszTip,
            /* [annotation][in] */ 
            _In_  int cchTip);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnBeforeItemDelete)
        HRESULT ( STDMETHODCALLTYPE *OnBeforeItemDelete )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnItemAdded)
        HRESULT ( STDMETHODCALLTYPE *OnItemAdded )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  BOOL fIsRoot);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnItemDeleted)
        HRESULT ( STDMETHODCALLTYPE *OnItemDeleted )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  BOOL fIsRoot);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnBeforeContextMenu)
        HRESULT ( STDMETHODCALLTYPE *OnBeforeContextMenu )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnAfterContextMenu)
        HRESULT ( STDMETHODCALLTYPE *OnAfterContextMenu )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_opt_  IShellItem *psi,
            /* [annotation][in] */ 
            _In_  IContextMenu *pcmIn,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnBeforeStateImageChange)
        HRESULT ( STDMETHODCALLTYPE *OnBeforeStateImageChange )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlEvents, OnGetDefaultIconIndex)
        HRESULT ( STDMETHODCALLTYPE *OnGetDefaultIconIndex )( 
            INameSpaceTreeControlEvents * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Out_  int *piDefaultIcon,
            /* [annotation][out] */ 
            _Out_  int *piOpenIcon);
        
        END_INTERFACE
    } INameSpaceTreeControlEventsVtbl;

    interface INameSpaceTreeControlEvents
    {
        CONST_VTBL struct INameSpaceTreeControlEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INameSpaceTreeControlEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INameSpaceTreeControlEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INameSpaceTreeControlEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INameSpaceTreeControlEvents_OnItemClick(This,psi,nstceHitTest,nstceClickType)	\
    ( (This)->lpVtbl -> OnItemClick(This,psi,nstceHitTest,nstceClickType) ) 

#define INameSpaceTreeControlEvents_OnPropertyItemCommit(This,psi)	\
    ( (This)->lpVtbl -> OnPropertyItemCommit(This,psi) ) 

#define INameSpaceTreeControlEvents_OnItemStateChanging(This,psi,nstcisMask,nstcisState)	\
    ( (This)->lpVtbl -> OnItemStateChanging(This,psi,nstcisMask,nstcisState) ) 

#define INameSpaceTreeControlEvents_OnItemStateChanged(This,psi,nstcisMask,nstcisState)	\
    ( (This)->lpVtbl -> OnItemStateChanged(This,psi,nstcisMask,nstcisState) ) 

#define INameSpaceTreeControlEvents_OnSelectionChanged(This,psiaSelection)	\
    ( (This)->lpVtbl -> OnSelectionChanged(This,psiaSelection) ) 

#define INameSpaceTreeControlEvents_OnKeyboardInput(This,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> OnKeyboardInput(This,uMsg,wParam,lParam) ) 

#define INameSpaceTreeControlEvents_OnBeforeExpand(This,psi)	\
    ( (This)->lpVtbl -> OnBeforeExpand(This,psi) ) 

#define INameSpaceTreeControlEvents_OnAfterExpand(This,psi)	\
    ( (This)->lpVtbl -> OnAfterExpand(This,psi) ) 

#define INameSpaceTreeControlEvents_OnBeginLabelEdit(This,psi)	\
    ( (This)->lpVtbl -> OnBeginLabelEdit(This,psi) ) 

#define INameSpaceTreeControlEvents_OnEndLabelEdit(This,psi)	\
    ( (This)->lpVtbl -> OnEndLabelEdit(This,psi) ) 

#define INameSpaceTreeControlEvents_OnGetToolTip(This,psi,pszTip,cchTip)	\
    ( (This)->lpVtbl -> OnGetToolTip(This,psi,pszTip,cchTip) ) 

#define INameSpaceTreeControlEvents_OnBeforeItemDelete(This,psi)	\
    ( (This)->lpVtbl -> OnBeforeItemDelete(This,psi) ) 

#define INameSpaceTreeControlEvents_OnItemAdded(This,psi,fIsRoot)	\
    ( (This)->lpVtbl -> OnItemAdded(This,psi,fIsRoot) ) 

#define INameSpaceTreeControlEvents_OnItemDeleted(This,psi,fIsRoot)	\
    ( (This)->lpVtbl -> OnItemDeleted(This,psi,fIsRoot) ) 

#define INameSpaceTreeControlEvents_OnBeforeContextMenu(This,psi,riid,ppv)	\
    ( (This)->lpVtbl -> OnBeforeContextMenu(This,psi,riid,ppv) ) 

#define INameSpaceTreeControlEvents_OnAfterContextMenu(This,psi,pcmIn,riid,ppv)	\
    ( (This)->lpVtbl -> OnAfterContextMenu(This,psi,pcmIn,riid,ppv) ) 

#define INameSpaceTreeControlEvents_OnBeforeStateImageChange(This,psi)	\
    ( (This)->lpVtbl -> OnBeforeStateImageChange(This,psi) ) 

#define INameSpaceTreeControlEvents_OnGetDefaultIconIndex(This,psi,piDefaultIcon,piOpenIcon)	\
    ( (This)->lpVtbl -> OnGetDefaultIconIndex(This,psi,piDefaultIcon,piOpenIcon) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INameSpaceTreeControlEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0041 */
/* [local] */ 

#define NSTCDHPOS_ONTOP  -1


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0041_v0_0_s_ifspec;

#ifndef __INameSpaceTreeControlDropHandler_INTERFACE_DEFINED__
#define __INameSpaceTreeControlDropHandler_INTERFACE_DEFINED__

/* interface INameSpaceTreeControlDropHandler */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_INameSpaceTreeControlDropHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9C665D6-C2F2-4c19-BF33-8322D7352F51")
    INameSpaceTreeControlDropHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDragEnter( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  BOOL fOutsideSource,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDragOver( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDragPosition( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iNewPosition,
            /* [annotation][in] */ 
            _In_  int iOldPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDrop( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iPosition,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDropPosition( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iNewPosition,
            /* [annotation][in] */ 
            _In_  int iOldPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDragLeave( 
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INameSpaceTreeControlDropHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INameSpaceTreeControlDropHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INameSpaceTreeControlDropHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INameSpaceTreeControlDropHandler * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDragEnter)
        HRESULT ( STDMETHODCALLTYPE *OnDragEnter )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  BOOL fOutsideSource,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDragOver)
        HRESULT ( STDMETHODCALLTYPE *OnDragOver )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDragPosition)
        HRESULT ( STDMETHODCALLTYPE *OnDragPosition )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iNewPosition,
            /* [annotation][in] */ 
            _In_  int iOldPosition);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDrop)
        HRESULT ( STDMETHODCALLTYPE *OnDrop )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iPosition,
            /* [annotation][in] */ 
            _In_  DWORD grfKeyState,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwEffect);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDropPosition)
        HRESULT ( STDMETHODCALLTYPE *OnDropPosition )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver,
            /* [annotation][in] */ 
            _In_  IShellItemArray *psiaData,
            /* [annotation][in] */ 
            _In_  int iNewPosition,
            /* [annotation][in] */ 
            _In_  int iOldPosition);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlDropHandler, OnDragLeave)
        HRESULT ( STDMETHODCALLTYPE *OnDragLeave )( 
            INameSpaceTreeControlDropHandler * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellItem *psiOver);
        
        END_INTERFACE
    } INameSpaceTreeControlDropHandlerVtbl;

    interface INameSpaceTreeControlDropHandler
    {
        CONST_VTBL struct INameSpaceTreeControlDropHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INameSpaceTreeControlDropHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INameSpaceTreeControlDropHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INameSpaceTreeControlDropHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INameSpaceTreeControlDropHandler_OnDragEnter(This,psiOver,psiaData,fOutsideSource,grfKeyState,pdwEffect)	\
    ( (This)->lpVtbl -> OnDragEnter(This,psiOver,psiaData,fOutsideSource,grfKeyState,pdwEffect) ) 

#define INameSpaceTreeControlDropHandler_OnDragOver(This,psiOver,psiaData,grfKeyState,pdwEffect)	\
    ( (This)->lpVtbl -> OnDragOver(This,psiOver,psiaData,grfKeyState,pdwEffect) ) 

#define INameSpaceTreeControlDropHandler_OnDragPosition(This,psiOver,psiaData,iNewPosition,iOldPosition)	\
    ( (This)->lpVtbl -> OnDragPosition(This,psiOver,psiaData,iNewPosition,iOldPosition) ) 

#define INameSpaceTreeControlDropHandler_OnDrop(This,psiOver,psiaData,iPosition,grfKeyState,pdwEffect)	\
    ( (This)->lpVtbl -> OnDrop(This,psiOver,psiaData,iPosition,grfKeyState,pdwEffect) ) 

#define INameSpaceTreeControlDropHandler_OnDropPosition(This,psiOver,psiaData,iNewPosition,iOldPosition)	\
    ( (This)->lpVtbl -> OnDropPosition(This,psiOver,psiaData,iNewPosition,iOldPosition) ) 

#define INameSpaceTreeControlDropHandler_OnDragLeave(This,psiOver)	\
    ( (This)->lpVtbl -> OnDragLeave(This,psiOver) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INameSpaceTreeControlDropHandler_INTERFACE_DEFINED__ */


#ifndef __INameSpaceTreeAccessible_INTERFACE_DEFINED__
#define __INameSpaceTreeAccessible_INTERFACE_DEFINED__

/* interface INameSpaceTreeAccessible */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_INameSpaceTreeAccessible;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71f312de-43ed-4190-8477-e9536b82350b")
    INameSpaceTreeAccessible : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnGetDefaultAccessibilityAction( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrDefaultAction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDoDefaultAccessibilityAction( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnGetAccessibilityRole( 
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarRole) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INameSpaceTreeAccessibleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INameSpaceTreeAccessible * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INameSpaceTreeAccessible * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INameSpaceTreeAccessible * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeAccessible, OnGetDefaultAccessibilityAction)
        HRESULT ( STDMETHODCALLTYPE *OnGetDefaultAccessibilityAction )( 
            INameSpaceTreeAccessible * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrDefaultAction);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeAccessible, OnDoDefaultAccessibilityAction)
        HRESULT ( STDMETHODCALLTYPE *OnDoDefaultAccessibilityAction )( 
            INameSpaceTreeAccessible * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeAccessible, OnGetAccessibilityRole)
        HRESULT ( STDMETHODCALLTYPE *OnGetAccessibilityRole )( 
            INameSpaceTreeAccessible * This,
            /* [annotation][in] */ 
            _In_  IShellItem *psi,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarRole);
        
        END_INTERFACE
    } INameSpaceTreeAccessibleVtbl;

    interface INameSpaceTreeAccessible
    {
        CONST_VTBL struct INameSpaceTreeAccessibleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INameSpaceTreeAccessible_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INameSpaceTreeAccessible_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INameSpaceTreeAccessible_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INameSpaceTreeAccessible_OnGetDefaultAccessibilityAction(This,psi,pbstrDefaultAction)	\
    ( (This)->lpVtbl -> OnGetDefaultAccessibilityAction(This,psi,pbstrDefaultAction) ) 

#define INameSpaceTreeAccessible_OnDoDefaultAccessibilityAction(This,psi)	\
    ( (This)->lpVtbl -> OnDoDefaultAccessibilityAction(This,psi) ) 

#define INameSpaceTreeAccessible_OnGetAccessibilityRole(This,psi,pvarRole)	\
    ( (This)->lpVtbl -> OnGetAccessibilityRole(This,psi,pvarRole) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INameSpaceTreeAccessible_INTERFACE_DEFINED__ */


#ifndef __INameSpaceTreeControlCustomDraw_INTERFACE_DEFINED__
#define __INameSpaceTreeControlCustomDraw_INTERFACE_DEFINED__

/* interface INameSpaceTreeControlCustomDraw */
/* [local][uuid][object] */ 

typedef struct NSTCCUSTOMDRAW
    {
    IShellItem *psi;
    UINT uItemState;
    NSTCITEMSTATE nstcis;
    LPCWSTR pszText;
    int iImage;
    HIMAGELIST himl;
    int iLevel;
    int iIndent;
    } 	NSTCCUSTOMDRAW;


EXTERN_C const IID IID_INameSpaceTreeControlCustomDraw;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D3BA758-33EE-42d5-BB7B-5F3431D86C78")
    INameSpaceTreeControlCustomDraw : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PrePaint( 
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][out] */ 
            _Out_  LRESULT *plres) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PostPaint( 
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ItemPrePaint( 
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][in] */ 
            _In_  NSTCCUSTOMDRAW *pnstccdItem,
            /* [annotation][out][in] */ 
            _Inout_  COLORREF *pclrText,
            /* [annotation][out][in] */ 
            _Inout_  COLORREF *pclrTextBk,
            /* [annotation][out] */ 
            _Out_  LRESULT *plres) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ItemPostPaint( 
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][in] */ 
            _In_  NSTCCUSTOMDRAW *pnstccdItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INameSpaceTreeControlCustomDrawVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INameSpaceTreeControlCustomDraw * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INameSpaceTreeControlCustomDraw * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INameSpaceTreeControlCustomDraw * This);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlCustomDraw, PrePaint)
        HRESULT ( STDMETHODCALLTYPE *PrePaint )( 
            INameSpaceTreeControlCustomDraw * This,
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][out] */ 
            _Out_  LRESULT *plres);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlCustomDraw, PostPaint)
        HRESULT ( STDMETHODCALLTYPE *PostPaint )( 
            INameSpaceTreeControlCustomDraw * This,
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlCustomDraw, ItemPrePaint)
        HRESULT ( STDMETHODCALLTYPE *ItemPrePaint )( 
            INameSpaceTreeControlCustomDraw * This,
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][in] */ 
            _In_  NSTCCUSTOMDRAW *pnstccdItem,
            /* [annotation][out][in] */ 
            _Inout_  COLORREF *pclrText,
            /* [annotation][out][in] */ 
            _Inout_  COLORREF *pclrTextBk,
            /* [annotation][out] */ 
            _Out_  LRESULT *plres);
        
        DECLSPEC_XFGVIRT(INameSpaceTreeControlCustomDraw, ItemPostPaint)
        HRESULT ( STDMETHODCALLTYPE *ItemPostPaint )( 
            INameSpaceTreeControlCustomDraw * This,
            /* [annotation][in] */ 
            _In_  HDC hdc,
            /* [annotation][in] */ 
            _In_  RECT *prc,
            /* [annotation][in] */ 
            _In_  NSTCCUSTOMDRAW *pnstccdItem);
        
        END_INTERFACE
    } INameSpaceTreeControlCustomDrawVtbl;

    interface INameSpaceTreeControlCustomDraw
    {
        CONST_VTBL struct INameSpaceTreeControlCustomDrawVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INameSpaceTreeControlCustomDraw_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INameSpaceTreeControlCustomDraw_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INameSpaceTreeControlCustomDraw_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INameSpaceTreeControlCustomDraw_PrePaint(This,hdc,prc,plres)	\
    ( (This)->lpVtbl -> PrePaint(This,hdc,prc,plres) ) 

#define INameSpaceTreeControlCustomDraw_PostPaint(This,hdc,prc)	\
    ( (This)->lpVtbl -> PostPaint(This,hdc,prc) ) 

#define INameSpaceTreeControlCustomDraw_ItemPrePaint(This,hdc,prc,pnstccdItem,pclrText,pclrTextBk,plres)	\
    ( (This)->lpVtbl -> ItemPrePaint(This,hdc,prc,pnstccdItem,pclrText,pclrTextBk,plres) ) 

#define INameSpaceTreeControlCustomDraw_ItemPostPaint(This,hdc,prc,pnstccdItem)	\
    ( (This)->lpVtbl -> ItemPostPaint(This,hdc,prc,pnstccdItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INameSpaceTreeControlCustomDraw_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0044 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0044_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0044_v0_0_s_ifspec;

#ifndef __ITrayDeskBand_INTERFACE_DEFINED__
#define __ITrayDeskBand_INTERFACE_DEFINED__

/* interface ITrayDeskBand */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITrayDeskBand;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D67E846-5B9C-4db8-9CBC-DDE12F4254F1")
    ITrayDeskBand : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowDeskBand( 
            /* [in] */ __RPC__in REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HideDeskBand( 
            /* [in] */ __RPC__in REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDeskBandShown( 
            /* [in] */ __RPC__in REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeskBandRegistrationChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITrayDeskBandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITrayDeskBand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITrayDeskBand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITrayDeskBand * This);
        
        DECLSPEC_XFGVIRT(ITrayDeskBand, ShowDeskBand)
        HRESULT ( STDMETHODCALLTYPE *ShowDeskBand )( 
            __RPC__in ITrayDeskBand * This,
            /* [in] */ __RPC__in REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(ITrayDeskBand, HideDeskBand)
        HRESULT ( STDMETHODCALLTYPE *HideDeskBand )( 
            __RPC__in ITrayDeskBand * This,
            /* [in] */ __RPC__in REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(ITrayDeskBand, IsDeskBandShown)
        HRESULT ( STDMETHODCALLTYPE *IsDeskBandShown )( 
            __RPC__in ITrayDeskBand * This,
            /* [in] */ __RPC__in REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(ITrayDeskBand, DeskBandRegistrationChanged)
        HRESULT ( STDMETHODCALLTYPE *DeskBandRegistrationChanged )( 
            __RPC__in ITrayDeskBand * This);
        
        END_INTERFACE
    } ITrayDeskBandVtbl;

    interface ITrayDeskBand
    {
        CONST_VTBL struct ITrayDeskBandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITrayDeskBand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITrayDeskBand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITrayDeskBand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITrayDeskBand_ShowDeskBand(This,clsid)	\
    ( (This)->lpVtbl -> ShowDeskBand(This,clsid) ) 

#define ITrayDeskBand_HideDeskBand(This,clsid)	\
    ( (This)->lpVtbl -> HideDeskBand(This,clsid) ) 

#define ITrayDeskBand_IsDeskBandShown(This,clsid)	\
    ( (This)->lpVtbl -> IsDeskBandShown(This,clsid) ) 

#define ITrayDeskBand_DeskBandRegistrationChanged(This)	\
    ( (This)->lpVtbl -> DeskBandRegistrationChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITrayDeskBand_INTERFACE_DEFINED__ */


#ifndef __IBandHost_INTERFACE_DEFINED__
#define __IBandHost_INTERFACE_DEFINED__

/* interface IBandHost */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IBandHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B9075C7C-D48E-403f-AB99-D6C77A1084AC")
    IBandHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateBand( 
            /* [in] */ __RPC__in REFCLSID rclsidBand,
            /* [in] */ BOOL fAvailable,
            /* [in] */ BOOL fVisible,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBandAvailability( 
            /* [in] */ __RPC__in REFCLSID rclsidBand,
            /* [in] */ BOOL fAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyBand( 
            /* [in] */ __RPC__in REFCLSID rclsidBand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBandHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBandHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBandHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBandHost * This);
        
        DECLSPEC_XFGVIRT(IBandHost, CreateBand)
        HRESULT ( STDMETHODCALLTYPE *CreateBand )( 
            __RPC__in IBandHost * This,
            /* [in] */ __RPC__in REFCLSID rclsidBand,
            /* [in] */ BOOL fAvailable,
            /* [in] */ BOOL fVisible,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IBandHost, SetBandAvailability)
        HRESULT ( STDMETHODCALLTYPE *SetBandAvailability )( 
            __RPC__in IBandHost * This,
            /* [in] */ __RPC__in REFCLSID rclsidBand,
            /* [in] */ BOOL fAvailable);
        
        DECLSPEC_XFGVIRT(IBandHost, DestroyBand)
        HRESULT ( STDMETHODCALLTYPE *DestroyBand )( 
            __RPC__in IBandHost * This,
            /* [in] */ __RPC__in REFCLSID rclsidBand);
        
        END_INTERFACE
    } IBandHostVtbl;

    interface IBandHost
    {
        CONST_VTBL struct IBandHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBandHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBandHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBandHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBandHost_CreateBand(This,rclsidBand,fAvailable,fVisible,riid,ppv)	\
    ( (This)->lpVtbl -> CreateBand(This,rclsidBand,fAvailable,fVisible,riid,ppv) ) 

#define IBandHost_SetBandAvailability(This,rclsidBand,fAvailable)	\
    ( (This)->lpVtbl -> SetBandAvailability(This,rclsidBand,fAvailable) ) 

#define IBandHost_DestroyBand(This,rclsidBand)	\
    ( (This)->lpVtbl -> DestroyBand(This,rclsidBand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBandHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0046 */
/* [local] */ 

#define SID_SBandHost IID_IBandHost
#endif  // NTDDI_VISTA

extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0048_v0_0_s_ifspec;

#ifndef __IComputerInfoChangeNotify_INTERFACE_DEFINED__
#define __IComputerInfoChangeNotify_INTERFACE_DEFINED__

/* interface IComputerInfoChangeNotify */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IComputerInfoChangeNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0DF60D92-6818-46d6-B358-D66170DDE466")
    IComputerInfoChangeNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ComputerInfoChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComputerInfoChangeNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComputerInfoChangeNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComputerInfoChangeNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComputerInfoChangeNotify * This);
        
        DECLSPEC_XFGVIRT(IComputerInfoChangeNotify, ComputerInfoChanged)
        HRESULT ( STDMETHODCALLTYPE *ComputerInfoChanged )( 
            __RPC__in IComputerInfoChangeNotify * This);
        
        END_INTERFACE
    } IComputerInfoChangeNotifyVtbl;

    interface IComputerInfoChangeNotify
    {
        CONST_VTBL struct IComputerInfoChangeNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComputerInfoChangeNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComputerInfoChangeNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComputerInfoChangeNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComputerInfoChangeNotify_ComputerInfoChanged(This)	\
    ( (This)->lpVtbl -> ComputerInfoChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComputerInfoChangeNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0049 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN7)
#endif // NTDDI_WIN7


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0049_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0049_v0_0_s_ifspec;

#ifndef __IDesktopGadget_INTERFACE_DEFINED__
#define __IDesktopGadget_INTERFACE_DEFINED__

/* interface IDesktopGadget */
/* [uuid][object] */ 


EXTERN_C const IID IID_IDesktopGadget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c1646bc4-f298-4f91-a204-eb2dd1709d1a")
    IDesktopGadget : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RunGadget( 
            /* [in] */ __RPC__in LPCWSTR gadgetPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDesktopGadgetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDesktopGadget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDesktopGadget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDesktopGadget * This);
        
        DECLSPEC_XFGVIRT(IDesktopGadget, RunGadget)
        HRESULT ( STDMETHODCALLTYPE *RunGadget )( 
            __RPC__in IDesktopGadget * This,
            /* [in] */ __RPC__in LPCWSTR gadgetPath);
        
        END_INTERFACE
    } IDesktopGadgetVtbl;

    interface IDesktopGadget
    {
        CONST_VTBL struct IDesktopGadgetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDesktopGadget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDesktopGadget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDesktopGadget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDesktopGadget_RunGadget(This,gadgetPath)	\
    ( (This)->lpVtbl -> RunGadget(This,gadgetPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDesktopGadget_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0050 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#endif // NTDDI_WINTHRESHOLD


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0050_v0_0_s_ifspec;


#ifndef __ShellObjects_LIBRARY_DEFINED__
#define __ShellObjects_LIBRARY_DEFINED__

/* library ShellObjects */
/* [version][lcid][uuid] */ 

#define SID_PublishingWizard CLSID_PublishingWizard

EXTERN_C const IID LIBID_ShellObjects;

EXTERN_C const CLSID CLSID_QueryCancelAutoPlay;

#ifdef __cplusplus

class DECLSPEC_UUID("331F1768-05A9-4ddd-B86E-DAE34DDC998A")
QueryCancelAutoPlay;
#endif

EXTERN_C const CLSID CLSID_TimeCategorizer;

#ifdef __cplusplus

class DECLSPEC_UUID("3bb4118f-ddfd-4d30-a348-9fb5d6bf1afe")
TimeCategorizer;
#endif

EXTERN_C const CLSID CLSID_AlphabeticalCategorizer;

#ifdef __cplusplus

class DECLSPEC_UUID("3c2654c6-7372-4f6b-b310-55d6128f49d2")
AlphabeticalCategorizer;
#endif

EXTERN_C const CLSID CLSID_MergedCategorizer;

#ifdef __cplusplus

class DECLSPEC_UUID("8e827c11-33e7-4bc1-b242-8cd9a1c2b304")
MergedCategorizer;
#endif

EXTERN_C const CLSID CLSID_ImageProperties;

#ifdef __cplusplus

class DECLSPEC_UUID("7ab770c7-0e23-4d7a-8aa2-19bfad479829")
ImageProperties;
#endif

EXTERN_C const CLSID CLSID_CDBurn;

#ifdef __cplusplus

class DECLSPEC_UUID("fbeb8a05-beee-4442-804e-409d6c4515e9")
CDBurn;
#endif

EXTERN_C const CLSID CLSID_StartMenuPin;

#ifdef __cplusplus

class DECLSPEC_UUID("a2a9545d-a0c2-42b4-9708-a0b2badd77c8")
StartMenuPin;
#endif

EXTERN_C const CLSID CLSID_WebWizardHost;

#ifdef __cplusplus

class DECLSPEC_UUID("c827f149-55c1-4d28-935e-57e47caed973")
WebWizardHost;
#endif

EXTERN_C const CLSID CLSID_PublishDropTarget;

#ifdef __cplusplus

class DECLSPEC_UUID("CC6EEFFB-43F6-46c5-9619-51D571967F7D")
PublishDropTarget;
#endif

EXTERN_C const CLSID CLSID_PublishingWizard;

#ifdef __cplusplus

class DECLSPEC_UUID("6b33163c-76a5-4b6c-bf21-45de9cd503a1")
PublishingWizard;
#endif

EXTERN_C const CLSID CLSID_InternetPrintOrdering;

#ifdef __cplusplus

class DECLSPEC_UUID("add36aa8-751a-4579-a266-d66f5202ccbb")
InternetPrintOrdering;
#endif

EXTERN_C const CLSID CLSID_FolderViewHost;

#ifdef __cplusplus

class DECLSPEC_UUID("20b1cb23-6968-4eb9-b7d4-a66d00d07cee")
FolderViewHost;
#endif

EXTERN_C const CLSID CLSID_ExplorerBrowser;

#ifdef __cplusplus

class DECLSPEC_UUID("71f96385-ddd6-48d3-a0c1-ae06e8b055fb")
ExplorerBrowser;
#endif

EXTERN_C const CLSID CLSID_ImageRecompress;

#ifdef __cplusplus

class DECLSPEC_UUID("6e33091c-d2f8-4740-b55e-2e11d1477a2c")
ImageRecompress;
#endif

EXTERN_C const CLSID CLSID_TrayBandSiteService;

#ifdef __cplusplus

class DECLSPEC_UUID("F60AD0A0-E5E1-45cb-B51A-E15B9F8B2934")
TrayBandSiteService;
#endif

EXTERN_C const CLSID CLSID_TrayDeskBand;

#ifdef __cplusplus

class DECLSPEC_UUID("E6442437-6C68-4f52-94DD-2CFED267EFB9")
TrayDeskBand;
#endif

EXTERN_C const CLSID CLSID_AttachmentServices;

#ifdef __cplusplus

class DECLSPEC_UUID("4125dd96-e03a-4103-8f70-e0597d803b9c")
AttachmentServices;
#endif

EXTERN_C const CLSID CLSID_DocPropShellExtension;

#ifdef __cplusplus

class DECLSPEC_UUID("883373C3-BF89-11D1-BE35-080036B11A03")
DocPropShellExtension;
#endif

EXTERN_C const CLSID CLSID_FSCopyHandler;

#ifdef __cplusplus

class DECLSPEC_UUID("D197380A-0A79-4dc8-A033-ED882C2FA14B")
FSCopyHandler;
#endif

EXTERN_C const CLSID CLSID_PreviousVersions;

#ifdef __cplusplus

class DECLSPEC_UUID("596AB062-B4D2-4215-9F74-E9109B0A8153")
PreviousVersions;
#endif

EXTERN_C const CLSID CLSID_NamespaceTreeControl;

#ifdef __cplusplus

class DECLSPEC_UUID("AE054212-3535-4430-83ED-D501AA6680E6")
NamespaceTreeControl;
#endif

EXTERN_C const CLSID CLSID_IENamespaceTreeControl;

#ifdef __cplusplus

class DECLSPEC_UUID("ACE52D03-E5CD-4b20-82FF-E71B11BEAE1D")
IENamespaceTreeControl;
#endif

EXTERN_C const CLSID CLSID_ApplicationAssociationRegistrationUI;

#ifdef __cplusplus

class DECLSPEC_UUID("1968106d-f3b5-44cf-890e-116fcb9ecef1")
ApplicationAssociationRegistrationUI;
#endif

EXTERN_C const CLSID CLSID_DesktopGadget;

#ifdef __cplusplus

class DECLSPEC_UUID("924ccc1b-6562-4c85-8657-d177925222b6")
DesktopGadget;
#endif

EXTERN_C const CLSID CLSID_AccessibilityDockingService;

#ifdef __cplusplus

class DECLSPEC_UUID("29CE1D46-B481-4AA0-A08A-D3EBC8ACA402")
AccessibilityDockingService;
#endif

EXTERN_C const CLSID CLSID_ExecuteFolder;

#ifdef __cplusplus

class DECLSPEC_UUID("11dbb47c-a525-400b-9e80-a54615a090c0")
ExecuteFolder;
#endif

EXTERN_C const CLSID CLSID_VirtualDesktopManager;

#ifdef __cplusplus

class DECLSPEC_UUID("aa509086-5ca9-4c25-8f95-589d3c07b48a")
VirtualDesktopManager;
#endif

EXTERN_C const CLSID CLSID_StorageProviderBanners;

#ifdef __cplusplus

class DECLSPEC_UUID("7CCDF9F4-E576-455A-8BC7-F6EC68D6F063")
StorageProviderBanners;
#endif
#endif /* __ShellObjects_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_shobjidl_0000_0051 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN7)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#if defined(__cplusplus) && !defined(CINTERFACE)

// These functions properly initialize their _Outptr_ parameters to NULL, and only return NULL
// on failure, but /analyze can't presently distinguish the failure case from the success case, and
// throws warning C6387 anyway.  Thus, the warning is disabled to avoid generating noise for code
// that includes shobjidl.h and compiles with /analyze.
#pragma warning(push)
#pragma warning(disable:6387)
__inline HRESULT SHResolveFolderPathInLibrary(_In_ IShellLibrary *plib, _In_ PCWSTR pszFolderPath, _In_ DWORD dwTimeout, _Outptr_ PWSTR *ppszResolvedPath)
{
    *ppszResolvedPath = NULL;
    PIDLIST_ABSOLUTE pidlFolder = SHSimpleIDListFromPath(pszFolderPath);
    HRESULT hr = pidlFolder ? S_OK : E_INVALIDARG;
    if (SUCCEEDED(hr))
    {
        IShellItem *psiFolder;
        hr = SHCreateItemFromIDList(pidlFolder, IID_PPV_ARGS(&psiFolder));
        if (SUCCEEDED(hr))
        {
            IShellItem *psiResolved;
            hr = plib->ResolveFolder(psiFolder, dwTimeout, IID_PPV_ARGS(&psiResolved));
            if (SUCCEEDED(hr))
            {
                hr = psiResolved->GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING, ppszResolvedPath);
                psiResolved->Release();
            }
            psiFolder->Release();
        }
        CoTaskMemFree(pidlFolder);
    }
    return hr;
}

#pragma warning(pop)
#endif  // __cplusplus && !CINTERFACE
#endif  // _WIN32_IE >= _WIN32_IE_IE70
#endif  // NTDDI_WIN7

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [v1_enum] */ 
enum UNDOCK_REASON
    {
        UR_RESOLUTION_CHANGE	= 0,
        UR_MONITOR_DISCONNECT	= 1
    } 	UNDOCK_REASON;



extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0051_v0_0_s_ifspec;

#ifndef __IAccessibilityDockingServiceCallback_INTERFACE_DEFINED__
#define __IAccessibilityDockingServiceCallback_INTERFACE_DEFINED__

/* interface IAccessibilityDockingServiceCallback */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibilityDockingServiceCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("157733FD-A592-42E5-B594-248468C5A81B")
    IAccessibilityDockingServiceCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Undocked( 
            /* [in] */ UNDOCK_REASON undockReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibilityDockingServiceCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibilityDockingServiceCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibilityDockingServiceCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibilityDockingServiceCallback * This);
        
        DECLSPEC_XFGVIRT(IAccessibilityDockingServiceCallback, Undocked)
        HRESULT ( STDMETHODCALLTYPE *Undocked )( 
            __RPC__in IAccessibilityDockingServiceCallback * This,
            /* [in] */ UNDOCK_REASON undockReason);
        
        END_INTERFACE
    } IAccessibilityDockingServiceCallbackVtbl;

    interface IAccessibilityDockingServiceCallback
    {
        CONST_VTBL struct IAccessibilityDockingServiceCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibilityDockingServiceCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibilityDockingServiceCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibilityDockingServiceCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibilityDockingServiceCallback_Undocked(This,undockReason)	\
    ( (This)->lpVtbl -> Undocked(This,undockReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibilityDockingServiceCallback_INTERFACE_DEFINED__ */


#ifndef __IAccessibilityDockingService_INTERFACE_DEFINED__
#define __IAccessibilityDockingService_INTERFACE_DEFINED__

/* interface IAccessibilityDockingService */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibilityDockingService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8849DC22-CEDF-4C95-998D-051419DD3F76")
    IAccessibilityDockingService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAvailableSize( 
            /* [in] */ __RPC__in HMONITOR hMonitor,
            /* [out] */ __RPC__out UINT *pcxFixed,
            /* [out] */ __RPC__out UINT *pcyMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DockWindow( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ __RPC__in HMONITOR hMonitor,
            /* [in] */ UINT cyRequested,
            /* [in] */ __RPC__in_opt IAccessibilityDockingServiceCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UndockWindow( 
            /* [in] */ __RPC__in HWND hwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibilityDockingServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibilityDockingService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibilityDockingService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibilityDockingService * This);
        
        DECLSPEC_XFGVIRT(IAccessibilityDockingService, GetAvailableSize)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableSize )( 
            __RPC__in IAccessibilityDockingService * This,
            /* [in] */ __RPC__in HMONITOR hMonitor,
            /* [out] */ __RPC__out UINT *pcxFixed,
            /* [out] */ __RPC__out UINT *pcyMax);
        
        DECLSPEC_XFGVIRT(IAccessibilityDockingService, DockWindow)
        HRESULT ( STDMETHODCALLTYPE *DockWindow )( 
            __RPC__in IAccessibilityDockingService * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ __RPC__in HMONITOR hMonitor,
            /* [in] */ UINT cyRequested,
            /* [in] */ __RPC__in_opt IAccessibilityDockingServiceCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IAccessibilityDockingService, UndockWindow)
        HRESULT ( STDMETHODCALLTYPE *UndockWindow )( 
            __RPC__in IAccessibilityDockingService * This,
            /* [in] */ __RPC__in HWND hwnd);
        
        END_INTERFACE
    } IAccessibilityDockingServiceVtbl;

    interface IAccessibilityDockingService
    {
        CONST_VTBL struct IAccessibilityDockingServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibilityDockingService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibilityDockingService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibilityDockingService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibilityDockingService_GetAvailableSize(This,hMonitor,pcxFixed,pcyMax)	\
    ( (This)->lpVtbl -> GetAvailableSize(This,hMonitor,pcxFixed,pcyMax) ) 

#define IAccessibilityDockingService_DockWindow(This,hwnd,hMonitor,cyRequested,pCallback)	\
    ( (This)->lpVtbl -> DockWindow(This,hwnd,hMonitor,cyRequested,pCallback) ) 

#define IAccessibilityDockingService_UndockWindow(This,hwnd)	\
    ( (This)->lpVtbl -> UndockWindow(This,hwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibilityDockingService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0053 */
/* [local] */ 

#endif // NTDDI_WIN8
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0053_v0_0_s_ifspec;

#ifndef __IStorageProviderBanners_INTERFACE_DEFINED__
#define __IStorageProviderBanners_INTERFACE_DEFINED__

/* interface IStorageProviderBanners */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IStorageProviderBanners;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5efb46d7-47C0-4b68-acda-ded47c90ec91")
    IStorageProviderBanners : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBanner( 
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId,
            /* [in] */ __RPC__in LPCWSTR contentId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearBanner( 
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearAllBanners( 
            /* [in] */ __RPC__in LPCWSTR providerIdentity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBanner( 
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId,
            /* [out] */ __RPC__deref_out_opt LPWSTR *contentId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStorageProviderBannersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStorageProviderBanners * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStorageProviderBanners * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStorageProviderBanners * This);
        
        DECLSPEC_XFGVIRT(IStorageProviderBanners, SetBanner)
        HRESULT ( STDMETHODCALLTYPE *SetBanner )( 
            __RPC__in IStorageProviderBanners * This,
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId,
            /* [in] */ __RPC__in LPCWSTR contentId);
        
        DECLSPEC_XFGVIRT(IStorageProviderBanners, ClearBanner)
        HRESULT ( STDMETHODCALLTYPE *ClearBanner )( 
            __RPC__in IStorageProviderBanners * This,
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId);
        
        DECLSPEC_XFGVIRT(IStorageProviderBanners, ClearAllBanners)
        HRESULT ( STDMETHODCALLTYPE *ClearAllBanners )( 
            __RPC__in IStorageProviderBanners * This,
            /* [in] */ __RPC__in LPCWSTR providerIdentity);
        
        DECLSPEC_XFGVIRT(IStorageProviderBanners, GetBanner)
        HRESULT ( STDMETHODCALLTYPE *GetBanner )( 
            __RPC__in IStorageProviderBanners * This,
            /* [in] */ __RPC__in LPCWSTR providerIdentity,
            /* [in] */ __RPC__in LPCWSTR subscriptionId,
            /* [out] */ __RPC__deref_out_opt LPWSTR *contentId);
        
        END_INTERFACE
    } IStorageProviderBannersVtbl;

    interface IStorageProviderBanners
    {
        CONST_VTBL struct IStorageProviderBannersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStorageProviderBanners_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStorageProviderBanners_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStorageProviderBanners_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStorageProviderBanners_SetBanner(This,providerIdentity,subscriptionId,contentId)	\
    ( (This)->lpVtbl -> SetBanner(This,providerIdentity,subscriptionId,contentId) ) 

#define IStorageProviderBanners_ClearBanner(This,providerIdentity,subscriptionId)	\
    ( (This)->lpVtbl -> ClearBanner(This,providerIdentity,subscriptionId) ) 

#define IStorageProviderBanners_ClearAllBanners(This,providerIdentity)	\
    ( (This)->lpVtbl -> ClearAllBanners(This,providerIdentity) ) 

#define IStorageProviderBanners_GetBanner(This,providerIdentity,subscriptionId,contentId)	\
    ( (This)->lpVtbl -> GetBanner(This,providerIdentity,subscriptionId,contentId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStorageProviderBanners_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0054 */
/* [local] */ 

#endif // NTDDI_WIN10_RS4


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0054_v0_0_s_ifspec;

#ifndef __IStorageProviderCopyHook_INTERFACE_DEFINED__
#define __IStorageProviderCopyHook_INTERFACE_DEFINED__

/* interface IStorageProviderCopyHook */
/* [uuid][object] */ 


EXTERN_C const IID IID_IStorageProviderCopyHook;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7bf992a9-af7a-4dba-b2e5-4d080b1ecbc6")
    IStorageProviderCopyHook : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CopyCallback( 
            /* [unique][in] */ __RPC__in_opt HWND hwnd,
            /* [in] */ UINT operation,
            /* [in] */ UINT flags,
            /* [string][in] */ __RPC__in_string LPCWSTR srcFile,
            /* [in] */ DWORD srcAttribs,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR destFile,
            /* [in] */ DWORD destAttribs,
            /* [out] */ __RPC__out UINT *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStorageProviderCopyHookVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStorageProviderCopyHook * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStorageProviderCopyHook * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStorageProviderCopyHook * This);
        
        DECLSPEC_XFGVIRT(IStorageProviderCopyHook, CopyCallback)
        HRESULT ( STDMETHODCALLTYPE *CopyCallback )( 
            __RPC__in IStorageProviderCopyHook * This,
            /* [unique][in] */ __RPC__in_opt HWND hwnd,
            /* [in] */ UINT operation,
            /* [in] */ UINT flags,
            /* [string][in] */ __RPC__in_string LPCWSTR srcFile,
            /* [in] */ DWORD srcAttribs,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR destFile,
            /* [in] */ DWORD destAttribs,
            /* [out] */ __RPC__out UINT *result);
        
        END_INTERFACE
    } IStorageProviderCopyHookVtbl;

    interface IStorageProviderCopyHook
    {
        CONST_VTBL struct IStorageProviderCopyHookVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStorageProviderCopyHook_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStorageProviderCopyHook_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStorageProviderCopyHook_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStorageProviderCopyHook_CopyCallback(This,hwnd,operation,flags,srcFile,srcAttribs,destFile,destAttribs,result)	\
    ( (This)->lpVtbl -> CopyCallback(This,hwnd,operation,flags,srcFile,srcAttribs,destFile,destAttribs,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStorageProviderCopyHook_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shobjidl_0000_0055 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0055_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0055_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HMONITOR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * ); 
void                      __RPC_USER  HMONITOR_UserFree(     __RPC__in unsigned long *, __RPC__in HMONITOR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  PCIDLIST_ABSOLUTE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PCIDLIST_ABSOLUTE_UserFree(     __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * ); 

unsigned long             __RPC_USER  PITEMID_CHILD_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in PITEMID_CHILD * ); 
unsigned char * __RPC_USER  PITEMID_CHILD_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PITEMID_CHILD * ); 
unsigned char * __RPC_USER  PITEMID_CHILD_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PITEMID_CHILD * ); 
void                      __RPC_USER  PITEMID_CHILD_UserFree(     __RPC__in unsigned long *, __RPC__in PITEMID_CHILD * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HMONITOR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * ); 
void                      __RPC_USER  HMONITOR_UserFree64(     __RPC__in unsigned long *, __RPC__in HMONITOR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  PCIDLIST_ABSOLUTE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PCIDLIST_ABSOLUTE_UserFree64(     __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * ); 

unsigned long             __RPC_USER  PITEMID_CHILD_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in PITEMID_CHILD * ); 
unsigned char * __RPC_USER  PITEMID_CHILD_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PITEMID_CHILD * ); 
unsigned char * __RPC_USER  PITEMID_CHILD_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PITEMID_CHILD * ); 
void                      __RPC_USER  PITEMID_CHILD_UserFree64(     __RPC__in unsigned long *, __RPC__in PITEMID_CHILD * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IResultsFolder_AddIDList_Proxy( 
    IResultsFolder * This,
    /* [annotation][in] */ 
    _In_  PCIDLIST_ABSOLUTE pidl,
    /* [annotation][out] */ 
    _Outptr_opt_  PITEMID_CHILD *ppidlAdded);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IResultsFolder_AddIDList_Stub( 
    __RPC__in IResultsFolder * This,
    /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl,
    /* [out] */ __RPC__deref_out_opt PITEMID_CHILD *ppidlAdded);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



