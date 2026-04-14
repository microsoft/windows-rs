

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

#ifndef __mfobjects_h__
#define __mfobjects_h__

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

#ifndef __IMFAttributes_FWD_DEFINED__
#define __IMFAttributes_FWD_DEFINED__
typedef interface IMFAttributes IMFAttributes;

#endif 	/* __IMFAttributes_FWD_DEFINED__ */


#ifndef __IMFMediaBuffer_FWD_DEFINED__
#define __IMFMediaBuffer_FWD_DEFINED__
typedef interface IMFMediaBuffer IMFMediaBuffer;

#endif 	/* __IMFMediaBuffer_FWD_DEFINED__ */


#ifndef __IMFSample_FWD_DEFINED__
#define __IMFSample_FWD_DEFINED__
typedef interface IMFSample IMFSample;

#endif 	/* __IMFSample_FWD_DEFINED__ */


#ifndef __IMF2DBuffer_FWD_DEFINED__
#define __IMF2DBuffer_FWD_DEFINED__
typedef interface IMF2DBuffer IMF2DBuffer;

#endif 	/* __IMF2DBuffer_FWD_DEFINED__ */


#ifndef __IMF2DBuffer2_FWD_DEFINED__
#define __IMF2DBuffer2_FWD_DEFINED__
typedef interface IMF2DBuffer2 IMF2DBuffer2;

#endif 	/* __IMF2DBuffer2_FWD_DEFINED__ */


#ifndef __IMFDXGIBuffer_FWD_DEFINED__
#define __IMFDXGIBuffer_FWD_DEFINED__
typedef interface IMFDXGIBuffer IMFDXGIBuffer;

#endif 	/* __IMFDXGIBuffer_FWD_DEFINED__ */


#ifndef __IMFDXGICrossAdapterBuffer_FWD_DEFINED__
#define __IMFDXGICrossAdapterBuffer_FWD_DEFINED__
typedef interface IMFDXGICrossAdapterBuffer IMFDXGICrossAdapterBuffer;

#endif 	/* __IMFDXGICrossAdapterBuffer_FWD_DEFINED__ */


#ifndef __IMFMediaType_FWD_DEFINED__
#define __IMFMediaType_FWD_DEFINED__
typedef interface IMFMediaType IMFMediaType;

#endif 	/* __IMFMediaType_FWD_DEFINED__ */


#ifndef __IMFAudioMediaType_FWD_DEFINED__
#define __IMFAudioMediaType_FWD_DEFINED__
typedef interface IMFAudioMediaType IMFAudioMediaType;

#endif 	/* __IMFAudioMediaType_FWD_DEFINED__ */


#ifndef __IMFVideoMediaType_FWD_DEFINED__
#define __IMFVideoMediaType_FWD_DEFINED__
typedef interface IMFVideoMediaType IMFVideoMediaType;

#endif 	/* __IMFVideoMediaType_FWD_DEFINED__ */


#ifndef __IMFAsyncResult_FWD_DEFINED__
#define __IMFAsyncResult_FWD_DEFINED__
typedef interface IMFAsyncResult IMFAsyncResult;

#endif 	/* __IMFAsyncResult_FWD_DEFINED__ */


#ifndef __IMFAsyncCallback_FWD_DEFINED__
#define __IMFAsyncCallback_FWD_DEFINED__
typedef interface IMFAsyncCallback IMFAsyncCallback;

#endif 	/* __IMFAsyncCallback_FWD_DEFINED__ */


#ifndef __IMFAsyncCallbackLogging_FWD_DEFINED__
#define __IMFAsyncCallbackLogging_FWD_DEFINED__
typedef interface IMFAsyncCallbackLogging IMFAsyncCallbackLogging;

#endif 	/* __IMFAsyncCallbackLogging_FWD_DEFINED__ */


#ifndef __IMFMediaEvent_FWD_DEFINED__
#define __IMFMediaEvent_FWD_DEFINED__
typedef interface IMFMediaEvent IMFMediaEvent;

#endif 	/* __IMFMediaEvent_FWD_DEFINED__ */


#ifndef __IMFMediaEventGenerator_FWD_DEFINED__
#define __IMFMediaEventGenerator_FWD_DEFINED__
typedef interface IMFMediaEventGenerator IMFMediaEventGenerator;

#endif 	/* __IMFMediaEventGenerator_FWD_DEFINED__ */


#ifndef __IMFRemoteAsyncCallback_FWD_DEFINED__
#define __IMFRemoteAsyncCallback_FWD_DEFINED__
typedef interface IMFRemoteAsyncCallback IMFRemoteAsyncCallback;

#endif 	/* __IMFRemoteAsyncCallback_FWD_DEFINED__ */


#ifndef __IMFByteStream_FWD_DEFINED__
#define __IMFByteStream_FWD_DEFINED__
typedef interface IMFByteStream IMFByteStream;

#endif 	/* __IMFByteStream_FWD_DEFINED__ */


#ifndef __IMFByteStreamProxyClassFactory_FWD_DEFINED__
#define __IMFByteStreamProxyClassFactory_FWD_DEFINED__
typedef interface IMFByteStreamProxyClassFactory IMFByteStreamProxyClassFactory;

#endif 	/* __IMFByteStreamProxyClassFactory_FWD_DEFINED__ */


#ifndef __IMFSampleOutputStream_FWD_DEFINED__
#define __IMFSampleOutputStream_FWD_DEFINED__
typedef interface IMFSampleOutputStream IMFSampleOutputStream;

#endif 	/* __IMFSampleOutputStream_FWD_DEFINED__ */


#ifndef __IMFCollection_FWD_DEFINED__
#define __IMFCollection_FWD_DEFINED__
typedef interface IMFCollection IMFCollection;

#endif 	/* __IMFCollection_FWD_DEFINED__ */


#ifndef __IMFMediaEventQueue_FWD_DEFINED__
#define __IMFMediaEventQueue_FWD_DEFINED__
typedef interface IMFMediaEventQueue IMFMediaEventQueue;

#endif 	/* __IMFMediaEventQueue_FWD_DEFINED__ */


#ifndef __IMFActivate_FWD_DEFINED__
#define __IMFActivate_FWD_DEFINED__
typedef interface IMFActivate IMFActivate;

#endif 	/* __IMFActivate_FWD_DEFINED__ */


#ifndef __IMFPluginControl_FWD_DEFINED__
#define __IMFPluginControl_FWD_DEFINED__
typedef interface IMFPluginControl IMFPluginControl;

#endif 	/* __IMFPluginControl_FWD_DEFINED__ */


#ifndef __IMFPluginControl2_FWD_DEFINED__
#define __IMFPluginControl2_FWD_DEFINED__
typedef interface IMFPluginControl2 IMFPluginControl2;

#endif 	/* __IMFPluginControl2_FWD_DEFINED__ */


#ifndef __IMFDXGIDeviceManager_FWD_DEFINED__
#define __IMFDXGIDeviceManager_FWD_DEFINED__
typedef interface IMFDXGIDeviceManager IMFDXGIDeviceManager;

#endif 	/* __IMFDXGIDeviceManager_FWD_DEFINED__ */


#ifndef __IMFMuxStreamAttributesManager_FWD_DEFINED__
#define __IMFMuxStreamAttributesManager_FWD_DEFINED__
typedef interface IMFMuxStreamAttributesManager IMFMuxStreamAttributesManager;

#endif 	/* __IMFMuxStreamAttributesManager_FWD_DEFINED__ */


#ifndef __IMFMuxStreamMediaTypeManager_FWD_DEFINED__
#define __IMFMuxStreamMediaTypeManager_FWD_DEFINED__
typedef interface IMFMuxStreamMediaTypeManager IMFMuxStreamMediaTypeManager;

#endif 	/* __IMFMuxStreamMediaTypeManager_FWD_DEFINED__ */


#ifndef __IMFMuxStreamSampleManager_FWD_DEFINED__
#define __IMFMuxStreamSampleManager_FWD_DEFINED__
typedef interface IMFMuxStreamSampleManager IMFMuxStreamSampleManager;

#endif 	/* __IMFMuxStreamSampleManager_FWD_DEFINED__ */


#ifndef __IMFSecureBuffer_FWD_DEFINED__
#define __IMFSecureBuffer_FWD_DEFINED__
typedef interface IMFSecureBuffer IMFSecureBuffer;

#endif 	/* __IMFSecureBuffer_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "propsys.h"
#include "mediaobj.h"
#include "mmreg.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfobjects_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
typedef ULONGLONG QWORD;

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum _MF_ATTRIBUTE_TYPE
    {
        MF_ATTRIBUTE_UINT32	= VT_UI4,
        MF_ATTRIBUTE_UINT64	= VT_UI8,
        MF_ATTRIBUTE_DOUBLE	= VT_R8,
        MF_ATTRIBUTE_GUID	= VT_CLSID,
        MF_ATTRIBUTE_STRING	= VT_LPWSTR,
        MF_ATTRIBUTE_BLOB	= ( VT_VECTOR | VT_UI1 ) ,
        MF_ATTRIBUTE_IUNKNOWN	= VT_UNKNOWN
    } 	MF_ATTRIBUTE_TYPE;

typedef 
enum _MF_ATTRIBUTES_MATCH_TYPE
    {
        MF_ATTRIBUTES_MATCH_OUR_ITEMS	= 0,
        MF_ATTRIBUTES_MATCH_THEIR_ITEMS	= 1,
        MF_ATTRIBUTES_MATCH_ALL_ITEMS	= 2,
        MF_ATTRIBUTES_MATCH_INTERSECTION	= 3,
        MF_ATTRIBUTES_MATCH_SMALLER	= 4
    } 	MF_ATTRIBUTES_MATCH_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0000_v0_0_s_ifspec;

#ifndef __IMFAttributes_INTERFACE_DEFINED__
#define __IMFAttributes_INTERFACE_DEFINED__

