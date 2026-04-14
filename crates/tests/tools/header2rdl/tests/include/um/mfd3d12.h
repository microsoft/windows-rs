

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

#ifndef __mfd3d12_h__
#define __mfd3d12_h__

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

#ifndef __IMFD3D12SynchronizationObjectCommands_FWD_DEFINED__
#define __IMFD3D12SynchronizationObjectCommands_FWD_DEFINED__
typedef interface IMFD3D12SynchronizationObjectCommands IMFD3D12SynchronizationObjectCommands;

#endif 	/* __IMFD3D12SynchronizationObjectCommands_FWD_DEFINED__ */


#ifndef __IMFD3D12SynchronizationObject_FWD_DEFINED__
#define __IMFD3D12SynchronizationObject_FWD_DEFINED__
typedef interface IMFD3D12SynchronizationObject IMFD3D12SynchronizationObject;

#endif 	/* __IMFD3D12SynchronizationObject_FWD_DEFINED__ */


/* header files for imported files */
#include "mfidl.h"
#include "d3d12.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfd3d12_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfd3d12_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfd3d12_0000_0000_v0_0_s_ifspec;

#ifndef __IMFD3D12SynchronizationObjectCommands_INTERFACE_DEFINED__
#define __IMFD3D12SynchronizationObjectCommands_INTERFACE_DEFINED__

