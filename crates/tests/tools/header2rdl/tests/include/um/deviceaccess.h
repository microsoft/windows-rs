

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

#ifndef __deviceaccess_h__
#define __deviceaccess_h__

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

#ifndef __IDeviceRequestCompletionCallback_FWD_DEFINED__
#define __IDeviceRequestCompletionCallback_FWD_DEFINED__
typedef interface IDeviceRequestCompletionCallback IDeviceRequestCompletionCallback;

#endif 	/* __IDeviceRequestCompletionCallback_FWD_DEFINED__ */


#ifndef __IDeviceIoControl_FWD_DEFINED__
#define __IDeviceIoControl_FWD_DEFINED__
typedef interface IDeviceIoControl IDeviceIoControl;

#endif 	/* __IDeviceIoControl_FWD_DEFINED__ */


#ifndef __ICreateDeviceAccessAsync_FWD_DEFINED__
#define __ICreateDeviceAccessAsync_FWD_DEFINED__
typedef interface ICreateDeviceAccessAsync ICreateDeviceAccessAsync;

#endif 	/* __ICreateDeviceAccessAsync_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_deviceaccess_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)
STDAPI
CreateDeviceAccessInstance(
    _In_ PCWSTR deviceInterfacePath,
    _In_ DWORD desiredAccess,
    _COM_Outptr_ ICreateDeviceAccessAsync **createAsync
    );
#endif
EXTERN_GUID( CLSID_DeviceIoControl, 0x12d3e372, 0x874b, 0x457d, 0x9f, 0xdf, 0x73, 0x97, 0x77, 0x78, 0x68, 0x6c );



extern RPC_IF_HANDLE __MIDL_itf_deviceaccess_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_deviceaccess_0000_0000_v0_0_s_ifspec;

#ifndef __IDeviceRequestCompletionCallback_INTERFACE_DEFINED__
#define __IDeviceRequestCompletionCallback_INTERFACE_DEFINED__

/* interface IDeviceRequestCompletionCallback */
/* [unique][uuid][object] */ 

#define RequestCompletion Invoke