/* interface IMFAttributes */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFAttributes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2cd2d921-c447-44a7-a13c-4adabfc247e3")
    IMFAttributes : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetItem( 
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemType( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareItem( 
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Compare( 
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUINT32( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUINT64( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDouble( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGUID( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringLength( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllocatedString( 
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBlobSize( 
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBlob( 
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllocatedBlob( 
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUnknown( 
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetItem( 
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteItem( 
            __RPC__in REFGUID guidKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAllItems( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUINT32( 
            __RPC__in REFGUID guidKey,
            UINT32 unValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUINT64( 
            __RPC__in REFGUID guidKey,
            UINT64 unValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDouble( 
            __RPC__in REFGUID guidKey,
            double fValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGUID( 
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetString( 
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBlob( 
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUnknown( 
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out UINT32 *pcItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemByIndex( 
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyAllItems( 
            /* [in] */ __RPC__in_opt IMFAttributes *pDest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAttributesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFAttributes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFAttributes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFAttributes * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFAttributes * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFAttributes * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFAttributes * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFAttributes * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFAttributes * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFAttributes * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFAttributes * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFAttributes * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        END_INTERFACE
    } IMFAttributesVtbl;

    interface IMFAttributes
    {
        CONST_VTBL struct IMFAttributesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAttributes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAttributes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAttributes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAttributes_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFAttributes_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFAttributes_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFAttributes_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFAttributes_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFAttributes_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFAttributes_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFAttributes_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFAttributes_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFAttributes_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFAttributes_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFAttributes_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFAttributes_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFAttributes_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFAttributes_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFAttributes_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFAttributes_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFAttributes_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFAttributes_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFAttributes_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFAttributes_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFAttributes_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFAttributes_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFAttributes_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFAttributes_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFAttributes_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFAttributes_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFAttributes_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFAttributes_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFAttributes_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAttributes_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0001 */
/* [local] */ 


enum MF_ATTRIBUTE_SERIALIZE_OPTIONS
    {
        MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF	= 0x1
    } ;
STDAPI MFSerializeAttributesToStream(
  IMFAttributes * pAttr, 
  DWORD dwOptions, 
  IStream * pStm);
STDAPI MFDeserializeAttributesFromStream(
  IMFAttributes * pAttr, 
  DWORD dwOptions, 
  IStream * pStm);


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0001_v0_0_s_ifspec;

#ifndef __IMFMediaBuffer_INTERFACE_DEFINED__
#define __IMFMediaBuffer_INTERFACE_DEFINED__

/* interface IMFMediaBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("045FA593-8799-42b8-BC8D-8968C6453507")
    IMFMediaBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lock( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_to_(*pcbMaxLength, *pcbCurrentLength)  BYTE **ppbBuffer,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbMaxLength,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbCurrentLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unlock( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentLength( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCurrentLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentLength( 
            /* [in] */ DWORD cbCurrentLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxLength( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFMediaBuffer, Lock)
        HRESULT ( STDMETHODCALLTYPE *Lock )( 
            IMFMediaBuffer * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_to_(*pcbMaxLength, *pcbCurrentLength)  BYTE **ppbBuffer,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbMaxLength,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbCurrentLength);
        
        DECLSPEC_XFGVIRT(IMFMediaBuffer, Unlock)
        HRESULT ( STDMETHODCALLTYPE *Unlock )( 
            IMFMediaBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFMediaBuffer, GetCurrentLength)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLength )( 
            IMFMediaBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCurrentLength);
        
        DECLSPEC_XFGVIRT(IMFMediaBuffer, SetCurrentLength)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentLength )( 
            IMFMediaBuffer * This,
            /* [in] */ DWORD cbCurrentLength);
        
        DECLSPEC_XFGVIRT(IMFMediaBuffer, GetMaxLength)
        HRESULT ( STDMETHODCALLTYPE *GetMaxLength )( 
            IMFMediaBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLength);
        
        END_INTERFACE
    } IMFMediaBufferVtbl;

    interface IMFMediaBuffer
    {
        CONST_VTBL struct IMFMediaBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaBuffer_Lock(This,ppbBuffer,pcbMaxLength,pcbCurrentLength)	\
    ( (This)->lpVtbl -> Lock(This,ppbBuffer,pcbMaxLength,pcbCurrentLength) ) 

#define IMFMediaBuffer_Unlock(This)	\
    ( (This)->lpVtbl -> Unlock(This) ) 

#define IMFMediaBuffer_GetCurrentLength(This,pcbCurrentLength)	\
    ( (This)->lpVtbl -> GetCurrentLength(This,pcbCurrentLength) ) 

#define IMFMediaBuffer_SetCurrentLength(This,cbCurrentLength)	\
    ( (This)->lpVtbl -> SetCurrentLength(This,cbCurrentLength) ) 

#define IMFMediaBuffer_GetMaxLength(This,pcbMaxLength)	\
    ( (This)->lpVtbl -> GetMaxLength(This,pcbMaxLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaBuffer_INTERFACE_DEFINED__ */


#ifndef __IMFSample_INTERFACE_DEFINED__
#define __IMFSample_INTERFACE_DEFINED__

/* interface IMFSample */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c40a00f2-b93a-4d80-ae8c-5a1c634f58e4")
    IMFSample : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSampleFlags( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSampleFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleFlags( 
            /* [in] */ DWORD dwSampleFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSampleTime( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleTime( 
            /* [in] */ LONGLONG hnsSampleTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSampleDuration( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleDuration( 
            /* [in] */ LONGLONG hnsSampleDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBufferCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwBufferCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBufferByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaBuffer **ppBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertToContiguousBuffer( 
            /* [annotation][out] */ 
            _Out_  IMFMediaBuffer **ppBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddBuffer( 
            /* [in] */ IMFMediaBuffer *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveBufferByIndex( 
            /* [in] */ DWORD dwIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllBuffers( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalLength( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcbTotalLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyToBuffer( 
            /* [in] */ IMFMediaBuffer *pBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFSample * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFSample * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFSample * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFSample * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFSample * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFSample * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFSample * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFSample * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFSample * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFSample * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFSample * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFSample * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFSample * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFSample, GetSampleFlags)
        HRESULT ( STDMETHODCALLTYPE *GetSampleFlags )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSampleFlags);
        
        DECLSPEC_XFGVIRT(IMFSample, SetSampleFlags)
        HRESULT ( STDMETHODCALLTYPE *SetSampleFlags )( 
            IMFSample * This,
            /* [in] */ DWORD dwSampleFlags);
        
        DECLSPEC_XFGVIRT(IMFSample, GetSampleTime)
        HRESULT ( STDMETHODCALLTYPE *GetSampleTime )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleTime);
        
        DECLSPEC_XFGVIRT(IMFSample, SetSampleTime)
        HRESULT ( STDMETHODCALLTYPE *SetSampleTime )( 
            IMFSample * This,
            /* [in] */ LONGLONG hnsSampleTime);
        
        DECLSPEC_XFGVIRT(IMFSample, GetSampleDuration)
        HRESULT ( STDMETHODCALLTYPE *GetSampleDuration )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleDuration);
        
        DECLSPEC_XFGVIRT(IMFSample, SetSampleDuration)
        HRESULT ( STDMETHODCALLTYPE *SetSampleDuration )( 
            IMFSample * This,
            /* [in] */ LONGLONG hnsSampleDuration);
        
        DECLSPEC_XFGVIRT(IMFSample, GetBufferCount)
        HRESULT ( STDMETHODCALLTYPE *GetBufferCount )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwBufferCount);
        
        DECLSPEC_XFGVIRT(IMFSample, GetBufferByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetBufferByIndex )( 
            IMFSample * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaBuffer **ppBuffer);
        
        DECLSPEC_XFGVIRT(IMFSample, ConvertToContiguousBuffer)
        HRESULT ( STDMETHODCALLTYPE *ConvertToContiguousBuffer )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  IMFMediaBuffer **ppBuffer);
        
        DECLSPEC_XFGVIRT(IMFSample, AddBuffer)
        HRESULT ( STDMETHODCALLTYPE *AddBuffer )( 
            IMFSample * This,
            /* [in] */ IMFMediaBuffer *pBuffer);
        
        DECLSPEC_XFGVIRT(IMFSample, RemoveBufferByIndex)
        HRESULT ( STDMETHODCALLTYPE *RemoveBufferByIndex )( 
            IMFSample * This,
            /* [in] */ DWORD dwIndex);
        
        DECLSPEC_XFGVIRT(IMFSample, RemoveAllBuffers)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllBuffers )( 
            IMFSample * This);
        
        DECLSPEC_XFGVIRT(IMFSample, GetTotalLength)
        HRESULT ( STDMETHODCALLTYPE *GetTotalLength )( 
            IMFSample * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbTotalLength);
        
        DECLSPEC_XFGVIRT(IMFSample, CopyToBuffer)
        HRESULT ( STDMETHODCALLTYPE *CopyToBuffer )( 
            IMFSample * This,
            /* [in] */ IMFMediaBuffer *pBuffer);
        
        END_INTERFACE
    } IMFSampleVtbl;

    interface IMFSample
    {
        CONST_VTBL struct IMFSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSample_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFSample_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFSample_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFSample_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFSample_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFSample_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFSample_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFSample_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFSample_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFSample_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFSample_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFSample_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFSample_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFSample_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFSample_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFSample_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFSample_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFSample_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFSample_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFSample_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFSample_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFSample_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFSample_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFSample_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFSample_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFSample_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFSample_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFSample_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFSample_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFSample_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFSample_GetSampleFlags(This,pdwSampleFlags)	\
    ( (This)->lpVtbl -> GetSampleFlags(This,pdwSampleFlags) ) 

#define IMFSample_SetSampleFlags(This,dwSampleFlags)	\
    ( (This)->lpVtbl -> SetSampleFlags(This,dwSampleFlags) ) 

#define IMFSample_GetSampleTime(This,phnsSampleTime)	\
    ( (This)->lpVtbl -> GetSampleTime(This,phnsSampleTime) ) 

#define IMFSample_SetSampleTime(This,hnsSampleTime)	\
    ( (This)->lpVtbl -> SetSampleTime(This,hnsSampleTime) ) 

#define IMFSample_GetSampleDuration(This,phnsSampleDuration)	\
    ( (This)->lpVtbl -> GetSampleDuration(This,phnsSampleDuration) ) 

#define IMFSample_SetSampleDuration(This,hnsSampleDuration)	\
    ( (This)->lpVtbl -> SetSampleDuration(This,hnsSampleDuration) ) 

#define IMFSample_GetBufferCount(This,pdwBufferCount)	\
    ( (This)->lpVtbl -> GetBufferCount(This,pdwBufferCount) ) 

#define IMFSample_GetBufferByIndex(This,dwIndex,ppBuffer)	\
    ( (This)->lpVtbl -> GetBufferByIndex(This,dwIndex,ppBuffer) ) 

#define IMFSample_ConvertToContiguousBuffer(This,ppBuffer)	\
    ( (This)->lpVtbl -> ConvertToContiguousBuffer(This,ppBuffer) ) 

#define IMFSample_AddBuffer(This,pBuffer)	\
    ( (This)->lpVtbl -> AddBuffer(This,pBuffer) ) 

#define IMFSample_RemoveBufferByIndex(This,dwIndex)	\
    ( (This)->lpVtbl -> RemoveBufferByIndex(This,dwIndex) ) 

#define IMFSample_RemoveAllBuffers(This)	\
    ( (This)->lpVtbl -> RemoveAllBuffers(This) ) 

#define IMFSample_GetTotalLength(This,pcbTotalLength)	\
    ( (This)->lpVtbl -> GetTotalLength(This,pcbTotalLength) ) 

#define IMFSample_CopyToBuffer(This,pBuffer)	\
    ( (This)->lpVtbl -> CopyToBuffer(This,pBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSample_INTERFACE_DEFINED__ */


#ifndef __IMF2DBuffer_INTERFACE_DEFINED__
#define __IMF2DBuffer_INTERFACE_DEFINED__

/* interface IMF2DBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMF2DBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7DC9D5F9-9ED9-44ec-9BBF-0600BB589FBB")
    IMF2DBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lock2D( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(_Inexpressible_(ComputePlaneSize(*plPitch)))  BYTE **ppbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unlock2D( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScanline0AndPitch( 
            /* [annotation][out] */ 
            _Out_  BYTE **pbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsContiguousFormat( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsContiguous) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContiguousLength( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcbLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ContiguousCopyTo( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(cbDestBuffer)  BYTE *pbDestBuffer,
            /* [in] */ DWORD cbDestBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ContiguousCopyFrom( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbSrcBuffer)  const BYTE *pbSrcBuffer,
            /* [in] */ DWORD cbSrcBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMF2DBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMF2DBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMF2DBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMF2DBuffer * This);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, Lock2D)
        HRESULT ( STDMETHODCALLTYPE *Lock2D )( 
            IMF2DBuffer * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(_Inexpressible_(ComputePlaneSize(*plPitch)))  BYTE **ppbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, Unlock2D)
        HRESULT ( STDMETHODCALLTYPE *Unlock2D )( 
            IMF2DBuffer * This);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, GetScanline0AndPitch)
        HRESULT ( STDMETHODCALLTYPE *GetScanline0AndPitch )( 
            IMF2DBuffer * This,
            /* [annotation][out] */ 
            _Out_  BYTE **pbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, IsContiguousFormat)
        HRESULT ( STDMETHODCALLTYPE *IsContiguousFormat )( 
            IMF2DBuffer * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsContiguous);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, GetContiguousLength)
        HRESULT ( STDMETHODCALLTYPE *GetContiguousLength )( 
            IMF2DBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbLength);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, ContiguousCopyTo)
        HRESULT ( STDMETHODCALLTYPE *ContiguousCopyTo )( 
            IMF2DBuffer * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(cbDestBuffer)  BYTE *pbDestBuffer,
            /* [in] */ DWORD cbDestBuffer);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, ContiguousCopyFrom)
        HRESULT ( STDMETHODCALLTYPE *ContiguousCopyFrom )( 
            IMF2DBuffer * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbSrcBuffer)  const BYTE *pbSrcBuffer,
            /* [in] */ DWORD cbSrcBuffer);
        
        END_INTERFACE
    } IMF2DBufferVtbl;

    interface IMF2DBuffer
    {
        CONST_VTBL struct IMF2DBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMF2DBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMF2DBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMF2DBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMF2DBuffer_Lock2D(This,ppbScanline0,plPitch)	\
    ( (This)->lpVtbl -> Lock2D(This,ppbScanline0,plPitch) ) 

#define IMF2DBuffer_Unlock2D(This)	\
    ( (This)->lpVtbl -> Unlock2D(This) ) 

#define IMF2DBuffer_GetScanline0AndPitch(This,pbScanline0,plPitch)	\
    ( (This)->lpVtbl -> GetScanline0AndPitch(This,pbScanline0,plPitch) ) 

#define IMF2DBuffer_IsContiguousFormat(This,pfIsContiguous)	\
    ( (This)->lpVtbl -> IsContiguousFormat(This,pfIsContiguous) ) 

#define IMF2DBuffer_GetContiguousLength(This,pcbLength)	\
    ( (This)->lpVtbl -> GetContiguousLength(This,pcbLength) ) 

#define IMF2DBuffer_ContiguousCopyTo(This,pbDestBuffer,cbDestBuffer)	\
    ( (This)->lpVtbl -> ContiguousCopyTo(This,pbDestBuffer,cbDestBuffer) ) 

#define IMF2DBuffer_ContiguousCopyFrom(This,pbSrcBuffer,cbSrcBuffer)	\
    ( (This)->lpVtbl -> ContiguousCopyFrom(This,pbSrcBuffer,cbSrcBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMF2DBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0004 */
/* [local] */ 

typedef 
enum _MF2DBuffer_LockFlags
    {
        MF2DBuffer_LockFlags_LockTypeMask	= ( ( 0x1 | 0x2 )  | 0x3 ) ,
        MF2DBuffer_LockFlags_Read	= 0x1,
        MF2DBuffer_LockFlags_Write	= 0x2,
        MF2DBuffer_LockFlags_ReadWrite	= 0x3,
        MF2DBuffer_LockFlags_ForceDWORD	= 0x7fffffff
    } 	MF2DBuffer_LockFlags;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0004_v0_0_s_ifspec;

#ifndef __IMF2DBuffer2_INTERFACE_DEFINED__
#define __IMF2DBuffer2_INTERFACE_DEFINED__

/* interface IMF2DBuffer2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMF2DBuffer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33ae5ea6-4316-436f-8ddd-d73d22f829ec")
    IMF2DBuffer2 : public IMF2DBuffer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lock2DSize( 
            /* [annotation][in] */ 
            _In_  MF2DBuffer_LockFlags lockFlags,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(_Inexpressible_(ComputePlaneSize(*plPitch)))  BYTE **ppbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbBufferLength)  BYTE **ppbBufferStart,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Copy2DTo( 
            /* [annotation][in] */ 
            _In_  IMF2DBuffer2 *pDestBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMF2DBuffer2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMF2DBuffer2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMF2DBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMF2DBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, Lock2D)
        HRESULT ( STDMETHODCALLTYPE *Lock2D )( 
            IMF2DBuffer2 * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(_Inexpressible_(ComputePlaneSize(*plPitch)))  BYTE **ppbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, Unlock2D)
        HRESULT ( STDMETHODCALLTYPE *Unlock2D )( 
            IMF2DBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, GetScanline0AndPitch)
        HRESULT ( STDMETHODCALLTYPE *GetScanline0AndPitch )( 
            IMF2DBuffer2 * This,
            /* [annotation][out] */ 
            _Out_  BYTE **pbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, IsContiguousFormat)
        HRESULT ( STDMETHODCALLTYPE *IsContiguousFormat )( 
            IMF2DBuffer2 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsContiguous);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, GetContiguousLength)
        HRESULT ( STDMETHODCALLTYPE *GetContiguousLength )( 
            IMF2DBuffer2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbLength);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, ContiguousCopyTo)
        HRESULT ( STDMETHODCALLTYPE *ContiguousCopyTo )( 
            IMF2DBuffer2 * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(cbDestBuffer)  BYTE *pbDestBuffer,
            /* [in] */ DWORD cbDestBuffer);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer, ContiguousCopyFrom)
        HRESULT ( STDMETHODCALLTYPE *ContiguousCopyFrom )( 
            IMF2DBuffer2 * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbSrcBuffer)  const BYTE *pbSrcBuffer,
            /* [in] */ DWORD cbSrcBuffer);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer2, Lock2DSize)
        HRESULT ( STDMETHODCALLTYPE *Lock2DSize )( 
            IMF2DBuffer2 * This,
            /* [annotation][in] */ 
            _In_  MF2DBuffer_LockFlags lockFlags,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(_Inexpressible_(ComputePlaneSize(*plPitch)))  BYTE **ppbScanline0,
            /* [annotation][out] */ 
            _Out_  LONG *plPitch,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbBufferLength)  BYTE **ppbBufferStart,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbBufferLength);
        
        DECLSPEC_XFGVIRT(IMF2DBuffer2, Copy2DTo)
        HRESULT ( STDMETHODCALLTYPE *Copy2DTo )( 
            IMF2DBuffer2 * This,
            /* [annotation][in] */ 
            _In_  IMF2DBuffer2 *pDestBuffer);
        
        END_INTERFACE
    } IMF2DBuffer2Vtbl;

    interface IMF2DBuffer2
    {
        CONST_VTBL struct IMF2DBuffer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMF2DBuffer2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMF2DBuffer2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMF2DBuffer2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMF2DBuffer2_Lock2D(This,ppbScanline0,plPitch)	\
    ( (This)->lpVtbl -> Lock2D(This,ppbScanline0,plPitch) ) 

#define IMF2DBuffer2_Unlock2D(This)	\
    ( (This)->lpVtbl -> Unlock2D(This) ) 

#define IMF2DBuffer2_GetScanline0AndPitch(This,pbScanline0,plPitch)	\
    ( (This)->lpVtbl -> GetScanline0AndPitch(This,pbScanline0,plPitch) ) 

#define IMF2DBuffer2_IsContiguousFormat(This,pfIsContiguous)	\
    ( (This)->lpVtbl -> IsContiguousFormat(This,pfIsContiguous) ) 

#define IMF2DBuffer2_GetContiguousLength(This,pcbLength)	\
    ( (This)->lpVtbl -> GetContiguousLength(This,pcbLength) ) 

#define IMF2DBuffer2_ContiguousCopyTo(This,pbDestBuffer,cbDestBuffer)	\
    ( (This)->lpVtbl -> ContiguousCopyTo(This,pbDestBuffer,cbDestBuffer) ) 

#define IMF2DBuffer2_ContiguousCopyFrom(This,pbSrcBuffer,cbSrcBuffer)	\
    ( (This)->lpVtbl -> ContiguousCopyFrom(This,pbSrcBuffer,cbSrcBuffer) ) 


#define IMF2DBuffer2_Lock2DSize(This,lockFlags,ppbScanline0,plPitch,ppbBufferStart,pcbBufferLength)	\
    ( (This)->lpVtbl -> Lock2DSize(This,lockFlags,ppbScanline0,plPitch,ppbBufferStart,pcbBufferLength) ) 

#define IMF2DBuffer2_Copy2DTo(This,pDestBuffer)	\
    ( (This)->lpVtbl -> Copy2DTo(This,pDestBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMF2DBuffer2_INTERFACE_DEFINED__ */


#ifndef __IMFDXGIBuffer_INTERFACE_DEFINED__
#define __IMFDXGIBuffer_INTERFACE_DEFINED__

/* interface IMFDXGIBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFDXGIBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7174cfa-1c9e-48b1-8866-626226bfc258")
    IMFDXGIBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetResource( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubresourceIndex( 
            /* [annotation][out] */ 
            _Out_  UINT *puSubresource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUnknown( 
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUnknown( 
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnkData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDXGIBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDXGIBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDXGIBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDXGIBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFDXGIBuffer, GetResource)
        HRESULT ( STDMETHODCALLTYPE *GetResource )( 
            IMFDXGIBuffer * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFDXGIBuffer, GetSubresourceIndex)
        HRESULT ( STDMETHODCALLTYPE *GetSubresourceIndex )( 
            IMFDXGIBuffer * This,
            /* [annotation][out] */ 
            _Out_  UINT *puSubresource);
        
        DECLSPEC_XFGVIRT(IMFDXGIBuffer, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFDXGIBuffer * This,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFDXGIBuffer, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFDXGIBuffer * This,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnkData);
        
        END_INTERFACE
    } IMFDXGIBufferVtbl;

    interface IMFDXGIBuffer
    {
        CONST_VTBL struct IMFDXGIBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDXGIBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDXGIBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDXGIBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDXGIBuffer_GetResource(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetResource(This,riid,ppvObject) ) 

#define IMFDXGIBuffer_GetSubresourceIndex(This,puSubresource)	\
    ( (This)->lpVtbl -> GetSubresourceIndex(This,puSubresource) ) 

#define IMFDXGIBuffer_GetUnknown(This,guid,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetUnknown(This,guid,riid,ppvObject) ) 

#define IMFDXGIBuffer_SetUnknown(This,guid,pUnkData)	\
    ( (This)->lpVtbl -> SetUnknown(This,guid,pUnkData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDXGIBuffer_INTERFACE_DEFINED__ */


#ifndef __IMFDXGICrossAdapterBuffer_INTERFACE_DEFINED__
#define __IMFDXGICrossAdapterBuffer_INTERFACE_DEFINED__

/* interface IMFDXGICrossAdapterBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFDXGICrossAdapterBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B25D03FB-D148-45EF-BFED-F778B7566C07")
    IMFDXGICrossAdapterBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetResourceForDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubresourceIndexForDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][out] */ 
            _Out_  UINT *puSubresource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUnknownForDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  LPVOID *ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUnknownForDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnkData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDXGICrossAdapterBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDXGICrossAdapterBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDXGICrossAdapterBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDXGICrossAdapterBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFDXGICrossAdapterBuffer, GetResourceForDevice)
        HRESULT ( STDMETHODCALLTYPE *GetResourceForDevice )( 
            IMFDXGICrossAdapterBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFDXGICrossAdapterBuffer, GetSubresourceIndexForDevice)
        HRESULT ( STDMETHODCALLTYPE *GetSubresourceIndexForDevice )( 
            IMFDXGICrossAdapterBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][out] */ 
            _Out_  UINT *puSubresource);
        
        DECLSPEC_XFGVIRT(IMFDXGICrossAdapterBuffer, GetUnknownForDevice)
        HRESULT ( STDMETHODCALLTYPE *GetUnknownForDevice )( 
            IMFDXGICrossAdapterBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _COM_Outptr_  LPVOID *ppvObject);
        
        DECLSPEC_XFGVIRT(IMFDXGICrossAdapterBuffer, SetUnknownForDevice)
        HRESULT ( STDMETHODCALLTYPE *SetUnknownForDevice )( 
            IMFDXGICrossAdapterBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation][in] */ 
            _In_  REFIID guid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnkData);
        
        END_INTERFACE
    } IMFDXGICrossAdapterBufferVtbl;

    interface IMFDXGICrossAdapterBuffer
    {
        CONST_VTBL struct IMFDXGICrossAdapterBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDXGICrossAdapterBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDXGICrossAdapterBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDXGICrossAdapterBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDXGICrossAdapterBuffer_GetResourceForDevice(This,pUnkDevice,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetResourceForDevice(This,pUnkDevice,riid,ppvObject) ) 

#define IMFDXGICrossAdapterBuffer_GetSubresourceIndexForDevice(This,pUnkDevice,puSubresource)	\
    ( (This)->lpVtbl -> GetSubresourceIndexForDevice(This,pUnkDevice,puSubresource) ) 

#define IMFDXGICrossAdapterBuffer_GetUnknownForDevice(This,pUnkDevice,guid,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetUnknownForDevice(This,pUnkDevice,guid,riid,ppvObject) ) 

#define IMFDXGICrossAdapterBuffer_SetUnknownForDevice(This,pUnkDevice,guid,pUnkData)	\
    ( (This)->lpVtbl -> SetUnknownForDevice(This,pUnkDevice,guid,pUnkData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDXGICrossAdapterBuffer_INTERFACE_DEFINED__ */


#ifndef __IMFMediaType_INTERFACE_DEFINED__
#define __IMFMediaType_INTERFACE_DEFINED__

/* interface IMFMediaType */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("44ae0fa8-ea31-4109-8d2e-4cae4997c555")
    IMFMediaType : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMajorType( 
            /* [out] */ __RPC__out GUID *pguidMajorType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCompressedFormat( 
            /* [out] */ __RPC__out BOOL *pfCompressed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt IMFMediaType *pIMediaType,
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetRepresentation( 
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FreeRepresentation( 
            /* [in] */ GUID guidRepresentation,
            /* [in] */ LPVOID pvRepresentation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFMediaType * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFMediaType * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFMediaType * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFMediaType * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFMediaType * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetMajorType)
        HRESULT ( STDMETHODCALLTYPE *GetMajorType )( 
            __RPC__in IMFMediaType * This,
            /* [out] */ __RPC__out GUID *pguidMajorType);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsCompressedFormat)
        HRESULT ( STDMETHODCALLTYPE *IsCompressedFormat )( 
            __RPC__in IMFMediaType * This,
            /* [out] */ __RPC__out BOOL *pfCompressed);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IMFMediaType * This,
            /* [in] */ __RPC__in_opt IMFMediaType *pIMediaType,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetRepresentation )( 
            IMFMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation);
        
        DECLSPEC_XFGVIRT(IMFMediaType, FreeRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FreeRepresentation )( 
            IMFMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [in] */ LPVOID pvRepresentation);
        
        END_INTERFACE
    } IMFMediaTypeVtbl;

    interface IMFMediaType
    {
        CONST_VTBL struct IMFMediaTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaType_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFMediaType_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFMediaType_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFMediaType_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFMediaType_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFMediaType_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFMediaType_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFMediaType_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFMediaType_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFMediaType_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFMediaType_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFMediaType_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFMediaType_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFMediaType_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFMediaType_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFMediaType_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFMediaType_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFMediaType_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFMediaType_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFMediaType_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFMediaType_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFMediaType_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFMediaType_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFMediaType_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFMediaType_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFMediaType_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFMediaType_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFMediaType_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFMediaType_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFMediaType_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFMediaType_GetMajorType(This,pguidMajorType)	\
    ( (This)->lpVtbl -> GetMajorType(This,pguidMajorType) ) 

#define IMFMediaType_IsCompressedFormat(This,pfCompressed)	\
    ( (This)->lpVtbl -> IsCompressedFormat(This,pfCompressed) ) 

#define IMFMediaType_IsEqual(This,pIMediaType,pdwFlags)	\
    ( (This)->lpVtbl -> IsEqual(This,pIMediaType,pdwFlags) ) 

#define IMFMediaType_GetRepresentation(This,guidRepresentation,ppvRepresentation)	\
    ( (This)->lpVtbl -> GetRepresentation(This,guidRepresentation,ppvRepresentation) ) 

#define IMFMediaType_FreeRepresentation(This,guidRepresentation,pvRepresentation)	\
    ( (This)->lpVtbl -> FreeRepresentation(This,guidRepresentation,pvRepresentation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaType_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0008 */
/* [local] */ 

#define MF_MEDIATYPE_EQUAL_MAJOR_TYPES  0x00000001
#define MF_MEDIATYPE_EQUAL_FORMAT_TYPES 0x00000002
#define MF_MEDIATYPE_EQUAL_FORMAT_DATA  0x00000004
#define MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA  0x00000008

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0008_v0_0_s_ifspec;

#ifndef __IMFAudioMediaType_INTERFACE_DEFINED__
#define __IMFAudioMediaType_INTERFACE_DEFINED__

/* interface IMFAudioMediaType */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFAudioMediaType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26a0adc3-ce26-4672-9304-69552edd3faf")
    IMFAudioMediaType : public IMFMediaType
    {
    public:
        virtual const WAVEFORMATEX *STDMETHODCALLTYPE GetAudioFormat( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAudioMediaTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFAudioMediaType * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFAudioMediaType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFAudioMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFAudioMediaType * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFAudioMediaType * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFAudioMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFAudioMediaType * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFAudioMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFAudioMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFAudioMediaType * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFAudioMediaType * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFAudioMediaType * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetMajorType)
        HRESULT ( STDMETHODCALLTYPE *GetMajorType )( 
            IMFAudioMediaType * This,
            /* [out] */ GUID *pguidMajorType);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsCompressedFormat)
        HRESULT ( STDMETHODCALLTYPE *IsCompressedFormat )( 
            IMFAudioMediaType * This,
            /* [out] */ BOOL *pfCompressed);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            IMFAudioMediaType * This,
            /* [in] */ IMFMediaType *pIMediaType,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetRepresentation )( 
            IMFAudioMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation);
        
        DECLSPEC_XFGVIRT(IMFMediaType, FreeRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FreeRepresentation )( 
            IMFAudioMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [in] */ LPVOID pvRepresentation);
        
        DECLSPEC_XFGVIRT(IMFAudioMediaType, GetAudioFormat)
        const WAVEFORMATEX *( STDMETHODCALLTYPE *GetAudioFormat )( 
            IMFAudioMediaType * This);
        
        END_INTERFACE
    } IMFAudioMediaTypeVtbl;

    interface IMFAudioMediaType
    {
        CONST_VTBL struct IMFAudioMediaTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAudioMediaType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAudioMediaType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAudioMediaType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAudioMediaType_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFAudioMediaType_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFAudioMediaType_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFAudioMediaType_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFAudioMediaType_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFAudioMediaType_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFAudioMediaType_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFAudioMediaType_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFAudioMediaType_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFAudioMediaType_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFAudioMediaType_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFAudioMediaType_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFAudioMediaType_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFAudioMediaType_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFAudioMediaType_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFAudioMediaType_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFAudioMediaType_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFAudioMediaType_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFAudioMediaType_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFAudioMediaType_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFAudioMediaType_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFAudioMediaType_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFAudioMediaType_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFAudioMediaType_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFAudioMediaType_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFAudioMediaType_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFAudioMediaType_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFAudioMediaType_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFAudioMediaType_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFAudioMediaType_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFAudioMediaType_GetMajorType(This,pguidMajorType)	\
    ( (This)->lpVtbl -> GetMajorType(This,pguidMajorType) ) 

#define IMFAudioMediaType_IsCompressedFormat(This,pfCompressed)	\
    ( (This)->lpVtbl -> IsCompressedFormat(This,pfCompressed) ) 

#define IMFAudioMediaType_IsEqual(This,pIMediaType,pdwFlags)	\
    ( (This)->lpVtbl -> IsEqual(This,pIMediaType,pdwFlags) ) 

#define IMFAudioMediaType_GetRepresentation(This,guidRepresentation,ppvRepresentation)	\
    ( (This)->lpVtbl -> GetRepresentation(This,guidRepresentation,ppvRepresentation) ) 

#define IMFAudioMediaType_FreeRepresentation(This,guidRepresentation,pvRepresentation)	\
    ( (This)->lpVtbl -> FreeRepresentation(This,guidRepresentation,pvRepresentation) ) 


#define IMFAudioMediaType_GetAudioFormat(This)	\
    ( (This)->lpVtbl -> GetAudioFormat(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAudioMediaType_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0009 */
/* [local] */ 

#ifndef _WINGDI_
typedef DWORD RGBQUAD;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_mfobjects_0000_0009_0001
    {
    DWORD biSize;
    LONG biWidth;
    LONG biHeight;
    WORD biPlanes;
    WORD biBitCount;
    DWORD biCompression;
    DWORD biSizeImage;
    LONG biXPelsPerMeter;
    LONG biYPelsPerMeter;
    DWORD biClrUsed;
    DWORD biClrImportant;
    } 	BITMAPINFOHEADER;

typedef /* [public] */ struct __MIDL___MIDL_itf_mfobjects_0000_0009_0002
    {
    BITMAPINFOHEADER bmiHeader;
    RGBQUAD bmiColors[ 1 ];
    } 	BITMAPINFO;

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef /* [public] */ struct __MIDL___MIDL_itf_mfobjects_0000_0009_0003
    {
    GUID guidMajorType;
    GUID guidSubtype;
    } 	MFT_REGISTER_TYPE_INFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if !defined( _MFVIDEOFORMAT_ )
#define _MFVIDEOFORMAT_
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum _MFVideoInterlaceMode
    {
        MFVideoInterlace_Unknown	= 0,
        MFVideoInterlace_Progressive	= 2,
        MFVideoInterlace_FieldInterleavedUpperFirst	= 3,
        MFVideoInterlace_FieldInterleavedLowerFirst	= 4,
        MFVideoInterlace_FieldSingleUpper	= 5,
        MFVideoInterlace_FieldSingleLower	= 6,
        MFVideoInterlace_MixedInterlaceOrProgressive	= 7,
        MFVideoInterlace_Last	= ( MFVideoInterlace_MixedInterlaceOrProgressive + 1 ) ,
        MFVideoInterlace_ForceDWORD	= 0x7fffffff
    } 	MFVideoInterlaceMode;

#define MFVideoInterlace_FieldSingleUpperFirst MFVideoInterlace_FieldSingleUpper
#define MFVideoInterlace_FieldSingleLowerFirst MFVideoInterlace_FieldSingleLower
typedef 
enum _MFVideoTransferFunction
    {
        MFVideoTransFunc_Unknown	= 0,
        MFVideoTransFunc_10	= 1,
        MFVideoTransFunc_18	= 2,
        MFVideoTransFunc_20	= 3,
        MFVideoTransFunc_22	= 4,
        MFVideoTransFunc_709	= 5,
        MFVideoTransFunc_240M	= 6,
        MFVideoTransFunc_sRGB	= 7,
        MFVideoTransFunc_28	= 8,
        MFVideoTransFunc_Log_100	= 9,
        MFVideoTransFunc_Log_316	= 10,
        MFVideoTransFunc_709_sym	= 11,
        MFVideoTransFunc_2020_const	= 12,
        MFVideoTransFunc_2020	= 13,
        MFVideoTransFunc_26	= 14,
        MFVideoTransFunc_2084	= 15,
        MFVideoTransFunc_HLG	= 16,
        MFVideoTransFunc_10_rel	= 17,
        MFVideoTransFunc_BT1361_ECG	= 18,
        MFVideoTransFunc_SMPTE428	= 19,
        MFVideoTransFunc_Last	= ( MFVideoTransFunc_SMPTE428 + 1 ) ,
        MFVideoTransFunc_ForceDWORD	= 0x7fffffff
    } 	MFVideoTransferFunction;

typedef 
enum _MFVideoPrimaries
    {
        MFVideoPrimaries_Unknown	= 0,
        MFVideoPrimaries_reserved	= 1,
        MFVideoPrimaries_BT709	= 2,
        MFVideoPrimaries_BT470_2_SysM	= 3,
        MFVideoPrimaries_BT470_2_SysBG	= 4,
        MFVideoPrimaries_SMPTE170M	= 5,
        MFVideoPrimaries_SMPTE240M	= 6,
        MFVideoPrimaries_EBU3213	= 7,
        MFVideoPrimaries_SMPTE_C	= 8,
        MFVideoPrimaries_BT2020	= 9,
        MFVideoPrimaries_XYZ	= 10,
        MFVideoPrimaries_DCI_P3	= 11,
        MFVideoPrimaries_ACES	= 12,
        MFVideoPrimaries_Display_P3	= 13,
        MFVideoPrimaries_Last	= ( MFVideoPrimaries_Display_P3 + 1 ) ,
        MFVideoPrimaries_ForceDWORD	= 0x7fffffff
    } 	MFVideoPrimaries;

typedef 
enum _MFVideoLighting
    {
        MFVideoLighting_Unknown	= 0,
        MFVideoLighting_bright	= 1,
        MFVideoLighting_office	= 2,
        MFVideoLighting_dim	= 3,
        MFVideoLighting_dark	= 4,
        MFVideoLighting_Last	= ( MFVideoLighting_dark + 1 ) ,
        MFVideoLighting_ForceDWORD	= 0x7fffffff
    } 	MFVideoLighting;

typedef 
enum _MFVideoTransferMatrix
    {
        MFVideoTransferMatrix_Unknown	= 0,
        MFVideoTransferMatrix_BT709	= 1,
        MFVideoTransferMatrix_BT601	= 2,
        MFVideoTransferMatrix_SMPTE240M	= 3,
        MFVideoTransferMatrix_BT2020_10	= 4,
        MFVideoTransferMatrix_BT2020_12	= 5,
        MFVideoTransferMatrix_Identity	= 6,
        MFVideoTransferMatrix_FCC47	= 7,
        MFVideoTransferMatrix_YCgCo	= 8,
        MFVideoTransferMatrix_SMPTE2085	= 9,
        MFVideoTransferMatrix_Chroma	= 10,
        MFVideoTransferMatrix_Chroma_const	= 11,
        MFVideoTransferMatrix_ICtCp	= 12,
        MFVideoTransferMatrix_Last	= ( MFVideoTransferMatrix_ICtCp + 1 ) ,
        MFVideoTransferMatrix_ForceDWORD	= 0x7fffffff
    } 	MFVideoTransferMatrix;

typedef 
enum _MFVideoChromaSubsampling
    {
        MFVideoChromaSubsampling_Unknown	= 0,
        MFVideoChromaSubsampling_ProgressiveChroma	= 0x8,
        MFVideoChromaSubsampling_Horizontally_Cosited	= 0x4,
        MFVideoChromaSubsampling_Vertically_Cosited	= 0x2,
        MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes	= 0x1,
        MFVideoChromaSubsampling_MPEG2	= ( MFVideoChromaSubsampling_Horizontally_Cosited | MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes ) ,
        MFVideoChromaSubsampling_MPEG1	= MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
        MFVideoChromaSubsampling_DV_PAL	= ( MFVideoChromaSubsampling_Horizontally_Cosited | MFVideoChromaSubsampling_Vertically_Cosited ) ,
        MFVideoChromaSubsampling_Cosited	= ( ( MFVideoChromaSubsampling_Horizontally_Cosited | MFVideoChromaSubsampling_Vertically_Cosited )  | MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes ) ,
        MFVideoChromaSubsampling_Last	= ( MFVideoChromaSubsampling_Cosited + 1 ) ,
        MFVideoChromaSubsampling_ForceDWORD	= 0x7fffffff
    } 	MFVideoChromaSubsampling;

typedef 
enum _MFNominalRange
    {
        MFNominalRange_Unknown	= 0,
        MFNominalRange_Normal	= 1,
        MFNominalRange_Wide	= 2,
        MFNominalRange_0_255	= 1,
        MFNominalRange_16_235	= 2,
        MFNominalRange_48_208	= 3,
        MFNominalRange_64_127	= 4,
        MFNominalRange_Last	= ( MFNominalRange_64_127 + 1 ) ,
        MFNominalRange_ForceDWORD	= 0x7fffffff
    } 	MFNominalRange;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
typedef 
enum _MFVideoFlags
    {
        MFVideoFlag_PAD_TO_Mask	= ( 0x1 | 0x2 ) ,
        MFVideoFlag_PAD_TO_None	= ( 0 * 0x1 ) ,
        MFVideoFlag_PAD_TO_4x3	= ( 1 * 0x1 ) ,
        MFVideoFlag_PAD_TO_16x9	= ( 2 * 0x1 ) ,
        MFVideoFlag_SrcContentHintMask	= ( ( 0x4 | 0x8 )  | 0x10 ) ,
        MFVideoFlag_SrcContentHintNone	= ( 0 * 0x4 ) ,
        MFVideoFlag_SrcContentHint16x9	= ( 1 * 0x4 ) ,
        MFVideoFlag_SrcContentHint235_1	= ( 2 * 0x4 ) ,
        MFVideoFlag_AnalogProtected	= 0x20,
        MFVideoFlag_DigitallyProtected	= 0x40,
        MFVideoFlag_ProgressiveContent	= 0x80,
        MFVideoFlag_FieldRepeatCountMask	= ( ( 0x100 | 0x200 )  | 0x400 ) ,
        MFVideoFlag_FieldRepeatCountShift	= 8,
        MFVideoFlag_ProgressiveSeqReset	= 0x800,
        MFVideoFlag_PanScanEnabled	= 0x20000,
        MFVideoFlag_LowerFieldFirst	= 0x40000,
        MFVideoFlag_BottomUpLinearRep	= 0x80000,
        MFVideoFlags_DXVASurface	= 0x100000,
        MFVideoFlags_RenderTargetSurface	= 0x400000,
        MFVideoFlags_ForceQWORD	= 0x7fffffff
    } 	MFVideoFlags;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef struct _MFRatio
    {
    DWORD Numerator;
    DWORD Denominator;
    } 	MFRatio;

typedef struct _MFOffset
    {
    WORD fract;
    short value;
    } 	MFOffset;

typedef struct _MFVideoArea
    {
    MFOffset OffsetX;
    MFOffset OffsetY;
    SIZE Area;
    } 	MFVideoArea;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
typedef struct _MFVideoInfo
    {
    DWORD dwWidth;
    DWORD dwHeight;
    MFRatio PixelAspectRatio;
    MFVideoChromaSubsampling SourceChromaSubsampling;
    MFVideoInterlaceMode InterlaceMode;
    MFVideoTransferFunction TransferFunction;
    MFVideoPrimaries ColorPrimaries;
    MFVideoTransferMatrix TransferMatrix;
    MFVideoLighting SourceLighting;
    MFRatio FramesPerSecond;
    MFNominalRange NominalRange;
    MFVideoArea GeometricAperture;
    MFVideoArea MinimumDisplayAperture;
    MFVideoArea PanScanAperture;
    unsigned __int64 VideoFlags;
    } 	MFVideoInfo;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef struct __MFAYUVSample
    {
    BYTE bCrValue;
    BYTE bCbValue;
    BYTE bYValue;
    BYTE bSampleAlpha8;
    } 	MFAYUVSample;

typedef struct _MFARGB
    {
    BYTE rgbBlue;
    BYTE rgbGreen;
    BYTE rgbRed;
    BYTE rgbAlpha;
    } 	MFARGB;

typedef union _MFPaletteEntry
    {
    MFARGB ARGB;
    MFAYUVSample AYCbCr;
    } 	MFPaletteEntry;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
typedef struct _MFVideoSurfaceInfo
    {
    DWORD Format;
    DWORD PaletteEntries;
    /* [size_is] */ MFPaletteEntry Palette[ 1 ];
    } 	MFVideoSurfaceInfo;

typedef struct _MFVideoCompressedInfo
    {
    LONGLONG AvgBitrate;
    LONGLONG AvgBitErrorRate;
    DWORD MaxKeyFrameSpacing;
    } 	MFVideoCompressedInfo;

typedef struct _MFVIDEOFORMAT
    {
    DWORD dwSize;
    MFVideoInfo videoInfo;
    GUID guidFormat;
    MFVideoCompressedInfo compressedInfo;
    MFVideoSurfaceInfo surfaceInfo;
    } 	MFVIDEOFORMAT;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
typedef 
enum _MFStandardVideoFormat
    {
        MFStdVideoFormat_reserved	= 0,
        MFStdVideoFormat_NTSC	= ( MFStdVideoFormat_reserved + 1 ) ,
        MFStdVideoFormat_PAL	= ( MFStdVideoFormat_NTSC + 1 ) ,
        MFStdVideoFormat_DVD_NTSC	= ( MFStdVideoFormat_PAL + 1 ) ,
        MFStdVideoFormat_DVD_PAL	= ( MFStdVideoFormat_DVD_NTSC + 1 ) ,
        MFStdVideoFormat_DV_PAL	= ( MFStdVideoFormat_DVD_PAL + 1 ) ,
        MFStdVideoFormat_DV_NTSC	= ( MFStdVideoFormat_DV_PAL + 1 ) ,
        MFStdVideoFormat_ATSC_SD480i	= ( MFStdVideoFormat_DV_NTSC + 1 ) ,
        MFStdVideoFormat_ATSC_HD1080i	= ( MFStdVideoFormat_ATSC_SD480i + 1 ) ,
        MFStdVideoFormat_ATSC_HD720p	= ( MFStdVideoFormat_ATSC_HD1080i + 1 ) 
    } 	MFStandardVideoFormat;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0009_v0_0_s_ifspec;

#ifndef __IMFVideoMediaType_INTERFACE_DEFINED__
#define __IMFVideoMediaType_INTERFACE_DEFINED__

/* interface IMFVideoMediaType */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoMediaType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b99f381f-a8f9-47a2-a5af-ca3a225a3890")
    IMFVideoMediaType : public IMFMediaType
    {
    public:
        virtual const MFVIDEOFORMAT *STDMETHODCALLTYPE GetVideoFormat( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoRepresentation( 
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation,
            /* [in] */ LONG lStride) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoMediaTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoMediaType * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFVideoMediaType * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFVideoMediaType * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFVideoMediaType * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFVideoMediaType * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFVideoMediaType * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFVideoMediaType * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetMajorType)
        HRESULT ( STDMETHODCALLTYPE *GetMajorType )( 
            IMFVideoMediaType * This,
            /* [out] */ GUID *pguidMajorType);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsCompressedFormat)
        HRESULT ( STDMETHODCALLTYPE *IsCompressedFormat )( 
            IMFVideoMediaType * This,
            /* [out] */ BOOL *pfCompressed);
        
        DECLSPEC_XFGVIRT(IMFMediaType, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            IMFVideoMediaType * This,
            /* [in] */ IMFMediaType *pIMediaType,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFMediaType, GetRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetRepresentation )( 
            IMFVideoMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation);
        
        DECLSPEC_XFGVIRT(IMFMediaType, FreeRepresentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FreeRepresentation )( 
            IMFVideoMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [in] */ LPVOID pvRepresentation);
        
        DECLSPEC_XFGVIRT(IMFVideoMediaType, GetVideoFormat)
        const MFVIDEOFORMAT *( STDMETHODCALLTYPE *GetVideoFormat )( 
            IMFVideoMediaType * This);
        
        DECLSPEC_XFGVIRT(IMFVideoMediaType, GetVideoRepresentation)
        HRESULT ( STDMETHODCALLTYPE *GetVideoRepresentation )( 
            IMFVideoMediaType * This,
            /* [in] */ GUID guidRepresentation,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppvRepresentation,
            /* [in] */ LONG lStride);
        
        END_INTERFACE
    } IMFVideoMediaTypeVtbl;

    interface IMFVideoMediaType
    {
        CONST_VTBL struct IMFVideoMediaTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoMediaType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoMediaType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoMediaType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoMediaType_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFVideoMediaType_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFVideoMediaType_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFVideoMediaType_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFVideoMediaType_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFVideoMediaType_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFVideoMediaType_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFVideoMediaType_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFVideoMediaType_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFVideoMediaType_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFVideoMediaType_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFVideoMediaType_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFVideoMediaType_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFVideoMediaType_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFVideoMediaType_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFVideoMediaType_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFVideoMediaType_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFVideoMediaType_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFVideoMediaType_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFVideoMediaType_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFVideoMediaType_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFVideoMediaType_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFVideoMediaType_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFVideoMediaType_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFVideoMediaType_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFVideoMediaType_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFVideoMediaType_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFVideoMediaType_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFVideoMediaType_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFVideoMediaType_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFVideoMediaType_GetMajorType(This,pguidMajorType)	\
    ( (This)->lpVtbl -> GetMajorType(This,pguidMajorType) ) 

#define IMFVideoMediaType_IsCompressedFormat(This,pfCompressed)	\
    ( (This)->lpVtbl -> IsCompressedFormat(This,pfCompressed) ) 

#define IMFVideoMediaType_IsEqual(This,pIMediaType,pdwFlags)	\
    ( (This)->lpVtbl -> IsEqual(This,pIMediaType,pdwFlags) ) 

#define IMFVideoMediaType_GetRepresentation(This,guidRepresentation,ppvRepresentation)	\
    ( (This)->lpVtbl -> GetRepresentation(This,guidRepresentation,ppvRepresentation) ) 

#define IMFVideoMediaType_FreeRepresentation(This,guidRepresentation,pvRepresentation)	\
    ( (This)->lpVtbl -> FreeRepresentation(This,guidRepresentation,pvRepresentation) ) 


#define IMFVideoMediaType_GetVideoFormat(This)	\
    ( (This)->lpVtbl -> GetVideoFormat(This) ) 

#define IMFVideoMediaType_GetVideoRepresentation(This,guidRepresentation,ppvRepresentation,lStride)	\
    ( (This)->lpVtbl -> GetVideoRepresentation(This,guidRepresentation,ppvRepresentation,lStride) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoMediaType_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0010_v0_0_s_ifspec;

#ifndef __IMFAsyncResult_INTERFACE_DEFINED__
#define __IMFAsyncResult_INTERFACE_DEFINED__

/* interface IMFAsyncResult */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFAsyncResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ac6b7889-0740-4d51-8619-905994a55cc6")
    IMFAsyncResult : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ HRESULT hrStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObject) = 0;
        
        virtual /* [local] */ IUnknown *STDMETHODCALLTYPE GetStateNoAddRef( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAsyncResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFAsyncResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IMFAsyncResult, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IMFAsyncResult * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkState);
        
        DECLSPEC_XFGVIRT(IMFAsyncResult, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMFAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IMFAsyncResult, SetStatus)
        HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IMFAsyncResult * This,
            /* [in] */ HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(IMFAsyncResult, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IMFAsyncResult * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFAsyncResult, GetStateNoAddRef)
        /* [local] */ IUnknown *( STDMETHODCALLTYPE *GetStateNoAddRef )( 
            IMFAsyncResult * This);
        
        END_INTERFACE
    } IMFAsyncResultVtbl;

    interface IMFAsyncResult
    {
        CONST_VTBL struct IMFAsyncResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAsyncResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAsyncResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAsyncResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAsyncResult_GetState(This,ppunkState)	\
    ( (This)->lpVtbl -> GetState(This,ppunkState) ) 

#define IMFAsyncResult_GetStatus(This)	\
    ( (This)->lpVtbl -> GetStatus(This) ) 

#define IMFAsyncResult_SetStatus(This,hrStatus)	\
    ( (This)->lpVtbl -> SetStatus(This,hrStatus) ) 

#define IMFAsyncResult_GetObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetObject(This,ppObject) ) 

#define IMFAsyncResult_GetStateNoAddRef(This)	\
    ( (This)->lpVtbl -> GetStateNoAddRef(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAsyncResult_INTERFACE_DEFINED__ */


#ifndef __IMFAsyncCallback_INTERFACE_DEFINED__
#define __IMFAsyncCallback_INTERFACE_DEFINED__

/* interface IMFAsyncCallback */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFAsyncCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a27003cf-2354-4f2a-8d6a-ab7cff15437e")
    IMFAsyncCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetParameters( 
            /* [out] */ __RPC__out DWORD *pdwFlags,
            /* [out] */ __RPC__out DWORD *pdwQueue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ __RPC__in_opt IMFAsyncResult *pAsyncResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAsyncCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFAsyncCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallback, GetParameters)
        HRESULT ( STDMETHODCALLTYPE *GetParameters )( 
            __RPC__in IMFAsyncCallback * This,
            /* [out] */ __RPC__out DWORD *pdwFlags,
            /* [out] */ __RPC__out DWORD *pdwQueue);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallback, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in IMFAsyncCallback * This,
            /* [in] */ __RPC__in_opt IMFAsyncResult *pAsyncResult);
        
        END_INTERFACE
    } IMFAsyncCallbackVtbl;

    interface IMFAsyncCallback
    {
        CONST_VTBL struct IMFAsyncCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAsyncCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAsyncCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAsyncCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAsyncCallback_GetParameters(This,pdwFlags,pdwQueue)	\
    ( (This)->lpVtbl -> GetParameters(This,pdwFlags,pdwQueue) ) 

#define IMFAsyncCallback_Invoke(This,pAsyncResult)	\
    ( (This)->lpVtbl -> Invoke(This,pAsyncResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAsyncCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0012 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0012_v0_0_s_ifspec;

#ifndef __IMFAsyncCallbackLogging_INTERFACE_DEFINED__
#define __IMFAsyncCallbackLogging_INTERFACE_DEFINED__

/* interface IMFAsyncCallbackLogging */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFAsyncCallbackLogging;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7a4dca1-f5f0-47b6-b92b-bf0106d25791")
    IMFAsyncCallbackLogging : public IMFAsyncCallback
    {
    public:
        virtual void *STDMETHODCALLTYPE GetObjectPointer( void) = 0;
        
        virtual DWORD STDMETHODCALLTYPE GetObjectTag( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAsyncCallbackLoggingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFAsyncCallbackLogging * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFAsyncCallbackLogging * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFAsyncCallbackLogging * This);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallback, GetParameters)
        HRESULT ( STDMETHODCALLTYPE *GetParameters )( 
            IMFAsyncCallbackLogging * This,
            /* [out] */ DWORD *pdwFlags,
            /* [out] */ DWORD *pdwQueue);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallback, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMFAsyncCallbackLogging * This,
            /* [in] */ IMFAsyncResult *pAsyncResult);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallbackLogging, GetObjectPointer)
        void *( STDMETHODCALLTYPE *GetObjectPointer )( 
            IMFAsyncCallbackLogging * This);
        
        DECLSPEC_XFGVIRT(IMFAsyncCallbackLogging, GetObjectTag)
        DWORD ( STDMETHODCALLTYPE *GetObjectTag )( 
            IMFAsyncCallbackLogging * This);
        
        END_INTERFACE
    } IMFAsyncCallbackLoggingVtbl;

    interface IMFAsyncCallbackLogging
    {
        CONST_VTBL struct IMFAsyncCallbackLoggingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAsyncCallbackLogging_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAsyncCallbackLogging_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAsyncCallbackLogging_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAsyncCallbackLogging_GetParameters(This,pdwFlags,pdwQueue)	\
    ( (This)->lpVtbl -> GetParameters(This,pdwFlags,pdwQueue) ) 

#define IMFAsyncCallbackLogging_Invoke(This,pAsyncResult)	\
    ( (This)->lpVtbl -> Invoke(This,pAsyncResult) ) 


#define IMFAsyncCallbackLogging_GetObjectPointer(This)	\
    ( (This)->lpVtbl -> GetObjectPointer(This) ) 

#define IMFAsyncCallbackLogging_GetObjectTag(This)	\
    ( (This)->lpVtbl -> GetObjectTag(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAsyncCallbackLogging_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0013 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
#define MFASYNC_FAST_IO_PROCESSING_CALLBACK 0x00000001
#define MFASYNC_SIGNAL_CALLBACK 0x00000002
#define MFASYNC_BLOCKING_CALLBACK 0x00000004
#define MFASYNC_REPLY_CALLBACK 0x00000008
#define MFASYNC_LOCALIZE_REMOTE_CALLBACK 0x00000010
#define MFASYNC_CALLBACK_QUEUE_UNDEFINED              0x00000000
#define MFASYNC_CALLBACK_QUEUE_STANDARD               0x00000001
#define MFASYNC_CALLBACK_QUEUE_RT                     0x00000002
#define MFASYNC_CALLBACK_QUEUE_IO                     0x00000003
#define MFASYNC_CALLBACK_QUEUE_TIMER                  0x00000004
#define MFASYNC_CALLBACK_QUEUE_MULTITHREADED          0x00000005
#define MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION          0x00000007
#define MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK           0xFFFF0000
#define MFASYNC_CALLBACK_QUEUE_ALL                    0xFFFFFFFF

enum __MIDL___MIDL_itf_mfobjects_0000_0013_0001
    {
        MEUnknown	= 0,
        MEError	= 1,
        MEExtendedType	= 2,
        MENonFatalError	= 3,
        MEGenericV1Anchor	= MENonFatalError,
        MESessionUnknown	= 100,
        MESessionTopologySet	= 101,
        MESessionTopologiesCleared	= 102,
        MESessionStarted	= 103,
        MESessionPaused	= 104,
        MESessionStopped	= 105,
        MESessionClosed	= 106,
        MESessionEnded	= 107,
        MESessionRateChanged	= 108,
        MESessionScrubSampleComplete	= 109,
        MESessionCapabilitiesChanged	= 110,
        MESessionTopologyStatus	= 111,
        MESessionNotifyPresentationTime	= 112,
        MENewPresentation	= 113,
        MELicenseAcquisitionStart	= 114,
        MELicenseAcquisitionCompleted	= 115,
        MEIndividualizationStart	= 116,
        MEIndividualizationCompleted	= 117,
        MEEnablerProgress	= 118,
        MEEnablerCompleted	= 119,
        MEPolicyError	= 120,
        MEPolicyReport	= 121,
        MEBufferingStarted	= 122,
        MEBufferingStopped	= 123,
        MEConnectStart	= 124,
        MEConnectEnd	= 125,
        MEReconnectStart	= 126,
        MEReconnectEnd	= 127,
        MERendererEvent	= 128,
        MESessionStreamSinkFormatChanged	= 129,
        MESessionV1Anchor	= MESessionStreamSinkFormatChanged,
        MESourceUnknown	= 200,
        MESourceStarted	= 201,
        MEStreamStarted	= 202,
        MESourceSeeked	= 203,
        MEStreamSeeked	= 204,
        MENewStream	= 205,
        MEUpdatedStream	= 206,
        MESourceStopped	= 207,
        MEStreamStopped	= 208,
        MESourcePaused	= 209,
        MEStreamPaused	= 210,
        MEEndOfPresentation	= 211,
        MEEndOfStream	= 212,
        MEMediaSample	= 213,
        MEStreamTick	= 214,
        MEStreamThinMode	= 215,
        MEStreamFormatChanged	= 216,
        MESourceRateChanged	= 217,
        MEEndOfPresentationSegment	= 218,
        MESourceCharacteristicsChanged	= 219,
        MESourceRateChangeRequested	= 220,
        MESourceMetadataChanged	= 221,
        MESequencerSourceTopologyUpdated	= 222,
        MESourceV1Anchor	= MESequencerSourceTopologyUpdated,
        MESinkUnknown	= 300,
        MEStreamSinkStarted	= 301,
        MEStreamSinkStopped	= 302,
        MEStreamSinkPaused	= 303,
        MEStreamSinkRateChanged	= 304,
        MEStreamSinkRequestSample	= 305,
        MEStreamSinkMarker	= 306,
        MEStreamSinkPrerolled	= 307,
        MEStreamSinkScrubSampleComplete	= 308,
        MEStreamSinkFormatChanged	= 309,
        MEStreamSinkDeviceChanged	= 310,
        MEQualityNotify	= 311,
        MESinkInvalidated	= 312,
        MEAudioSessionNameChanged	= 313,
        MEAudioSessionVolumeChanged	= 314,
        MEAudioSessionDeviceRemoved	= 315,
        MEAudioSessionServerShutdown	= 316,
        MEAudioSessionGroupingParamChanged	= 317,
        MEAudioSessionIconChanged	= 318,
        MEAudioSessionFormatChanged	= 319,
        MEAudioSessionDisconnected	= 320,
        MEAudioSessionExclusiveModeOverride	= 321,
        MESinkV1Anchor	= MEAudioSessionExclusiveModeOverride,
        MECaptureAudioSessionVolumeChanged	= 322,
        MECaptureAudioSessionDeviceRemoved	= 323,
        MECaptureAudioSessionFormatChanged	= 324,
        MECaptureAudioSessionDisconnected	= 325,
        MECaptureAudioSessionExclusiveModeOverride	= 326,
        MECaptureAudioSessionServerShutdown	= 327,
        MESinkV2Anchor	= MECaptureAudioSessionServerShutdown,
        METrustUnknown	= 400,
        MEPolicyChanged	= 401,
        MEContentProtectionMessage	= 402,
        MEPolicySet	= 403,
        METrustV1Anchor	= MEPolicySet,
        MEWMDRMLicenseBackupCompleted	= 500,
        MEWMDRMLicenseBackupProgress	= 501,
        MEWMDRMLicenseRestoreCompleted	= 502,
        MEWMDRMLicenseRestoreProgress	= 503,
        MEWMDRMLicenseAcquisitionCompleted	= 506,
        MEWMDRMIndividualizationCompleted	= 508,
        MEWMDRMIndividualizationProgress	= 513,
        MEWMDRMProximityCompleted	= 514,
        MEWMDRMLicenseStoreCleaned	= 515,
        MEWMDRMRevocationDownloadCompleted	= 516,
        MEWMDRMV1Anchor	= MEWMDRMRevocationDownloadCompleted,
        METransformUnknown	= 600,
        METransformNeedInput	= ( METransformUnknown + 1 ) ,
        METransformHaveOutput	= ( METransformNeedInput + 1 ) ,
        METransformDrainComplete	= ( METransformHaveOutput + 1 ) ,
        METransformMarker	= ( METransformDrainComplete + 1 ) ,
        METransformInputStreamStateChanged	= ( METransformMarker + 1 ) ,
        MEByteStreamCharacteristicsChanged	= 700,
        MEVideoCaptureDeviceRemoved	= 800,
        MEVideoCaptureDevicePreempted	= 801,
        MEStreamSinkFormatInvalidated	= 802,
        MEEncodingParameters	= 803,
        MEContentProtectionMetadata	= 900,
        MEDeviceThermalStateChanged	= 950,
        MEReservedMax	= 10000
    } ;
typedef DWORD MediaEventType;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0013_v0_0_s_ifspec;

#ifndef __IMFMediaEvent_INTERFACE_DEFINED__
#define __IMFMediaEvent_INTERFACE_DEFINED__

/* interface IMFMediaEvent */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF598932-F10C-4E39-BBA2-C308F101DAA3")
    IMFMediaEvent : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out MediaEventType *pmet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtendedType( 
            /* [out] */ __RPC__out GUID *pguidExtendedType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out HRESULT *phrStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [out] */ __RPC__out PROPVARIANT *pvValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFMediaEvent * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFMediaEvent * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFMediaEvent * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFMediaEvent * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFMediaEvent, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IMFMediaEvent * This,
            /* [out] */ __RPC__out MediaEventType *pmet);
        
        DECLSPEC_XFGVIRT(IMFMediaEvent, GetExtendedType)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedType )( 
            __RPC__in IMFMediaEvent * This,
            /* [out] */ __RPC__out GUID *pguidExtendedType);
        
        DECLSPEC_XFGVIRT(IMFMediaEvent, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMFMediaEvent * This,
            /* [out] */ __RPC__out HRESULT *phrStatus);
        
        DECLSPEC_XFGVIRT(IMFMediaEvent, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IMFMediaEvent * This,
            /* [out] */ __RPC__out PROPVARIANT *pvValue);
        
        END_INTERFACE
    } IMFMediaEventVtbl;

    interface IMFMediaEvent
    {
        CONST_VTBL struct IMFMediaEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEvent_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFMediaEvent_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFMediaEvent_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFMediaEvent_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFMediaEvent_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFMediaEvent_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFMediaEvent_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFMediaEvent_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFMediaEvent_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFMediaEvent_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFMediaEvent_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFMediaEvent_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFMediaEvent_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFMediaEvent_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFMediaEvent_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFMediaEvent_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFMediaEvent_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFMediaEvent_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFMediaEvent_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFMediaEvent_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFMediaEvent_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFMediaEvent_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFMediaEvent_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFMediaEvent_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFMediaEvent_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFMediaEvent_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFMediaEvent_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFMediaEvent_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFMediaEvent_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFMediaEvent_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFMediaEvent_GetType(This,pmet)	\
    ( (This)->lpVtbl -> GetType(This,pmet) ) 

#define IMFMediaEvent_GetExtendedType(This,pguidExtendedType)	\
    ( (This)->lpVtbl -> GetExtendedType(This,pguidExtendedType) ) 

#define IMFMediaEvent_GetStatus(This,phrStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,phrStatus) ) 

#define IMFMediaEvent_GetValue(This,pvValue)	\
    ( (This)->lpVtbl -> GetValue(This,pvValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEvent_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0014 */
/* [local] */ 

#define MF_EVENT_FLAG_NO_WAIT 0x00000001



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0014_v0_0_s_ifspec;

#ifndef __IMFMediaEventGenerator_INTERFACE_DEFINED__
#define __IMFMediaEventGenerator_INTERFACE_DEFINED__

/* interface IMFMediaEventGenerator */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEventGenerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2CD0BD52-BCD5-4B89-B62C-EADC0C031E7D")
    IMFMediaEventGenerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginGetEvent( 
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndGetEvent( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueEvent( 
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEventGeneratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaEventGenerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaEventGenerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaEventGenerator * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFMediaEventGenerator * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaEventGenerator * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaEventGenerator * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFMediaEventGenerator * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        END_INTERFACE
    } IMFMediaEventGeneratorVtbl;

    interface IMFMediaEventGenerator
    {
        CONST_VTBL struct IMFMediaEventGeneratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEventGenerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEventGenerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEventGenerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEventGenerator_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaEventGenerator_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaEventGenerator_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaEventGenerator_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_RemoteBeginGetEvent_Proxy( 
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFMediaEventGenerator_RemoteBeginGetEvent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_RemoteEndGetEvent_Proxy( 
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out DWORD *pcbEvent,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbEvent) BYTE **ppbEvent);


void __RPC_STUB IMFMediaEventGenerator_RemoteEndGetEvent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFMediaEventGenerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0015 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0015_v0_0_s_ifspec;

#ifndef __IMFRemoteAsyncCallback_INTERFACE_DEFINED__
#define __IMFRemoteAsyncCallback_INTERFACE_DEFINED__

/* interface IMFRemoteAsyncCallback */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFRemoteAsyncCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a27003d0-2354-4f2a-8d6a-ab7cff15437e")
    IMFRemoteAsyncCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in_opt IUnknown *pRemoteResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRemoteAsyncCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFRemoteAsyncCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFRemoteAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFRemoteAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IMFRemoteAsyncCallback, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in IMFRemoteAsyncCallback * This,
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in_opt IUnknown *pRemoteResult);
        
        END_INTERFACE
    } IMFRemoteAsyncCallbackVtbl;

    interface IMFRemoteAsyncCallback
    {
        CONST_VTBL struct IMFRemoteAsyncCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRemoteAsyncCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRemoteAsyncCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRemoteAsyncCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRemoteAsyncCallback_Invoke(This,hr,pRemoteResult)	\
    ( (This)->lpVtbl -> Invoke(This,hr,pRemoteResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRemoteAsyncCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum _MFBYTESTREAM_SEEK_ORIGIN
    {
        msoBegin	= 0,
        msoCurrent	= ( msoBegin + 1 ) 
    } 	MFBYTESTREAM_SEEK_ORIGIN;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0016_v0_0_s_ifspec;

#ifndef __IMFByteStream_INTERFACE_DEFINED__
#define __IMFByteStream_INTERFACE_DEFINED__

/* interface IMFByteStream */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ad4c1b00-4bf7-422f-9175-756693d9130d")
    IMFByteStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ __RPC__out QWORD *pqwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLength( 
            /* [in] */ QWORD qwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentPosition( 
            /* [out] */ __RPC__out QWORD *pqwPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentPosition( 
            /* [in] */ QWORD qwPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEndOfStream( 
            /* [out] */ __RPC__out BOOL *pfEndOfStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [size_is][out] */ __RPC__out_ecount_full(cb) BYTE *pb,
            /* [in] */ ULONG cb,
            /* [out] */ __RPC__out ULONG *pcbRead) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginRead( 
            /* [annotation][out] */ 
            _Out_writes_bytes_(cb)  BYTE *pb,
            /* [in] */ ULONG cb,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndRead( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write( 
            /* [size_is][in] */ __RPC__in_ecount_full(cb) const BYTE *pb,
            /* [in] */ ULONG cb,
            /* [out] */ __RPC__out ULONG *pcbWritten) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginWrite( 
            /* [annotation][in] */ 
            _In_reads_bytes_(cb)  const BYTE *pb,
            /* [in] */ ULONG cb,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndWrite( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ MFBYTESTREAM_SEEK_ORIGIN SeekOrigin,
            /* [in] */ LONGLONG llSeekOffset,
            /* [in] */ DWORD dwSeekFlags,
            /* [out] */ __RPC__out QWORD *pqwCurrentPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStream * This);
        
        DECLSPEC_XFGVIRT(IMFByteStream, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IMFByteStream * This,
            /* [out] */ __RPC__out DWORD *pdwCapabilities);
        
        DECLSPEC_XFGVIRT(IMFByteStream, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            __RPC__in IMFByteStream * This,
            /* [out] */ __RPC__out QWORD *pqwLength);
        
        DECLSPEC_XFGVIRT(IMFByteStream, SetLength)
        HRESULT ( STDMETHODCALLTYPE *SetLength )( 
            __RPC__in IMFByteStream * This,
            /* [in] */ QWORD qwLength);
        
        DECLSPEC_XFGVIRT(IMFByteStream, GetCurrentPosition)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentPosition )( 
            __RPC__in IMFByteStream * This,
            /* [out] */ __RPC__out QWORD *pqwPosition);
        
        DECLSPEC_XFGVIRT(IMFByteStream, SetCurrentPosition)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentPosition )( 
            __RPC__in IMFByteStream * This,
            /* [in] */ QWORD qwPosition);
        
        DECLSPEC_XFGVIRT(IMFByteStream, IsEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *IsEndOfStream )( 
            __RPC__in IMFByteStream * This,
            /* [out] */ __RPC__out BOOL *pfEndOfStream);
        
        DECLSPEC_XFGVIRT(IMFByteStream, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IMFByteStream * This,
            /* [size_is][out] */ __RPC__out_ecount_full(cb) BYTE *pb,
            /* [in] */ ULONG cb,
            /* [out] */ __RPC__out ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(IMFByteStream, BeginRead)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRead )( 
            IMFByteStream * This,
            /* [annotation][out] */ 
            _Out_writes_bytes_(cb)  BYTE *pb,
            /* [in] */ ULONG cb,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFByteStream, EndRead)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndRead )( 
            IMFByteStream * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(IMFByteStream, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IMFByteStream * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cb) const BYTE *pb,
            /* [in] */ ULONG cb,
            /* [out] */ __RPC__out ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IMFByteStream, BeginWrite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginWrite )( 
            IMFByteStream * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(cb)  const BYTE *pb,
            /* [in] */ ULONG cb,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFByteStream, EndWrite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndWrite )( 
            IMFByteStream * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IMFByteStream, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IMFByteStream * This,
            /* [in] */ MFBYTESTREAM_SEEK_ORIGIN SeekOrigin,
            /* [in] */ LONGLONG llSeekOffset,
            /* [in] */ DWORD dwSeekFlags,
            /* [out] */ __RPC__out QWORD *pqwCurrentPosition);
        
        DECLSPEC_XFGVIRT(IMFByteStream, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in IMFByteStream * This);
        
        DECLSPEC_XFGVIRT(IMFByteStream, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMFByteStream * This);
        
        END_INTERFACE
    } IMFByteStreamVtbl;

    interface IMFByteStream
    {
        CONST_VTBL struct IMFByteStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStream_GetCapabilities(This,pdwCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilities) ) 

#define IMFByteStream_GetLength(This,pqwLength)	\
    ( (This)->lpVtbl -> GetLength(This,pqwLength) ) 

#define IMFByteStream_SetLength(This,qwLength)	\
    ( (This)->lpVtbl -> SetLength(This,qwLength) ) 

#define IMFByteStream_GetCurrentPosition(This,pqwPosition)	\
    ( (This)->lpVtbl -> GetCurrentPosition(This,pqwPosition) ) 

#define IMFByteStream_SetCurrentPosition(This,qwPosition)	\
    ( (This)->lpVtbl -> SetCurrentPosition(This,qwPosition) ) 

#define IMFByteStream_IsEndOfStream(This,pfEndOfStream)	\
    ( (This)->lpVtbl -> IsEndOfStream(This,pfEndOfStream) ) 

#define IMFByteStream_Read(This,pb,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pb,cb,pcbRead) ) 

#define IMFByteStream_BeginRead(This,pb,cb,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginRead(This,pb,cb,pCallback,punkState) ) 

#define IMFByteStream_EndRead(This,pResult,pcbRead)	\
    ( (This)->lpVtbl -> EndRead(This,pResult,pcbRead) ) 

#define IMFByteStream_Write(This,pb,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pb,cb,pcbWritten) ) 

#define IMFByteStream_BeginWrite(This,pb,cb,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginWrite(This,pb,cb,pCallback,punkState) ) 

#define IMFByteStream_EndWrite(This,pResult,pcbWritten)	\
    ( (This)->lpVtbl -> EndWrite(This,pResult,pcbWritten) ) 

#define IMFByteStream_Seek(This,SeekOrigin,llSeekOffset,dwSeekFlags,pqwCurrentPosition)	\
    ( (This)->lpVtbl -> Seek(This,SeekOrigin,llSeekOffset,dwSeekFlags,pqwCurrentPosition) ) 

#define IMFByteStream_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IMFByteStream_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_RemoteBeginRead_Proxy( 
    __RPC__in IMFByteStream * This,
    /* [in] */ ULONG cb,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFByteStream_RemoteBeginRead_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_RemoteEndRead_Proxy( 
    __RPC__in IMFByteStream * This,
    /* [in] */ __RPC__in_opt IUnknown *punkResult,
    /* [length_is][size_is][out][in] */ __RPC__inout_ecount_part(cb, *pcbRead) BYTE *pb,
    /* [in] */ ULONG cb,
    /* [out][in] */ __RPC__inout ULONG *pcbRead);


void __RPC_STUB IMFByteStream_RemoteEndRead_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_RemoteBeginWrite_Proxy( 
    __RPC__in IMFByteStream * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const BYTE *pb,
    /* [in] */ ULONG cb,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFByteStream_RemoteBeginWrite_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_RemoteEndWrite_Proxy( 
    __RPC__in IMFByteStream * This,
    /* [in] */ __RPC__in_opt IUnknown *punkResult,
    /* [out] */ __RPC__out ULONG *pcbWritten);


void __RPC_STUB IMFByteStream_RemoteEndWrite_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFByteStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0017 */
/* [local] */ 

#define MFBYTESTREAM_IS_READABLE                0x00000001
#define MFBYTESTREAM_IS_WRITABLE                0x00000002
#define MFBYTESTREAM_IS_SEEKABLE                0x00000004
#define MFBYTESTREAM_IS_REMOTE                  0x00000008
#define MFBYTESTREAM_IS_DIRECTORY               0x00000080
#define MFBYTESTREAM_HAS_SLOW_SEEK              0x00000100
#define MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED    0x00000200
#if (WINVER >= _WIN32_WINNT_WIN7) 
#define MFBYTESTREAM_SHARE_WRITE                0x00000400
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
#define MFBYTESTREAM_DOES_NOT_USE_NETWORK        0x00000800
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#define MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO 0x00000001
EXTERN_GUID( MF_BYTESTREAM_ORIGIN_NAME, 0xfc358288, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
EXTERN_GUID( MF_BYTESTREAM_CONTENT_TYPE, 0xfc358289, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
EXTERN_GUID( MF_BYTESTREAM_DURATION, 0xfc35828a, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
EXTERN_GUID( MF_BYTESTREAM_LAST_MODIFIED_TIME, 0xfc35828b, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_BYTESTREAM_IFO_FILE_URI, 0xfc35828c, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
EXTERN_GUID( MF_BYTESTREAM_DLNA_PROFILE_ID, 0xfc35828d, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a);
EXTERN_GUID( MF_BYTESTREAM_EFFECTIVE_URL, 0x9afa0209, 0x89d1, 0x42af, 0x84, 0x56, 0x1d, 0xe6, 0xb5, 0x62, 0xd6, 0x91);
EXTERN_GUID( MF_BYTESTREAM_TRANSCODED, 0xb6c5c282, 0x4dc9, 0x4db9, 0xab, 0x48, 0xcf, 0x3b, 0x6d, 0x8b, 0xc5, 0xe0 );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
EXTERN_GUID(CLSID_MFByteStreamProxyClassFactory, 0x770e8e77, 0x4916, 0x441c, 0xa9, 0xa7, 0xb3, 0x42, 0xd0, 0xee, 0xbc, 0x71 );


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0017_v0_0_s_ifspec;

#ifndef __IMFByteStreamProxyClassFactory_INTERFACE_DEFINED__
#define __IMFByteStreamProxyClassFactory_INTERFACE_DEFINED__

/* interface IMFByteStreamProxyClassFactory */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamProxyClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a6b43f84-5c0a-42e8-a44d-b1857a76992f")
    IMFByteStreamProxyClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateByteStreamProxy( 
            /* [in] */ __RPC__in_opt IMFByteStream *pByteStream,
            /* [unique][in] */ __RPC__in_opt IMFAttributes *pAttributes,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamProxyClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStreamProxyClassFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStreamProxyClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStreamProxyClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamProxyClassFactory, CreateByteStreamProxy)
        HRESULT ( STDMETHODCALLTYPE *CreateByteStreamProxy )( 
            __RPC__in IMFByteStreamProxyClassFactory * This,
            /* [in] */ __RPC__in_opt IMFByteStream *pByteStream,
            /* [unique][in] */ __RPC__in_opt IMFAttributes *pAttributes,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppvObject);
        
        END_INTERFACE
    } IMFByteStreamProxyClassFactoryVtbl;

    interface IMFByteStreamProxyClassFactory
    {
        CONST_VTBL struct IMFByteStreamProxyClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamProxyClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamProxyClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamProxyClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamProxyClassFactory_CreateByteStreamProxy(This,pByteStream,pAttributes,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateByteStreamProxy(This,pByteStream,pAttributes,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamProxyClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0018 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfobjects_0000_0018_0001
    {
        MF_ACCESSMODE_READ	= 1,
        MF_ACCESSMODE_WRITE	= 2,
        MF_ACCESSMODE_READWRITE	= 3
    } 	MF_FILE_ACCESSMODE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfobjects_0000_0018_0002
    {
        MF_OPENMODE_FAIL_IF_NOT_EXIST	= 0,
        MF_OPENMODE_FAIL_IF_EXIST	= 1,
        MF_OPENMODE_RESET_IF_EXIST	= 2,
        MF_OPENMODE_APPEND_IF_EXIST	= 3,
        MF_OPENMODE_DELETE_IF_EXIST	= 4
    } 	MF_FILE_OPENMODE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfobjects_0000_0018_0003
    {
        MF_FILEFLAGS_NONE	= 0,
        MF_FILEFLAGS_NOBUFFERING	= 0x1,
        MF_FILEFLAGS_ALLOW_WRITE_SHARING	= 0x2
    } 	MF_FILE_FLAGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0018_v0_0_s_ifspec;

#ifndef __IMFSampleOutputStream_INTERFACE_DEFINED__
#define __IMFSampleOutputStream_INTERFACE_DEFINED__

/* interface IMFSampleOutputStream */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFSampleOutputStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8feed468-6f7e-440d-869a-49bdd283ad0d")
    IMFSampleOutputStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginWriteSample( 
            /* [in] */ __RPC__in_opt IMFSample *pSample,
            /* [in] */ __RPC__in_opt IMFAsyncCallback *pCallback,
            /* [in] */ __RPC__in_opt IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndWriteSample( 
            /* [in] */ __RPC__in_opt IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleOutputStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFSampleOutputStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFSampleOutputStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFSampleOutputStream * This);
        
        DECLSPEC_XFGVIRT(IMFSampleOutputStream, BeginWriteSample)
        HRESULT ( STDMETHODCALLTYPE *BeginWriteSample )( 
            __RPC__in IMFSampleOutputStream * This,
            /* [in] */ __RPC__in_opt IMFSample *pSample,
            /* [in] */ __RPC__in_opt IMFAsyncCallback *pCallback,
            /* [in] */ __RPC__in_opt IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFSampleOutputStream, EndWriteSample)
        HRESULT ( STDMETHODCALLTYPE *EndWriteSample )( 
            __RPC__in IMFSampleOutputStream * This,
            /* [in] */ __RPC__in_opt IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFSampleOutputStream, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMFSampleOutputStream * This);
        
        END_INTERFACE
    } IMFSampleOutputStreamVtbl;

    interface IMFSampleOutputStream
    {
        CONST_VTBL struct IMFSampleOutputStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSampleOutputStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSampleOutputStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSampleOutputStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSampleOutputStream_BeginWriteSample(This,pSample,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginWriteSample(This,pSample,pCallback,punkState) ) 

#define IMFSampleOutputStream_EndWriteSample(This,pResult)	\
    ( (This)->lpVtbl -> EndWriteSample(This,pResult) ) 

#define IMFSampleOutputStream_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSampleOutputStream_INTERFACE_DEFINED__ */


#ifndef __IMFCollection_INTERFACE_DEFINED__
#define __IMFCollection_INTERFACE_DEFINED__

/* interface IMFCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5BC8A76B-869A-46a3-9B03-FA218A66AEBE")
    IMFCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetElementCount( 
            /* [out] */ __RPC__out DWORD *pcElements) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetElement( 
            /* [in] */ DWORD dwElementIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddElement( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveElement( 
            /* [in] */ DWORD dwElementIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertElementAt( 
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllElements( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFCollection * This);
        
        DECLSPEC_XFGVIRT(IMFCollection, GetElementCount)
        HRESULT ( STDMETHODCALLTYPE *GetElementCount )( 
            __RPC__in IMFCollection * This,
            /* [out] */ __RPC__out DWORD *pcElements);
        
        DECLSPEC_XFGVIRT(IMFCollection, GetElement)
        HRESULT ( STDMETHODCALLTYPE *GetElement )( 
            __RPC__in IMFCollection * This,
            /* [in] */ DWORD dwElementIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkElement);
        
        DECLSPEC_XFGVIRT(IMFCollection, AddElement)
        HRESULT ( STDMETHODCALLTYPE *AddElement )( 
            __RPC__in IMFCollection * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkElement);
        
        DECLSPEC_XFGVIRT(IMFCollection, RemoveElement)
        HRESULT ( STDMETHODCALLTYPE *RemoveElement )( 
            __RPC__in IMFCollection * This,
            /* [in] */ DWORD dwElementIndex,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkElement);
        
        DECLSPEC_XFGVIRT(IMFCollection, InsertElementAt)
        HRESULT ( STDMETHODCALLTYPE *InsertElementAt )( 
            __RPC__in IMFCollection * This,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFCollection, RemoveAllElements)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllElements )( 
            __RPC__in IMFCollection * This);
        
        END_INTERFACE
    } IMFCollectionVtbl;

    interface IMFCollection
    {
        CONST_VTBL struct IMFCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCollection_GetElementCount(This,pcElements)	\
    ( (This)->lpVtbl -> GetElementCount(This,pcElements) ) 

#define IMFCollection_GetElement(This,dwElementIndex,ppUnkElement)	\
    ( (This)->lpVtbl -> GetElement(This,dwElementIndex,ppUnkElement) ) 

#define IMFCollection_AddElement(This,pUnkElement)	\
    ( (This)->lpVtbl -> AddElement(This,pUnkElement) ) 

#define IMFCollection_RemoveElement(This,dwElementIndex,ppUnkElement)	\
    ( (This)->lpVtbl -> RemoveElement(This,dwElementIndex,ppUnkElement) ) 

#define IMFCollection_InsertElementAt(This,dwIndex,pUnknown)	\
    ( (This)->lpVtbl -> InsertElementAt(This,dwIndex,pUnknown) ) 

#define IMFCollection_RemoveAllElements(This)	\
    ( (This)->lpVtbl -> RemoveAllElements(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCollection_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEventQueue_INTERFACE_DEFINED__
#define __IMFMediaEventQueue_INTERFACE_DEFINED__

/* interface IMFMediaEventQueue */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEventQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36f846fc-2256-48b6-b58e-e2b638316581")
    IMFMediaEventQueue : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginGetEvent( 
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetEvent( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueEvent( 
            /* [in] */ IMFMediaEvent *pEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueEventParamVar( 
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ const PROPVARIANT *pvValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueEventParamUnk( 
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ IUnknown *pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEventQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEventQueue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEventQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEventQueue * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            IMFMediaEventQueue * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, BeginGetEvent)
        HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaEventQueue * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, EndGetEvent)
        HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaEventQueue * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            IMFMediaEventQueue * This,
            /* [in] */ IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, QueueEventParamVar)
        HRESULT ( STDMETHODCALLTYPE *QueueEventParamVar )( 
            IMFMediaEventQueue * This,
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, QueueEventParamUnk)
        HRESULT ( STDMETHODCALLTYPE *QueueEventParamUnk )( 
            IMFMediaEventQueue * This,
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(IMFMediaEventQueue, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaEventQueue * This);
        
        END_INTERFACE
    } IMFMediaEventQueueVtbl;

    interface IMFMediaEventQueue
    {
        CONST_VTBL struct IMFMediaEventQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEventQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEventQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEventQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEventQueue_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaEventQueue_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaEventQueue_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaEventQueue_QueueEvent(This,pEvent)	\
    ( (This)->lpVtbl -> QueueEvent(This,pEvent) ) 

#define IMFMediaEventQueue_QueueEventParamVar(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEventParamVar(This,met,guidExtendedType,hrStatus,pvValue) ) 

#define IMFMediaEventQueue_QueueEventParamUnk(This,met,guidExtendedType,hrStatus,pUnk)	\
    ( (This)->lpVtbl -> QueueEventParamUnk(This,met,guidExtendedType,hrStatus,pUnk) ) 

#define IMFMediaEventQueue_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEventQueue_INTERFACE_DEFINED__ */


#ifndef __IMFActivate_INTERFACE_DEFINED__
#define __IMFActivate_INTERFACE_DEFINED__

/* interface IMFActivate */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFActivate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FEE9E9A-4A89-47a6-899C-B6A53A70FB67")
    IMFActivate : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateObject( 
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShutdownObject( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetachObject( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFActivateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFActivate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFActivate * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFActivate * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFActivate * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFActivate * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFActivate * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFActivate, ActivateObject)
        HRESULT ( STDMETHODCALLTYPE *ActivateObject )( 
            __RPC__in IMFActivate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IMFActivate, ShutdownObject)
        HRESULT ( STDMETHODCALLTYPE *ShutdownObject )( 
            __RPC__in IMFActivate * This);
        
        DECLSPEC_XFGVIRT(IMFActivate, DetachObject)
        HRESULT ( STDMETHODCALLTYPE *DetachObject )( 
            __RPC__in IMFActivate * This);
        
        END_INTERFACE
    } IMFActivateVtbl;

    interface IMFActivate
    {
        CONST_VTBL struct IMFActivateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFActivate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFActivate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFActivate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFActivate_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFActivate_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFActivate_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFActivate_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFActivate_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFActivate_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFActivate_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFActivate_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFActivate_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFActivate_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFActivate_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFActivate_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFActivate_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFActivate_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFActivate_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFActivate_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFActivate_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFActivate_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFActivate_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFActivate_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFActivate_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFActivate_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFActivate_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFActivate_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFActivate_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFActivate_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFActivate_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFActivate_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFActivate_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFActivate_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFActivate_ActivateObject(This,riid,ppv)	\
    ( (This)->lpVtbl -> ActivateObject(This,riid,ppv) ) 

#define IMFActivate_ShutdownObject(This)	\
    ( (This)->lpVtbl -> ShutdownObject(This) ) 

#define IMFActivate_DetachObject(This)	\
    ( (This)->lpVtbl -> DetachObject(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFActivate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0022 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
typedef 
enum _MF_Plugin_Type
    {
        MF_Plugin_Type_MFT	= 0,
        MF_Plugin_Type_MediaSource	= 1,
        MF_Plugin_Type_MFT_MatchOutputType	= 2,
        MF_Plugin_Type_Other	= ( DWORD  )-1
    } 	MF_Plugin_Type;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0022_v0_0_s_ifspec;

#ifndef __IMFPluginControl_INTERFACE_DEFINED__
#define __IMFPluginControl_INTERFACE_DEFINED__

/* interface IMFPluginControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFPluginControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5c6c44bf-1db6-435b-9249-e8cd10fdec96")
    IMFPluginControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreferredClsid( 
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreferredClsidByIndex( 
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  LPWSTR *selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreferredClsid( 
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _In_opt_  const CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDisabled( 
            DWORD pluginType,
            REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisabledByIndex( 
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisabled( 
            DWORD pluginType,
            REFCLSID clsid,
            BOOL disabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPluginControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFPluginControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFPluginControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFPluginControl * This);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsid )( 
            IMFPluginControl * This,
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetPreferredClsidByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsidByIndex )( 
            IMFPluginControl * This,
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  LPWSTR *selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, SetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *SetPreferredClsid )( 
            IMFPluginControl * This,
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _In_opt_  const CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, IsDisabled)
        HRESULT ( STDMETHODCALLTYPE *IsDisabled )( 
            IMFPluginControl * This,
            DWORD pluginType,
            REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetDisabledByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetDisabledByIndex )( 
            IMFPluginControl * This,
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, SetDisabled)
        HRESULT ( STDMETHODCALLTYPE *SetDisabled )( 
            IMFPluginControl * This,
            DWORD pluginType,
            REFCLSID clsid,
            BOOL disabled);
        
        END_INTERFACE
    } IMFPluginControlVtbl;

    interface IMFPluginControl
    {
        CONST_VTBL struct IMFPluginControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPluginControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPluginControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPluginControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPluginControl_GetPreferredClsid(This,pluginType,selector,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsid(This,pluginType,selector,clsid) ) 

#define IMFPluginControl_GetPreferredClsidByIndex(This,pluginType,index,selector,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsidByIndex(This,pluginType,index,selector,clsid) ) 

#define IMFPluginControl_SetPreferredClsid(This,pluginType,selector,clsid)	\
    ( (This)->lpVtbl -> SetPreferredClsid(This,pluginType,selector,clsid) ) 

#define IMFPluginControl_IsDisabled(This,pluginType,clsid)	\
    ( (This)->lpVtbl -> IsDisabled(This,pluginType,clsid) ) 

#define IMFPluginControl_GetDisabledByIndex(This,pluginType,index,clsid)	\
    ( (This)->lpVtbl -> GetDisabledByIndex(This,pluginType,index,clsid) ) 

#define IMFPluginControl_SetDisabled(This,pluginType,clsid,disabled)	\
    ( (This)->lpVtbl -> SetDisabled(This,pluginType,clsid,disabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPluginControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0023 */
/* [local] */ 

typedef 
enum MF_PLUGIN_CONTROL_POLICY
    {
        MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS	= 0,
        MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS	= 1,
        MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS	= 2,
        MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE	= 3
    } 	MF_PLUGIN_CONTROL_POLICY;



extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0023_v0_0_s_ifspec;

#ifndef __IMFPluginControl2_INTERFACE_DEFINED__
#define __IMFPluginControl2_INTERFACE_DEFINED__

/* interface IMFPluginControl2 */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFPluginControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6982083-3DDC-45CB-AF5E-0F7A8CE4DE77")
    IMFPluginControl2 : public IMFPluginControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPolicy( 
            /* [in] */ MF_PLUGIN_CONTROL_POLICY policy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPluginControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFPluginControl2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFPluginControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFPluginControl2 * This);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsid )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetPreferredClsidByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsidByIndex )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  LPWSTR *selector,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, SetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *SetPreferredClsid )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            /* [annotation] */ 
            _In_  LPCWSTR selector,
            /* [annotation] */ 
            _In_opt_  const CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, IsDisabled)
        HRESULT ( STDMETHODCALLTYPE *IsDisabled )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, GetDisabledByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetDisabledByIndex )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            DWORD index,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IMFPluginControl, SetDisabled)
        HRESULT ( STDMETHODCALLTYPE *SetDisabled )( 
            IMFPluginControl2 * This,
            DWORD pluginType,
            REFCLSID clsid,
            BOOL disabled);
        
        DECLSPEC_XFGVIRT(IMFPluginControl2, SetPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetPolicy )( 
            IMFPluginControl2 * This,
            /* [in] */ MF_PLUGIN_CONTROL_POLICY policy);
        
        END_INTERFACE
    } IMFPluginControl2Vtbl;

    interface IMFPluginControl2
    {
        CONST_VTBL struct IMFPluginControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPluginControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPluginControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPluginControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPluginControl2_GetPreferredClsid(This,pluginType,selector,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsid(This,pluginType,selector,clsid) ) 

#define IMFPluginControl2_GetPreferredClsidByIndex(This,pluginType,index,selector,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsidByIndex(This,pluginType,index,selector,clsid) ) 

#define IMFPluginControl2_SetPreferredClsid(This,pluginType,selector,clsid)	\
    ( (This)->lpVtbl -> SetPreferredClsid(This,pluginType,selector,clsid) ) 

#define IMFPluginControl2_IsDisabled(This,pluginType,clsid)	\
    ( (This)->lpVtbl -> IsDisabled(This,pluginType,clsid) ) 

#define IMFPluginControl2_GetDisabledByIndex(This,pluginType,index,clsid)	\
    ( (This)->lpVtbl -> GetDisabledByIndex(This,pluginType,index,clsid) ) 

#define IMFPluginControl2_SetDisabled(This,pluginType,clsid,disabled)	\
    ( (This)->lpVtbl -> SetDisabled(This,pluginType,clsid,disabled) ) 


#define IMFPluginControl2_SetPolicy(This,policy)	\
    ( (This)->lpVtbl -> SetPolicy(This,policy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPluginControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0024 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0024_v0_0_s_ifspec;

#ifndef __IMFDXGIDeviceManager_INTERFACE_DEFINED__
#define __IMFDXGIDeviceManager_INTERFACE_DEFINED__

/* interface IMFDXGIDeviceManager */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFDXGIDeviceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eb533d5d-2db6-40f8-97a9-494692014f07")
    IMFDXGIDeviceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CloseDeviceHandle( 
            /* [annotation] */ 
            _In_  HANDLE hDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoService( 
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  REFIID riid,
            /* [annotation] */ 
            _Outptr_  void **ppService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockDevice( 
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  REFIID riid,
            /* [annotation] */ 
            _Outptr_  void **ppUnkDevice,
            /* [annotation] */ 
            _In_  BOOL fBlock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenDeviceHandle( 
            /* [annotation] */ 
            _Out_  HANDLE *phDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetDevice( 
            /* [annotation] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation] */ 
            _In_  UINT resetToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TestDevice( 
            /* [annotation] */ 
            _In_  HANDLE hDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockDevice( 
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  BOOL fSaveState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDXGIDeviceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDXGIDeviceManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDXGIDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDXGIDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, CloseDeviceHandle)
        HRESULT ( STDMETHODCALLTYPE *CloseDeviceHandle )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  HANDLE hDevice);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, GetVideoService)
        HRESULT ( STDMETHODCALLTYPE *GetVideoService )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  REFIID riid,
            /* [annotation] */ 
            _Outptr_  void **ppService);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, LockDevice)
        HRESULT ( STDMETHODCALLTYPE *LockDevice )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  REFIID riid,
            /* [annotation] */ 
            _Outptr_  void **ppUnkDevice,
            /* [annotation] */ 
            _In_  BOOL fBlock);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, OpenDeviceHandle)
        HRESULT ( STDMETHODCALLTYPE *OpenDeviceHandle )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _Out_  HANDLE *phDevice);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, ResetDevice)
        HRESULT ( STDMETHODCALLTYPE *ResetDevice )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  IUnknown *pUnkDevice,
            /* [annotation] */ 
            _In_  UINT resetToken);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, TestDevice)
        HRESULT ( STDMETHODCALLTYPE *TestDevice )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  HANDLE hDevice);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManager, UnlockDevice)
        HRESULT ( STDMETHODCALLTYPE *UnlockDevice )( 
            IMFDXGIDeviceManager * This,
            /* [annotation] */ 
            _In_  HANDLE hDevice,
            /* [annotation] */ 
            _In_  BOOL fSaveState);
        
        END_INTERFACE
    } IMFDXGIDeviceManagerVtbl;

    interface IMFDXGIDeviceManager
    {
        CONST_VTBL struct IMFDXGIDeviceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDXGIDeviceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDXGIDeviceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDXGIDeviceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDXGIDeviceManager_CloseDeviceHandle(This,hDevice)	\
    ( (This)->lpVtbl -> CloseDeviceHandle(This,hDevice) ) 

#define IMFDXGIDeviceManager_GetVideoService(This,hDevice,riid,ppService)	\
    ( (This)->lpVtbl -> GetVideoService(This,hDevice,riid,ppService) ) 

#define IMFDXGIDeviceManager_LockDevice(This,hDevice,riid,ppUnkDevice,fBlock)	\
    ( (This)->lpVtbl -> LockDevice(This,hDevice,riid,ppUnkDevice,fBlock) ) 

#define IMFDXGIDeviceManager_OpenDeviceHandle(This,phDevice)	\
    ( (This)->lpVtbl -> OpenDeviceHandle(This,phDevice) ) 

#define IMFDXGIDeviceManager_ResetDevice(This,pUnkDevice,resetToken)	\
    ( (This)->lpVtbl -> ResetDevice(This,pUnkDevice,resetToken) ) 

#define IMFDXGIDeviceManager_TestDevice(This,hDevice)	\
    ( (This)->lpVtbl -> TestDevice(This,hDevice) ) 

#define IMFDXGIDeviceManager_UnlockDevice(This,hDevice,fSaveState)	\
    ( (This)->lpVtbl -> UnlockDevice(This,hDevice,fSaveState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDXGIDeviceManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0025 */
/* [local] */ 

typedef 
enum MF_DXGI_DEVICE_MANAGER_MODE
    {
        MF_DXGI_DEVICE_MANAGER_MODE_INVALID	= 0,
        MF_DXGI_DEVICE_MANAGER_MODE_D3D11	= ( MF_DXGI_DEVICE_MANAGER_MODE_INVALID + 1 ) ,
        MF_DXGI_DEVICE_MANAGER_MODE_D3D12	= ( MF_DXGI_DEVICE_MANAGER_MODE_D3D11 + 1 ) 
    } 	MF_DXGI_DEVICE_MANAGER_MODE;

typedef 
enum _MF_STREAM_STATE
    {
        MF_STREAM_STATE_STOPPED	= 0,
        MF_STREAM_STATE_PAUSED	= ( MF_STREAM_STATE_STOPPED + 1 ) ,
        MF_STREAM_STATE_RUNNING	= ( MF_STREAM_STATE_PAUSED + 1 ) 
    } 	MF_STREAM_STATE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0025_v0_0_s_ifspec;

#ifndef __IMFMuxStreamAttributesManager_INTERFACE_DEFINED__
#define __IMFMuxStreamAttributesManager_INTERFACE_DEFINED__

/* interface IMFMuxStreamAttributesManager */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFMuxStreamAttributesManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CE8BD576-E440-43B3-BE34-1E53F565F7E8")
    IMFMuxStreamAttributesManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes( 
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFAttributes **ppStreamAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMuxStreamAttributesManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMuxStreamAttributesManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMuxStreamAttributesManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMuxStreamAttributesManager * This);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamAttributesManager, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFMuxStreamAttributesManager * This,
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamAttributesManager, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            IMFMuxStreamAttributesManager * This,
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFAttributes **ppStreamAttributes);
        
        END_INTERFACE
    } IMFMuxStreamAttributesManagerVtbl;

    interface IMFMuxStreamAttributesManager
    {
        CONST_VTBL struct IMFMuxStreamAttributesManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMuxStreamAttributesManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMuxStreamAttributesManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMuxStreamAttributesManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMuxStreamAttributesManager_GetStreamCount(This,pdwMuxStreamCount)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pdwMuxStreamCount) ) 

#define IMFMuxStreamAttributesManager_GetAttributes(This,dwMuxStreamIndex,ppStreamAttributes)	\
    ( (This)->lpVtbl -> GetAttributes(This,dwMuxStreamIndex,ppStreamAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMuxStreamAttributesManager_INTERFACE_DEFINED__ */


#ifndef __IMFMuxStreamMediaTypeManager_INTERFACE_DEFINED__
#define __IMFMuxStreamMediaTypeManager_INTERFACE_DEFINED__

/* interface IMFMuxStreamMediaTypeManager */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFMuxStreamMediaTypeManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("505A2C72-42F7-4690-AEAB-8F513D0FFDB8")
    IMFMuxStreamMediaTypeManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaType( 
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamConfigurationCount( 
            /* [annotation] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStreamConfiguration( 
            /* [annotation] */ 
            _In_  ULONGLONG ullStreamMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStreamConfiguration( 
            /* [annotation] */ 
            _In_  ULONGLONG ullStreamMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamConfiguration( 
            /* [annotation] */ 
            _In_  DWORD ulIndex,
            /* [annotation] */ 
            _Out_  ULONGLONG *pullStreamMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMuxStreamMediaTypeManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMuxStreamMediaTypeManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMuxStreamMediaTypeManager * This);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, GetStreamConfigurationCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamConfigurationCount )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, AddStreamConfiguration)
        HRESULT ( STDMETHODCALLTYPE *AddStreamConfiguration )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _In_  ULONGLONG ullStreamMask);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, RemoveStreamConfiguration)
        HRESULT ( STDMETHODCALLTYPE *RemoveStreamConfiguration )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _In_  ULONGLONG ullStreamMask);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamMediaTypeManager, GetStreamConfiguration)
        HRESULT ( STDMETHODCALLTYPE *GetStreamConfiguration )( 
            IMFMuxStreamMediaTypeManager * This,
            /* [annotation] */ 
            _In_  DWORD ulIndex,
            /* [annotation] */ 
            _Out_  ULONGLONG *pullStreamMask);
        
        END_INTERFACE
    } IMFMuxStreamMediaTypeManagerVtbl;

    interface IMFMuxStreamMediaTypeManager
    {
        CONST_VTBL struct IMFMuxStreamMediaTypeManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMuxStreamMediaTypeManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMuxStreamMediaTypeManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMuxStreamMediaTypeManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMuxStreamMediaTypeManager_GetStreamCount(This,pdwMuxStreamCount)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pdwMuxStreamCount) ) 

#define IMFMuxStreamMediaTypeManager_GetMediaType(This,dwMuxStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,dwMuxStreamIndex,ppMediaType) ) 

#define IMFMuxStreamMediaTypeManager_GetStreamConfigurationCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetStreamConfigurationCount(This,pdwCount) ) 

#define IMFMuxStreamMediaTypeManager_AddStreamConfiguration(This,ullStreamMask)	\
    ( (This)->lpVtbl -> AddStreamConfiguration(This,ullStreamMask) ) 

#define IMFMuxStreamMediaTypeManager_RemoveStreamConfiguration(This,ullStreamMask)	\
    ( (This)->lpVtbl -> RemoveStreamConfiguration(This,ullStreamMask) ) 

#define IMFMuxStreamMediaTypeManager_GetStreamConfiguration(This,ulIndex,pullStreamMask)	\
    ( (This)->lpVtbl -> GetStreamConfiguration(This,ulIndex,pullStreamMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMuxStreamMediaTypeManager_INTERFACE_DEFINED__ */


#ifndef __IMFMuxStreamSampleManager_INTERFACE_DEFINED__
#define __IMFMuxStreamSampleManager_INTERFACE_DEFINED__

/* interface IMFMuxStreamSampleManager */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFMuxStreamSampleManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74ABBC19-B1CC-4E41-BB8B-9D9B86A8F6CA")
    IMFMuxStreamSampleManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSample( 
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFSample **ppSample) = 0;
        
        virtual ULONGLONG STDMETHODCALLTYPE GetStreamConfiguration( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMuxStreamSampleManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMuxStreamSampleManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMuxStreamSampleManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMuxStreamSampleManager * This);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamSampleManager, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFMuxStreamSampleManager * This,
            /* [annotation] */ 
            _Out_  DWORD *pdwMuxStreamCount);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamSampleManager, GetSample)
        HRESULT ( STDMETHODCALLTYPE *GetSample )( 
            IMFMuxStreamSampleManager * This,
            /* [annotation] */ 
            _In_  DWORD dwMuxStreamIndex,
            /* [annotation] */ 
            _COM_Outptr_  IMFSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMFMuxStreamSampleManager, GetStreamConfiguration)
        ULONGLONG ( STDMETHODCALLTYPE *GetStreamConfiguration )( 
            IMFMuxStreamSampleManager * This);
        
        END_INTERFACE
    } IMFMuxStreamSampleManagerVtbl;

    interface IMFMuxStreamSampleManager
    {
        CONST_VTBL struct IMFMuxStreamSampleManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMuxStreamSampleManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMuxStreamSampleManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMuxStreamSampleManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMuxStreamSampleManager_GetStreamCount(This,pdwMuxStreamCount)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pdwMuxStreamCount) ) 

#define IMFMuxStreamSampleManager_GetSample(This,dwMuxStreamIndex,ppSample)	\
    ( (This)->lpVtbl -> GetSample(This,dwMuxStreamIndex,ppSample) ) 

#define IMFMuxStreamSampleManager_GetStreamConfiguration(This)	\
    ( (This)->lpVtbl -> GetStreamConfiguration(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMuxStreamSampleManager_INTERFACE_DEFINED__ */


#ifndef __IMFSecureBuffer_INTERFACE_DEFINED__
#define __IMFSecureBuffer_INTERFACE_DEFINED__

/* interface IMFSecureBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSecureBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C1209904-E584-4752-A2D6-7F21693F8B21")
    IMFSecureBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIdentifier( 
            /* [annotation][out] */ 
            _Out_  GUID *pGuidIdentifier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSecureBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSecureBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSecureBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSecureBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSecureBuffer, GetIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetIdentifier )( 
            IMFSecureBuffer * This,
            /* [annotation][out] */ 
            _Out_  GUID *pGuidIdentifier);
        
        END_INTERFACE
    } IMFSecureBufferVtbl;

    interface IMFSecureBuffer
    {
        CONST_VTBL struct IMFSecureBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSecureBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSecureBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSecureBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSecureBuffer_GetIdentifier(This,pGuidIdentifier)	\
    ( (This)->lpVtbl -> GetIdentifier(This,pGuidIdentifier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSecureBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfobjects_0000_0029 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfobjects_0000_0029_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_BeginGetEvent_Proxy( 
    IMFMediaEventGenerator * This,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_BeginGetEvent_Stub( 
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_EndGetEvent_Proxy( 
    IMFMediaEventGenerator * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  IMFMediaEvent **ppEvent);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaEventGenerator_EndGetEvent_Stub( 
    __RPC__in IMFMediaEventGenerator * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out DWORD *pcbEvent,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbEvent) BYTE **ppbEvent);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFByteStream_BeginRead_Proxy( 
    IMFByteStream * This,
    /* [annotation][out] */ 
    _Out_writes_bytes_(cb)  BYTE *pb,
    /* [in] */ ULONG cb,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_BeginRead_Stub( 
    __RPC__in IMFByteStream * This,
    /* [in] */ ULONG cb,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFByteStream_EndRead_Proxy( 
    IMFByteStream * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  ULONG *pcbRead);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_EndRead_Stub( 
    __RPC__in IMFByteStream * This,
    /* [in] */ __RPC__in_opt IUnknown *punkResult,
    /* [length_is][size_is][out][in] */ __RPC__inout_ecount_part(cb, *pcbRead) BYTE *pb,
    /* [in] */ ULONG cb,
    /* [out][in] */ __RPC__inout ULONG *pcbRead);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFByteStream_BeginWrite_Proxy( 
    IMFByteStream * This,
    /* [annotation][in] */ 
    _In_reads_bytes_(cb)  const BYTE *pb,
    /* [in] */ ULONG cb,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_BeginWrite_Stub( 
    __RPC__in IMFByteStream * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const BYTE *pb,
    /* [in] */ ULONG cb,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFByteStream_EndWrite_Proxy( 
    IMFByteStream * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  ULONG *pcbWritten);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFByteStream_EndWrite_Stub( 
    __RPC__in IMFByteStream * This,
    /* [in] */ __RPC__in_opt IUnknown *punkResult,
    /* [out] */ __RPC__out ULONG *pcbWritten);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


