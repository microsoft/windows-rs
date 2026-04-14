

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


#ifndef __rtscom_h__
#define __rtscom_h__

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

#ifndef __IRealTimeStylus_FWD_DEFINED__
#define __IRealTimeStylus_FWD_DEFINED__
typedef interface IRealTimeStylus IRealTimeStylus;

#endif 	/* __IRealTimeStylus_FWD_DEFINED__ */


#ifndef __IRealTimeStylus2_FWD_DEFINED__
#define __IRealTimeStylus2_FWD_DEFINED__
typedef interface IRealTimeStylus2 IRealTimeStylus2;

#endif 	/* __IRealTimeStylus2_FWD_DEFINED__ */


#ifndef __IRealTimeStylus3_FWD_DEFINED__
#define __IRealTimeStylus3_FWD_DEFINED__
typedef interface IRealTimeStylus3 IRealTimeStylus3;

#endif 	/* __IRealTimeStylus3_FWD_DEFINED__ */


#ifndef __IRealTimeStylusSynchronization_FWD_DEFINED__
#define __IRealTimeStylusSynchronization_FWD_DEFINED__
typedef interface IRealTimeStylusSynchronization IRealTimeStylusSynchronization;

#endif 	/* __IRealTimeStylusSynchronization_FWD_DEFINED__ */


#ifndef __IStrokeBuilder_FWD_DEFINED__
#define __IStrokeBuilder_FWD_DEFINED__
typedef interface IStrokeBuilder IStrokeBuilder;

#endif 	/* __IStrokeBuilder_FWD_DEFINED__ */


#ifndef __IStylusPlugin_FWD_DEFINED__
#define __IStylusPlugin_FWD_DEFINED__
typedef interface IStylusPlugin IStylusPlugin;

#endif 	/* __IStylusPlugin_FWD_DEFINED__ */


#ifndef __IStylusSyncPlugin_FWD_DEFINED__
#define __IStylusSyncPlugin_FWD_DEFINED__
typedef interface IStylusSyncPlugin IStylusSyncPlugin;

#endif 	/* __IStylusSyncPlugin_FWD_DEFINED__ */


#ifndef __IStylusAsyncPlugin_FWD_DEFINED__
#define __IStylusAsyncPlugin_FWD_DEFINED__
typedef interface IStylusAsyncPlugin IStylusAsyncPlugin;

#endif 	/* __IStylusAsyncPlugin_FWD_DEFINED__ */


#ifndef __IDynamicRenderer_FWD_DEFINED__
#define __IDynamicRenderer_FWD_DEFINED__
typedef interface IDynamicRenderer IDynamicRenderer;

#endif 	/* __IDynamicRenderer_FWD_DEFINED__ */


#ifndef __IGestureRecognizer_FWD_DEFINED__
#define __IGestureRecognizer_FWD_DEFINED__
typedef interface IGestureRecognizer IGestureRecognizer;

#endif 	/* __IGestureRecognizer_FWD_DEFINED__ */


#ifndef __RealTimeStylus_FWD_DEFINED__
#define __RealTimeStylus_FWD_DEFINED__

#ifdef __cplusplus
typedef class RealTimeStylus RealTimeStylus;
#else
typedef struct RealTimeStylus RealTimeStylus;
#endif /* __cplusplus */

#endif 	/* __RealTimeStylus_FWD_DEFINED__ */


#ifndef __DynamicRenderer_FWD_DEFINED__
#define __DynamicRenderer_FWD_DEFINED__

#ifdef __cplusplus
typedef class DynamicRenderer DynamicRenderer;
#else
typedef struct DynamicRenderer DynamicRenderer;
#endif /* __cplusplus */

#endif 	/* __DynamicRenderer_FWD_DEFINED__ */


#ifndef __GestureRecognizer_FWD_DEFINED__
#define __GestureRecognizer_FWD_DEFINED__

#ifdef __cplusplus
typedef class GestureRecognizer GestureRecognizer;
#else
typedef struct GestureRecognizer GestureRecognizer;
#endif /* __cplusplus */

#endif 	/* __GestureRecognizer_FWD_DEFINED__ */


#ifndef __StrokeBuilder_FWD_DEFINED__
#define __StrokeBuilder_FWD_DEFINED__

#ifdef __cplusplus
typedef class StrokeBuilder StrokeBuilder;
#else
typedef struct StrokeBuilder StrokeBuilder;
#endif /* __cplusplus */

#endif 	/* __StrokeBuilder_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "tpcshrd.h"
#include "msinkaut.h"
#include "manipulations.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rtscom_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








typedef 
enum RealTimeStylusDataInterest
    {
        RTSDI_AllData	= 0xffffffff,
        RTSDI_None	= 0,
        RTSDI_Error	= 0x1,
        RTSDI_RealTimeStylusEnabled	= 0x2,
        RTSDI_RealTimeStylusDisabled	= 0x4,
        RTSDI_StylusNew	= 0x8,
        RTSDI_StylusInRange	= 0x10,
        RTSDI_InAirPackets	= 0x20,
        RTSDI_StylusOutOfRange	= 0x40,
        RTSDI_StylusDown	= 0x80,
        RTSDI_Packets	= 0x100,
        RTSDI_StylusUp	= 0x200,
        RTSDI_StylusButtonUp	= 0x400,
        RTSDI_StylusButtonDown	= 0x800,
        RTSDI_SystemEvents	= 0x1000,
        RTSDI_TabletAdded	= 0x2000,
        RTSDI_TabletRemoved	= 0x4000,
        RTSDI_CustomStylusDataAdded	= 0x8000,
        RTSDI_UpdateMapping	= 0x10000,
        RTSDI_DefaultEvents	= ( ( ( ( ( ( RTSDI_RealTimeStylusEnabled | RTSDI_RealTimeStylusDisabled )  | RTSDI_StylusDown )  | RTSDI_Packets )  | RTSDI_StylusUp )  | RTSDI_SystemEvents )  | RTSDI_CustomStylusDataAdded ) 
    } 	RealTimeStylusDataInterest;

typedef CURSOR_ID STYLUS_ID;

typedef struct StylusInfo
    {
    TABLET_CONTEXT_ID tcid;
    STYLUS_ID cid;
    BOOL bIsInvertedCursor;
    } 	StylusInfo;

typedef 
enum StylusQueue
    {
        SyncStylusQueue	= 0x1,
        AsyncStylusQueueImmediate	= 0x2,
        AsyncStylusQueue	= 0x3
    } 	StylusQueue;

typedef 
enum RealTimeStylusLockType
    {
        RTSLT_ObjLock	= 0x1,
        RTSLT_SyncEventLock	= 0x2,
        RTSLT_AsyncEventLock	= 0x4,
        RTSLT_ExcludeCallback	= 0x8,
        RTSLT_SyncObjLock	= 0xb,
        RTSLT_AsyncObjLock	= 0xd
    } 	RealTimeStylusLockType;

typedef struct GESTURE_DATA
    {
    int gestureId;
    int recoConfidence;
    int strokeCount;
    } 	GESTURE_DATA;

typedef struct DYNAMIC_RENDERER_CACHED_DATA
    {
    LONG strokeId;
    IDynamicRenderer *dynamicRenderer;
    } 	DYNAMIC_RENDERER_CACHED_DATA;

