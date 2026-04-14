

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

#ifndef __relogger_h__
#define __relogger_h__

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

#ifndef __ITraceEvent_FWD_DEFINED__
#define __ITraceEvent_FWD_DEFINED__
typedef interface ITraceEvent ITraceEvent;

#endif 	/* __ITraceEvent_FWD_DEFINED__ */


#ifndef __ITraceEventCallback_FWD_DEFINED__
#define __ITraceEventCallback_FWD_DEFINED__
typedef interface ITraceEventCallback ITraceEventCallback;

#endif 	/* __ITraceEventCallback_FWD_DEFINED__ */


#ifndef __ITraceRelogger_FWD_DEFINED__
#define __ITraceRelogger_FWD_DEFINED__
typedef interface ITraceRelogger ITraceRelogger;

#endif 	/* __ITraceRelogger_FWD_DEFINED__ */


#ifndef __ITraceEvent_FWD_DEFINED__
#define __ITraceEvent_FWD_DEFINED__
typedef interface ITraceEvent ITraceEvent;

#endif 	/* __ITraceEvent_FWD_DEFINED__ */


#ifndef __ITraceEventCallback_FWD_DEFINED__
#define __ITraceEventCallback_FWD_DEFINED__
typedef interface ITraceEventCallback ITraceEventCallback;

#endif 	/* __ITraceEventCallback_FWD_DEFINED__ */


#ifndef __ITraceRelogger_FWD_DEFINED__
#define __ITraceRelogger_FWD_DEFINED__
typedef interface ITraceRelogger ITraceRelogger;

#endif 	/* __ITraceRelogger_FWD_DEFINED__ */


#ifndef __CTraceRelogger_FWD_DEFINED__
#define __CTraceRelogger_FWD_DEFINED__

#ifdef __cplusplus
typedef class CTraceRelogger CTraceRelogger;
#else
typedef struct CTraceRelogger CTraceRelogger;
#endif /* __cplusplus */

#endif 	/* __CTraceRelogger_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_relogger_0000_0000 */
/* [local] */ 

//  Copyright (c) Microsoft Corporation.  All rights reserved.
#include <winapifamily.h>
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#ifndef EVENT_DESCRIPTOR_DEF
#define EVENT_DESCRIPTOR_DEF
typedef struct _EVENT_DESCRIPTOR
    {
    USHORT Id;
    UCHAR Version;
    UCHAR Channel;
    UCHAR Level;
    UCHAR Opcode;
    USHORT Task;
    ULONGLONG Keyword;
    } 	EVENT_DESCRIPTOR;

typedef struct _EVENT_DESCRIPTOR *PEVENT_DESCRIPTOR;

typedef const EVENT_DESCRIPTOR *PCEVENT_DESCRIPTOR;

#endif
#pragma warning(push)
#pragma warning (disable:4201) 
#ifndef EVENT_HEADER_DEF
#define EVENT_HEADER_DEF
typedef struct _EVENT_HEADER
    {
    USHORT Size;
    USHORT HeaderType;
    USHORT Flags;
    USHORT EventProperty;
    ULONG ThreadId;
    ULONG ProcessId;
    LARGE_INTEGER TimeStamp;
    GUID ProviderId;
    EVENT_DESCRIPTOR EventDescriptor;
    union 
        {
        struct 
            {
            ULONG KernelTime;
            ULONG UserTime;
            } 	DUMMYSTRUCTNAME;
        ULONG64 ProcessorTime;
        } 	DUMMYUNIONNAME;
    GUID ActivityId;
    } 	EVENT_HEADER;

typedef struct _EVENT_HEADER *PEVENT_HEADER;

#endif
#ifndef ETW_BUFFER_CONTEXT_DEF
#define ETW_BUFFER_CONTEXT_DEF
typedef struct _ETW_BUFFER_CONTEXT
    {
    union 
        {
        struct 
            {
            UCHAR ProcessorNumber;
            UCHAR Alignment;
            } 	DUMMYSTRUCTNAME;
        USHORT ProcessorIndex;
        } 	DUMMYUNIONNAME;
    USHORT LoggerId;
    } 	ETW_BUFFER_CONTEXT;

