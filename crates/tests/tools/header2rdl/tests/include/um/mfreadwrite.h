

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

#ifndef __mfreadwrite_h__
#define __mfreadwrite_h__

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

#ifndef __IMFReadWriteClassFactory_FWD_DEFINED__
#define __IMFReadWriteClassFactory_FWD_DEFINED__
typedef interface IMFReadWriteClassFactory IMFReadWriteClassFactory;

#endif 	/* __IMFReadWriteClassFactory_FWD_DEFINED__ */


#ifndef __IMFSourceReader_FWD_DEFINED__
#define __IMFSourceReader_FWD_DEFINED__
typedef interface IMFSourceReader IMFSourceReader;

#endif 	/* __IMFSourceReader_FWD_DEFINED__ */


#ifndef __IMFSourceReaderEx_FWD_DEFINED__
#define __IMFSourceReaderEx_FWD_DEFINED__
typedef interface IMFSourceReaderEx IMFSourceReaderEx;

#endif 	/* __IMFSourceReaderEx_FWD_DEFINED__ */


#ifndef __IMFSourceReaderCallback_FWD_DEFINED__
#define __IMFSourceReaderCallback_FWD_DEFINED__
typedef interface IMFSourceReaderCallback IMFSourceReaderCallback;

#endif 	/* __IMFSourceReaderCallback_FWD_DEFINED__ */


#ifndef __IMFSourceReaderCallback2_FWD_DEFINED__
#define __IMFSourceReaderCallback2_FWD_DEFINED__
typedef interface IMFSourceReaderCallback2 IMFSourceReaderCallback2;

#endif 	/* __IMFSourceReaderCallback2_FWD_DEFINED__ */


#ifndef __IMFSinkWriter_FWD_DEFINED__
#define __IMFSinkWriter_FWD_DEFINED__
typedef interface IMFSinkWriter IMFSinkWriter;

#endif 	/* __IMFSinkWriter_FWD_DEFINED__ */


#ifndef __IMFSinkWriterEx_FWD_DEFINED__
#define __IMFSinkWriterEx_FWD_DEFINED__
typedef interface IMFSinkWriterEx IMFSinkWriterEx;

#endif 	/* __IMFSinkWriterEx_FWD_DEFINED__ */


#ifndef __IMFSinkWriterEncoderConfig_FWD_DEFINED__
#define __IMFSinkWriterEncoderConfig_FWD_DEFINED__
typedef interface IMFSinkWriterEncoderConfig IMFSinkWriterEncoderConfig;

#endif 	/* __IMFSinkWriterEncoderConfig_FWD_DEFINED__ */


#ifndef __IMFSinkWriterCallback_FWD_DEFINED__
#define __IMFSinkWriterCallback_FWD_DEFINED__
typedef interface IMFSinkWriterCallback IMFSinkWriterCallback;

#endif 	/* __IMFSinkWriterCallback_FWD_DEFINED__ */


#ifndef __IMFSinkWriterCallback2_FWD_DEFINED__
#define __IMFSinkWriterCallback2_FWD_DEFINED__
typedef interface IMFSinkWriterCallback2 IMFSinkWriterCallback2;

#endif 	/* __IMFSinkWriterCallback2_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"
#include "MFTransform.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfreadwrite_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_GUID(CLSID_MFReadWriteClassFactory, 0x48e2ed0f, 0x98c2, 0x4a37, 0xbe, 0xd5, 0x16, 0x63, 0x12, 0xdd, 0xd8, 0x3f);


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0000_v0_0_s_ifspec;

#ifndef __IMFReadWriteClassFactory_INTERFACE_DEFINED__
#define __IMFReadWriteClassFactory_INTERFACE_DEFINED__

