

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

#ifndef __msfeeds_h__
#define __msfeeds_h__

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

#ifndef __IXFeedsManager_FWD_DEFINED__
#define __IXFeedsManager_FWD_DEFINED__
typedef interface IXFeedsManager IXFeedsManager;

#endif 	/* __IXFeedsManager_FWD_DEFINED__ */


#ifndef __IXFeedsEnum_FWD_DEFINED__
#define __IXFeedsEnum_FWD_DEFINED__
typedef interface IXFeedsEnum IXFeedsEnum;

#endif 	/* __IXFeedsEnum_FWD_DEFINED__ */


#ifndef __IXFeedFolder_FWD_DEFINED__
#define __IXFeedFolder_FWD_DEFINED__
typedef interface IXFeedFolder IXFeedFolder;

#endif 	/* __IXFeedFolder_FWD_DEFINED__ */


#ifndef __IXFeedFolderEvents_FWD_DEFINED__
#define __IXFeedFolderEvents_FWD_DEFINED__
typedef interface IXFeedFolderEvents IXFeedFolderEvents;

#endif 	/* __IXFeedFolderEvents_FWD_DEFINED__ */


#ifndef __IXFeed_FWD_DEFINED__
#define __IXFeed_FWD_DEFINED__
typedef interface IXFeed IXFeed;

#endif 	/* __IXFeed_FWD_DEFINED__ */


#ifndef __IXFeed2_FWD_DEFINED__
#define __IXFeed2_FWD_DEFINED__
typedef interface IXFeed2 IXFeed2;

#endif 	/* __IXFeed2_FWD_DEFINED__ */


#ifndef __IXFeedEvents_FWD_DEFINED__
#define __IXFeedEvents_FWD_DEFINED__
typedef interface IXFeedEvents IXFeedEvents;

#endif 	/* __IXFeedEvents_FWD_DEFINED__ */


#ifndef __IXFeedItem_FWD_DEFINED__
#define __IXFeedItem_FWD_DEFINED__
typedef interface IXFeedItem IXFeedItem;

#endif 	/* __IXFeedItem_FWD_DEFINED__ */


#ifndef __IXFeedItem2_FWD_DEFINED__
#define __IXFeedItem2_FWD_DEFINED__
typedef interface IXFeedItem2 IXFeedItem2;

#endif 	/* __IXFeedItem2_FWD_DEFINED__ */


#ifndef __IXFeedEnclosure_FWD_DEFINED__
#define __IXFeedEnclosure_FWD_DEFINED__
typedef interface IXFeedEnclosure IXFeedEnclosure;

#endif 	/* __IXFeedEnclosure_FWD_DEFINED__ */


#ifndef __IFeedsManager_FWD_DEFINED__
#define __IFeedsManager_FWD_DEFINED__
typedef interface IFeedsManager IFeedsManager;

#endif 	/* __IFeedsManager_FWD_DEFINED__ */


#ifndef __IFeedsEnum_FWD_DEFINED__
#define __IFeedsEnum_FWD_DEFINED__
typedef interface IFeedsEnum IFeedsEnum;

#endif 	/* __IFeedsEnum_FWD_DEFINED__ */


#ifndef __IFeedFolder_FWD_DEFINED__
#define __IFeedFolder_FWD_DEFINED__
typedef interface IFeedFolder IFeedFolder;

#endif 	/* __IFeedFolder_FWD_DEFINED__ */


#ifndef __IFeedFolderEvents_FWD_DEFINED__
#define __IFeedFolderEvents_FWD_DEFINED__
typedef interface IFeedFolderEvents IFeedFolderEvents;

#endif 	/* __IFeedFolderEvents_FWD_DEFINED__ */


#ifndef __IFeed_FWD_DEFINED__
#define __IFeed_FWD_DEFINED__
typedef interface IFeed IFeed;

#endif 	/* __IFeed_FWD_DEFINED__ */


#ifndef __IFeed2_FWD_DEFINED__
#define __IFeed2_FWD_DEFINED__
typedef interface IFeed2 IFeed2;

#endif 	/* __IFeed2_FWD_DEFINED__ */


#ifndef __IFeedEvents_FWD_DEFINED__
#define __IFeedEvents_FWD_DEFINED__
typedef interface IFeedEvents IFeedEvents;

#endif 	/* __IFeedEvents_FWD_DEFINED__ */


#ifndef __IFeedItem_FWD_DEFINED__
#define __IFeedItem_FWD_DEFINED__
typedef interface IFeedItem IFeedItem;

#endif 	/* __IFeedItem_FWD_DEFINED__ */


#ifndef __IFeedItem2_FWD_DEFINED__
#define __IFeedItem2_FWD_DEFINED__
typedef interface IFeedItem2 IFeedItem2;

#endif 	/* __IFeedItem2_FWD_DEFINED__ */


#ifndef __IFeedEnclosure_FWD_DEFINED__
#define __IFeedEnclosure_FWD_DEFINED__
typedef interface IFeedEnclosure IFeedEnclosure;

#endif 	/* __IFeedEnclosure_FWD_DEFINED__ */


#ifndef __FeedsManager_FWD_DEFINED__
#define __FeedsManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FeedsManager FeedsManager;
#else
typedef struct FeedsManager FeedsManager;
#endif /* __cplusplus */

#endif 	/* __FeedsManager_FWD_DEFINED__ */


#ifndef __FeedFolderWatcher_FWD_DEFINED__
#define __FeedFolderWatcher_FWD_DEFINED__

#ifdef __cplusplus
typedef class FeedFolderWatcher FeedFolderWatcher;
#else
typedef struct FeedFolderWatcher FeedFolderWatcher;
#endif /* __cplusplus */

#endif 	/* __FeedFolderWatcher_FWD_DEFINED__ */


#ifndef __FeedWatcher_FWD_DEFINED__
#define __FeedWatcher_FWD_DEFINED__

#ifdef __cplusplus
typedef class FeedWatcher FeedWatcher;
#else
typedef struct FeedWatcher FeedWatcher;
#endif /* __cplusplus */

#endif 	/* __FeedWatcher_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msfeeds_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// msfeeds.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
// RSS Platform Interfaces.

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// {FE6B11C3-C72E-4061-86C6-9D163121F229}
DEFINE_GUID(CLSID_XFeedsManager, 0xfe6b11c3, 0xc72e, 0x4061, 0x86, 0xc6, 0x9d, 0x16, 0x31, 0x21, 0xf2, 0x29);
typedef /* [v1_enum] */ 
enum FEEDS_BACKGROUNDSYNC_ACTION
    {
        FBSA_DISABLE	= 0,
        FBSA_ENABLE	= 1,
        FBSA_RUNNOW	= 2
    } 	FEEDS_BACKGROUNDSYNC_ACTION;

typedef /* [v1_enum] */ 
enum FEEDS_BACKGROUNDSYNC_STATUS
    {
        FBSS_DISABLED	= 0,
        FBSS_ENABLED	= 1
    } 	FEEDS_BACKGROUNDSYNC_STATUS;

typedef /* [v1_enum] */ 
enum FEEDS_EVENTS_SCOPE
    {
        FES_ALL	= 0,
        FES_SELF_ONLY	= 1,
        FES_SELF_AND_CHILDREN_ONLY	= 2
    } 	FEEDS_EVENTS_SCOPE;

typedef /* [v1_enum] */ 
enum FEEDS_EVENTS_MASK
    {
        FEM_FOLDEREVENTS	= 0x1,
        FEM_FEEDEVENTS	= 0x2
    } 	FEEDS_EVENTS_MASK;


#define FEEDS_XML_COUNT_MAX ((UINT)-1)

typedef /* [v1_enum] */ 
enum FEEDS_XML_SORT_PROPERTY
    {
        FXSP_NONE	= 0,
        FXSP_PUBDATE	= 1,
        FXSP_DOWNLOADTIME	= 2
    } 	FEEDS_XML_SORT_PROPERTY;

typedef /* [v1_enum] */ 
enum FEEDS_XML_SORT_ORDER
    {
        FXSO_NONE	= 0,
        FXSO_ASCENDING	= 1,
        FXSO_DESCENDING	= 2
    } 	FEEDS_XML_SORT_ORDER;

typedef /* [v1_enum] */ 
enum FEEDS_XML_FILTER_FLAGS
    {
        FXFF_ALL	= 0,
        FXFF_UNREAD	= 0x1,
        FXFF_READ	= 0x2
    } 	FEEDS_XML_FILTER_FLAGS;

typedef /* [v1_enum] */ 
enum FEEDS_XML_INCLUDE_FLAGS
    {
        FXIF_NONE	= 0,
        FXIF_CF_EXTENSIONS	= 0x1
    } 	FEEDS_XML_INCLUDE_FLAGS;

typedef /* [v1_enum] */ 
enum FEEDS_DOWNLOAD_STATUS
    {
        FDS_NONE	= 0,
        FDS_PENDING	= 1,
        FDS_DOWNLOADING	= 2,
        FDS_DOWNLOADED	= 3,
        FDS_DOWNLOAD_FAILED	= 4
    } 	FEEDS_DOWNLOAD_STATUS;

typedef /* [v1_enum] */ 
enum FEEDS_SYNC_SETTING
    {
        FSS_DEFAULT	= 0,
        FSS_INTERVAL	= 1,
        FSS_MANUAL	= 2,
        FSS_SUGGESTED	= 3
    } 	FEEDS_SYNC_SETTING;

typedef /* [v1_enum] */ 
enum FEEDS_DOWNLOAD_ERROR
    {
        FDE_NONE	= 0,
        FDE_DOWNLOAD_FAILED	= 1,
        FDE_INVALID_FEED_FORMAT	= 2,
        FDE_NORMALIZATION_FAILED	= 3,
        FDE_PERSISTENCE_FAILED	= 4,
        FDE_DOWNLOAD_BLOCKED	= 5,
        FDE_CANCELED	= 6,
        FDE_UNSUPPORTED_AUTH	= 7,
        FDE_BACKGROUND_DOWNLOAD_DISABLED	= 8,
        FDE_NOT_EXIST	= 9,
        FDE_UNSUPPORTED_MSXML	= 10,
        FDE_UNSUPPORTED_DTD	= 11,
        FDE_DOWNLOAD_SIZE_LIMIT_EXCEEDED	= 12,
        FDE_ACCESS_DENIED	= 13,
        FDE_AUTH_FAILED	= 14,
        FDE_INVALID_AUTH	= 15
    } 	FEEDS_DOWNLOAD_ERROR;

typedef /* [v1_enum] */ 
enum FEEDS_EVENTS_ITEM_COUNT_FLAGS
    {
        FEICF_READ_ITEM_COUNT_CHANGED	= 0x1,
        FEICF_UNREAD_ITEM_COUNT_CHANGED	= 0x2
    } 	FEEDS_EVENTS_ITEM_COUNT_FLAGS;

typedef int FEICF;

typedef 
enum FEEDS_ERROR_CODE
    {
        FEC_E_ERRORBASE	= 0xc0040200L,
        FEC_E_INVALIDMSXMLPROPERTY	= FEC_E_ERRORBASE,
        FEC_E_DOWNLOADSIZELIMITEXCEEDED	= ( FEC_E_INVALIDMSXMLPROPERTY + 1 ) 
    } 	FEEDS_ERROR_CODE;



extern RPC_IF_HANDLE __MIDL_itf_msfeeds_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msfeeds_0000_0000_v0_0_s_ifspec;

#ifndef __IXFeedsManager_INTERFACE_DEFINED__
#define __IXFeedsManager_INTERFACE_DEFINED__

