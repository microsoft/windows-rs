

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

#ifndef __callobj_h__
#define __callobj_h__

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

#ifndef __ICallFrame_FWD_DEFINED__
#define __ICallFrame_FWD_DEFINED__
typedef interface ICallFrame ICallFrame;

#endif 	/* __ICallFrame_FWD_DEFINED__ */


#ifndef __ICallIndirect_FWD_DEFINED__
#define __ICallIndirect_FWD_DEFINED__
typedef interface ICallIndirect ICallIndirect;

#endif 	/* __ICallIndirect_FWD_DEFINED__ */


#ifndef __ICallInterceptor_FWD_DEFINED__
#define __ICallInterceptor_FWD_DEFINED__
typedef interface ICallInterceptor ICallInterceptor;

#endif 	/* __ICallInterceptor_FWD_DEFINED__ */


#ifndef __ICallFrameEvents_FWD_DEFINED__
#define __ICallFrameEvents_FWD_DEFINED__
typedef interface ICallFrameEvents ICallFrameEvents;

#endif 	/* __ICallFrameEvents_FWD_DEFINED__ */


#ifndef __ICallUnmarshal_FWD_DEFINED__
#define __ICallUnmarshal_FWD_DEFINED__
typedef interface ICallUnmarshal ICallUnmarshal;

#endif 	/* __ICallUnmarshal_FWD_DEFINED__ */


#ifndef __ICallFrameWalker_FWD_DEFINED__
#define __ICallFrameWalker_FWD_DEFINED__
typedef interface ICallFrameWalker ICallFrameWalker;

#endif 	/* __ICallFrameWalker_FWD_DEFINED__ */


#ifndef __IInterfaceRelated_FWD_DEFINED__
#define __IInterfaceRelated_FWD_DEFINED__
typedef interface IInterfaceRelated IInterfaceRelated;

#endif 	/* __IInterfaceRelated_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_callobj_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0000_v0_0_s_ifspec;

#ifndef __ICallFrame_INTERFACE_DEFINED__
#define __ICallFrame_INTERFACE_DEFINED__

/* interface ICallFrame */
/* [local][unique][object][uuid] */ 

typedef /* [public][public][public] */ struct __MIDL_ICallFrame_0001
    {
    ULONG iMethod;
    BOOL fHasInValues;
    BOOL fHasInOutValues;
    BOOL fHasOutValues;
    BOOL fDerivesFromIDispatch;
    LONG cInInterfacesMax;
    LONG cInOutInterfacesMax;
    LONG cOutInterfacesMax;
    LONG cTopLevelInInterfaces;
    IID iid;
    ULONG cMethod;
    ULONG cParams;
    } 	CALLFRAMEINFO;

typedef /* [public][public] */ struct __MIDL_ICallFrame_0002
    {
    BOOLEAN fIn;
    BOOLEAN fOut;
    ULONG stackOffset;
    ULONG cbParam;
    } 	CALLFRAMEPARAMINFO;

typedef /* [public][public] */ 
enum __MIDL_ICallFrame_0003
    {
        CALLFRAME_COPY_NESTED	= 1,
        CALLFRAME_COPY_INDEPENDENT	= 2
    } 	CALLFRAME_COPY;


enum CALLFRAME_FREE
    {
        CALLFRAME_FREE_NONE	= 0,
        CALLFRAME_FREE_IN	= 1,
        CALLFRAME_FREE_INOUT	= 2,
        CALLFRAME_FREE_OUT	= 4,
        CALLFRAME_FREE_TOP_INOUT	= 8,
        CALLFRAME_FREE_TOP_OUT	= 16,
        CALLFRAME_FREE_ALL	= 31
    } ;

enum CALLFRAME_NULL
    {
        CALLFRAME_NULL_NONE	= 0,
        CALLFRAME_NULL_INOUT	= 2,
        CALLFRAME_NULL_OUT	= 4,
        CALLFRAME_NULL_ALL	= 6
    } ;