/* interface IMFReadWriteClassFactory */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFReadWriteClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E7FE2E12-661C-40DA-92F9-4F002AB67627")
    IMFReadWriteClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromURL( 
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszURL,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromObject( 
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  IUnknown *punkObject,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFReadWriteClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFReadWriteClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFReadWriteClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFReadWriteClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFReadWriteClassFactory, CreateInstanceFromURL)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceFromURL )( 
            IMFReadWriteClassFactory * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszURL,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFReadWriteClassFactory, CreateInstanceFromObject)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceFromObject )( 
            IMFReadWriteClassFactory * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  IUnknown *punkObject,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject);
        
        END_INTERFACE
    } IMFReadWriteClassFactoryVtbl;

    interface IMFReadWriteClassFactory
    {
        CONST_VTBL struct IMFReadWriteClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFReadWriteClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFReadWriteClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFReadWriteClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFReadWriteClassFactory_CreateInstanceFromURL(This,clsid,pwszURL,pAttributes,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceFromURL(This,clsid,pwszURL,pAttributes,riid,ppvObject) ) 

#define IMFReadWriteClassFactory_CreateInstanceFromObject(This,clsid,punkObject,pAttributes,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceFromObject(This,clsid,punkObject,pAttributes,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFReadWriteClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0001 */
/* [local] */ 

EXTERN_GUID(CLSID_MFSourceReader, 0x1777133c, 0x0881, 0x411b, 0xa5, 0x77, 0xad, 0x54, 0x5f, 0x07, 0x14, 0xc4);
STDAPI MFCreateSourceReaderFromURL(
    _In_ LPCWSTR pwszURL,
    _In_opt_ IMFAttributes *pAttributes,
    _Out_ IMFSourceReader **ppSourceReader );
STDAPI MFCreateSourceReaderFromByteStream(
    _In_ IMFByteStream *pByteStream,
    _In_opt_ IMFAttributes *pAttributes,
    _Out_ IMFSourceReader **ppSourceReader );
STDAPI MFCreateSourceReaderFromMediaSource(
    _In_ IMFMediaSource *pMediaSource,
    _In_opt_ IMFAttributes *pAttributes,
    _Out_ IMFSourceReader **ppSourceReader );
EXTERN_GUID( MF_SOURCE_READER_ASYNC_CALLBACK, 0x1e3dbeac, 0xbb43, 0x4c35, 0xb5, 0x07, 0xcd, 0x64, 0x44, 0x64, 0xc9, 0x65);
EXTERN_GUID( MF_SOURCE_READER_D3D_MANAGER, 0xec822da2, 0xe1e9, 0x4b29, 0xa0, 0xd8, 0x56, 0x3c, 0x71, 0x9f, 0x52, 0x69);
EXTERN_GUID( MF_SOURCE_READER_DISABLE_DXVA, 0xaa456cfd, 0x3943, 0x4a1e, 0xa7, 0x7d, 0x18, 0x38, 0xc0, 0xea, 0x2e, 0x35);
EXTERN_GUID( MF_SOURCE_READER_MEDIASOURCE_CONFIG, 0x9085abeb, 0x0354, 0x48f9, 0xab, 0xb5, 0x20, 0x0d, 0xf8, 0x38, 0xc6, 0x8e);
EXTERN_GUID( MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS, 0x6d23f5c8, 0xc5d7, 0x4a9b, 0x99, 0x71, 0x5d, 0x11, 0xf8, 0xbc, 0xa8, 0x80);
EXTERN_GUID( MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 0xfb394f3d, 0xccf1, 0x42ee, 0xbb, 0xb3, 0xf9, 0xb8, 0x45, 0xd5, 0x68, 0x1d);
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, 0xf81da2c, 0xb537, 0x4672, 0xa8, 0xb2, 0xa6, 0x81, 0xb1, 0x73, 0x7, 0xa3);
EXTERN_GUID( MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS, 0x9d3365dd, 0x58f, 0x4cfb, 0x9f, 0x97, 0xb3, 0x14, 0xcc, 0x99, 0xc8, 0xad );
#endif
EXTERN_GUID( MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN, 0x56b67165, 0x219e, 0x456d, 0xa2, 0x2e, 0x2d, 0x30, 0x04, 0xc7, 0xfe, 0x56);
EXTERN_GUID( MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS, 0xdfd4f008, 0xb5fd, 0x4e78, 0xae, 0x44, 0x62, 0xa1, 0xe6, 0x7b, 0xbe, 0x27);
EXTERN_GUID( MF_SOURCE_READER_D3D11_BIND_FLAGS, 0x33f3197b, 0xf73a, 0x4e14, 0x8d, 0x85, 0xe, 0x4c, 0x43, 0x68, 0x78, 0x8d);
EXTERN_GUID(MF_SOURCE_READER_PASSTHROUGH_MODE, 0x43ff126, 0xfe2c, 0x4708, 0xa0, 0x9b, 0xda, 0x2a, 0xb4, 0x35, 0xce, 0xd9);
typedef /* [v1_enum] */ 
enum MF_SOURCE_READER_FLAG
    {
        MF_SOURCE_READERF_ERROR	= 0x1,
        MF_SOURCE_READERF_ENDOFSTREAM	= 0x2,
        MF_SOURCE_READERF_NEWSTREAM	= 0x4,
        MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED	= 0x10,
        MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED	= 0x20,
        MF_SOURCE_READERF_STREAMTICK	= 0x100,
        MF_SOURCE_READERF_ALLEFFECTSREMOVED	= 0x200
    } 	MF_SOURCE_READER_FLAG;

DEFINE_ENUM_FLAG_OPERATORS(MF_SOURCE_READER_FLAG)
typedef /* [v1_enum] */ 
enum MF_SOURCE_READER_CONTROL_FLAG
    {
        MF_SOURCE_READER_CONTROLF_DRAIN	= 0x1
    } 	MF_SOURCE_READER_CONTROL_FLAG;

DEFINE_ENUM_FLAG_OPERATORS(MF_SOURCE_READER_CONTROL_FLAG)

enum __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001
    {
        MF_SOURCE_READER_INVALID_STREAM_INDEX	= 0xffffffff,
        MF_SOURCE_READER_ALL_STREAMS	= 0xfffffffe,
        MF_SOURCE_READER_ANY_STREAM	= 0xfffffffe,
        MF_SOURCE_READER_FIRST_AUDIO_STREAM	= 0xfffffffd,
        MF_SOURCE_READER_FIRST_VIDEO_STREAM	= 0xfffffffc,
        MF_SOURCE_READER_MEDIASOURCE	= 0xffffffff
    } ;
#if (WINVER >= _WIN32_WINNT_WIN8) 

enum __MIDL___MIDL_itf_mfreadwrite_0000_0001_0002
    {
        MF_SOURCE_READER_CURRENT_TYPE_INDEX	= 0xffffffff
    } ;
#endif // (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0001_v0_0_s_ifspec;

