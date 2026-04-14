

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

#ifndef __shellhandwriting_h__
#define __shellhandwriting_h__

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

#ifndef __ITfDetermineProximateHandwritingTargetArgs_FWD_DEFINED__
#define __ITfDetermineProximateHandwritingTargetArgs_FWD_DEFINED__
typedef interface ITfDetermineProximateHandwritingTargetArgs ITfDetermineProximateHandwritingTargetArgs;

#endif 	/* __ITfDetermineProximateHandwritingTargetArgs_FWD_DEFINED__ */


#ifndef __ITfFocusHandwritingTargetArgs_FWD_DEFINED__
#define __ITfFocusHandwritingTargetArgs_FWD_DEFINED__
typedef interface ITfFocusHandwritingTargetArgs ITfFocusHandwritingTargetArgs;

#endif 	/* __ITfFocusHandwritingTargetArgs_FWD_DEFINED__ */


#ifndef __ITfHandwritingSink_FWD_DEFINED__
#define __ITfHandwritingSink_FWD_DEFINED__
typedef interface ITfHandwritingSink ITfHandwritingSink;

#endif 	/* __ITfHandwritingSink_FWD_DEFINED__ */


#ifndef __ITfHandwritingRequest_FWD_DEFINED__
#define __ITfHandwritingRequest_FWD_DEFINED__
typedef interface ITfHandwritingRequest ITfHandwritingRequest;

#endif 	/* __ITfHandwritingRequest_FWD_DEFINED__ */


#ifndef __ITfHandwriting_FWD_DEFINED__
#define __ITfHandwriting_FWD_DEFINED__
typedef interface ITfHandwriting ITfHandwriting;

#endif 	/* __ITfHandwriting_FWD_DEFINED__ */


#ifndef __IHandwritingInputRoutingCallback_FWD_DEFINED__
#define __IHandwritingInputRoutingCallback_FWD_DEFINED__
typedef interface IHandwritingInputRoutingCallback IHandwritingInputRoutingCallback;

#endif 	/* __IHandwritingInputRoutingCallback_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shellhandwriting_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define TF_E_TOO_LATE    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x050A)
typedef /* [v1_enum] */ 
enum TfHandwritingState
    {
        TF_HANDWRITING_AUTO	= 0,
        TF_HANDWRITING_DISABLED	= 1,
        TF_HANDWRITING_ENABLED	= 2,
        TF_HANDWRITING_POINTERDELIVERY	= 3
    } 	TfHandwritingState;

typedef /* [v1_enum] */ 
enum TfProximateHandwritingTargetResponse
    {
        TF_NO_HANDWRITING_TARGET_PROXIMATE	= 0,
        TF_HANDWRITING_TARGET_PROXIMATE	= 1,
        TF_USE_SYSTEM_TARGETING	= 2,
        TF_USE_POINTER_DELIVERY	= 3
    } 	TfProximateHandwritingTargetResponse;

typedef /* [v1_enum] */ 
enum TfHandwritingFocusTargetResponse
    {
        TF_NO_HANDWRITING_TARGET	= 0,
        TF_HANDWRITING_TARGET_FOCUSED	= ( TF_NO_HANDWRITING_TARGET + 1 ) ,
        TF_HANDWRITING_TARGET_FOCUSED_NO_CORRECTIONS	= ( TF_HANDWRITING_TARGET_FOCUSED + 1 ) ,
        TF_PERFORM_SYSTEM_TARGETING	= ( TF_HANDWRITING_TARGET_FOCUSED_NO_CORRECTIONS + 1 ) 
    } 	TfHandwritingFocusTargetResponse;

typedef /* [v1_enum] */ 
enum TfInputEvaluation
    {
        TF_IE_HANDWRITING	= 0,
        TF_IE_TAP	= 1,
        TF_IE_CANCEL_HANDWRITING	= 2
    } 	TfInputEvaluation;



extern RPC_IF_HANDLE __MIDL_itf_shellhandwriting_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shellhandwriting_0000_0000_v0_0_s_ifspec;

#ifndef __ITfDetermineProximateHandwritingTargetArgs_INTERFACE_DEFINED__
#define __ITfDetermineProximateHandwritingTargetArgs_INTERFACE_DEFINED__