enum CALLFRAME_WALK
    {
        CALLFRAME_WALK_IN	= 1,
        CALLFRAME_WALK_INOUT	= 2,
        CALLFRAME_WALK_OUT	= 4
    } ;
typedef /* [public][public][public][public][public][public][public] */ struct __MIDL_ICallFrame_0004
    {
    BOOLEAN fIn;
    DWORD dwDestContext;
    LPVOID pvDestContext;
    IUnknown *punkReserved;
    GUID guidTransferSyntax;
    } 	CALLFRAME_MARSHALCONTEXT;


EXTERN_C const IID IID_ICallFrame;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D573B4B0-894E-11d2-B8B6-00C04FB9618A")
    ICallFrame : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [out] */ CALLFRAMEINFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIIDAndMethod( 
            /* [out] */ IID *pIID,
            /* [out] */ ULONG *piMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNames( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszInterface,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszMethod) = 0;
        
        virtual PVOID STDMETHODCALLTYPE GetStackLocation( void) = 0;
        
        virtual void STDMETHODCALLTYPE SetStackLocation( 
            /* [in] */ PVOID pvStack) = 0;
        
        virtual void STDMETHODCALLTYPE SetReturnValue( 
            /* [in] */ HRESULT hr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReturnValue( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamInfo( 
            /* [in] */ ULONG iparam,
            /* [out] */ CALLFRAMEPARAMINFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParam( 
            /* [in] */ ULONG iparam,
            /* [in] */ VARIANT *pvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParam( 
            /* [in] */ ULONG iparam,
            /* [out] */ VARIANT *pvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Copy( 
            /* [in] */ CALLFRAME_COPY copyControl,
            /* [in] */ ICallFrameWalker *pWalker,
            /* [out] */ ICallFrame **ppFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Free( 
            /* [in] */ ICallFrame *pframeArgsDest,
            /* [in] */ ICallFrameWalker *pWalkerDestFree,
            /* [in] */ ICallFrameWalker *pWalkerCopy,
            /* [in] */ DWORD freeFlags,
            /* [in] */ ICallFrameWalker *pWalkerFree,
            /* [in] */ DWORD nullFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeParam( 
            /* [in] */ ULONG iparam,
            /* [in] */ DWORD freeFlags,
            /* [in] */ ICallFrameWalker *pWalkerFree,
            /* [in] */ DWORD nullFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WalkFrame( 
            /* [in] */ DWORD walkWhat,
            /* [in] */ ICallFrameWalker *pWalker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMarshalSizeMax( 
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pmshlContext,
            /* [in] */ MSHLFLAGS mshlflags,
            /* [out] */ ULONG *pcbBufferNeeded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Marshal( 
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pmshlContext,
            /* [in] */ MSHLFLAGS mshlflags,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [out] */ ULONG *pcbBufferUsed,
            /* [out] */ RPCOLEDATAREP *pdataRep,
            /* [out] */ ULONG *prpcFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unmarshal( 
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext,
            /* [out] */ ULONG *pcbUnmarshalled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseMarshalData( 
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ ULONG ibFirstRelease,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ void *pvReceiver,
            ...) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallFrameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallFrame * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallFrame * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallFrame * This);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            ICallFrame * This,
            /* [out] */ CALLFRAMEINFO *pInfo);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetIIDAndMethod)
        HRESULT ( STDMETHODCALLTYPE *GetIIDAndMethod )( 
            ICallFrame * This,
            /* [out] */ IID *pIID,
            /* [out] */ ULONG *piMethod);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetNames)
        HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            ICallFrame * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszInterface,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszMethod);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetStackLocation)
        PVOID ( STDMETHODCALLTYPE *GetStackLocation )( 
            ICallFrame * This);
        
        DECLSPEC_XFGVIRT(ICallFrame, SetStackLocation)
        void ( STDMETHODCALLTYPE *SetStackLocation )( 
            ICallFrame * This,
            /* [in] */ PVOID pvStack);
        
        DECLSPEC_XFGVIRT(ICallFrame, SetReturnValue)
        void ( STDMETHODCALLTYPE *SetReturnValue )( 
            ICallFrame * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetReturnValue)
        HRESULT ( STDMETHODCALLTYPE *GetReturnValue )( 
            ICallFrame * This);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetParamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetParamInfo )( 
            ICallFrame * This,
            /* [in] */ ULONG iparam,
            /* [out] */ CALLFRAMEPARAMINFO *pInfo);
        
        DECLSPEC_XFGVIRT(ICallFrame, SetParam)
        HRESULT ( STDMETHODCALLTYPE *SetParam )( 
            ICallFrame * This,
            /* [in] */ ULONG iparam,
            /* [in] */ VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetParam)
        HRESULT ( STDMETHODCALLTYPE *GetParam )( 
            ICallFrame * This,
            /* [in] */ ULONG iparam,
            /* [out] */ VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(ICallFrame, Copy)
        HRESULT ( STDMETHODCALLTYPE *Copy )( 
            ICallFrame * This,
            /* [in] */ CALLFRAME_COPY copyControl,
            /* [in] */ ICallFrameWalker *pWalker,
            /* [out] */ ICallFrame **ppFrame);
        
        DECLSPEC_XFGVIRT(ICallFrame, Free)
        HRESULT ( STDMETHODCALLTYPE *Free )( 
            ICallFrame * This,
            /* [in] */ ICallFrame *pframeArgsDest,
            /* [in] */ ICallFrameWalker *pWalkerDestFree,
            /* [in] */ ICallFrameWalker *pWalkerCopy,
            /* [in] */ DWORD freeFlags,
            /* [in] */ ICallFrameWalker *pWalkerFree,
            /* [in] */ DWORD nullFlags);
        
        DECLSPEC_XFGVIRT(ICallFrame, FreeParam)
        HRESULT ( STDMETHODCALLTYPE *FreeParam )( 
            ICallFrame * This,
            /* [in] */ ULONG iparam,
            /* [in] */ DWORD freeFlags,
            /* [in] */ ICallFrameWalker *pWalkerFree,
            /* [in] */ DWORD nullFlags);
        
        DECLSPEC_XFGVIRT(ICallFrame, WalkFrame)
        HRESULT ( STDMETHODCALLTYPE *WalkFrame )( 
            ICallFrame * This,
            /* [in] */ DWORD walkWhat,
            /* [in] */ ICallFrameWalker *pWalker);
        
        DECLSPEC_XFGVIRT(ICallFrame, GetMarshalSizeMax)
        HRESULT ( STDMETHODCALLTYPE *GetMarshalSizeMax )( 
            ICallFrame * This,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pmshlContext,
            /* [in] */ MSHLFLAGS mshlflags,
            /* [out] */ ULONG *pcbBufferNeeded);
        
        DECLSPEC_XFGVIRT(ICallFrame, Marshal)
        HRESULT ( STDMETHODCALLTYPE *Marshal )( 
            ICallFrame * This,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pmshlContext,
            /* [in] */ MSHLFLAGS mshlflags,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [out] */ ULONG *pcbBufferUsed,
            /* [out] */ RPCOLEDATAREP *pdataRep,
            /* [out] */ ULONG *prpcFlags);
        
        DECLSPEC_XFGVIRT(ICallFrame, Unmarshal)
        HRESULT ( STDMETHODCALLTYPE *Unmarshal )( 
            ICallFrame * This,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext,
            /* [out] */ ULONG *pcbUnmarshalled);
        
        DECLSPEC_XFGVIRT(ICallFrame, ReleaseMarshalData)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMarshalData )( 
            ICallFrame * This,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ ULONG ibFirstRelease,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext);
        
        DECLSPEC_XFGVIRT(ICallFrame, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICallFrame * This,
            /* [in] */ void *pvReceiver,
            ...);
        
        END_INTERFACE
    } ICallFrameVtbl;

    interface ICallFrame
    {
        CONST_VTBL struct ICallFrameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallFrame_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallFrame_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallFrame_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallFrame_GetInfo(This,pInfo)	\
    ( (This)->lpVtbl -> GetInfo(This,pInfo) ) 

#define ICallFrame_GetIIDAndMethod(This,pIID,piMethod)	\
    ( (This)->lpVtbl -> GetIIDAndMethod(This,pIID,piMethod) ) 

#define ICallFrame_GetNames(This,pwszInterface,pwszMethod)	\
    ( (This)->lpVtbl -> GetNames(This,pwszInterface,pwszMethod) ) 

#define ICallFrame_GetStackLocation(This)	\
    ( (This)->lpVtbl -> GetStackLocation(This) ) 

#define ICallFrame_SetStackLocation(This,pvStack)	\
    ( (This)->lpVtbl -> SetStackLocation(This,pvStack) ) 

#define ICallFrame_SetReturnValue(This,hr)	\
    ( (This)->lpVtbl -> SetReturnValue(This,hr) ) 

#define ICallFrame_GetReturnValue(This)	\
    ( (This)->lpVtbl -> GetReturnValue(This) ) 

#define ICallFrame_GetParamInfo(This,iparam,pInfo)	\
    ( (This)->lpVtbl -> GetParamInfo(This,iparam,pInfo) ) 

#define ICallFrame_SetParam(This,iparam,pvar)	\
    ( (This)->lpVtbl -> SetParam(This,iparam,pvar) ) 

#define ICallFrame_GetParam(This,iparam,pvar)	\
    ( (This)->lpVtbl -> GetParam(This,iparam,pvar) ) 

#define ICallFrame_Copy(This,copyControl,pWalker,ppFrame)	\
    ( (This)->lpVtbl -> Copy(This,copyControl,pWalker,ppFrame) ) 

#define ICallFrame_Free(This,pframeArgsDest,pWalkerDestFree,pWalkerCopy,freeFlags,pWalkerFree,nullFlags)	\
    ( (This)->lpVtbl -> Free(This,pframeArgsDest,pWalkerDestFree,pWalkerCopy,freeFlags,pWalkerFree,nullFlags) ) 

#define ICallFrame_FreeParam(This,iparam,freeFlags,pWalkerFree,nullFlags)	\
    ( (This)->lpVtbl -> FreeParam(This,iparam,freeFlags,pWalkerFree,nullFlags) ) 

#define ICallFrame_WalkFrame(This,walkWhat,pWalker)	\
    ( (This)->lpVtbl -> WalkFrame(This,walkWhat,pWalker) ) 

#define ICallFrame_GetMarshalSizeMax(This,pmshlContext,mshlflags,pcbBufferNeeded)	\
    ( (This)->lpVtbl -> GetMarshalSizeMax(This,pmshlContext,mshlflags,pcbBufferNeeded) ) 

#define ICallFrame_Marshal(This,pmshlContext,mshlflags,pBuffer,cbBuffer,pcbBufferUsed,pdataRep,prpcFlags)	\
    ( (This)->lpVtbl -> Marshal(This,pmshlContext,mshlflags,pBuffer,cbBuffer,pcbBufferUsed,pdataRep,prpcFlags) ) 

#define ICallFrame_Unmarshal(This,pBuffer,cbBuffer,dataRep,pcontext,pcbUnmarshalled)	\
    ( (This)->lpVtbl -> Unmarshal(This,pBuffer,cbBuffer,dataRep,pcontext,pcbUnmarshalled) ) 

#define ICallFrame_ReleaseMarshalData(This,pBuffer,cbBuffer,ibFirstRelease,dataRep,pcontext)	\
    ( (This)->lpVtbl -> ReleaseMarshalData(This,pBuffer,cbBuffer,ibFirstRelease,dataRep,pcontext) ) 

#define ICallFrame_Invoke(This,pvReceiver,...)	\
    ( (This)->lpVtbl -> Invoke(This,pvReceiver,...) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallFrame_INTERFACE_DEFINED__ */


#ifndef __ICallIndirect_INTERFACE_DEFINED__
#define __ICallIndirect_INTERFACE_DEFINED__

/* interface ICallIndirect */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ICallIndirect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D573B4B1-894E-11d2-B8B6-00C04FB9618A")
    ICallIndirect : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CallIndirect( 
            /* [out] */ HRESULT *phrReturn,
            /* [in] */ ULONG iMethod,
            /* [in] */ void *pvArgs,
            /* [out] */ ULONG *cbArgs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodInfo( 
            /* [in] */ ULONG iMethod,
            /* [out] */ CALLFRAMEINFO *pInfo,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStackSize( 
            /* [in] */ ULONG iMethod,
            /* [out] */ ULONG *cbArgs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIID( 
            /* [annotation][out] */ 
            _Out_opt_  IID *piid,
            /* [annotation][out] */ 
            _Out_opt_  BOOL *pfDerivesFromIDispatch,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcMethod,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallIndirectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallIndirect * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallIndirect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallIndirect * This);
        
        DECLSPEC_XFGVIRT(ICallIndirect, CallIndirect)
        HRESULT ( STDMETHODCALLTYPE *CallIndirect )( 
            ICallIndirect * This,
            /* [out] */ HRESULT *phrReturn,
            /* [in] */ ULONG iMethod,
            /* [in] */ void *pvArgs,
            /* [out] */ ULONG *cbArgs);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetMethodInfo)
        HRESULT ( STDMETHODCALLTYPE *GetMethodInfo )( 
            ICallIndirect * This,
            /* [in] */ ULONG iMethod,
            /* [out] */ CALLFRAMEINFO *pInfo,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszMethod);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetStackSize)
        HRESULT ( STDMETHODCALLTYPE *GetStackSize )( 
            ICallIndirect * This,
            /* [in] */ ULONG iMethod,
            /* [out] */ ULONG *cbArgs);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetIID)
        HRESULT ( STDMETHODCALLTYPE *GetIID )( 
            ICallIndirect * This,
            /* [annotation][out] */ 
            _Out_opt_  IID *piid,
            /* [annotation][out] */ 
            _Out_opt_  BOOL *pfDerivesFromIDispatch,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcMethod,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszInterface);
        
        END_INTERFACE
    } ICallIndirectVtbl;

    interface ICallIndirect
    {
        CONST_VTBL struct ICallIndirectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallIndirect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallIndirect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallIndirect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallIndirect_CallIndirect(This,phrReturn,iMethod,pvArgs,cbArgs)	\
    ( (This)->lpVtbl -> CallIndirect(This,phrReturn,iMethod,pvArgs,cbArgs) ) 

#define ICallIndirect_GetMethodInfo(This,iMethod,pInfo,pwszMethod)	\
    ( (This)->lpVtbl -> GetMethodInfo(This,iMethod,pInfo,pwszMethod) ) 

#define ICallIndirect_GetStackSize(This,iMethod,cbArgs)	\
    ( (This)->lpVtbl -> GetStackSize(This,iMethod,cbArgs) ) 

#define ICallIndirect_GetIID(This,piid,pfDerivesFromIDispatch,pcMethod,pwszInterface)	\
    ( (This)->lpVtbl -> GetIID(This,piid,pfDerivesFromIDispatch,pcMethod,pwszInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallIndirect_INTERFACE_DEFINED__ */


#ifndef __ICallInterceptor_INTERFACE_DEFINED__
#define __ICallInterceptor_INTERFACE_DEFINED__

/* interface ICallInterceptor */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ICallInterceptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60C7CA75-896D-11d2-B8B6-00C04FB9618A")
    ICallInterceptor : public ICallIndirect
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterSink( 
            /* [in] */ ICallFrameEvents *psink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisteredSink( 
            /* [out] */ ICallFrameEvents **ppsink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallInterceptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallInterceptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallInterceptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallInterceptor * This);
        
        DECLSPEC_XFGVIRT(ICallIndirect, CallIndirect)
        HRESULT ( STDMETHODCALLTYPE *CallIndirect )( 
            ICallInterceptor * This,
            /* [out] */ HRESULT *phrReturn,
            /* [in] */ ULONG iMethod,
            /* [in] */ void *pvArgs,
            /* [out] */ ULONG *cbArgs);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetMethodInfo)
        HRESULT ( STDMETHODCALLTYPE *GetMethodInfo )( 
            ICallInterceptor * This,
            /* [in] */ ULONG iMethod,
            /* [out] */ CALLFRAMEINFO *pInfo,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszMethod);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetStackSize)
        HRESULT ( STDMETHODCALLTYPE *GetStackSize )( 
            ICallInterceptor * This,
            /* [in] */ ULONG iMethod,
            /* [out] */ ULONG *cbArgs);
        
        DECLSPEC_XFGVIRT(ICallIndirect, GetIID)
        HRESULT ( STDMETHODCALLTYPE *GetIID )( 
            ICallInterceptor * This,
            /* [annotation][out] */ 
            _Out_opt_  IID *piid,
            /* [annotation][out] */ 
            _Out_opt_  BOOL *pfDerivesFromIDispatch,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcMethod,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *pwszInterface);
        
        DECLSPEC_XFGVIRT(ICallInterceptor, RegisterSink)
        HRESULT ( STDMETHODCALLTYPE *RegisterSink )( 
            ICallInterceptor * This,
            /* [in] */ ICallFrameEvents *psink);
        
        DECLSPEC_XFGVIRT(ICallInterceptor, GetRegisteredSink)
        HRESULT ( STDMETHODCALLTYPE *GetRegisteredSink )( 
            ICallInterceptor * This,
            /* [out] */ ICallFrameEvents **ppsink);
        
        END_INTERFACE
    } ICallInterceptorVtbl;

    interface ICallInterceptor
    {
        CONST_VTBL struct ICallInterceptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallInterceptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallInterceptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallInterceptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallInterceptor_CallIndirect(This,phrReturn,iMethod,pvArgs,cbArgs)	\
    ( (This)->lpVtbl -> CallIndirect(This,phrReturn,iMethod,pvArgs,cbArgs) ) 

#define ICallInterceptor_GetMethodInfo(This,iMethod,pInfo,pwszMethod)	\
    ( (This)->lpVtbl -> GetMethodInfo(This,iMethod,pInfo,pwszMethod) ) 

#define ICallInterceptor_GetStackSize(This,iMethod,cbArgs)	\
    ( (This)->lpVtbl -> GetStackSize(This,iMethod,cbArgs) ) 

#define ICallInterceptor_GetIID(This,piid,pfDerivesFromIDispatch,pcMethod,pwszInterface)	\
    ( (This)->lpVtbl -> GetIID(This,piid,pfDerivesFromIDispatch,pcMethod,pwszInterface) ) 


#define ICallInterceptor_RegisterSink(This,psink)	\
    ( (This)->lpVtbl -> RegisterSink(This,psink) ) 

#define ICallInterceptor_GetRegisteredSink(This,ppsink)	\
    ( (This)->lpVtbl -> GetRegisteredSink(This,ppsink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallInterceptor_INTERFACE_DEFINED__ */


#ifndef __ICallFrameEvents_INTERFACE_DEFINED__
#define __ICallFrameEvents_INTERFACE_DEFINED__

/* interface ICallFrameEvents */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ICallFrameEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD5E0843-FC91-11d0-97D7-00C04FB9618A")
    ICallFrameEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCall( 
            /* [in] */ ICallFrame *pFrame) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallFrameEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallFrameEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallFrameEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallFrameEvents * This);
        
        DECLSPEC_XFGVIRT(ICallFrameEvents, OnCall)
        HRESULT ( STDMETHODCALLTYPE *OnCall )( 
            ICallFrameEvents * This,
            /* [in] */ ICallFrame *pFrame);
        
        END_INTERFACE
    } ICallFrameEventsVtbl;

    interface ICallFrameEvents
    {
        CONST_VTBL struct ICallFrameEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallFrameEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallFrameEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallFrameEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallFrameEvents_OnCall(This,pFrame)	\
    ( (This)->lpVtbl -> OnCall(This,pFrame) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallFrameEvents_INTERFACE_DEFINED__ */


#ifndef __ICallUnmarshal_INTERFACE_DEFINED__
#define __ICallUnmarshal_INTERFACE_DEFINED__

/* interface ICallUnmarshal */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ICallUnmarshal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5333B003-2E42-11d2-B89D-00C04FB9618A")
    ICallUnmarshal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Unmarshal( 
            /* [in] */ ULONG iMethod,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ BOOL fForceBufferCopy,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext,
            /* [out] */ ULONG *pcbUnmarshalled,
            /* [out] */ ICallFrame **ppFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseMarshalData( 
            /* [in] */ ULONG iMethod,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ ULONG ibFirstRelease,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallUnmarshalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallUnmarshal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallUnmarshal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallUnmarshal * This);
        
        DECLSPEC_XFGVIRT(ICallUnmarshal, Unmarshal)
        HRESULT ( STDMETHODCALLTYPE *Unmarshal )( 
            ICallUnmarshal * This,
            /* [in] */ ULONG iMethod,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ BOOL fForceBufferCopy,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext,
            /* [out] */ ULONG *pcbUnmarshalled,
            /* [out] */ ICallFrame **ppFrame);
        
        DECLSPEC_XFGVIRT(ICallUnmarshal, ReleaseMarshalData)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMarshalData )( 
            ICallUnmarshal * This,
            /* [in] */ ULONG iMethod,
            /* [size_is][in] */ PVOID pBuffer,
            /* [in] */ ULONG cbBuffer,
            /* [in] */ ULONG ibFirstRelease,
            /* [in] */ RPCOLEDATAREP dataRep,
            /* [in] */ CALLFRAME_MARSHALCONTEXT *pcontext);
        
        END_INTERFACE
    } ICallUnmarshalVtbl;

    interface ICallUnmarshal
    {
        CONST_VTBL struct ICallUnmarshalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallUnmarshal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallUnmarshal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallUnmarshal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallUnmarshal_Unmarshal(This,iMethod,pBuffer,cbBuffer,fForceBufferCopy,dataRep,pcontext,pcbUnmarshalled,ppFrame)	\
    ( (This)->lpVtbl -> Unmarshal(This,iMethod,pBuffer,cbBuffer,fForceBufferCopy,dataRep,pcontext,pcbUnmarshalled,ppFrame) ) 

#define ICallUnmarshal_ReleaseMarshalData(This,iMethod,pBuffer,cbBuffer,ibFirstRelease,dataRep,pcontext)	\
    ( (This)->lpVtbl -> ReleaseMarshalData(This,iMethod,pBuffer,cbBuffer,ibFirstRelease,dataRep,pcontext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallUnmarshal_INTERFACE_DEFINED__ */


#ifndef __ICallFrameWalker_INTERFACE_DEFINED__
#define __ICallFrameWalker_INTERFACE_DEFINED__

/* interface ICallFrameWalker */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ICallFrameWalker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08B23919-392D-11d2-B8A4-00C04FB9618A")
    ICallFrameWalker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnWalkInterface( 
            /* [in] */ REFIID iid,
            /* [in] */ PVOID *ppvInterface,
            /* [in] */ BOOL fIn,
            /* [in] */ BOOL fOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallFrameWalkerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallFrameWalker * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallFrameWalker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallFrameWalker * This);
        
        DECLSPEC_XFGVIRT(ICallFrameWalker, OnWalkInterface)
        HRESULT ( STDMETHODCALLTYPE *OnWalkInterface )( 
            ICallFrameWalker * This,
            /* [in] */ REFIID iid,
            /* [in] */ PVOID *ppvInterface,
            /* [in] */ BOOL fIn,
            /* [in] */ BOOL fOut);
        
        END_INTERFACE
    } ICallFrameWalkerVtbl;

    interface ICallFrameWalker
    {
        CONST_VTBL struct ICallFrameWalkerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallFrameWalker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallFrameWalker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallFrameWalker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallFrameWalker_OnWalkInterface(This,iid,ppvInterface,fIn,fOut)	\
    ( (This)->lpVtbl -> OnWalkInterface(This,iid,ppvInterface,fIn,fOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallFrameWalker_INTERFACE_DEFINED__ */


#ifndef __IInterfaceRelated_INTERFACE_DEFINED__
#define __IInterfaceRelated_INTERFACE_DEFINED__

/* interface IInterfaceRelated */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_IInterfaceRelated;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D1FB5A79-7706-11d1-ADBA-00C04FC2ADC0")
    IInterfaceRelated : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetIID( 
            /* [in] */ REFIID iid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIID( 
            /* [out] */ IID *piid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInterfaceRelatedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInterfaceRelated * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInterfaceRelated * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInterfaceRelated * This);
        
        DECLSPEC_XFGVIRT(IInterfaceRelated, SetIID)
        HRESULT ( STDMETHODCALLTYPE *SetIID )( 
            IInterfaceRelated * This,
            /* [in] */ REFIID iid);
        
        DECLSPEC_XFGVIRT(IInterfaceRelated, GetIID)
        HRESULT ( STDMETHODCALLTYPE *GetIID )( 
            IInterfaceRelated * This,
            /* [out] */ IID *piid);
        
        END_INTERFACE
    } IInterfaceRelatedVtbl;

    interface IInterfaceRelated
    {
        CONST_VTBL struct IInterfaceRelatedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInterfaceRelated_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInterfaceRelated_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInterfaceRelated_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInterfaceRelated_SetIID(This,iid)	\
    ( (This)->lpVtbl -> SetIID(This,iid) ) 

#define IInterfaceRelated_GetIID(This,piid)	\
    ( (This)->lpVtbl -> GetIID(This,piid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInterfaceRelated_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_callobj_0000_0007 */
/* [local] */ 

#define CALLFRAME_E_ALREADYINVOKED  _HRESULT_TYPEDEF_(  0x8004d090 )
#define CALLFRAME_E_COULDNTMAKECALL _HRESULT_TYPEDEF_(  0x8004d091 )


extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0007_v0_0_s_ifspec;

#ifndef __ICallFrameAPIs_INTERFACE_DEFINED__
#define __ICallFrameAPIs_INTERFACE_DEFINED__

/* interface ICallFrameAPIs */
/* [local][uuid] */ 

HRESULT __stdcall CoGetInterceptor( 
    /* [in] */ REFIID iidIntercepted,
    /* [in] */ IUnknown *punkOuter,
    /* [in] */ REFIID iid,
    /* [out] */ void **ppv);

HRESULT __stdcall CoGetInterceptorFromTypeInfo( 
    /* [in] */ REFIID iidIntercepted,
    /* [in] */ IUnknown *punkOuter,
    /* [in] */ ITypeInfo *typeInfo,
    /* [in] */ REFIID iid,
    /* [out] */ void **ppv);



extern RPC_IF_HANDLE ICallFrameAPIs_v0_0_c_ifspec;
extern RPC_IF_HANDLE ICallFrameAPIs_v0_0_s_ifspec;
#endif /* __ICallFrameAPIs_INTERFACE_DEFINED__ */

/* interface __MIDL_itf_callobj_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_callobj_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


