

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

#ifndef __msp_h__
#define __msp_h__

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

#ifndef __ITPluggableTerminalEventSink_FWD_DEFINED__
#define __ITPluggableTerminalEventSink_FWD_DEFINED__
typedef interface ITPluggableTerminalEventSink ITPluggableTerminalEventSink;

#endif 	/* __ITPluggableTerminalEventSink_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalEventSinkRegistration_FWD_DEFINED__
#define __ITPluggableTerminalEventSinkRegistration_FWD_DEFINED__
typedef interface ITPluggableTerminalEventSinkRegistration ITPluggableTerminalEventSinkRegistration;

#endif 	/* __ITPluggableTerminalEventSinkRegistration_FWD_DEFINED__ */


#ifndef __ITMSPAddress_FWD_DEFINED__
#define __ITMSPAddress_FWD_DEFINED__
typedef interface ITMSPAddress ITMSPAddress;

#endif 	/* __ITMSPAddress_FWD_DEFINED__ */


/* header files for imported files */
#include "tapi3if.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msp_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved.*/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef long *MSP_HANDLE;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msp_0000_0000_0001
    {
        ADDRESS_TERMINAL_AVAILABLE	= 0,
        ADDRESS_TERMINAL_UNAVAILABLE	= ( ADDRESS_TERMINAL_AVAILABLE + 1 ) 
    } 	MSP_ADDRESS_EVENT;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msp_0000_0000_0002
    {
        CALL_NEW_STREAM	= 0,
        CALL_STREAM_FAIL	= ( CALL_NEW_STREAM + 1 ) ,
        CALL_TERMINAL_FAIL	= ( CALL_STREAM_FAIL + 1 ) ,
        CALL_STREAM_NOT_USED	= ( CALL_TERMINAL_FAIL + 1 ) ,
        CALL_STREAM_ACTIVE	= ( CALL_STREAM_NOT_USED + 1 ) ,
        CALL_STREAM_INACTIVE	= ( CALL_STREAM_ACTIVE + 1 ) 
    } 	MSP_CALL_EVENT;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msp_0000_0000_0003
    {
        CALL_CAUSE_UNKNOWN	= 0,
        CALL_CAUSE_BAD_DEVICE	= ( CALL_CAUSE_UNKNOWN + 1 ) ,
        CALL_CAUSE_CONNECT_FAIL	= ( CALL_CAUSE_BAD_DEVICE + 1 ) ,
        CALL_CAUSE_LOCAL_REQUEST	= ( CALL_CAUSE_CONNECT_FAIL + 1 ) ,
        CALL_CAUSE_REMOTE_REQUEST	= ( CALL_CAUSE_LOCAL_REQUEST + 1 ) ,
        CALL_CAUSE_MEDIA_TIMEOUT	= ( CALL_CAUSE_REMOTE_REQUEST + 1 ) ,
        CALL_CAUSE_MEDIA_RECOVERED	= ( CALL_CAUSE_MEDIA_TIMEOUT + 1 ) ,
        CALL_CAUSE_QUALITY_OF_SERVICE	= ( CALL_CAUSE_MEDIA_RECOVERED + 1 ) 
    } 	MSP_CALL_EVENT_CAUSE;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msp_0000_0000_0004
    {
        ME_ADDRESS_EVENT	= 0,
        ME_CALL_EVENT	= ( ME_ADDRESS_EVENT + 1 ) ,
        ME_TSP_DATA	= ( ME_CALL_EVENT + 1 ) ,
        ME_PRIVATE_EVENT	= ( ME_TSP_DATA + 1 ) ,
        ME_ASR_TERMINAL_EVENT	= ( ME_PRIVATE_EVENT + 1 ) ,
        ME_TTS_TERMINAL_EVENT	= ( ME_ASR_TERMINAL_EVENT + 1 ) ,
        ME_FILE_TERMINAL_EVENT	= ( ME_TTS_TERMINAL_EVENT + 1 ) ,
        ME_TONE_TERMINAL_EVENT	= ( ME_FILE_TERMINAL_EVENT + 1 ) 
    } 	MSP_EVENT;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_msp_0000_0000_0005
    {
    DWORD dwSize;
    MSP_EVENT Event;
    MSP_HANDLE hCall;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            MSP_ADDRESS_EVENT Type;
            ITTerminal *pTerminal;
            } 	MSP_ADDRESS_EVENT_INFO;
        /* [case()] */ struct 
            {
            MSP_CALL_EVENT Type;
            MSP_CALL_EVENT_CAUSE Cause;
            ITStream *pStream;
            ITTerminal *pTerminal;
            HRESULT hrError;
            } 	MSP_CALL_EVENT_INFO;
        /* [case()] */ struct 
            {
            DWORD dwBufferSize;
            BYTE pBuffer[ 1 ];
            } 	MSP_TSP_DATA;
        /* [case()] */ struct 
            {
            IDispatch *pEvent;
            long lEventCode;
            } 	MSP_PRIVATE_EVENT_INFO;
        /* [case()] */ struct 
            {
            ITTerminal *pParentFileTerminal;
            ITFileTrack *pFileTrack;
            TERMINAL_MEDIA_STATE TerminalMediaState;
            FT_STATE_EVENT_CAUSE ftecEventCause;
            HRESULT hrErrorCode;
            } 	MSP_FILE_TERMINAL_EVENT_INFO;
        /* [case()] */ struct 
            {
            ITTerminal *pASRTerminal;
            HRESULT hrErrorCode;
            } 	MSP_ASR_TERMINAL_EVENT_INFO;
        /* [case()] */ struct 
            {
            ITTerminal *pTTSTerminal;
            HRESULT hrErrorCode;
            } 	MSP_TTS_TERMINAL_EVENT_INFO;
        /* [case()] */ struct 
            {
            ITTerminal *pToneTerminal;
            HRESULT hrErrorCode;
            } 	MSP_TONE_TERMINAL_EVENT_INFO;
        } 	;
    } 	MSP_EVENT_INFO;



