

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

#ifndef __mmstream_h__
#define __mmstream_h__

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

#ifndef __IMultiMediaStream_FWD_DEFINED__
#define __IMultiMediaStream_FWD_DEFINED__
typedef interface IMultiMediaStream IMultiMediaStream;

#endif 	/* __IMultiMediaStream_FWD_DEFINED__ */


#ifndef __IMediaStream_FWD_DEFINED__
#define __IMediaStream_FWD_DEFINED__
typedef interface IMediaStream IMediaStream;

#endif 	/* __IMediaStream_FWD_DEFINED__ */


#ifndef __IStreamSample_FWD_DEFINED__
#define __IStreamSample_FWD_DEFINED__
typedef interface IStreamSample IStreamSample;

#endif 	/* __IStreamSample_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mmstream_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define MS_ERROR_CODE(x) MAKE_HRESULT(1, FACILITY_ITF, (x) + 0x400)
#define MS_SUCCESS_CODE(x) MAKE_HRESULT(0, FACILITY_ITF, x)
#define MS_S_PENDING                  MS_SUCCESS_CODE(1)
#define MS_S_NOUPDATE                 MS_SUCCESS_CODE(2)
#define MS_S_ENDOFSTREAM              MS_SUCCESS_CODE(3)
#define MS_E_SAMPLEALLOC              MS_ERROR_CODE(1)
#define MS_E_PURPOSEID                MS_ERROR_CODE(2)
#define MS_E_NOSTREAM                 MS_ERROR_CODE(3)
#define MS_E_NOSEEKING                MS_ERROR_CODE(4)
#define MS_E_INCOMPATIBLE             MS_ERROR_CODE(5)
#define MS_E_BUSY                     MS_ERROR_CODE(6)
#define MS_E_NOTINIT                  MS_ERROR_CODE(7)
#define MS_E_SOURCEALREADYDEFINED     MS_ERROR_CODE(8)
#define MS_E_INVALIDSTREAMTYPE        MS_ERROR_CODE(9)
#define MS_E_NOTRUNNING               MS_ERROR_CODE(10)
// {A35FF56A-9FDA-11d0-8FDF-00C04FD9189D}
DEFINE_GUID(MSPID_PrimaryVideo, 
0xa35ff56a, 0x9fda, 0x11d0, 0x8f, 0xdf, 0x0, 0xc0, 0x4f, 0xd9, 0x18, 0x9d);
// {A35FF56B-9FDA-11d0-8FDF-00C04FD9189D}
DEFINE_GUID(MSPID_PrimaryAudio,
0xa35ff56b, 0x9fda, 0x11d0, 0x8f, 0xdf, 0x0, 0xc0, 0x4f, 0xd9, 0x18, 0x9d);
#if(_WIN32_WINNT < 0x0400)
typedef void ( __stdcall *PAPCFUNC )( 
    DWORD_PTR dwParam);

#endif
typedef LONGLONG STREAM_TIME;

typedef GUID MSPID;

typedef REFGUID REFMSPID;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_mmstream_0000_0000_0001
    {
        STREAMTYPE_READ	= 0,
        STREAMTYPE_WRITE	= 1,
        STREAMTYPE_TRANSFORM	= 2
    } 	STREAM_TYPE;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_mmstream_0000_0000_0002
    {
        STREAMSTATE_STOP	= 0,
        STREAMSTATE_RUN	= 1
    } 	STREAM_STATE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mmstream_0000_0000_0003
    {
        COMPSTAT_NOUPDATEOK	= 0x1,
        COMPSTAT_WAIT	= 0x2,
        COMPSTAT_ABORT	= 0x4
    } 	COMPLETION_STATUS_FLAGS;


enum __MIDL___MIDL_itf_mmstream_0000_0000_0004
    {
        MMSSF_HASCLOCK	= 0x1,
        MMSSF_SUPPORTSEEK	= 0x2,
        MMSSF_ASYNCHRONOUS	= 0x4
    } ;

enum __MIDL___MIDL_itf_mmstream_0000_0000_0005
    {
        SSUPDATE_ASYNC	= 0x1,
        SSUPDATE_CONTINUOUS	= 0x2
    } ;





extern RPC_IF_HANDLE __MIDL_itf_mmstream_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmstream_0000_0000_v0_0_s_ifspec;

