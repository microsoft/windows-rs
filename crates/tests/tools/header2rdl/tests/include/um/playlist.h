

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

#ifndef __playlist_h__
#define __playlist_h__

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

#ifndef __IAMPlayListItem_FWD_DEFINED__
#define __IAMPlayListItem_FWD_DEFINED__
typedef interface IAMPlayListItem IAMPlayListItem;

#endif 	/* __IAMPlayListItem_FWD_DEFINED__ */


#ifndef __IAMPlayList_FWD_DEFINED__
#define __IAMPlayList_FWD_DEFINED__
typedef interface IAMPlayList IAMPlayList;

#endif 	/* __IAMPlayList_FWD_DEFINED__ */


#ifndef __ISpecifyParticularPages_FWD_DEFINED__
#define __ISpecifyParticularPages_FWD_DEFINED__
typedef interface ISpecifyParticularPages ISpecifyParticularPages;

#endif 	/* __ISpecifyParticularPages_FWD_DEFINED__ */


#ifndef __IAMRebuild_FWD_DEFINED__
#define __IAMRebuild_FWD_DEFINED__
typedef interface IAMRebuild IAMRebuild;

#endif 	/* __IAMRebuild_FWD_DEFINED__ */


#ifndef __IBufferingTime_FWD_DEFINED__
#define __IBufferingTime_FWD_DEFINED__
typedef interface IBufferingTime IBufferingTime;

#endif 	/* __IBufferingTime_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_playlist_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

enum AMPlayListItemFlags
    {
        AMPLAYLISTITEM_CANSKIP	= 0x1,
        AMPLAYLISTITEM_CANBIND	= 0x2
    } ;


extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0000_v0_0_s_ifspec;

#ifndef __IAMPlayListItem_INTERFACE_DEFINED__
#define __IAMPlayListItem_INTERFACE_DEFINED__