#if defined(__cplusplus)
const GUID GUID_GESTURE_DATA = {0x41E4EC0F, 0x26AA, 0x455A, {0x9A, 0xA5, 0x2C, 0xD3, 0x6C, 0xF6, 0x3F, 0xB9}};
const GUID GUID_DYNAMIC_RENDERER_CACHED_DATA = {0xBF531B92, 0x25BF, 0x4A95, {0x89, 0xAD, 0x0E, 0x47, 0x6B, 0x34, 0xB4, 0xF5}};
#endif // of if defined(__cplusplus)


extern RPC_IF_HANDLE __MIDL_itf_rtscom_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rtscom_0000_0000_v0_0_s_ifspec;


#ifndef __TPCRTSLib_LIBRARY_DEFINED__
#define __TPCRTSLib_LIBRARY_DEFINED__

/* library TPCRTSLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_TPCRTSLib;

#ifndef __IRealTimeStylus_INTERFACE_DEFINED__
#define __IRealTimeStylus_INTERFACE_DEFINED__

/* interface IRealTimeStylus */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRealTimeStylus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A8BB5D22-3144-4a7b-93CD-F34A16BE513A")
    IRealTimeStylus : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out BOOL *pfEnable) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ BOOL fEnable) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HWND( 
            /* [retval][out] */ __RPC__out HANDLE_PTR *phwnd) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HWND( 
            /* [in] */ HANDLE_PTR hwnd) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowInputRectangle( 
            /* [retval][out] */ __RPC__out RECT *prcWndInputRect) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WindowInputRectangle( 
            /* [in] */ __RPC__in const RECT *prcWndInputRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStylusSyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [in] */ __RPC__in_opt IStylusSyncPlugin *piPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStylusSyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [out][in] */ __RPC__deref_inout_opt IStylusSyncPlugin **ppiPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllStylusSyncPlugins( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStylusSyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [out] */ __RPC__deref_out_opt IStylusSyncPlugin **ppiPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStylusSyncPluginCount( 
            /* [out] */ __RPC__out ULONG *pcPlugins) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStylusAsyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [in] */ __RPC__in_opt IStylusAsyncPlugin *piPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStylusAsyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [out][in] */ __RPC__deref_inout_opt IStylusAsyncPlugin **ppiPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllStylusAsyncPlugins( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStylusAsyncPlugin( 
            /* [in] */ ULONG iIndex,
            /* [out] */ __RPC__deref_out_opt IStylusAsyncPlugin **ppiPlugin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStylusAsyncPluginCount( 
            /* [out] */ __RPC__out ULONG *pcPlugins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ChildRealTimeStylusPlugin( 
            /* [retval][out] */ __RPC__deref_out_opt IRealTimeStylus **ppiRTS) = 0;
        
        virtual /* [propputref] */ HRESULT STDMETHODCALLTYPE putref_ChildRealTimeStylusPlugin( 
            /* [unique][in] */ __RPC__in_opt IRealTimeStylus *piRTS) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddCustomStylusDataToQueue( 
            /* [in] */ StylusQueue sq,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) BYTE *pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearStylusQueues( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllTabletsMode( 
            /* [in] */ BOOL fUseMouseForInput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSingleTabletMode( 
            /* [in] */ __RPC__in_opt IInkTablet *piTablet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **ppiSingleTablet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTabletContextIdFromTablet( 
            /* [in] */ __RPC__in_opt IInkTablet *piTablet,
            /* [retval][out] */ __RPC__out TABLET_CONTEXT_ID *ptcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTabletFromTabletContextId( 
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **ppiTablet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllTabletContextIds( 
            /* [out][in] */ __RPC__inout ULONG *pcTcidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTcidCount) TABLET_CONTEXT_ID **ppTcids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStyluses( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **ppiInkCursors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStylusForId( 
            /* [in] */ STYLUS_ID sid,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursor **ppiInkCursor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDesiredPacketDescription( 
            /* [range][in] */ __RPC__in_range(0,32) ULONG cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) const GUID *pPropertyGuids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDesiredPacketDescription( 
            /* [out][in] */ __RPC__inout ULONG *pcProperties,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcProperties) GUID **ppPropertyGuids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPacketDescriptionData( 
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [unique][out][in] */ __RPC__inout_opt FLOAT *pfInkToDeviceScaleX,
            /* [unique][out][in] */ __RPC__inout_opt FLOAT *pfInkToDeviceScaleY,
            /* [out][in] */ __RPC__inout ULONG *pcPacketProperties,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcPacketProperties) PACKET_PROPERTY **ppPacketProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRealTimeStylusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRealTimeStylus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRealTimeStylus * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, get_Enabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__out BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, put_Enabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, get_HWND)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HWND )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__out HANDLE_PTR *phwnd);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, put_HWND)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HWND )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ HANDLE_PTR hwnd);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, get_WindowInputRectangle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowInputRectangle )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__out RECT *prcWndInputRect);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, put_WindowInputRectangle)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WindowInputRectangle )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ __RPC__in const RECT *prcWndInputRect);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, AddStylusSyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *AddStylusSyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [in] */ __RPC__in_opt IStylusSyncPlugin *piPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, RemoveStylusSyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *RemoveStylusSyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [out][in] */ __RPC__deref_inout_opt IStylusSyncPlugin **ppiPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, RemoveAllStylusSyncPlugins)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStylusSyncPlugins )( 
            __RPC__in IRealTimeStylus * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStylusSyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *GetStylusSyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [out] */ __RPC__deref_out_opt IStylusSyncPlugin **ppiPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStylusSyncPluginCount)
        HRESULT ( STDMETHODCALLTYPE *GetStylusSyncPluginCount )( 
            __RPC__in IRealTimeStylus * This,
            /* [out] */ __RPC__out ULONG *pcPlugins);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, AddStylusAsyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *AddStylusAsyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [in] */ __RPC__in_opt IStylusAsyncPlugin *piPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, RemoveStylusAsyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *RemoveStylusAsyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [out][in] */ __RPC__deref_inout_opt IStylusAsyncPlugin **ppiPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, RemoveAllStylusAsyncPlugins)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStylusAsyncPlugins )( 
            __RPC__in IRealTimeStylus * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStylusAsyncPlugin)
        HRESULT ( STDMETHODCALLTYPE *GetStylusAsyncPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ ULONG iIndex,
            /* [out] */ __RPC__deref_out_opt IStylusAsyncPlugin **ppiPlugin);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStylusAsyncPluginCount)
        HRESULT ( STDMETHODCALLTYPE *GetStylusAsyncPluginCount )( 
            __RPC__in IRealTimeStylus * This,
            /* [out] */ __RPC__out ULONG *pcPlugins);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, get_ChildRealTimeStylusPlugin)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChildRealTimeStylusPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__deref_out_opt IRealTimeStylus **ppiRTS);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, putref_ChildRealTimeStylusPlugin)
        /* [propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ChildRealTimeStylusPlugin )( 
            __RPC__in IRealTimeStylus * This,
            /* [unique][in] */ __RPC__in_opt IRealTimeStylus *piRTS);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, AddCustomStylusDataToQueue)
        HRESULT ( STDMETHODCALLTYPE *AddCustomStylusDataToQueue )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ StylusQueue sq,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) BYTE *pbData);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, ClearStylusQueues)
        HRESULT ( STDMETHODCALLTYPE *ClearStylusQueues )( 
            __RPC__in IRealTimeStylus * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, SetAllTabletsMode)
        HRESULT ( STDMETHODCALLTYPE *SetAllTabletsMode )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ BOOL fUseMouseForInput);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, SetSingleTabletMode)
        HRESULT ( STDMETHODCALLTYPE *SetSingleTabletMode )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetTablet)
        HRESULT ( STDMETHODCALLTYPE *GetTablet )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **ppiSingleTablet);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetTabletContextIdFromTablet)
        HRESULT ( STDMETHODCALLTYPE *GetTabletContextIdFromTablet )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet,
            /* [retval][out] */ __RPC__out TABLET_CONTEXT_ID *ptcid);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetTabletFromTabletContextId)
        HRESULT ( STDMETHODCALLTYPE *GetTabletFromTabletContextId )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **ppiTablet);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetAllTabletContextIds)
        HRESULT ( STDMETHODCALLTYPE *GetAllTabletContextIds )( 
            __RPC__in IRealTimeStylus * This,
            /* [out][in] */ __RPC__inout ULONG *pcTcidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTcidCount) TABLET_CONTEXT_ID **ppTcids);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStyluses)
        HRESULT ( STDMETHODCALLTYPE *GetStyluses )( 
            __RPC__in IRealTimeStylus * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **ppiInkCursors);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetStylusForId)
        HRESULT ( STDMETHODCALLTYPE *GetStylusForId )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ STYLUS_ID sid,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursor **ppiInkCursor);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, SetDesiredPacketDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDesiredPacketDescription )( 
            __RPC__in IRealTimeStylus * This,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) const GUID *pPropertyGuids);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetDesiredPacketDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDesiredPacketDescription )( 
            __RPC__in IRealTimeStylus * This,
            /* [out][in] */ __RPC__inout ULONG *pcProperties,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcProperties) GUID **ppPropertyGuids);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus, GetPacketDescriptionData)
        HRESULT ( STDMETHODCALLTYPE *GetPacketDescriptionData )( 
            __RPC__in IRealTimeStylus * This,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [unique][out][in] */ __RPC__inout_opt FLOAT *pfInkToDeviceScaleX,
            /* [unique][out][in] */ __RPC__inout_opt FLOAT *pfInkToDeviceScaleY,
            /* [out][in] */ __RPC__inout ULONG *pcPacketProperties,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcPacketProperties) PACKET_PROPERTY **ppPacketProperties);
        
        END_INTERFACE
    } IRealTimeStylusVtbl;

    interface IRealTimeStylus
    {
        CONST_VTBL struct IRealTimeStylusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRealTimeStylus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRealTimeStylus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRealTimeStylus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRealTimeStylus_get_Enabled(This,pfEnable)	\
    ( (This)->lpVtbl -> get_Enabled(This,pfEnable) ) 

#define IRealTimeStylus_put_Enabled(This,fEnable)	\
    ( (This)->lpVtbl -> put_Enabled(This,fEnable) ) 

#define IRealTimeStylus_get_HWND(This,phwnd)	\
    ( (This)->lpVtbl -> get_HWND(This,phwnd) ) 

#define IRealTimeStylus_put_HWND(This,hwnd)	\
    ( (This)->lpVtbl -> put_HWND(This,hwnd) ) 

#define IRealTimeStylus_get_WindowInputRectangle(This,prcWndInputRect)	\
    ( (This)->lpVtbl -> get_WindowInputRectangle(This,prcWndInputRect) ) 

#define IRealTimeStylus_put_WindowInputRectangle(This,prcWndInputRect)	\
    ( (This)->lpVtbl -> put_WindowInputRectangle(This,prcWndInputRect) ) 

#define IRealTimeStylus_AddStylusSyncPlugin(This,iIndex,piPlugin)	\
    ( (This)->lpVtbl -> AddStylusSyncPlugin(This,iIndex,piPlugin) ) 

#define IRealTimeStylus_RemoveStylusSyncPlugin(This,iIndex,ppiPlugin)	\
    ( (This)->lpVtbl -> RemoveStylusSyncPlugin(This,iIndex,ppiPlugin) ) 

#define IRealTimeStylus_RemoveAllStylusSyncPlugins(This)	\
    ( (This)->lpVtbl -> RemoveAllStylusSyncPlugins(This) ) 

#define IRealTimeStylus_GetStylusSyncPlugin(This,iIndex,ppiPlugin)	\
    ( (This)->lpVtbl -> GetStylusSyncPlugin(This,iIndex,ppiPlugin) ) 

#define IRealTimeStylus_GetStylusSyncPluginCount(This,pcPlugins)	\
    ( (This)->lpVtbl -> GetStylusSyncPluginCount(This,pcPlugins) ) 

#define IRealTimeStylus_AddStylusAsyncPlugin(This,iIndex,piPlugin)	\
    ( (This)->lpVtbl -> AddStylusAsyncPlugin(This,iIndex,piPlugin) ) 

#define IRealTimeStylus_RemoveStylusAsyncPlugin(This,iIndex,ppiPlugin)	\
    ( (This)->lpVtbl -> RemoveStylusAsyncPlugin(This,iIndex,ppiPlugin) ) 

#define IRealTimeStylus_RemoveAllStylusAsyncPlugins(This)	\
    ( (This)->lpVtbl -> RemoveAllStylusAsyncPlugins(This) ) 

#define IRealTimeStylus_GetStylusAsyncPlugin(This,iIndex,ppiPlugin)	\
    ( (This)->lpVtbl -> GetStylusAsyncPlugin(This,iIndex,ppiPlugin) ) 

#define IRealTimeStylus_GetStylusAsyncPluginCount(This,pcPlugins)	\
    ( (This)->lpVtbl -> GetStylusAsyncPluginCount(This,pcPlugins) ) 

#define IRealTimeStylus_get_ChildRealTimeStylusPlugin(This,ppiRTS)	\
    ( (This)->lpVtbl -> get_ChildRealTimeStylusPlugin(This,ppiRTS) ) 

#define IRealTimeStylus_putref_ChildRealTimeStylusPlugin(This,piRTS)	\
    ( (This)->lpVtbl -> putref_ChildRealTimeStylusPlugin(This,piRTS) ) 

#define IRealTimeStylus_AddCustomStylusDataToQueue(This,sq,pGuidId,cbData,pbData)	\
    ( (This)->lpVtbl -> AddCustomStylusDataToQueue(This,sq,pGuidId,cbData,pbData) ) 

#define IRealTimeStylus_ClearStylusQueues(This)	\
    ( (This)->lpVtbl -> ClearStylusQueues(This) ) 

#define IRealTimeStylus_SetAllTabletsMode(This,fUseMouseForInput)	\
    ( (This)->lpVtbl -> SetAllTabletsMode(This,fUseMouseForInput) ) 

#define IRealTimeStylus_SetSingleTabletMode(This,piTablet)	\
    ( (This)->lpVtbl -> SetSingleTabletMode(This,piTablet) ) 

#define IRealTimeStylus_GetTablet(This,ppiSingleTablet)	\
    ( (This)->lpVtbl -> GetTablet(This,ppiSingleTablet) ) 

#define IRealTimeStylus_GetTabletContextIdFromTablet(This,piTablet,ptcid)	\
    ( (This)->lpVtbl -> GetTabletContextIdFromTablet(This,piTablet,ptcid) ) 

#define IRealTimeStylus_GetTabletFromTabletContextId(This,tcid,ppiTablet)	\
    ( (This)->lpVtbl -> GetTabletFromTabletContextId(This,tcid,ppiTablet) ) 

#define IRealTimeStylus_GetAllTabletContextIds(This,pcTcidCount,ppTcids)	\
    ( (This)->lpVtbl -> GetAllTabletContextIds(This,pcTcidCount,ppTcids) ) 

#define IRealTimeStylus_GetStyluses(This,ppiInkCursors)	\
    ( (This)->lpVtbl -> GetStyluses(This,ppiInkCursors) ) 

#define IRealTimeStylus_GetStylusForId(This,sid,ppiInkCursor)	\
    ( (This)->lpVtbl -> GetStylusForId(This,sid,ppiInkCursor) ) 

#define IRealTimeStylus_SetDesiredPacketDescription(This,cProperties,pPropertyGuids)	\
    ( (This)->lpVtbl -> SetDesiredPacketDescription(This,cProperties,pPropertyGuids) ) 

#define IRealTimeStylus_GetDesiredPacketDescription(This,pcProperties,ppPropertyGuids)	\
    ( (This)->lpVtbl -> GetDesiredPacketDescription(This,pcProperties,ppPropertyGuids) ) 

#define IRealTimeStylus_GetPacketDescriptionData(This,tcid,pfInkToDeviceScaleX,pfInkToDeviceScaleY,pcPacketProperties,ppPacketProperties)	\
    ( (This)->lpVtbl -> GetPacketDescriptionData(This,tcid,pfInkToDeviceScaleX,pfInkToDeviceScaleY,pcPacketProperties,ppPacketProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRealTimeStylus_INTERFACE_DEFINED__ */


#ifndef __IRealTimeStylus2_INTERFACE_DEFINED__
#define __IRealTimeStylus2_INTERFACE_DEFINED__

/* interface IRealTimeStylus2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRealTimeStylus2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B5F2A6CD-3179-4a3e-B9C4-BB5865962BE2")
    IRealTimeStylus2 : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FlicksEnabled( 
            /* [retval][out] */ __RPC__out BOOL *pfEnable) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FlicksEnabled( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRealTimeStylus2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRealTimeStylus2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRealTimeStylus2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRealTimeStylus2 * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus2, get_FlicksEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FlicksEnabled )( 
            __RPC__in IRealTimeStylus2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus2, put_FlicksEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FlicksEnabled )( 
            __RPC__in IRealTimeStylus2 * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IRealTimeStylus2Vtbl;

    interface IRealTimeStylus2
    {
        CONST_VTBL struct IRealTimeStylus2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRealTimeStylus2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRealTimeStylus2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRealTimeStylus2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRealTimeStylus2_get_FlicksEnabled(This,pfEnable)	\
    ( (This)->lpVtbl -> get_FlicksEnabled(This,pfEnable) ) 

#define IRealTimeStylus2_put_FlicksEnabled(This,fEnable)	\
    ( (This)->lpVtbl -> put_FlicksEnabled(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRealTimeStylus2_INTERFACE_DEFINED__ */


#ifndef __IRealTimeStylus3_INTERFACE_DEFINED__
#define __IRealTimeStylus3_INTERFACE_DEFINED__

/* interface IRealTimeStylus3 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRealTimeStylus3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D70230A3-6986-4051-B57A-1CF69F4D9DB5")
    IRealTimeStylus3 : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MultiTouchEnabled( 
            /* [retval][out] */ __RPC__out BOOL *pfEnable) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MultiTouchEnabled( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRealTimeStylus3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRealTimeStylus3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRealTimeStylus3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRealTimeStylus3 * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus3, get_MultiTouchEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultiTouchEnabled )( 
            __RPC__in IRealTimeStylus3 * This,
            /* [retval][out] */ __RPC__out BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(IRealTimeStylus3, put_MultiTouchEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultiTouchEnabled )( 
            __RPC__in IRealTimeStylus3 * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IRealTimeStylus3Vtbl;

    interface IRealTimeStylus3
    {
        CONST_VTBL struct IRealTimeStylus3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRealTimeStylus3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRealTimeStylus3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRealTimeStylus3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRealTimeStylus3_get_MultiTouchEnabled(This,pfEnable)	\
    ( (This)->lpVtbl -> get_MultiTouchEnabled(This,pfEnable) ) 

#define IRealTimeStylus3_put_MultiTouchEnabled(This,fEnable)	\
    ( (This)->lpVtbl -> put_MultiTouchEnabled(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRealTimeStylus3_INTERFACE_DEFINED__ */


#ifndef __IRealTimeStylusSynchronization_INTERFACE_DEFINED__
#define __IRealTimeStylusSynchronization_INTERFACE_DEFINED__

/* interface IRealTimeStylusSynchronization */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRealTimeStylusSynchronization;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA87EAB8-AB4A-4cea-B5CB-46D84C6A2509")
    IRealTimeStylusSynchronization : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AcquireLock( 
            /* [in] */ RealTimeStylusLockType lock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseLock( 
            /* [in] */ RealTimeStylusLockType lock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRealTimeStylusSynchronizationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRealTimeStylusSynchronization * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRealTimeStylusSynchronization * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRealTimeStylusSynchronization * This);
        
        DECLSPEC_XFGVIRT(IRealTimeStylusSynchronization, AcquireLock)
        HRESULT ( STDMETHODCALLTYPE *AcquireLock )( 
            __RPC__in IRealTimeStylusSynchronization * This,
            /* [in] */ RealTimeStylusLockType lock);
        
        DECLSPEC_XFGVIRT(IRealTimeStylusSynchronization, ReleaseLock)
        HRESULT ( STDMETHODCALLTYPE *ReleaseLock )( 
            __RPC__in IRealTimeStylusSynchronization * This,
            /* [in] */ RealTimeStylusLockType lock);
        
        END_INTERFACE
    } IRealTimeStylusSynchronizationVtbl;

    interface IRealTimeStylusSynchronization
    {
        CONST_VTBL struct IRealTimeStylusSynchronizationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRealTimeStylusSynchronization_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRealTimeStylusSynchronization_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRealTimeStylusSynchronization_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRealTimeStylusSynchronization_AcquireLock(This,lock)	\
    ( (This)->lpVtbl -> AcquireLock(This,lock) ) 

#define IRealTimeStylusSynchronization_ReleaseLock(This,lock)	\
    ( (This)->lpVtbl -> ReleaseLock(This,lock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRealTimeStylusSynchronization_INTERFACE_DEFINED__ */


#ifndef __IStrokeBuilder_INTERFACE_DEFINED__
#define __IStrokeBuilder_INTERFACE_DEFINED__

/* interface IStrokeBuilder */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IStrokeBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A5FD4E2D-C44B-4092-9177-260905EB672B")
    IStrokeBuilder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateStroke( 
            /* [range][in] */ __RPC__in_range(0,0xfffff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) const LONG *pPackets,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPacketProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cPacketProperties) const PACKET_PROPERTY *pPacketProperties,
            /* [in] */ FLOAT fInkToDeviceScaleX,
            /* [in] */ FLOAT fInkToDeviceScaleY,
            /* [out][in] */ __RPC__deref_inout_opt IInkStrokeDisp **ppIInkStroke) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginStroke( 
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const LONG *pPacket,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPacketProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cPacketProperties) PACKET_PROPERTY *pPacketProperties,
            /* [in] */ FLOAT fInkToDeviceScaleX,
            /* [in] */ FLOAT fInkToDeviceScaleY,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IInkStrokeDisp **ppIInkStroke) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AppendPackets( 
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) const LONG *pPackets) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndStroke( 
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [out][in] */ __RPC__deref_inout_opt IInkStrokeDisp **ppIInkStroke,
            /* [out][in] */ __RPC__inout RECT *pDirtyRect) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ppiInkObj) = 0;
        
        virtual /* [propputref] */ HRESULT STDMETHODCALLTYPE putref_Ink( 
            /* [in] */ __RPC__in_opt IInkDisp *piInkObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStrokeBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStrokeBuilder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStrokeBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStrokeBuilder * This);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, CreateStroke)
        HRESULT ( STDMETHODCALLTYPE *CreateStroke )( 
            __RPC__in IStrokeBuilder * This,
            /* [range][in] */ __RPC__in_range(0,0xfffff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) const LONG *pPackets,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPacketProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cPacketProperties) const PACKET_PROPERTY *pPacketProperties,
            /* [in] */ FLOAT fInkToDeviceScaleX,
            /* [in] */ FLOAT fInkToDeviceScaleY,
            /* [out][in] */ __RPC__deref_inout_opt IInkStrokeDisp **ppIInkStroke);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, BeginStroke)
        HRESULT ( STDMETHODCALLTYPE *BeginStroke )( 
            __RPC__in IStrokeBuilder * This,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const LONG *pPacket,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPacketProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cPacketProperties) PACKET_PROPERTY *pPacketProperties,
            /* [in] */ FLOAT fInkToDeviceScaleX,
            /* [in] */ FLOAT fInkToDeviceScaleY,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IInkStrokeDisp **ppIInkStroke);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, AppendPackets)
        HRESULT ( STDMETHODCALLTYPE *AppendPackets )( 
            __RPC__in IStrokeBuilder * This,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) const LONG *pPackets);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, EndStroke)
        HRESULT ( STDMETHODCALLTYPE *EndStroke )( 
            __RPC__in IStrokeBuilder * This,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [out][in] */ __RPC__deref_inout_opt IInkStrokeDisp **ppIInkStroke,
            /* [out][in] */ __RPC__inout RECT *pDirtyRect);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, get_Ink)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IStrokeBuilder * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ppiInkObj);
        
        DECLSPEC_XFGVIRT(IStrokeBuilder, putref_Ink)
        /* [propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Ink )( 
            __RPC__in IStrokeBuilder * This,
            /* [in] */ __RPC__in_opt IInkDisp *piInkObj);
        
        END_INTERFACE
    } IStrokeBuilderVtbl;

    interface IStrokeBuilder
    {
        CONST_VTBL struct IStrokeBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStrokeBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStrokeBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStrokeBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStrokeBuilder_CreateStroke(This,cPktBuffLength,pPackets,cPacketProperties,pPacketProperties,fInkToDeviceScaleX,fInkToDeviceScaleY,ppIInkStroke)	\
    ( (This)->lpVtbl -> CreateStroke(This,cPktBuffLength,pPackets,cPacketProperties,pPacketProperties,fInkToDeviceScaleX,fInkToDeviceScaleY,ppIInkStroke) ) 

#define IStrokeBuilder_BeginStroke(This,tcid,sid,pPacket,cPacketProperties,pPacketProperties,fInkToDeviceScaleX,fInkToDeviceScaleY,ppIInkStroke)	\
    ( (This)->lpVtbl -> BeginStroke(This,tcid,sid,pPacket,cPacketProperties,pPacketProperties,fInkToDeviceScaleX,fInkToDeviceScaleY,ppIInkStroke) ) 

#define IStrokeBuilder_AppendPackets(This,tcid,sid,cPktBuffLength,pPackets)	\
    ( (This)->lpVtbl -> AppendPackets(This,tcid,sid,cPktBuffLength,pPackets) ) 

#define IStrokeBuilder_EndStroke(This,tcid,sid,ppIInkStroke,pDirtyRect)	\
    ( (This)->lpVtbl -> EndStroke(This,tcid,sid,ppIInkStroke,pDirtyRect) ) 

#define IStrokeBuilder_get_Ink(This,ppiInkObj)	\
    ( (This)->lpVtbl -> get_Ink(This,ppiInkObj) ) 

#define IStrokeBuilder_putref_Ink(This,piInkObj)	\
    ( (This)->lpVtbl -> putref_Ink(This,piInkObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStrokeBuilder_INTERFACE_DEFINED__ */


#ifndef __IStylusPlugin_INTERFACE_DEFINED__
#define __IStylusPlugin_INTERFACE_DEFINED__

/* interface IStylusPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IStylusPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A81436D8-4757-4fd1-A185-133F97C6C545")
    IStylusPlugin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RealTimeStylusEnabled( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RealTimeStylusDisabled( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusInRange( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusOutOfRange( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusDown( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusUp( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusButtonDown( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StylusButtonUp( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InAirPackets( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Packets( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CustomStylusDataAdded( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) const BYTE *pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SystemEvent( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ SYSTEM_EVENT event,
            /* [in] */ SYSTEM_EVENT_DATA eventdata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TabletAdded( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TabletRemoved( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ LONG iTabletIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Error( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IStylusPlugin *piPlugin,
            /* [in] */ RealTimeStylusDataInterest dataInterest,
            /* [in] */ HRESULT hrErrorCode,
            /* [out][in] */ __RPC__inout LONG_PTR *lptrKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateMapping( 
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DataInterest( 
            /* [retval][out] */ __RPC__out RealTimeStylusDataInterest *pDataInterest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStylusPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStylusPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStylusPlugin * This);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusEnabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusEnabled )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusDisabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusDisabled )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusInRange)
        HRESULT ( STDMETHODCALLTYPE *StylusInRange )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusOutOfRange)
        HRESULT ( STDMETHODCALLTYPE *StylusOutOfRange )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusDown)
        HRESULT ( STDMETHODCALLTYPE *StylusDown )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusUp)
        HRESULT ( STDMETHODCALLTYPE *StylusUp )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonDown)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonDown )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonUp)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonUp )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, InAirPackets)
        HRESULT ( STDMETHODCALLTYPE *InAirPackets )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Packets)
        HRESULT ( STDMETHODCALLTYPE *Packets )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, CustomStylusDataAdded)
        HRESULT ( STDMETHODCALLTYPE *CustomStylusDataAdded )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) const BYTE *pbData);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, SystemEvent)
        HRESULT ( STDMETHODCALLTYPE *SystemEvent )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ SYSTEM_EVENT event,
            /* [in] */ SYSTEM_EVENT_DATA eventdata);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletAdded)
        HRESULT ( STDMETHODCALLTYPE *TabletAdded )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletRemoved)
        HRESULT ( STDMETHODCALLTYPE *TabletRemoved )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ LONG iTabletIndex);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Error)
        HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IStylusPlugin *piPlugin,
            /* [in] */ RealTimeStylusDataInterest dataInterest,
            /* [in] */ HRESULT hrErrorCode,
            /* [out][in] */ __RPC__inout LONG_PTR *lptrKey);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, UpdateMapping)
        HRESULT ( STDMETHODCALLTYPE *UpdateMapping )( 
            __RPC__in IStylusPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, DataInterest)
        HRESULT ( STDMETHODCALLTYPE *DataInterest )( 
            __RPC__in IStylusPlugin * This,
            /* [retval][out] */ __RPC__out RealTimeStylusDataInterest *pDataInterest);
        
        END_INTERFACE
    } IStylusPluginVtbl;

    interface IStylusPlugin
    {
        CONST_VTBL struct IStylusPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStylusPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStylusPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStylusPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStylusPlugin_RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusPlugin_RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusPlugin_StylusInRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusInRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusPlugin_StylusOutOfRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusOutOfRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusPlugin_StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusPlugin_StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusPlugin_StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusPlugin_StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusPlugin_InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusPlugin_Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusPlugin_CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData)	\
    ( (This)->lpVtbl -> CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData) ) 

