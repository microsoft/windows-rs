

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

#ifndef __austream_h__
#define __austream_h__

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

#ifndef __IAudioMediaStream_FWD_DEFINED__
#define __IAudioMediaStream_FWD_DEFINED__
typedef interface IAudioMediaStream IAudioMediaStream;

#endif 	/* __IAudioMediaStream_FWD_DEFINED__ */


#ifndef __IAudioStreamSample_FWD_DEFINED__
#define __IAudioStreamSample_FWD_DEFINED__
typedef interface IAudioStreamSample IAudioStreamSample;

#endif 	/* __IAudioStreamSample_FWD_DEFINED__ */


#ifndef __IMemoryData_FWD_DEFINED__
#define __IMemoryData_FWD_DEFINED__
typedef interface IMemoryData IMemoryData;

#endif 	/* __IMemoryData_FWD_DEFINED__ */


#ifndef __IAudioData_FWD_DEFINED__
#define __IAudioData_FWD_DEFINED__
typedef interface IAudioData IAudioData;

#endif 	/* __IAudioData_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "mmstream.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_austream_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
//   The following declarations within the 'if 0' block are dummy typedefs used to make
//   the ddstream.idl file build.  The actual definitions are contained in DDRAW.H
//
#if 0
typedef struct tWAVEFORMATEX WAVEFORMATEX;

#endif






extern RPC_IF_HANDLE __MIDL_itf_austream_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_austream_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioMediaStream_INTERFACE_DEFINED__
#define __IAudioMediaStream_INTERFACE_DEFINED__

/* interface IAudioMediaStream */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7537560-a3be-11d0-8212-00c04fc32c45")
    IAudioMediaStream : public IMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [out] */ WAVEFORMATEX *pWaveFormatCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFormat( 
            /* [in] */ const WAVEFORMATEX *lpWaveFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSample( 
            /* [in] */ IAudioData *pAudioData,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IAudioStreamSample **ppSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioMediaStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMultiMediaStream )( 
            IAudioMediaStream * This,
            /* [out] */ IMultiMediaStream **ppMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            IAudioMediaStream * This,
            /* [out] */ MSPID *pPurposeId,
            /* [out] */ STREAM_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMediaStream, SetSameFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSameFormat )( 
            IAudioMediaStream * This,
            /* [in] */ IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaStream, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IAudioMediaStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, CreateSharedSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedSample )( 
            IAudioMediaStream * This,
            /* [in] */ IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppNewSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, SendEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SendEndOfStream )( 
            IAudioMediaStream * This,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAudioMediaStream, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IAudioMediaStream * This,
            /* [out] */ WAVEFORMATEX *pWaveFormatCurrent);
        
        DECLSPEC_XFGVIRT(IAudioMediaStream, SetFormat)
        HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            IAudioMediaStream * This,
            /* [in] */ const WAVEFORMATEX *lpWaveFormat);
        
        DECLSPEC_XFGVIRT(IAudioMediaStream, CreateSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSample )( 
            IAudioMediaStream * This,
            /* [in] */ IAudioData *pAudioData,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IAudioStreamSample **ppSample);
        
        END_INTERFACE
    } IAudioMediaStreamVtbl;

    interface IAudioMediaStream
    {
        CONST_VTBL struct IAudioMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioMediaStream_GetMultiMediaStream(This,ppMultiMediaStream)	\
    ( (This)->lpVtbl -> GetMultiMediaStream(This,ppMultiMediaStream) ) 

#define IAudioMediaStream_GetInformation(This,pPurposeId,pType)	\
    ( (This)->lpVtbl -> GetInformation(This,pPurposeId,pType) ) 

#define IAudioMediaStream_SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags)	\
    ( (This)->lpVtbl -> SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags) ) 

#define IAudioMediaStream_AllocateSample(This,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,dwFlags,ppSample) ) 

#define IAudioMediaStream_CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample)	\
    ( (This)->lpVtbl -> CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample) ) 