EXTERN_C const IID IID_IDeviceRequestCompletionCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("999BAD24-9ACD-45BB-8669-2A2FC0288B04")
    IDeviceRequestCompletionCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestCompletion( 
            /* [in] */ HRESULT requestResult,
            /* [in] */ DWORD bytesReturned) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeviceRequestCompletionCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeviceRequestCompletionCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeviceRequestCompletionCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeviceRequestCompletionCallback * This);
        
        DECLSPEC_XFGVIRT(IDeviceRequestCompletionCallback, RequestCompletion)
        HRESULT ( STDMETHODCALLTYPE *RequestCompletion )( 
            __RPC__in IDeviceRequestCompletionCallback * This,
            /* [in] */ HRESULT requestResult,
            /* [in] */ DWORD bytesReturned);
        
        END_INTERFACE
    } IDeviceRequestCompletionCallbackVtbl;

    interface IDeviceRequestCompletionCallback
    {
        CONST_VTBL struct IDeviceRequestCompletionCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeviceRequestCompletionCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeviceRequestCompletionCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeviceRequestCompletionCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeviceRequestCompletionCallback_RequestCompletion(This,requestResult,bytesReturned)	\
    ( (This)->lpVtbl -> RequestCompletion(This,requestResult,bytesReturned) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeviceRequestCompletionCallback_INTERFACE_DEFINED__ */


#ifndef __IDeviceIoControl_INTERFACE_DEFINED__
#define __IDeviceIoControl_INTERFACE_DEFINED__

/* interface IDeviceIoControl */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDeviceIoControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9EEFE161-23AB-4F18-9B49-991B586AE970")
    IDeviceIoControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeviceIoControlSync( 
            /* [annotation][in] */ 
            _In_  DWORD ioControlCode,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(inputBufferSize)  UCHAR *inputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD inputBufferSize,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(outputBufferSize)  UCHAR *outputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD outputBufferSize,
            /* [annotation][out] */ 
            _Out_  DWORD *bytesReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceIoControlAsync( 
            /* [annotation][in] */ 
            _In_  DWORD ioControlCode,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(inputBufferSize)  UCHAR *inputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD inputBufferSize,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(outputBufferSize)  UCHAR *outputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD outputBufferSize,
            /* [annotation][in] */ 
            _In_  IDeviceRequestCompletionCallback *requestCompletionCallback,
            /* [annotation][out] */ 
            _Out_opt_  ULONG_PTR *cancelContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelOperation( 
            /* [annotation][in] */ 
            _In_  ULONG_PTR cancelContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeviceIoControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDeviceIoControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDeviceIoControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDeviceIoControl * This);
        
        DECLSPEC_XFGVIRT(IDeviceIoControl, DeviceIoControlSync)
        HRESULT ( STDMETHODCALLTYPE *DeviceIoControlSync )( 
            IDeviceIoControl * This,
            /* [annotation][in] */ 
            _In_  DWORD ioControlCode,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(inputBufferSize)  UCHAR *inputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD inputBufferSize,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(outputBufferSize)  UCHAR *outputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD outputBufferSize,
            /* [annotation][out] */ 
            _Out_  DWORD *bytesReturned);
        
        DECLSPEC_XFGVIRT(IDeviceIoControl, DeviceIoControlAsync)
        HRESULT ( STDMETHODCALLTYPE *DeviceIoControlAsync )( 
            IDeviceIoControl * This,
            /* [annotation][in] */ 
            _In_  DWORD ioControlCode,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(inputBufferSize)  UCHAR *inputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD inputBufferSize,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(outputBufferSize)  UCHAR *outputBuffer,
            /* [annotation][in] */ 
            _In_  DWORD outputBufferSize,
            /* [annotation][in] */ 
            _In_  IDeviceRequestCompletionCallback *requestCompletionCallback,
            /* [annotation][out] */ 
            _Out_opt_  ULONG_PTR *cancelContext);
        
        DECLSPEC_XFGVIRT(IDeviceIoControl, CancelOperation)
        HRESULT ( STDMETHODCALLTYPE *CancelOperation )( 
            IDeviceIoControl * This,
            /* [annotation][in] */ 
            _In_  ULONG_PTR cancelContext);
        
        END_INTERFACE
    } IDeviceIoControlVtbl;

    interface IDeviceIoControl
    {
        CONST_VTBL struct IDeviceIoControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeviceIoControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeviceIoControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeviceIoControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeviceIoControl_DeviceIoControlSync(This,ioControlCode,inputBuffer,inputBufferSize,outputBuffer,outputBufferSize,bytesReturned)	\
    ( (This)->lpVtbl -> DeviceIoControlSync(This,ioControlCode,inputBuffer,inputBufferSize,outputBuffer,outputBufferSize,bytesReturned) ) 

#define IDeviceIoControl_DeviceIoControlAsync(This,ioControlCode,inputBuffer,inputBufferSize,outputBuffer,outputBufferSize,requestCompletionCallback,cancelContext)	\
    ( (This)->lpVtbl -> DeviceIoControlAsync(This,ioControlCode,inputBuffer,inputBufferSize,outputBuffer,outputBufferSize,requestCompletionCallback,cancelContext) ) 

#define IDeviceIoControl_CancelOperation(This,cancelContext)	\
    ( (This)->lpVtbl -> CancelOperation(This,cancelContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeviceIoControl_INTERFACE_DEFINED__ */


#ifndef __ICreateDeviceAccessAsync_INTERFACE_DEFINED__
#define __ICreateDeviceAccessAsync_INTERFACE_DEFINED__

/* interface ICreateDeviceAccessAsync */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ICreateDeviceAccessAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3474628F-683D-42D2-ABCB-DB018C6503BC")
    ICreateDeviceAccessAsync : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Wait( 
            /* [annotation][in] */ 
            _In_  DWORD timeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResult( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  void **deviceAccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateDeviceAccessAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateDeviceAccessAsync * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateDeviceAccessAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateDeviceAccessAsync * This);
        
        DECLSPEC_XFGVIRT(ICreateDeviceAccessAsync, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            ICreateDeviceAccessAsync * This);
        
        DECLSPEC_XFGVIRT(ICreateDeviceAccessAsync, Wait)
        HRESULT ( STDMETHODCALLTYPE *Wait )( 
            ICreateDeviceAccessAsync * This,
            /* [annotation][in] */ 
            _In_  DWORD timeout);
        
        DECLSPEC_XFGVIRT(ICreateDeviceAccessAsync, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            ICreateDeviceAccessAsync * This);
        
        DECLSPEC_XFGVIRT(ICreateDeviceAccessAsync, GetResult)
        HRESULT ( STDMETHODCALLTYPE *GetResult )( 
            ICreateDeviceAccessAsync * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  void **deviceAccess);
        
        END_INTERFACE
    } ICreateDeviceAccessAsyncVtbl;

    interface ICreateDeviceAccessAsync
    {
        CONST_VTBL struct ICreateDeviceAccessAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateDeviceAccessAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateDeviceAccessAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateDeviceAccessAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateDeviceAccessAsync_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define ICreateDeviceAccessAsync_Wait(This,timeout)	\
    ( (This)->lpVtbl -> Wait(This,timeout) ) 

#define ICreateDeviceAccessAsync_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define ICreateDeviceAccessAsync_GetResult(This,riid,deviceAccess)	\
    ( (This)->lpVtbl -> GetResult(This,riid,deviceAccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateDeviceAccessAsync_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