/* interface ITfDetermineProximateHandwritingTargetArgs */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_ITfDetermineProximateHandwritingTargetArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E676F4B-1951-530C-74D2-0DFDB76F17EB")
    ITfDetermineProximateHandwritingTargetArgs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPointerTargetInfo( 
            /* [out] */ HWND *targetWindow,
            /* [out] */ POINT *targetScreenPoint,
            /* [out] */ SIZE *distanceThreshold) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetResponse( 
            TfProximateHandwritingTargetResponse response) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDetermineProximateHandwritingTargetArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfDetermineProximateHandwritingTargetArgs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfDetermineProximateHandwritingTargetArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfDetermineProximateHandwritingTargetArgs * This);
        
        DECLSPEC_XFGVIRT(ITfDetermineProximateHandwritingTargetArgs, GetPointerTargetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPointerTargetInfo )( 
            ITfDetermineProximateHandwritingTargetArgs * This,
            /* [out] */ HWND *targetWindow,
            /* [out] */ POINT *targetScreenPoint,
            /* [out] */ SIZE *distanceThreshold);
        
        DECLSPEC_XFGVIRT(ITfDetermineProximateHandwritingTargetArgs, SetResponse)
        HRESULT ( STDMETHODCALLTYPE *SetResponse )( 
            ITfDetermineProximateHandwritingTargetArgs * This,
            TfProximateHandwritingTargetResponse response);
        
        END_INTERFACE
    } ITfDetermineProximateHandwritingTargetArgsVtbl;

    interface ITfDetermineProximateHandwritingTargetArgs
    {
        CONST_VTBL struct ITfDetermineProximateHandwritingTargetArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDetermineProximateHandwritingTargetArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDetermineProximateHandwritingTargetArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDetermineProximateHandwritingTargetArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDetermineProximateHandwritingTargetArgs_GetPointerTargetInfo(This,targetWindow,targetScreenPoint,distanceThreshold)	\
    ( (This)->lpVtbl -> GetPointerTargetInfo(This,targetWindow,targetScreenPoint,distanceThreshold) ) 

#define ITfDetermineProximateHandwritingTargetArgs_SetResponse(This,response)	\
    ( (This)->lpVtbl -> SetResponse(This,response) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDetermineProximateHandwritingTargetArgs_INTERFACE_DEFINED__ */


#ifndef __ITfFocusHandwritingTargetArgs_INTERFACE_DEFINED__
#define __ITfFocusHandwritingTargetArgs_INTERFACE_DEFINED__

/* interface ITfFocusHandwritingTargetArgs */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_ITfFocusHandwritingTargetArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5C4EC0E8-E7EE-51C7-6B7A-1CABBEE700E6")
    ITfFocusHandwritingTargetArgs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPointerTargetInfo( 
            /* [out] */ HWND *targetWindow,
            /* [out] */ RECT *targetScreenArea,
            /* [out] */ SIZE *distanceThreshold) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetResponse( 
            TfHandwritingFocusTargetResponse response) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfFocusHandwritingTargetArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfFocusHandwritingTargetArgs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfFocusHandwritingTargetArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfFocusHandwritingTargetArgs * This);
        
        DECLSPEC_XFGVIRT(ITfFocusHandwritingTargetArgs, GetPointerTargetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPointerTargetInfo )( 
            ITfFocusHandwritingTargetArgs * This,
            /* [out] */ HWND *targetWindow,
            /* [out] */ RECT *targetScreenArea,
            /* [out] */ SIZE *distanceThreshold);
        
        DECLSPEC_XFGVIRT(ITfFocusHandwritingTargetArgs, SetResponse)
        HRESULT ( STDMETHODCALLTYPE *SetResponse )( 
            ITfFocusHandwritingTargetArgs * This,
            TfHandwritingFocusTargetResponse response);
        
        END_INTERFACE
    } ITfFocusHandwritingTargetArgsVtbl;

    interface ITfFocusHandwritingTargetArgs
    {
        CONST_VTBL struct ITfFocusHandwritingTargetArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfFocusHandwritingTargetArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfFocusHandwritingTargetArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfFocusHandwritingTargetArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfFocusHandwritingTargetArgs_GetPointerTargetInfo(This,targetWindow,targetScreenArea,distanceThreshold)	\
    ( (This)->lpVtbl -> GetPointerTargetInfo(This,targetWindow,targetScreenArea,distanceThreshold) ) 

#define ITfFocusHandwritingTargetArgs_SetResponse(This,response)	\
    ( (This)->lpVtbl -> SetResponse(This,response) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfFocusHandwritingTargetArgs_INTERFACE_DEFINED__ */


#ifndef __ITfHandwritingSink_INTERFACE_DEFINED__
#define __ITfHandwritingSink_INTERFACE_DEFINED__

/* interface ITfHandwritingSink */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_ITfHandwritingSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3575A140-A10F-555F-35F9-45C865EB93BE")
    ITfHandwritingSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DetermineProximateHandwritingTarget( 
            /* [in] */ ITfDetermineProximateHandwritingTargetArgs *determineProximateHandwritingTargetArgs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FocusHandwritingTarget( 
            /* [in] */ ITfFocusHandwritingTargetArgs *focusHandwritingTargetArgs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfHandwritingSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfHandwritingSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfHandwritingSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfHandwritingSink * This);
        
        DECLSPEC_XFGVIRT(ITfHandwritingSink, DetermineProximateHandwritingTarget)
        HRESULT ( STDMETHODCALLTYPE *DetermineProximateHandwritingTarget )( 
            ITfHandwritingSink * This,
            /* [in] */ ITfDetermineProximateHandwritingTargetArgs *determineProximateHandwritingTargetArgs);
        
        DECLSPEC_XFGVIRT(ITfHandwritingSink, FocusHandwritingTarget)
        HRESULT ( STDMETHODCALLTYPE *FocusHandwritingTarget )( 
            ITfHandwritingSink * This,
            /* [in] */ ITfFocusHandwritingTargetArgs *focusHandwritingTargetArgs);
        
        END_INTERFACE
    } ITfHandwritingSinkVtbl;

    interface ITfHandwritingSink
    {
        CONST_VTBL struct ITfHandwritingSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfHandwritingSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfHandwritingSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfHandwritingSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfHandwritingSink_DetermineProximateHandwritingTarget(This,determineProximateHandwritingTargetArgs)	\
    ( (This)->lpVtbl -> DetermineProximateHandwritingTarget(This,determineProximateHandwritingTargetArgs) ) 

#define ITfHandwritingSink_FocusHandwritingTarget(This,focusHandwritingTargetArgs)	\
    ( (This)->lpVtbl -> FocusHandwritingTarget(This,focusHandwritingTargetArgs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfHandwritingSink_INTERFACE_DEFINED__ */


#ifndef __ITfHandwritingRequest_INTERFACE_DEFINED__
#define __ITfHandwritingRequest_INTERFACE_DEFINED__

/* interface ITfHandwritingRequest */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_ITfHandwritingRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE54892A-2FC4-52E7-8765-DE0E8E88081B")
    ITfHandwritingRequest : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInputEvaluation( 
            TfInputEvaluation inputEvaluation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfHandwritingRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfHandwritingRequest * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfHandwritingRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfHandwritingRequest * This);
        
        DECLSPEC_XFGVIRT(ITfHandwritingRequest, SetInputEvaluation)
        HRESULT ( STDMETHODCALLTYPE *SetInputEvaluation )( 
            ITfHandwritingRequest * This,
            TfInputEvaluation inputEvaluation);
        
        END_INTERFACE
    } ITfHandwritingRequestVtbl;

    interface ITfHandwritingRequest
    {
        CONST_VTBL struct ITfHandwritingRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfHandwritingRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfHandwritingRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfHandwritingRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfHandwritingRequest_SetInputEvaluation(This,inputEvaluation)	\
    ( (This)->lpVtbl -> SetInputEvaluation(This,inputEvaluation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfHandwritingRequest_INTERFACE_DEFINED__ */


#ifndef __ITfHandwriting_INTERFACE_DEFINED__
#define __ITfHandwriting_INTERFACE_DEFINED__

/* interface ITfHandwriting */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_ITfHandwriting;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59714133-8E20-5101-B1AE-D2CD9BAD8CE5")
    ITfHandwriting : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHandwritingState( 
            /* [out] */ TfHandwritingState *handwritingState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHandwritingState( 
            TfHandwritingState handwritingState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestHandwritingForPointer( 
            UINT32 pointerId,
            UINT64 handwritingStrokeId,
            /* [out] */ BOOL *requestAccepted,
            /* [out] */ ITfHandwritingRequest **request) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHandwritingDistanceThreshold( 
            /* [out] */ SIZE *distanceThresholdPixels) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfHandwritingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfHandwriting * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfHandwriting * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfHandwriting * This);
        
        DECLSPEC_XFGVIRT(ITfHandwriting, GetHandwritingState)
        HRESULT ( STDMETHODCALLTYPE *GetHandwritingState )( 
            ITfHandwriting * This,
            /* [out] */ TfHandwritingState *handwritingState);
        
        DECLSPEC_XFGVIRT(ITfHandwriting, SetHandwritingState)
        HRESULT ( STDMETHODCALLTYPE *SetHandwritingState )( 
            ITfHandwriting * This,
            TfHandwritingState handwritingState);
        
        DECLSPEC_XFGVIRT(ITfHandwriting, RequestHandwritingForPointer)
        HRESULT ( STDMETHODCALLTYPE *RequestHandwritingForPointer )( 
            ITfHandwriting * This,
            UINT32 pointerId,
            UINT64 handwritingStrokeId,
            /* [out] */ BOOL *requestAccepted,
            /* [out] */ ITfHandwritingRequest **request);
        
        DECLSPEC_XFGVIRT(ITfHandwriting, GetHandwritingDistanceThreshold)
        HRESULT ( STDMETHODCALLTYPE *GetHandwritingDistanceThreshold )( 
            ITfHandwriting * This,
            /* [out] */ SIZE *distanceThresholdPixels);
        
        END_INTERFACE
    } ITfHandwritingVtbl;

    interface ITfHandwriting
    {
        CONST_VTBL struct ITfHandwritingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfHandwriting_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfHandwriting_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfHandwriting_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfHandwriting_GetHandwritingState(This,handwritingState)	\
    ( (This)->lpVtbl -> GetHandwritingState(This,handwritingState) ) 

#define ITfHandwriting_SetHandwritingState(This,handwritingState)	\
    ( (This)->lpVtbl -> SetHandwritingState(This,handwritingState) ) 

#define ITfHandwriting_RequestHandwritingForPointer(This,pointerId,handwritingStrokeId,requestAccepted,request)	\
    ( (This)->lpVtbl -> RequestHandwritingForPointer(This,pointerId,handwritingStrokeId,requestAccepted,request) ) 

#define ITfHandwriting_GetHandwritingDistanceThreshold(This,distanceThresholdPixels)	\
    ( (This)->lpVtbl -> GetHandwritingDistanceThreshold(This,distanceThresholdPixels) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfHandwriting_INTERFACE_DEFINED__ */


#ifndef __IHandwritingInputRoutingCallback_INTERFACE_DEFINED__
#define __IHandwritingInputRoutingCallback_INTERFACE_DEFINED__

/* interface IHandwritingInputRoutingCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IHandwritingInputRoutingCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E3E84B6E-E625-5F79-C39E-32F1FF2056F6")
    IHandwritingInputRoutingCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetThreadIdForInput( 
            UINT32 pointerId,
            POINT *targetScreenPoint,
            HWND targetHWnd,
            /* [out] */ UINT32 *uiThreadId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHandwritingInputRoutingCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHandwritingInputRoutingCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHandwritingInputRoutingCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHandwritingInputRoutingCallback * This);
        
        DECLSPEC_XFGVIRT(IHandwritingInputRoutingCallback, GetThreadIdForInput)
        HRESULT ( STDMETHODCALLTYPE *GetThreadIdForInput )( 
            IHandwritingInputRoutingCallback * This,
            UINT32 pointerId,
            POINT *targetScreenPoint,
            HWND targetHWnd,
            /* [out] */ UINT32 *uiThreadId);
        
        END_INTERFACE
    } IHandwritingInputRoutingCallbackVtbl;

    interface IHandwritingInputRoutingCallback
    {
        CONST_VTBL struct IHandwritingInputRoutingCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHandwritingInputRoutingCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHandwritingInputRoutingCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHandwritingInputRoutingCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHandwritingInputRoutingCallback_GetThreadIdForInput(This,pointerId,targetScreenPoint,targetHWnd,uiThreadId)	\
    ( (This)->lpVtbl -> GetThreadIdForInput(This,pointerId,targetScreenPoint,targetHWnd,uiThreadId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHandwritingInputRoutingCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shellhandwriting_0000_0006 */
/* [local] */ 

HRESULT WINAPI RegisterHandwritingInputRoutingCallback(IHandwritingInputRoutingCallback * callback);
HRESULT WINAPI GetHandwritingStrokeIdForPointer(_In_ UINT32 pointerId, _Out_ UINT64* handwritingStrokeId);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // NTDDI_WIN10_NI


extern RPC_IF_HANDLE __MIDL_itf_shellhandwriting_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shellhandwriting_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