typedef struct _ETW_BUFFER_CONTEXT *PETW_BUFFER_CONTEXT;

#endif
#pragma warning(pop)
#ifndef EVENT_HEADER_EXTENDED_DATA_ITEM_DEF
#define EVENT_HEADER_EXTENDED_DATA_ITEM_DEF
typedef struct _EVENT_HEADER_EXTENDED_DATA_ITEM
    {
    USHORT Reserved1;
    USHORT ExtType;
    USHORT Linkage;
    USHORT DataSize;
    ULONGLONG DataPtr;
    } 	EVENT_HEADER_EXTENDED_DATA_ITEM;

typedef struct _EVENT_HEADER_EXTENDED_DATA_ITEM *PEVENT_HEADER_EXTENDED_DATA_ITEM;

#endif
#ifndef EVENT_RECORD_DEF
#define EVENT_RECORD_DEF
typedef struct _EVENT_RECORD
    {
    EVENT_HEADER EventHeader;
    ETW_BUFFER_CONTEXT BufferContext;
    USHORT ExtendedDataCount;
    USHORT UserDataLength;
    PEVENT_HEADER_EXTENDED_DATA_ITEM ExtendedData;
    PVOID UserData;
    PVOID UserContext;
    } 	EVENT_RECORD;

typedef struct _EVENT_RECORD *PEVENT_RECORD;

typedef const EVENT_RECORD *PCEVENT_RECORD;

#endif
typedef ULONG64 RELOGSTREAM_ID;




extern RPC_IF_HANDLE __MIDL_itf_relogger_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_relogger_0000_0000_v0_0_s_ifspec;

#ifndef __ITraceEvent_INTERFACE_DEFINED__
#define __ITraceEvent_INTERFACE_DEFINED__

