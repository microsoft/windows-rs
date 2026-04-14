

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

#ifndef __ddpchunk_h__
#define __ddpchunk_h__

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

#ifndef __IDedupChunkLibrary_FWD_DEFINED__
#define __IDedupChunkLibrary_FWD_DEFINED__
typedef interface IDedupChunkLibrary IDedupChunkLibrary;

#endif 	/* __IDedupChunkLibrary_FWD_DEFINED__ */


#ifndef __IDedupIterateChunksHash32_FWD_DEFINED__
#define __IDedupIterateChunksHash32_FWD_DEFINED__
typedef interface IDedupIterateChunksHash32 IDedupIterateChunksHash32;

#endif 	/* __IDedupIterateChunksHash32_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ddpchunk_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


enum _DEDUP_SET_PARAM_TYPE
    {
        DEDUP_PT_MinChunkSizeBytes	= 1,
        DEDUP_PT_MaxChunkSizeBytes	= 2,
        DEDUP_PT_AvgChunkSizeBytes	= 3,
        DEDUP_PT_InvariantChunking	= 4,
        DEDUP_PT_DisableStrongHashComputation	= 5
    } ;
#define	DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM	( 1024 )

typedef struct _DEDUP_CHUNK_INFO_HASH32
    {
    DWORD ChunkFlags;
    ULONGLONG ChunkOffsetInStream;
    ULONGLONG ChunkSize;
    BYTE HashVal[ 32 ];
    } 	DEDUP_CHUNK_INFO_HASH32;



extern RPC_IF_HANDLE __MIDL_itf_ddpchunk_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpchunk_0000_0000_v0_0_s_ifspec;

#ifndef __IDedupChunkLibrary_INTERFACE_DEFINED__
#define __IDedupChunkLibrary_INTERFACE_DEFINED__

/* interface IDedupChunkLibrary */
/* [object][version][uuid] */ 


EXTERN_C const IID IID_IDedupChunkLibrary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB5144D7-2720-4DCC-8777-78597416EC23")
    IDedupChunkLibrary : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeForPushBuffers( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParameter( 
            /* [in] */ DWORD dwParamType,
            /* [in] */ VARIANT vParamValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartChunking( 
            /* [in] */ IID iidIteratorInterfaceID,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppChunksEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupChunkLibraryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupChunkLibrary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupChunkLibrary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupChunkLibrary * This);
        
        DECLSPEC_XFGVIRT(IDedupChunkLibrary, InitializeForPushBuffers)
        HRESULT ( STDMETHODCALLTYPE *InitializeForPushBuffers )( 
            __RPC__in IDedupChunkLibrary * This);
        
        DECLSPEC_XFGVIRT(IDedupChunkLibrary, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            __RPC__in IDedupChunkLibrary * This);
        
        DECLSPEC_XFGVIRT(IDedupChunkLibrary, SetParameter)
        HRESULT ( STDMETHODCALLTYPE *SetParameter )( 
            __RPC__in IDedupChunkLibrary * This,
            /* [in] */ DWORD dwParamType,
            /* [in] */ VARIANT vParamValue);
        
        DECLSPEC_XFGVIRT(IDedupChunkLibrary, StartChunking)
        HRESULT ( STDMETHODCALLTYPE *StartChunking )( 
            __RPC__in IDedupChunkLibrary * This,
            /* [in] */ IID iidIteratorInterfaceID,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppChunksEnum);
        
        END_INTERFACE
    } IDedupChunkLibraryVtbl;

    interface IDedupChunkLibrary
    {
        CONST_VTBL struct IDedupChunkLibraryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupChunkLibrary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupChunkLibrary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupChunkLibrary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupChunkLibrary_InitializeForPushBuffers(This)	\
    ( (This)->lpVtbl -> InitializeForPushBuffers(This) ) 

#define IDedupChunkLibrary_Uninitialize(This)	\
    ( (This)->lpVtbl -> Uninitialize(This) ) 

#define IDedupChunkLibrary_SetParameter(This,dwParamType,vParamValue)	\
    ( (This)->lpVtbl -> SetParameter(This,dwParamType,vParamValue) ) 

#define IDedupChunkLibrary_StartChunking(This,iidIteratorInterfaceID,ppChunksEnum)	\
    ( (This)->lpVtbl -> StartChunking(This,iidIteratorInterfaceID,ppChunksEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupChunkLibrary_INTERFACE_DEFINED__ */


#ifndef __IDedupIterateChunksHash32_INTERFACE_DEFINED__
#define __IDedupIterateChunksHash32_INTERFACE_DEFINED__

/* interface IDedupIterateChunksHash32 */
/* [object][version][uuid] */ 


EXTERN_C const IID IID_IDedupIterateChunksHash32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90B584D3-72AA-400F-9767-CAD866A5A2D8")
    IDedupIterateChunksHash32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PushBuffer( 
            /* [size_is][in] */ __RPC__in_ecount_full(ulBufferLength) BYTE *pBuffer,
            /* [in] */ ULONG ulBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [range][in] */ __RPC__in_range(0,DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM) ULONG ulMaxChunks,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulMaxChunks, *pulFetched) DEDUP_CHUNK_INFO_HASH32 *pArrChunks,
            /* [out] */ __RPC__out ULONG *pulFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Drain( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupIterateChunksHash32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupIterateChunksHash32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupIterateChunksHash32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupIterateChunksHash32 * This);
        
        DECLSPEC_XFGVIRT(IDedupIterateChunksHash32, PushBuffer)
        HRESULT ( STDMETHODCALLTYPE *PushBuffer )( 
            __RPC__in IDedupIterateChunksHash32 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(ulBufferLength) BYTE *pBuffer,
            /* [in] */ ULONG ulBufferLength);
        
        DECLSPEC_XFGVIRT(IDedupIterateChunksHash32, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IDedupIterateChunksHash32 * This,
            /* [range][in] */ __RPC__in_range(0,DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM) ULONG ulMaxChunks,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulMaxChunks, *pulFetched) DEDUP_CHUNK_INFO_HASH32 *pArrChunks,
            /* [out] */ __RPC__out ULONG *pulFetched);
        
        DECLSPEC_XFGVIRT(IDedupIterateChunksHash32, Drain)
        HRESULT ( STDMETHODCALLTYPE *Drain )( 
            __RPC__in IDedupIterateChunksHash32 * This);
        
        DECLSPEC_XFGVIRT(IDedupIterateChunksHash32, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IDedupIterateChunksHash32 * This);
        
        END_INTERFACE
    } IDedupIterateChunksHash32Vtbl;

    interface IDedupIterateChunksHash32
    {
        CONST_VTBL struct IDedupIterateChunksHash32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupIterateChunksHash32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupIterateChunksHash32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupIterateChunksHash32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupIterateChunksHash32_PushBuffer(This,pBuffer,ulBufferLength)	\
    ( (This)->lpVtbl -> PushBuffer(This,pBuffer,ulBufferLength) ) 

#define IDedupIterateChunksHash32_Next(This,ulMaxChunks,pArrChunks,pulFetched)	\
    ( (This)->lpVtbl -> Next(This,ulMaxChunks,pArrChunks,pulFetched) ) 

#define IDedupIterateChunksHash32_Drain(This)	\
    ( (This)->lpVtbl -> Drain(This) ) 

#define IDedupIterateChunksHash32_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupIterateChunksHash32_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ddpchunk_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ddpchunk_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpchunk_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


