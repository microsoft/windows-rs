

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


#ifndef __control_h__
#define __control_h__

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

#ifndef __IAMCollection_FWD_DEFINED__
#define __IAMCollection_FWD_DEFINED__
typedef interface IAMCollection IAMCollection;

#endif 	/* __IAMCollection_FWD_DEFINED__ */


#ifndef __IMediaControl_FWD_DEFINED__
#define __IMediaControl_FWD_DEFINED__
typedef interface IMediaControl IMediaControl;

#endif 	/* __IMediaControl_FWD_DEFINED__ */


#ifndef __IMediaEvent_FWD_DEFINED__
#define __IMediaEvent_FWD_DEFINED__
typedef interface IMediaEvent IMediaEvent;

#endif 	/* __IMediaEvent_FWD_DEFINED__ */


#ifndef __IMediaEventEx_FWD_DEFINED__
#define __IMediaEventEx_FWD_DEFINED__
typedef interface IMediaEventEx IMediaEventEx;

#endif 	/* __IMediaEventEx_FWD_DEFINED__ */


#ifndef __IMediaPosition_FWD_DEFINED__
#define __IMediaPosition_FWD_DEFINED__
typedef interface IMediaPosition IMediaPosition;

#endif 	/* __IMediaPosition_FWD_DEFINED__ */


#ifndef __IBasicAudio_FWD_DEFINED__
#define __IBasicAudio_FWD_DEFINED__
typedef interface IBasicAudio IBasicAudio;

#endif 	/* __IBasicAudio_FWD_DEFINED__ */


#ifndef __IVideoWindow_FWD_DEFINED__
#define __IVideoWindow_FWD_DEFINED__
typedef interface IVideoWindow IVideoWindow;

#endif 	/* __IVideoWindow_FWD_DEFINED__ */


#ifndef __IBasicVideo_FWD_DEFINED__
#define __IBasicVideo_FWD_DEFINED__
typedef interface IBasicVideo IBasicVideo;

#endif 	/* __IBasicVideo_FWD_DEFINED__ */


#ifndef __IBasicVideo2_FWD_DEFINED__
#define __IBasicVideo2_FWD_DEFINED__
typedef interface IBasicVideo2 IBasicVideo2;

#endif 	/* __IBasicVideo2_FWD_DEFINED__ */


#ifndef __IDeferredCommand_FWD_DEFINED__
#define __IDeferredCommand_FWD_DEFINED__
typedef interface IDeferredCommand IDeferredCommand;

#endif 	/* __IDeferredCommand_FWD_DEFINED__ */


#ifndef __IQueueCommand_FWD_DEFINED__
#define __IQueueCommand_FWD_DEFINED__
typedef interface IQueueCommand IQueueCommand;

#endif 	/* __IQueueCommand_FWD_DEFINED__ */


#ifndef __FilgraphManager_FWD_DEFINED__
#define __FilgraphManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FilgraphManager FilgraphManager;
#else
typedef struct FilgraphManager FilgraphManager;
#endif /* __cplusplus */

#endif 	/* __FilgraphManager_FWD_DEFINED__ */


#ifndef __IFilterInfo_FWD_DEFINED__
#define __IFilterInfo_FWD_DEFINED__
typedef interface IFilterInfo IFilterInfo;

#endif 	/* __IFilterInfo_FWD_DEFINED__ */


#ifndef __IRegFilterInfo_FWD_DEFINED__
#define __IRegFilterInfo_FWD_DEFINED__
typedef interface IRegFilterInfo IRegFilterInfo;

#endif 	/* __IRegFilterInfo_FWD_DEFINED__ */


#ifndef __IMediaTypeInfo_FWD_DEFINED__
#define __IMediaTypeInfo_FWD_DEFINED__
typedef interface IMediaTypeInfo IMediaTypeInfo;

#endif 	/* __IMediaTypeInfo_FWD_DEFINED__ */


#ifndef __IPinInfo_FWD_DEFINED__
#define __IPinInfo_FWD_DEFINED__
typedef interface IPinInfo IPinInfo;

#endif 	/* __IPinInfo_FWD_DEFINED__ */


#ifndef __IAMStats_FWD_DEFINED__
#define __IAMStats_FWD_DEFINED__
typedef interface IAMStats IAMStats;

#endif 	/* __IAMStats_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __QuartzTypeLib_LIBRARY_DEFINED__
#define __QuartzTypeLib_LIBRARY_DEFINED__

/* library QuartzTypeLib */
/* [version][lcid][helpstring][uuid] */ 

typedef double REFTIME;

typedef LONG_PTR OAEVENT;

typedef LONG_PTR OAHWND;

typedef long OAFilterState;


