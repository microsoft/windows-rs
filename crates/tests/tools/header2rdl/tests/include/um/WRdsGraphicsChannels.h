

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

#ifndef __wrdsgraphicschannels_h__
#define __wrdsgraphicschannels_h__

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

#ifndef __IWRdsGraphicsChannelEvents_FWD_DEFINED__
#define __IWRdsGraphicsChannelEvents_FWD_DEFINED__
typedef interface IWRdsGraphicsChannelEvents IWRdsGraphicsChannelEvents;

#endif 	/* __IWRdsGraphicsChannelEvents_FWD_DEFINED__ */


#ifndef __IWRdsGraphicsChannel_FWD_DEFINED__
#define __IWRdsGraphicsChannel_FWD_DEFINED__
typedef interface IWRdsGraphicsChannel IWRdsGraphicsChannel;

#endif 	/* __IWRdsGraphicsChannel_FWD_DEFINED__ */


#ifndef __IWRdsGraphicsChannelManager_FWD_DEFINED__
#define __IWRdsGraphicsChannelManager_FWD_DEFINED__
typedef interface IWRdsGraphicsChannelManager IWRdsGraphicsChannelManager;

#endif 	/* __IWRdsGraphicsChannelManager_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wrdsgraphicschannels_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define	WRdsGraphicsChannels_Bandwidth_Unavailable	( ( ULONG  )-1 )

#define	WRdsGraphicsChannels_LossyChannelMaxMessageSize	( 988 )

#define	WRdsGraphicsChannels_RTT_Unavailable	( ( ULONG  )-1 )

EXTERN_C __declspec(selectany) const IID IID_IWRdsGraphicsChannelEvents = {0x67f2368c, 0xd674, 0x4fae, {0x66, 0xa5, 0xd2, 0x06, 0x28, 0xa6, 0x40, 0xd2}};


extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0000_v0_0_s_ifspec;

#ifndef __IWRdsGraphicsChannelEvents_INTERFACE_DEFINED__
#define __IWRdsGraphicsChannelEvents_INTERFACE_DEFINED__