#ifndef __IMFSourceReader_INTERFACE_DEFINED__
#define __IMFSourceReader_INTERFACE_DEFINED__

/* interface IMFSourceReader */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70ae66f2-c809-4e4f-8915-bdcb406b7993")
    IMFSourceReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamSelection( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pfSelected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamSelection( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL fSelected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNativeMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pdwReserved,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentPosition( 
            /* [annotation][in] */ 
            _In_  REFGUID guidTimeFormat,
            /* [annotation][in] */ 
            _In_  REFPROPVARIANT varPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadSample( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwControlFlags,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwActualStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwStreamFlags,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pllTimestamp,
            /* [annotation][out] */ 
            _Out_opt_  IMFSample **ppSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentationAttribute( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarAttribute) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceReader * This);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSelection )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pfSelected);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *SetStreamSelection )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL fSelected);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetNativeMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetNativeMediaType )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetCurrentMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentMediaType )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetCurrentMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentMediaType )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pdwReserved,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetCurrentPosition)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentPosition )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidTimeFormat,
            /* [annotation][in] */ 
            _In_  REFPROPVARIANT varPosition);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, ReadSample)
        HRESULT ( STDMETHODCALLTYPE *ReadSample )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwControlFlags,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwActualStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwStreamFlags,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pllTimestamp,
            /* [annotation][out] */ 
            _Out_opt_  IMFSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetServiceForStream)
        HRESULT ( STDMETHODCALLTYPE *GetServiceForStream )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetPresentationAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationAttribute )( 
            IMFSourceReader * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarAttribute);
        
        END_INTERFACE
    } IMFSourceReaderVtbl;

    interface IMFSourceReader
    {
        CONST_VTBL struct IMFSourceReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceReader_GetStreamSelection(This,dwStreamIndex,pfSelected)	\
    ( (This)->lpVtbl -> GetStreamSelection(This,dwStreamIndex,pfSelected) ) 

#define IMFSourceReader_SetStreamSelection(This,dwStreamIndex,fSelected)	\
    ( (This)->lpVtbl -> SetStreamSelection(This,dwStreamIndex,fSelected) ) 

#define IMFSourceReader_GetNativeMediaType(This,dwStreamIndex,dwMediaTypeIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetNativeMediaType(This,dwStreamIndex,dwMediaTypeIndex,ppMediaType) ) 

#define IMFSourceReader_GetCurrentMediaType(This,dwStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetCurrentMediaType(This,dwStreamIndex,ppMediaType) ) 

#define IMFSourceReader_SetCurrentMediaType(This,dwStreamIndex,pdwReserved,pMediaType)	\
    ( (This)->lpVtbl -> SetCurrentMediaType(This,dwStreamIndex,pdwReserved,pMediaType) ) 

#define IMFSourceReader_SetCurrentPosition(This,guidTimeFormat,varPosition)	\
    ( (This)->lpVtbl -> SetCurrentPosition(This,guidTimeFormat,varPosition) ) 

#define IMFSourceReader_ReadSample(This,dwStreamIndex,dwControlFlags,pdwActualStreamIndex,pdwStreamFlags,pllTimestamp,ppSample)	\
    ( (This)->lpVtbl -> ReadSample(This,dwStreamIndex,dwControlFlags,pdwActualStreamIndex,pdwStreamFlags,pllTimestamp,ppSample) ) 

#define IMFSourceReader_Flush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> Flush(This,dwStreamIndex) ) 

#define IMFSourceReader_GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject) ) 

#define IMFSourceReader_GetPresentationAttribute(This,dwStreamIndex,guidAttribute,pvarAttribute)	\
    ( (This)->lpVtbl -> GetPresentationAttribute(This,dwStreamIndex,guidAttribute,pvarAttribute) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceReader_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0002_v0_0_s_ifspec;

#ifndef __IMFSourceReaderEx_INTERFACE_DEFINED__
#define __IMFSourceReaderEx_INTERFACE_DEFINED__

/* interface IMFSourceReaderEx */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceReaderEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7b981cf0-560e-4116-9875-b099895f23d7")
    IMFSourceReaderEx : public IMFSourceReader
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNativeMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTransformForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pTransformOrActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllTransformsForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwTransformIndex,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuidCategory,
            /* [annotation][out] */ 
            _Out_  IMFTransform **ppTransform) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceReaderExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceReaderEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceReaderEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceReaderEx * This);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSelection )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pfSelected);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *SetStreamSelection )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL fSelected);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetNativeMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetNativeMediaType )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetCurrentMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentMediaType )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetCurrentMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentMediaType )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pdwReserved,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, SetCurrentPosition)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentPosition )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidTimeFormat,
            /* [annotation][in] */ 
            _In_  REFPROPVARIANT varPosition);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, ReadSample)
        HRESULT ( STDMETHODCALLTYPE *ReadSample )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwControlFlags,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwActualStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwStreamFlags,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pllTimestamp,
            /* [annotation][out] */ 
            _Out_opt_  IMFSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetServiceForStream)
        HRESULT ( STDMETHODCALLTYPE *GetServiceForStream )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFSourceReader, GetPresentationAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationAttribute )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarAttribute);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderEx, SetNativeMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetNativeMediaType )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamFlags);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderEx, AddTransformForStream)
        HRESULT ( STDMETHODCALLTYPE *AddTransformForStream )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pTransformOrActivate);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderEx, RemoveAllTransformsForStream)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllTransformsForStream )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderEx, GetTransformForStream)
        HRESULT ( STDMETHODCALLTYPE *GetTransformForStream )( 
            IMFSourceReaderEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwTransformIndex,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuidCategory,
            /* [annotation][out] */ 
            _Out_  IMFTransform **ppTransform);
        
        END_INTERFACE
    } IMFSourceReaderExVtbl;

    interface IMFSourceReaderEx
    {
        CONST_VTBL struct IMFSourceReaderExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceReaderEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceReaderEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceReaderEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceReaderEx_GetStreamSelection(This,dwStreamIndex,pfSelected)	\
    ( (This)->lpVtbl -> GetStreamSelection(This,dwStreamIndex,pfSelected) ) 

#define IMFSourceReaderEx_SetStreamSelection(This,dwStreamIndex,fSelected)	\
    ( (This)->lpVtbl -> SetStreamSelection(This,dwStreamIndex,fSelected) ) 

#define IMFSourceReaderEx_GetNativeMediaType(This,dwStreamIndex,dwMediaTypeIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetNativeMediaType(This,dwStreamIndex,dwMediaTypeIndex,ppMediaType) ) 

#define IMFSourceReaderEx_GetCurrentMediaType(This,dwStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetCurrentMediaType(This,dwStreamIndex,ppMediaType) ) 

#define IMFSourceReaderEx_SetCurrentMediaType(This,dwStreamIndex,pdwReserved,pMediaType)	\
    ( (This)->lpVtbl -> SetCurrentMediaType(This,dwStreamIndex,pdwReserved,pMediaType) ) 

#define IMFSourceReaderEx_SetCurrentPosition(This,guidTimeFormat,varPosition)	\
    ( (This)->lpVtbl -> SetCurrentPosition(This,guidTimeFormat,varPosition) ) 

#define IMFSourceReaderEx_ReadSample(This,dwStreamIndex,dwControlFlags,pdwActualStreamIndex,pdwStreamFlags,pllTimestamp,ppSample)	\
    ( (This)->lpVtbl -> ReadSample(This,dwStreamIndex,dwControlFlags,pdwActualStreamIndex,pdwStreamFlags,pllTimestamp,ppSample) ) 

#define IMFSourceReaderEx_Flush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> Flush(This,dwStreamIndex) ) 

#define IMFSourceReaderEx_GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject) ) 

#define IMFSourceReaderEx_GetPresentationAttribute(This,dwStreamIndex,guidAttribute,pvarAttribute)	\
    ( (This)->lpVtbl -> GetPresentationAttribute(This,dwStreamIndex,guidAttribute,pvarAttribute) ) 


#define IMFSourceReaderEx_SetNativeMediaType(This,dwStreamIndex,pMediaType,pdwStreamFlags)	\
    ( (This)->lpVtbl -> SetNativeMediaType(This,dwStreamIndex,pMediaType,pdwStreamFlags) ) 

#define IMFSourceReaderEx_AddTransformForStream(This,dwStreamIndex,pTransformOrActivate)	\
    ( (This)->lpVtbl -> AddTransformForStream(This,dwStreamIndex,pTransformOrActivate) ) 

#define IMFSourceReaderEx_RemoveAllTransformsForStream(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> RemoveAllTransformsForStream(This,dwStreamIndex) ) 

#define IMFSourceReaderEx_GetTransformForStream(This,dwStreamIndex,dwTransformIndex,pGuidCategory,ppTransform)	\
    ( (This)->lpVtbl -> GetTransformForStream(This,dwStreamIndex,dwTransformIndex,pGuidCategory,ppTransform) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceReaderEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0003_v0_0_s_ifspec;

#ifndef __IMFSourceReaderCallback_INTERFACE_DEFINED__
#define __IMFSourceReaderCallback_INTERFACE_DEFINED__

/* interface IMFSourceReaderCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceReaderCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("deec8d99-fa1d-4d82-84c2-2c8969944867")
    IMFSourceReaderCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnReadSample( 
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamFlags,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp,
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFlush( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceReaderCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceReaderCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceReaderCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceReaderCallback * This);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnReadSample)
        HRESULT ( STDMETHODCALLTYPE *OnReadSample )( 
            IMFSourceReaderCallback * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamFlags,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp,
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnFlush)
        HRESULT ( STDMETHODCALLTYPE *OnFlush )( 
            IMFSourceReaderCallback * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            IMFSourceReaderCallback * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        END_INTERFACE
    } IMFSourceReaderCallbackVtbl;

    interface IMFSourceReaderCallback
    {
        CONST_VTBL struct IMFSourceReaderCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceReaderCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceReaderCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceReaderCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceReaderCallback_OnReadSample(This,hrStatus,dwStreamIndex,dwStreamFlags,llTimestamp,pSample)	\
    ( (This)->lpVtbl -> OnReadSample(This,hrStatus,dwStreamIndex,dwStreamFlags,llTimestamp,pSample) ) 

#define IMFSourceReaderCallback_OnFlush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> OnFlush(This,dwStreamIndex) ) 

#define IMFSourceReaderCallback_OnEvent(This,dwStreamIndex,pEvent)	\
    ( (This)->lpVtbl -> OnEvent(This,dwStreamIndex,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceReaderCallback_INTERFACE_DEFINED__ */


#ifndef __IMFSourceReaderCallback2_INTERFACE_DEFINED__
#define __IMFSourceReaderCallback2_INTERFACE_DEFINED__

/* interface IMFSourceReaderCallback2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceReaderCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF839FE6-8C2A-4DD2-B6EA-C22D6961AF05")
    IMFSourceReaderCallback2 : public IMFSourceReaderCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTransformChange( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStreamError( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceReaderCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceReaderCallback2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceReaderCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceReaderCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnReadSample)
        HRESULT ( STDMETHODCALLTYPE *OnReadSample )( 
            IMFSourceReaderCallback2 * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamFlags,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp,
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnFlush)
        HRESULT ( STDMETHODCALLTYPE *OnFlush )( 
            IMFSourceReaderCallback2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            IMFSourceReaderCallback2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback2, OnTransformChange)
        HRESULT ( STDMETHODCALLTYPE *OnTransformChange )( 
            IMFSourceReaderCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFSourceReaderCallback2, OnStreamError)
        HRESULT ( STDMETHODCALLTYPE *OnStreamError )( 
            IMFSourceReaderCallback2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus);
        
        END_INTERFACE
    } IMFSourceReaderCallback2Vtbl;

    interface IMFSourceReaderCallback2
    {
        CONST_VTBL struct IMFSourceReaderCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceReaderCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceReaderCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceReaderCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceReaderCallback2_OnReadSample(This,hrStatus,dwStreamIndex,dwStreamFlags,llTimestamp,pSample)	\
    ( (This)->lpVtbl -> OnReadSample(This,hrStatus,dwStreamIndex,dwStreamFlags,llTimestamp,pSample) ) 

#define IMFSourceReaderCallback2_OnFlush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> OnFlush(This,dwStreamIndex) ) 

#define IMFSourceReaderCallback2_OnEvent(This,dwStreamIndex,pEvent)	\
    ( (This)->lpVtbl -> OnEvent(This,dwStreamIndex,pEvent) ) 


#define IMFSourceReaderCallback2_OnTransformChange(This)	\
    ( (This)->lpVtbl -> OnTransformChange(This) ) 

#define IMFSourceReaderCallback2_OnStreamError(This,dwStreamIndex,hrStatus)	\
    ( (This)->lpVtbl -> OnStreamError(This,dwStreamIndex,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceReaderCallback2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0005 */
/* [local] */ 

EXTERN_GUID(CLSID_MFSinkWriter, 0xa3bbfb17, 0x8273, 0x4e52, 0x9e, 0x0e, 0x97, 0x39, 0xdc, 0x88, 0x79, 0x90);
STDAPI MFCreateSinkWriterFromURL(
    _In_opt_ LPCWSTR pwszOutputURL,
    _In_opt_ IMFByteStream *pByteStream,
    _In_opt_ IMFAttributes *pAttributes,
    _Out_ IMFSinkWriter **ppSinkWriter );
STDAPI MFCreateSinkWriterFromMediaSink(
    _In_ IMFMediaSink *pMediaSink,
    _In_opt_ IMFAttributes *pAttributes,
    _Out_ IMFSinkWriter **ppSinkWriter );
EXTERN_GUID( MF_SINK_WRITER_ASYNC_CALLBACK, 0x48cb183e, 0x7b0b, 0x46f4, 0x82, 0x2e, 0x5e, 0x1d, 0x2d, 0xda, 0x43, 0x54);
EXTERN_GUID( MF_SINK_WRITER_DISABLE_THROTTLING, 0x08b845d8, 0x2b74, 0x4afe, 0x9d, 0x53, 0xbe, 0x16, 0xd2, 0xd5, 0xae, 0x4f);
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MF_SINK_WRITER_D3D_MANAGER, 0xec822da2, 0xe1e9, 0x4b29, 0xa0, 0xd8, 0x56, 0x3c, 0x71, 0x9f, 0x52, 0x69);
EXTERN_GUID( MF_SINK_WRITER_ENCODER_CONFIG, 0xad91cd04, 0xa7cc, 0x4ac7, 0x99, 0xb6, 0xa5, 0x7b, 0x9a, 0x4a, 0x7c, 0x70);
#endif

enum __MIDL___MIDL_itf_mfreadwrite_0000_0005_0001
    {
        MF_SINK_WRITER_INVALID_STREAM_INDEX	= 0xffffffff,
        MF_SINK_WRITER_ALL_STREAMS	= 0xfffffffe,
        MF_SINK_WRITER_MEDIASINK	= 0xffffffff
    } ;
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct _MF_SINK_WRITER_STATISTICS
    {
    DWORD cb;
    LONGLONG llLastTimestampReceived;
    LONGLONG llLastTimestampEncoded;
    LONGLONG llLastTimestampProcessed;
    LONGLONG llLastStreamTickReceived;
    LONGLONG llLastSinkSampleRequest;
    QWORD qwNumSamplesReceived;
    QWORD qwNumSamplesEncoded;
    QWORD qwNumSamplesProcessed;
    QWORD qwNumStreamTicksReceived;
    DWORD dwByteCountQueued;
    QWORD qwByteCountProcessed;
    DWORD dwNumOutstandingSinkSampleRequests;
    DWORD dwAverageSampleRateReceived;
    DWORD dwAverageSampleRateEncoded;
    DWORD dwAverageSampleRateProcessed;
    } 	MF_SINK_WRITER_STATISTICS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0005_v0_0_s_ifspec;

#ifndef __IMFSinkWriter_INTERFACE_DEFINED__
#define __IMFSinkWriter_INTERFACE_DEFINED__

/* interface IMFSinkWriter */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSinkWriter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3137f1cd-fe5e-4805-a5d8-fb477448cb3d")
    IMFSinkWriter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddStream( 
            /* [annotation][in] */ 
            _In_  IMFMediaType *pTargetMediaType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pInputMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginWriting( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteSample( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendStreamTick( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlaceMarker( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyEndOfSegment( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finalize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatistics( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  MF_SINK_WRITER_STATISTICS *pStats) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSinkWriterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSinkWriter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSinkWriter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSinkWriter * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pTargetMediaType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, SetInputMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetInputMediaType )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pInputMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingParameters);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, BeginWriting)
        HRESULT ( STDMETHODCALLTYPE *BeginWriting )( 
            IMFSinkWriter * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, WriteSample)
        HRESULT ( STDMETHODCALLTYPE *WriteSample )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, SendStreamTick)
        HRESULT ( STDMETHODCALLTYPE *SendStreamTick )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, PlaceMarker)
        HRESULT ( STDMETHODCALLTYPE *PlaceMarker )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, NotifyEndOfSegment)
        HRESULT ( STDMETHODCALLTYPE *NotifyEndOfSegment )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, Finalize)
        HRESULT ( STDMETHODCALLTYPE *Finalize )( 
            IMFSinkWriter * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, GetServiceForStream)
        HRESULT ( STDMETHODCALLTYPE *GetServiceForStream )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            IMFSinkWriter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  MF_SINK_WRITER_STATISTICS *pStats);
        
        END_INTERFACE
    } IMFSinkWriterVtbl;

    interface IMFSinkWriter
    {
        CONST_VTBL struct IMFSinkWriterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSinkWriter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSinkWriter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSinkWriter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSinkWriter_AddStream(This,pTargetMediaType,pdwStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,pTargetMediaType,pdwStreamIndex) ) 

