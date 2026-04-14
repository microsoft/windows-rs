

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

#ifndef __ddpdataport_h__
#define __ddpdataport_h__

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

#ifndef __IDedupDataPort_FWD_DEFINED__
#define __IDedupDataPort_FWD_DEFINED__
typedef interface IDedupDataPort IDedupDataPort;

#endif 	/* __IDedupDataPort_FWD_DEFINED__ */


#ifndef __IDedupDataPortManager_FWD_DEFINED__
#define __IDedupDataPortManager_FWD_DEFINED__
typedef interface IDedupDataPortManager IDedupDataPortManager;

#endif 	/* __IDedupDataPortManager_FWD_DEFINED__ */


#ifndef __IDedupDataPort_FWD_DEFINED__
#define __IDedupDataPort_FWD_DEFINED__
typedef interface IDedupDataPort IDedupDataPort;

#endif 	/* __IDedupDataPort_FWD_DEFINED__ */


#ifndef __IDedupDataPortManager_FWD_DEFINED__
#define __IDedupDataPortManager_FWD_DEFINED__
typedef interface IDedupDataPortManager IDedupDataPortManager;

#endif 	/* __IDedupDataPortManager_FWD_DEFINED__ */


#ifndef __DedupDataPort_FWD_DEFINED__
#define __DedupDataPort_FWD_DEFINED__

#ifdef __cplusplus
typedef class DedupDataPort DedupDataPort;
#else
typedef struct DedupDataPort DedupDataPort;
#endif /* __cplusplus */

#endif 	/* __DedupDataPort_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "ddpcommon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ddpdataport_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_ddpdataport_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpdataport_0000_0000_v0_0_s_ifspec;

#ifndef __IDedupDataPort_INTERFACE_DEFINED__
#define __IDedupDataPort_INTERFACE_DEFINED__

/* interface IDedupDataPort */
/* [version][object][uuid] */ 