#define IAudioMediaStream_SendEndOfStream(This,dwFlags)	\
    ( (This)->lpVtbl -> SendEndOfStream(This,dwFlags) ) 


#define IAudioMediaStream_GetFormat(This,pWaveFormatCurrent)	\
    ( (This)->lpVtbl -> GetFormat(This,pWaveFormatCurrent) ) 

#define IAudioMediaStream_SetFormat(This,lpWaveFormat)	\
    ( (This)->lpVtbl -> SetFormat(This,lpWaveFormat) ) 

#define IAudioMediaStream_CreateSample(This,pAudioData,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> CreateSample(This,pAudioData,dwFlags,ppSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioMediaStream_INTERFACE_DEFINED__ */


#ifndef __IAudioStreamSample_INTERFACE_DEFINED__
#define __IAudioStreamSample_INTERFACE_DEFINED__

/* interface IAudioStreamSample */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioStreamSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("345fee00-aba5-11d0-8212-00c04fc32c45")
    IAudioStreamSample : public IStreamSample
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAudioData( 
            /* [out] */ IAudioData **ppAudio) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioStreamSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioStreamSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioStreamSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioStreamSample * This);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IAudioStreamSample * This,
            /* [in] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *GetSampleTimes )( 
            IAudioStreamSample * This,
            /* [out] */ STREAM_TIME *pStartTime,
            /* [out] */ STREAM_TIME *pEndTime,
            /* [out] */ STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, SetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *SetSampleTimes )( 
            IAudioStreamSample * This,
            /* [in] */ const STREAM_TIME *pStartTime,
            /* [in] */ const STREAM_TIME *pEndTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IAudioStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hEvent,
            /* [in] */ PAPCFUNC pfnAPC,
            /* [in] */ DWORD_PTR dwAPCData);
        
        DECLSPEC_XFGVIRT(IStreamSample, CompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *CompletionStatus )( 
            IAudioStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(IAudioStreamSample, GetAudioData)
        HRESULT ( STDMETHODCALLTYPE *GetAudioData )( 
            IAudioStreamSample * This,
            /* [out] */ IAudioData **ppAudio);
        
        END_INTERFACE
    } IAudioStreamSampleVtbl;

    interface IAudioStreamSample
    {
        CONST_VTBL struct IAudioStreamSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioStreamSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioStreamSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioStreamSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioStreamSample_GetMediaStream(This,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,ppMediaStream) ) 

#define IAudioStreamSample_GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime)	\
    ( (This)->lpVtbl -> GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime) ) 

#define IAudioStreamSample_SetSampleTimes(This,pStartTime,pEndTime)	\
    ( (This)->lpVtbl -> SetSampleTimes(This,pStartTime,pEndTime) ) 

#define IAudioStreamSample_Update(This,dwFlags,hEvent,pfnAPC,dwAPCData)	\
    ( (This)->lpVtbl -> Update(This,dwFlags,hEvent,pfnAPC,dwAPCData) ) 

#define IAudioStreamSample_CompletionStatus(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> CompletionStatus(This,dwFlags,dwMilliseconds) ) 