#define IMFSinkWriter_SetInputMediaType(This,dwStreamIndex,pInputMediaType,pEncodingParameters)	\
    ( (This)->lpVtbl -> SetInputMediaType(This,dwStreamIndex,pInputMediaType,pEncodingParameters) ) 

#define IMFSinkWriter_BeginWriting(This)	\
    ( (This)->lpVtbl -> BeginWriting(This) ) 

#define IMFSinkWriter_WriteSample(This,dwStreamIndex,pSample)	\
    ( (This)->lpVtbl -> WriteSample(This,dwStreamIndex,pSample) ) 

#define IMFSinkWriter_SendStreamTick(This,dwStreamIndex,llTimestamp)	\
    ( (This)->lpVtbl -> SendStreamTick(This,dwStreamIndex,llTimestamp) ) 

#define IMFSinkWriter_PlaceMarker(This,dwStreamIndex,pvContext)	\
    ( (This)->lpVtbl -> PlaceMarker(This,dwStreamIndex,pvContext) ) 

#define IMFSinkWriter_NotifyEndOfSegment(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> NotifyEndOfSegment(This,dwStreamIndex) ) 

#define IMFSinkWriter_Flush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> Flush(This,dwStreamIndex) ) 

#define IMFSinkWriter_Finalize(This)	\
    ( (This)->lpVtbl -> Finalize(This) ) 