EXTERN_C const IID IID_IDedupDataPort;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7963d734-40a9-4ea3-bbf6-5a89d26f7ae8")
    IDedupDataPort : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DedupDataPortVolumeStatus *pStatus,
            /* [optional][out] */ __RPC__out DWORD *pDataHeadroomMb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupChunks( 
            /* [in] */ DWORD Count,
            /* [size_is][in] */ __RPC__in_ecount_full(Count) DedupHash *pHashes,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertChunks( 
            /* [in] */ DWORD ChunkCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ChunkCount) DedupChunk *pChunkMetadata,
            /* [in] */ DWORD DataByteCount,
            /* [size_is][in] */ __RPC__in_ecount_full(DataByteCount) BYTE *pChunkData,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertChunksWithStream( 
            /* [in] */ DWORD ChunkCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ChunkCount) DedupChunk *pChunkMetadata,
            /* [in] */ DWORD DataByteCount,
            /* [in] */ __RPC__in_opt IStream *pChunkDataStream,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitStreams( 
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) DedupStream *pStreams,
            /* [in] */ DWORD EntryCount,
            /* [size_is][in] */ __RPC__in_ecount_full(EntryCount) DedupStreamEntry *pEntries,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitStreamsWithStream( 
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) DedupStream *pStreams,
            /* [in] */ DWORD EntryCount,
            /* [in] */ __RPC__in_opt IStream *pEntriesStream,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreams( 
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) BSTR *pStreamPaths,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamsResults( 
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [in] */ DWORD StreamEntryIndex,
            /* [out] */ __RPC__out DWORD *pStreamCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pStreamCount) DedupStream **ppStreams,
            /* [out] */ __RPC__out DWORD *pEntryCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pEntryCount) DedupStreamEntry **ppEntries,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pStreamCount) HRESULT **ppItemResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChunks( 
            /* [in] */ DWORD Count,
            /* [size_is][in] */ __RPC__in_ecount_full(Count) DedupHash *pHashes,
            /* [out] */ __RPC__out GUID *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChunksResults( 
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [in] */ DWORD ChunkIndex,
            /* [out] */ __RPC__out DWORD *pChunkCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pChunkCount) DedupChunk **ppChunkMetadata,
            /* [out] */ __RPC__out DWORD *pDataByteCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pDataByteCount) BYTE **ppChunkData,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pChunkCount) HRESULT **ppItemResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestStatus( 
            /* [in] */ GUID RequestId,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestResults( 
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [out] */ __RPC__out HRESULT *pBatchResult,
            /* [out] */ __RPC__out DWORD *pBatchCount,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pBatchCount) HRESULT **ppItemResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupDataPortVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupDataPort * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupDataPort * This);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IDedupDataPort * This,
            /* [out] */ __RPC__out DedupDataPortVolumeStatus *pStatus,
            /* [optional][out] */ __RPC__out DWORD *pDataHeadroomMb);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, LookupChunks)
        HRESULT ( STDMETHODCALLTYPE *LookupChunks )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD Count,
            /* [size_is][in] */ __RPC__in_ecount_full(Count) DedupHash *pHashes,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, InsertChunks)
        HRESULT ( STDMETHODCALLTYPE *InsertChunks )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD ChunkCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ChunkCount) DedupChunk *pChunkMetadata,
            /* [in] */ DWORD DataByteCount,
            /* [size_is][in] */ __RPC__in_ecount_full(DataByteCount) BYTE *pChunkData,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, InsertChunksWithStream)
        HRESULT ( STDMETHODCALLTYPE *InsertChunksWithStream )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD ChunkCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ChunkCount) DedupChunk *pChunkMetadata,
            /* [in] */ DWORD DataByteCount,
            /* [in] */ __RPC__in_opt IStream *pChunkDataStream,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, CommitStreams)
        HRESULT ( STDMETHODCALLTYPE *CommitStreams )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) DedupStream *pStreams,
            /* [in] */ DWORD EntryCount,
            /* [size_is][in] */ __RPC__in_ecount_full(EntryCount) DedupStreamEntry *pEntries,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, CommitStreamsWithStream)
        HRESULT ( STDMETHODCALLTYPE *CommitStreamsWithStream )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) DedupStream *pStreams,
            /* [in] */ DWORD EntryCount,
            /* [in] */ __RPC__in_opt IStream *pEntriesStream,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetStreams)
        HRESULT ( STDMETHODCALLTYPE *GetStreams )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD StreamCount,
            /* [size_is][in] */ __RPC__in_ecount_full(StreamCount) BSTR *pStreamPaths,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetStreamsResults)
        HRESULT ( STDMETHODCALLTYPE *GetStreamsResults )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [in] */ DWORD StreamEntryIndex,
            /* [out] */ __RPC__out DWORD *pStreamCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pStreamCount) DedupStream **ppStreams,
            /* [out] */ __RPC__out DWORD *pEntryCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pEntryCount) DedupStreamEntry **ppEntries,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pStreamCount) HRESULT **ppItemResults);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetChunks)
        HRESULT ( STDMETHODCALLTYPE *GetChunks )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ DWORD Count,
            /* [size_is][in] */ __RPC__in_ecount_full(Count) DedupHash *pHashes,
            /* [out] */ __RPC__out GUID *pRequestId);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetChunksResults)
        HRESULT ( STDMETHODCALLTYPE *GetChunksResults )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [in] */ DWORD ChunkIndex,
            /* [out] */ __RPC__out DWORD *pChunkCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pChunkCount) DedupChunk **ppChunkMetadata,
            /* [out] */ __RPC__out DWORD *pDataByteCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pDataByteCount) BYTE **ppChunkData,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pChunkCount) HRESULT **ppItemResults);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetRequestStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRequestStatus )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ GUID RequestId,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IDedupDataPort, GetRequestResults)
        HRESULT ( STDMETHODCALLTYPE *GetRequestResults )( 
            __RPC__in IDedupDataPort * This,
            /* [in] */ GUID RequestId,
            /* [in] */ DWORD MaxWaitMs,
            /* [out] */ __RPC__out HRESULT *pBatchResult,
            /* [out] */ __RPC__out DWORD *pBatchCount,
            /* [out] */ __RPC__out DedupDataPortRequestStatus *pStatus,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pBatchCount) HRESULT **ppItemResults);
        
        END_INTERFACE
    } IDedupDataPortVtbl;

    interface IDedupDataPort
    {
        CONST_VTBL struct IDedupDataPortVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupDataPort_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupDataPort_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupDataPort_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupDataPort_GetStatus(This,pStatus,pDataHeadroomMb)	\
    ( (This)->lpVtbl -> GetStatus(This,pStatus,pDataHeadroomMb) ) 

#define IDedupDataPort_LookupChunks(This,Count,pHashes,pRequestId)	\
    ( (This)->lpVtbl -> LookupChunks(This,Count,pHashes,pRequestId) ) 

#define IDedupDataPort_InsertChunks(This,ChunkCount,pChunkMetadata,DataByteCount,pChunkData,pRequestId)	\
    ( (This)->lpVtbl -> InsertChunks(This,ChunkCount,pChunkMetadata,DataByteCount,pChunkData,pRequestId) ) 

#define IDedupDataPort_InsertChunksWithStream(This,ChunkCount,pChunkMetadata,DataByteCount,pChunkDataStream,pRequestId)	\
    ( (This)->lpVtbl -> InsertChunksWithStream(This,ChunkCount,pChunkMetadata,DataByteCount,pChunkDataStream,pRequestId) ) 

#define IDedupDataPort_CommitStreams(This,StreamCount,pStreams,EntryCount,pEntries,pRequestId)	\
    ( (This)->lpVtbl -> CommitStreams(This,StreamCount,pStreams,EntryCount,pEntries,pRequestId) ) 

#define IDedupDataPort_CommitStreamsWithStream(This,StreamCount,pStreams,EntryCount,pEntriesStream,pRequestId)	\
    ( (This)->lpVtbl -> CommitStreamsWithStream(This,StreamCount,pStreams,EntryCount,pEntriesStream,pRequestId) ) 

#define IDedupDataPort_GetStreams(This,StreamCount,pStreamPaths,pRequestId)	\
    ( (This)->lpVtbl -> GetStreams(This,StreamCount,pStreamPaths,pRequestId) ) 

#define IDedupDataPort_GetStreamsResults(This,RequestId,MaxWaitMs,StreamEntryIndex,pStreamCount,ppStreams,pEntryCount,ppEntries,pStatus,ppItemResults)	\
    ( (This)->lpVtbl -> GetStreamsResults(This,RequestId,MaxWaitMs,StreamEntryIndex,pStreamCount,ppStreams,pEntryCount,ppEntries,pStatus,ppItemResults) ) 

#define IDedupDataPort_GetChunks(This,Count,pHashes,pRequestId)	\
    ( (This)->lpVtbl -> GetChunks(This,Count,pHashes,pRequestId) ) 

#define IDedupDataPort_GetChunksResults(This,RequestId,MaxWaitMs,ChunkIndex,pChunkCount,ppChunkMetadata,pDataByteCount,ppChunkData,pStatus,ppItemResults)	\
    ( (This)->lpVtbl -> GetChunksResults(This,RequestId,MaxWaitMs,ChunkIndex,pChunkCount,ppChunkMetadata,pDataByteCount,ppChunkData,pStatus,ppItemResults) ) 

#define IDedupDataPort_GetRequestStatus(This,RequestId,pStatus)	\
    ( (This)->lpVtbl -> GetRequestStatus(This,RequestId,pStatus) ) 

#define IDedupDataPort_GetRequestResults(This,RequestId,MaxWaitMs,pBatchResult,pBatchCount,pStatus,ppItemResults)	\
    ( (This)->lpVtbl -> GetRequestResults(This,RequestId,MaxWaitMs,pBatchResult,pBatchCount,pStatus,ppItemResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupDataPort_INTERFACE_DEFINED__ */


#ifndef __IDedupDataPortManager_INTERFACE_DEFINED__
#define __IDedupDataPortManager_INTERFACE_DEFINED__

/* interface IDedupDataPortManager */
/* [version][object][uuid] */ 


EXTERN_C const IID IID_IDedupDataPortManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("44677452-b90a-445e-8192-cdcfe81511fb")
    IDedupDataPortManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetConfiguration( 
            /* [out] */ __RPC__out DWORD *pMinChunkSize,
            /* [out] */ __RPC__out DWORD *pMaxChunkSize,
            /* [out] */ __RPC__out DedupChunkingAlgorithm *pChunkingAlgorithm,
            /* [out] */ __RPC__out DedupHashingAlgorithm *pHashingAlgorithm,
            /* [out] */ __RPC__out DedupCompressionAlgorithm *pCompressionAlgorithm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVolumeStatus( 
            /* [in] */ DWORD Options,
            /* [in] */ __RPC__in BSTR Path,
            /* [out] */ __RPC__out DedupDataPortVolumeStatus *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVolumeDataPort( 
            /* [in] */ DWORD Options,
            /* [in] */ __RPC__in BSTR Path,
            /* [out] */ __RPC__deref_out_opt IDedupDataPort **ppDataPort) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupDataPortManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupDataPortManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupDataPortManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupDataPortManager * This);
        
        DECLSPEC_XFGVIRT(IDedupDataPortManager, GetConfiguration)
        HRESULT ( STDMETHODCALLTYPE *GetConfiguration )( 
            __RPC__in IDedupDataPortManager * This,
            /* [out] */ __RPC__out DWORD *pMinChunkSize,
            /* [out] */ __RPC__out DWORD *pMaxChunkSize,
            /* [out] */ __RPC__out DedupChunkingAlgorithm *pChunkingAlgorithm,
            /* [out] */ __RPC__out DedupHashingAlgorithm *pHashingAlgorithm,
            /* [out] */ __RPC__out DedupCompressionAlgorithm *pCompressionAlgorithm);
        
        DECLSPEC_XFGVIRT(IDedupDataPortManager, GetVolumeStatus)
        HRESULT ( STDMETHODCALLTYPE *GetVolumeStatus )( 
            __RPC__in IDedupDataPortManager * This,
            /* [in] */ DWORD Options,
            /* [in] */ __RPC__in BSTR Path,
            /* [out] */ __RPC__out DedupDataPortVolumeStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IDedupDataPortManager, GetVolumeDataPort)
        HRESULT ( STDMETHODCALLTYPE *GetVolumeDataPort )( 
            __RPC__in IDedupDataPortManager * This,
            /* [in] */ DWORD Options,
            /* [in] */ __RPC__in BSTR Path,
            /* [out] */ __RPC__deref_out_opt IDedupDataPort **ppDataPort);
        
        END_INTERFACE
    } IDedupDataPortManagerVtbl;

    interface IDedupDataPortManager
    {
        CONST_VTBL struct IDedupDataPortManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupDataPortManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupDataPortManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupDataPortManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupDataPortManager_GetConfiguration(This,pMinChunkSize,pMaxChunkSize,pChunkingAlgorithm,pHashingAlgorithm,pCompressionAlgorithm)	\
    ( (This)->lpVtbl -> GetConfiguration(This,pMinChunkSize,pMaxChunkSize,pChunkingAlgorithm,pHashingAlgorithm,pCompressionAlgorithm) ) 

#define IDedupDataPortManager_GetVolumeStatus(This,Options,Path,pStatus)	\
    ( (This)->lpVtbl -> GetVolumeStatus(This,Options,Path,pStatus) ) 

#define IDedupDataPortManager_GetVolumeDataPort(This,Options,Path,ppDataPort)	\
    ( (This)->lpVtbl -> GetVolumeDataPort(This,Options,Path,ppDataPort) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupDataPortManager_INTERFACE_DEFINED__ */



#ifndef __DedupDataPort_LIBRARY_DEFINED__
#define __DedupDataPort_LIBRARY_DEFINED__

/* library DedupDataPort */
/* [version][uuid] */ 




EXTERN_C const IID LIBID_DedupDataPort;

EXTERN_C const CLSID CLSID_DedupDataPort;

#ifdef __cplusplus

class DECLSPEC_UUID("8f107207-1829-48b2-a64b-e61f8e0d9acb")
DedupDataPort;
#endif
#endif /* __DedupDataPort_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_ddpdataport_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ddpdataport_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpdataport_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