extern RPC_IF_HANDLE __MIDL_itf_msp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msp_0000_0000_v0_0_s_ifspec;

#ifndef __ITPluggableTerminalEventSink_INTERFACE_DEFINED__
#define __ITPluggableTerminalEventSink_INTERFACE_DEFINED__

/* interface ITPluggableTerminalEventSink */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPluggableTerminalEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6E0887BE-BA1A-492e-BD10-4020EC5E33E0")
    ITPluggableTerminalEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FireEvent( 
            /* [in] */ __RPC__in const MSP_EVENT_INFO *pMspEventInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalEventSink * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalEventSink, FireEvent)
        HRESULT ( STDMETHODCALLTYPE *FireEvent )( 
            __RPC__in ITPluggableTerminalEventSink * This,
            /* [in] */ __RPC__in const MSP_EVENT_INFO *pMspEventInfo);
        
        END_INTERFACE
    } ITPluggableTerminalEventSinkVtbl;

    interface ITPluggableTerminalEventSink
    {
        CONST_VTBL struct ITPluggableTerminalEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalEventSink_FireEvent(This,pMspEventInfo)	\
    ( (This)->lpVtbl -> FireEvent(This,pMspEventInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalEventSink_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalEventSinkRegistration_INTERFACE_DEFINED__
#define __ITPluggableTerminalEventSinkRegistration_INTERFACE_DEFINED__

/* interface ITPluggableTerminalEventSinkRegistration */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPluggableTerminalEventSinkRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F7115709-A216-4957-A759-060AB32A90D1")
    ITPluggableTerminalEventSinkRegistration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterSink( 
            /* [in] */ __RPC__in_opt ITPluggableTerminalEventSink *pEventSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterSink( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalEventSinkRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalEventSinkRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalEventSinkRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalEventSinkRegistration * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalEventSinkRegistration, RegisterSink)
        HRESULT ( STDMETHODCALLTYPE *RegisterSink )( 
            __RPC__in ITPluggableTerminalEventSinkRegistration * This,
            /* [in] */ __RPC__in_opt ITPluggableTerminalEventSink *pEventSink);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalEventSinkRegistration, UnregisterSink)
        HRESULT ( STDMETHODCALLTYPE *UnregisterSink )( 
            __RPC__in ITPluggableTerminalEventSinkRegistration * This);
        
        END_INTERFACE
    } ITPluggableTerminalEventSinkRegistrationVtbl;

    interface ITPluggableTerminalEventSinkRegistration
    {
        CONST_VTBL struct ITPluggableTerminalEventSinkRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalEventSinkRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalEventSinkRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalEventSinkRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalEventSinkRegistration_RegisterSink(This,pEventSink)	\
    ( (This)->lpVtbl -> RegisterSink(This,pEventSink) ) 

#define ITPluggableTerminalEventSinkRegistration_UnregisterSink(This)	\
    ( (This)->lpVtbl -> UnregisterSink(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalEventSinkRegistration_INTERFACE_DEFINED__ */


#ifndef __ITMSPAddress_INTERFACE_DEFINED__
#define __ITMSPAddress_INTERFACE_DEFINED__

/* interface ITMSPAddress */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITMSPAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD600-3868-11D2-A045-00C04FB6809F")
    ITMSPAddress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in MSP_HANDLE hEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateMSPCall( 
            /* [in] */ __RPC__in MSP_HANDLE hCall,
            /* [in] */ DWORD dwReserved,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ __RPC__in_opt IUnknown *pOuterUnknown,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppStreamControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShutdownMSPCall( 
            /* [in] */ __RPC__in_opt IUnknown *pStreamControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReceiveTSPData( 
            /* [in] */ __RPC__in_opt IUnknown *pMSPCall,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pBuffer,
            /* [in] */ DWORD dwSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) byte *pEventBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMSPAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMSPAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMSPAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMSPAddress * This);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITMSPAddress * This,
            /* [in] */ __RPC__in MSP_HANDLE hEvent);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in ITMSPAddress * This);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, CreateMSPCall)
        HRESULT ( STDMETHODCALLTYPE *CreateMSPCall )( 
            __RPC__in ITMSPAddress * This,
            /* [in] */ __RPC__in MSP_HANDLE hCall,
            /* [in] */ DWORD dwReserved,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ __RPC__in_opt IUnknown *pOuterUnknown,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppStreamControl);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, ShutdownMSPCall)
        HRESULT ( STDMETHODCALLTYPE *ShutdownMSPCall )( 
            __RPC__in ITMSPAddress * This,
            /* [in] */ __RPC__in_opt IUnknown *pStreamControl);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, ReceiveTSPData)
        HRESULT ( STDMETHODCALLTYPE *ReceiveTSPData )( 
            __RPC__in ITMSPAddress * This,
            /* [in] */ __RPC__in_opt IUnknown *pMSPCall,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pBuffer,
            /* [in] */ DWORD dwSize);
        
        DECLSPEC_XFGVIRT(ITMSPAddress, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in ITMSPAddress * This,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) byte *pEventBuffer);
        
        END_INTERFACE
    } ITMSPAddressVtbl;

    interface ITMSPAddress
    {
        CONST_VTBL struct ITMSPAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMSPAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMSPAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMSPAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMSPAddress_Initialize(This,hEvent)	\
    ( (This)->lpVtbl -> Initialize(This,hEvent) ) 

#define ITMSPAddress_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define ITMSPAddress_CreateMSPCall(This,hCall,dwReserved,dwMediaType,pOuterUnknown,ppStreamControl)	\
    ( (This)->lpVtbl -> CreateMSPCall(This,hCall,dwReserved,dwMediaType,pOuterUnknown,ppStreamControl) ) 

#define ITMSPAddress_ShutdownMSPCall(This,pStreamControl)	\
    ( (This)->lpVtbl -> ShutdownMSPCall(This,pStreamControl) ) 

#define ITMSPAddress_ReceiveTSPData(This,pMSPCall,pBuffer,dwSize)	\
    ( (This)->lpVtbl -> ReceiveTSPData(This,pMSPCall,pBuffer,dwSize) ) 

#define ITMSPAddress_GetEvent(This,pdwSize,pEventBuffer)	\
    ( (This)->lpVtbl -> GetEvent(This,pdwSize,pEventBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMSPAddress_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msp_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msp_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msp_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