#define IStylusPlugin_SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata)	\
    ( (This)->lpVtbl -> SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata) ) 

#define IStylusPlugin_TabletAdded(This,piRtsSrc,piTablet)	\
    ( (This)->lpVtbl -> TabletAdded(This,piRtsSrc,piTablet) ) 

#define IStylusPlugin_TabletRemoved(This,piRtsSrc,iTabletIndex)	\
    ( (This)->lpVtbl -> TabletRemoved(This,piRtsSrc,iTabletIndex) ) 

#define IStylusPlugin_Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey)	\
    ( (This)->lpVtbl -> Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey) ) 

#define IStylusPlugin_UpdateMapping(This,piRtsSrc)	\
    ( (This)->lpVtbl -> UpdateMapping(This,piRtsSrc) ) 

#define IStylusPlugin_DataInterest(This,pDataInterest)	\
    ( (This)->lpVtbl -> DataInterest(This,pDataInterest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStylusPlugin_INTERFACE_DEFINED__ */


#ifndef __IStylusSyncPlugin_INTERFACE_DEFINED__
#define __IStylusSyncPlugin_INTERFACE_DEFINED__

/* interface IStylusSyncPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IStylusSyncPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A157B174-482F-4d71-A3F6-3A41DDD11BE9")
    IStylusSyncPlugin : public IStylusPlugin
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IStylusSyncPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStylusSyncPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStylusSyncPlugin * This);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusEnabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusEnabled )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusDisabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusDisabled )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusInRange)
        HRESULT ( STDMETHODCALLTYPE *StylusInRange )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusOutOfRange)
        HRESULT ( STDMETHODCALLTYPE *StylusOutOfRange )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusDown)
        HRESULT ( STDMETHODCALLTYPE *StylusDown )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusUp)
        HRESULT ( STDMETHODCALLTYPE *StylusUp )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonDown)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonDown )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonUp)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonUp )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, InAirPackets)
        HRESULT ( STDMETHODCALLTYPE *InAirPackets )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Packets)
        HRESULT ( STDMETHODCALLTYPE *Packets )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, CustomStylusDataAdded)
        HRESULT ( STDMETHODCALLTYPE *CustomStylusDataAdded )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) const BYTE *pbData);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, SystemEvent)
        HRESULT ( STDMETHODCALLTYPE *SystemEvent )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ SYSTEM_EVENT event,
            /* [in] */ SYSTEM_EVENT_DATA eventdata);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletAdded)
        HRESULT ( STDMETHODCALLTYPE *TabletAdded )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletRemoved)
        HRESULT ( STDMETHODCALLTYPE *TabletRemoved )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ LONG iTabletIndex);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Error)
        HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IStylusPlugin *piPlugin,
            /* [in] */ RealTimeStylusDataInterest dataInterest,
            /* [in] */ HRESULT hrErrorCode,
            /* [out][in] */ __RPC__inout LONG_PTR *lptrKey);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, UpdateMapping)
        HRESULT ( STDMETHODCALLTYPE *UpdateMapping )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, DataInterest)
        HRESULT ( STDMETHODCALLTYPE *DataInterest )( 
            __RPC__in IStylusSyncPlugin * This,
            /* [retval][out] */ __RPC__out RealTimeStylusDataInterest *pDataInterest);
        
        END_INTERFACE
    } IStylusSyncPluginVtbl;

    interface IStylusSyncPlugin
    {
        CONST_VTBL struct IStylusSyncPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStylusSyncPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStylusSyncPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStylusSyncPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStylusSyncPlugin_RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusSyncPlugin_RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusSyncPlugin_StylusInRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusInRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusSyncPlugin_StylusOutOfRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusOutOfRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusSyncPlugin_StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusSyncPlugin_StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusSyncPlugin_StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusSyncPlugin_StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusSyncPlugin_InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusSyncPlugin_Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusSyncPlugin_CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData)	\
    ( (This)->lpVtbl -> CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData) ) 