/* interface IAMPlayListItem */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAMPlayListItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868ff-0ad4-11ce-b03a-0020af0ba770")
    IAMPlayListItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceCount( 
            /* [out] */ __RPC__out DWORD *pdwSources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceURL( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceStart( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceDuration( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceStartMarker( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out DWORD *pdwMarker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceEndMarker( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out DWORD *pdwMarker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceStartMarkerName( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrStartMarker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceEndMarkerName( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrEndMarker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLinkURL( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScanDuration( 
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtScanDuration) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMPlayListItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMPlayListItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMPlayListItem * This);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in IAMPlayListItem * This,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceCount)
        HRESULT ( STDMETHODCALLTYPE *GetSourceCount )( 
            __RPC__in IAMPlayListItem * This,
            /* [out] */ __RPC__out DWORD *pdwSources);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceURL)
        HRESULT ( STDMETHODCALLTYPE *GetSourceURL )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceStart)
        HRESULT ( STDMETHODCALLTYPE *GetSourceStart )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtStart);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceDuration)
        HRESULT ( STDMETHODCALLTYPE *GetSourceDuration )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtDuration);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceStartMarker)
        HRESULT ( STDMETHODCALLTYPE *GetSourceStartMarker )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out DWORD *pdwMarker);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceEndMarker)
        HRESULT ( STDMETHODCALLTYPE *GetSourceEndMarker )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out DWORD *pdwMarker);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceStartMarkerName)
        HRESULT ( STDMETHODCALLTYPE *GetSourceStartMarkerName )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrStartMarker);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetSourceEndMarkerName)
        HRESULT ( STDMETHODCALLTYPE *GetSourceEndMarkerName )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrEndMarker);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetLinkURL)
        HRESULT ( STDMETHODCALLTYPE *GetLinkURL )( 
            __RPC__in IAMPlayListItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IAMPlayListItem, GetScanDuration)
        HRESULT ( STDMETHODCALLTYPE *GetScanDuration )( 
            __RPC__in IAMPlayListItem * This,
            /* [in] */ DWORD dwSourceIndex,
            /* [out] */ __RPC__out REFERENCE_TIME *prtScanDuration);
        
        END_INTERFACE
    } IAMPlayListItemVtbl;

    interface IAMPlayListItem
    {
        CONST_VTBL struct IAMPlayListItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMPlayListItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMPlayListItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMPlayListItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMPlayListItem_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IAMPlayListItem_GetSourceCount(This,pdwSources)	\
    ( (This)->lpVtbl -> GetSourceCount(This,pdwSources) ) 

#define IAMPlayListItem_GetSourceURL(This,dwSourceIndex,pbstrURL)	\
    ( (This)->lpVtbl -> GetSourceURL(This,dwSourceIndex,pbstrURL) ) 

#define IAMPlayListItem_GetSourceStart(This,dwSourceIndex,prtStart)	\
    ( (This)->lpVtbl -> GetSourceStart(This,dwSourceIndex,prtStart) ) 

#define IAMPlayListItem_GetSourceDuration(This,dwSourceIndex,prtDuration)	\
    ( (This)->lpVtbl -> GetSourceDuration(This,dwSourceIndex,prtDuration) ) 

#define IAMPlayListItem_GetSourceStartMarker(This,dwSourceIndex,pdwMarker)	\
    ( (This)->lpVtbl -> GetSourceStartMarker(This,dwSourceIndex,pdwMarker) ) 

#define IAMPlayListItem_GetSourceEndMarker(This,dwSourceIndex,pdwMarker)	\
    ( (This)->lpVtbl -> GetSourceEndMarker(This,dwSourceIndex,pdwMarker) ) 

#define IAMPlayListItem_GetSourceStartMarkerName(This,dwSourceIndex,pbstrStartMarker)	\
    ( (This)->lpVtbl -> GetSourceStartMarkerName(This,dwSourceIndex,pbstrStartMarker) ) 

#define IAMPlayListItem_GetSourceEndMarkerName(This,dwSourceIndex,pbstrEndMarker)	\
    ( (This)->lpVtbl -> GetSourceEndMarkerName(This,dwSourceIndex,pbstrEndMarker) ) 

#define IAMPlayListItem_GetLinkURL(This,pbstrURL)	\
    ( (This)->lpVtbl -> GetLinkURL(This,pbstrURL) ) 

#define IAMPlayListItem_GetScanDuration(This,dwSourceIndex,prtScanDuration)	\
    ( (This)->lpVtbl -> GetScanDuration(This,dwSourceIndex,prtScanDuration) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMPlayListItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_playlist_0000_0001 */
/* [local] */ 


enum AMPlayListFlags
    {
        AMPLAYLIST_STARTINSCANMODE	= 0x1,
        AMPLAYLIST_FORCEBANNER	= 0x2
    } ;

enum AMPlayListEventFlags
    {
        AMPLAYLISTEVENT_RESUME	= 0,
        AMPLAYLISTEVENT_BREAK	= 0x1,
        AMPLAYLISTEVENT_NEXT	= 0x2,
        AMPLAYLISTEVENT_MASK	= 0xf,
        AMPLAYLISTEVENT_REFRESH	= 0x10
    } ;


extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0001_v0_0_s_ifspec;

#ifndef __IAMPlayList_INTERFACE_DEFINED__
#define __IAMPlayList_INTERFACE_DEFINED__

/* interface IAMPlayList */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAMPlayList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868fe-0ad4-11ce-b03a-0020af0ba770")
    IAMPlayList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemCount( 
            /* [out] */ __RPC__out DWORD *pdwItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItem( 
            /* [in] */ DWORD dwItemIndex,
            /* [out] */ __RPC__deref_out_opt IAMPlayListItem **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamedEvent( 
            /* [in] */ __RPC__in WCHAR *pwszEventName,
            /* [in] */ DWORD dwItemIndex,
            /* [out] */ __RPC__deref_out_opt IAMPlayListItem **ppItem,
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRepeatInfo( 
            /* [out] */ __RPC__out DWORD *pdwRepeatCount,
            /* [out] */ __RPC__out DWORD *pdwRepeatStart,
            /* [out] */ __RPC__out DWORD *pdwRepeatEnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMPlayListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMPlayList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMPlayList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMPlayList * This);
        
        DECLSPEC_XFGVIRT(IAMPlayList, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in IAMPlayList * This,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IAMPlayList, GetItemCount)
        HRESULT ( STDMETHODCALLTYPE *GetItemCount )( 
            __RPC__in IAMPlayList * This,
            /* [out] */ __RPC__out DWORD *pdwItems);
        
        DECLSPEC_XFGVIRT(IAMPlayList, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IAMPlayList * This,
            /* [in] */ DWORD dwItemIndex,
            /* [out] */ __RPC__deref_out_opt IAMPlayListItem **ppItem);
        
        DECLSPEC_XFGVIRT(IAMPlayList, GetNamedEvent)
        HRESULT ( STDMETHODCALLTYPE *GetNamedEvent )( 
            __RPC__in IAMPlayList * This,
            /* [in] */ __RPC__in WCHAR *pwszEventName,
            /* [in] */ DWORD dwItemIndex,
            /* [out] */ __RPC__deref_out_opt IAMPlayListItem **ppItem,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IAMPlayList, GetRepeatInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRepeatInfo )( 
            __RPC__in IAMPlayList * This,
            /* [out] */ __RPC__out DWORD *pdwRepeatCount,
            /* [out] */ __RPC__out DWORD *pdwRepeatStart,
            /* [out] */ __RPC__out DWORD *pdwRepeatEnd);
        
        END_INTERFACE
    } IAMPlayListVtbl;

    interface IAMPlayList
    {
        CONST_VTBL struct IAMPlayListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMPlayList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMPlayList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMPlayList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMPlayList_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IAMPlayList_GetItemCount(This,pdwItems)	\
    ( (This)->lpVtbl -> GetItemCount(This,pdwItems) ) 

#define IAMPlayList_GetItem(This,dwItemIndex,ppItem)	\
    ( (This)->lpVtbl -> GetItem(This,dwItemIndex,ppItem) ) 

#define IAMPlayList_GetNamedEvent(This,pwszEventName,dwItemIndex,ppItem,pdwFlags)	\
    ( (This)->lpVtbl -> GetNamedEvent(This,pwszEventName,dwItemIndex,ppItem,pdwFlags) ) 

#define IAMPlayList_GetRepeatInfo(This,pdwRepeatCount,pdwRepeatStart,pdwRepeatEnd)	\
    ( (This)->lpVtbl -> GetRepeatInfo(This,pdwRepeatCount,pdwRepeatStart,pdwRepeatEnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMPlayList_INTERFACE_DEFINED__ */


#ifndef __ISpecifyParticularPages_INTERFACE_DEFINED__
#define __ISpecifyParticularPages_INTERFACE_DEFINED__

/* interface ISpecifyParticularPages */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ISpecifyParticularPages;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4C437B91-6E9E-11d1-A704-006097C4E476")
    ISpecifyParticularPages : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPages( 
            /* [in] */ __RPC__in REFGUID guidWhatPages,
            /* [out] */ __RPC__out CAUUID *pPages) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpecifyParticularPagesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpecifyParticularPages * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpecifyParticularPages * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpecifyParticularPages * This);
        
        DECLSPEC_XFGVIRT(ISpecifyParticularPages, GetPages)
        HRESULT ( STDMETHODCALLTYPE *GetPages )( 
            __RPC__in ISpecifyParticularPages * This,
            /* [in] */ __RPC__in REFGUID guidWhatPages,
            /* [out] */ __RPC__out CAUUID *pPages);
        
        END_INTERFACE
    } ISpecifyParticularPagesVtbl;

    interface ISpecifyParticularPages
    {
        CONST_VTBL struct ISpecifyParticularPagesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpecifyParticularPages_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpecifyParticularPages_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpecifyParticularPages_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpecifyParticularPages_GetPages(This,guidWhatPages,pPages)	\
    ( (This)->lpVtbl -> GetPages(This,guidWhatPages,pPages) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpecifyParticularPages_INTERFACE_DEFINED__ */


#ifndef __IAMRebuild_INTERFACE_DEFINED__
#define __IAMRebuild_INTERFACE_DEFINED__

/* interface IAMRebuild */
/* [object][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IAMRebuild;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02EF04DD-7580-11d1-BECE-00C04FB6E937")
    IAMRebuild : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RebuildNow( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMRebuildVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMRebuild * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMRebuild * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMRebuild * This);
        
        DECLSPEC_XFGVIRT(IAMRebuild, RebuildNow)
        HRESULT ( STDMETHODCALLTYPE *RebuildNow )( 
            IAMRebuild * This);
        
        END_INTERFACE
    } IAMRebuildVtbl;

    interface IAMRebuild
    {
        CONST_VTBL struct IAMRebuildVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMRebuild_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMRebuild_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMRebuild_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMRebuild_RebuildNow(This)	\
    ( (This)->lpVtbl -> RebuildNow(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMRebuild_INTERFACE_DEFINED__ */


#ifndef __IBufferingTime_INTERFACE_DEFINED__
#define __IBufferingTime_INTERFACE_DEFINED__

/* interface IBufferingTime */
/* [object][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IBufferingTime;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1E00486A-78DD-11D2-8DD3-006097C9A2B2")
    IBufferingTime : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBufferingTime( 
            DWORD *pdwMilliseconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBufferingTime( 
            DWORD dwMilliseconds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBufferingTimeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBufferingTime * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBufferingTime * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBufferingTime * This);
        
        DECLSPEC_XFGVIRT(IBufferingTime, GetBufferingTime)
        HRESULT ( STDMETHODCALLTYPE *GetBufferingTime )( 
            IBufferingTime * This,
            DWORD *pdwMilliseconds);
        
        DECLSPEC_XFGVIRT(IBufferingTime, SetBufferingTime)
        HRESULT ( STDMETHODCALLTYPE *SetBufferingTime )( 
            IBufferingTime * This,
            DWORD dwMilliseconds);
        
        END_INTERFACE
    } IBufferingTimeVtbl;

    interface IBufferingTime
    {
        CONST_VTBL struct IBufferingTimeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBufferingTime_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBufferingTime_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBufferingTime_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBufferingTime_GetBufferingTime(This,pdwMilliseconds)	\
    ( (This)->lpVtbl -> GetBufferingTime(This,pdwMilliseconds) ) 

#define IBufferingTime_SetBufferingTime(This,dwMilliseconds)	\
    ( (This)->lpVtbl -> SetBufferingTime(This,dwMilliseconds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBufferingTime_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_playlist_0000_0005 */
/* [local] */ 

EXTERN_GUID(IID_IAMPlayListItem,0x56a868ff,0x0ad4,0x11ce,0xb0,0xa3,0x0,0x20,0xaf,0x0b,0xa7,0x70);
EXTERN_GUID(IID_IAMRebuild,0x2ef04dd,0x7580,0x11d1,0xbe,0xce,0x0,0xc0,0x4f,0xb6,0xe9,0x37);
EXTERN_GUID(IID_IAMPlayList,0x56a868fe,0x0ad4,0x11ce,0xb0,0xa3,0x0,0x20,0xaf,0x0b,0xa7,0x70);
EXTERN_GUID(SPECIFYPAGES_STATISTICS,0x4c437b92,0x6e9e,0x11d1,0xa7,0x4,0x0,0x60,0x97,0xc4,0xe4,0x76);
EXTERN_GUID(IID_ISpecifyParticularPages,0x4c437b91,0x6e9e,0x11d1,0xa7,0x4,0x0,0x60,0x97,0xc4,0xe4,0x76);
EXTERN_GUID( IID_IBufferingTime, 0x1e00486a,0x78dd,0x11d2,0x8d,0xd3,0x0,0x60,0x97,0xc9,0xa2,0xb2 );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_playlist_0000_0005_v0_0_s_ifspec;

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