#ifndef __IMultiMediaStream_INTERFACE_DEFINED__
#define __IMultiMediaStream_INTERFACE_DEFINED__

/* interface IMultiMediaStream */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMultiMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B502D1BC-9A57-11d0-8FDE-00C04FD9189D")
    IMultiMediaStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInformation( 
            /* [out] */ DWORD *pdwFlags,
            /* [out] */ STREAM_TYPE *pStreamType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaStream( 
            /* [in] */ REFMSPID idPurpose,
            /* [out] */ IMediaStream **ppMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMediaStreams( 
            /* [in] */ long Index,
            /* [out] */ IMediaStream **ppMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [out] */ STREAM_STATE *pCurrentState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetState( 
            /* [in] */ STREAM_STATE NewState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [out] */ STREAM_TIME *pCurrentTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [out] */ STREAM_TIME *pDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ STREAM_TIME SeekTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndOfStreamEventHandle( 
            /* [out] */ HANDLE *phEOS) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMultiMediaStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMultiMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMultiMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            IMultiMediaStream * This,
            /* [out] */ DWORD *pdwFlags,
            /* [out] */ STREAM_TYPE *pStreamType);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IMultiMediaStream * This,
            /* [in] */ REFMSPID idPurpose,
            /* [out] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, EnumMediaStreams)
        HRESULT ( STDMETHODCALLTYPE *EnumMediaStreams )( 
            IMultiMediaStream * This,
            /* [in] */ long Index,
            /* [out] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IMultiMediaStream * This,
            /* [out] */ STREAM_STATE *pCurrentState);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            IMultiMediaStream * This,
            /* [in] */ STREAM_STATE NewState);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IMultiMediaStream * This,
            /* [out] */ STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetDuration)
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IMultiMediaStream * This,
            /* [out] */ STREAM_TIME *pDuration);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IMultiMediaStream * This,
            /* [in] */ STREAM_TIME SeekTime);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetEndOfStreamEventHandle)
        HRESULT ( STDMETHODCALLTYPE *GetEndOfStreamEventHandle )( 
            IMultiMediaStream * This,
            /* [out] */ HANDLE *phEOS);
        
        END_INTERFACE
    } IMultiMediaStreamVtbl;

    interface IMultiMediaStream
    {
        CONST_VTBL struct IMultiMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiMediaStream_GetInformation(This,pdwFlags,pStreamType)	\
    ( (This)->lpVtbl -> GetInformation(This,pdwFlags,pStreamType) ) 

#define IMultiMediaStream_GetMediaStream(This,idPurpose,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,idPurpose,ppMediaStream) ) 

#define IMultiMediaStream_EnumMediaStreams(This,Index,ppMediaStream)	\
    ( (This)->lpVtbl -> EnumMediaStreams(This,Index,ppMediaStream) ) 

#define IMultiMediaStream_GetState(This,pCurrentState)	\
    ( (This)->lpVtbl -> GetState(This,pCurrentState) ) 

#define IMultiMediaStream_SetState(This,NewState)	\
    ( (This)->lpVtbl -> SetState(This,NewState) ) 

#define IMultiMediaStream_GetTime(This,pCurrentTime)	\
    ( (This)->lpVtbl -> GetTime(This,pCurrentTime) ) 

#define IMultiMediaStream_GetDuration(This,pDuration)	\
    ( (This)->lpVtbl -> GetDuration(This,pDuration) ) 

#define IMultiMediaStream_Seek(This,SeekTime)	\
    ( (This)->lpVtbl -> Seek(This,SeekTime) ) 

#define IMultiMediaStream_GetEndOfStreamEventHandle(This,phEOS)	\
    ( (This)->lpVtbl -> GetEndOfStreamEventHandle(This,phEOS) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiMediaStream_INTERFACE_DEFINED__ */


#ifndef __IMediaStream_INTERFACE_DEFINED__
#define __IMediaStream_INTERFACE_DEFINED__

/* interface IMediaStream */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B502D1BD-9A57-11d0-8FDE-00C04FD9189D")
    IMediaStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMultiMediaStream( 
            /* [out] */ __RPC__deref_out_opt IMultiMediaStream **ppMultiMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInformation( 
            /* [out] */ __RPC__out MSPID *pPurposeId,
            /* [out] */ __RPC__out STREAM_TYPE *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSameFormat( 
            /* [in] */ __RPC__in_opt IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocateSample( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSharedSample( 
            /* [in] */ __RPC__in_opt IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppNewSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendEndOfStream( 
            DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMultiMediaStream )( 
            __RPC__in IMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IMultiMediaStream **ppMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            __RPC__in IMediaStream * This,
            /* [out] */ __RPC__out MSPID *pPurposeId,
            /* [out] */ __RPC__out STREAM_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMediaStream, SetSameFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSameFormat )( 
            __RPC__in IMediaStream * This,
            /* [in] */ __RPC__in_opt IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaStream, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            __RPC__in IMediaStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, CreateSharedSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedSample )( 
            __RPC__in IMediaStream * This,
            /* [in] */ __RPC__in_opt IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppNewSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, SendEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SendEndOfStream )( 
            __RPC__in IMediaStream * This,
            DWORD dwFlags);
        
        END_INTERFACE
    } IMediaStreamVtbl;

    interface IMediaStream
    {
        CONST_VTBL struct IMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaStream_GetMultiMediaStream(This,ppMultiMediaStream)	\
    ( (This)->lpVtbl -> GetMultiMediaStream(This,ppMultiMediaStream) ) 

#define IMediaStream_GetInformation(This,pPurposeId,pType)	\
    ( (This)->lpVtbl -> GetInformation(This,pPurposeId,pType) ) 

#define IMediaStream_SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags)	\
    ( (This)->lpVtbl -> SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags) ) 

#define IMediaStream_AllocateSample(This,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,dwFlags,ppSample) ) 

#define IMediaStream_CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample)	\
    ( (This)->lpVtbl -> CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample) ) 

#define IMediaStream_SendEndOfStream(This,dwFlags)	\
    ( (This)->lpVtbl -> SendEndOfStream(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaStream_INTERFACE_DEFINED__ */


#ifndef __IStreamSample_INTERFACE_DEFINED__
#define __IStreamSample_INTERFACE_DEFINED__

/* interface IStreamSample */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IStreamSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B502D1BE-9A57-11d0-8FDE-00C04FD9189D")
    IStreamSample : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMediaStream( 
            /* [in] */ IMediaStream **ppMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSampleTimes( 
            /* [out] */ STREAM_TIME *pStartTime,
            /* [out] */ STREAM_TIME *pEndTime,
            /* [out] */ STREAM_TIME *pCurrentTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleTimes( 
            /* [in] */ const STREAM_TIME *pStartTime,
            /* [in] */ const STREAM_TIME *pEndTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hEvent,
            /* [in] */ PAPCFUNC pfnAPC,
            /* [in] */ DWORD_PTR dwAPCData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompletionStatus( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStreamSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStreamSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStreamSample * This);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IStreamSample * This,
            /* [in] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *GetSampleTimes )( 
            IStreamSample * This,
            /* [out] */ STREAM_TIME *pStartTime,
            /* [out] */ STREAM_TIME *pEndTime,
            /* [out] */ STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, SetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *SetSampleTimes )( 
            IStreamSample * This,
            /* [in] */ const STREAM_TIME *pStartTime,
            /* [in] */ const STREAM_TIME *pEndTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hEvent,
            /* [in] */ PAPCFUNC pfnAPC,
            /* [in] */ DWORD_PTR dwAPCData);
        
        DECLSPEC_XFGVIRT(IStreamSample, CompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *CompletionStatus )( 
            IStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        END_INTERFACE
    } IStreamSampleVtbl;

    interface IStreamSample
    {
        CONST_VTBL struct IStreamSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamSample_GetMediaStream(This,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,ppMediaStream) ) 

#define IStreamSample_GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime)	\
    ( (This)->lpVtbl -> GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime) ) 

#define IStreamSample_SetSampleTimes(This,pStartTime,pEndTime)	\
    ( (This)->lpVtbl -> SetSampleTimes(This,pStartTime,pEndTime) ) 

#define IStreamSample_Update(This,dwFlags,hEvent,pfnAPC,dwAPCData)	\
    ( (This)->lpVtbl -> Update(This,dwFlags,hEvent,pfnAPC,dwAPCData) ) 

#define IStreamSample_CompletionStatus(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> CompletionStatus(This,dwFlags,dwMilliseconds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamSample_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmstream_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mmstream_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmstream_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


