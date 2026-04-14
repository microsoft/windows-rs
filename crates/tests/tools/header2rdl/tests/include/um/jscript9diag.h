

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

#ifndef __jscript9diag_h__
#define __jscript9diag_h__

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

#ifndef __IJsDebug_FWD_DEFINED__
#define __IJsDebug_FWD_DEFINED__
typedef interface IJsDebug IJsDebug;

#endif 	/* __IJsDebug_FWD_DEFINED__ */


#ifndef __IJsDebugProcess_FWD_DEFINED__
#define __IJsDebugProcess_FWD_DEFINED__
typedef interface IJsDebugProcess IJsDebugProcess;

#endif 	/* __IJsDebugProcess_FWD_DEFINED__ */


#ifndef __IJsDebugStackWalker_FWD_DEFINED__
#define __IJsDebugStackWalker_FWD_DEFINED__
typedef interface IJsDebugStackWalker IJsDebugStackWalker;

#endif 	/* __IJsDebugStackWalker_FWD_DEFINED__ */


#ifndef __IJsDebugFrame_FWD_DEFINED__
#define __IJsDebugFrame_FWD_DEFINED__
typedef interface IJsDebugFrame IJsDebugFrame;

#endif 	/* __IJsDebugFrame_FWD_DEFINED__ */


#ifndef __IJsDebugProperty_FWD_DEFINED__
#define __IJsDebugProperty_FWD_DEFINED__
typedef interface IJsDebugProperty IJsDebugProperty;

#endif 	/* __IJsDebugProperty_FWD_DEFINED__ */


#ifndef __IJsEnumDebugProperty_FWD_DEFINED__
#define __IJsEnumDebugProperty_FWD_DEFINED__
typedef interface IJsEnumDebugProperty IJsEnumDebugProperty;

#endif 	/* __IJsEnumDebugProperty_FWD_DEFINED__ */


#ifndef __IJsDebugBreakPoint_FWD_DEFINED__
#define __IJsDebugBreakPoint_FWD_DEFINED__
typedef interface IJsDebugBreakPoint IJsDebugBreakPoint;

#endif 	/* __IJsDebugBreakPoint_FWD_DEFINED__ */


#ifndef __IEnumJsStackFrames_FWD_DEFINED__
#define __IEnumJsStackFrames_FWD_DEFINED__
typedef interface IEnumJsStackFrames IEnumJsStackFrames;

#endif 	/* __IEnumJsStackFrames_FWD_DEFINED__ */


#ifndef __IJsDebugDataTarget_FWD_DEFINED__
#define __IJsDebugDataTarget_FWD_DEFINED__
typedef interface IJsDebugDataTarget IJsDebugDataTarget;

#endif 	/* __IJsDebugDataTarget_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_jscript9diag_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)









#define FACILITY_JsDEBUG 0xdc7
// The Js runtime and the Js diag do not match.
#define E_JsDEBUG_MISMATCHED_RUNTIME      MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x1)
// Thread is not known to have any JS code, and will have no frames.
#define E_JsDEBUG_UNKNOWN_THREAD          MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x2)
// Frame is outside of the interpreter. For example, portions of the Js dll which are
// logically not part of the interpreter.
#define E_JsDEBUG_OUTSIDE_OF_VM   MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x4)
// Specified memory address could not be written/read from
#define E_JsDEBUG_INVALID_MEMORY_ADDRESS  MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x5)
// No source location found to bind the breakpoint
#define E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND  MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x6)
// Runtime not in debug mode
#define E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE MAKE_HRESULT(1, FACILITY_JsDEBUG, 0x7)


extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0000_v0_0_s_ifspec;

#ifndef __IJsDebug_INTERFACE_DEFINED__
#define __IJsDebug_INTERFACE_DEFINED__