#define IStylusSyncPlugin_SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata)	\
    ( (This)->lpVtbl -> SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata) ) 

#define IStylusSyncPlugin_TabletAdded(This,piRtsSrc,piTablet)	\
    ( (This)->lpVtbl -> TabletAdded(This,piRtsSrc,piTablet) ) 

#define IStylusSyncPlugin_TabletRemoved(This,piRtsSrc,iTabletIndex)	\
    ( (This)->lpVtbl -> TabletRemoved(This,piRtsSrc,iTabletIndex) ) 

#define IStylusSyncPlugin_Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey)	\
    ( (This)->lpVtbl -> Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey) ) 

#define IStylusSyncPlugin_UpdateMapping(This,piRtsSrc)	\
    ( (This)->lpVtbl -> UpdateMapping(This,piRtsSrc) ) 

#define IStylusSyncPlugin_DataInterest(This,pDataInterest)	\
    ( (This)->lpVtbl -> DataInterest(This,pDataInterest) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStylusSyncPlugin_INTERFACE_DEFINED__ */


#ifndef __IStylusAsyncPlugin_INTERFACE_DEFINED__
#define __IStylusAsyncPlugin_INTERFACE_DEFINED__

/* interface IStylusAsyncPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IStylusAsyncPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A7CCA85A-31BC-4cd2-AADC-3289A3AF11C8")
    IStylusAsyncPlugin : public IStylusPlugin
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IStylusAsyncPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStylusAsyncPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStylusAsyncPlugin * This);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusEnabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusEnabled )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, RealTimeStylusDisabled)
        HRESULT ( STDMETHODCALLTYPE *RealTimeStylusDisabled )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [range][in] */ __RPC__in_range(0,8) ULONG cTcidCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cTcidCount) const TABLET_CONTEXT_ID *pTcids);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusInRange)
        HRESULT ( STDMETHODCALLTYPE *StylusInRange )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusOutOfRange)
        HRESULT ( STDMETHODCALLTYPE *StylusOutOfRange )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusDown)
        HRESULT ( STDMETHODCALLTYPE *StylusDown )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusUp)
        HRESULT ( STDMETHODCALLTYPE *StylusUp )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [range][in] */ __RPC__in_range(0,32) ULONG cPropCountPerPkt,
            /* [size_is][in] */ __RPC__in_ecount_full(cPropCountPerPkt) LONG *pPacket,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkt);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonDown)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonDown )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, StylusButtonUp)
        HRESULT ( STDMETHODCALLTYPE *StylusButtonUp )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ __RPC__in const GUID *pGuidStylusButton,
            /* [out][in] */ __RPC__inout POINT *pStylusPos);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, InAirPackets)
        HRESULT ( STDMETHODCALLTYPE *InAirPackets )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Packets)
        HRESULT ( STDMETHODCALLTYPE *Packets )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const StylusInfo *pStylusInfo,
            /* [in] */ ULONG cPktCount,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cPktBuffLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cPktBuffLength) LONG *pPackets,
            /* [out][in] */ __RPC__inout ULONG *pcInOutPkts,
            /* [out][in] */ __RPC__deref_inout_opt LONG **ppInOutPkts);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, CustomStylusDataAdded)
        HRESULT ( STDMETHODCALLTYPE *CustomStylusDataAdded )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in const GUID *pGuidId,
            /* [range][in] */ __RPC__in_range(0,0x7fff) ULONG cbData,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbData) const BYTE *pbData);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, SystemEvent)
        HRESULT ( STDMETHODCALLTYPE *SystemEvent )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ TABLET_CONTEXT_ID tcid,
            /* [in] */ STYLUS_ID sid,
            /* [in] */ SYSTEM_EVENT event,
            /* [in] */ SYSTEM_EVENT_DATA eventdata);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletAdded)
        HRESULT ( STDMETHODCALLTYPE *TabletAdded )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IInkTablet *piTablet);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, TabletRemoved)
        HRESULT ( STDMETHODCALLTYPE *TabletRemoved )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ LONG iTabletIndex);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, Error)
        HRESULT ( STDMETHODCALLTYPE *Error )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc,
            /* [in] */ __RPC__in_opt IStylusPlugin *piPlugin,
            /* [in] */ RealTimeStylusDataInterest dataInterest,
            /* [in] */ HRESULT hrErrorCode,
            /* [out][in] */ __RPC__inout LONG_PTR *lptrKey);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, UpdateMapping)
        HRESULT ( STDMETHODCALLTYPE *UpdateMapping )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [in] */ __RPC__in_opt IRealTimeStylus *piRtsSrc);
        
        DECLSPEC_XFGVIRT(IStylusPlugin, DataInterest)
        HRESULT ( STDMETHODCALLTYPE *DataInterest )( 
            __RPC__in IStylusAsyncPlugin * This,
            /* [retval][out] */ __RPC__out RealTimeStylusDataInterest *pDataInterest);
        
        END_INTERFACE
    } IStylusAsyncPluginVtbl;

    interface IStylusAsyncPlugin
    {
        CONST_VTBL struct IStylusAsyncPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStylusAsyncPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStylusAsyncPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStylusAsyncPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStylusAsyncPlugin_RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusEnabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusAsyncPlugin_RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids)	\
    ( (This)->lpVtbl -> RealTimeStylusDisabled(This,piRtsSrc,cTcidCount,pTcids) ) 

