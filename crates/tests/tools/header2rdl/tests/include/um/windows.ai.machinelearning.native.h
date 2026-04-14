

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* at Tue Jan 19 03:14:07 2038
 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __Windows2EAI2EMachineLearning2ENative_h__
#define __Windows2EAI2EMachineLearning2ENative_h__

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

#ifndef __ITensorNative_FWD_DEFINED__
#define __ITensorNative_FWD_DEFINED__
typedef interface ITensorNative ITensorNative;

#endif 	/* __ITensorNative_FWD_DEFINED__ */


#ifndef __ITensorStaticsNative_FWD_DEFINED__
#define __ITensorStaticsNative_FWD_DEFINED__
typedef interface ITensorStaticsNative ITensorStaticsNative;

#endif 	/* __ITensorStaticsNative_FWD_DEFINED__ */


#ifndef __ILearningModelDeviceFactoryNative_FWD_DEFINED__
#define __ILearningModelDeviceFactoryNative_FWD_DEFINED__
typedef interface ILearningModelDeviceFactoryNative ILearningModelDeviceFactoryNative;

#endif 	/* __ILearningModelDeviceFactoryNative_FWD_DEFINED__ */


#ifndef __ILearningModelSessionOptionsNative_FWD_DEFINED__
#define __ILearningModelSessionOptionsNative_FWD_DEFINED__
typedef interface ILearningModelSessionOptionsNative ILearningModelSessionOptionsNative;

#endif 	/* __ILearningModelSessionOptionsNative_FWD_DEFINED__ */


#ifndef __ILearningModelSessionOptionsNative1_FWD_DEFINED__
#define __ILearningModelSessionOptionsNative1_FWD_DEFINED__
typedef interface ILearningModelSessionOptionsNative1 ILearningModelSessionOptionsNative1;

#endif 	/* __ILearningModelSessionOptionsNative1_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "d3d12.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
struct IMLOperatorRegistry; 
struct __declspec(uuid("1adaa23a-eb67-41f3-aad8-5d984e9bacd4")) __declspec(novtable) ILearningModelOperatorProviderNative : IUnknown  
{                                                                                                                                       
    STDMETHOD(GetRegistry)(IMLOperatorRegistry** ppOperatorRegistry) PURE;                                                              
};                                                                                                                                      


extern RPC_IF_HANDLE __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0000_v0_0_s_ifspec;

#ifndef __ITensorNative_INTERFACE_DEFINED__
#define __ITensorNative_INTERFACE_DEFINED__