/* interface IXFeedsManager */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedsManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5357e238-fb12-4aca-a930-cab7832b84bf")
    IXFeedsManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RootFolder( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSubscribed( 
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [out] */ __RPC__out BOOL *pbSubscribed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExistsFeed( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out BOOL *pbFeedExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFeed( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFeedByUrl( 
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExistsFolder( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out BOOL *pbFolderExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFolder( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteFeed( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteFolder( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackgroundSync( 
            /* [in] */ FEEDS_BACKGROUNDSYNC_ACTION fbsa) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackgroundSyncStatus( 
            /* [out] */ __RPC__out FEEDS_BACKGROUNDSYNC_STATUS *pfbss) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DefaultInterval( 
            /* [out] */ __RPC__out UINT *puiInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultInterval( 
            /* [in] */ UINT uiInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsyncSyncAll( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Normalize( 
            /* [in] */ __RPC__in_opt IStream *pStreamIn,
            /* [out] */ __RPC__deref_out_opt IStream **ppStreamOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ItemCountLimit( 
            /* [out] */ __RPC__out UINT *puiItemCountLimit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedsManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, RootFolder)
        HRESULT ( STDMETHODCALLTYPE *RootFolder )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, IsSubscribed)
        HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [out] */ __RPC__out BOOL *pbSubscribed);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, ExistsFeed)
        HRESULT ( STDMETHODCALLTYPE *ExistsFeed )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out BOOL *pbFeedExists);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, GetFeed)
        HRESULT ( STDMETHODCALLTYPE *GetFeed )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, GetFeedByUrl)
        HRESULT ( STDMETHODCALLTYPE *GetFeedByUrl )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, ExistsFolder)
        HRESULT ( STDMETHODCALLTYPE *ExistsFolder )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out BOOL *pbFolderExists);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, GetFolder)
        HRESULT ( STDMETHODCALLTYPE *GetFolder )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, DeleteFeed)
        HRESULT ( STDMETHODCALLTYPE *DeleteFeed )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, DeleteFolder)
        HRESULT ( STDMETHODCALLTYPE *DeleteFolder )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, BackgroundSync)
        HRESULT ( STDMETHODCALLTYPE *BackgroundSync )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ FEEDS_BACKGROUNDSYNC_ACTION fbsa);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, BackgroundSyncStatus)
        HRESULT ( STDMETHODCALLTYPE *BackgroundSyncStatus )( 
            __RPC__in IXFeedsManager * This,
            /* [out] */ __RPC__out FEEDS_BACKGROUNDSYNC_STATUS *pfbss);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, DefaultInterval)
        HRESULT ( STDMETHODCALLTYPE *DefaultInterval )( 
            __RPC__in IXFeedsManager * This,
            /* [out] */ __RPC__out UINT *puiInterval);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, SetDefaultInterval)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultInterval )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ UINT uiInterval);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, AsyncSyncAll)
        HRESULT ( STDMETHODCALLTYPE *AsyncSyncAll )( 
            __RPC__in IXFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, Normalize)
        HRESULT ( STDMETHODCALLTYPE *Normalize )( 
            __RPC__in IXFeedsManager * This,
            /* [in] */ __RPC__in_opt IStream *pStreamIn,
            /* [out] */ __RPC__deref_out_opt IStream **ppStreamOut);
        
        DECLSPEC_XFGVIRT(IXFeedsManager, ItemCountLimit)
        HRESULT ( STDMETHODCALLTYPE *ItemCountLimit )( 
            __RPC__in IXFeedsManager * This,
            /* [out] */ __RPC__out UINT *puiItemCountLimit);
        
        END_INTERFACE
    } IXFeedsManagerVtbl;

    interface IXFeedsManager
    {
        CONST_VTBL struct IXFeedsManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedsManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedsManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedsManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedsManager_RootFolder(This,riid,ppv)	\
    ( (This)->lpVtbl -> RootFolder(This,riid,ppv) ) 

#define IXFeedsManager_IsSubscribed(This,pszUrl,pbSubscribed)	\
    ( (This)->lpVtbl -> IsSubscribed(This,pszUrl,pbSubscribed) ) 

#define IXFeedsManager_ExistsFeed(This,pszPath,pbFeedExists)	\
    ( (This)->lpVtbl -> ExistsFeed(This,pszPath,pbFeedExists) ) 

#define IXFeedsManager_GetFeed(This,pszPath,riid,ppv)	\
    ( (This)->lpVtbl -> GetFeed(This,pszPath,riid,ppv) ) 

#define IXFeedsManager_GetFeedByUrl(This,pszUrl,riid,ppv)	\
    ( (This)->lpVtbl -> GetFeedByUrl(This,pszUrl,riid,ppv) ) 

#define IXFeedsManager_ExistsFolder(This,pszPath,pbFolderExists)	\
    ( (This)->lpVtbl -> ExistsFolder(This,pszPath,pbFolderExists) ) 

#define IXFeedsManager_GetFolder(This,pszPath,riid,ppv)	\
    ( (This)->lpVtbl -> GetFolder(This,pszPath,riid,ppv) ) 

#define IXFeedsManager_DeleteFeed(This,pszPath)	\
    ( (This)->lpVtbl -> DeleteFeed(This,pszPath) ) 

#define IXFeedsManager_DeleteFolder(This,pszPath)	\
    ( (This)->lpVtbl -> DeleteFolder(This,pszPath) ) 

#define IXFeedsManager_BackgroundSync(This,fbsa)	\
    ( (This)->lpVtbl -> BackgroundSync(This,fbsa) ) 

#define IXFeedsManager_BackgroundSyncStatus(This,pfbss)	\
    ( (This)->lpVtbl -> BackgroundSyncStatus(This,pfbss) ) 

#define IXFeedsManager_DefaultInterval(This,puiInterval)	\
    ( (This)->lpVtbl -> DefaultInterval(This,puiInterval) ) 

#define IXFeedsManager_SetDefaultInterval(This,uiInterval)	\
    ( (This)->lpVtbl -> SetDefaultInterval(This,uiInterval) ) 

#define IXFeedsManager_AsyncSyncAll(This)	\
    ( (This)->lpVtbl -> AsyncSyncAll(This) ) 

#define IXFeedsManager_Normalize(This,pStreamIn,ppStreamOut)	\
    ( (This)->lpVtbl -> Normalize(This,pStreamIn,ppStreamOut) ) 

#define IXFeedsManager_ItemCountLimit(This,puiItemCountLimit)	\
    ( (This)->lpVtbl -> ItemCountLimit(This,puiItemCountLimit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedsManager_INTERFACE_DEFINED__ */


#ifndef __IXFeedsEnum_INTERFACE_DEFINED__
#define __IXFeedsEnum_INTERFACE_DEFINED__

/* interface IXFeedsEnum */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedsEnum;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dc43a9d5-5015-4301-8c96-a47434b4d658")
    IXFeedsEnum : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [out] */ __RPC__out UINT *puiCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ UINT uiIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedsEnumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedsEnum * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedsEnum * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedsEnum * This);
        
        DECLSPEC_XFGVIRT(IXFeedsEnum, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            __RPC__in IXFeedsEnum * This,
            /* [out] */ __RPC__out UINT *puiCount);
        
        DECLSPEC_XFGVIRT(IXFeedsEnum, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IXFeedsEnum * This,
            /* [in] */ UINT uiIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IXFeedsEnumVtbl;

    interface IXFeedsEnum
    {
        CONST_VTBL struct IXFeedsEnumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedsEnum_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedsEnum_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedsEnum_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedsEnum_Count(This,puiCount)	\
    ( (This)->lpVtbl -> Count(This,puiCount) ) 

#define IXFeedsEnum_Item(This,uiIndex,riid,ppv)	\
    ( (This)->lpVtbl -> Item(This,uiIndex,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedsEnum_INTERFACE_DEFINED__ */


#ifndef __IXFeedFolder_INTERFACE_DEFINED__
#define __IXFeedFolder_INTERFACE_DEFINED__

/* interface IXFeedFolder */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedFolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c963678-3a51-4b88-8531-98b90b6508f2")
    IXFeedFolder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Feeds( 
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Subfolders( 
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFeed( 
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSubfolder( 
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExistsFeed( 
            __RPC__in LPCWSTR pszName,
            __RPC__in BOOL *pbFeedExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExistsSubfolder( 
            __RPC__in LPCWSTR pszName,
            __RPC__in BOOL *pbSubfolderExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFeed( 
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubfolder( 
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Name( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ __RPC__in LPCWSTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Path( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Parent( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRoot( 
            /* [out] */ __RPC__out BOOL *pbIsRootFeedFolder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWatcher( 
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TotalUnreadItemCount( 
            /* [out] */ __RPC__out UINT *puiTotalUnreadItemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TotalItemCount( 
            /* [out] */ __RPC__out UINT *puiTotalItemCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedFolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Feeds)
        HRESULT ( STDMETHODCALLTYPE *Feeds )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Subfolders)
        HRESULT ( STDMETHODCALLTYPE *Subfolders )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, CreateFeed)
        HRESULT ( STDMETHODCALLTYPE *CreateFeed )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, CreateSubfolder)
        HRESULT ( STDMETHODCALLTYPE *CreateSubfolder )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, ExistsFeed)
        HRESULT ( STDMETHODCALLTYPE *ExistsFeed )( 
            __RPC__in IXFeedFolder * This,
            __RPC__in LPCWSTR pszName,
            __RPC__in BOOL *pbFeedExists);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, ExistsSubfolder)
        HRESULT ( STDMETHODCALLTYPE *ExistsSubfolder )( 
            __RPC__in IXFeedFolder * This,
            __RPC__in LPCWSTR pszName,
            __RPC__in BOOL *pbSubfolderExists);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, GetFeed)
        HRESULT ( STDMETHODCALLTYPE *GetFeed )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, GetSubfolder)
        HRESULT ( STDMETHODCALLTYPE *GetSubfolder )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IXFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Name)
        HRESULT ( STDMETHODCALLTYPE *Name )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Path)
        HRESULT ( STDMETHODCALLTYPE *Path )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, IsRoot)
        HRESULT ( STDMETHODCALLTYPE *IsRoot )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__out BOOL *pbIsRootFeedFolder);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, GetWatcher)
        HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IXFeedFolder * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, TotalUnreadItemCount)
        HRESULT ( STDMETHODCALLTYPE *TotalUnreadItemCount )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__out UINT *puiTotalUnreadItemCount);
        
        DECLSPEC_XFGVIRT(IXFeedFolder, TotalItemCount)
        HRESULT ( STDMETHODCALLTYPE *TotalItemCount )( 
            __RPC__in IXFeedFolder * This,
            /* [out] */ __RPC__out UINT *puiTotalItemCount);
        
        END_INTERFACE
    } IXFeedFolderVtbl;

    interface IXFeedFolder
    {
        CONST_VTBL struct IXFeedFolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedFolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedFolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedFolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedFolder_Feeds(This,ppfe)	\
    ( (This)->lpVtbl -> Feeds(This,ppfe) ) 

#define IXFeedFolder_Subfolders(This,ppfe)	\
    ( (This)->lpVtbl -> Subfolders(This,ppfe) ) 

#define IXFeedFolder_CreateFeed(This,pszName,pszUrl,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFeed(This,pszName,pszUrl,riid,ppv) ) 

#define IXFeedFolder_CreateSubfolder(This,pszName,riid,ppv)	\
    ( (This)->lpVtbl -> CreateSubfolder(This,pszName,riid,ppv) ) 

#define IXFeedFolder_ExistsFeed(This,pszName,pbFeedExists)	\
    ( (This)->lpVtbl -> ExistsFeed(This,pszName,pbFeedExists) ) 

#define IXFeedFolder_ExistsSubfolder(This,pszName,pbSubfolderExists)	\
    ( (This)->lpVtbl -> ExistsSubfolder(This,pszName,pbSubfolderExists) ) 

#define IXFeedFolder_GetFeed(This,pszName,riid,ppv)	\
    ( (This)->lpVtbl -> GetFeed(This,pszName,riid,ppv) ) 

#define IXFeedFolder_GetSubfolder(This,pszName,riid,ppv)	\
    ( (This)->lpVtbl -> GetSubfolder(This,pszName,riid,ppv) ) 

#define IXFeedFolder_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IXFeedFolder_Name(This,ppszName)	\
    ( (This)->lpVtbl -> Name(This,ppszName) ) 

#define IXFeedFolder_Rename(This,pszName)	\
    ( (This)->lpVtbl -> Rename(This,pszName) ) 

#define IXFeedFolder_Path(This,ppszPath)	\
    ( (This)->lpVtbl -> Path(This,ppszPath) ) 

#define IXFeedFolder_Move(This,pszPath)	\
    ( (This)->lpVtbl -> Move(This,pszPath) ) 

#define IXFeedFolder_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeedFolder_IsRoot(This,pbIsRootFeedFolder)	\
    ( (This)->lpVtbl -> IsRoot(This,pbIsRootFeedFolder) ) 

#define IXFeedFolder_GetWatcher(This,scope,mask,riid,ppv)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,riid,ppv) ) 

#define IXFeedFolder_TotalUnreadItemCount(This,puiTotalUnreadItemCount)	\
    ( (This)->lpVtbl -> TotalUnreadItemCount(This,puiTotalUnreadItemCount) ) 

#define IXFeedFolder_TotalItemCount(This,puiTotalItemCount)	\
    ( (This)->lpVtbl -> TotalItemCount(This,puiTotalItemCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedFolder_INTERFACE_DEFINED__ */


#ifndef __IXFeedFolderEvents_INTERFACE_DEFINED__
#define __IXFeedFolderEvents_INTERFACE_DEFINED__

/* interface IXFeedFolderEvents */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedFolderEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7964b769-234a-4bb1-a5f4-90454c8ad07e")
    IXFeedFolderEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Error( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderAdded( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderDeleted( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderRenamed( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderMovedFrom( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderMovedTo( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FolderItemCountChanged( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedAdded( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDeleted( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedRenamed( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedUrlChanged( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedMovedFrom( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedMovedTo( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDownloading( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDownloadCompleted( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEEDS_DOWNLOAD_ERROR fde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedItemCountChanged( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedFolderEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, Error)
        HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IXFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderAdded)
        HRESULT ( STDMETHODCALLTYPE *FolderAdded )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderDeleted)
        HRESULT ( STDMETHODCALLTYPE *FolderDeleted )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderRenamed)
        HRESULT ( STDMETHODCALLTYPE *FolderRenamed )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderMovedFrom)
        HRESULT ( STDMETHODCALLTYPE *FolderMovedFrom )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderMovedTo)
        HRESULT ( STDMETHODCALLTYPE *FolderMovedTo )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FolderItemCountChanged)
        HRESULT ( STDMETHODCALLTYPE *FolderItemCountChanged )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedAdded)
        HRESULT ( STDMETHODCALLTYPE *FeedAdded )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedDeleted)
        HRESULT ( STDMETHODCALLTYPE *FeedDeleted )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedRenamed)
        HRESULT ( STDMETHODCALLTYPE *FeedRenamed )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedUrlChanged)
        HRESULT ( STDMETHODCALLTYPE *FeedUrlChanged )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedMovedFrom)
        HRESULT ( STDMETHODCALLTYPE *FeedMovedFrom )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedMovedTo)
        HRESULT ( STDMETHODCALLTYPE *FeedMovedTo )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedDownloading)
        HRESULT ( STDMETHODCALLTYPE *FeedDownloading )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedDownloadCompleted)
        HRESULT ( STDMETHODCALLTYPE *FeedDownloadCompleted )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEEDS_DOWNLOAD_ERROR fde);
        
        DECLSPEC_XFGVIRT(IXFeedFolderEvents, FeedItemCountChanged)
        HRESULT ( STDMETHODCALLTYPE *FeedItemCountChanged )( 
            __RPC__in IXFeedFolderEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags);
        
        END_INTERFACE
    } IXFeedFolderEventsVtbl;

    interface IXFeedFolderEvents
    {
        CONST_VTBL struct IXFeedFolderEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedFolderEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedFolderEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedFolderEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedFolderEvents_Error(This)	\
    ( (This)->lpVtbl -> Error(This) ) 

#define IXFeedFolderEvents_FolderAdded(This,pszPath)	\
    ( (This)->lpVtbl -> FolderAdded(This,pszPath) ) 

#define IXFeedFolderEvents_FolderDeleted(This,pszPath)	\
    ( (This)->lpVtbl -> FolderDeleted(This,pszPath) ) 

#define IXFeedFolderEvents_FolderRenamed(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FolderRenamed(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FolderMovedFrom(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FolderMovedFrom(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FolderMovedTo(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FolderMovedTo(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FolderItemCountChanged(This,pszPath,feicfFlags)	\
    ( (This)->lpVtbl -> FolderItemCountChanged(This,pszPath,feicfFlags) ) 

#define IXFeedFolderEvents_FeedAdded(This,pszPath)	\
    ( (This)->lpVtbl -> FeedAdded(This,pszPath) ) 

#define IXFeedFolderEvents_FeedDeleted(This,pszPath)	\
    ( (This)->lpVtbl -> FeedDeleted(This,pszPath) ) 

#define IXFeedFolderEvents_FeedRenamed(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FeedRenamed(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FeedUrlChanged(This,pszPath)	\
    ( (This)->lpVtbl -> FeedUrlChanged(This,pszPath) ) 

#define IXFeedFolderEvents_FeedMovedFrom(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FeedMovedFrom(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FeedMovedTo(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FeedMovedTo(This,pszPath,pszOldPath) ) 

#define IXFeedFolderEvents_FeedDownloading(This,pszPath)	\
    ( (This)->lpVtbl -> FeedDownloading(This,pszPath) ) 

#define IXFeedFolderEvents_FeedDownloadCompleted(This,pszPath,fde)	\
    ( (This)->lpVtbl -> FeedDownloadCompleted(This,pszPath,fde) ) 

#define IXFeedFolderEvents_FeedItemCountChanged(This,pszPath,feicfFlags)	\
    ( (This)->lpVtbl -> FeedItemCountChanged(This,pszPath,feicfFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedFolderEvents_INTERFACE_DEFINED__ */


#ifndef __IXFeed_INTERFACE_DEFINED__
#define __IXFeed_INTERFACE_DEFINED__

/* interface IXFeed */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeed;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a44179a4-e0f6-403b-af8d-d080f425a451")
    IXFeed : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Xml( 
            /* [in] */ UINT uiItemCount,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [out] */ __RPC__deref_out_opt IStream **pps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Name( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ __RPC__in LPCWSTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Url( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUrl( 
            /* [in] */ __RPC__in LPCWSTR pszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LocalId( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Path( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Parent( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastWriteTime( 
            /* [out] */ __RPC__out SYSTEMTIME *pstLastWriteTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Download( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsyncDownload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelAsyncDownload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncSetting( 
            /* [out] */ __RPC__out FEEDS_SYNC_SETTING *pfss) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncSetting( 
            /* [in] */ FEEDS_SYNC_SETTING fss) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Interval( 
            /* [out] */ __RPC__out UINT *puiInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInterval( 
            /* [in] */ UINT uiInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastDownloadTime( 
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LocalEnclosurePath( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Items( 
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItem( 
            /* [in] */ UINT uiId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MarkAllItemsRead( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MaxItemCount( 
            /* [out] */ __RPC__out UINT *puiMaxItemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxItemCount( 
            /* [in] */ UINT uiMaxItemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadEnclosuresAutomatically( 
            /* [out] */ __RPC__out BOOL *pbDownloadEnclosuresAutomatically) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDownloadEnclosuresAutomatically( 
            /* [in] */ BOOL bDownloadEnclosuresAutomatically) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadStatus( 
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *pfds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastDownloadError( 
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *pfde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Merge( 
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in LPCWSTR pszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadUrl( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Title( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Description( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Link( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszHomePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Image( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszImageUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastBuildDate( 
            /* [out] */ __RPC__out SYSTEMTIME *pstLastBuildDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PubDate( 
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Ttl( 
            /* [out] */ __RPC__out UINT *puiTtl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Language( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Copyright( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCopyright) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsList( 
            /* [out] */ __RPC__out BOOL *pbIsList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWatcher( 
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnreadItemCount( 
            /* [out] */ __RPC__out UINT *puiUnreadItemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ItemCount( 
            /* [out] */ __RPC__out UINT *puiItemCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, Xml)
        HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IXFeed * This,
            /* [in] */ UINT uiItemCount,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [out] */ __RPC__deref_out_opt IStream **pps);
        
        DECLSPEC_XFGVIRT(IXFeed, Name)
        HRESULT ( STDMETHODCALLTYPE *Name )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IXFeed, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IXFeed, Url)
        HRESULT ( STDMETHODCALLTYPE *Url )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, SetUrl)
        HRESULT ( STDMETHODCALLTYPE *SetUrl )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in LPCWSTR pszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, LocalId)
        HRESULT ( STDMETHODCALLTYPE *LocalId )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(IXFeed, Path)
        HRESULT ( STDMETHODCALLTYPE *Path )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, LastWriteTime)
        HRESULT ( STDMETHODCALLTYPE *LastWriteTime )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastWriteTime);
        
        DECLSPEC_XFGVIRT(IXFeed, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, Download)
        HRESULT ( STDMETHODCALLTYPE *Download )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, AsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, CancelAsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, SyncSetting)
        HRESULT ( STDMETHODCALLTYPE *SyncSetting )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out FEEDS_SYNC_SETTING *pfss);
        
        DECLSPEC_XFGVIRT(IXFeed, SetSyncSetting)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSetting )( 
            __RPC__in IXFeed * This,
            /* [in] */ FEEDS_SYNC_SETTING fss);
        
        DECLSPEC_XFGVIRT(IXFeed, Interval)
        HRESULT ( STDMETHODCALLTYPE *Interval )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out UINT *puiInterval);
        
        DECLSPEC_XFGVIRT(IXFeed, SetInterval)
        HRESULT ( STDMETHODCALLTYPE *SetInterval )( 
            __RPC__in IXFeed * This,
            /* [in] */ UINT uiInterval);
        
        DECLSPEC_XFGVIRT(IXFeed, LastDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadTime )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime);
        
        DECLSPEC_XFGVIRT(IXFeed, LocalEnclosurePath)
        HRESULT ( STDMETHODCALLTYPE *LocalEnclosurePath )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Items)
        HRESULT ( STDMETHODCALLTYPE *Items )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe);
        
        DECLSPEC_XFGVIRT(IXFeed, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IXFeed * This,
            /* [in] */ UINT uiId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, MarkAllItemsRead)
        HRESULT ( STDMETHODCALLTYPE *MarkAllItemsRead )( 
            __RPC__in IXFeed * This);
        
        DECLSPEC_XFGVIRT(IXFeed, MaxItemCount)
        HRESULT ( STDMETHODCALLTYPE *MaxItemCount )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out UINT *puiMaxItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, SetMaxItemCount)
        HRESULT ( STDMETHODCALLTYPE *SetMaxItemCount )( 
            __RPC__in IXFeed * This,
            /* [in] */ UINT uiMaxItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadEnclosuresAutomatically)
        HRESULT ( STDMETHODCALLTYPE *DownloadEnclosuresAutomatically )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out BOOL *pbDownloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IXFeed, SetDownloadEnclosuresAutomatically)
        HRESULT ( STDMETHODCALLTYPE *SetDownloadEnclosuresAutomatically )( 
            __RPC__in IXFeed * This,
            /* [in] */ BOOL bDownloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadStatus)
        HRESULT ( STDMETHODCALLTYPE *DownloadStatus )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *pfds);
        
        DECLSPEC_XFGVIRT(IXFeed, LastDownloadError)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadError )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *pfde);
        
        DECLSPEC_XFGVIRT(IXFeed, Merge)
        HRESULT ( STDMETHODCALLTYPE *Merge )( 
            __RPC__in IXFeed * This,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in LPCWSTR pszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *DownloadUrl )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, Title)
        HRESULT ( STDMETHODCALLTYPE *Title )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle);
        
        DECLSPEC_XFGVIRT(IXFeed, Description)
        HRESULT ( STDMETHODCALLTYPE *Description )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IXFeed, Link)
        HRESULT ( STDMETHODCALLTYPE *Link )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszHomePage);
        
        DECLSPEC_XFGVIRT(IXFeed, Image)
        HRESULT ( STDMETHODCALLTYPE *Image )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszImageUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, LastBuildDate)
        HRESULT ( STDMETHODCALLTYPE *LastBuildDate )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastBuildDate);
        
        DECLSPEC_XFGVIRT(IXFeed, PubDate)
        HRESULT ( STDMETHODCALLTYPE *PubDate )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate);
        
        DECLSPEC_XFGVIRT(IXFeed, Ttl)
        HRESULT ( STDMETHODCALLTYPE *Ttl )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out UINT *puiTtl);
        
        DECLSPEC_XFGVIRT(IXFeed, Language)
        HRESULT ( STDMETHODCALLTYPE *Language )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszLanguage);
        
        DECLSPEC_XFGVIRT(IXFeed, Copyright)
        HRESULT ( STDMETHODCALLTYPE *Copyright )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCopyright);
        
        DECLSPEC_XFGVIRT(IXFeed, IsList)
        HRESULT ( STDMETHODCALLTYPE *IsList )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out BOOL *pbIsList);
        
        DECLSPEC_XFGVIRT(IXFeed, GetWatcher)
        HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IXFeed * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, UnreadItemCount)
        HRESULT ( STDMETHODCALLTYPE *UnreadItemCount )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out UINT *puiUnreadItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, ItemCount)
        HRESULT ( STDMETHODCALLTYPE *ItemCount )( 
            __RPC__in IXFeed * This,
            /* [out] */ __RPC__out UINT *puiItemCount);
        
        END_INTERFACE
    } IXFeedVtbl;

    interface IXFeed
    {
        CONST_VTBL struct IXFeedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeed_Xml(This,uiItemCount,sortProperty,sortOrder,filterFlags,includeFlags,pps)	\
    ( (This)->lpVtbl -> Xml(This,uiItemCount,sortProperty,sortOrder,filterFlags,includeFlags,pps) ) 

#define IXFeed_Name(This,ppszName)	\
    ( (This)->lpVtbl -> Name(This,ppszName) ) 

#define IXFeed_Rename(This,pszName)	\
    ( (This)->lpVtbl -> Rename(This,pszName) ) 

#define IXFeed_Url(This,ppszUrl)	\
    ( (This)->lpVtbl -> Url(This,ppszUrl) ) 

#define IXFeed_SetUrl(This,pszUrl)	\
    ( (This)->lpVtbl -> SetUrl(This,pszUrl) ) 

#define IXFeed_LocalId(This,pguid)	\
    ( (This)->lpVtbl -> LocalId(This,pguid) ) 

#define IXFeed_Path(This,ppszPath)	\
    ( (This)->lpVtbl -> Path(This,ppszPath) ) 

#define IXFeed_Move(This,pszPath)	\
    ( (This)->lpVtbl -> Move(This,pszPath) ) 

#define IXFeed_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeed_LastWriteTime(This,pstLastWriteTime)	\
    ( (This)->lpVtbl -> LastWriteTime(This,pstLastWriteTime) ) 

#define IXFeed_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IXFeed_Download(This)	\
    ( (This)->lpVtbl -> Download(This) ) 

#define IXFeed_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IXFeed_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IXFeed_SyncSetting(This,pfss)	\
    ( (This)->lpVtbl -> SyncSetting(This,pfss) ) 

#define IXFeed_SetSyncSetting(This,fss)	\
    ( (This)->lpVtbl -> SetSyncSetting(This,fss) ) 

#define IXFeed_Interval(This,puiInterval)	\
    ( (This)->lpVtbl -> Interval(This,puiInterval) ) 

#define IXFeed_SetInterval(This,uiInterval)	\
    ( (This)->lpVtbl -> SetInterval(This,uiInterval) ) 

#define IXFeed_LastDownloadTime(This,pstLastDownloadTime)	\
    ( (This)->lpVtbl -> LastDownloadTime(This,pstLastDownloadTime) ) 

#define IXFeed_LocalEnclosurePath(This,ppszPath)	\
    ( (This)->lpVtbl -> LocalEnclosurePath(This,ppszPath) ) 

#define IXFeed_Items(This,ppfe)	\
    ( (This)->lpVtbl -> Items(This,ppfe) ) 

#define IXFeed_GetItem(This,uiId,riid,ppv)	\
    ( (This)->lpVtbl -> GetItem(This,uiId,riid,ppv) ) 

#define IXFeed_MarkAllItemsRead(This)	\
    ( (This)->lpVtbl -> MarkAllItemsRead(This) ) 

#define IXFeed_MaxItemCount(This,puiMaxItemCount)	\
    ( (This)->lpVtbl -> MaxItemCount(This,puiMaxItemCount) ) 

#define IXFeed_SetMaxItemCount(This,uiMaxItemCount)	\
    ( (This)->lpVtbl -> SetMaxItemCount(This,uiMaxItemCount) ) 

#define IXFeed_DownloadEnclosuresAutomatically(This,pbDownloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> DownloadEnclosuresAutomatically(This,pbDownloadEnclosuresAutomatically) ) 

#define IXFeed_SetDownloadEnclosuresAutomatically(This,bDownloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> SetDownloadEnclosuresAutomatically(This,bDownloadEnclosuresAutomatically) ) 

#define IXFeed_DownloadStatus(This,pfds)	\
    ( (This)->lpVtbl -> DownloadStatus(This,pfds) ) 

#define IXFeed_LastDownloadError(This,pfde)	\
    ( (This)->lpVtbl -> LastDownloadError(This,pfde) ) 

#define IXFeed_Merge(This,pStream,pszUrl)	\
    ( (This)->lpVtbl -> Merge(This,pStream,pszUrl) ) 

#define IXFeed_DownloadUrl(This,ppszUrl)	\
    ( (This)->lpVtbl -> DownloadUrl(This,ppszUrl) ) 

#define IXFeed_Title(This,ppszTitle)	\
    ( (This)->lpVtbl -> Title(This,ppszTitle) ) 

#define IXFeed_Description(This,ppszDescription)	\
    ( (This)->lpVtbl -> Description(This,ppszDescription) ) 

#define IXFeed_Link(This,ppszHomePage)	\
    ( (This)->lpVtbl -> Link(This,ppszHomePage) ) 

#define IXFeed_Image(This,ppszImageUrl)	\
    ( (This)->lpVtbl -> Image(This,ppszImageUrl) ) 

#define IXFeed_LastBuildDate(This,pstLastBuildDate)	\
    ( (This)->lpVtbl -> LastBuildDate(This,pstLastBuildDate) ) 

#define IXFeed_PubDate(This,pstPubDate)	\
    ( (This)->lpVtbl -> PubDate(This,pstPubDate) ) 

#define IXFeed_Ttl(This,puiTtl)	\
    ( (This)->lpVtbl -> Ttl(This,puiTtl) ) 

#define IXFeed_Language(This,ppszLanguage)	\
    ( (This)->lpVtbl -> Language(This,ppszLanguage) ) 

#define IXFeed_Copyright(This,ppszCopyright)	\
    ( (This)->lpVtbl -> Copyright(This,ppszCopyright) ) 

#define IXFeed_IsList(This,pbIsList)	\
    ( (This)->lpVtbl -> IsList(This,pbIsList) ) 

#define IXFeed_GetWatcher(This,scope,mask,riid,ppv)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,riid,ppv) ) 

#define IXFeed_UnreadItemCount(This,puiUnreadItemCount)	\
    ( (This)->lpVtbl -> UnreadItemCount(This,puiUnreadItemCount) ) 

#define IXFeed_ItemCount(This,puiItemCount)	\
    ( (This)->lpVtbl -> ItemCount(This,puiItemCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeed_INTERFACE_DEFINED__ */


#ifndef __IXFeed2_INTERFACE_DEFINED__
#define __IXFeed2_INTERFACE_DEFINED__

/* interface IXFeed2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeed2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ce528e77-3716-4eb7-956d-f5e37502e12a")
    IXFeed2 : public IXFeed
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetItemByEffectiveId( 
            /* [in] */ UINT uiEffectiveId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastItemDownloadTime( 
            /* [out] */ __RPC__out SYSTEMTIME *pstLastItemDownloadTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Username( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUsername) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Password( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCredentials( 
            /* [in] */ __RPC__in LPCWSTR pszUsername,
            /* [in] */ __RPC__in LPCWSTR pszPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearCredentials( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeed2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, Xml)
        HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ UINT uiItemCount,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [out] */ __RPC__deref_out_opt IStream **pps);
        
        DECLSPEC_XFGVIRT(IXFeed, Name)
        HRESULT ( STDMETHODCALLTYPE *Name )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IXFeed, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IXFeed, Url)
        HRESULT ( STDMETHODCALLTYPE *Url )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, SetUrl)
        HRESULT ( STDMETHODCALLTYPE *SetUrl )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in LPCWSTR pszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, LocalId)
        HRESULT ( STDMETHODCALLTYPE *LocalId )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(IXFeed, Path)
        HRESULT ( STDMETHODCALLTYPE *Path )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, LastWriteTime)
        HRESULT ( STDMETHODCALLTYPE *LastWriteTime )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastWriteTime);
        
        DECLSPEC_XFGVIRT(IXFeed, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, Download)
        HRESULT ( STDMETHODCALLTYPE *Download )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, AsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, CancelAsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, SyncSetting)
        HRESULT ( STDMETHODCALLTYPE *SyncSetting )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out FEEDS_SYNC_SETTING *pfss);
        
        DECLSPEC_XFGVIRT(IXFeed, SetSyncSetting)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSetting )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ FEEDS_SYNC_SETTING fss);
        
        DECLSPEC_XFGVIRT(IXFeed, Interval)
        HRESULT ( STDMETHODCALLTYPE *Interval )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out UINT *puiInterval);
        
        DECLSPEC_XFGVIRT(IXFeed, SetInterval)
        HRESULT ( STDMETHODCALLTYPE *SetInterval )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ UINT uiInterval);
        
        DECLSPEC_XFGVIRT(IXFeed, LastDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadTime )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime);
        
        DECLSPEC_XFGVIRT(IXFeed, LocalEnclosurePath)
        HRESULT ( STDMETHODCALLTYPE *LocalEnclosurePath )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeed, Items)
        HRESULT ( STDMETHODCALLTYPE *Items )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt IXFeedsEnum **ppfe);
        
        DECLSPEC_XFGVIRT(IXFeed, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ UINT uiId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, MarkAllItemsRead)
        HRESULT ( STDMETHODCALLTYPE *MarkAllItemsRead )( 
            __RPC__in IXFeed2 * This);
        
        DECLSPEC_XFGVIRT(IXFeed, MaxItemCount)
        HRESULT ( STDMETHODCALLTYPE *MaxItemCount )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out UINT *puiMaxItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, SetMaxItemCount)
        HRESULT ( STDMETHODCALLTYPE *SetMaxItemCount )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ UINT uiMaxItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadEnclosuresAutomatically)
        HRESULT ( STDMETHODCALLTYPE *DownloadEnclosuresAutomatically )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out BOOL *pbDownloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IXFeed, SetDownloadEnclosuresAutomatically)
        HRESULT ( STDMETHODCALLTYPE *SetDownloadEnclosuresAutomatically )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ BOOL bDownloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadStatus)
        HRESULT ( STDMETHODCALLTYPE *DownloadStatus )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *pfds);
        
        DECLSPEC_XFGVIRT(IXFeed, LastDownloadError)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadError )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *pfde);
        
        DECLSPEC_XFGVIRT(IXFeed, Merge)
        HRESULT ( STDMETHODCALLTYPE *Merge )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in LPCWSTR pszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, DownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *DownloadUrl )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, Title)
        HRESULT ( STDMETHODCALLTYPE *Title )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle);
        
        DECLSPEC_XFGVIRT(IXFeed, Description)
        HRESULT ( STDMETHODCALLTYPE *Description )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IXFeed, Link)
        HRESULT ( STDMETHODCALLTYPE *Link )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszHomePage);
        
        DECLSPEC_XFGVIRT(IXFeed, Image)
        HRESULT ( STDMETHODCALLTYPE *Image )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszImageUrl);
        
        DECLSPEC_XFGVIRT(IXFeed, LastBuildDate)
        HRESULT ( STDMETHODCALLTYPE *LastBuildDate )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastBuildDate);
        
        DECLSPEC_XFGVIRT(IXFeed, PubDate)
        HRESULT ( STDMETHODCALLTYPE *PubDate )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate);
        
        DECLSPEC_XFGVIRT(IXFeed, Ttl)
        HRESULT ( STDMETHODCALLTYPE *Ttl )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out UINT *puiTtl);
        
        DECLSPEC_XFGVIRT(IXFeed, Language)
        HRESULT ( STDMETHODCALLTYPE *Language )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszLanguage);
        
        DECLSPEC_XFGVIRT(IXFeed, Copyright)
        HRESULT ( STDMETHODCALLTYPE *Copyright )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCopyright);
        
        DECLSPEC_XFGVIRT(IXFeed, IsList)
        HRESULT ( STDMETHODCALLTYPE *IsList )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out BOOL *pbIsList);
        
        DECLSPEC_XFGVIRT(IXFeed, GetWatcher)
        HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed, UnreadItemCount)
        HRESULT ( STDMETHODCALLTYPE *UnreadItemCount )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out UINT *puiUnreadItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed, ItemCount)
        HRESULT ( STDMETHODCALLTYPE *ItemCount )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out UINT *puiItemCount);
        
        DECLSPEC_XFGVIRT(IXFeed2, GetItemByEffectiveId)
        HRESULT ( STDMETHODCALLTYPE *GetItemByEffectiveId )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ UINT uiEffectiveId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeed2, LastItemDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *LastItemDownloadTime )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastItemDownloadTime);
        
        DECLSPEC_XFGVIRT(IXFeed2, Username)
        HRESULT ( STDMETHODCALLTYPE *Username )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUsername);
        
        DECLSPEC_XFGVIRT(IXFeed2, Password)
        HRESULT ( STDMETHODCALLTYPE *Password )( 
            __RPC__in IXFeed2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPassword);
        
        DECLSPEC_XFGVIRT(IXFeed2, SetCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IXFeed2 * This,
            /* [in] */ __RPC__in LPCWSTR pszUsername,
            /* [in] */ __RPC__in LPCWSTR pszPassword);
        
        DECLSPEC_XFGVIRT(IXFeed2, ClearCredentials)
        HRESULT ( STDMETHODCALLTYPE *ClearCredentials )( 
            __RPC__in IXFeed2 * This);
        
        END_INTERFACE
    } IXFeed2Vtbl;

    interface IXFeed2
    {
        CONST_VTBL struct IXFeed2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeed2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeed2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeed2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeed2_Xml(This,uiItemCount,sortProperty,sortOrder,filterFlags,includeFlags,pps)	\
    ( (This)->lpVtbl -> Xml(This,uiItemCount,sortProperty,sortOrder,filterFlags,includeFlags,pps) ) 

#define IXFeed2_Name(This,ppszName)	\
    ( (This)->lpVtbl -> Name(This,ppszName) ) 

#define IXFeed2_Rename(This,pszName)	\
    ( (This)->lpVtbl -> Rename(This,pszName) ) 

#define IXFeed2_Url(This,ppszUrl)	\
    ( (This)->lpVtbl -> Url(This,ppszUrl) ) 

#define IXFeed2_SetUrl(This,pszUrl)	\
    ( (This)->lpVtbl -> SetUrl(This,pszUrl) ) 

#define IXFeed2_LocalId(This,pguid)	\
    ( (This)->lpVtbl -> LocalId(This,pguid) ) 

#define IXFeed2_Path(This,ppszPath)	\
    ( (This)->lpVtbl -> Path(This,ppszPath) ) 

#define IXFeed2_Move(This,pszPath)	\
    ( (This)->lpVtbl -> Move(This,pszPath) ) 

#define IXFeed2_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeed2_LastWriteTime(This,pstLastWriteTime)	\
    ( (This)->lpVtbl -> LastWriteTime(This,pstLastWriteTime) ) 

#define IXFeed2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IXFeed2_Download(This)	\
    ( (This)->lpVtbl -> Download(This) ) 

#define IXFeed2_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IXFeed2_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IXFeed2_SyncSetting(This,pfss)	\
    ( (This)->lpVtbl -> SyncSetting(This,pfss) ) 

#define IXFeed2_SetSyncSetting(This,fss)	\
    ( (This)->lpVtbl -> SetSyncSetting(This,fss) ) 

#define IXFeed2_Interval(This,puiInterval)	\
    ( (This)->lpVtbl -> Interval(This,puiInterval) ) 

#define IXFeed2_SetInterval(This,uiInterval)	\
    ( (This)->lpVtbl -> SetInterval(This,uiInterval) ) 

#define IXFeed2_LastDownloadTime(This,pstLastDownloadTime)	\
    ( (This)->lpVtbl -> LastDownloadTime(This,pstLastDownloadTime) ) 

#define IXFeed2_LocalEnclosurePath(This,ppszPath)	\
    ( (This)->lpVtbl -> LocalEnclosurePath(This,ppszPath) ) 

#define IXFeed2_Items(This,ppfe)	\
    ( (This)->lpVtbl -> Items(This,ppfe) ) 

#define IXFeed2_GetItem(This,uiId,riid,ppv)	\
    ( (This)->lpVtbl -> GetItem(This,uiId,riid,ppv) ) 

#define IXFeed2_MarkAllItemsRead(This)	\
    ( (This)->lpVtbl -> MarkAllItemsRead(This) ) 

#define IXFeed2_MaxItemCount(This,puiMaxItemCount)	\
    ( (This)->lpVtbl -> MaxItemCount(This,puiMaxItemCount) ) 

#define IXFeed2_SetMaxItemCount(This,uiMaxItemCount)	\
    ( (This)->lpVtbl -> SetMaxItemCount(This,uiMaxItemCount) ) 

#define IXFeed2_DownloadEnclosuresAutomatically(This,pbDownloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> DownloadEnclosuresAutomatically(This,pbDownloadEnclosuresAutomatically) ) 

#define IXFeed2_SetDownloadEnclosuresAutomatically(This,bDownloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> SetDownloadEnclosuresAutomatically(This,bDownloadEnclosuresAutomatically) ) 

#define IXFeed2_DownloadStatus(This,pfds)	\
    ( (This)->lpVtbl -> DownloadStatus(This,pfds) ) 

#define IXFeed2_LastDownloadError(This,pfde)	\
    ( (This)->lpVtbl -> LastDownloadError(This,pfde) ) 

#define IXFeed2_Merge(This,pStream,pszUrl)	\
    ( (This)->lpVtbl -> Merge(This,pStream,pszUrl) ) 

#define IXFeed2_DownloadUrl(This,ppszUrl)	\
    ( (This)->lpVtbl -> DownloadUrl(This,ppszUrl) ) 

#define IXFeed2_Title(This,ppszTitle)	\
    ( (This)->lpVtbl -> Title(This,ppszTitle) ) 

#define IXFeed2_Description(This,ppszDescription)	\
    ( (This)->lpVtbl -> Description(This,ppszDescription) ) 

#define IXFeed2_Link(This,ppszHomePage)	\
    ( (This)->lpVtbl -> Link(This,ppszHomePage) ) 

#define IXFeed2_Image(This,ppszImageUrl)	\
    ( (This)->lpVtbl -> Image(This,ppszImageUrl) ) 

#define IXFeed2_LastBuildDate(This,pstLastBuildDate)	\
    ( (This)->lpVtbl -> LastBuildDate(This,pstLastBuildDate) ) 

#define IXFeed2_PubDate(This,pstPubDate)	\
    ( (This)->lpVtbl -> PubDate(This,pstPubDate) ) 

#define IXFeed2_Ttl(This,puiTtl)	\
    ( (This)->lpVtbl -> Ttl(This,puiTtl) ) 

#define IXFeed2_Language(This,ppszLanguage)	\
    ( (This)->lpVtbl -> Language(This,ppszLanguage) ) 

#define IXFeed2_Copyright(This,ppszCopyright)	\
    ( (This)->lpVtbl -> Copyright(This,ppszCopyright) ) 

#define IXFeed2_IsList(This,pbIsList)	\
    ( (This)->lpVtbl -> IsList(This,pbIsList) ) 

#define IXFeed2_GetWatcher(This,scope,mask,riid,ppv)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,riid,ppv) ) 

#define IXFeed2_UnreadItemCount(This,puiUnreadItemCount)	\
    ( (This)->lpVtbl -> UnreadItemCount(This,puiUnreadItemCount) ) 

#define IXFeed2_ItemCount(This,puiItemCount)	\
    ( (This)->lpVtbl -> ItemCount(This,puiItemCount) ) 


#define IXFeed2_GetItemByEffectiveId(This,uiEffectiveId,riid,ppv)	\
    ( (This)->lpVtbl -> GetItemByEffectiveId(This,uiEffectiveId,riid,ppv) ) 

#define IXFeed2_LastItemDownloadTime(This,pstLastItemDownloadTime)	\
    ( (This)->lpVtbl -> LastItemDownloadTime(This,pstLastItemDownloadTime) ) 

#define IXFeed2_Username(This,ppszUsername)	\
    ( (This)->lpVtbl -> Username(This,ppszUsername) ) 

#define IXFeed2_Password(This,ppszPassword)	\
    ( (This)->lpVtbl -> Password(This,ppszPassword) ) 

#define IXFeed2_SetCredentials(This,pszUsername,pszPassword)	\
    ( (This)->lpVtbl -> SetCredentials(This,pszUsername,pszPassword) ) 

#define IXFeed2_ClearCredentials(This)	\
    ( (This)->lpVtbl -> ClearCredentials(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeed2_INTERFACE_DEFINED__ */


#ifndef __IXFeedEvents_INTERFACE_DEFINED__
#define __IXFeedEvents_INTERFACE_DEFINED__

/* interface IXFeedEvents */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1630852e-1263-465b-98e5-fe60ffec4ac2")
    IXFeedEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Error( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDeleted( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedRenamed( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedUrlChanged( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedMoved( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDownloading( 
            /* [in] */ __RPC__in LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedDownloadCompleted( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEEDS_DOWNLOAD_ERROR fde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FeedItemCountChanged( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, Error)
        HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IXFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedDeleted)
        HRESULT ( STDMETHODCALLTYPE *FeedDeleted )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedRenamed)
        HRESULT ( STDMETHODCALLTYPE *FeedRenamed )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedUrlChanged)
        HRESULT ( STDMETHODCALLTYPE *FeedUrlChanged )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedMoved)
        HRESULT ( STDMETHODCALLTYPE *FeedMoved )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ __RPC__in LPCWSTR pszOldPath);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedDownloading)
        HRESULT ( STDMETHODCALLTYPE *FeedDownloading )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedDownloadCompleted)
        HRESULT ( STDMETHODCALLTYPE *FeedDownloadCompleted )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEEDS_DOWNLOAD_ERROR fde);
        
        DECLSPEC_XFGVIRT(IXFeedEvents, FeedItemCountChanged)
        HRESULT ( STDMETHODCALLTYPE *FeedItemCountChanged )( 
            __RPC__in IXFeedEvents * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [in] */ FEICF feicfFlags);
        
        END_INTERFACE
    } IXFeedEventsVtbl;

    interface IXFeedEvents
    {
        CONST_VTBL struct IXFeedEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedEvents_Error(This)	\
    ( (This)->lpVtbl -> Error(This) ) 

#define IXFeedEvents_FeedDeleted(This,pszPath)	\
    ( (This)->lpVtbl -> FeedDeleted(This,pszPath) ) 

#define IXFeedEvents_FeedRenamed(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FeedRenamed(This,pszPath,pszOldPath) ) 

#define IXFeedEvents_FeedUrlChanged(This,pszPath)	\
    ( (This)->lpVtbl -> FeedUrlChanged(This,pszPath) ) 

#define IXFeedEvents_FeedMoved(This,pszPath,pszOldPath)	\
    ( (This)->lpVtbl -> FeedMoved(This,pszPath,pszOldPath) ) 

#define IXFeedEvents_FeedDownloading(This,pszPath)	\
    ( (This)->lpVtbl -> FeedDownloading(This,pszPath) ) 

#define IXFeedEvents_FeedDownloadCompleted(This,pszPath,fde)	\
    ( (This)->lpVtbl -> FeedDownloadCompleted(This,pszPath,fde) ) 

#define IXFeedEvents_FeedItemCountChanged(This,pszPath,feicfFlags)	\
    ( (This)->lpVtbl -> FeedItemCountChanged(This,pszPath,feicfFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedEvents_INTERFACE_DEFINED__ */


#ifndef __IXFeedItem_INTERFACE_DEFINED__
#define __IXFeedItem_INTERFACE_DEFINED__

/* interface IXFeedItem */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e757b2f5-e73e-434e-a1bf-2bd7c3e60fcb")
    IXFeedItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Xml( 
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS fxif,
            /* [out] */ __RPC__deref_out_opt IStream **pps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Title( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Link( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Guid( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Description( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PubDate( 
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Comments( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Author( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszAuthor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enclosure( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRead( 
            /* [out] */ __RPC__out BOOL *pbIsRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsRead( 
            /* [in] */ BOOL bIsRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LocalId( 
            /* [out] */ __RPC__out UINT *puiId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Parent( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadUrl( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastDownloadTime( 
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Modified( 
            /* [out] */ __RPC__out SYSTEMTIME *pstModifiedTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedItem * This);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Xml)
        HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IXFeedItem * This,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS fxif,
            /* [out] */ __RPC__deref_out_opt IStream **pps);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Title)
        HRESULT ( STDMETHODCALLTYPE *Title )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Link)
        HRESULT ( STDMETHODCALLTYPE *Link )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Guid)
        HRESULT ( STDMETHODCALLTYPE *Guid )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszGuid);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Description)
        HRESULT ( STDMETHODCALLTYPE *Description )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IXFeedItem, PubDate)
        HRESULT ( STDMETHODCALLTYPE *PubDate )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Comments)
        HRESULT ( STDMETHODCALLTYPE *Comments )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Author)
        HRESULT ( STDMETHODCALLTYPE *Author )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszAuthor);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Enclosure)
        HRESULT ( STDMETHODCALLTYPE *Enclosure )( 
            __RPC__in IXFeedItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedItem, IsRead)
        HRESULT ( STDMETHODCALLTYPE *IsRead )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__out BOOL *pbIsRead);
        
        DECLSPEC_XFGVIRT(IXFeedItem, SetIsRead)
        HRESULT ( STDMETHODCALLTYPE *SetIsRead )( 
            __RPC__in IXFeedItem * This,
            /* [in] */ BOOL bIsRead);
        
        DECLSPEC_XFGVIRT(IXFeedItem, LocalId)
        HRESULT ( STDMETHODCALLTYPE *LocalId )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__out UINT *puiId);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeedItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IXFeedItem * This);
        
        DECLSPEC_XFGVIRT(IXFeedItem, DownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *DownloadUrl )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, LastDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadTime )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Modified)
        HRESULT ( STDMETHODCALLTYPE *Modified )( 
            __RPC__in IXFeedItem * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstModifiedTime);
        
        END_INTERFACE
    } IXFeedItemVtbl;

    interface IXFeedItem
    {
        CONST_VTBL struct IXFeedItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedItem_Xml(This,fxif,pps)	\
    ( (This)->lpVtbl -> Xml(This,fxif,pps) ) 

#define IXFeedItem_Title(This,ppszTitle)	\
    ( (This)->lpVtbl -> Title(This,ppszTitle) ) 

#define IXFeedItem_Link(This,ppszUrl)	\
    ( (This)->lpVtbl -> Link(This,ppszUrl) ) 

#define IXFeedItem_Guid(This,ppszGuid)	\
    ( (This)->lpVtbl -> Guid(This,ppszGuid) ) 

#define IXFeedItem_Description(This,ppszDescription)	\
    ( (This)->lpVtbl -> Description(This,ppszDescription) ) 

#define IXFeedItem_PubDate(This,pstPubDate)	\
    ( (This)->lpVtbl -> PubDate(This,pstPubDate) ) 

#define IXFeedItem_Comments(This,ppszUrl)	\
    ( (This)->lpVtbl -> Comments(This,ppszUrl) ) 

#define IXFeedItem_Author(This,ppszAuthor)	\
    ( (This)->lpVtbl -> Author(This,ppszAuthor) ) 

#define IXFeedItem_Enclosure(This,riid,ppv)	\
    ( (This)->lpVtbl -> Enclosure(This,riid,ppv) ) 

#define IXFeedItem_IsRead(This,pbIsRead)	\
    ( (This)->lpVtbl -> IsRead(This,pbIsRead) ) 

#define IXFeedItem_SetIsRead(This,bIsRead)	\
    ( (This)->lpVtbl -> SetIsRead(This,bIsRead) ) 

#define IXFeedItem_LocalId(This,puiId)	\
    ( (This)->lpVtbl -> LocalId(This,puiId) ) 

#define IXFeedItem_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeedItem_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IXFeedItem_DownloadUrl(This,ppszUrl)	\
    ( (This)->lpVtbl -> DownloadUrl(This,ppszUrl) ) 

#define IXFeedItem_LastDownloadTime(This,pstLastDownloadTime)	\
    ( (This)->lpVtbl -> LastDownloadTime(This,pstLastDownloadTime) ) 

#define IXFeedItem_Modified(This,pstModifiedTime)	\
    ( (This)->lpVtbl -> Modified(This,pstModifiedTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedItem_INTERFACE_DEFINED__ */


#ifndef __IXFeedItem2_INTERFACE_DEFINED__
#define __IXFeedItem2_INTERFACE_DEFINED__

/* interface IXFeedItem2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6cda2dc7-9013-4522-9970-2a9dd9ead5a3")
    IXFeedItem2 : public IXFeedItem
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EffectiveId( 
            /* [out] */ __RPC__out UINT *puiEffectiveId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Xml)
        HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IXFeedItem2 * This,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS fxif,
            /* [out] */ __RPC__deref_out_opt IStream **pps);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Title)
        HRESULT ( STDMETHODCALLTYPE *Title )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszTitle);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Link)
        HRESULT ( STDMETHODCALLTYPE *Link )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Guid)
        HRESULT ( STDMETHODCALLTYPE *Guid )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszGuid);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Description)
        HRESULT ( STDMETHODCALLTYPE *Description )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IXFeedItem, PubDate)
        HRESULT ( STDMETHODCALLTYPE *PubDate )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstPubDate);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Comments)
        HRESULT ( STDMETHODCALLTYPE *Comments )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Author)
        HRESULT ( STDMETHODCALLTYPE *Author )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszAuthor);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Enclosure)
        HRESULT ( STDMETHODCALLTYPE *Enclosure )( 
            __RPC__in IXFeedItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedItem, IsRead)
        HRESULT ( STDMETHODCALLTYPE *IsRead )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out BOOL *pbIsRead);
        
        DECLSPEC_XFGVIRT(IXFeedItem, SetIsRead)
        HRESULT ( STDMETHODCALLTYPE *SetIsRead )( 
            __RPC__in IXFeedItem2 * This,
            /* [in] */ BOOL bIsRead);
        
        DECLSPEC_XFGVIRT(IXFeedItem, LocalId)
        HRESULT ( STDMETHODCALLTYPE *LocalId )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out UINT *puiId);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeedItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IXFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IXFeedItem, DownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *DownloadUrl )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedItem, LastDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadTime )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstLastDownloadTime);
        
        DECLSPEC_XFGVIRT(IXFeedItem, Modified)
        HRESULT ( STDMETHODCALLTYPE *Modified )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out SYSTEMTIME *pstModifiedTime);
        
        DECLSPEC_XFGVIRT(IXFeedItem2, EffectiveId)
        HRESULT ( STDMETHODCALLTYPE *EffectiveId )( 
            __RPC__in IXFeedItem2 * This,
            /* [out] */ __RPC__out UINT *puiEffectiveId);
        
        END_INTERFACE
    } IXFeedItem2Vtbl;

    interface IXFeedItem2
    {
        CONST_VTBL struct IXFeedItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedItem2_Xml(This,fxif,pps)	\
    ( (This)->lpVtbl -> Xml(This,fxif,pps) ) 

#define IXFeedItem2_Title(This,ppszTitle)	\
    ( (This)->lpVtbl -> Title(This,ppszTitle) ) 

#define IXFeedItem2_Link(This,ppszUrl)	\
    ( (This)->lpVtbl -> Link(This,ppszUrl) ) 

#define IXFeedItem2_Guid(This,ppszGuid)	\
    ( (This)->lpVtbl -> Guid(This,ppszGuid) ) 

#define IXFeedItem2_Description(This,ppszDescription)	\
    ( (This)->lpVtbl -> Description(This,ppszDescription) ) 

#define IXFeedItem2_PubDate(This,pstPubDate)	\
    ( (This)->lpVtbl -> PubDate(This,pstPubDate) ) 

#define IXFeedItem2_Comments(This,ppszUrl)	\
    ( (This)->lpVtbl -> Comments(This,ppszUrl) ) 

#define IXFeedItem2_Author(This,ppszAuthor)	\
    ( (This)->lpVtbl -> Author(This,ppszAuthor) ) 

#define IXFeedItem2_Enclosure(This,riid,ppv)	\
    ( (This)->lpVtbl -> Enclosure(This,riid,ppv) ) 

#define IXFeedItem2_IsRead(This,pbIsRead)	\
    ( (This)->lpVtbl -> IsRead(This,pbIsRead) ) 

#define IXFeedItem2_SetIsRead(This,bIsRead)	\
    ( (This)->lpVtbl -> SetIsRead(This,bIsRead) ) 

#define IXFeedItem2_LocalId(This,puiId)	\
    ( (This)->lpVtbl -> LocalId(This,puiId) ) 

#define IXFeedItem2_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeedItem2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IXFeedItem2_DownloadUrl(This,ppszUrl)	\
    ( (This)->lpVtbl -> DownloadUrl(This,ppszUrl) ) 

#define IXFeedItem2_LastDownloadTime(This,pstLastDownloadTime)	\
    ( (This)->lpVtbl -> LastDownloadTime(This,pstLastDownloadTime) ) 

#define IXFeedItem2_Modified(This,pstModifiedTime)	\
    ( (This)->lpVtbl -> Modified(This,pstModifiedTime) ) 


#define IXFeedItem2_EffectiveId(This,puiEffectiveId)	\
    ( (This)->lpVtbl -> EffectiveId(This,puiEffectiveId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedItem2_INTERFACE_DEFINED__ */


#ifndef __IXFeedEnclosure_INTERFACE_DEFINED__
#define __IXFeedEnclosure_INTERFACE_DEFINED__

/* interface IXFeedEnclosure */
/* [object][uuid] */ 


EXTERN_C const IID IID_IXFeedEnclosure;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BFBFB953-644F-4792-B69C-DFACA4CBF89A")
    IXFeedEnclosure : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Url( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Type( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszMimeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Length( 
            /* [out] */ __RPC__out UINT *puiLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsyncDownload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelAsyncDownload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadStatus( 
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *pfds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LastDownloadError( 
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *pfde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LocalPath( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Parent( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadUrl( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadMimeType( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszMimeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFile( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFile( 
            /* [in] */ __RPC__in LPCWSTR pszDownloadUrl,
            /* [in] */ __RPC__in LPCWSTR pszDownloadFilePath,
            /* [in] */ __RPC__in LPCWSTR pszDownloadMimeType,
            /* [in] */ __RPC__in LPCWSTR pszEnclosureFilename) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXFeedEnclosureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXFeedEnclosure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, Url)
        HRESULT ( STDMETHODCALLTYPE *Url )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, Type)
        HRESULT ( STDMETHODCALLTYPE *Type )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszMimeType);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, Length)
        HRESULT ( STDMETHODCALLTYPE *Length )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__out UINT *puiLength);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, AsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IXFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, CancelAsyncDownload)
        HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IXFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, DownloadStatus)
        HRESULT ( STDMETHODCALLTYPE *DownloadStatus )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *pfds);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, LastDownloadError)
        HRESULT ( STDMETHODCALLTYPE *LastDownloadError )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *pfde);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, LocalPath)
        HRESULT ( STDMETHODCALLTYPE *LocalPath )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, Parent)
        HRESULT ( STDMETHODCALLTYPE *Parent )( 
            __RPC__in IXFeedEnclosure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, DownloadUrl)
        HRESULT ( STDMETHODCALLTYPE *DownloadUrl )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszUrl);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, DownloadMimeType)
        HRESULT ( STDMETHODCALLTYPE *DownloadMimeType )( 
            __RPC__in IXFeedEnclosure * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszMimeType);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, RemoveFile)
        HRESULT ( STDMETHODCALLTYPE *RemoveFile )( 
            __RPC__in IXFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IXFeedEnclosure, SetFile)
        HRESULT ( STDMETHODCALLTYPE *SetFile )( 
            __RPC__in IXFeedEnclosure * This,
            /* [in] */ __RPC__in LPCWSTR pszDownloadUrl,
            /* [in] */ __RPC__in LPCWSTR pszDownloadFilePath,
            /* [in] */ __RPC__in LPCWSTR pszDownloadMimeType,
            /* [in] */ __RPC__in LPCWSTR pszEnclosureFilename);
        
        END_INTERFACE
    } IXFeedEnclosureVtbl;

    interface IXFeedEnclosure
    {
        CONST_VTBL struct IXFeedEnclosureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXFeedEnclosure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXFeedEnclosure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXFeedEnclosure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXFeedEnclosure_Url(This,ppszUrl)	\
    ( (This)->lpVtbl -> Url(This,ppszUrl) ) 

#define IXFeedEnclosure_Type(This,ppszMimeType)	\
    ( (This)->lpVtbl -> Type(This,ppszMimeType) ) 

#define IXFeedEnclosure_Length(This,puiLength)	\
    ( (This)->lpVtbl -> Length(This,puiLength) ) 

#define IXFeedEnclosure_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IXFeedEnclosure_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IXFeedEnclosure_DownloadStatus(This,pfds)	\
    ( (This)->lpVtbl -> DownloadStatus(This,pfds) ) 

#define IXFeedEnclosure_LastDownloadError(This,pfde)	\
    ( (This)->lpVtbl -> LastDownloadError(This,pfde) ) 

#define IXFeedEnclosure_LocalPath(This,ppszPath)	\
    ( (This)->lpVtbl -> LocalPath(This,ppszPath) ) 

#define IXFeedEnclosure_Parent(This,riid,ppv)	\
    ( (This)->lpVtbl -> Parent(This,riid,ppv) ) 

#define IXFeedEnclosure_DownloadUrl(This,ppszUrl)	\
    ( (This)->lpVtbl -> DownloadUrl(This,ppszUrl) ) 

#define IXFeedEnclosure_DownloadMimeType(This,ppszMimeType)	\
    ( (This)->lpVtbl -> DownloadMimeType(This,ppszMimeType) ) 

#define IXFeedEnclosure_RemoveFile(This)	\
    ( (This)->lpVtbl -> RemoveFile(This) ) 

#define IXFeedEnclosure_SetFile(This,pszDownloadUrl,pszDownloadFilePath,pszDownloadMimeType,pszEnclosureFilename)	\
    ( (This)->lpVtbl -> SetFile(This,pszDownloadUrl,pszDownloadFilePath,pszDownloadMimeType,pszEnclosureFilename) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXFeedEnclosure_INTERFACE_DEFINED__ */



#ifndef __Feeds_LIBRARY_DEFINED__
#define __Feeds_LIBRARY_DEFINED__

/* library Feeds */
/* [custom][version][lcid][helpstring][uuid] */ 

#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion

EXTERN_C const IID LIBID_Feeds;

#ifndef __IFeedsManager_INTERFACE_DEFINED__
#define __IFeedsManager_INTERFACE_DEFINED__

/* interface IFeedsManager */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedsManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a74029cc-1f1a-4906-88f0-810638d86591")
    IFeedsManager : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RootFolder( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsSubscribed( 
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *subscribed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExistsFeed( 
            /* [in] */ __RPC__in BSTR feedPath,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFeed( 
            /* [in] */ __RPC__in BSTR feedPath,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFeedByUrl( 
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExistsFolder( 
            /* [in] */ __RPC__in BSTR folderPath,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFolder( 
            /* [in] */ __RPC__in BSTR folderPath,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteFeed( 
            /* [in] */ __RPC__in BSTR feedPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteFolder( 
            /* [in] */ __RPC__in BSTR folderPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BackgroundSync( 
            /* [in] */ FEEDS_BACKGROUNDSYNC_ACTION action) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackgroundSyncStatus( 
            /* [retval][out] */ __RPC__out FEEDS_BACKGROUNDSYNC_STATUS *status) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DefaultInterval( 
            /* [retval][out] */ __RPC__out LONG *minutes) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DefaultInterval( 
            /* [in] */ LONG minutes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AsyncSyncAll( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Normalize( 
            /* [in] */ __RPC__in BSTR feedXmlIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedXmlOut) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ItemCountLimit( 
            /* [retval][out] */ __RPC__out LONG *itemCountLimit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedsManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedsManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedsManager * This,
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
        
        DECLSPEC_XFGVIRT(IFeedsManager, get_RootFolder)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RootFolder )( 
            __RPC__in IFeedsManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedsManager, IsSubscribed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *subscribed);
        
        DECLSPEC_XFGVIRT(IFeedsManager, ExistsFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExistsFeed )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedPath,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IFeedsManager, GetFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFeed )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedPath,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedsManager, GetFeedByUrl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFeedByUrl )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedsManager, ExistsFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExistsFolder )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR folderPath,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IFeedsManager, GetFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFolder )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR folderPath,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedsManager, DeleteFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteFeed )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedPath);
        
        DECLSPEC_XFGVIRT(IFeedsManager, DeleteFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteFolder )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR folderPath);
        
        DECLSPEC_XFGVIRT(IFeedsManager, BackgroundSync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BackgroundSync )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ FEEDS_BACKGROUNDSYNC_ACTION action);
        
        DECLSPEC_XFGVIRT(IFeedsManager, get_BackgroundSyncStatus)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackgroundSyncStatus )( 
            __RPC__in IFeedsManager * This,
            /* [retval][out] */ __RPC__out FEEDS_BACKGROUNDSYNC_STATUS *status);
        
        DECLSPEC_XFGVIRT(IFeedsManager, get_DefaultInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultInterval )( 
            __RPC__in IFeedsManager * This,
            /* [retval][out] */ __RPC__out LONG *minutes);
        
        DECLSPEC_XFGVIRT(IFeedsManager, put_DefaultInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultInterval )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ LONG minutes);
        
        DECLSPEC_XFGVIRT(IFeedsManager, AsyncSyncAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AsyncSyncAll )( 
            __RPC__in IFeedsManager * This);
        
        DECLSPEC_XFGVIRT(IFeedsManager, Normalize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Normalize )( 
            __RPC__in IFeedsManager * This,
            /* [in] */ __RPC__in BSTR feedXmlIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedXmlOut);
        
        DECLSPEC_XFGVIRT(IFeedsManager, get_ItemCountLimit)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ItemCountLimit )( 
            __RPC__in IFeedsManager * This,
            /* [retval][out] */ __RPC__out LONG *itemCountLimit);
        
        END_INTERFACE
    } IFeedsManagerVtbl;

    interface IFeedsManager
    {
        CONST_VTBL struct IFeedsManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedsManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedsManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedsManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedsManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedsManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedsManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedsManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedsManager_get_RootFolder(This,disp)	\
    ( (This)->lpVtbl -> get_RootFolder(This,disp) ) 

#define IFeedsManager_IsSubscribed(This,feedUrl,subscribed)	\
    ( (This)->lpVtbl -> IsSubscribed(This,feedUrl,subscribed) ) 

#define IFeedsManager_ExistsFeed(This,feedPath,exists)	\
    ( (This)->lpVtbl -> ExistsFeed(This,feedPath,exists) ) 

#define IFeedsManager_GetFeed(This,feedPath,disp)	\
    ( (This)->lpVtbl -> GetFeed(This,feedPath,disp) ) 

#define IFeedsManager_GetFeedByUrl(This,feedUrl,disp)	\
    ( (This)->lpVtbl -> GetFeedByUrl(This,feedUrl,disp) ) 

#define IFeedsManager_ExistsFolder(This,folderPath,exists)	\
    ( (This)->lpVtbl -> ExistsFolder(This,folderPath,exists) ) 

#define IFeedsManager_GetFolder(This,folderPath,disp)	\
    ( (This)->lpVtbl -> GetFolder(This,folderPath,disp) ) 

#define IFeedsManager_DeleteFeed(This,feedPath)	\
    ( (This)->lpVtbl -> DeleteFeed(This,feedPath) ) 

#define IFeedsManager_DeleteFolder(This,folderPath)	\
    ( (This)->lpVtbl -> DeleteFolder(This,folderPath) ) 

#define IFeedsManager_BackgroundSync(This,action)	\
    ( (This)->lpVtbl -> BackgroundSync(This,action) ) 

#define IFeedsManager_get_BackgroundSyncStatus(This,status)	\
    ( (This)->lpVtbl -> get_BackgroundSyncStatus(This,status) ) 

#define IFeedsManager_get_DefaultInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_DefaultInterval(This,minutes) ) 

#define IFeedsManager_put_DefaultInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_DefaultInterval(This,minutes) ) 

#define IFeedsManager_AsyncSyncAll(This)	\
    ( (This)->lpVtbl -> AsyncSyncAll(This) ) 

#define IFeedsManager_Normalize(This,feedXmlIn,feedXmlOut)	\
    ( (This)->lpVtbl -> Normalize(This,feedXmlIn,feedXmlOut) ) 

#define IFeedsManager_get_ItemCountLimit(This,itemCountLimit)	\
    ( (This)->lpVtbl -> get_ItemCountLimit(This,itemCountLimit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedsManager_INTERFACE_DEFINED__ */


#ifndef __IFeedsEnum_INTERFACE_DEFINED__
#define __IFeedsEnum_INTERFACE_DEFINED__

/* interface IFeedsEnum */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedsEnum;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e3cd0028-2eed-4c60-8fae-a3225309a836")
    IFeedsEnum : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ LONG index,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [restricted][hidden][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **enumVar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedsEnumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedsEnum * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedsEnum * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedsEnum * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedsEnum * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedsEnum * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedsEnum * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedsEnum * This,
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
        
        DECLSPEC_XFGVIRT(IFeedsEnum, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFeedsEnum * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeedsEnum, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IFeedsEnum * This,
            /* [in] */ LONG index,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedsEnum, get__NewEnum)
        /* [restricted][hidden][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFeedsEnum * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **enumVar);
        
        END_INTERFACE
    } IFeedsEnumVtbl;

    interface IFeedsEnum
    {
        CONST_VTBL struct IFeedsEnumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedsEnum_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedsEnum_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedsEnum_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedsEnum_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedsEnum_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedsEnum_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedsEnum_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedsEnum_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IFeedsEnum_Item(This,index,disp)	\
    ( (This)->lpVtbl -> Item(This,index,disp) ) 

#define IFeedsEnum_get__NewEnum(This,enumVar)	\
    ( (This)->lpVtbl -> get__NewEnum(This,enumVar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedsEnum_INTERFACE_DEFINED__ */


#ifndef __IFeedFolder_INTERFACE_DEFINED__
#define __IFeedFolder_INTERFACE_DEFINED__

/* interface IFeedFolder */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedFolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81f04ad1-4194-4d7d-86d6-11813cec163c")
    IFeedFolder : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Feeds( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Subfolders( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateFeed( 
            /* [in] */ __RPC__in BSTR feedName,
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSubfolder( 
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExistsFeed( 
            /* [in] */ __RPC__in BSTR feedName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFeed( 
            /* [in] */ __RPC__in BSTR feedName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExistsSubfolder( 
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSubfolder( 
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folderName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ __RPC__in BSTR folderName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folderPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ __RPC__in BSTR newParentPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsRoot( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isRoot) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TotalUnreadItemCount( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TotalItemCount( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWatcher( 
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedFolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedFolder * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedFolder * This,
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
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_Feeds)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Feeds )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_Subfolders)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Subfolders )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, CreateFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFeed )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR feedName,
            /* [in] */ __RPC__in BSTR feedUrl,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, CreateSubfolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSubfolder )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, ExistsFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExistsFeed )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR feedName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IFeedFolder, GetFeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFeed )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR feedName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, ExistsSubfolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExistsSubfolder )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IFeedFolder, GetSubfolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubfolder )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR folderName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFeedFolder * This);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folderName);
        
        DECLSPEC_XFGVIRT(IFeedFolder, Rename)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR folderName);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folderPath);
        
        DECLSPEC_XFGVIRT(IFeedFolder, Move)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ __RPC__in BSTR newParentPath);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_IsRoot)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsRoot )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isRoot);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_TotalUnreadItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TotalUnreadItemCount )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeedFolder, get_TotalItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TotalItemCount )( 
            __RPC__in IFeedFolder * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeedFolder, GetWatcher)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IFeedFolder * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        END_INTERFACE
    } IFeedFolderVtbl;

    interface IFeedFolder
    {
        CONST_VTBL struct IFeedFolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedFolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedFolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedFolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedFolder_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedFolder_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedFolder_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedFolder_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedFolder_get_Feeds(This,disp)	\
    ( (This)->lpVtbl -> get_Feeds(This,disp) ) 

#define IFeedFolder_get_Subfolders(This,disp)	\
    ( (This)->lpVtbl -> get_Subfolders(This,disp) ) 

#define IFeedFolder_CreateFeed(This,feedName,feedUrl,disp)	\
    ( (This)->lpVtbl -> CreateFeed(This,feedName,feedUrl,disp) ) 

#define IFeedFolder_CreateSubfolder(This,folderName,disp)	\
    ( (This)->lpVtbl -> CreateSubfolder(This,folderName,disp) ) 

#define IFeedFolder_ExistsFeed(This,feedName,exists)	\
    ( (This)->lpVtbl -> ExistsFeed(This,feedName,exists) ) 

#define IFeedFolder_GetFeed(This,feedName,disp)	\
    ( (This)->lpVtbl -> GetFeed(This,feedName,disp) ) 

#define IFeedFolder_ExistsSubfolder(This,folderName,exists)	\
    ( (This)->lpVtbl -> ExistsSubfolder(This,folderName,exists) ) 

#define IFeedFolder_GetSubfolder(This,folderName,disp)	\
    ( (This)->lpVtbl -> GetSubfolder(This,folderName,disp) ) 

#define IFeedFolder_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFeedFolder_get_Name(This,folderName)	\
    ( (This)->lpVtbl -> get_Name(This,folderName) ) 

#define IFeedFolder_Rename(This,folderName)	\
    ( (This)->lpVtbl -> Rename(This,folderName) ) 

#define IFeedFolder_get_Path(This,folderPath)	\
    ( (This)->lpVtbl -> get_Path(This,folderPath) ) 

#define IFeedFolder_Move(This,newParentPath)	\
    ( (This)->lpVtbl -> Move(This,newParentPath) ) 

#define IFeedFolder_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeedFolder_get_IsRoot(This,isRoot)	\
    ( (This)->lpVtbl -> get_IsRoot(This,isRoot) ) 

#define IFeedFolder_get_TotalUnreadItemCount(This,count)	\
    ( (This)->lpVtbl -> get_TotalUnreadItemCount(This,count) ) 

#define IFeedFolder_get_TotalItemCount(This,count)	\
    ( (This)->lpVtbl -> get_TotalItemCount(This,count) ) 

#define IFeedFolder_GetWatcher(This,scope,mask,disp)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,disp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedFolder_INTERFACE_DEFINED__ */


#ifndef __IFeedFolderEvents_INTERFACE_DEFINED__
#define __IFeedFolderEvents_INTERFACE_DEFINED__

/* interface IFeedFolderEvents */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedFolderEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20a59fa6-a844-4630-9e98-175f70b4d55b")
    IFeedFolderEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Error( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderAdded( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderDeleted( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderRenamed( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderMovedFrom( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderMovedTo( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FolderItemCountChanged( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedAdded( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDeleted( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedRenamed( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedUrlChanged( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedMovedFrom( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedMovedTo( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDownloading( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDownloadCompleted( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ FEEDS_DOWNLOAD_ERROR error) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedItemCountChanged( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedFolderEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedFolderEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedFolderEvents * This,
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
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, Error)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IFeedFolderEvents * This);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderAdded )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderDeleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderDeleted )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderRenamed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderRenamed )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderMovedFrom)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderMovedFrom )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderMovedTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderMovedTo )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FolderItemCountChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FolderItemCountChanged )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedAdded )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedDeleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDeleted )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedRenamed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedRenamed )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedUrlChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedUrlChanged )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedMovedFrom)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedMovedFrom )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedMovedTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedMovedTo )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedDownloading)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDownloading )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedDownloadCompleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDownloadCompleted )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ FEEDS_DOWNLOAD_ERROR error);
        
        DECLSPEC_XFGVIRT(IFeedFolderEvents, FeedItemCountChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedItemCountChanged )( 
            __RPC__in IFeedFolderEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType);
        
        END_INTERFACE
    } IFeedFolderEventsVtbl;

    interface IFeedFolderEvents
    {
        CONST_VTBL struct IFeedFolderEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedFolderEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedFolderEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedFolderEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedFolderEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedFolderEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedFolderEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedFolderEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedFolderEvents_Error(This)	\
    ( (This)->lpVtbl -> Error(This) ) 

#define IFeedFolderEvents_FolderAdded(This,path)	\
    ( (This)->lpVtbl -> FolderAdded(This,path) ) 

#define IFeedFolderEvents_FolderDeleted(This,path)	\
    ( (This)->lpVtbl -> FolderDeleted(This,path) ) 

#define IFeedFolderEvents_FolderRenamed(This,path,oldPath)	\
    ( (This)->lpVtbl -> FolderRenamed(This,path,oldPath) ) 

#define IFeedFolderEvents_FolderMovedFrom(This,path,oldPath)	\
    ( (This)->lpVtbl -> FolderMovedFrom(This,path,oldPath) ) 

#define IFeedFolderEvents_FolderMovedTo(This,path,oldPath)	\
    ( (This)->lpVtbl -> FolderMovedTo(This,path,oldPath) ) 

#define IFeedFolderEvents_FolderItemCountChanged(This,path,itemCountType)	\
    ( (This)->lpVtbl -> FolderItemCountChanged(This,path,itemCountType) ) 

#define IFeedFolderEvents_FeedAdded(This,path)	\
    ( (This)->lpVtbl -> FeedAdded(This,path) ) 

#define IFeedFolderEvents_FeedDeleted(This,path)	\
    ( (This)->lpVtbl -> FeedDeleted(This,path) ) 

#define IFeedFolderEvents_FeedRenamed(This,path,oldPath)	\
    ( (This)->lpVtbl -> FeedRenamed(This,path,oldPath) ) 

#define IFeedFolderEvents_FeedUrlChanged(This,path)	\
    ( (This)->lpVtbl -> FeedUrlChanged(This,path) ) 

#define IFeedFolderEvents_FeedMovedFrom(This,path,oldPath)	\
    ( (This)->lpVtbl -> FeedMovedFrom(This,path,oldPath) ) 

#define IFeedFolderEvents_FeedMovedTo(This,path,oldPath)	\
    ( (This)->lpVtbl -> FeedMovedTo(This,path,oldPath) ) 

#define IFeedFolderEvents_FeedDownloading(This,path)	\
    ( (This)->lpVtbl -> FeedDownloading(This,path) ) 

#define IFeedFolderEvents_FeedDownloadCompleted(This,path,error)	\
    ( (This)->lpVtbl -> FeedDownloadCompleted(This,path,error) ) 

#define IFeedFolderEvents_FeedItemCountChanged(This,path,itemCountType)	\
    ( (This)->lpVtbl -> FeedItemCountChanged(This,path,itemCountType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedFolderEvents_INTERFACE_DEFINED__ */


#ifndef __IFeed_INTERFACE_DEFINED__
#define __IFeed_INTERFACE_DEFINED__

/* interface IFeed */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeed;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7f915d8-2ede-42bc-98e7-a5d05063a757")
    IFeed : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Xml( 
            /* [in] */ LONG count,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Url( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Url( 
            /* [in] */ __RPC__in BSTR feedUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocalId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedGuid) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ __RPC__in BSTR newParentPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastWriteTime( 
            /* [retval][out] */ __RPC__out DATE *lastWrite) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Download( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AsyncDownload( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelAsyncDownload( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SyncSetting( 
            /* [retval][out] */ __RPC__out FEEDS_SYNC_SETTING *syncSetting) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SyncSetting( 
            /* [in] */ FEEDS_SYNC_SETTING syncSetting) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Interval( 
            /* [retval][out] */ __RPC__out LONG *minutes) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Interval( 
            /* [in] */ LONG minutes) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastDownloadTime( 
            /* [retval][out] */ __RPC__out DATE *lastDownload) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocalEnclosurePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Items( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetItem( 
            /* [in] */ LONG itemId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Link( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *homePage) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Image( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *imageUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastBuildDate( 
            /* [retval][out] */ __RPC__out DATE *lastBuildDate) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_PubDate( 
            /* [retval][out] */ __RPC__out DATE *lastPopulateDate) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Ttl( 
            /* [retval][out] */ __RPC__out LONG *ttl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Language( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *language) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Copyright( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *copyright) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MaxItemCount( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaxItemCount( 
            /* [in] */ LONG count) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadEnclosuresAutomatically( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *downloadEnclosuresAutomatically) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DownloadEnclosuresAutomatically( 
            /* [in] */ VARIANT_BOOL downloadEnclosuresAutomatically) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadStatus( 
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *status) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastDownloadError( 
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *error) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Merge( 
            /* [in] */ __RPC__in BSTR feedXml,
            /* [in] */ __RPC__in BSTR feedUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadUrl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsList( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MarkAllItemsRead( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWatcher( 
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UnreadItemCount( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ItemCount( 
            /* [retval][out] */ __RPC__out LONG *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeed * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeed * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeed * This,
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
        
        DECLSPEC_XFGVIRT(IFeed, Xml)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IFeed * This,
            /* [in] */ LONG count,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml);
        
        DECLSPEC_XFGVIRT(IFeed, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFeed, Rename)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFeed, get_Url)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Url )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, put_Url)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Url )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in BSTR feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_LocalId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalId )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedGuid);
        
        DECLSPEC_XFGVIRT(IFeed, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFeed, Move)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in BSTR newParentPath);
        
        DECLSPEC_XFGVIRT(IFeed, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastWriteTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastWriteTime )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out DATE *lastWrite);
        
        DECLSPEC_XFGVIRT(IFeed, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IFeed, Download)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Download )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IFeed, AsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IFeed, CancelAsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IFeed, get_SyncSetting)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SyncSetting )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out FEEDS_SYNC_SETTING *syncSetting);
        
        DECLSPEC_XFGVIRT(IFeed, put_SyncSetting)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SyncSetting )( 
            __RPC__in IFeed * This,
            /* [in] */ FEEDS_SYNC_SETTING syncSetting);
        
        DECLSPEC_XFGVIRT(IFeed, get_Interval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Interval )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out LONG *minutes);
        
        DECLSPEC_XFGVIRT(IFeed, put_Interval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Interval )( 
            __RPC__in IFeed * This,
            /* [in] */ LONG minutes);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastDownloadTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadTime )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out DATE *lastDownload);
        
        DECLSPEC_XFGVIRT(IFeed, get_LocalEnclosurePath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalEnclosurePath )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFeed, get_Items)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Items )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, GetItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IFeed * This,
            /* [in] */ LONG itemId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title);
        
        DECLSPEC_XFGVIRT(IFeed, get_Description)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFeed, get_Link)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Link )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *homePage);
        
        DECLSPEC_XFGVIRT(IFeed, get_Image)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Image )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *imageUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastBuildDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastBuildDate )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out DATE *lastBuildDate);
        
        DECLSPEC_XFGVIRT(IFeed, get_PubDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PubDate )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out DATE *lastPopulateDate);
        
        DECLSPEC_XFGVIRT(IFeed, get_Ttl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Ttl )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out LONG *ttl);
        
        DECLSPEC_XFGVIRT(IFeed, get_Language)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Language )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *language);
        
        DECLSPEC_XFGVIRT(IFeed, get_Copyright)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Copyright )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *copyright);
        
        DECLSPEC_XFGVIRT(IFeed, get_MaxItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxItemCount )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeed, put_MaxItemCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxItemCount )( 
            __RPC__in IFeed * This,
            /* [in] */ LONG count);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadEnclosuresAutomatically)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadEnclosuresAutomatically )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *downloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IFeed, put_DownloadEnclosuresAutomatically)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DownloadEnclosuresAutomatically )( 
            __RPC__in IFeed * This,
            /* [in] */ VARIANT_BOOL downloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadStatus)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadStatus )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *status);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastDownloadError)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadError )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *error);
        
        DECLSPEC_XFGVIRT(IFeed, Merge)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Merge )( 
            __RPC__in IFeed * This,
            /* [in] */ __RPC__in BSTR feedXml,
            /* [in] */ __RPC__in BSTR feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadUrl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadUrl )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_IsList)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsList )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isList);
        
        DECLSPEC_XFGVIRT(IFeed, MarkAllItemsRead)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MarkAllItemsRead )( 
            __RPC__in IFeed * This);
        
        DECLSPEC_XFGVIRT(IFeed, GetWatcher)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IFeed * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_UnreadItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UnreadItemCount )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeed, get_ItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ItemCount )( 
            __RPC__in IFeed * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        END_INTERFACE
    } IFeedVtbl;

    interface IFeed
    {
        CONST_VTBL struct IFeedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeed_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeed_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeed_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeed_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeed_Xml(This,count,sortProperty,sortOrder,filterFlags,includeFlags,xml)	\
    ( (This)->lpVtbl -> Xml(This,count,sortProperty,sortOrder,filterFlags,includeFlags,xml) ) 

#define IFeed_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFeed_Rename(This,name)	\
    ( (This)->lpVtbl -> Rename(This,name) ) 

#define IFeed_get_Url(This,feedUrl)	\
    ( (This)->lpVtbl -> get_Url(This,feedUrl) ) 

#define IFeed_put_Url(This,feedUrl)	\
    ( (This)->lpVtbl -> put_Url(This,feedUrl) ) 

#define IFeed_get_LocalId(This,feedGuid)	\
    ( (This)->lpVtbl -> get_LocalId(This,feedGuid) ) 

#define IFeed_get_Path(This,path)	\
    ( (This)->lpVtbl -> get_Path(This,path) ) 

#define IFeed_Move(This,newParentPath)	\
    ( (This)->lpVtbl -> Move(This,newParentPath) ) 

#define IFeed_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeed_get_LastWriteTime(This,lastWrite)	\
    ( (This)->lpVtbl -> get_LastWriteTime(This,lastWrite) ) 

#define IFeed_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFeed_Download(This)	\
    ( (This)->lpVtbl -> Download(This) ) 

#define IFeed_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IFeed_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IFeed_get_SyncSetting(This,syncSetting)	\
    ( (This)->lpVtbl -> get_SyncSetting(This,syncSetting) ) 

#define IFeed_put_SyncSetting(This,syncSetting)	\
    ( (This)->lpVtbl -> put_SyncSetting(This,syncSetting) ) 

#define IFeed_get_Interval(This,minutes)	\
    ( (This)->lpVtbl -> get_Interval(This,minutes) ) 

#define IFeed_put_Interval(This,minutes)	\
    ( (This)->lpVtbl -> put_Interval(This,minutes) ) 

#define IFeed_get_LastDownloadTime(This,lastDownload)	\
    ( (This)->lpVtbl -> get_LastDownloadTime(This,lastDownload) ) 

#define IFeed_get_LocalEnclosurePath(This,path)	\
    ( (This)->lpVtbl -> get_LocalEnclosurePath(This,path) ) 

#define IFeed_get_Items(This,disp)	\
    ( (This)->lpVtbl -> get_Items(This,disp) ) 

#define IFeed_GetItem(This,itemId,disp)	\
    ( (This)->lpVtbl -> GetItem(This,itemId,disp) ) 

#define IFeed_get_Title(This,title)	\
    ( (This)->lpVtbl -> get_Title(This,title) ) 

#define IFeed_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFeed_get_Link(This,homePage)	\
    ( (This)->lpVtbl -> get_Link(This,homePage) ) 

#define IFeed_get_Image(This,imageUrl)	\
    ( (This)->lpVtbl -> get_Image(This,imageUrl) ) 

#define IFeed_get_LastBuildDate(This,lastBuildDate)	\
    ( (This)->lpVtbl -> get_LastBuildDate(This,lastBuildDate) ) 

#define IFeed_get_PubDate(This,lastPopulateDate)	\
    ( (This)->lpVtbl -> get_PubDate(This,lastPopulateDate) ) 

#define IFeed_get_Ttl(This,ttl)	\
    ( (This)->lpVtbl -> get_Ttl(This,ttl) ) 

#define IFeed_get_Language(This,language)	\
    ( (This)->lpVtbl -> get_Language(This,language) ) 

#define IFeed_get_Copyright(This,copyright)	\
    ( (This)->lpVtbl -> get_Copyright(This,copyright) ) 

#define IFeed_get_MaxItemCount(This,count)	\
    ( (This)->lpVtbl -> get_MaxItemCount(This,count) ) 

#define IFeed_put_MaxItemCount(This,count)	\
    ( (This)->lpVtbl -> put_MaxItemCount(This,count) ) 

#define IFeed_get_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> get_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically) ) 

#define IFeed_put_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> put_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically) ) 

#define IFeed_get_DownloadStatus(This,status)	\
    ( (This)->lpVtbl -> get_DownloadStatus(This,status) ) 

#define IFeed_get_LastDownloadError(This,error)	\
    ( (This)->lpVtbl -> get_LastDownloadError(This,error) ) 

#define IFeed_Merge(This,feedXml,feedUrl)	\
    ( (This)->lpVtbl -> Merge(This,feedXml,feedUrl) ) 

#define IFeed_get_DownloadUrl(This,feedUrl)	\
    ( (This)->lpVtbl -> get_DownloadUrl(This,feedUrl) ) 

#define IFeed_get_IsList(This,isList)	\
    ( (This)->lpVtbl -> get_IsList(This,isList) ) 

#define IFeed_MarkAllItemsRead(This)	\
    ( (This)->lpVtbl -> MarkAllItemsRead(This) ) 

#define IFeed_GetWatcher(This,scope,mask,disp)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,disp) ) 

#define IFeed_get_UnreadItemCount(This,count)	\
    ( (This)->lpVtbl -> get_UnreadItemCount(This,count) ) 

#define IFeed_get_ItemCount(This,count)	\
    ( (This)->lpVtbl -> get_ItemCount(This,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeed_INTERFACE_DEFINED__ */


#ifndef __IFeed2_INTERFACE_DEFINED__
#define __IFeed2_INTERFACE_DEFINED__

/* interface IFeed2 */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeed2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33f2ea09-1398-4ab9-b6a4-f94b49d0a42e")
    IFeed2 : public IFeed
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetItemByEffectiveId( 
            /* [in] */ LONG itemEffectiveId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastItemDownloadTime( 
            /* [retval][out] */ __RPC__out DATE *lastItemDownloadTime) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Username( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *username) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Password( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *password) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetCredentials( 
            /* [in] */ __RPC__in BSTR username,
            /* [in] */ __RPC__in BSTR password) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearCredentials( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeed2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeed2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeed2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeed2 * This,
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
        
        DECLSPEC_XFGVIRT(IFeed, Xml)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IFeed2 * This,
            /* [in] */ LONG count,
            /* [in] */ FEEDS_XML_SORT_PROPERTY sortProperty,
            /* [in] */ FEEDS_XML_SORT_ORDER sortOrder,
            /* [in] */ FEEDS_XML_FILTER_FLAGS filterFlags,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml);
        
        DECLSPEC_XFGVIRT(IFeed, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFeed, Rename)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFeed, get_Url)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Url )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, put_Url)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Url )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in BSTR feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_LocalId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalId )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedGuid);
        
        DECLSPEC_XFGVIRT(IFeed, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFeed, Move)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in BSTR newParentPath);
        
        DECLSPEC_XFGVIRT(IFeed, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastWriteTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastWriteTime )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out DATE *lastWrite);
        
        DECLSPEC_XFGVIRT(IFeed, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IFeed, Download)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Download )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IFeed, AsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IFeed, CancelAsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IFeed, get_SyncSetting)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SyncSetting )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out FEEDS_SYNC_SETTING *syncSetting);
        
        DECLSPEC_XFGVIRT(IFeed, put_SyncSetting)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SyncSetting )( 
            __RPC__in IFeed2 * This,
            /* [in] */ FEEDS_SYNC_SETTING syncSetting);
        
        DECLSPEC_XFGVIRT(IFeed, get_Interval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Interval )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out LONG *minutes);
        
        DECLSPEC_XFGVIRT(IFeed, put_Interval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Interval )( 
            __RPC__in IFeed2 * This,
            /* [in] */ LONG minutes);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastDownloadTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadTime )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out DATE *lastDownload);
        
        DECLSPEC_XFGVIRT(IFeed, get_LocalEnclosurePath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalEnclosurePath )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFeed, get_Items)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Items )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, GetItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IFeed2 * This,
            /* [in] */ LONG itemId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title);
        
        DECLSPEC_XFGVIRT(IFeed, get_Description)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFeed, get_Link)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Link )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *homePage);
        
        DECLSPEC_XFGVIRT(IFeed, get_Image)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Image )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *imageUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastBuildDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastBuildDate )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out DATE *lastBuildDate);
        
        DECLSPEC_XFGVIRT(IFeed, get_PubDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PubDate )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out DATE *lastPopulateDate);
        
        DECLSPEC_XFGVIRT(IFeed, get_Ttl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Ttl )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out LONG *ttl);
        
        DECLSPEC_XFGVIRT(IFeed, get_Language)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Language )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *language);
        
        DECLSPEC_XFGVIRT(IFeed, get_Copyright)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Copyright )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *copyright);
        
        DECLSPEC_XFGVIRT(IFeed, get_MaxItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxItemCount )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeed, put_MaxItemCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxItemCount )( 
            __RPC__in IFeed2 * This,
            /* [in] */ LONG count);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadEnclosuresAutomatically)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadEnclosuresAutomatically )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *downloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IFeed, put_DownloadEnclosuresAutomatically)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DownloadEnclosuresAutomatically )( 
            __RPC__in IFeed2 * This,
            /* [in] */ VARIANT_BOOL downloadEnclosuresAutomatically);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadStatus)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadStatus )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *status);
        
        DECLSPEC_XFGVIRT(IFeed, get_LastDownloadError)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadError )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *error);
        
        DECLSPEC_XFGVIRT(IFeed, Merge)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Merge )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in BSTR feedXml,
            /* [in] */ __RPC__in BSTR feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_DownloadUrl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadUrl )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *feedUrl);
        
        DECLSPEC_XFGVIRT(IFeed, get_IsList)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsList )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isList);
        
        DECLSPEC_XFGVIRT(IFeed, MarkAllItemsRead)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MarkAllItemsRead )( 
            __RPC__in IFeed2 * This);
        
        DECLSPEC_XFGVIRT(IFeed, GetWatcher)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWatcher )( 
            __RPC__in IFeed2 * This,
            /* [in] */ FEEDS_EVENTS_SCOPE scope,
            /* [in] */ FEEDS_EVENTS_MASK mask,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed, get_UnreadItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UnreadItemCount )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeed, get_ItemCount)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ItemCount )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFeed2, GetItemByEffectiveId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetItemByEffectiveId )( 
            __RPC__in IFeed2 * This,
            /* [in] */ LONG itemEffectiveId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeed2, get_LastItemDownloadTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastItemDownloadTime )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__out DATE *lastItemDownloadTime);
        
        DECLSPEC_XFGVIRT(IFeed2, get_Username)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Username )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *username);
        
        DECLSPEC_XFGVIRT(IFeed2, get_Password)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Password )( 
            __RPC__in IFeed2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *password);
        
        DECLSPEC_XFGVIRT(IFeed2, SetCredentials)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IFeed2 * This,
            /* [in] */ __RPC__in BSTR username,
            /* [in] */ __RPC__in BSTR password);
        
        DECLSPEC_XFGVIRT(IFeed2, ClearCredentials)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearCredentials )( 
            __RPC__in IFeed2 * This);
        
        END_INTERFACE
    } IFeed2Vtbl;

    interface IFeed2
    {
        CONST_VTBL struct IFeed2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeed2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeed2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeed2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeed2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeed2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeed2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeed2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeed2_Xml(This,count,sortProperty,sortOrder,filterFlags,includeFlags,xml)	\
    ( (This)->lpVtbl -> Xml(This,count,sortProperty,sortOrder,filterFlags,includeFlags,xml) ) 

#define IFeed2_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFeed2_Rename(This,name)	\
    ( (This)->lpVtbl -> Rename(This,name) ) 

#define IFeed2_get_Url(This,feedUrl)	\
    ( (This)->lpVtbl -> get_Url(This,feedUrl) ) 

#define IFeed2_put_Url(This,feedUrl)	\
    ( (This)->lpVtbl -> put_Url(This,feedUrl) ) 

#define IFeed2_get_LocalId(This,feedGuid)	\
    ( (This)->lpVtbl -> get_LocalId(This,feedGuid) ) 

#define IFeed2_get_Path(This,path)	\
    ( (This)->lpVtbl -> get_Path(This,path) ) 

#define IFeed2_Move(This,newParentPath)	\
    ( (This)->lpVtbl -> Move(This,newParentPath) ) 

#define IFeed2_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeed2_get_LastWriteTime(This,lastWrite)	\
    ( (This)->lpVtbl -> get_LastWriteTime(This,lastWrite) ) 

#define IFeed2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFeed2_Download(This)	\
    ( (This)->lpVtbl -> Download(This) ) 

#define IFeed2_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IFeed2_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IFeed2_get_SyncSetting(This,syncSetting)	\
    ( (This)->lpVtbl -> get_SyncSetting(This,syncSetting) ) 

#define IFeed2_put_SyncSetting(This,syncSetting)	\
    ( (This)->lpVtbl -> put_SyncSetting(This,syncSetting) ) 

#define IFeed2_get_Interval(This,minutes)	\
    ( (This)->lpVtbl -> get_Interval(This,minutes) ) 

#define IFeed2_put_Interval(This,minutes)	\
    ( (This)->lpVtbl -> put_Interval(This,minutes) ) 

#define IFeed2_get_LastDownloadTime(This,lastDownload)	\
    ( (This)->lpVtbl -> get_LastDownloadTime(This,lastDownload) ) 

#define IFeed2_get_LocalEnclosurePath(This,path)	\
    ( (This)->lpVtbl -> get_LocalEnclosurePath(This,path) ) 

#define IFeed2_get_Items(This,disp)	\
    ( (This)->lpVtbl -> get_Items(This,disp) ) 

#define IFeed2_GetItem(This,itemId,disp)	\
    ( (This)->lpVtbl -> GetItem(This,itemId,disp) ) 

#define IFeed2_get_Title(This,title)	\
    ( (This)->lpVtbl -> get_Title(This,title) ) 

#define IFeed2_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFeed2_get_Link(This,homePage)	\
    ( (This)->lpVtbl -> get_Link(This,homePage) ) 

#define IFeed2_get_Image(This,imageUrl)	\
    ( (This)->lpVtbl -> get_Image(This,imageUrl) ) 

#define IFeed2_get_LastBuildDate(This,lastBuildDate)	\
    ( (This)->lpVtbl -> get_LastBuildDate(This,lastBuildDate) ) 

#define IFeed2_get_PubDate(This,lastPopulateDate)	\
    ( (This)->lpVtbl -> get_PubDate(This,lastPopulateDate) ) 

#define IFeed2_get_Ttl(This,ttl)	\
    ( (This)->lpVtbl -> get_Ttl(This,ttl) ) 

#define IFeed2_get_Language(This,language)	\
    ( (This)->lpVtbl -> get_Language(This,language) ) 

#define IFeed2_get_Copyright(This,copyright)	\
    ( (This)->lpVtbl -> get_Copyright(This,copyright) ) 

#define IFeed2_get_MaxItemCount(This,count)	\
    ( (This)->lpVtbl -> get_MaxItemCount(This,count) ) 

#define IFeed2_put_MaxItemCount(This,count)	\
    ( (This)->lpVtbl -> put_MaxItemCount(This,count) ) 

#define IFeed2_get_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> get_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically) ) 

#define IFeed2_put_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically)	\
    ( (This)->lpVtbl -> put_DownloadEnclosuresAutomatically(This,downloadEnclosuresAutomatically) ) 

#define IFeed2_get_DownloadStatus(This,status)	\
    ( (This)->lpVtbl -> get_DownloadStatus(This,status) ) 

#define IFeed2_get_LastDownloadError(This,error)	\
    ( (This)->lpVtbl -> get_LastDownloadError(This,error) ) 

#define IFeed2_Merge(This,feedXml,feedUrl)	\
    ( (This)->lpVtbl -> Merge(This,feedXml,feedUrl) ) 

#define IFeed2_get_DownloadUrl(This,feedUrl)	\
    ( (This)->lpVtbl -> get_DownloadUrl(This,feedUrl) ) 

#define IFeed2_get_IsList(This,isList)	\
    ( (This)->lpVtbl -> get_IsList(This,isList) ) 

#define IFeed2_MarkAllItemsRead(This)	\
    ( (This)->lpVtbl -> MarkAllItemsRead(This) ) 

#define IFeed2_GetWatcher(This,scope,mask,disp)	\
    ( (This)->lpVtbl -> GetWatcher(This,scope,mask,disp) ) 

#define IFeed2_get_UnreadItemCount(This,count)	\
    ( (This)->lpVtbl -> get_UnreadItemCount(This,count) ) 

#define IFeed2_get_ItemCount(This,count)	\
    ( (This)->lpVtbl -> get_ItemCount(This,count) ) 


#define IFeed2_GetItemByEffectiveId(This,itemEffectiveId,disp)	\
    ( (This)->lpVtbl -> GetItemByEffectiveId(This,itemEffectiveId,disp) ) 

#define IFeed2_get_LastItemDownloadTime(This,lastItemDownloadTime)	\
    ( (This)->lpVtbl -> get_LastItemDownloadTime(This,lastItemDownloadTime) ) 

#define IFeed2_get_Username(This,username)	\
    ( (This)->lpVtbl -> get_Username(This,username) ) 

#define IFeed2_get_Password(This,password)	\
    ( (This)->lpVtbl -> get_Password(This,password) ) 

#define IFeed2_SetCredentials(This,username,password)	\
    ( (This)->lpVtbl -> SetCredentials(This,username,password) ) 

#define IFeed2_ClearCredentials(This)	\
    ( (This)->lpVtbl -> ClearCredentials(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeed2_INTERFACE_DEFINED__ */


#ifndef __IFeedEvents_INTERFACE_DEFINED__
#define __IFeedEvents_INTERFACE_DEFINED__

/* interface IFeedEvents */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("abf35c99-0681-47ea-9a8c-1436a375a99e")
    IFeedEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Error( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDeleted( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedRenamed( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedUrlChanged( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedMoved( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDownloading( 
            /* [in] */ __RPC__in const BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedDownloadCompleted( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ FEEDS_DOWNLOAD_ERROR error) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FeedItemCountChanged( 
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedEvents * This,
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
        
        DECLSPEC_XFGVIRT(IFeedEvents, Error)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IFeedEvents * This);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedDeleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDeleted )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedRenamed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedRenamed )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedUrlChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedUrlChanged )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedMoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedMoved )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ __RPC__in const BSTR oldPath);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedDownloading)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDownloading )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedDownloadCompleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedDownloadCompleted )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ FEEDS_DOWNLOAD_ERROR error);
        
        DECLSPEC_XFGVIRT(IFeedEvents, FeedItemCountChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FeedItemCountChanged )( 
            __RPC__in IFeedEvents * This,
            /* [in] */ __RPC__in const BSTR path,
            /* [in] */ LONG itemCountType);
        
        END_INTERFACE
    } IFeedEventsVtbl;

    interface IFeedEvents
    {
        CONST_VTBL struct IFeedEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedEvents_Error(This)	\
    ( (This)->lpVtbl -> Error(This) ) 

#define IFeedEvents_FeedDeleted(This,path)	\
    ( (This)->lpVtbl -> FeedDeleted(This,path) ) 

#define IFeedEvents_FeedRenamed(This,path,oldPath)	\
    ( (This)->lpVtbl -> FeedRenamed(This,path,oldPath) ) 

#define IFeedEvents_FeedUrlChanged(This,path)	\
    ( (This)->lpVtbl -> FeedUrlChanged(This,path) ) 

#define IFeedEvents_FeedMoved(This,path,oldPath)	\
    ( (This)->lpVtbl -> FeedMoved(This,path,oldPath) ) 

#define IFeedEvents_FeedDownloading(This,path)	\
    ( (This)->lpVtbl -> FeedDownloading(This,path) ) 

#define IFeedEvents_FeedDownloadCompleted(This,path,error)	\
    ( (This)->lpVtbl -> FeedDownloadCompleted(This,path,error) ) 

#define IFeedEvents_FeedItemCountChanged(This,path,itemCountType)	\
    ( (This)->lpVtbl -> FeedItemCountChanged(This,path,itemCountType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedEvents_INTERFACE_DEFINED__ */


#ifndef __IFeedItem_INTERFACE_DEFINED__
#define __IFeedItem_INTERFACE_DEFINED__

/* interface IFeedItem */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a1e6cad-0a47-4da2-a13d-5baaa5c8bd4f")
    IFeedItem : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Xml( 
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Link( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *linkUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Guid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemGuid) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_PubDate( 
            /* [retval][out] */ __RPC__out DATE *pubDate) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Comments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *comments) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Author( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *author) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Enclosure( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsRead( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isRead) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_IsRead( 
            /* [in] */ VARIANT_BOOL isRead) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocalId( 
            /* [retval][out] */ __RPC__out LONG *itemId) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadUrl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastDownloadTime( 
            /* [retval][out] */ __RPC__out DATE *lastDownload) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out DATE *modified) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedItem * This,
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
        
        DECLSPEC_XFGVIRT(IFeedItem, Xml)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IFeedItem * This,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Link)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Link )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *linkUrl);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Guid)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemGuid);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Description)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_PubDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PubDate )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__out DATE *pubDate);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Comments)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Comments )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *comments);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Author)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *author);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Enclosure)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enclosure )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_IsRead)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsRead )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isRead);
        
        DECLSPEC_XFGVIRT(IFeedItem, put_IsRead)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsRead )( 
            __RPC__in IFeedItem * This,
            /* [in] */ VARIANT_BOOL isRead);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_LocalId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalId )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__out LONG *itemId);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedItem, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFeedItem * This);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_DownloadUrl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadUrl )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemUrl);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_LastDownloadTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadTime )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__out DATE *lastDownload);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Modified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in IFeedItem * This,
            /* [retval][out] */ __RPC__out DATE *modified);
        
        END_INTERFACE
    } IFeedItemVtbl;

    interface IFeedItem
    {
        CONST_VTBL struct IFeedItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedItem_Xml(This,includeFlags,xml)	\
    ( (This)->lpVtbl -> Xml(This,includeFlags,xml) ) 

#define IFeedItem_get_Title(This,title)	\
    ( (This)->lpVtbl -> get_Title(This,title) ) 

#define IFeedItem_get_Link(This,linkUrl)	\
    ( (This)->lpVtbl -> get_Link(This,linkUrl) ) 

#define IFeedItem_get_Guid(This,itemGuid)	\
    ( (This)->lpVtbl -> get_Guid(This,itemGuid) ) 

#define IFeedItem_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFeedItem_get_PubDate(This,pubDate)	\
    ( (This)->lpVtbl -> get_PubDate(This,pubDate) ) 

#define IFeedItem_get_Comments(This,comments)	\
    ( (This)->lpVtbl -> get_Comments(This,comments) ) 

#define IFeedItem_get_Author(This,author)	\
    ( (This)->lpVtbl -> get_Author(This,author) ) 

#define IFeedItem_get_Enclosure(This,disp)	\
    ( (This)->lpVtbl -> get_Enclosure(This,disp) ) 

#define IFeedItem_get_IsRead(This,isRead)	\
    ( (This)->lpVtbl -> get_IsRead(This,isRead) ) 

#define IFeedItem_put_IsRead(This,isRead)	\
    ( (This)->lpVtbl -> put_IsRead(This,isRead) ) 

#define IFeedItem_get_LocalId(This,itemId)	\
    ( (This)->lpVtbl -> get_LocalId(This,itemId) ) 

#define IFeedItem_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeedItem_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFeedItem_get_DownloadUrl(This,itemUrl)	\
    ( (This)->lpVtbl -> get_DownloadUrl(This,itemUrl) ) 

#define IFeedItem_get_LastDownloadTime(This,lastDownload)	\
    ( (This)->lpVtbl -> get_LastDownloadTime(This,lastDownload) ) 

#define IFeedItem_get_Modified(This,modified)	\
    ( (This)->lpVtbl -> get_Modified(This,modified) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedItem_INTERFACE_DEFINED__ */


#ifndef __IFeedItem2_INTERFACE_DEFINED__
#define __IFeedItem2_INTERFACE_DEFINED__

/* interface IFeedItem2 */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79ac9ef4-f9c1-4d2b-a50b-a7ffba4dcf37")
    IFeedItem2 : public IFeedItem
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_EffectiveId( 
            /* [retval][out] */ __RPC__out LONG *effectiveId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedItem2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedItem2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedItem2 * This,
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
        
        DECLSPEC_XFGVIRT(IFeedItem, Xml)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Xml )( 
            __RPC__in IFeedItem2 * This,
            /* [in] */ FEEDS_XML_INCLUDE_FLAGS includeFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *title);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Link)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Link )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *linkUrl);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Guid)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemGuid);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Description)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_PubDate)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PubDate )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pubDate);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Comments)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Comments )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *comments);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Author)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *author);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Enclosure)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enclosure )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_IsRead)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsRead )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isRead);
        
        DECLSPEC_XFGVIRT(IFeedItem, put_IsRead)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsRead )( 
            __RPC__in IFeedItem2 * This,
            /* [in] */ VARIANT_BOOL isRead);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_LocalId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalId )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out LONG *itemId);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedItem, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFeedItem2 * This);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_DownloadUrl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadUrl )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *itemUrl);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_LastDownloadTime)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadTime )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out DATE *lastDownload);
        
        DECLSPEC_XFGVIRT(IFeedItem, get_Modified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out DATE *modified);
        
        DECLSPEC_XFGVIRT(IFeedItem2, get_EffectiveId)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EffectiveId )( 
            __RPC__in IFeedItem2 * This,
            /* [retval][out] */ __RPC__out LONG *effectiveId);
        
        END_INTERFACE
    } IFeedItem2Vtbl;

    interface IFeedItem2
    {
        CONST_VTBL struct IFeedItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedItem2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedItem2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedItem2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedItem2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedItem2_Xml(This,includeFlags,xml)	\
    ( (This)->lpVtbl -> Xml(This,includeFlags,xml) ) 

#define IFeedItem2_get_Title(This,title)	\
    ( (This)->lpVtbl -> get_Title(This,title) ) 

#define IFeedItem2_get_Link(This,linkUrl)	\
    ( (This)->lpVtbl -> get_Link(This,linkUrl) ) 

#define IFeedItem2_get_Guid(This,itemGuid)	\
    ( (This)->lpVtbl -> get_Guid(This,itemGuid) ) 

#define IFeedItem2_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFeedItem2_get_PubDate(This,pubDate)	\
    ( (This)->lpVtbl -> get_PubDate(This,pubDate) ) 

#define IFeedItem2_get_Comments(This,comments)	\
    ( (This)->lpVtbl -> get_Comments(This,comments) ) 

#define IFeedItem2_get_Author(This,author)	\
    ( (This)->lpVtbl -> get_Author(This,author) ) 

#define IFeedItem2_get_Enclosure(This,disp)	\
    ( (This)->lpVtbl -> get_Enclosure(This,disp) ) 

#define IFeedItem2_get_IsRead(This,isRead)	\
    ( (This)->lpVtbl -> get_IsRead(This,isRead) ) 

#define IFeedItem2_put_IsRead(This,isRead)	\
    ( (This)->lpVtbl -> put_IsRead(This,isRead) ) 

#define IFeedItem2_get_LocalId(This,itemId)	\
    ( (This)->lpVtbl -> get_LocalId(This,itemId) ) 

#define IFeedItem2_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeedItem2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFeedItem2_get_DownloadUrl(This,itemUrl)	\
    ( (This)->lpVtbl -> get_DownloadUrl(This,itemUrl) ) 

#define IFeedItem2_get_LastDownloadTime(This,lastDownload)	\
    ( (This)->lpVtbl -> get_LastDownloadTime(This,lastDownload) ) 

#define IFeedItem2_get_Modified(This,modified)	\
    ( (This)->lpVtbl -> get_Modified(This,modified) ) 


#define IFeedItem2_get_EffectiveId(This,effectiveId)	\
    ( (This)->lpVtbl -> get_EffectiveId(This,effectiveId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedItem2_INTERFACE_DEFINED__ */


#ifndef __IFeedEnclosure_INTERFACE_DEFINED__
#define __IFeedEnclosure_INTERFACE_DEFINED__

/* interface IFeedEnclosure */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IFeedEnclosure;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("361C26F7-90A4-4e67-AE09-3A36A546436A")
    IFeedEnclosure : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Url( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *enclosureUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mimeType) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Length( 
            /* [retval][out] */ __RPC__out LONG *length) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AsyncDownload( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelAsyncDownload( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadStatus( 
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *status) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LastDownloadError( 
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *error) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocalPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *localPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadUrl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *enclosureUrl) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DownloadMimeType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mimeType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveFile( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetFile( 
            /* [in] */ __RPC__in BSTR downloadUrl,
            /* [in] */ __RPC__in BSTR downloadFilePath,
            /* [in] */ __RPC__in BSTR downloadMimeType,
            /* [in] */ __RPC__in BSTR enclosureFilename) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFeedEnclosureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFeedEnclosure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFeedEnclosure * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFeedEnclosure * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFeedEnclosure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFeedEnclosure * This,
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
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_Url)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Url )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *enclosureUrl);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_Type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mimeType);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_Length)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Length )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__out LONG *length);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, AsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AsyncDownload )( 
            __RPC__in IFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, CancelAsyncDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelAsyncDownload )( 
            __RPC__in IFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_DownloadStatus)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadStatus )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_STATUS *status);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_LastDownloadError)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LastDownloadError )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__out FEEDS_DOWNLOAD_ERROR *error);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_LocalPath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocalPath )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *localPath);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_Parent)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **disp);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_DownloadUrl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadUrl )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *enclosureUrl);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, get_DownloadMimeType)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DownloadMimeType )( 
            __RPC__in IFeedEnclosure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mimeType);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, RemoveFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveFile )( 
            __RPC__in IFeedEnclosure * This);
        
        DECLSPEC_XFGVIRT(IFeedEnclosure, SetFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetFile )( 
            __RPC__in IFeedEnclosure * This,
            /* [in] */ __RPC__in BSTR downloadUrl,
            /* [in] */ __RPC__in BSTR downloadFilePath,
            /* [in] */ __RPC__in BSTR downloadMimeType,
            /* [in] */ __RPC__in BSTR enclosureFilename);
        
        END_INTERFACE
    } IFeedEnclosureVtbl;

    interface IFeedEnclosure
    {
        CONST_VTBL struct IFeedEnclosureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFeedEnclosure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFeedEnclosure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFeedEnclosure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFeedEnclosure_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFeedEnclosure_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFeedEnclosure_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFeedEnclosure_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFeedEnclosure_get_Url(This,enclosureUrl)	\
    ( (This)->lpVtbl -> get_Url(This,enclosureUrl) ) 

#define IFeedEnclosure_get_Type(This,mimeType)	\
    ( (This)->lpVtbl -> get_Type(This,mimeType) ) 

#define IFeedEnclosure_get_Length(This,length)	\
    ( (This)->lpVtbl -> get_Length(This,length) ) 

#define IFeedEnclosure_AsyncDownload(This)	\
    ( (This)->lpVtbl -> AsyncDownload(This) ) 

#define IFeedEnclosure_CancelAsyncDownload(This)	\
    ( (This)->lpVtbl -> CancelAsyncDownload(This) ) 

#define IFeedEnclosure_get_DownloadStatus(This,status)	\
    ( (This)->lpVtbl -> get_DownloadStatus(This,status) ) 

#define IFeedEnclosure_get_LastDownloadError(This,error)	\
    ( (This)->lpVtbl -> get_LastDownloadError(This,error) ) 

#define IFeedEnclosure_get_LocalPath(This,localPath)	\
    ( (This)->lpVtbl -> get_LocalPath(This,localPath) ) 

#define IFeedEnclosure_get_Parent(This,disp)	\
    ( (This)->lpVtbl -> get_Parent(This,disp) ) 

#define IFeedEnclosure_get_DownloadUrl(This,enclosureUrl)	\
    ( (This)->lpVtbl -> get_DownloadUrl(This,enclosureUrl) ) 

#define IFeedEnclosure_get_DownloadMimeType(This,mimeType)	\
    ( (This)->lpVtbl -> get_DownloadMimeType(This,mimeType) ) 

#define IFeedEnclosure_RemoveFile(This)	\
    ( (This)->lpVtbl -> RemoveFile(This) ) 

#define IFeedEnclosure_SetFile(This,downloadUrl,downloadFilePath,downloadMimeType,enclosureFilename)	\
    ( (This)->lpVtbl -> SetFile(This,downloadUrl,downloadFilePath,downloadMimeType,enclosureFilename) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFeedEnclosure_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_FeedsManager;

#ifdef __cplusplus

class DECLSPEC_UUID("faeb54c4-f66f-4806-83a0-805299f5e3ad")
FeedsManager;
#endif

EXTERN_C const CLSID CLSID_FeedFolderWatcher;

#ifdef __cplusplus

class DECLSPEC_UUID("281001ed-7765-4cb0-84af-e9b387af01ff")
FeedFolderWatcher;
#endif

EXTERN_C const CLSID CLSID_FeedWatcher;

#ifdef __cplusplus

class DECLSPEC_UUID("18a6737b-f433-4687-89bc-a1b4dfb9f123")
FeedWatcher;
#endif
#endif /* __Feeds_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msfeeds_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msfeeds_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msfeeds_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