#define IStylusAsyncPlugin_StylusInRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusInRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusAsyncPlugin_StylusOutOfRange(This,piRtsSrc,tcid,sid)	\
    ( (This)->lpVtbl -> StylusOutOfRange(This,piRtsSrc,tcid,sid) ) 

#define IStylusAsyncPlugin_StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusDown(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusAsyncPlugin_StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt)	\
    ( (This)->lpVtbl -> StylusUp(This,piRtsSrc,pStylusInfo,cPropCountPerPkt,pPacket,ppInOutPkt) ) 

#define IStylusAsyncPlugin_StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonDown(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusAsyncPlugin_StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos)	\
    ( (This)->lpVtbl -> StylusButtonUp(This,piRtsSrc,sid,pGuidStylusButton,pStylusPos) ) 

#define IStylusAsyncPlugin_InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> InAirPackets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusAsyncPlugin_Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts)	\
    ( (This)->lpVtbl -> Packets(This,piRtsSrc,pStylusInfo,cPktCount,cPktBuffLength,pPackets,pcInOutPkts,ppInOutPkts) ) 

#define IStylusAsyncPlugin_CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData)	\
    ( (This)->lpVtbl -> CustomStylusDataAdded(This,piRtsSrc,pGuidId,cbData,pbData) ) 