/* interface IWRdsGraphicsChannelEvents */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsGraphicsChannelEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("67f2368c-d674-4fae-66a5-d20628a640d2")
    IWRdsGraphicsChannelEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDataReceived( 
            /* [in] */ ULONG cbSize,
            /* [in] */ __RPC__in BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClose( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChannelOpened( 
            /* [in] */ HRESULT OpenResult,
            /* [in] */ __RPC__in_opt IUnknown *pOpenContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDataSent( 
            /* [in] */ __RPC__in_opt IUnknown *pWriteContext,
            /* [in] */ BOOL bCancelled,
            /* [in] */ __RPC__in BYTE *pBuffer,
            /* [in] */ ULONG cbBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMetricsUpdate( 
            /* [in] */ DWORD bandwidth,
            /* [in] */ DWORD RTT,
            /* [in] */ UINT64 lastSentByteIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsGraphicsChannelEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsGraphicsChannelEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsGraphicsChannelEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsGraphicsChannelEvents * This);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelEvents, OnDataReceived)
        HRESULT ( STDMETHODCALLTYPE *OnDataReceived )( 
            __RPC__in IWRdsGraphicsChannelEvents * This,
            /* [in] */ ULONG cbSize,
            /* [in] */ __RPC__in BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelEvents, OnClose)
        HRESULT ( STDMETHODCALLTYPE *OnClose )( 
            __RPC__in IWRdsGraphicsChannelEvents * This);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelEvents, OnChannelOpened)
        HRESULT ( STDMETHODCALLTYPE *OnChannelOpened )( 
            __RPC__in IWRdsGraphicsChannelEvents * This,
            /* [in] */ HRESULT OpenResult,
            /* [in] */ __RPC__in_opt IUnknown *pOpenContext);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelEvents, OnDataSent)
        HRESULT ( STDMETHODCALLTYPE *OnDataSent )( 
            __RPC__in IWRdsGraphicsChannelEvents * This,
            /* [in] */ __RPC__in_opt IUnknown *pWriteContext,
            /* [in] */ BOOL bCancelled,
            /* [in] */ __RPC__in BYTE *pBuffer,
            /* [in] */ ULONG cbBuffer);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelEvents, OnMetricsUpdate)
        HRESULT ( STDMETHODCALLTYPE *OnMetricsUpdate )( 
            __RPC__in IWRdsGraphicsChannelEvents * This,
            /* [in] */ DWORD bandwidth,
            /* [in] */ DWORD RTT,
            /* [in] */ UINT64 lastSentByteIndex);
        
        END_INTERFACE
    } IWRdsGraphicsChannelEventsVtbl;

    interface IWRdsGraphicsChannelEvents
    {
        CONST_VTBL struct IWRdsGraphicsChannelEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsGraphicsChannelEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsGraphicsChannelEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsGraphicsChannelEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsGraphicsChannelEvents_OnDataReceived(This,cbSize,pBuffer)	\
    ( (This)->lpVtbl -> OnDataReceived(This,cbSize,pBuffer) ) 

#define IWRdsGraphicsChannelEvents_OnClose(This)	\
    ( (This)->lpVtbl -> OnClose(This) ) 

#define IWRdsGraphicsChannelEvents_OnChannelOpened(This,OpenResult,pOpenContext)	\
    ( (This)->lpVtbl -> OnChannelOpened(This,OpenResult,pOpenContext) ) 

#define IWRdsGraphicsChannelEvents_OnDataSent(This,pWriteContext,bCancelled,pBuffer,cbBuffer)	\
    ( (This)->lpVtbl -> OnDataSent(This,pWriteContext,bCancelled,pBuffer,cbBuffer) ) 

#define IWRdsGraphicsChannelEvents_OnMetricsUpdate(This,bandwidth,RTT,lastSentByteIndex)	\
    ( (This)->lpVtbl -> OnMetricsUpdate(This,bandwidth,RTT,lastSentByteIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsGraphicsChannelEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wrdsgraphicschannels_0000_0001 */
/* [local] */ 

EXTERN_C __declspec(selectany) const IID IID_IWRdsGraphicsChannel = {0x684b7a0b, 0xedff, 0x43ad, {0xd5, 0xa2, 0x4a, 0x8d, 0x53, 0x88, 0xf4, 0x01}};


extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0001_v0_0_s_ifspec;

#ifndef __IWRdsGraphicsChannel_INTERFACE_DEFINED__
#define __IWRdsGraphicsChannel_INTERFACE_DEFINED__

/* interface IWRdsGraphicsChannel */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsGraphicsChannel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("684b7a0b-edff-43ad-d5a2-4a8d5388f401")
    IWRdsGraphicsChannel : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ ULONG cbSize,
            /* [in] */ __RPC__in BYTE *pBuffer,
            /* [in] */ __RPC__in_opt IUnknown *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in_opt IWRdsGraphicsChannelEvents *pChannelEvents,
            /* [in] */ __RPC__in_opt IUnknown *pOpenContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsGraphicsChannelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsGraphicsChannel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsGraphicsChannel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsGraphicsChannel * This);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannel, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IWRdsGraphicsChannel * This,
            /* [in] */ ULONG cbSize,
            /* [in] */ __RPC__in BYTE *pBuffer,
            /* [in] */ __RPC__in_opt IUnknown *pContext);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannel, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IWRdsGraphicsChannel * This);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannel, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IWRdsGraphicsChannel * This,
            /* [in] */ __RPC__in_opt IWRdsGraphicsChannelEvents *pChannelEvents,
            /* [in] */ __RPC__in_opt IUnknown *pOpenContext);
        
        END_INTERFACE
    } IWRdsGraphicsChannelVtbl;

    interface IWRdsGraphicsChannel
    {
        CONST_VTBL struct IWRdsGraphicsChannelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsGraphicsChannel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsGraphicsChannel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsGraphicsChannel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsGraphicsChannel_Write(This,cbSize,pBuffer,pContext)	\
    ( (This)->lpVtbl -> Write(This,cbSize,pBuffer,pContext) ) 

#define IWRdsGraphicsChannel_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IWRdsGraphicsChannel_Open(This,pChannelEvents,pOpenContext)	\
    ( (This)->lpVtbl -> Open(This,pChannelEvents,pOpenContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsGraphicsChannel_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wrdsgraphicschannels_0000_0002 */
/* [local] */ 

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wrdsgraphicschannels_0000_0002_0001
    {
        WRdsGraphicsChannelType_GuaranteedDelivery	= 0,
        WRdsGraphicsChannelType_BestEffortDelivery	= 1
    } 	WRdsGraphicsChannelType;

EXTERN_C __declspec(selectany) const IID IID_IWRdsGraphicsChannelManager = {0x0fd57159, 0xe83e, 0x476a, {0xa8, 0xb9, 0x4a, 0x79, 0x76, 0xe7, 0x1e, 0x18}};


extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0002_v0_0_s_ifspec;

#ifndef __IWRdsGraphicsChannelManager_INTERFACE_DEFINED__
#define __IWRdsGraphicsChannelManager_INTERFACE_DEFINED__

/* interface IWRdsGraphicsChannelManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsGraphicsChannelManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0fd57159-e83e-476a-a8b9-4a7976e71e18")
    IWRdsGraphicsChannelManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateChannel( 
            /* [in] */ __RPC__in const char *pszChannelName,
            /* [in] */ WRdsGraphicsChannelType channelType,
            /* [retval][out] */ __RPC__deref_out_opt IWRdsGraphicsChannel **ppVirtualChannel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsGraphicsChannelManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsGraphicsChannelManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsGraphicsChannelManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsGraphicsChannelManager * This);
        
        DECLSPEC_XFGVIRT(IWRdsGraphicsChannelManager, CreateChannel)
        HRESULT ( STDMETHODCALLTYPE *CreateChannel )( 
            __RPC__in IWRdsGraphicsChannelManager * This,
            /* [in] */ __RPC__in const char *pszChannelName,
            /* [in] */ WRdsGraphicsChannelType channelType,
            /* [retval][out] */ __RPC__deref_out_opt IWRdsGraphicsChannel **ppVirtualChannel);
        
        END_INTERFACE
    } IWRdsGraphicsChannelManagerVtbl;

    interface IWRdsGraphicsChannelManager
    {
        CONST_VTBL struct IWRdsGraphicsChannelManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsGraphicsChannelManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsGraphicsChannelManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsGraphicsChannelManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsGraphicsChannelManager_CreateChannel(This,pszChannelName,channelType,ppVirtualChannel)	\
    ( (This)->lpVtbl -> CreateChannel(This,pszChannelName,channelType,ppVirtualChannel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsGraphicsChannelManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wrdsgraphicschannels_0000_0003 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
#if defined(_WIN64)
#define RFX_GFX_UNALIGNED __unaligned
#elif defined(_ARM_)
#define RFX_GFX_UNALIGNED __unaligned
#else
#define RFX_GFX_UNALIGNED
#endif
#define RFX_RDP_MSG_PREFIX          0x00
#define RFX_GFX_MSG_PREFIX          0x30
#define RFX_GFX_MSG_PREFIX_MASK     0x30
#define RFX_GFX_MSG_MAKE_16BIT_CODE(B0, B1)  ((B1) << 8 | (B0))
#define RFX_GFX_MSG_TYPE_MOUSE_POINTER_SHAPE            RFX_GFX_MSG_MAKE_16BIT_CODE(0x8B, 0x00)
#define RFX_GFX_MSG_TYPE_MOUSE_POINTER_POSITION         RFX_GFX_MSG_MAKE_16BIT_CODE(0x88, 0x00)
#define RFX_GFX_MSG_TYPE_MOUSE_POINTER_CACHED           RFX_GFX_MSG_MAKE_16BIT_CODE(0x8A, 0x00)
#define RFX_GFX_MSG_TYPE_CLIENT_DESKTOP_INFO_REQUEST    RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x1)
#define RFX_GFX_MSG_TYPE_CLIENT_DESKTOP_INFO_RESPONSE   RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x2)
#define RFX_GFX_MSG_TYPE_DESKTOP_CONFIG_CHANGE_NOTIFY   RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x3)
#define RFX_GFX_MSG_TYPE_DESKTOP_CONFIG_CHANGE_CONFIRM  RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x4)
#define RFX_GFX_MSG_TYPE_DESKTOP_SIZE_CHANGE_NOTIFY     RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x5)
#define RFX_GFX_MSG_TYPE_DISCONNECT_NOTIFY              RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x6)
#define RFX_GFX_MSG_TYPE_DESKTOP_RESEND_REQUEST         RFX_GFX_MSG_MAKE_16BIT_CODE(RFX_GFX_MSG_PREFIX, 0x7)
#pragma pack(push, rfx_gfx_msg, 1)
typedef struct tagRFX_GFX_RECT
    {
    INT32 left;
    INT32 top;
    INT32 right;
    INT32 bottom;
    } 	RFX_GFX_RECT;

typedef RFX_GFX_RECT RFX_GFX_UNALIGNED *PRFX_GFX_RECT;
typedef struct tagRFX_GFX_MSG_HEADER
    {
    UINT16 uMSGType;
    UINT16 cbSize;
    } 	RFX_GFX_MSG_HEADER;

typedef RFX_GFX_MSG_HEADER RFX_GFX_UNALIGNED *PRFX_GFX_MSG_HEADER;
typedef struct tagRFX_GFX_MONITOR_INFO
    {
    INT32 left;
    INT32 top;
    INT32 right;
    INT32 bottom;
    UINT32 physicalWidth;
    UINT32 physicalHeight;
    UINT32 orientation;
    BOOL primary;
    } 	RFX_GFX_MONITOR_INFO;

typedef RFX_GFX_MONITOR_INFO RFX_GFX_UNALIGNED *PRFX_GFX_MONITOR_INFO;
typedef struct tagRFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST
    {
    RFX_GFX_MSG_HEADER channelHdr;
    } 	RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST;

typedef RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST RFX_GFX_UNALIGNED *PRFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST;
#define	RFX_GFX_MAX_SUPPORTED_MONITORS	( 16 )

#define	RFX_CLIENT_ID_LENGTH	( 32 )

typedef struct tagRFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE
    {
    RFX_GFX_MSG_HEADER channelHdr;
    UINT32 reserved;
    UINT32 monitorCount;
    RFX_GFX_MONITOR_INFO MonitorData[ 16 ];
    WCHAR clientUniqueId[ 32 ];
    } 	RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE;

typedef RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE RFX_GFX_UNALIGNED *PRFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE;
typedef struct tagRFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY
    {
    RFX_GFX_MSG_HEADER channelHdr;
    ULONG ulWidth;
    ULONG ulHeight;
    ULONG ulBpp;
    ULONG Reserved;
    } 	RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY;

typedef RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY RFX_GFX_UNALIGNED *PRFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY;
typedef struct tagRFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM
    {
    RFX_GFX_MSG_HEADER channelHdr;
    } 	RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM;

typedef RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM RFX_GFX_UNALIGNED *PRFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM;
typedef struct tagRFX_GFX_MSG_DESKTOP_INPUT_RESET
    {
    RFX_GFX_MSG_HEADER channelHdr;
    UINT32 ulWidth;
    UINT32 ulHeight;
    } 	RFX_GFX_MSG_DESKTOP_INPUT_RESET;

typedef RFX_GFX_MSG_DESKTOP_INPUT_RESET RFX_GFX_UNALIGNED *PRFX_GFX_MSG_DESKTOP_INPUT_RESET;
typedef struct tagRFX_GFX_MSG_DISCONNECT_NOTIFY
    {
    RFX_GFX_MSG_HEADER channelHdr;
    ULONG DisconnectReason;
    } 	RFX_GFX_MSG_DISCONNECT_NOTIFY;

typedef RFX_GFX_MSG_DISCONNECT_NOTIFY RFX_GFX_UNALIGNED *PRFX_GFX_MSG_DISCONNECT_NOTIFY;
typedef struct tagRFX_GFX_MSG_DESKTOP_RESEND_REQUEST
    {
    RFX_GFX_MSG_HEADER channelHdr;
    RFX_GFX_RECT RedrawRect;
    } 	RFX_GFX_MSG_DESKTOP_RESEND_REQUEST;

typedef RFX_GFX_MSG_DESKTOP_RESEND_REQUEST RFX_GFX_UNALIGNED *PRFX_GFX_MSG_DESKTOP_RESEND_REQUEST;
typedef struct tagRFX_GFX_MSG_RDP_DATA
    {
    RFX_GFX_MSG_HEADER channelHdr;
    UINT8 rdpData[ 1 ];
    } 	RFX_GFX_MSG_RDP_DATA;

typedef RFX_GFX_MSG_RDP_DATA RFX_GFX_UNALIGNED *PRFX_GFX_MSG_RDP_DATA;
#pragma pack(pop, rfx_gfx_msg)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wrdsgraphicschannels_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