DEFINE_GUID(LIBID_QuartzTypeLib,0x56a868b0,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#ifndef __IAMCollection_INTERFACE_DEFINED__
#define __IAMCollection_INTERFACE_DEFINED__

/* interface IAMCollection */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IAMCollection,0x56a868b9,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b9-0ad4-11ce-b03a-0020af0ba770")
    IAMCollection : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long lItem,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAMCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAMCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAMCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAMCollection * This,
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
        
        DECLSPEC_XFGVIRT(IAMCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAMCollection * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAMCollection, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IAMCollection * This,
            /* [in] */ long lItem,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IAMCollection, get__NewEnum)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAMCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        END_INTERFACE
    } IAMCollectionVtbl;

    interface IAMCollection
    {
        CONST_VTBL struct IAMCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAMCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAMCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAMCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAMCollection_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAMCollection_Item(This,lItem,ppUnk)	\
    ( (This)->lpVtbl -> Item(This,lItem,ppUnk) ) 

#define IAMCollection_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMCollection_INTERFACE_DEFINED__ */


#ifndef __IMediaControl_INTERFACE_DEFINED__
#define __IMediaControl_INTERFACE_DEFINED__

/* interface IMediaControl */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IMediaControl,0x56a868b1,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b1-0ad4-11ce-b03a-0020af0ba770")
    IMediaControl : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Run( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [in] */ LONG msTimeout,
            /* [out] */ __RPC__out OAFilterState *pfs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderFile( 
            /* [in] */ __RPC__in BSTR strFilename) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSourceFilter( 
            /* [in] */ __RPC__in BSTR strFilename,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilterCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RegFilterCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopWhenReady( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMediaControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMediaControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMediaControl * This,
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
        
        DECLSPEC_XFGVIRT(IMediaControl, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IMediaControl * This);
        
        DECLSPEC_XFGVIRT(IMediaControl, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMediaControl * This);
        
        DECLSPEC_XFGVIRT(IMediaControl, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMediaControl * This);
        
        DECLSPEC_XFGVIRT(IMediaControl, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IMediaControl * This,
            /* [in] */ LONG msTimeout,
            /* [out] */ __RPC__out OAFilterState *pfs);
        
        DECLSPEC_XFGVIRT(IMediaControl, RenderFile)
        HRESULT ( STDMETHODCALLTYPE *RenderFile )( 
            __RPC__in IMediaControl * This,
            /* [in] */ __RPC__in BSTR strFilename);
        
        DECLSPEC_XFGVIRT(IMediaControl, AddSourceFilter)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilter )( 
            __RPC__in IMediaControl * This,
            /* [in] */ __RPC__in BSTR strFilename,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IMediaControl, get_FilterCollection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterCollection )( 
            __RPC__in IMediaControl * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IMediaControl, get_RegFilterCollection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegFilterCollection )( 
            __RPC__in IMediaControl * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IMediaControl, StopWhenReady)
        HRESULT ( STDMETHODCALLTYPE *StopWhenReady )( 
            __RPC__in IMediaControl * This);
        
        END_INTERFACE
    } IMediaControlVtbl;

    interface IMediaControl
    {
        CONST_VTBL struct IMediaControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMediaControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMediaControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMediaControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMediaControl_Run(This)	\
    ( (This)->lpVtbl -> Run(This) ) 

#define IMediaControl_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMediaControl_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMediaControl_GetState(This,msTimeout,pfs)	\
    ( (This)->lpVtbl -> GetState(This,msTimeout,pfs) ) 

#define IMediaControl_RenderFile(This,strFilename)	\
    ( (This)->lpVtbl -> RenderFile(This,strFilename) ) 

#define IMediaControl_AddSourceFilter(This,strFilename,ppUnk)	\
    ( (This)->lpVtbl -> AddSourceFilter(This,strFilename,ppUnk) ) 

#define IMediaControl_get_FilterCollection(This,ppUnk)	\
    ( (This)->lpVtbl -> get_FilterCollection(This,ppUnk) ) 

#define IMediaControl_get_RegFilterCollection(This,ppUnk)	\
    ( (This)->lpVtbl -> get_RegFilterCollection(This,ppUnk) ) 

#define IMediaControl_StopWhenReady(This)	\
    ( (This)->lpVtbl -> StopWhenReady(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaControl_INTERFACE_DEFINED__ */


#ifndef __IMediaEvent_INTERFACE_DEFINED__
#define __IMediaEvent_INTERFACE_DEFINED__

/* interface IMediaEvent */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IMediaEvent,0x56a868b6,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b6-0ad4-11ce-b03a-0020af0ba770")
    IMediaEvent : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEventHandle( 
            /* [out] */ __RPC__out OAEVENT *hEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [out] */ __RPC__out long *lEventCode,
            /* [out] */ __RPC__out LONG_PTR *lParam1,
            /* [out] */ __RPC__out LONG_PTR *lParam2,
            /* [in] */ long msTimeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitForCompletion( 
            /* [in] */ long msTimeout,
            /* [out] */ __RPC__out long *pEvCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelDefaultHandling( 
            /* [in] */ long lEvCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreDefaultHandling( 
            /* [in] */ long lEvCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeEventParams( 
            /* [in] */ long lEvCode,
            /* [in] */ LONG_PTR lParam1,
            /* [in] */ LONG_PTR lParam2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMediaEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMediaEvent * This,
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
        
        DECLSPEC_XFGVIRT(IMediaEvent, GetEventHandle)
        HRESULT ( STDMETHODCALLTYPE *GetEventHandle )( 
            __RPC__in IMediaEvent * This,
            /* [out] */ __RPC__out OAEVENT *hEvent);
        
        DECLSPEC_XFGVIRT(IMediaEvent, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMediaEvent * This,
            /* [out] */ __RPC__out long *lEventCode,
            /* [out] */ __RPC__out LONG_PTR *lParam1,
            /* [out] */ __RPC__out LONG_PTR *lParam2,
            /* [in] */ long msTimeout);
        
        DECLSPEC_XFGVIRT(IMediaEvent, WaitForCompletion)
        HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ long msTimeout,
            /* [out] */ __RPC__out long *pEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, CancelDefaultHandling)
        HRESULT ( STDMETHODCALLTYPE *CancelDefaultHandling )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ long lEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, RestoreDefaultHandling)
        HRESULT ( STDMETHODCALLTYPE *RestoreDefaultHandling )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ long lEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, FreeEventParams)
        HRESULT ( STDMETHODCALLTYPE *FreeEventParams )( 
            __RPC__in IMediaEvent * This,
            /* [in] */ long lEvCode,
            /* [in] */ LONG_PTR lParam1,
            /* [in] */ LONG_PTR lParam2);
        
        END_INTERFACE
    } IMediaEventVtbl;

    interface IMediaEvent
    {
        CONST_VTBL struct IMediaEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMediaEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMediaEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMediaEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMediaEvent_GetEventHandle(This,hEvent)	\
    ( (This)->lpVtbl -> GetEventHandle(This,hEvent) ) 

#define IMediaEvent_GetEvent(This,lEventCode,lParam1,lParam2,msTimeout)	\
    ( (This)->lpVtbl -> GetEvent(This,lEventCode,lParam1,lParam2,msTimeout) ) 

#define IMediaEvent_WaitForCompletion(This,msTimeout,pEvCode)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,msTimeout,pEvCode) ) 

#define IMediaEvent_CancelDefaultHandling(This,lEvCode)	\
    ( (This)->lpVtbl -> CancelDefaultHandling(This,lEvCode) ) 

#define IMediaEvent_RestoreDefaultHandling(This,lEvCode)	\
    ( (This)->lpVtbl -> RestoreDefaultHandling(This,lEvCode) ) 

#define IMediaEvent_FreeEventParams(This,lEvCode,lParam1,lParam2)	\
    ( (This)->lpVtbl -> FreeEventParams(This,lEvCode,lParam1,lParam2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaEvent_INTERFACE_DEFINED__ */


#ifndef __IMediaEventEx_INTERFACE_DEFINED__
#define __IMediaEventEx_INTERFACE_DEFINED__

/* interface IMediaEventEx */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_IMediaEventEx,0x56a868c0,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868c0-0ad4-11ce-b03a-0020af0ba770")
    IMediaEventEx : public IMediaEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNotifyWindow( 
            /* [in] */ OAHWND hwnd,
            /* [in] */ long lMsg,
            /* [in] */ LONG_PTR lInstanceData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNotifyFlags( 
            /* [in] */ long lNoNotifyFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNotifyFlags( 
            /* [out] */ __RPC__out long *lplNoNotifyFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaEventExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaEventEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaEventEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMediaEventEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMediaEventEx * This,
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
        
        DECLSPEC_XFGVIRT(IMediaEvent, GetEventHandle)
        HRESULT ( STDMETHODCALLTYPE *GetEventHandle )( 
            __RPC__in IMediaEventEx * This,
            /* [out] */ __RPC__out OAEVENT *hEvent);
        
        DECLSPEC_XFGVIRT(IMediaEvent, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMediaEventEx * This,
            /* [out] */ __RPC__out long *lEventCode,
            /* [out] */ __RPC__out LONG_PTR *lParam1,
            /* [out] */ __RPC__out LONG_PTR *lParam2,
            /* [in] */ long msTimeout);
        
        DECLSPEC_XFGVIRT(IMediaEvent, WaitForCompletion)
        HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ long msTimeout,
            /* [out] */ __RPC__out long *pEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, CancelDefaultHandling)
        HRESULT ( STDMETHODCALLTYPE *CancelDefaultHandling )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ long lEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, RestoreDefaultHandling)
        HRESULT ( STDMETHODCALLTYPE *RestoreDefaultHandling )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ long lEvCode);
        
        DECLSPEC_XFGVIRT(IMediaEvent, FreeEventParams)
        HRESULT ( STDMETHODCALLTYPE *FreeEventParams )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ long lEvCode,
            /* [in] */ LONG_PTR lParam1,
            /* [in] */ LONG_PTR lParam2);
        
        DECLSPEC_XFGVIRT(IMediaEventEx, SetNotifyWindow)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyWindow )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ OAHWND hwnd,
            /* [in] */ long lMsg,
            /* [in] */ LONG_PTR lInstanceData);
        
        DECLSPEC_XFGVIRT(IMediaEventEx, SetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyFlags )( 
            __RPC__in IMediaEventEx * This,
            /* [in] */ long lNoNotifyFlags);
        
        DECLSPEC_XFGVIRT(IMediaEventEx, GetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyFlags )( 
            __RPC__in IMediaEventEx * This,
            /* [out] */ __RPC__out long *lplNoNotifyFlags);
        
        END_INTERFACE
    } IMediaEventExVtbl;

    interface IMediaEventEx
    {
        CONST_VTBL struct IMediaEventExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaEventEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaEventEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaEventEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaEventEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMediaEventEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMediaEventEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMediaEventEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMediaEventEx_GetEventHandle(This,hEvent)	\
    ( (This)->lpVtbl -> GetEventHandle(This,hEvent) ) 

#define IMediaEventEx_GetEvent(This,lEventCode,lParam1,lParam2,msTimeout)	\
    ( (This)->lpVtbl -> GetEvent(This,lEventCode,lParam1,lParam2,msTimeout) ) 

#define IMediaEventEx_WaitForCompletion(This,msTimeout,pEvCode)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,msTimeout,pEvCode) ) 

#define IMediaEventEx_CancelDefaultHandling(This,lEvCode)	\
    ( (This)->lpVtbl -> CancelDefaultHandling(This,lEvCode) ) 

#define IMediaEventEx_RestoreDefaultHandling(This,lEvCode)	\
    ( (This)->lpVtbl -> RestoreDefaultHandling(This,lEvCode) ) 

#define IMediaEventEx_FreeEventParams(This,lEvCode,lParam1,lParam2)	\
    ( (This)->lpVtbl -> FreeEventParams(This,lEvCode,lParam1,lParam2) ) 


#define IMediaEventEx_SetNotifyWindow(This,hwnd,lMsg,lInstanceData)	\
    ( (This)->lpVtbl -> SetNotifyWindow(This,hwnd,lMsg,lInstanceData) ) 

#define IMediaEventEx_SetNotifyFlags(This,lNoNotifyFlags)	\
    ( (This)->lpVtbl -> SetNotifyFlags(This,lNoNotifyFlags) ) 

#define IMediaEventEx_GetNotifyFlags(This,lplNoNotifyFlags)	\
    ( (This)->lpVtbl -> GetNotifyFlags(This,lplNoNotifyFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaEventEx_INTERFACE_DEFINED__ */


#ifndef __IMediaPosition_INTERFACE_DEFINED__
#define __IMediaPosition_INTERFACE_DEFINED__

/* interface IMediaPosition */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IMediaPosition,0x56a868b2,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b2-0ad4-11ce-b03a-0020af0ba770")
    IMediaPosition : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Duration( 
            /* [retval][out] */ __RPC__out REFTIME *plength) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CurrentPosition( 
            /* [in] */ REFTIME llTime) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPosition( 
            /* [retval][out] */ __RPC__out REFTIME *pllTime) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StopTime( 
            /* [retval][out] */ __RPC__out REFTIME *pllTime) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_StopTime( 
            /* [in] */ REFTIME llTime) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PrerollTime( 
            /* [retval][out] */ __RPC__out REFTIME *pllTime) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PrerollTime( 
            /* [in] */ REFTIME llTime) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Rate( 
            /* [in] */ double dRate) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Rate( 
            /* [retval][out] */ __RPC__out double *pdRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanSeekForward( 
            /* [retval][out] */ __RPC__out LONG *pCanSeekForward) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanSeekBackward( 
            /* [retval][out] */ __RPC__out LONG *pCanSeekBackward) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaPositionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaPosition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaPosition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMediaPosition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMediaPosition * This,
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
        
        DECLSPEC_XFGVIRT(IMediaPosition, get_Duration)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Duration )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out REFTIME *plength);
        
        DECLSPEC_XFGVIRT(IMediaPosition, put_CurrentPosition)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CurrentPosition )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ REFTIME llTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, get_CurrentPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPosition )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out REFTIME *pllTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, get_StopTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StopTime )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out REFTIME *pllTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, put_StopTime)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_StopTime )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ REFTIME llTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, get_PrerollTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrerollTime )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out REFTIME *pllTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, put_PrerollTime)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrerollTime )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ REFTIME llTime);
        
        DECLSPEC_XFGVIRT(IMediaPosition, put_Rate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Rate )( 
            __RPC__in IMediaPosition * This,
            /* [in] */ double dRate);
        
        DECLSPEC_XFGVIRT(IMediaPosition, get_Rate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Rate )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out double *pdRate);
        
        DECLSPEC_XFGVIRT(IMediaPosition, CanSeekForward)
        HRESULT ( STDMETHODCALLTYPE *CanSeekForward )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out LONG *pCanSeekForward);
        
        DECLSPEC_XFGVIRT(IMediaPosition, CanSeekBackward)
        HRESULT ( STDMETHODCALLTYPE *CanSeekBackward )( 
            __RPC__in IMediaPosition * This,
            /* [retval][out] */ __RPC__out LONG *pCanSeekBackward);
        
        END_INTERFACE
    } IMediaPositionVtbl;

    interface IMediaPosition
    {
        CONST_VTBL struct IMediaPositionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaPosition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaPosition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaPosition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaPosition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMediaPosition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMediaPosition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMediaPosition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMediaPosition_get_Duration(This,plength)	\
    ( (This)->lpVtbl -> get_Duration(This,plength) ) 

#define IMediaPosition_put_CurrentPosition(This,llTime)	\
    ( (This)->lpVtbl -> put_CurrentPosition(This,llTime) ) 

#define IMediaPosition_get_CurrentPosition(This,pllTime)	\
    ( (This)->lpVtbl -> get_CurrentPosition(This,pllTime) ) 

#define IMediaPosition_get_StopTime(This,pllTime)	\
    ( (This)->lpVtbl -> get_StopTime(This,pllTime) ) 

#define IMediaPosition_put_StopTime(This,llTime)	\
    ( (This)->lpVtbl -> put_StopTime(This,llTime) ) 

#define IMediaPosition_get_PrerollTime(This,pllTime)	\
    ( (This)->lpVtbl -> get_PrerollTime(This,pllTime) ) 

#define IMediaPosition_put_PrerollTime(This,llTime)	\
    ( (This)->lpVtbl -> put_PrerollTime(This,llTime) ) 

#define IMediaPosition_put_Rate(This,dRate)	\
    ( (This)->lpVtbl -> put_Rate(This,dRate) ) 

#define IMediaPosition_get_Rate(This,pdRate)	\
    ( (This)->lpVtbl -> get_Rate(This,pdRate) ) 

#define IMediaPosition_CanSeekForward(This,pCanSeekForward)	\
    ( (This)->lpVtbl -> CanSeekForward(This,pCanSeekForward) ) 

#define IMediaPosition_CanSeekBackward(This,pCanSeekBackward)	\
    ( (This)->lpVtbl -> CanSeekBackward(This,pCanSeekBackward) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaPosition_INTERFACE_DEFINED__ */


#ifndef __IBasicAudio_INTERFACE_DEFINED__
#define __IBasicAudio_INTERFACE_DEFINED__

/* interface IBasicAudio */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IBasicAudio,0x56a868b3,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b3-0ad4-11ce-b03a-0020af0ba770")
    IBasicAudio : public IDispatch
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Volume( 
            /* [in] */ long lVolume) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Volume( 
            /* [retval][out] */ __RPC__out long *plVolume) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Balance( 
            /* [in] */ long lBalance) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Balance( 
            /* [retval][out] */ __RPC__out long *plBalance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBasicAudioVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBasicAudio * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBasicAudio * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBasicAudio * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBasicAudio * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBasicAudio * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBasicAudio * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBasicAudio * This,
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
        
        DECLSPEC_XFGVIRT(IBasicAudio, put_Volume)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Volume )( 
            __RPC__in IBasicAudio * This,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(IBasicAudio, get_Volume)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Volume )( 
            __RPC__in IBasicAudio * This,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(IBasicAudio, put_Balance)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Balance )( 
            __RPC__in IBasicAudio * This,
            /* [in] */ long lBalance);
        
        DECLSPEC_XFGVIRT(IBasicAudio, get_Balance)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Balance )( 
            __RPC__in IBasicAudio * This,
            /* [retval][out] */ __RPC__out long *plBalance);
        
        END_INTERFACE
    } IBasicAudioVtbl;

    interface IBasicAudio
    {
        CONST_VTBL struct IBasicAudioVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBasicAudio_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBasicAudio_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBasicAudio_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBasicAudio_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBasicAudio_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBasicAudio_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBasicAudio_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBasicAudio_put_Volume(This,lVolume)	\
    ( (This)->lpVtbl -> put_Volume(This,lVolume) ) 

#define IBasicAudio_get_Volume(This,plVolume)	\
    ( (This)->lpVtbl -> get_Volume(This,plVolume) ) 

#define IBasicAudio_put_Balance(This,lBalance)	\
    ( (This)->lpVtbl -> put_Balance(This,lBalance) ) 

#define IBasicAudio_get_Balance(This,plBalance)	\
    ( (This)->lpVtbl -> get_Balance(This,plBalance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBasicAudio_INTERFACE_DEFINED__ */


#ifndef __IVideoWindow_INTERFACE_DEFINED__
#define __IVideoWindow_INTERFACE_DEFINED__

/* interface IVideoWindow */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IVideoWindow,0x56a868b4,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b4-0ad4-11ce-b03a-0020af0ba770")
    IVideoWindow : public IDispatch
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Caption( 
            /* [in] */ __RPC__in BSTR strCaption) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Caption( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strCaption) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WindowStyle( 
            /* [in] */ long WindowStyle) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowStyle( 
            /* [retval][out] */ __RPC__out long *WindowStyle) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WindowStyleEx( 
            /* [in] */ long WindowStyleEx) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowStyleEx( 
            /* [retval][out] */ __RPC__out long *WindowStyleEx) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AutoShow( 
            /* [in] */ long AutoShow) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AutoShow( 
            /* [retval][out] */ __RPC__out long *AutoShow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WindowState( 
            /* [in] */ long WindowState) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowState( 
            /* [retval][out] */ __RPC__out long *WindowState) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BackgroundPalette( 
            /* [in] */ long BackgroundPalette) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BackgroundPalette( 
            /* [retval][out] */ __RPC__out long *pBackgroundPalette) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Visible( 
            /* [in] */ long Visible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out long *pVisible) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Left( 
            /* [in] */ long Left) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Left( 
            /* [retval][out] */ __RPC__out long *pLeft) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ long Width) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out long *pWidth) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Top( 
            /* [in] */ long Top) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Top( 
            /* [retval][out] */ __RPC__out long *pTop) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Height( 
            /* [in] */ long Height) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Height( 
            /* [retval][out] */ __RPC__out long *pHeight) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Owner( 
            /* [in] */ OAHWND Owner) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Owner( 
            /* [retval][out] */ __RPC__out OAHWND *Owner) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MessageDrain( 
            /* [in] */ OAHWND Drain) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MessageDrain( 
            /* [retval][out] */ __RPC__out OAHWND *Drain) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BorderColor( 
            /* [retval][out] */ __RPC__out long *Color) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BorderColor( 
            /* [in] */ long Color) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FullScreenMode( 
            /* [retval][out] */ __RPC__out long *FullScreenMode) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FullScreenMode( 
            /* [in] */ long FullScreenMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWindowForeground( 
            /* [in] */ long Focus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyOwnerMessage( 
            /* [in] */ OAHWND hwnd,
            /* [in] */ long uMsg,
            /* [in] */ LONG_PTR wParam,
            /* [in] */ LONG_PTR lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWindowPosition( 
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWindowPosition( 
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMinIdealImageSize( 
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxIdealImageSize( 
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRestorePosition( 
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HideCursor( 
            /* [in] */ long HideCursor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCursorHidden( 
            /* [out] */ __RPC__out long *CursorHidden) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoWindowVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVideoWindow * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVideoWindow * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVideoWindow * This,
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
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Caption)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Caption )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ __RPC__in BSTR strCaption);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Caption)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strCaption);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_WindowStyle)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WindowStyle )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long WindowStyle);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_WindowStyle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowStyle )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *WindowStyle);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_WindowStyleEx)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WindowStyleEx )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long WindowStyleEx);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_WindowStyleEx)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowStyleEx )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *WindowStyleEx);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_AutoShow)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoShow )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long AutoShow);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_AutoShow)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoShow )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *AutoShow);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_WindowState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WindowState )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long WindowState);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_WindowState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowState )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *WindowState);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_BackgroundPalette)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BackgroundPalette )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long BackgroundPalette);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_BackgroundPalette)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackgroundPalette )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pBackgroundPalette);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Visible)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Visible);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Visible)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pVisible);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Left)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Left);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Left)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pLeft);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Width)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Width);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Width)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pWidth);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Top)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Top);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Top)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pTop);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Height)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Height )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Height)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_Owner)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Owner )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ OAHWND Owner);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_Owner)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Owner )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out OAHWND *Owner);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_MessageDrain)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageDrain )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ OAHWND Drain);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_MessageDrain)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageDrain )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out OAHWND *Drain);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_BorderColor)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BorderColor )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *Color);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_BorderColor)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BorderColor )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Color);
        
        DECLSPEC_XFGVIRT(IVideoWindow, get_FullScreenMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullScreenMode )( 
            __RPC__in IVideoWindow * This,
            /* [retval][out] */ __RPC__out long *FullScreenMode);
        
        DECLSPEC_XFGVIRT(IVideoWindow, put_FullScreenMode)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FullScreenMode )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long FullScreenMode);
        
        DECLSPEC_XFGVIRT(IVideoWindow, SetWindowForeground)
        HRESULT ( STDMETHODCALLTYPE *SetWindowForeground )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Focus);
        
        DECLSPEC_XFGVIRT(IVideoWindow, NotifyOwnerMessage)
        HRESULT ( STDMETHODCALLTYPE *NotifyOwnerMessage )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ OAHWND hwnd,
            /* [in] */ long uMsg,
            /* [in] */ LONG_PTR wParam,
            /* [in] */ LONG_PTR lParam);
        
        DECLSPEC_XFGVIRT(IVideoWindow, SetWindowPosition)
        HRESULT ( STDMETHODCALLTYPE *SetWindowPosition )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IVideoWindow, GetWindowPosition)
        HRESULT ( STDMETHODCALLTYPE *GetWindowPosition )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IVideoWindow, GetMinIdealImageSize)
        HRESULT ( STDMETHODCALLTYPE *GetMinIdealImageSize )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IVideoWindow, GetMaxIdealImageSize)
        HRESULT ( STDMETHODCALLTYPE *GetMaxIdealImageSize )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IVideoWindow, GetRestorePosition)
        HRESULT ( STDMETHODCALLTYPE *GetRestorePosition )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IVideoWindow, HideCursor)
        HRESULT ( STDMETHODCALLTYPE *HideCursor )( 
            __RPC__in IVideoWindow * This,
            /* [in] */ long HideCursor);
        
        DECLSPEC_XFGVIRT(IVideoWindow, IsCursorHidden)
        HRESULT ( STDMETHODCALLTYPE *IsCursorHidden )( 
            __RPC__in IVideoWindow * This,
            /* [out] */ __RPC__out long *CursorHidden);
        
        END_INTERFACE
    } IVideoWindowVtbl;

    interface IVideoWindow
    {
        CONST_VTBL struct IVideoWindowVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoWindow_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoWindow_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoWindow_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoWindow_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVideoWindow_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVideoWindow_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVideoWindow_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVideoWindow_put_Caption(This,strCaption)	\
    ( (This)->lpVtbl -> put_Caption(This,strCaption) ) 

#define IVideoWindow_get_Caption(This,strCaption)	\
    ( (This)->lpVtbl -> get_Caption(This,strCaption) ) 

#define IVideoWindow_put_WindowStyle(This,WindowStyle)	\
    ( (This)->lpVtbl -> put_WindowStyle(This,WindowStyle) ) 

#define IVideoWindow_get_WindowStyle(This,WindowStyle)	\
    ( (This)->lpVtbl -> get_WindowStyle(This,WindowStyle) ) 

#define IVideoWindow_put_WindowStyleEx(This,WindowStyleEx)	\
    ( (This)->lpVtbl -> put_WindowStyleEx(This,WindowStyleEx) ) 

#define IVideoWindow_get_WindowStyleEx(This,WindowStyleEx)	\
    ( (This)->lpVtbl -> get_WindowStyleEx(This,WindowStyleEx) ) 

#define IVideoWindow_put_AutoShow(This,AutoShow)	\
    ( (This)->lpVtbl -> put_AutoShow(This,AutoShow) ) 

#define IVideoWindow_get_AutoShow(This,AutoShow)	\
    ( (This)->lpVtbl -> get_AutoShow(This,AutoShow) ) 

#define IVideoWindow_put_WindowState(This,WindowState)	\
    ( (This)->lpVtbl -> put_WindowState(This,WindowState) ) 

#define IVideoWindow_get_WindowState(This,WindowState)	\
    ( (This)->lpVtbl -> get_WindowState(This,WindowState) ) 

#define IVideoWindow_put_BackgroundPalette(This,BackgroundPalette)	\
    ( (This)->lpVtbl -> put_BackgroundPalette(This,BackgroundPalette) ) 

#define IVideoWindow_get_BackgroundPalette(This,pBackgroundPalette)	\
    ( (This)->lpVtbl -> get_BackgroundPalette(This,pBackgroundPalette) ) 

#define IVideoWindow_put_Visible(This,Visible)	\
    ( (This)->lpVtbl -> put_Visible(This,Visible) ) 

#define IVideoWindow_get_Visible(This,pVisible)	\
    ( (This)->lpVtbl -> get_Visible(This,pVisible) ) 

#define IVideoWindow_put_Left(This,Left)	\
    ( (This)->lpVtbl -> put_Left(This,Left) ) 

#define IVideoWindow_get_Left(This,pLeft)	\
    ( (This)->lpVtbl -> get_Left(This,pLeft) ) 

#define IVideoWindow_put_Width(This,Width)	\
    ( (This)->lpVtbl -> put_Width(This,Width) ) 

#define IVideoWindow_get_Width(This,pWidth)	\
    ( (This)->lpVtbl -> get_Width(This,pWidth) ) 

#define IVideoWindow_put_Top(This,Top)	\
    ( (This)->lpVtbl -> put_Top(This,Top) ) 

#define IVideoWindow_get_Top(This,pTop)	\
    ( (This)->lpVtbl -> get_Top(This,pTop) ) 

#define IVideoWindow_put_Height(This,Height)	\
    ( (This)->lpVtbl -> put_Height(This,Height) ) 

#define IVideoWindow_get_Height(This,pHeight)	\
    ( (This)->lpVtbl -> get_Height(This,pHeight) ) 

#define IVideoWindow_put_Owner(This,Owner)	\
    ( (This)->lpVtbl -> put_Owner(This,Owner) ) 

#define IVideoWindow_get_Owner(This,Owner)	\
    ( (This)->lpVtbl -> get_Owner(This,Owner) ) 

#define IVideoWindow_put_MessageDrain(This,Drain)	\
    ( (This)->lpVtbl -> put_MessageDrain(This,Drain) ) 

#define IVideoWindow_get_MessageDrain(This,Drain)	\
    ( (This)->lpVtbl -> get_MessageDrain(This,Drain) ) 

#define IVideoWindow_get_BorderColor(This,Color)	\
    ( (This)->lpVtbl -> get_BorderColor(This,Color) ) 

#define IVideoWindow_put_BorderColor(This,Color)	\
    ( (This)->lpVtbl -> put_BorderColor(This,Color) ) 

#define IVideoWindow_get_FullScreenMode(This,FullScreenMode)	\
    ( (This)->lpVtbl -> get_FullScreenMode(This,FullScreenMode) ) 

#define IVideoWindow_put_FullScreenMode(This,FullScreenMode)	\
    ( (This)->lpVtbl -> put_FullScreenMode(This,FullScreenMode) ) 

#define IVideoWindow_SetWindowForeground(This,Focus)	\
    ( (This)->lpVtbl -> SetWindowForeground(This,Focus) ) 

#define IVideoWindow_NotifyOwnerMessage(This,hwnd,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> NotifyOwnerMessage(This,hwnd,uMsg,wParam,lParam) ) 

#define IVideoWindow_SetWindowPosition(This,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> SetWindowPosition(This,Left,Top,Width,Height) ) 

#define IVideoWindow_GetWindowPosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetWindowPosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IVideoWindow_GetMinIdealImageSize(This,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetMinIdealImageSize(This,pWidth,pHeight) ) 

#define IVideoWindow_GetMaxIdealImageSize(This,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetMaxIdealImageSize(This,pWidth,pHeight) ) 

#define IVideoWindow_GetRestorePosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetRestorePosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IVideoWindow_HideCursor(This,HideCursor)	\
    ( (This)->lpVtbl -> HideCursor(This,HideCursor) ) 

#define IVideoWindow_IsCursorHidden(This,CursorHidden)	\
    ( (This)->lpVtbl -> IsCursorHidden(This,CursorHidden) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoWindow_INTERFACE_DEFINED__ */


#ifndef __IBasicVideo_INTERFACE_DEFINED__
#define __IBasicVideo_INTERFACE_DEFINED__

/* interface IBasicVideo */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IBasicVideo,0x56a868b5,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b5-0ad4-11ce-b03a-0020af0ba770")
    IBasicVideo : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AvgTimePerFrame( 
            /* [retval][out] */ __RPC__out REFTIME *pAvgTimePerFrame) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BitRate( 
            /* [retval][out] */ __RPC__out long *pBitRate) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BitErrorRate( 
            /* [retval][out] */ __RPC__out long *pBitErrorRate) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VideoWidth( 
            /* [retval][out] */ __RPC__out long *pVideoWidth) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VideoHeight( 
            /* [retval][out] */ __RPC__out long *pVideoHeight) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SourceLeft( 
            /* [in] */ long SourceLeft) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SourceLeft( 
            /* [retval][out] */ __RPC__out long *pSourceLeft) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SourceWidth( 
            /* [in] */ long SourceWidth) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SourceWidth( 
            /* [retval][out] */ __RPC__out long *pSourceWidth) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SourceTop( 
            /* [in] */ long SourceTop) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SourceTop( 
            /* [retval][out] */ __RPC__out long *pSourceTop) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SourceHeight( 
            /* [in] */ long SourceHeight) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SourceHeight( 
            /* [retval][out] */ __RPC__out long *pSourceHeight) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DestinationLeft( 
            /* [in] */ long DestinationLeft) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DestinationLeft( 
            /* [retval][out] */ __RPC__out long *pDestinationLeft) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DestinationWidth( 
            /* [in] */ long DestinationWidth) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DestinationWidth( 
            /* [retval][out] */ __RPC__out long *pDestinationWidth) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DestinationTop( 
            /* [in] */ long DestinationTop) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DestinationTop( 
            /* [retval][out] */ __RPC__out long *pDestinationTop) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DestinationHeight( 
            /* [in] */ long DestinationHeight) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DestinationHeight( 
            /* [retval][out] */ __RPC__out long *pDestinationHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSourcePosition( 
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourcePosition( 
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultSourcePosition( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDestinationPosition( 
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDestinationPosition( 
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultDestinationPosition( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoSize( 
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoPaletteEntries( 
            /* [in] */ long StartIndex,
            /* [in] */ long Entries,
            /* [out] */ __RPC__out long *pRetrieved,
            /* [out] */ __RPC__out long *pPalette) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentImage( 
            /* [out][in] */ __RPC__inout long *pBufferSize,
            /* [out] */ __RPC__out long *pDIBImage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUsingDefaultSource( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUsingDefaultDestination( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBasicVideoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBasicVideo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBasicVideo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBasicVideo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBasicVideo * This,
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
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_AvgTimePerFrame)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvgTimePerFrame )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out REFTIME *pAvgTimePerFrame);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_BitRate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BitRate )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pBitRate);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_BitErrorRate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BitErrorRate )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pBitErrorRate);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_VideoWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VideoWidth )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pVideoWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_VideoHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VideoHeight )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pVideoHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceLeft)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceLeft )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long SourceLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceLeft)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceLeft )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pSourceLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceWidth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceWidth )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long SourceWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceWidth )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pSourceWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceTop)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceTop )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long SourceTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceTop)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceTop )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pSourceTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceHeight)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceHeight )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long SourceHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceHeight )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pSourceHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationLeft)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationLeft )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long DestinationLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationLeft)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationLeft )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pDestinationLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationWidth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationWidth )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long DestinationWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationWidth )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pDestinationWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationTop)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationTop )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long DestinationTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationTop)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationTop )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pDestinationTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationHeight)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationHeight )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long DestinationHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationHeight )( 
            __RPC__in IBasicVideo * This,
            /* [retval][out] */ __RPC__out long *pDestinationHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *SetSourcePosition )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IBasicVideo * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDefaultSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSourcePosition )( 
            __RPC__in IBasicVideo * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *SetDestinationPosition )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *GetDestinationPosition )( 
            __RPC__in IBasicVideo * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDefaultDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultDestinationPosition )( 
            __RPC__in IBasicVideo * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetVideoSize )( 
            __RPC__in IBasicVideo * This,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetVideoPaletteEntries)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPaletteEntries )( 
            __RPC__in IBasicVideo * This,
            /* [in] */ long StartIndex,
            /* [in] */ long Entries,
            /* [out] */ __RPC__out long *pRetrieved,
            /* [out] */ __RPC__out long *pPalette);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetCurrentImage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentImage )( 
            __RPC__in IBasicVideo * This,
            /* [out][in] */ __RPC__inout long *pBufferSize,
            /* [out] */ __RPC__out long *pDIBImage);
        
        DECLSPEC_XFGVIRT(IBasicVideo, IsUsingDefaultSource)
        HRESULT ( STDMETHODCALLTYPE *IsUsingDefaultSource )( 
            __RPC__in IBasicVideo * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, IsUsingDefaultDestination)
        HRESULT ( STDMETHODCALLTYPE *IsUsingDefaultDestination )( 
            __RPC__in IBasicVideo * This);
        
        END_INTERFACE
    } IBasicVideoVtbl;

    interface IBasicVideo
    {
        CONST_VTBL struct IBasicVideoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBasicVideo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBasicVideo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBasicVideo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBasicVideo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBasicVideo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBasicVideo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBasicVideo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBasicVideo_get_AvgTimePerFrame(This,pAvgTimePerFrame)	\
    ( (This)->lpVtbl -> get_AvgTimePerFrame(This,pAvgTimePerFrame) ) 

#define IBasicVideo_get_BitRate(This,pBitRate)	\
    ( (This)->lpVtbl -> get_BitRate(This,pBitRate) ) 

#define IBasicVideo_get_BitErrorRate(This,pBitErrorRate)	\
    ( (This)->lpVtbl -> get_BitErrorRate(This,pBitErrorRate) ) 

#define IBasicVideo_get_VideoWidth(This,pVideoWidth)	\
    ( (This)->lpVtbl -> get_VideoWidth(This,pVideoWidth) ) 

#define IBasicVideo_get_VideoHeight(This,pVideoHeight)	\
    ( (This)->lpVtbl -> get_VideoHeight(This,pVideoHeight) ) 

#define IBasicVideo_put_SourceLeft(This,SourceLeft)	\
    ( (This)->lpVtbl -> put_SourceLeft(This,SourceLeft) ) 

#define IBasicVideo_get_SourceLeft(This,pSourceLeft)	\
    ( (This)->lpVtbl -> get_SourceLeft(This,pSourceLeft) ) 

#define IBasicVideo_put_SourceWidth(This,SourceWidth)	\
    ( (This)->lpVtbl -> put_SourceWidth(This,SourceWidth) ) 

#define IBasicVideo_get_SourceWidth(This,pSourceWidth)	\
    ( (This)->lpVtbl -> get_SourceWidth(This,pSourceWidth) ) 

#define IBasicVideo_put_SourceTop(This,SourceTop)	\
    ( (This)->lpVtbl -> put_SourceTop(This,SourceTop) ) 

#define IBasicVideo_get_SourceTop(This,pSourceTop)	\
    ( (This)->lpVtbl -> get_SourceTop(This,pSourceTop) ) 

#define IBasicVideo_put_SourceHeight(This,SourceHeight)	\
    ( (This)->lpVtbl -> put_SourceHeight(This,SourceHeight) ) 

#define IBasicVideo_get_SourceHeight(This,pSourceHeight)	\
    ( (This)->lpVtbl -> get_SourceHeight(This,pSourceHeight) ) 

#define IBasicVideo_put_DestinationLeft(This,DestinationLeft)	\
    ( (This)->lpVtbl -> put_DestinationLeft(This,DestinationLeft) ) 

#define IBasicVideo_get_DestinationLeft(This,pDestinationLeft)	\
    ( (This)->lpVtbl -> get_DestinationLeft(This,pDestinationLeft) ) 

#define IBasicVideo_put_DestinationWidth(This,DestinationWidth)	\
    ( (This)->lpVtbl -> put_DestinationWidth(This,DestinationWidth) ) 

#define IBasicVideo_get_DestinationWidth(This,pDestinationWidth)	\
    ( (This)->lpVtbl -> get_DestinationWidth(This,pDestinationWidth) ) 

#define IBasicVideo_put_DestinationTop(This,DestinationTop)	\
    ( (This)->lpVtbl -> put_DestinationTop(This,DestinationTop) ) 

#define IBasicVideo_get_DestinationTop(This,pDestinationTop)	\
    ( (This)->lpVtbl -> get_DestinationTop(This,pDestinationTop) ) 

#define IBasicVideo_put_DestinationHeight(This,DestinationHeight)	\
    ( (This)->lpVtbl -> put_DestinationHeight(This,DestinationHeight) ) 

#define IBasicVideo_get_DestinationHeight(This,pDestinationHeight)	\
    ( (This)->lpVtbl -> get_DestinationHeight(This,pDestinationHeight) ) 

#define IBasicVideo_SetSourcePosition(This,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> SetSourcePosition(This,Left,Top,Width,Height) ) 

#define IBasicVideo_GetSourcePosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IBasicVideo_SetDefaultSourcePosition(This)	\
    ( (This)->lpVtbl -> SetDefaultSourcePosition(This) ) 

#define IBasicVideo_SetDestinationPosition(This,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> SetDestinationPosition(This,Left,Top,Width,Height) ) 

#define IBasicVideo_GetDestinationPosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetDestinationPosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IBasicVideo_SetDefaultDestinationPosition(This)	\
    ( (This)->lpVtbl -> SetDefaultDestinationPosition(This) ) 

#define IBasicVideo_GetVideoSize(This,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetVideoSize(This,pWidth,pHeight) ) 

#define IBasicVideo_GetVideoPaletteEntries(This,StartIndex,Entries,pRetrieved,pPalette)	\
    ( (This)->lpVtbl -> GetVideoPaletteEntries(This,StartIndex,Entries,pRetrieved,pPalette) ) 

#define IBasicVideo_GetCurrentImage(This,pBufferSize,pDIBImage)	\
    ( (This)->lpVtbl -> GetCurrentImage(This,pBufferSize,pDIBImage) ) 

#define IBasicVideo_IsUsingDefaultSource(This)	\
    ( (This)->lpVtbl -> IsUsingDefaultSource(This) ) 

#define IBasicVideo_IsUsingDefaultDestination(This)	\
    ( (This)->lpVtbl -> IsUsingDefaultDestination(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBasicVideo_INTERFACE_DEFINED__ */


#ifndef __IBasicVideo2_INTERFACE_DEFINED__
#define __IBasicVideo2_INTERFACE_DEFINED__

/* interface IBasicVideo2 */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_IBasicVideo2,0x329bb360,0xf6ea,0x11d1,0x90,0x38,0x00,0xa0,0xc9,0x69,0x72,0x98);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("329bb360-f6ea-11d1-9038-00a0c9697298")
    IBasicVideo2 : public IBasicVideo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreferredAspectRatio( 
            /* [out] */ __RPC__out long *plAspectX,
            /* [out] */ __RPC__out long *plAspectY) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBasicVideo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBasicVideo2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBasicVideo2 * This,
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
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_AvgTimePerFrame)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvgTimePerFrame )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out REFTIME *pAvgTimePerFrame);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_BitRate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BitRate )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pBitRate);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_BitErrorRate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BitErrorRate )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pBitErrorRate);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_VideoWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VideoWidth )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pVideoWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_VideoHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VideoHeight )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pVideoHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceLeft)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceLeft )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long SourceLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceLeft)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceLeft )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pSourceLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceWidth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceWidth )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long SourceWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceWidth )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pSourceWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceTop)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceTop )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long SourceTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceTop)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceTop )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pSourceTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_SourceHeight)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceHeight )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long SourceHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_SourceHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceHeight )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pSourceHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationLeft)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationLeft )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long DestinationLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationLeft)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationLeft )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pDestinationLeft);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationWidth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationWidth )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long DestinationWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationWidth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationWidth )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pDestinationWidth);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationTop)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationTop )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long DestinationTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationTop)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationTop )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pDestinationTop);
        
        DECLSPEC_XFGVIRT(IBasicVideo, put_DestinationHeight)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationHeight )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long DestinationHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, get_DestinationHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationHeight )( 
            __RPC__in IBasicVideo2 * This,
            /* [retval][out] */ __RPC__out long *pDestinationHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *SetSourcePosition )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IBasicVideo2 * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDefaultSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSourcePosition )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *SetDestinationPosition )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long Left,
            /* [in] */ long Top,
            /* [in] */ long Width,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *GetDestinationPosition )( 
            __RPC__in IBasicVideo2 * This,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, SetDefaultDestinationPosition)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultDestinationPosition )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetVideoSize )( 
            __RPC__in IBasicVideo2 * This,
            /* [out] */ __RPC__out long *pWidth,
            /* [out] */ __RPC__out long *pHeight);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetVideoPaletteEntries)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPaletteEntries )( 
            __RPC__in IBasicVideo2 * This,
            /* [in] */ long StartIndex,
            /* [in] */ long Entries,
            /* [out] */ __RPC__out long *pRetrieved,
            /* [out] */ __RPC__out long *pPalette);
        
        DECLSPEC_XFGVIRT(IBasicVideo, GetCurrentImage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentImage )( 
            __RPC__in IBasicVideo2 * This,
            /* [out][in] */ __RPC__inout long *pBufferSize,
            /* [out] */ __RPC__out long *pDIBImage);
        
        DECLSPEC_XFGVIRT(IBasicVideo, IsUsingDefaultSource)
        HRESULT ( STDMETHODCALLTYPE *IsUsingDefaultSource )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo, IsUsingDefaultDestination)
        HRESULT ( STDMETHODCALLTYPE *IsUsingDefaultDestination )( 
            __RPC__in IBasicVideo2 * This);
        
        DECLSPEC_XFGVIRT(IBasicVideo2, GetPreferredAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredAspectRatio )( 
            __RPC__in IBasicVideo2 * This,
            /* [out] */ __RPC__out long *plAspectX,
            /* [out] */ __RPC__out long *plAspectY);
        
        END_INTERFACE
    } IBasicVideo2Vtbl;

    interface IBasicVideo2
    {
        CONST_VTBL struct IBasicVideo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBasicVideo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBasicVideo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBasicVideo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBasicVideo2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBasicVideo2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBasicVideo2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBasicVideo2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBasicVideo2_get_AvgTimePerFrame(This,pAvgTimePerFrame)	\
    ( (This)->lpVtbl -> get_AvgTimePerFrame(This,pAvgTimePerFrame) ) 

#define IBasicVideo2_get_BitRate(This,pBitRate)	\
    ( (This)->lpVtbl -> get_BitRate(This,pBitRate) ) 

#define IBasicVideo2_get_BitErrorRate(This,pBitErrorRate)	\
    ( (This)->lpVtbl -> get_BitErrorRate(This,pBitErrorRate) ) 

#define IBasicVideo2_get_VideoWidth(This,pVideoWidth)	\
    ( (This)->lpVtbl -> get_VideoWidth(This,pVideoWidth) ) 

#define IBasicVideo2_get_VideoHeight(This,pVideoHeight)	\
    ( (This)->lpVtbl -> get_VideoHeight(This,pVideoHeight) ) 

#define IBasicVideo2_put_SourceLeft(This,SourceLeft)	\
    ( (This)->lpVtbl -> put_SourceLeft(This,SourceLeft) ) 

#define IBasicVideo2_get_SourceLeft(This,pSourceLeft)	\
    ( (This)->lpVtbl -> get_SourceLeft(This,pSourceLeft) ) 

#define IBasicVideo2_put_SourceWidth(This,SourceWidth)	\
    ( (This)->lpVtbl -> put_SourceWidth(This,SourceWidth) ) 

#define IBasicVideo2_get_SourceWidth(This,pSourceWidth)	\
    ( (This)->lpVtbl -> get_SourceWidth(This,pSourceWidth) ) 

#define IBasicVideo2_put_SourceTop(This,SourceTop)	\
    ( (This)->lpVtbl -> put_SourceTop(This,SourceTop) ) 

#define IBasicVideo2_get_SourceTop(This,pSourceTop)	\
    ( (This)->lpVtbl -> get_SourceTop(This,pSourceTop) ) 

#define IBasicVideo2_put_SourceHeight(This,SourceHeight)	\
    ( (This)->lpVtbl -> put_SourceHeight(This,SourceHeight) ) 

#define IBasicVideo2_get_SourceHeight(This,pSourceHeight)	\
    ( (This)->lpVtbl -> get_SourceHeight(This,pSourceHeight) ) 

#define IBasicVideo2_put_DestinationLeft(This,DestinationLeft)	\
    ( (This)->lpVtbl -> put_DestinationLeft(This,DestinationLeft) ) 

#define IBasicVideo2_get_DestinationLeft(This,pDestinationLeft)	\
    ( (This)->lpVtbl -> get_DestinationLeft(This,pDestinationLeft) ) 

#define IBasicVideo2_put_DestinationWidth(This,DestinationWidth)	\
    ( (This)->lpVtbl -> put_DestinationWidth(This,DestinationWidth) ) 

#define IBasicVideo2_get_DestinationWidth(This,pDestinationWidth)	\
    ( (This)->lpVtbl -> get_DestinationWidth(This,pDestinationWidth) ) 

#define IBasicVideo2_put_DestinationTop(This,DestinationTop)	\
    ( (This)->lpVtbl -> put_DestinationTop(This,DestinationTop) ) 

#define IBasicVideo2_get_DestinationTop(This,pDestinationTop)	\
    ( (This)->lpVtbl -> get_DestinationTop(This,pDestinationTop) ) 

#define IBasicVideo2_put_DestinationHeight(This,DestinationHeight)	\
    ( (This)->lpVtbl -> put_DestinationHeight(This,DestinationHeight) ) 

#define IBasicVideo2_get_DestinationHeight(This,pDestinationHeight)	\
    ( (This)->lpVtbl -> get_DestinationHeight(This,pDestinationHeight) ) 

#define IBasicVideo2_SetSourcePosition(This,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> SetSourcePosition(This,Left,Top,Width,Height) ) 

#define IBasicVideo2_GetSourcePosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IBasicVideo2_SetDefaultSourcePosition(This)	\
    ( (This)->lpVtbl -> SetDefaultSourcePosition(This) ) 

#define IBasicVideo2_SetDestinationPosition(This,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> SetDestinationPosition(This,Left,Top,Width,Height) ) 

#define IBasicVideo2_GetDestinationPosition(This,pLeft,pTop,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetDestinationPosition(This,pLeft,pTop,pWidth,pHeight) ) 

#define IBasicVideo2_SetDefaultDestinationPosition(This)	\
    ( (This)->lpVtbl -> SetDefaultDestinationPosition(This) ) 

#define IBasicVideo2_GetVideoSize(This,pWidth,pHeight)	\
    ( (This)->lpVtbl -> GetVideoSize(This,pWidth,pHeight) ) 

#define IBasicVideo2_GetVideoPaletteEntries(This,StartIndex,Entries,pRetrieved,pPalette)	\
    ( (This)->lpVtbl -> GetVideoPaletteEntries(This,StartIndex,Entries,pRetrieved,pPalette) ) 

#define IBasicVideo2_GetCurrentImage(This,pBufferSize,pDIBImage)	\
    ( (This)->lpVtbl -> GetCurrentImage(This,pBufferSize,pDIBImage) ) 

#define IBasicVideo2_IsUsingDefaultSource(This)	\
    ( (This)->lpVtbl -> IsUsingDefaultSource(This) ) 

#define IBasicVideo2_IsUsingDefaultDestination(This)	\
    ( (This)->lpVtbl -> IsUsingDefaultDestination(This) ) 


#define IBasicVideo2_GetPreferredAspectRatio(This,plAspectX,plAspectY)	\
    ( (This)->lpVtbl -> GetPreferredAspectRatio(This,plAspectX,plAspectY) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBasicVideo2_INTERFACE_DEFINED__ */


#ifndef __IDeferredCommand_INTERFACE_DEFINED__
#define __IDeferredCommand_INTERFACE_DEFINED__

/* interface IDeferredCommand */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_IDeferredCommand,0x56a868b8,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b8-0ad4-11ce-b03a-0020af0ba770")
    IDeferredCommand : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Confidence( 
            /* [out] */ __RPC__out LONG *pConfidence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Postpone( 
            /* [in] */ REFTIME newtime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHResult( 
            /* [out] */ __RPC__out HRESULT *phrResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeferredCommandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeferredCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeferredCommand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeferredCommand * This);
        
        DECLSPEC_XFGVIRT(IDeferredCommand, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IDeferredCommand * This);
        
        DECLSPEC_XFGVIRT(IDeferredCommand, Confidence)
        HRESULT ( STDMETHODCALLTYPE *Confidence )( 
            __RPC__in IDeferredCommand * This,
            /* [out] */ __RPC__out LONG *pConfidence);
        
        DECLSPEC_XFGVIRT(IDeferredCommand, Postpone)
        HRESULT ( STDMETHODCALLTYPE *Postpone )( 
            __RPC__in IDeferredCommand * This,
            /* [in] */ REFTIME newtime);
        
        DECLSPEC_XFGVIRT(IDeferredCommand, GetHResult)
        HRESULT ( STDMETHODCALLTYPE *GetHResult )( 
            __RPC__in IDeferredCommand * This,
            /* [out] */ __RPC__out HRESULT *phrResult);
        
        END_INTERFACE
    } IDeferredCommandVtbl;

    interface IDeferredCommand
    {
        CONST_VTBL struct IDeferredCommandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeferredCommand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeferredCommand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeferredCommand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeferredCommand_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IDeferredCommand_Confidence(This,pConfidence)	\
    ( (This)->lpVtbl -> Confidence(This,pConfidence) ) 

#define IDeferredCommand_Postpone(This,newtime)	\
    ( (This)->lpVtbl -> Postpone(This,newtime) ) 

#define IDeferredCommand_GetHResult(This,phrResult)	\
    ( (This)->lpVtbl -> GetHResult(This,phrResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeferredCommand_INTERFACE_DEFINED__ */


#ifndef __IQueueCommand_INTERFACE_DEFINED__
#define __IQueueCommand_INTERFACE_DEFINED__

/* interface IQueueCommand */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_IQueueCommand,0x56a868b7,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868b7-0ad4-11ce-b03a-0020af0ba770")
    IQueueCommand : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InvokeAtStreamTime( 
            /* [out] */ __RPC__deref_out_opt IDeferredCommand **pCmd,
            /* [in] */ REFTIME time,
            /* [in] */ __RPC__in GUID *iid,
            /* [in] */ long dispidMethod,
            /* [in] */ short wFlags,
            /* [in] */ long cArgs,
            /* [in] */ __RPC__in VARIANT *pDispParams,
            /* [out][in] */ __RPC__inout VARIANT *pvarResult,
            /* [out] */ __RPC__out short *puArgErr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvokeAtPresentationTime( 
            /* [out] */ __RPC__deref_out_opt IDeferredCommand **pCmd,
            /* [in] */ REFTIME time,
            /* [in] */ __RPC__in GUID *iid,
            /* [in] */ long dispidMethod,
            /* [in] */ short wFlags,
            /* [in] */ long cArgs,
            /* [in] */ __RPC__in VARIANT *pDispParams,
            /* [out][in] */ __RPC__inout VARIANT *pvarResult,
            /* [out] */ __RPC__out short *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueueCommandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueueCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueueCommand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueueCommand * This);
        
        DECLSPEC_XFGVIRT(IQueueCommand, InvokeAtStreamTime)
        HRESULT ( STDMETHODCALLTYPE *InvokeAtStreamTime )( 
            __RPC__in IQueueCommand * This,
            /* [out] */ __RPC__deref_out_opt IDeferredCommand **pCmd,
            /* [in] */ REFTIME time,
            /* [in] */ __RPC__in GUID *iid,
            /* [in] */ long dispidMethod,
            /* [in] */ short wFlags,
            /* [in] */ long cArgs,
            /* [in] */ __RPC__in VARIANT *pDispParams,
            /* [out][in] */ __RPC__inout VARIANT *pvarResult,
            /* [out] */ __RPC__out short *puArgErr);
        
        DECLSPEC_XFGVIRT(IQueueCommand, InvokeAtPresentationTime)
        HRESULT ( STDMETHODCALLTYPE *InvokeAtPresentationTime )( 
            __RPC__in IQueueCommand * This,
            /* [out] */ __RPC__deref_out_opt IDeferredCommand **pCmd,
            /* [in] */ REFTIME time,
            /* [in] */ __RPC__in GUID *iid,
            /* [in] */ long dispidMethod,
            /* [in] */ short wFlags,
            /* [in] */ long cArgs,
            /* [in] */ __RPC__in VARIANT *pDispParams,
            /* [out][in] */ __RPC__inout VARIANT *pvarResult,
            /* [out] */ __RPC__out short *puArgErr);
        
        END_INTERFACE
    } IQueueCommandVtbl;

    interface IQueueCommand
    {
        CONST_VTBL struct IQueueCommandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueueCommand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueueCommand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueueCommand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueueCommand_InvokeAtStreamTime(This,pCmd,time,iid,dispidMethod,wFlags,cArgs,pDispParams,pvarResult,puArgErr)	\
    ( (This)->lpVtbl -> InvokeAtStreamTime(This,pCmd,time,iid,dispidMethod,wFlags,cArgs,pDispParams,pvarResult,puArgErr) ) 

#define IQueueCommand_InvokeAtPresentationTime(This,pCmd,time,iid,dispidMethod,wFlags,cArgs,pDispParams,pvarResult,puArgErr)	\
    ( (This)->lpVtbl -> InvokeAtPresentationTime(This,pCmd,time,iid,dispidMethod,wFlags,cArgs,pDispParams,pvarResult,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueueCommand_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_FilgraphManager,0xe436ebb3,0x524f,0x11ce,0x9f,0x53,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#ifdef __cplusplus

class DECLSPEC_UUID("e436ebb3-524f-11ce-9f53-0020af0ba770")
FilgraphManager;
#endif

#ifndef __IFilterInfo_INTERFACE_DEFINED__
#define __IFilterInfo_INTERFACE_DEFINED__

/* interface IFilterInfo */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IFilterInfo,0x56a868ba,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868ba-0ad4-11ce-b03a-0020af0ba770")
    IFilterInfo : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindPin( 
            /* [in] */ __RPC__in BSTR strPinID,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VendorInfo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strVendorInfo) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Filter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Pins( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsFileSource( 
            /* [retval][out] */ __RPC__out LONG *pbIsSource) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Filename( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrFilename) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Filename( 
            /* [in] */ __RPC__in BSTR strFilename) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFilterInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFilterInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFilterInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFilterInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFilterInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFilterInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFilterInfo * This,
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
        
        DECLSPEC_XFGVIRT(IFilterInfo, FindPin)
        HRESULT ( STDMETHODCALLTYPE *FindPin )( 
            __RPC__in IFilterInfo * This,
            /* [in] */ __RPC__in BSTR strPinID,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strName);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_VendorInfo)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VendorInfo )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strVendorInfo);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_Filter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_Pins)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pins )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_IsFileSource)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFileSource )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__out LONG *pbIsSource);
        
        DECLSPEC_XFGVIRT(IFilterInfo, get_Filename)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Filename )( 
            __RPC__in IFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrFilename);
        
        DECLSPEC_XFGVIRT(IFilterInfo, put_Filename)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Filename )( 
            __RPC__in IFilterInfo * This,
            /* [in] */ __RPC__in BSTR strFilename);
        
        END_INTERFACE
    } IFilterInfoVtbl;

    interface IFilterInfo
    {
        CONST_VTBL struct IFilterInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFilterInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFilterInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFilterInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFilterInfo_FindPin(This,strPinID,ppUnk)	\
    ( (This)->lpVtbl -> FindPin(This,strPinID,ppUnk) ) 

#define IFilterInfo_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#define IFilterInfo_get_VendorInfo(This,strVendorInfo)	\
    ( (This)->lpVtbl -> get_VendorInfo(This,strVendorInfo) ) 

#define IFilterInfo_get_Filter(This,ppUnk)	\
    ( (This)->lpVtbl -> get_Filter(This,ppUnk) ) 

#define IFilterInfo_get_Pins(This,ppUnk)	\
    ( (This)->lpVtbl -> get_Pins(This,ppUnk) ) 

#define IFilterInfo_get_IsFileSource(This,pbIsSource)	\
    ( (This)->lpVtbl -> get_IsFileSource(This,pbIsSource) ) 

#define IFilterInfo_get_Filename(This,pstrFilename)	\
    ( (This)->lpVtbl -> get_Filename(This,pstrFilename) ) 

#define IFilterInfo_put_Filename(This,strFilename)	\
    ( (This)->lpVtbl -> put_Filename(This,strFilename) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterInfo_INTERFACE_DEFINED__ */


#ifndef __IRegFilterInfo_INTERFACE_DEFINED__
#define __IRegFilterInfo_INTERFACE_DEFINED__

/* interface IRegFilterInfo */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IRegFilterInfo,0x56a868bb,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868bb-0ad4-11ce-b03a-0020af0ba770")
    IRegFilterInfo : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Filter( 
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegFilterInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRegFilterInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRegFilterInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRegFilterInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRegFilterInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRegFilterInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRegFilterInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRegFilterInfo * This,
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
        
        DECLSPEC_XFGVIRT(IRegFilterInfo, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRegFilterInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strName);
        
        DECLSPEC_XFGVIRT(IRegFilterInfo, Filter)
        HRESULT ( STDMETHODCALLTYPE *Filter )( 
            __RPC__in IRegFilterInfo * This,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        END_INTERFACE
    } IRegFilterInfoVtbl;

    interface IRegFilterInfo
    {
        CONST_VTBL struct IRegFilterInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegFilterInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegFilterInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegFilterInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegFilterInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRegFilterInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRegFilterInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRegFilterInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRegFilterInfo_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#define IRegFilterInfo_Filter(This,ppUnk)	\
    ( (This)->lpVtbl -> Filter(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegFilterInfo_INTERFACE_DEFINED__ */


#ifndef __IMediaTypeInfo_INTERFACE_DEFINED__
#define __IMediaTypeInfo_INTERFACE_DEFINED__

/* interface IMediaTypeInfo */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IMediaTypeInfo,0x56a868bc,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868bc-0ad4-11ce-b03a-0020af0ba770")
    IMediaTypeInfo : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Subtype( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaTypeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaTypeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaTypeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaTypeInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMediaTypeInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMediaTypeInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMediaTypeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMediaTypeInfo * This,
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
        
        DECLSPEC_XFGVIRT(IMediaTypeInfo, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IMediaTypeInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strType);
        
        DECLSPEC_XFGVIRT(IMediaTypeInfo, get_Subtype)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subtype )( 
            __RPC__in IMediaTypeInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strType);
        
        END_INTERFACE
    } IMediaTypeInfoVtbl;

    interface IMediaTypeInfo
    {
        CONST_VTBL struct IMediaTypeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaTypeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaTypeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaTypeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaTypeInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMediaTypeInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMediaTypeInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMediaTypeInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMediaTypeInfo_get_Type(This,strType)	\
    ( (This)->lpVtbl -> get_Type(This,strType) ) 

#define IMediaTypeInfo_get_Subtype(This,strType)	\
    ( (This)->lpVtbl -> get_Subtype(This,strType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaTypeInfo_INTERFACE_DEFINED__ */


#ifndef __IPinInfo_INTERFACE_DEFINED__
#define __IPinInfo_INTERFACE_DEFINED__

/* interface IPinInfo */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IPinInfo,0x56a868bd,0x0ad4,0x11ce,0xb0,0x3a,0x00,0x20,0xaf,0x0b,0xa7,0x70);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868bd-0ad4-11ce-b03a-0020af0ba770")
    IPinInfo : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Pin( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ConnectedTo( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ConnectionMediaType( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilterInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppUnk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out LONG *ppDirection) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PinID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strPinID) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MediaTypes( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ __RPC__in_opt IUnknown *pPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectDirect( 
            /* [in] */ __RPC__in_opt IUnknown *pPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectWithType( 
            /* [in] */ __RPC__in_opt IUnknown *pPin,
            /* [in] */ __RPC__in_opt IDispatch *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Render( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPinInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPinInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPinInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPinInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPinInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPinInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPinInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPinInfo * This,
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
        
        DECLSPEC_XFGVIRT(IPinInfo, get_Pin)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pin )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_ConnectedTo)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectedTo )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_ConnectionMediaType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionMediaType )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_FilterInfo)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterInfo )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_Direction)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__out LONG *ppDirection);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_PinID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinID )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strPinID);
        
        DECLSPEC_XFGVIRT(IPinInfo, get_MediaTypes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypes )( 
            __RPC__in IPinInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnk);
        
        DECLSPEC_XFGVIRT(IPinInfo, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IPinInfo * This,
            /* [in] */ __RPC__in_opt IUnknown *pPin);
        
        DECLSPEC_XFGVIRT(IPinInfo, ConnectDirect)
        HRESULT ( STDMETHODCALLTYPE *ConnectDirect )( 
            __RPC__in IPinInfo * This,
            /* [in] */ __RPC__in_opt IUnknown *pPin);
        
        DECLSPEC_XFGVIRT(IPinInfo, ConnectWithType)
        HRESULT ( STDMETHODCALLTYPE *ConnectWithType )( 
            __RPC__in IPinInfo * This,
            /* [in] */ __RPC__in_opt IUnknown *pPin,
            /* [in] */ __RPC__in_opt IDispatch *pMediaType);
        
        DECLSPEC_XFGVIRT(IPinInfo, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IPinInfo * This);
        
        DECLSPEC_XFGVIRT(IPinInfo, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            __RPC__in IPinInfo * This);
        
        END_INTERFACE
    } IPinInfoVtbl;

    interface IPinInfo
    {
        CONST_VTBL struct IPinInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPinInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPinInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPinInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPinInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPinInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPinInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPinInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPinInfo_get_Pin(This,ppUnk)	\
    ( (This)->lpVtbl -> get_Pin(This,ppUnk) ) 

#define IPinInfo_get_ConnectedTo(This,ppUnk)	\
    ( (This)->lpVtbl -> get_ConnectedTo(This,ppUnk) ) 

#define IPinInfo_get_ConnectionMediaType(This,ppUnk)	\
    ( (This)->lpVtbl -> get_ConnectionMediaType(This,ppUnk) ) 

#define IPinInfo_get_FilterInfo(This,ppUnk)	\
    ( (This)->lpVtbl -> get_FilterInfo(This,ppUnk) ) 

#define IPinInfo_get_Name(This,ppUnk)	\
    ( (This)->lpVtbl -> get_Name(This,ppUnk) ) 

#define IPinInfo_get_Direction(This,ppDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,ppDirection) ) 

#define IPinInfo_get_PinID(This,strPinID)	\
    ( (This)->lpVtbl -> get_PinID(This,strPinID) ) 

#define IPinInfo_get_MediaTypes(This,ppUnk)	\
    ( (This)->lpVtbl -> get_MediaTypes(This,ppUnk) ) 

#define IPinInfo_Connect(This,pPin)	\
    ( (This)->lpVtbl -> Connect(This,pPin) ) 

#define IPinInfo_ConnectDirect(This,pPin)	\
    ( (This)->lpVtbl -> ConnectDirect(This,pPin) ) 

#define IPinInfo_ConnectWithType(This,pPin,pMediaType)	\
    ( (This)->lpVtbl -> ConnectWithType(This,pPin,pMediaType) ) 

#define IPinInfo_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IPinInfo_Render(This)	\
    ( (This)->lpVtbl -> Render(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPinInfo_INTERFACE_DEFINED__ */


#ifndef __IAMStats_INTERFACE_DEFINED__
#define __IAMStats_INTERFACE_DEFINED__

/* interface IAMStats */
/* [object][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IAMStats,0xbc9bcf80,0xdcd2,0x11d2,0xab,0xf6,0x00,0xa0,0xc9,0x05,0xf3,0x75);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc9bcf80-dcd2-11d2-abf6-00a0c905f375")
    IAMStats : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValueByIndex( 
            /* [in] */ long lIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *szName,
            /* [out] */ __RPC__out long *lCount,
            /* [out] */ __RPC__out double *dLast,
            /* [out] */ __RPC__out double *dAverage,
            /* [out] */ __RPC__out double *dStdDev,
            /* [out] */ __RPC__out double *dMin,
            /* [out] */ __RPC__out double *dMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValueByName( 
            /* [in] */ __RPC__in BSTR szName,
            /* [out] */ __RPC__out long *lIndex,
            /* [out] */ __RPC__out long *lCount,
            /* [out] */ __RPC__out double *dLast,
            /* [out] */ __RPC__out double *dAverage,
            /* [out] */ __RPC__out double *dStdDev,
            /* [out] */ __RPC__out double *dMin,
            /* [out] */ __RPC__out double *dMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndex( 
            /* [in] */ __RPC__in BSTR szName,
            /* [in] */ long lCreate,
            /* [out] */ __RPC__out long *plIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddValue( 
            /* [in] */ long lIndex,
            /* [in] */ double dValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMStatsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMStats * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMStats * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMStats * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAMStats * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAMStats * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAMStats * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAMStats * This,
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
        
        DECLSPEC_XFGVIRT(IAMStats, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IAMStats * This);
        
        DECLSPEC_XFGVIRT(IAMStats, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAMStats * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAMStats, GetValueByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetValueByIndex )( 
            __RPC__in IAMStats * This,
            /* [in] */ long lIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *szName,
            /* [out] */ __RPC__out long *lCount,
            /* [out] */ __RPC__out double *dLast,
            /* [out] */ __RPC__out double *dAverage,
            /* [out] */ __RPC__out double *dStdDev,
            /* [out] */ __RPC__out double *dMin,
            /* [out] */ __RPC__out double *dMax);
        
        DECLSPEC_XFGVIRT(IAMStats, GetValueByName)
        HRESULT ( STDMETHODCALLTYPE *GetValueByName )( 
            __RPC__in IAMStats * This,
            /* [in] */ __RPC__in BSTR szName,
            /* [out] */ __RPC__out long *lIndex,
            /* [out] */ __RPC__out long *lCount,
            /* [out] */ __RPC__out double *dLast,
            /* [out] */ __RPC__out double *dAverage,
            /* [out] */ __RPC__out double *dStdDev,
            /* [out] */ __RPC__out double *dMin,
            /* [out] */ __RPC__out double *dMax);
        
        DECLSPEC_XFGVIRT(IAMStats, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IAMStats * This,
            /* [in] */ __RPC__in BSTR szName,
            /* [in] */ long lCreate,
            /* [out] */ __RPC__out long *plIndex);
        
        DECLSPEC_XFGVIRT(IAMStats, AddValue)
        HRESULT ( STDMETHODCALLTYPE *AddValue )( 
            __RPC__in IAMStats * This,
            /* [in] */ long lIndex,
            /* [in] */ double dValue);
        
        END_INTERFACE
    } IAMStatsVtbl;

    interface IAMStats
    {
        CONST_VTBL struct IAMStatsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMStats_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMStats_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMStats_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMStats_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAMStats_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAMStats_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAMStats_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAMStats_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IAMStats_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAMStats_GetValueByIndex(This,lIndex,szName,lCount,dLast,dAverage,dStdDev,dMin,dMax)	\
    ( (This)->lpVtbl -> GetValueByIndex(This,lIndex,szName,lCount,dLast,dAverage,dStdDev,dMin,dMax) ) 

#define IAMStats_GetValueByName(This,szName,lIndex,lCount,dLast,dAverage,dStdDev,dMin,dMax)	\
    ( (This)->lpVtbl -> GetValueByName(This,szName,lIndex,lCount,dLast,dAverage,dStdDev,dMin,dMax) ) 

#define IAMStats_GetIndex(This,szName,lCreate,plIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,szName,lCreate,plIndex) ) 

#define IAMStats_AddValue(This,lIndex,dValue)	\
    ( (This)->lpVtbl -> AddValue(This,lIndex,dValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMStats_INTERFACE_DEFINED__ */

#endif /* __QuartzTypeLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