#define IAudioStreamSample_GetAudioData(This,ppAudio)	\
    ( (This)->lpVtbl -> GetAudioData(This,ppAudio) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioStreamSample_INTERFACE_DEFINED__ */


#ifndef __IMemoryData_INTERFACE_DEFINED__
#define __IMemoryData_INTERFACE_DEFINED__

/* interface IMemoryData */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMemoryData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("327fc560-af60-11d0-8212-00c04fc32c45")
    IMemoryData : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBuffer( 
            /* [in] */ DWORD cbSize,
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [out] */ DWORD *pdwLength,
            /* [out] */ BYTE **ppbData,
            /* [out] */ DWORD *pcbActualData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActual( 
            /* [in] */ DWORD cbDataValid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMemoryDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMemoryData * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMemoryData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMemoryData * This);
        
        DECLSPEC_XFGVIRT(IMemoryData, SetBuffer)
        HRESULT ( STDMETHODCALLTYPE *SetBuffer )( 
            IMemoryData * This,
            /* [in] */ DWORD cbSize,
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMemoryData, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IMemoryData * This,
            /* [out] */ DWORD *pdwLength,
            /* [out] */ BYTE **ppbData,
            /* [out] */ DWORD *pcbActualData);
        
        DECLSPEC_XFGVIRT(IMemoryData, SetActual)
        HRESULT ( STDMETHODCALLTYPE *SetActual )( 
            IMemoryData * This,
            /* [in] */ DWORD cbDataValid);
        
        END_INTERFACE
    } IMemoryDataVtbl;

    interface IMemoryData
    {
        CONST_VTBL struct IMemoryDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMemoryData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMemoryData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMemoryData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMemoryData_SetBuffer(This,cbSize,pbData,dwFlags)	\
    ( (This)->lpVtbl -> SetBuffer(This,cbSize,pbData,dwFlags) ) 

#define IMemoryData_GetInfo(This,pdwLength,ppbData,pcbActualData)	\
    ( (This)->lpVtbl -> GetInfo(This,pdwLength,ppbData,pcbActualData) ) 

#define IMemoryData_SetActual(This,cbDataValid)	\
    ( (This)->lpVtbl -> SetActual(This,cbDataValid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMemoryData_INTERFACE_DEFINED__ */


#ifndef __IAudioData_INTERFACE_DEFINED__
#define __IAudioData_INTERFACE_DEFINED__

/* interface IAudioData */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54c719c0-af60-11d0-8212-00c04fc32c45")
    IAudioData : public IMemoryData
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [out] */ WAVEFORMATEX *pWaveFormatCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFormat( 
            /* [in] */ const WAVEFORMATEX *lpWaveFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioData * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioData * This);
        
        DECLSPEC_XFGVIRT(IMemoryData, SetBuffer)
        HRESULT ( STDMETHODCALLTYPE *SetBuffer )( 
            IAudioData * This,
            /* [in] */ DWORD cbSize,
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMemoryData, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IAudioData * This,
            /* [out] */ DWORD *pdwLength,
            /* [out] */ BYTE **ppbData,
            /* [out] */ DWORD *pcbActualData);
        
        DECLSPEC_XFGVIRT(IMemoryData, SetActual)
        HRESULT ( STDMETHODCALLTYPE *SetActual )( 
            IAudioData * This,
            /* [in] */ DWORD cbDataValid);
        
        DECLSPEC_XFGVIRT(IAudioData, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IAudioData * This,
            /* [out] */ WAVEFORMATEX *pWaveFormatCurrent);
        
        DECLSPEC_XFGVIRT(IAudioData, SetFormat)
        HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            IAudioData * This,
            /* [in] */ const WAVEFORMATEX *lpWaveFormat);
        
        END_INTERFACE
    } IAudioDataVtbl;

    interface IAudioData
    {
        CONST_VTBL struct IAudioDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioData_SetBuffer(This,cbSize,pbData,dwFlags)	\
    ( (This)->lpVtbl -> SetBuffer(This,cbSize,pbData,dwFlags) ) 

#define IAudioData_GetInfo(This,pdwLength,ppbData,pcbActualData)	\
    ( (This)->lpVtbl -> GetInfo(This,pdwLength,ppbData,pcbActualData) ) 

#define IAudioData_SetActual(This,cbDataValid)	\
    ( (This)->lpVtbl -> SetActual(This,cbDataValid) ) 


#define IAudioData_GetFormat(This,pWaveFormatCurrent)	\
    ( (This)->lpVtbl -> GetFormat(This,pWaveFormatCurrent) ) 

#define IAudioData_SetFormat(This,lpWaveFormat)	\
    ( (This)->lpVtbl -> SetFormat(This,lpWaveFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioData_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_austream_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_austream_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_austream_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