#define IStylusAsyncPlugin_SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata)	\
    ( (This)->lpVtbl -> SystemEvent(This,piRtsSrc,tcid,sid,event,eventdata) ) 

#define IStylusAsyncPlugin_TabletAdded(This,piRtsSrc,piTablet)	\
    ( (This)->lpVtbl -> TabletAdded(This,piRtsSrc,piTablet) ) 

#define IStylusAsyncPlugin_TabletRemoved(This,piRtsSrc,iTabletIndex)	\
    ( (This)->lpVtbl -> TabletRemoved(This,piRtsSrc,iTabletIndex) ) 

#define IStylusAsyncPlugin_Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey)	\
    ( (This)->lpVtbl -> Error(This,piRtsSrc,piPlugin,dataInterest,hrErrorCode,lptrKey) ) 

#define IStylusAsyncPlugin_UpdateMapping(This,piRtsSrc)	\
    ( (This)->lpVtbl -> UpdateMapping(This,piRtsSrc) ) 

#define IStylusAsyncPlugin_DataInterest(This,pDataInterest)	\
    ( (This)->lpVtbl -> DataInterest(This,pDataInterest) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStylusAsyncPlugin_INTERFACE_DEFINED__ */


#ifndef __IDynamicRenderer_INTERFACE_DEFINED__
#define __IDynamicRenderer_INTERFACE_DEFINED__

/* interface IDynamicRenderer */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDynamicRenderer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A079468E-7165-46f9-B7AF-98AD01A93009")
    IDynamicRenderer : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out BOOL *bEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ BOOL bEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HWND( 
            /* [retval][out] */ __RPC__out HANDLE_PTR *hwnd) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HWND( 
            /* [in] */ HANDLE_PTR hwnd) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClipRectangle( 
            /* [retval][out] */ __RPC__out RECT *prcClipRect) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClipRectangle( 
            /* [in] */ __RPC__in const RECT *prcClipRect) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClipRegion( 
            /* [retval][out] */ __RPC__out HANDLE_PTR *phClipRgn) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClipRegion( 
            /* [in] */ HANDLE_PTR hClipRgn) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **ppiDA) = 0;
        
        virtual /* [propputref] */ HRESULT STDMETHODCALLTYPE putref_DrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *piDA) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataCacheEnabled( 
            /* [retval][out] */ __RPC__out BOOL *pfCacheData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DataCacheEnabled( 
            /* [in] */ BOOL fCacheData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseCachedData( 
            ULONG strokeId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Draw( 
            /* [in] */ HANDLE_PTR hDC) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDynamicRendererVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDynamicRenderer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDynamicRenderer * This);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_Enabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__out BOOL *bEnabled);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, put_Enabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ BOOL bEnabled);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_HWND)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HWND )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__out HANDLE_PTR *hwnd);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, put_HWND)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HWND )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ HANDLE_PTR hwnd);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_ClipRectangle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClipRectangle )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__out RECT *prcClipRect);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, put_ClipRectangle)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClipRectangle )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ __RPC__in const RECT *prcClipRect);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_ClipRegion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClipRegion )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__out HANDLE_PTR *phClipRgn);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, put_ClipRegion)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClipRegion )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ HANDLE_PTR hClipRgn);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_DrawingAttributes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DrawingAttributes )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **ppiDA);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, putref_DrawingAttributes)
        /* [propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DrawingAttributes )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *piDA);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, get_DataCacheEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCacheEnabled )( 
            __RPC__in IDynamicRenderer * This,
            /* [retval][out] */ __RPC__out BOOL *pfCacheData);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, put_DataCacheEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCacheEnabled )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ BOOL fCacheData);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, ReleaseCachedData)
        HRESULT ( STDMETHODCALLTYPE *ReleaseCachedData )( 
            __RPC__in IDynamicRenderer * This,
            ULONG strokeId);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, Refresh)
        HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IDynamicRenderer * This);
        
        DECLSPEC_XFGVIRT(IDynamicRenderer, Draw)
        HRESULT ( STDMETHODCALLTYPE *Draw )( 
            __RPC__in IDynamicRenderer * This,
            /* [in] */ HANDLE_PTR hDC);
        
        END_INTERFACE
    } IDynamicRendererVtbl;

    interface IDynamicRenderer
    {
        CONST_VTBL struct IDynamicRendererVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDynamicRenderer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDynamicRenderer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDynamicRenderer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDynamicRenderer_get_Enabled(This,bEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,bEnabled) ) 