/* interface ITensorNative */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ITensorNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52f547ef-5b03-49b5-82d6-565f1ee0dd49")
    ITensorNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [size_is][size_is][out] */ BYTE **value,
            /* [out] */ UINT32 *capacity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetD3D12Resource( 
            /* [out] */ ID3D12Resource **result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITensorNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITensorNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITensorNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITensorNative * This);
        
        DECLSPEC_XFGVIRT(ITensorNative, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ITensorNative * This,
            /* [size_is][size_is][out] */ BYTE **value,
            /* [out] */ UINT32 *capacity);
        
        DECLSPEC_XFGVIRT(ITensorNative, GetD3D12Resource)
        HRESULT ( STDMETHODCALLTYPE *GetD3D12Resource )( 
            ITensorNative * This,
            /* [out] */ ID3D12Resource **result);
        
        END_INTERFACE
    } ITensorNativeVtbl;

    interface ITensorNative
    {
        CONST_VTBL struct ITensorNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITensorNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITensorNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITensorNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITensorNative_GetBuffer(This,value,capacity)	\
    ( (This)->lpVtbl -> GetBuffer(This,value,capacity) ) 

#define ITensorNative_GetD3D12Resource(This,result)	\
    ( (This)->lpVtbl -> GetD3D12Resource(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITensorNative_INTERFACE_DEFINED__ */


#ifndef __ITensorStaticsNative_INTERFACE_DEFINED__
#define __ITensorStaticsNative_INTERFACE_DEFINED__

/* interface ITensorStaticsNative */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ITensorStaticsNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39d055a4-66f6-4ebc-95d9-7a29ebe7690a")
    ITensorStaticsNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromD3D12Resource( 
            ID3D12Resource *value,
            /* [size_is] */ __int64 *shape,
            int shapeCount,
            /* [out] */ IUnknown **result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITensorStaticsNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITensorStaticsNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITensorStaticsNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITensorStaticsNative * This);
        
        DECLSPEC_XFGVIRT(ITensorStaticsNative, CreateFromD3D12Resource)
        HRESULT ( STDMETHODCALLTYPE *CreateFromD3D12Resource )( 
            ITensorStaticsNative * This,
            ID3D12Resource *value,
            /* [size_is] */ __int64 *shape,
            int shapeCount,
            /* [out] */ IUnknown **result);
        
        END_INTERFACE
    } ITensorStaticsNativeVtbl;

    interface ITensorStaticsNative
    {
        CONST_VTBL struct ITensorStaticsNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITensorStaticsNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITensorStaticsNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITensorStaticsNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITensorStaticsNative_CreateFromD3D12Resource(This,value,shape,shapeCount,result)	\
    ( (This)->lpVtbl -> CreateFromD3D12Resource(This,value,shape,shapeCount,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITensorStaticsNative_INTERFACE_DEFINED__ */


#ifndef __ILearningModelDeviceFactoryNative_INTERFACE_DEFINED__
#define __ILearningModelDeviceFactoryNative_INTERFACE_DEFINED__

/* interface ILearningModelDeviceFactoryNative */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ILearningModelDeviceFactoryNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1e9b31a1-662e-4ae0-af67-f63bb337e634")
    ILearningModelDeviceFactoryNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromD3D12CommandQueue( 
            ID3D12CommandQueue *value,
            /* [out] */ IUnknown **result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILearningModelDeviceFactoryNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ILearningModelDeviceFactoryNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ILearningModelDeviceFactoryNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ILearningModelDeviceFactoryNative * This);
        
        DECLSPEC_XFGVIRT(ILearningModelDeviceFactoryNative, CreateFromD3D12CommandQueue)
        HRESULT ( STDMETHODCALLTYPE *CreateFromD3D12CommandQueue )( 
            ILearningModelDeviceFactoryNative * This,
            ID3D12CommandQueue *value,
            /* [out] */ IUnknown **result);
        
        END_INTERFACE
    } ILearningModelDeviceFactoryNativeVtbl;

    interface ILearningModelDeviceFactoryNative
    {
        CONST_VTBL struct ILearningModelDeviceFactoryNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILearningModelDeviceFactoryNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILearningModelDeviceFactoryNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILearningModelDeviceFactoryNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILearningModelDeviceFactoryNative_CreateFromD3D12CommandQueue(This,value,result)	\
    ( (This)->lpVtbl -> CreateFromD3D12CommandQueue(This,value,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILearningModelDeviceFactoryNative_INTERFACE_DEFINED__ */


#ifndef __ILearningModelSessionOptionsNative_INTERFACE_DEFINED__
#define __ILearningModelSessionOptionsNative_INTERFACE_DEFINED__

/* interface ILearningModelSessionOptionsNative */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ILearningModelSessionOptionsNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c71e953f-37b4-4564-8658-d8396866db0d")
    ILearningModelSessionOptionsNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetIntraOpNumThreadsOverride( 
            UINT32 intraOpNumThreads) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILearningModelSessionOptionsNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ILearningModelSessionOptionsNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ILearningModelSessionOptionsNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ILearningModelSessionOptionsNative * This);
        
        DECLSPEC_XFGVIRT(ILearningModelSessionOptionsNative, SetIntraOpNumThreadsOverride)
        HRESULT ( STDMETHODCALLTYPE *SetIntraOpNumThreadsOverride )( 
            ILearningModelSessionOptionsNative * This,
            UINT32 intraOpNumThreads);
        
        END_INTERFACE
    } ILearningModelSessionOptionsNativeVtbl;

    interface ILearningModelSessionOptionsNative
    {
        CONST_VTBL struct ILearningModelSessionOptionsNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILearningModelSessionOptionsNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILearningModelSessionOptionsNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILearningModelSessionOptionsNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILearningModelSessionOptionsNative_SetIntraOpNumThreadsOverride(This,intraOpNumThreads)	\
    ( (This)->lpVtbl -> SetIntraOpNumThreadsOverride(This,intraOpNumThreads) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILearningModelSessionOptionsNative_INTERFACE_DEFINED__ */


#ifndef __ILearningModelSessionOptionsNative1_INTERFACE_DEFINED__
#define __ILearningModelSessionOptionsNative1_INTERFACE_DEFINED__

/* interface ILearningModelSessionOptionsNative1 */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ILearningModelSessionOptionsNative1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5da37a26-0526-414b-91e4-2a0fa3ddba40")
    ILearningModelSessionOptionsNative1 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetIntraOpThreadSpinning( 
            boolean allowSpinning) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILearningModelSessionOptionsNative1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ILearningModelSessionOptionsNative1 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ILearningModelSessionOptionsNative1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ILearningModelSessionOptionsNative1 * This);
        
        DECLSPEC_XFGVIRT(ILearningModelSessionOptionsNative1, SetIntraOpThreadSpinning)
        HRESULT ( STDMETHODCALLTYPE *SetIntraOpThreadSpinning )( 
            ILearningModelSessionOptionsNative1 * This,
            boolean allowSpinning);
        
        END_INTERFACE
    } ILearningModelSessionOptionsNative1Vtbl;

    interface ILearningModelSessionOptionsNative1
    {
        CONST_VTBL struct ILearningModelSessionOptionsNative1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILearningModelSessionOptionsNative1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILearningModelSessionOptionsNative1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILearningModelSessionOptionsNative1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILearningModelSessionOptionsNative1_SetIntraOpThreadSpinning(This,allowSpinning)	\
    ( (This)->lpVtbl -> SetIntraOpThreadSpinning(This,allowSpinning) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILearningModelSessionOptionsNative1_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */


extern RPC_IF_HANDLE __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eai2Emachinelearning2Enative_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