/* interface IJsDebug */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebug;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BE0E89DA-2AC5-4C04-AC5E-59956AAE3613")
    IJsDebug : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenVirtualProcess( 
            /* [in] */ DWORD processId,
            /* [in] */ UINT64 runtimeJsBaseAddress,
            /* [in] */ IJsDebugDataTarget *pDataTarget,
            /* [out] */ IJsDebugProcess **ppProcess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebug * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebug * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebug * This);
        
        DECLSPEC_XFGVIRT(IJsDebug, OpenVirtualProcess)
        HRESULT ( STDMETHODCALLTYPE *OpenVirtualProcess )( 
            IJsDebug * This,
            /* [in] */ DWORD processId,
            /* [in] */ UINT64 runtimeJsBaseAddress,
            /* [in] */ IJsDebugDataTarget *pDataTarget,
            /* [out] */ IJsDebugProcess **ppProcess);
        
        END_INTERFACE
    } IJsDebugVtbl;

    interface IJsDebug
    {
        CONST_VTBL struct IJsDebugVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebug_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebug_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebug_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebug_OpenVirtualProcess(This,processId,runtimeJsBaseAddress,pDataTarget,ppProcess)	\
    ( (This)->lpVtbl -> OpenVirtualProcess(This,processId,runtimeJsBaseAddress,pDataTarget,ppProcess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebug_INTERFACE_DEFINED__ */


#ifndef __IJsDebugProcess_INTERFACE_DEFINED__
#define __IJsDebugProcess_INTERFACE_DEFINED__

/* interface IJsDebugProcess */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugProcess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D587168-6A2D-4041-BD3B-0DE674502862")
    IJsDebugProcess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateStackWalker( 
            /* [in] */ DWORD threadId,
            /* [out] */ IJsDebugStackWalker **ppStackWalker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBreakPoint( 
            /* [in] */ UINT64 documentId,
            /* [in] */ DWORD characterOffset,
            /* [in] */ DWORD characterCount,
            /* [in] */ BOOL isEnabled,
            /* [out] */ IJsDebugBreakPoint **ppDebugBreakPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PerformAsyncBreak( 
            /* [in] */ DWORD threadId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExternalStepAddress( 
            /* [out] */ UINT64 *pCodeAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugProcessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugProcess * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugProcess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugProcess * This);
        
        DECLSPEC_XFGVIRT(IJsDebugProcess, CreateStackWalker)
        HRESULT ( STDMETHODCALLTYPE *CreateStackWalker )( 
            IJsDebugProcess * This,
            /* [in] */ DWORD threadId,
            /* [out] */ IJsDebugStackWalker **ppStackWalker);
        
        DECLSPEC_XFGVIRT(IJsDebugProcess, CreateBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *CreateBreakPoint )( 
            IJsDebugProcess * This,
            /* [in] */ UINT64 documentId,
            /* [in] */ DWORD characterOffset,
            /* [in] */ DWORD characterCount,
            /* [in] */ BOOL isEnabled,
            /* [out] */ IJsDebugBreakPoint **ppDebugBreakPoint);
        
        DECLSPEC_XFGVIRT(IJsDebugProcess, PerformAsyncBreak)
        HRESULT ( STDMETHODCALLTYPE *PerformAsyncBreak )( 
            IJsDebugProcess * This,
            /* [in] */ DWORD threadId);
        
        DECLSPEC_XFGVIRT(IJsDebugProcess, GetExternalStepAddress)
        HRESULT ( STDMETHODCALLTYPE *GetExternalStepAddress )( 
            IJsDebugProcess * This,
            /* [out] */ UINT64 *pCodeAddress);
        
        END_INTERFACE
    } IJsDebugProcessVtbl;

    interface IJsDebugProcess
    {
        CONST_VTBL struct IJsDebugProcessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugProcess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugProcess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugProcess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugProcess_CreateStackWalker(This,threadId,ppStackWalker)	\
    ( (This)->lpVtbl -> CreateStackWalker(This,threadId,ppStackWalker) ) 

#define IJsDebugProcess_CreateBreakPoint(This,documentId,characterOffset,characterCount,isEnabled,ppDebugBreakPoint)	\
    ( (This)->lpVtbl -> CreateBreakPoint(This,documentId,characterOffset,characterCount,isEnabled,ppDebugBreakPoint) ) 

#define IJsDebugProcess_PerformAsyncBreak(This,threadId)	\
    ( (This)->lpVtbl -> PerformAsyncBreak(This,threadId) ) 

#define IJsDebugProcess_GetExternalStepAddress(This,pCodeAddress)	\
    ( (This)->lpVtbl -> GetExternalStepAddress(This,pCodeAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugProcess_INTERFACE_DEFINED__ */


#ifndef __IJsDebugStackWalker_INTERFACE_DEFINED__
#define __IJsDebugStackWalker_INTERFACE_DEFINED__

/* interface IJsDebugStackWalker */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugStackWalker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB24B094-73C4-456C-A4EC-E90EA00BDFE3")
    IJsDebugStackWalker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNext( 
            /* [out] */ IJsDebugFrame **ppFrame) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugStackWalkerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugStackWalker * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugStackWalker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugStackWalker * This);
        
        DECLSPEC_XFGVIRT(IJsDebugStackWalker, GetNext)
        HRESULT ( STDMETHODCALLTYPE *GetNext )( 
            IJsDebugStackWalker * This,
            /* [out] */ IJsDebugFrame **ppFrame);
        
        END_INTERFACE
    } IJsDebugStackWalkerVtbl;

    interface IJsDebugStackWalker
    {
        CONST_VTBL struct IJsDebugStackWalkerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugStackWalker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugStackWalker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugStackWalker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugStackWalker_GetNext(This,ppFrame)	\
    ( (This)->lpVtbl -> GetNext(This,ppFrame) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugStackWalker_INTERFACE_DEFINED__ */


#ifndef __IJsDebugFrame_INTERFACE_DEFINED__
#define __IJsDebugFrame_INTERFACE_DEFINED__

/* interface IJsDebugFrame */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugFrame;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9196637-AB9D-44B2-BAD2-13B95B3F390E")
    IJsDebugFrame : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStackRange( 
            /* [out] */ UINT64 *pStart,
            /* [out] */ UINT64 *pEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [out] */ BSTR *pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentPositionWithId( 
            /* [out] */ UINT64 *pDocumentId,
            /* [out] */ DWORD *pCharacterOffset,
            /* [out] */ DWORD *pStatementCharCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentPositionWithName( 
            /* [out] */ BSTR *pDocumentName,
            /* [out] */ DWORD *pLine,
            /* [out] */ DWORD *pColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDebugProperty( 
            /* [out] */ IJsDebugProperty **ppDebugProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReturnAddress( 
            /* [out] */ UINT64 *pReturnAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Evaluate( 
            /* [in] */ LPCOLESTR pExpressionText,
            /* [out] */ IJsDebugProperty **ppDebugProperty,
            /* [out] */ BSTR *pError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugFrameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugFrame * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugFrame * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugFrame * This);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetStackRange)
        HRESULT ( STDMETHODCALLTYPE *GetStackRange )( 
            IJsDebugFrame * This,
            /* [out] */ UINT64 *pStart,
            /* [out] */ UINT64 *pEnd);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IJsDebugFrame * This,
            /* [out] */ BSTR *pName);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetDocumentPositionWithId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentPositionWithId )( 
            IJsDebugFrame * This,
            /* [out] */ UINT64 *pDocumentId,
            /* [out] */ DWORD *pCharacterOffset,
            /* [out] */ DWORD *pStatementCharCount);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetDocumentPositionWithName)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentPositionWithName )( 
            IJsDebugFrame * This,
            /* [out] */ BSTR *pDocumentName,
            /* [out] */ DWORD *pLine,
            /* [out] */ DWORD *pColumn);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetDebugProperty)
        HRESULT ( STDMETHODCALLTYPE *GetDebugProperty )( 
            IJsDebugFrame * This,
            /* [out] */ IJsDebugProperty **ppDebugProperty);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, GetReturnAddress)
        HRESULT ( STDMETHODCALLTYPE *GetReturnAddress )( 
            IJsDebugFrame * This,
            /* [out] */ UINT64 *pReturnAddress);
        
        DECLSPEC_XFGVIRT(IJsDebugFrame, Evaluate)
        HRESULT ( STDMETHODCALLTYPE *Evaluate )( 
            IJsDebugFrame * This,
            /* [in] */ LPCOLESTR pExpressionText,
            /* [out] */ IJsDebugProperty **ppDebugProperty,
            /* [out] */ BSTR *pError);
        
        END_INTERFACE
    } IJsDebugFrameVtbl;

    interface IJsDebugFrame
    {
        CONST_VTBL struct IJsDebugFrameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugFrame_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugFrame_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugFrame_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugFrame_GetStackRange(This,pStart,pEnd)	\
    ( (This)->lpVtbl -> GetStackRange(This,pStart,pEnd) ) 

#define IJsDebugFrame_GetName(This,pName)	\
    ( (This)->lpVtbl -> GetName(This,pName) ) 

#define IJsDebugFrame_GetDocumentPositionWithId(This,pDocumentId,pCharacterOffset,pStatementCharCount)	\
    ( (This)->lpVtbl -> GetDocumentPositionWithId(This,pDocumentId,pCharacterOffset,pStatementCharCount) ) 

#define IJsDebugFrame_GetDocumentPositionWithName(This,pDocumentName,pLine,pColumn)	\
    ( (This)->lpVtbl -> GetDocumentPositionWithName(This,pDocumentName,pLine,pColumn) ) 

#define IJsDebugFrame_GetDebugProperty(This,ppDebugProperty)	\
    ( (This)->lpVtbl -> GetDebugProperty(This,ppDebugProperty) ) 

#define IJsDebugFrame_GetReturnAddress(This,pReturnAddress)	\
    ( (This)->lpVtbl -> GetReturnAddress(This,pReturnAddress) ) 

#define IJsDebugFrame_Evaluate(This,pExpressionText,ppDebugProperty,pError)	\
    ( (This)->lpVtbl -> Evaluate(This,pExpressionText,ppDebugProperty,pError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugFrame_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_jscript9diag_0000_0004 */
/* [local] */ 

typedef 
enum JS_PROPERTY_MEMBERS
    {
        JS_PROPERTY_MEMBERS_ALL	= 0,
        JS_PROPERTY_MEMBERS_ARGUMENTS	= 1
    } 	JS_PROPERTY_MEMBERS;

typedef /* [v1_enum] */ 
enum JS_PROPERTY_ATTRIBUTES
    {
        JS_PROPERTY_ATTRIBUTE_NONE	= 0,
        JS_PROPERTY_HAS_CHILDREN	= 0x1,
        JS_PROPERTY_FAKE	= 0x2,
        JS_PROPERTY_METHOD	= 0x4,
        JS_PROPERTY_READONLY	= 0x8,
        JS_PROPERTY_NATIVE_WINRT_POINTER	= 0x10,
        JS_PROPERTY_FRAME_INTRYBLOCK	= 0x20,
        JS_PROPERTY_FRAME_INCATCHBLOCK	= 0x40,
        JS_PROPERTY_FRAME_INFINALLYBLOCK	= 0x80
    } 	JS_PROPERTY_ATTRIBUTES;

typedef struct tagJsDebugPropertyInfo
    {
    BSTR name;
    BSTR type;
    BSTR value;
    BSTR fullName;
    JS_PROPERTY_ATTRIBUTES attr;
    } 	JsDebugPropertyInfo;



extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0004_v0_0_s_ifspec;

#ifndef __IJsDebugProperty_INTERFACE_DEFINED__
#define __IJsDebugProperty_INTERFACE_DEFINED__

/* interface IJsDebugProperty */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8FFCF2B-3AA4-4320-85C3-52A312BA9633")
    IJsDebugProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyInfo( 
            /* [in] */ UINT nRadix,
            /* [out] */ JsDebugPropertyInfo *pPropertyInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMembers( 
            /* [in] */ JS_PROPERTY_MEMBERS members,
            /* [out] */ IJsEnumDebugProperty **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugProperty * This);
        
        DECLSPEC_XFGVIRT(IJsDebugProperty, GetPropertyInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyInfo )( 
            IJsDebugProperty * This,
            /* [in] */ UINT nRadix,
            /* [out] */ JsDebugPropertyInfo *pPropertyInfo);
        
        DECLSPEC_XFGVIRT(IJsDebugProperty, GetMembers)
        HRESULT ( STDMETHODCALLTYPE *GetMembers )( 
            IJsDebugProperty * This,
            /* [in] */ JS_PROPERTY_MEMBERS members,
            /* [out] */ IJsEnumDebugProperty **ppEnum);
        
        END_INTERFACE
    } IJsDebugPropertyVtbl;

    interface IJsDebugProperty
    {
        CONST_VTBL struct IJsDebugPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugProperty_GetPropertyInfo(This,nRadix,pPropertyInfo)	\
    ( (This)->lpVtbl -> GetPropertyInfo(This,nRadix,pPropertyInfo) ) 

#define IJsDebugProperty_GetMembers(This,members,ppEnum)	\
    ( (This)->lpVtbl -> GetMembers(This,members,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugProperty_INTERFACE_DEFINED__ */


#ifndef __IJsEnumDebugProperty_INTERFACE_DEFINED__
#define __IJsEnumDebugProperty_INTERFACE_DEFINED__

/* interface IJsEnumDebugProperty */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsEnumDebugProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4092432F-2F0F-4FE1-B638-5B74A52CDCBE")
    IJsEnumDebugProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG count,
            /* [length_is][size_is][out] */ IJsDebugProperty **ppDebugProperty,
            /* [out] */ ULONG *pActualCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ ULONG *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsEnumDebugPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsEnumDebugProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsEnumDebugProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsEnumDebugProperty * This);
        
        DECLSPEC_XFGVIRT(IJsEnumDebugProperty, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IJsEnumDebugProperty * This,
            /* [in] */ ULONG count,
            /* [length_is][size_is][out] */ IJsDebugProperty **ppDebugProperty,
            /* [out] */ ULONG *pActualCount);
        
        DECLSPEC_XFGVIRT(IJsEnumDebugProperty, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IJsEnumDebugProperty * This,
            /* [out] */ ULONG *pCount);
        
        END_INTERFACE
    } IJsEnumDebugPropertyVtbl;

    interface IJsEnumDebugProperty
    {
        CONST_VTBL struct IJsEnumDebugPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsEnumDebugProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsEnumDebugProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsEnumDebugProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsEnumDebugProperty_Next(This,count,ppDebugProperty,pActualCount)	\
    ( (This)->lpVtbl -> Next(This,count,ppDebugProperty,pActualCount) ) 

#define IJsEnumDebugProperty_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsEnumDebugProperty_INTERFACE_DEFINED__ */


#ifndef __IJsDebugBreakPoint_INTERFACE_DEFINED__
#define __IJsDebugBreakPoint_INTERFACE_DEFINED__

/* interface IJsDebugBreakPoint */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugBreakPoint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF6773E3-ED8D-488B-8A3E-5812577D1542")
    IJsDebugBreakPoint : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsEnabled( 
            /* [out] */ BOOL *pIsEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentPosition( 
            /* [out] */ UINT64 *pDocumentId,
            /* [out] */ DWORD *pCharacterOffset,
            /* [out] */ DWORD *pStatementCharCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugBreakPointVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugBreakPoint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugBreakPoint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugBreakPoint * This);
        
        DECLSPEC_XFGVIRT(IJsDebugBreakPoint, IsEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsEnabled )( 
            IJsDebugBreakPoint * This,
            /* [out] */ BOOL *pIsEnabled);
        
        DECLSPEC_XFGVIRT(IJsDebugBreakPoint, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IJsDebugBreakPoint * This);
        
        DECLSPEC_XFGVIRT(IJsDebugBreakPoint, Disable)
        HRESULT ( STDMETHODCALLTYPE *Disable )( 
            IJsDebugBreakPoint * This);
        
        DECLSPEC_XFGVIRT(IJsDebugBreakPoint, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IJsDebugBreakPoint * This);
        
        DECLSPEC_XFGVIRT(IJsDebugBreakPoint, GetDocumentPosition)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentPosition )( 
            IJsDebugBreakPoint * This,
            /* [out] */ UINT64 *pDocumentId,
            /* [out] */ DWORD *pCharacterOffset,
            /* [out] */ DWORD *pStatementCharCount);
        
        END_INTERFACE
    } IJsDebugBreakPointVtbl;

    interface IJsDebugBreakPoint
    {
        CONST_VTBL struct IJsDebugBreakPointVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugBreakPoint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugBreakPoint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugBreakPoint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugBreakPoint_IsEnabled(This,pIsEnabled)	\
    ( (This)->lpVtbl -> IsEnabled(This,pIsEnabled) ) 

#define IJsDebugBreakPoint_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define IJsDebugBreakPoint_Disable(This)	\
    ( (This)->lpVtbl -> Disable(This) ) 

#define IJsDebugBreakPoint_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IJsDebugBreakPoint_GetDocumentPosition(This,pDocumentId,pCharacterOffset,pStatementCharCount)	\
    ( (This)->lpVtbl -> GetDocumentPosition(This,pDocumentId,pCharacterOffset,pStatementCharCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugBreakPoint_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_jscript9diag_0000_0007 */
/* [local] */ 

typedef /* [public][public] */ struct __MIDL___MIDL_itf_jscript9diag_0000_0007_0001
    {
    UINT64 InstructionOffset;
    UINT64 ReturnOffset;
    UINT64 FrameOffset;
    UINT64 StackOffset;
    } 	JS_NATIVE_FRAME;



extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0007_v0_0_s_ifspec;

#ifndef __IEnumJsStackFrames_INTERFACE_DEFINED__
#define __IEnumJsStackFrames_INTERFACE_DEFINED__

/* interface IEnumJsStackFrames */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumJsStackFrames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5E7DA34B-FB51-4791-ABE7-CB5BDF419755")
    IEnumJsStackFrames : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cFrameCount,
            /* [length_is][size_is][out] */ JS_NATIVE_FRAME *pFrames,
            /* [out] */ ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumJsStackFramesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumJsStackFrames * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumJsStackFrames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumJsStackFrames * This);
        
        DECLSPEC_XFGVIRT(IEnumJsStackFrames, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumJsStackFrames * This,
            /* [in] */ ULONG cFrameCount,
            /* [length_is][size_is][out] */ JS_NATIVE_FRAME *pFrames,
            /* [out] */ ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumJsStackFrames, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumJsStackFrames * This);
        
        END_INTERFACE
    } IEnumJsStackFramesVtbl;

    interface IEnumJsStackFrames
    {
        CONST_VTBL struct IEnumJsStackFramesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumJsStackFrames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumJsStackFrames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumJsStackFrames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumJsStackFrames_Next(This,cFrameCount,pFrames,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cFrameCount,pFrames,pcFetched) ) 

#define IEnumJsStackFrames_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumJsStackFrames_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_jscript9diag_0000_0008 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum JsDebugReadMemoryFlags
    {
        None	= 0,
        JsDebugAllowPartialRead	= 0x1
    } 	JsDebugReadMemoryFlags;



extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0008_v0_0_s_ifspec;

#ifndef __IJsDebugDataTarget_INTERFACE_DEFINED__
#define __IJsDebugDataTarget_INTERFACE_DEFINED__

/* interface IJsDebugDataTarget */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IJsDebugDataTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53B28977-53A1-48E5-9000-5D0DFA893931")
    IJsDebugDataTarget : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReadMemory( 
            /* [in] */ UINT64 address,
            /* [in] */ JsDebugReadMemoryFlags flags,
            /* [length_is][size_is][out] */ BYTE *pBuffer,
            /* [in] */ DWORD size,
            /* [out] */ DWORD *pBytesRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteMemory( 
            /* [in] */ UINT64 address,
            /* [size_is][in] */ const BYTE *pMemory,
            /* [in] */ DWORD size) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocateVirtualMemory( 
            /* [in] */ UINT64 address,
            /* [in] */ DWORD size,
            /* [in] */ DWORD allocationType,
            /* [in] */ DWORD pageProtection,
            /* [out] */ UINT64 *pAllocatedAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeVirtualMemory( 
            /* [in] */ UINT64 address,
            /* [in] */ DWORD size,
            /* [in] */ DWORD freeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTlsValue( 
            /* [in] */ DWORD threadId,
            /* [in] */ UINT32 tlsIndex,
            /* [out] */ UINT64 *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadBSTR( 
            /* [in] */ UINT64 address,
            /* [out] */ BSTR *pString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadNullTerminatedString( 
            /* [in] */ UINT64 address,
            /* [in] */ UINT16 characterSize,
            /* [in] */ UINT32 maxCharacters,
            /* [out] */ BSTR *pString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStackFrameEnumerator( 
            /* [in] */ DWORD threadId,
            /* [out] */ IEnumJsStackFrames **ppEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThreadContext( 
            /* [in] */ DWORD threadId,
            /* [in] */ ULONG32 contextFlags,
            /* [in] */ ULONG32 contextSize,
            /* [size_is][out] */ void *pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJsDebugDataTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IJsDebugDataTarget * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IJsDebugDataTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IJsDebugDataTarget * This);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, ReadMemory)
        HRESULT ( STDMETHODCALLTYPE *ReadMemory )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [in] */ JsDebugReadMemoryFlags flags,
            /* [length_is][size_is][out] */ BYTE *pBuffer,
            /* [in] */ DWORD size,
            /* [out] */ DWORD *pBytesRead);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, WriteMemory)
        HRESULT ( STDMETHODCALLTYPE *WriteMemory )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [size_is][in] */ const BYTE *pMemory,
            /* [in] */ DWORD size);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, AllocateVirtualMemory)
        HRESULT ( STDMETHODCALLTYPE *AllocateVirtualMemory )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [in] */ DWORD size,
            /* [in] */ DWORD allocationType,
            /* [in] */ DWORD pageProtection,
            /* [out] */ UINT64 *pAllocatedAddress);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, FreeVirtualMemory)
        HRESULT ( STDMETHODCALLTYPE *FreeVirtualMemory )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [in] */ DWORD size,
            /* [in] */ DWORD freeType);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, GetTlsValue)
        HRESULT ( STDMETHODCALLTYPE *GetTlsValue )( 
            IJsDebugDataTarget * This,
            /* [in] */ DWORD threadId,
            /* [in] */ UINT32 tlsIndex,
            /* [out] */ UINT64 *pValue);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, ReadBSTR)
        HRESULT ( STDMETHODCALLTYPE *ReadBSTR )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [out] */ BSTR *pString);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, ReadNullTerminatedString)
        HRESULT ( STDMETHODCALLTYPE *ReadNullTerminatedString )( 
            IJsDebugDataTarget * This,
            /* [in] */ UINT64 address,
            /* [in] */ UINT16 characterSize,
            /* [in] */ UINT32 maxCharacters,
            /* [out] */ BSTR *pString);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, CreateStackFrameEnumerator)
        HRESULT ( STDMETHODCALLTYPE *CreateStackFrameEnumerator )( 
            IJsDebugDataTarget * This,
            /* [in] */ DWORD threadId,
            /* [out] */ IEnumJsStackFrames **ppEnumerator);
        
        DECLSPEC_XFGVIRT(IJsDebugDataTarget, GetThreadContext)
        HRESULT ( STDMETHODCALLTYPE *GetThreadContext )( 
            IJsDebugDataTarget * This,
            /* [in] */ DWORD threadId,
            /* [in] */ ULONG32 contextFlags,
            /* [in] */ ULONG32 contextSize,
            /* [size_is][out] */ void *pContext);
        
        END_INTERFACE
    } IJsDebugDataTargetVtbl;

    interface IJsDebugDataTarget
    {
        CONST_VTBL struct IJsDebugDataTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJsDebugDataTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJsDebugDataTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJsDebugDataTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJsDebugDataTarget_ReadMemory(This,address,flags,pBuffer,size,pBytesRead)	\
    ( (This)->lpVtbl -> ReadMemory(This,address,flags,pBuffer,size,pBytesRead) ) 

#define IJsDebugDataTarget_WriteMemory(This,address,pMemory,size)	\
    ( (This)->lpVtbl -> WriteMemory(This,address,pMemory,size) ) 

#define IJsDebugDataTarget_AllocateVirtualMemory(This,address,size,allocationType,pageProtection,pAllocatedAddress)	\
    ( (This)->lpVtbl -> AllocateVirtualMemory(This,address,size,allocationType,pageProtection,pAllocatedAddress) ) 

#define IJsDebugDataTarget_FreeVirtualMemory(This,address,size,freeType)	\
    ( (This)->lpVtbl -> FreeVirtualMemory(This,address,size,freeType) ) 

#define IJsDebugDataTarget_GetTlsValue(This,threadId,tlsIndex,pValue)	\
    ( (This)->lpVtbl -> GetTlsValue(This,threadId,tlsIndex,pValue) ) 

#define IJsDebugDataTarget_ReadBSTR(This,address,pString)	\
    ( (This)->lpVtbl -> ReadBSTR(This,address,pString) ) 

#define IJsDebugDataTarget_ReadNullTerminatedString(This,address,characterSize,maxCharacters,pString)	\
    ( (This)->lpVtbl -> ReadNullTerminatedString(This,address,characterSize,maxCharacters,pString) ) 

#define IJsDebugDataTarget_CreateStackFrameEnumerator(This,threadId,ppEnumerator)	\
    ( (This)->lpVtbl -> CreateStackFrameEnumerator(This,threadId,ppEnumerator) ) 

#define IJsDebugDataTarget_GetThreadContext(This,threadId,contextFlags,contextSize,pContext)	\
    ( (This)->lpVtbl -> GetThreadContext(This,threadId,contextFlags,contextSize,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJsDebugDataTarget_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_jscript9diag_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_jscript9diag_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