#define IMFSinkWriter_GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject) ) 

#define IMFSinkWriter_GetStatistics(This,dwStreamIndex,pStats)	\
    ( (This)->lpVtbl -> GetStatistics(This,dwStreamIndex,pStats) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSinkWriter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0006 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0006_v0_0_s_ifspec;

#ifndef __IMFSinkWriterEx_INTERFACE_DEFINED__
#define __IMFSinkWriterEx_INTERFACE_DEFINED__

/* interface IMFSinkWriterEx */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSinkWriterEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("588d72ab-5Bc1-496a-8714-b70617141b25")
    IMFSinkWriterEx : public IMFSinkWriter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransformForStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwTransformIndex,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuidCategory,
            /* [annotation][out] */ 
            _Out_  IMFTransform **ppTransform) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSinkWriterExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSinkWriterEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSinkWriterEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSinkWriterEx * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pTargetMediaType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, SetInputMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetInputMediaType )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pInputMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingParameters);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, BeginWriting)
        HRESULT ( STDMETHODCALLTYPE *BeginWriting )( 
            IMFSinkWriterEx * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, WriteSample)
        HRESULT ( STDMETHODCALLTYPE *WriteSample )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, SendStreamTick)
        HRESULT ( STDMETHODCALLTYPE *SendStreamTick )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LONGLONG llTimestamp);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, PlaceMarker)
        HRESULT ( STDMETHODCALLTYPE *PlaceMarker )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, NotifyEndOfSegment)
        HRESULT ( STDMETHODCALLTYPE *NotifyEndOfSegment )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, Finalize)
        HRESULT ( STDMETHODCALLTYPE *Finalize )( 
            IMFSinkWriterEx * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, GetServiceForStream)
        HRESULT ( STDMETHODCALLTYPE *GetServiceForStream )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFSinkWriter, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  MF_SINK_WRITER_STATISTICS *pStats);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterEx, GetTransformForStream)
        HRESULT ( STDMETHODCALLTYPE *GetTransformForStream )( 
            IMFSinkWriterEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwTransformIndex,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuidCategory,
            /* [annotation][out] */ 
            _Out_  IMFTransform **ppTransform);
        
        END_INTERFACE
    } IMFSinkWriterExVtbl;

    interface IMFSinkWriterEx
    {
        CONST_VTBL struct IMFSinkWriterExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSinkWriterEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSinkWriterEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSinkWriterEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSinkWriterEx_AddStream(This,pTargetMediaType,pdwStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,pTargetMediaType,pdwStreamIndex) ) 