#define IDynamicRenderer_put_Enabled(This,bEnabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,bEnabled) ) 

#define IDynamicRenderer_get_HWND(This,hwnd)	\
    ( (This)->lpVtbl -> get_HWND(This,hwnd) ) 

#define IDynamicRenderer_put_HWND(This,hwnd)	\
    ( (This)->lpVtbl -> put_HWND(This,hwnd) ) 

#define IDynamicRenderer_get_ClipRectangle(This,prcClipRect)	\
    ( (This)->lpVtbl -> get_ClipRectangle(This,prcClipRect) ) 

#define IDynamicRenderer_put_ClipRectangle(This,prcClipRect)	\
    ( (This)->lpVtbl -> put_ClipRectangle(This,prcClipRect) ) 

#define IDynamicRenderer_get_ClipRegion(This,phClipRgn)	\
    ( (This)->lpVtbl -> get_ClipRegion(This,phClipRgn) ) 

#define IDynamicRenderer_put_ClipRegion(This,hClipRgn)	\
    ( (This)->lpVtbl -> put_ClipRegion(This,hClipRgn) ) 

#define IDynamicRenderer_get_DrawingAttributes(This,ppiDA)	\
    ( (This)->lpVtbl -> get_DrawingAttributes(This,ppiDA) ) 