/* interface IMFD3D12SynchronizationObjectCommands */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IMFD3D12SynchronizationObjectCommands;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09D0F835-92FF-4E53-8EFA-40FAA551F233")
    IMFD3D12SynchronizationObjectCommands : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnqueueResourceReady( 
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pProducerCommandQueue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnqueueResourceReadyWait( 
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pConsumerCommandQueue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SignalEventOnResourceReady( 
            HANDLE hEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnqueueResourceRelease( 
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pConsumerCommandQueue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFD3D12SynchronizationObjectCommandsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFD3D12SynchronizationObjectCommands * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFD3D12SynchronizationObjectCommands * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFD3D12SynchronizationObjectCommands * This);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObjectCommands, EnqueueResourceReady)
        HRESULT ( STDMETHODCALLTYPE *EnqueueResourceReady )( 
            IMFD3D12SynchronizationObjectCommands * This,
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pProducerCommandQueue);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObjectCommands, EnqueueResourceReadyWait)
        HRESULT ( STDMETHODCALLTYPE *EnqueueResourceReadyWait )( 
            IMFD3D12SynchronizationObjectCommands * This,
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pConsumerCommandQueue);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObjectCommands, SignalEventOnResourceReady)
        HRESULT ( STDMETHODCALLTYPE *SignalEventOnResourceReady )( 
            IMFD3D12SynchronizationObjectCommands * This,
            HANDLE hEvent);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObjectCommands, EnqueueResourceRelease)
        HRESULT ( STDMETHODCALLTYPE *EnqueueResourceRelease )( 
            IMFD3D12SynchronizationObjectCommands * This,
            /* [annotation][in] */ 
            _In_  ID3D12CommandQueue *pConsumerCommandQueue);
        
        END_INTERFACE
    } IMFD3D12SynchronizationObjectCommandsVtbl;

    interface IMFD3D12SynchronizationObjectCommands
    {
        CONST_VTBL struct IMFD3D12SynchronizationObjectCommandsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFD3D12SynchronizationObjectCommands_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFD3D12SynchronizationObjectCommands_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFD3D12SynchronizationObjectCommands_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFD3D12SynchronizationObjectCommands_EnqueueResourceReady(This,pProducerCommandQueue)	\
    ( (This)->lpVtbl -> EnqueueResourceReady(This,pProducerCommandQueue) ) 

#define IMFD3D12SynchronizationObjectCommands_EnqueueResourceReadyWait(This,pConsumerCommandQueue)	\
    ( (This)->lpVtbl -> EnqueueResourceReadyWait(This,pConsumerCommandQueue) ) 

#define IMFD3D12SynchronizationObjectCommands_SignalEventOnResourceReady(This,hEvent)	\
    ( (This)->lpVtbl -> SignalEventOnResourceReady(This,hEvent) ) 

#define IMFD3D12SynchronizationObjectCommands_EnqueueResourceRelease(This,pConsumerCommandQueue)	\
    ( (This)->lpVtbl -> EnqueueResourceRelease(This,pConsumerCommandQueue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFD3D12SynchronizationObjectCommands_INTERFACE_DEFINED__ */


#ifndef __IMFD3D12SynchronizationObject_INTERFACE_DEFINED__
#define __IMFD3D12SynchronizationObject_INTERFACE_DEFINED__

/* interface IMFD3D12SynchronizationObject */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IMFD3D12SynchronizationObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("802302B0-82DE-45E1-B421-F19EE5BDAF23")
    IMFD3D12SynchronizationObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SignalEventOnFinalResourceRelease( 
            HANDLE hEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFD3D12SynchronizationObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFD3D12SynchronizationObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFD3D12SynchronizationObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFD3D12SynchronizationObject * This);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObject, SignalEventOnFinalResourceRelease)
        HRESULT ( STDMETHODCALLTYPE *SignalEventOnFinalResourceRelease )( 
            IMFD3D12SynchronizationObject * This,
            HANDLE hEvent);
        
        DECLSPEC_XFGVIRT(IMFD3D12SynchronizationObject, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IMFD3D12SynchronizationObject * This);
        
        END_INTERFACE
    } IMFD3D12SynchronizationObjectVtbl;

    interface IMFD3D12SynchronizationObject
    {
        CONST_VTBL struct IMFD3D12SynchronizationObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFD3D12SynchronizationObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFD3D12SynchronizationObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFD3D12SynchronizationObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFD3D12SynchronizationObject_SignalEventOnFinalResourceRelease(This,hEvent)	\
    ( (This)->lpVtbl -> SignalEventOnFinalResourceRelease(This,hEvent) ) 

#define IMFD3D12SynchronizationObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFD3D12SynchronizationObject_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfd3d12_0000_0002 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
HRESULT MFCreateD3D12SynchronizationObject( 
    /* [annotation][in] */ 
    _In_  ID3D12Device *pDevice,
    /* [annotation][in] */ 
    _In_  REFIID riid,
    /* [annotation][out] */ 
    _Outptr_  void **ppvSyncObject);

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_CO) */
typedef 
enum MF_MT_D3D_RESOURCE_VERSION_ENUM
    {
        MF_D3D11_RESOURCE	= 0,
        MF_D3D12_RESOURCE	= ( MF_D3D11_RESOURCE + 1 ) 
    } 	MF_MT_D3D_RESOURCE_VERSION_ENUM;

EXTERN_GUID( MF_D3D12_SYNCHRONIZATION_OBJECT, 0x2a7c8d6a, 0x85a6, 0x494d, 0xa0, 0x46, 0x6, 0xea, 0x1a, 0x13, 0x8f, 0x4b );
EXTERN_GUID( MF_MT_D3D_RESOURCE_VERSION, 0x174f1e85, 0xfe26, 0x453d, 0xb5, 0x2e, 0x5b, 0xdd, 0x4e, 0x55, 0xb9, 0x44 );
EXTERN_GUID( MF_MT_D3D12_CPU_READBACK, 0x28ee9fe3, 0xd481, 0x46a6, 0xb9, 0x8a, 0x7f, 0x69, 0xd5, 0x28, 0xe, 0x82 );
EXTERN_GUID( MF_MT_D3D12_TEXTURE_LAYOUT, 0x97c85caa, 0xbeb, 0x4ee1, 0x97, 0x15, 0xf2, 0x2f, 0xad, 0x8c, 0x10, 0xf5 );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, 0xeeac2585, 0x3430, 0x498c, 0x84, 0xa2, 0x77, 0xb1, 0xbb, 0xa5, 0x70, 0xf6 );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL, 0xb1138dc3, 0x1d5, 0x4c14, 0x9b, 0xdc, 0xcd, 0xc9, 0x33, 0x6f, 0x55, 0xb9 );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS, 0x82c85647, 0x5057, 0x4960, 0x95, 0x59, 0xf4, 0x5b, 0x8e, 0x27, 0x14, 0x27 );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE, 0xba06bfac, 0xffe3, 0x474a, 0xab, 0x55, 0x16, 0x1e, 0xe4, 0x41, 0x7a, 0x2e );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER, 0xa6a1e439, 0x2f96, 0x4ab5, 0x98, 0xdc, 0xad, 0xf7, 0x49, 0x73, 0x50, 0x5d );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS, 0xa4940b2, 0xcfd6, 0x4738, 0x9d, 0x2, 0x98, 0x11, 0x37, 0x34, 0x1, 0x5a );
EXTERN_GUID( MF_MT_D3D12_RESOURCE_DIMENSION,     0x5f772624, 0x16ca, 0x4b89, 0x96, 0x51, 0x5d, 0xdf, 0x76, 0x9f, 0x8a, 0xb8);
EXTERN_GUID( MF_SA_D3D12_HEAP_FLAGS, 0x496b3266, 0xd28f, 0x4f8c, 0x93, 0xa7, 0x4a, 0x59, 0x6b, 0x1a, 0x31, 0xa1 );
EXTERN_GUID( MF_SA_D3D12_HEAP_TYPE, 0x56f26a76, 0xbbc1, 0x4ce0, 0xbb, 0x11, 0xe2, 0x23, 0x68, 0xd8, 0x74, 0xed );
EXTERN_GUID( MF_SA_D3D12_CLEAR_VALUE, 0x86ba9a39, 0x526, 0x495d, 0x9a, 0xb5, 0x54, 0xec, 0x9f, 0xad, 0x6f, 0xc3 );
EXTERN_GUID ( MF_SA_D3D12_AWARE,     0x77f0bacb, 0x17a8, 0x4a50, 0x9a, 0x7d, 0xa5, 0xcc, 0x9, 0xd3, 0x9d, 0x44);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfd3d12_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfd3d12_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