#define IMFSinkWriterEx_SetInputMediaType(This,dwStreamIndex,pInputMediaType,pEncodingParameters)	\
    ( (This)->lpVtbl -> SetInputMediaType(This,dwStreamIndex,pInputMediaType,pEncodingParameters) ) 

#define IMFSinkWriterEx_BeginWriting(This)	\
    ( (This)->lpVtbl -> BeginWriting(This) ) 

#define IMFSinkWriterEx_WriteSample(This,dwStreamIndex,pSample)	\
    ( (This)->lpVtbl -> WriteSample(This,dwStreamIndex,pSample) ) 

#define IMFSinkWriterEx_SendStreamTick(This,dwStreamIndex,llTimestamp)	\
    ( (This)->lpVtbl -> SendStreamTick(This,dwStreamIndex,llTimestamp) ) 

#define IMFSinkWriterEx_PlaceMarker(This,dwStreamIndex,pvContext)	\
    ( (This)->lpVtbl -> PlaceMarker(This,dwStreamIndex,pvContext) ) 

#define IMFSinkWriterEx_NotifyEndOfSegment(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> NotifyEndOfSegment(This,dwStreamIndex) ) 

#define IMFSinkWriterEx_Flush(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> Flush(This,dwStreamIndex) ) 

#define IMFSinkWriterEx_Finalize(This)	\
    ( (This)->lpVtbl -> Finalize(This) ) 