#define IDynamicRenderer_putref_DrawingAttributes(This,piDA)	\
    ( (This)->lpVtbl -> putref_DrawingAttributes(This,piDA) ) 

#define IDynamicRenderer_get_DataCacheEnabled(This,pfCacheData)	\
    ( (This)->lpVtbl -> get_DataCacheEnabled(This,pfCacheData) ) 

#define IDynamicRenderer_put_DataCacheEnabled(This,fCacheData)	\
    ( (This)->lpVtbl -> put_DataCacheEnabled(This,fCacheData) ) 

#define IDynamicRenderer_ReleaseCachedData(This,strokeId)	\
    ( (This)->lpVtbl -> ReleaseCachedData(This,strokeId) ) 

#define IDynamicRenderer_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IDynamicRenderer_Draw(This,hDC)	\
    ( (This)->lpVtbl -> Draw(This,hDC) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDynamicRenderer_INTERFACE_DEFINED__ */


#ifndef __IGestureRecognizer_INTERFACE_DEFINED__
#define __IGestureRecognizer_INTERFACE_DEFINED__

/* interface IGestureRecognizer */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGestureRecognizer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE9EF86B-7054-45e3-AE22-3174DC8811B7")
    IGestureRecognizer : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out BOOL *pfEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ BOOL fEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaxStrokeCount( 
            /* [retval][out] */ __RPC__out long *pcStrokes) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaxStrokeCount( 
            /* [in] */ long cStrokes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableGestures( 
            /* [range][in] */ __RPC__in_range(0,64) ULONG cGestures,
            /* [size_is][in] */ __RPC__in_ecount_full(cGestures) const int *pGestures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGestureRecognizerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGestureRecognizer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGestureRecognizer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGestureRecognizer * This);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, get_Enabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IGestureRecognizer * This,
            /* [retval][out] */ __RPC__out BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, put_Enabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IGestureRecognizer * This,
            /* [in] */ BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, get_MaxStrokeCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxStrokeCount )( 
            __RPC__in IGestureRecognizer * This,
            /* [retval][out] */ __RPC__out long *pcStrokes);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, put_MaxStrokeCount)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxStrokeCount )( 
            __RPC__in IGestureRecognizer * This,
            /* [in] */ long cStrokes);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, EnableGestures)
        HRESULT ( STDMETHODCALLTYPE *EnableGestures )( 
            __RPC__in IGestureRecognizer * This,
            /* [range][in] */ __RPC__in_range(0,64) ULONG cGestures,
            /* [size_is][in] */ __RPC__in_ecount_full(cGestures) const int *pGestures);
        
        DECLSPEC_XFGVIRT(IGestureRecognizer, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IGestureRecognizer * This);
        
        END_INTERFACE
    } IGestureRecognizerVtbl;

    interface IGestureRecognizer
    {
        CONST_VTBL struct IGestureRecognizerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGestureRecognizer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGestureRecognizer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGestureRecognizer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGestureRecognizer_get_Enabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pfEnabled) ) 

#define IGestureRecognizer_put_Enabled(This,fEnabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,fEnabled) ) 

#define IGestureRecognizer_get_MaxStrokeCount(This,pcStrokes)	\
    ( (This)->lpVtbl -> get_MaxStrokeCount(This,pcStrokes) ) 

#define IGestureRecognizer_put_MaxStrokeCount(This,cStrokes)	\
    ( (This)->lpVtbl -> put_MaxStrokeCount(This,cStrokes) ) 

#define IGestureRecognizer_EnableGestures(This,cGestures,pGestures)	\
    ( (This)->lpVtbl -> EnableGestures(This,cGestures,pGestures) ) 

#define IGestureRecognizer_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGestureRecognizer_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_RealTimeStylus;

#ifdef __cplusplus

class DECLSPEC_UUID("E26B366D-F998-43ce-836F-CB6D904432B0")
RealTimeStylus;
#endif

EXTERN_C const CLSID CLSID_DynamicRenderer;

#ifdef __cplusplus

class DECLSPEC_UUID("ECD32AEA-746F-4dcb-BF68-082757FAFF18")
DynamicRenderer;
#endif

EXTERN_C const CLSID CLSID_GestureRecognizer;

#ifdef __cplusplus

class DECLSPEC_UUID("EA30C654-C62C-441f-AC00-95F9A196782C")
GestureRecognizer;
#endif

EXTERN_C const CLSID CLSID_StrokeBuilder;

#ifdef __cplusplus

class DECLSPEC_UUID("E810CEE7-6E51-4cb0-AA3A-0B985B70DAF7")
StrokeBuilder;
#endif
#endif /* __TPCRTSLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_rtscom_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_rtscom_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rtscom_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