/* interface ITraceEvent */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITraceEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC97F40-9028-4FF3-9B62-7D1F79CA7BCB")
    ITraceEvent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ ITraceEvent **NewEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserContext( 
            /* [retval][out] */ void **UserContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventRecord( 
            /* [retval][out] */ PEVENT_RECORD *EventRecord) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPayload( 
            /* [size_is][in] */ BYTE *Payload,
            /* [in] */ ULONG PayloadSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventDescriptor( 
            /* [in] */ PCEVENT_DESCRIPTOR EventDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProcessId( 
            /* [in] */ ULONG ProcessId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProcessorIndex( 
            /* [in] */ ULONG ProcessorIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThreadId( 
            /* [in] */ ULONG ThreadId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThreadTimes( 
            /* [in] */ ULONG KernelTime,
            /* [in] */ ULONG UserTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActivityId( 
            /* [in] */ LPCGUID ActivityId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimeStamp( 
            /* [in] */ LARGE_INTEGER *TimeStamp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProviderId( 
            /* [in] */ LPCGUID ProviderId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITraceEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITraceEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITraceEvent * This);
        
        DECLSPEC_XFGVIRT(ITraceEvent, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            ITraceEvent * This,
            /* [retval][out] */ ITraceEvent **NewEvent);
        
        DECLSPEC_XFGVIRT(ITraceEvent, GetUserContext)
        HRESULT ( STDMETHODCALLTYPE *GetUserContext )( 
            ITraceEvent * This,
            /* [retval][out] */ void **UserContext);
        
        DECLSPEC_XFGVIRT(ITraceEvent, GetEventRecord)
        HRESULT ( STDMETHODCALLTYPE *GetEventRecord )( 
            ITraceEvent * This,
            /* [retval][out] */ PEVENT_RECORD *EventRecord);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetPayload)
        HRESULT ( STDMETHODCALLTYPE *SetPayload )( 
            ITraceEvent * This,
            /* [size_is][in] */ BYTE *Payload,
            /* [in] */ ULONG PayloadSize);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetEventDescriptor)
        HRESULT ( STDMETHODCALLTYPE *SetEventDescriptor )( 
            ITraceEvent * This,
            /* [in] */ PCEVENT_DESCRIPTOR EventDescriptor);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetProcessId)
        HRESULT ( STDMETHODCALLTYPE *SetProcessId )( 
            ITraceEvent * This,
            /* [in] */ ULONG ProcessId);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetProcessorIndex)
        HRESULT ( STDMETHODCALLTYPE *SetProcessorIndex )( 
            ITraceEvent * This,
            /* [in] */ ULONG ProcessorIndex);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetThreadId)
        HRESULT ( STDMETHODCALLTYPE *SetThreadId )( 
            ITraceEvent * This,
            /* [in] */ ULONG ThreadId);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetThreadTimes)
        HRESULT ( STDMETHODCALLTYPE *SetThreadTimes )( 
            ITraceEvent * This,
            /* [in] */ ULONG KernelTime,
            /* [in] */ ULONG UserTime);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetActivityId)
        HRESULT ( STDMETHODCALLTYPE *SetActivityId )( 
            ITraceEvent * This,
            /* [in] */ LPCGUID ActivityId);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetTimeStamp)
        HRESULT ( STDMETHODCALLTYPE *SetTimeStamp )( 
            ITraceEvent * This,
            /* [in] */ LARGE_INTEGER *TimeStamp);
        
        DECLSPEC_XFGVIRT(ITraceEvent, SetProviderId)
        HRESULT ( STDMETHODCALLTYPE *SetProviderId )( 
            ITraceEvent * This,
            /* [in] */ LPCGUID ProviderId);
        
        END_INTERFACE
    } ITraceEventVtbl;

    interface ITraceEvent
    {
        CONST_VTBL struct ITraceEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceEvent_Clone(This,NewEvent)	\
    ( (This)->lpVtbl -> Clone(This,NewEvent) ) 

#define ITraceEvent_GetUserContext(This,UserContext)	\
    ( (This)->lpVtbl -> GetUserContext(This,UserContext) ) 

#define ITraceEvent_GetEventRecord(This,EventRecord)	\
    ( (This)->lpVtbl -> GetEventRecord(This,EventRecord) ) 

#define ITraceEvent_SetPayload(This,Payload,PayloadSize)	\
    ( (This)->lpVtbl -> SetPayload(This,Payload,PayloadSize) ) 

#define ITraceEvent_SetEventDescriptor(This,EventDescriptor)	\
    ( (This)->lpVtbl -> SetEventDescriptor(This,EventDescriptor) ) 

#define ITraceEvent_SetProcessId(This,ProcessId)	\
    ( (This)->lpVtbl -> SetProcessId(This,ProcessId) ) 

#define ITraceEvent_SetProcessorIndex(This,ProcessorIndex)	\
    ( (This)->lpVtbl -> SetProcessorIndex(This,ProcessorIndex) ) 

#define ITraceEvent_SetThreadId(This,ThreadId)	\
    ( (This)->lpVtbl -> SetThreadId(This,ThreadId) ) 

#define ITraceEvent_SetThreadTimes(This,KernelTime,UserTime)	\
    ( (This)->lpVtbl -> SetThreadTimes(This,KernelTime,UserTime) ) 

#define ITraceEvent_SetActivityId(This,ActivityId)	\
    ( (This)->lpVtbl -> SetActivityId(This,ActivityId) ) 

#define ITraceEvent_SetTimeStamp(This,TimeStamp)	\
    ( (This)->lpVtbl -> SetTimeStamp(This,TimeStamp) ) 

#define ITraceEvent_SetProviderId(This,ProviderId)	\
    ( (This)->lpVtbl -> SetProviderId(This,ProviderId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceEvent_INTERFACE_DEFINED__ */


#ifndef __ITraceEventCallback_INTERFACE_DEFINED__
#define __ITraceEventCallback_INTERFACE_DEFINED__

/* interface ITraceEventCallback */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITraceEventCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3ED25501-593F-43E9-8F38-3AB46F5A4A52")
    ITraceEventCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBeginProcessTrace( 
            /* [in] */ __RPC__in_opt ITraceEvent *HeaderEvent,
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFinalizeProcessTrace( 
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ __RPC__in_opt ITraceEvent *Event,
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceEventCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITraceEventCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITraceEventCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITraceEventCallback * This);
        
        DECLSPEC_XFGVIRT(ITraceEventCallback, OnBeginProcessTrace)
        HRESULT ( STDMETHODCALLTYPE *OnBeginProcessTrace )( 
            __RPC__in ITraceEventCallback * This,
            /* [in] */ __RPC__in_opt ITraceEvent *HeaderEvent,
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger);
        
        DECLSPEC_XFGVIRT(ITraceEventCallback, OnFinalizeProcessTrace)
        HRESULT ( STDMETHODCALLTYPE *OnFinalizeProcessTrace )( 
            __RPC__in ITraceEventCallback * This,
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger);
        
        DECLSPEC_XFGVIRT(ITraceEventCallback, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            __RPC__in ITraceEventCallback * This,
            /* [in] */ __RPC__in_opt ITraceEvent *Event,
            /* [in] */ __RPC__in_opt ITraceRelogger *Relogger);
        
        END_INTERFACE
    } ITraceEventCallbackVtbl;

    interface ITraceEventCallback
    {
        CONST_VTBL struct ITraceEventCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceEventCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceEventCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceEventCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceEventCallback_OnBeginProcessTrace(This,HeaderEvent,Relogger)	\
    ( (This)->lpVtbl -> OnBeginProcessTrace(This,HeaderEvent,Relogger) ) 

#define ITraceEventCallback_OnFinalizeProcessTrace(This,Relogger)	\
    ( (This)->lpVtbl -> OnFinalizeProcessTrace(This,Relogger) ) 

#define ITraceEventCallback_OnEvent(This,Event,Relogger)	\
    ( (This)->lpVtbl -> OnEvent(This,Event,Relogger) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceEventCallback_INTERFACE_DEFINED__ */


#ifndef __ITraceRelogger_INTERFACE_DEFINED__
#define __ITraceRelogger_INTERFACE_DEFINED__

/* interface ITraceRelogger */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITraceRelogger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F754AD43-3BCC-4286-8009-9C5DA214E84E")
    ITraceRelogger : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddLogfileTraceStream( 
            /* [in] */ BSTR LogfileName,
            /* [in] */ void *UserContext,
            /* [retval][out] */ RELOGSTREAM_ID *TraceStreamId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRealtimeTraceStream( 
            /* [in] */ BSTR LoggerName,
            /* [in] */ void *UserContext,
            /* [retval][out] */ RELOGSTREAM_ID *TraceStreamId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterCallback( 
            /* [in] */ ITraceEventCallback *Callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Inject( 
            /* [in] */ ITraceEvent *Event) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEventInstance( 
            /* [in] */ RELOGSTREAM_ID TraceStreamId,
            /* [in] */ ULONG Flags,
            /* [retval][out] */ ITraceEvent **Event) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessTrace( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputFilename( 
            /* [in] */ BSTR LogfileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompressionMode( 
            /* [in] */ BOOLEAN CompressionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceReloggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITraceRelogger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITraceRelogger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITraceRelogger * This);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, AddLogfileTraceStream)
        HRESULT ( STDMETHODCALLTYPE *AddLogfileTraceStream )( 
            ITraceRelogger * This,
            /* [in] */ BSTR LogfileName,
            /* [in] */ void *UserContext,
            /* [retval][out] */ RELOGSTREAM_ID *TraceStreamId);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, AddRealtimeTraceStream)
        HRESULT ( STDMETHODCALLTYPE *AddRealtimeTraceStream )( 
            ITraceRelogger * This,
            /* [in] */ BSTR LoggerName,
            /* [in] */ void *UserContext,
            /* [retval][out] */ RELOGSTREAM_ID *TraceStreamId);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, RegisterCallback)
        HRESULT ( STDMETHODCALLTYPE *RegisterCallback )( 
            ITraceRelogger * This,
            /* [in] */ ITraceEventCallback *Callback);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, Inject)
        HRESULT ( STDMETHODCALLTYPE *Inject )( 
            ITraceRelogger * This,
            /* [in] */ ITraceEvent *Event);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, CreateEventInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateEventInstance )( 
            ITraceRelogger * This,
            /* [in] */ RELOGSTREAM_ID TraceStreamId,
            /* [in] */ ULONG Flags,
            /* [retval][out] */ ITraceEvent **Event);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, ProcessTrace)
        HRESULT ( STDMETHODCALLTYPE *ProcessTrace )( 
            ITraceRelogger * This);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, SetOutputFilename)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFilename )( 
            ITraceRelogger * This,
            /* [in] */ BSTR LogfileName);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, SetCompressionMode)
        HRESULT ( STDMETHODCALLTYPE *SetCompressionMode )( 
            ITraceRelogger * This,
            /* [in] */ BOOLEAN CompressionMode);
        
        DECLSPEC_XFGVIRT(ITraceRelogger, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            ITraceRelogger * This);
        
        END_INTERFACE
    } ITraceReloggerVtbl;

    interface ITraceRelogger
    {
        CONST_VTBL struct ITraceReloggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceRelogger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceRelogger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceRelogger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceRelogger_AddLogfileTraceStream(This,LogfileName,UserContext,TraceStreamId)	\
    ( (This)->lpVtbl -> AddLogfileTraceStream(This,LogfileName,UserContext,TraceStreamId) ) 

#define ITraceRelogger_AddRealtimeTraceStream(This,LoggerName,UserContext,TraceStreamId)	\
    ( (This)->lpVtbl -> AddRealtimeTraceStream(This,LoggerName,UserContext,TraceStreamId) ) 

#define ITraceRelogger_RegisterCallback(This,Callback)	\
    ( (This)->lpVtbl -> RegisterCallback(This,Callback) ) 

#define ITraceRelogger_Inject(This,Event)	\
    ( (This)->lpVtbl -> Inject(This,Event) ) 

#define ITraceRelogger_CreateEventInstance(This,TraceStreamId,Flags,Event)	\
    ( (This)->lpVtbl -> CreateEventInstance(This,TraceStreamId,Flags,Event) ) 

#define ITraceRelogger_ProcessTrace(This)	\
    ( (This)->lpVtbl -> ProcessTrace(This) ) 

#define ITraceRelogger_SetOutputFilename(This,LogfileName)	\
    ( (This)->lpVtbl -> SetOutputFilename(This,LogfileName) ) 

#define ITraceRelogger_SetCompressionMode(This,CompressionMode)	\
    ( (This)->lpVtbl -> SetCompressionMode(This,CompressionMode) ) 

#define ITraceRelogger_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceRelogger_INTERFACE_DEFINED__ */



#ifndef __TraceReloggerLib_LIBRARY_DEFINED__
#define __TraceReloggerLib_LIBRARY_DEFINED__

/* library TraceReloggerLib */
/* [helpstring][uuid] */ 





EXTERN_C const IID LIBID_TraceReloggerLib;

EXTERN_C const CLSID CLSID_CTraceRelogger;

#ifdef __cplusplus

class DECLSPEC_UUID("7b40792d-05ff-44c4-9058-f440c71f17d4")
CTraceRelogger;
#endif
#endif /* __TraceReloggerLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_relogger_0000_0004 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
EXTERN_GUID(CLSID_TraceRelogger, 0x7b40792d, 0x05ff, 0x44c4, 0x90, 0x58, 0xf4, 0x40, 0xc7, 0x1f, 0x17, 0xd4);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_relogger_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_relogger_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