#define IMFSinkWriterEx_GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetServiceForStream(This,dwStreamIndex,guidService,riid,ppvObject) ) 

#define IMFSinkWriterEx_GetStatistics(This,dwStreamIndex,pStats)	\
    ( (This)->lpVtbl -> GetStatistics(This,dwStreamIndex,pStats) ) 


#define IMFSinkWriterEx_GetTransformForStream(This,dwStreamIndex,dwTransformIndex,pGuidCategory,ppTransform)	\
    ( (This)->lpVtbl -> GetTransformForStream(This,dwStreamIndex,dwTransformIndex,pGuidCategory,ppTransform) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSinkWriterEx_INTERFACE_DEFINED__ */


#ifndef __IMFSinkWriterEncoderConfig_INTERFACE_DEFINED__
#define __IMFSinkWriterEncoderConfig_INTERFACE_DEFINED__

/* interface IMFSinkWriterEncoderConfig */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSinkWriterEncoderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("17C3779E-3CDE-4EDE-8C60-3899F5F53AD6")
    IMFSinkWriterEncoderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTargetMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pTargetMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlaceEncodingParameters( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pEncodingParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSinkWriterEncoderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSinkWriterEncoderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSinkWriterEncoderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSinkWriterEncoderConfig * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterEncoderConfig, SetTargetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetTargetMediaType )( 
            IMFSinkWriterEncoderConfig * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pTargetMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingParameters);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterEncoderConfig, PlaceEncodingParameters)
        HRESULT ( STDMETHODCALLTYPE *PlaceEncodingParameters )( 
            IMFSinkWriterEncoderConfig * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pEncodingParameters);
        
        END_INTERFACE
    } IMFSinkWriterEncoderConfigVtbl;

    interface IMFSinkWriterEncoderConfig
    {
        CONST_VTBL struct IMFSinkWriterEncoderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSinkWriterEncoderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSinkWriterEncoderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSinkWriterEncoderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSinkWriterEncoderConfig_SetTargetMediaType(This,dwStreamIndex,pTargetMediaType,pEncodingParameters)	\
    ( (This)->lpVtbl -> SetTargetMediaType(This,dwStreamIndex,pTargetMediaType,pEncodingParameters) ) 

#define IMFSinkWriterEncoderConfig_PlaceEncodingParameters(This,dwStreamIndex,pEncodingParameters)	\
    ( (This)->lpVtbl -> PlaceEncodingParameters(This,dwStreamIndex,pEncodingParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSinkWriterEncoderConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0008 */
/* [local] */ 

#endif


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0008_v0_0_s_ifspec;

#ifndef __IMFSinkWriterCallback_INTERFACE_DEFINED__
#define __IMFSinkWriterCallback_INTERFACE_DEFINED__

/* interface IMFSinkWriterCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSinkWriterCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("666f76de-33d2-41b9-a458-29ed0a972c58")
    IMFSinkWriterCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnFinalize( 
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMarker( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSinkWriterCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSinkWriterCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSinkWriterCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSinkWriterCallback * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback, OnFinalize)
        HRESULT ( STDMETHODCALLTYPE *OnFinalize )( 
            IMFSinkWriterCallback * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback, OnMarker)
        HRESULT ( STDMETHODCALLTYPE *OnMarker )( 
            IMFSinkWriterCallback * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext);
        
        END_INTERFACE
    } IMFSinkWriterCallbackVtbl;

    interface IMFSinkWriterCallback
    {
        CONST_VTBL struct IMFSinkWriterCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSinkWriterCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSinkWriterCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSinkWriterCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSinkWriterCallback_OnFinalize(This,hrStatus)	\
    ( (This)->lpVtbl -> OnFinalize(This,hrStatus) ) 

#define IMFSinkWriterCallback_OnMarker(This,dwStreamIndex,pvContext)	\
    ( (This)->lpVtbl -> OnMarker(This,dwStreamIndex,pvContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSinkWriterCallback_INTERFACE_DEFINED__ */


#ifndef __IMFSinkWriterCallback2_INTERFACE_DEFINED__
#define __IMFSinkWriterCallback2_INTERFACE_DEFINED__

/* interface IMFSinkWriterCallback2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSinkWriterCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2456BD58-C067-4513-84FE-8D0C88FFDC61")
    IMFSinkWriterCallback2 : public IMFSinkWriterCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTransformChange( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStreamError( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSinkWriterCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSinkWriterCallback2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSinkWriterCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSinkWriterCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback, OnFinalize)
        HRESULT ( STDMETHODCALLTYPE *OnFinalize )( 
            IMFSinkWriterCallback2 * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback, OnMarker)
        HRESULT ( STDMETHODCALLTYPE *OnMarker )( 
            IMFSinkWriterCallback2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  LPVOID pvContext);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback2, OnTransformChange)
        HRESULT ( STDMETHODCALLTYPE *OnTransformChange )( 
            IMFSinkWriterCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFSinkWriterCallback2, OnStreamError)
        HRESULT ( STDMETHODCALLTYPE *OnStreamError )( 
            IMFSinkWriterCallback2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus);
        
        END_INTERFACE
    } IMFSinkWriterCallback2Vtbl;

    interface IMFSinkWriterCallback2
    {
        CONST_VTBL struct IMFSinkWriterCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSinkWriterCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSinkWriterCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSinkWriterCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSinkWriterCallback2_OnFinalize(This,hrStatus)	\
    ( (This)->lpVtbl -> OnFinalize(This,hrStatus) ) 

#define IMFSinkWriterCallback2_OnMarker(This,dwStreamIndex,pvContext)	\
    ( (This)->lpVtbl -> OnMarker(This,dwStreamIndex,pvContext) ) 


#define IMFSinkWriterCallback2_OnTransformChange(This)	\
    ( (This)->lpVtbl -> OnTransformChange(This) ) 

#define IMFSinkWriterCallback2_OnStreamError(This,dwStreamIndex,hrStatus)	\
    ( (This)->lpVtbl -> OnStreamError(This,dwStreamIndex,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSinkWriterCallback2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfreadwrite_0000_0010 */
/* [local] */ 

EXTERN_GUID( MF_READWRITE_DISABLE_CONVERTERS, 0x98d5b065, 0x1374, 0x4847, 0x8d, 0x5d, 0x31, 0x52, 0x0f, 0xee, 0x71, 0x56);
EXTERN_GUID( MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 0xa634a91c, 0x822b, 0x41b9, 0xa4, 0x94, 0x4d, 0xe4, 0x64, 0x36, 0x12, 0xb0);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_GUID( MF_READWRITE_MMCSS_CLASS, 0x39384300, 0xd0eb, 0x40b1, 0x87, 0xa0, 0x33, 0x18, 0x87, 0x1b, 0x5a, 0x53);
EXTERN_GUID( MF_READWRITE_MMCSS_PRIORITY, 0x43ad19ce, 0xf33f, 0x4ba9, 0xa5, 0x80, 0xe4, 0xcd, 0x12, 0xf2, 0xd1, 0x44);
EXTERN_GUID( MF_READWRITE_MMCSS_CLASS_AUDIO, 0x430847da, 0x0890, 0x4b0e, 0x93, 0x8c, 0x05, 0x43, 0x32, 0xc5, 0x47, 0xe1);
EXTERN_GUID( MF_READWRITE_MMCSS_PRIORITY_AUDIO, 0x273db885, 0x2de2, 0x4db2, 0xa6, 0xa7, 0xfd, 0xb6, 0x6f, 0xb4, 0x0b, 0x61);
EXTERN_GUID( MF_READWRITE_D3D_OPTIONAL, 0x216479d9, 0x3071, 0x42ca, 0xbb, 0x6c, 0x4c, 0x22, 0x10, 0x2e, 0x1d, 0x18);
EXTERN_GUID( MF_MEDIASINK_AUTOFINALIZE_SUPPORTED, 0x48c131be, 0x135a, 0x41cb, 0x82, 0x90, 0x3, 0x65, 0x25, 0x9, 0xc9, 0x99);
EXTERN_GUID( MF_MEDIASINK_ENABLE_AUTOFINALIZE, 0x34014265, 0xcb7e, 0x4cde, 0xac, 0x7c, 0xef, 0xfd, 0x3b, 0x3c, 0x25, 0x30);
EXTERN_GUID( MF_READWRITE_ENABLE_AUTOFINALIZE, 0xdd7ca129, 0x8cd1, 0x4dc5, 0x9d, 0xde, 0xce, 0x16, 0x86, 0x75, 0xde, 0x61);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfreadwrite_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


