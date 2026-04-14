

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

#ifndef __wmcontainer_h__
#define __wmcontainer_h__

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

#ifndef __IMFASFContentInfo_FWD_DEFINED__
#define __IMFASFContentInfo_FWD_DEFINED__
typedef interface IMFASFContentInfo IMFASFContentInfo;

#endif 	/* __IMFASFContentInfo_FWD_DEFINED__ */


#ifndef __IMFASFProfile_FWD_DEFINED__
#define __IMFASFProfile_FWD_DEFINED__
typedef interface IMFASFProfile IMFASFProfile;

#endif 	/* __IMFASFProfile_FWD_DEFINED__ */


#ifndef __IMFASFStreamConfig_FWD_DEFINED__
#define __IMFASFStreamConfig_FWD_DEFINED__
typedef interface IMFASFStreamConfig IMFASFStreamConfig;

#endif 	/* __IMFASFStreamConfig_FWD_DEFINED__ */


#ifndef __IMFASFMutualExclusion_FWD_DEFINED__
#define __IMFASFMutualExclusion_FWD_DEFINED__
typedef interface IMFASFMutualExclusion IMFASFMutualExclusion;

#endif 	/* __IMFASFMutualExclusion_FWD_DEFINED__ */


#ifndef __IMFASFStreamPrioritization_FWD_DEFINED__
#define __IMFASFStreamPrioritization_FWD_DEFINED__
typedef interface IMFASFStreamPrioritization IMFASFStreamPrioritization;

#endif 	/* __IMFASFStreamPrioritization_FWD_DEFINED__ */


#ifndef __IMFASFIndexer_FWD_DEFINED__
#define __IMFASFIndexer_FWD_DEFINED__
typedef interface IMFASFIndexer IMFASFIndexer;

#endif 	/* __IMFASFIndexer_FWD_DEFINED__ */


#ifndef __IMFASFSplitter_FWD_DEFINED__
#define __IMFASFSplitter_FWD_DEFINED__
typedef interface IMFASFSplitter IMFASFSplitter;

#endif 	/* __IMFASFSplitter_FWD_DEFINED__ */


#ifndef __IMFASFMultiplexer_FWD_DEFINED__
#define __IMFASFMultiplexer_FWD_DEFINED__
typedef interface IMFASFMultiplexer IMFASFMultiplexer;

#endif 	/* __IMFASFMultiplexer_FWD_DEFINED__ */


#ifndef __IMFASFStreamSelector_FWD_DEFINED__
#define __IMFASFStreamSelector_FWD_DEFINED__
typedef interface IMFASFStreamSelector IMFASFStreamSelector;

#endif 	/* __IMFASFStreamSelector_FWD_DEFINED__ */


#ifndef __IMFDRMNetHelper_FWD_DEFINED__
#define __IMFDRMNetHelper_FWD_DEFINED__
typedef interface IMFDRMNetHelper IMFDRMNetHelper;

#endif 	/* __IMFDRMNetHelper_FWD_DEFINED__ */


/* header files for imported files */
#include "mfidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmcontainer_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)



extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0000_v0_0_s_ifspec;

#ifndef __IMFASFContentInfo_INTERFACE_DEFINED__
#define __IMFASFContentInfo_INTERFACE_DEFINED__

/* interface IMFASFContentInfo */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFContentInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1DCA5CD-D5DA-4451-8E9E-DB5C59914EAD")
    IMFASFContentInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHeaderSize( 
            /* [in] */ IMFMediaBuffer *pIStartOfContent,
            /* [out] */ QWORD *cbHeaderSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseHeader( 
            /* [in] */ IMFMediaBuffer *pIHeaderBuffer,
            /* [in] */ QWORD cbOffsetWithinHeader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateHeader( 
            /* [out][in] */ IMFMediaBuffer *pIHeader,
            /* [out] */ DWORD *pcbHeader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProfile( 
            /* [out] */ IMFASFProfile **ppIProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProfile( 
            /* [in] */ IMFASFProfile *pIProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GeneratePresentationDescriptor( 
            /* [out] */ IMFPresentationDescriptor **ppIPresentationDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEncodingConfigurationPropertyStore( 
            /* [in] */ WORD wStreamNumber,
            /* [out] */ IPropertyStore **ppIStore) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFContentInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFContentInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFContentInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFContentInfo * This);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, GetHeaderSize)
        HRESULT ( STDMETHODCALLTYPE *GetHeaderSize )( 
            IMFASFContentInfo * This,
            /* [in] */ IMFMediaBuffer *pIStartOfContent,
            /* [out] */ QWORD *cbHeaderSize);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, ParseHeader)
        HRESULT ( STDMETHODCALLTYPE *ParseHeader )( 
            IMFASFContentInfo * This,
            /* [in] */ IMFMediaBuffer *pIHeaderBuffer,
            /* [in] */ QWORD cbOffsetWithinHeader);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, GenerateHeader)
        HRESULT ( STDMETHODCALLTYPE *GenerateHeader )( 
            IMFASFContentInfo * This,
            /* [out][in] */ IMFMediaBuffer *pIHeader,
            /* [out] */ DWORD *pcbHeader);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, GetProfile)
        HRESULT ( STDMETHODCALLTYPE *GetProfile )( 
            IMFASFContentInfo * This,
            /* [out] */ IMFASFProfile **ppIProfile);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, SetProfile)
        HRESULT ( STDMETHODCALLTYPE *SetProfile )( 
            IMFASFContentInfo * This,
            /* [in] */ IMFASFProfile *pIProfile);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, GeneratePresentationDescriptor)
        HRESULT ( STDMETHODCALLTYPE *GeneratePresentationDescriptor )( 
            IMFASFContentInfo * This,
            /* [out] */ IMFPresentationDescriptor **ppIPresentationDescriptor);
        
        DECLSPEC_XFGVIRT(IMFASFContentInfo, GetEncodingConfigurationPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *GetEncodingConfigurationPropertyStore )( 
            IMFASFContentInfo * This,
            /* [in] */ WORD wStreamNumber,
            /* [out] */ IPropertyStore **ppIStore);
        
        END_INTERFACE
    } IMFASFContentInfoVtbl;

    interface IMFASFContentInfo
    {
        CONST_VTBL struct IMFASFContentInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFContentInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFContentInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFContentInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFContentInfo_GetHeaderSize(This,pIStartOfContent,cbHeaderSize)	\
    ( (This)->lpVtbl -> GetHeaderSize(This,pIStartOfContent,cbHeaderSize) ) 

#define IMFASFContentInfo_ParseHeader(This,pIHeaderBuffer,cbOffsetWithinHeader)	\
    ( (This)->lpVtbl -> ParseHeader(This,pIHeaderBuffer,cbOffsetWithinHeader) ) 

#define IMFASFContentInfo_GenerateHeader(This,pIHeader,pcbHeader)	\
    ( (This)->lpVtbl -> GenerateHeader(This,pIHeader,pcbHeader) ) 

#define IMFASFContentInfo_GetProfile(This,ppIProfile)	\
    ( (This)->lpVtbl -> GetProfile(This,ppIProfile) ) 

#define IMFASFContentInfo_SetProfile(This,pIProfile)	\
    ( (This)->lpVtbl -> SetProfile(This,pIProfile) ) 

#define IMFASFContentInfo_GeneratePresentationDescriptor(This,ppIPresentationDescriptor)	\
    ( (This)->lpVtbl -> GeneratePresentationDescriptor(This,ppIPresentationDescriptor) ) 

#define IMFASFContentInfo_GetEncodingConfigurationPropertyStore(This,wStreamNumber,ppIStore)	\
    ( (This)->lpVtbl -> GetEncodingConfigurationPropertyStore(This,wStreamNumber,ppIStore) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFContentInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0001 */
/* [local] */ 

STDAPI MFCreateASFContentInfo( 
        /* out */     IMFASFContentInfo     **ppIContentInfo);





extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0001_v0_0_s_ifspec;

#ifndef __IMFASFProfile_INTERFACE_DEFINED__
#define __IMFASFProfile_INTERFACE_DEFINED__

/* interface IMFASFProfile */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D267BF6A-028B-4e0d-903D-43F0EF82D0D4")
    IMFASFProfile : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [out] */ DWORD *pcStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [in] */ DWORD dwStreamIndex,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ IMFASFStreamConfig **ppIStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamByNumber( 
            /* [in] */ WORD wStreamNumber,
            /* [out] */ IMFASFStreamConfig **ppIStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStream( 
            /* [in] */ IMFASFStreamConfig *pIStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStream( 
            /* [in] */ WORD wStreamNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStream( 
            /* [in] */ IMFMediaType *pIMediaType,
            /* [out] */ IMFASFStreamConfig **ppIStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMutualExclusionCount( 
            /* [out] */ DWORD *pcMutexs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMutualExclusion( 
            /* [in] */ DWORD dwMutexIndex,
            /* [out] */ IMFASFMutualExclusion **ppIMutex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMutualExclusion( 
            /* [in] */ IMFASFMutualExclusion *pIMutex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveMutualExclusion( 
            /* [in] */ DWORD dwMutexIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateMutualExclusion( 
            /* [out] */ IMFASFMutualExclusion **ppIMutex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamPrioritization( 
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStreamPrioritization( 
            /* [in] */ IMFASFStreamPrioritization *pIStreamPrioritization) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStreamPrioritization( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStreamPrioritization( 
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IMFASFProfile **ppIProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFProfile * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFASFProfile * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFASFProfile * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFASFProfile * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFASFProfile * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFASFProfile * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFASFProfile * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFASFProfile * This,
            /* [out] */ DWORD *pcStreams);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IMFASFProfile * This,
            /* [in] */ DWORD dwStreamIndex,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ IMFASFStreamConfig **ppIStream);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetStreamByNumber)
        HRESULT ( STDMETHODCALLTYPE *GetStreamByNumber )( 
            IMFASFProfile * This,
            /* [in] */ WORD wStreamNumber,
            /* [out] */ IMFASFStreamConfig **ppIStream);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, SetStream)
        HRESULT ( STDMETHODCALLTYPE *SetStream )( 
            IMFASFProfile * This,
            /* [in] */ IMFASFStreamConfig *pIStream);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, RemoveStream)
        HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            IMFASFProfile * This,
            /* [in] */ WORD wStreamNumber);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, CreateStream)
        HRESULT ( STDMETHODCALLTYPE *CreateStream )( 
            IMFASFProfile * This,
            /* [in] */ IMFMediaType *pIMediaType,
            /* [out] */ IMFASFStreamConfig **ppIStream);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetMutualExclusionCount)
        HRESULT ( STDMETHODCALLTYPE *GetMutualExclusionCount )( 
            IMFASFProfile * This,
            /* [out] */ DWORD *pcMutexs);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetMutualExclusion)
        HRESULT ( STDMETHODCALLTYPE *GetMutualExclusion )( 
            IMFASFProfile * This,
            /* [in] */ DWORD dwMutexIndex,
            /* [out] */ IMFASFMutualExclusion **ppIMutex);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, AddMutualExclusion)
        HRESULT ( STDMETHODCALLTYPE *AddMutualExclusion )( 
            IMFASFProfile * This,
            /* [in] */ IMFASFMutualExclusion *pIMutex);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, RemoveMutualExclusion)
        HRESULT ( STDMETHODCALLTYPE *RemoveMutualExclusion )( 
            IMFASFProfile * This,
            /* [in] */ DWORD dwMutexIndex);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, CreateMutualExclusion)
        HRESULT ( STDMETHODCALLTYPE *CreateMutualExclusion )( 
            IMFASFProfile * This,
            /* [out] */ IMFASFMutualExclusion **ppIMutex);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, GetStreamPrioritization)
        HRESULT ( STDMETHODCALLTYPE *GetStreamPrioritization )( 
            IMFASFProfile * This,
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, AddStreamPrioritization)
        HRESULT ( STDMETHODCALLTYPE *AddStreamPrioritization )( 
            IMFASFProfile * This,
            /* [in] */ IMFASFStreamPrioritization *pIStreamPrioritization);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, RemoveStreamPrioritization)
        HRESULT ( STDMETHODCALLTYPE *RemoveStreamPrioritization )( 
            IMFASFProfile * This);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, CreateStreamPrioritization)
        HRESULT ( STDMETHODCALLTYPE *CreateStreamPrioritization )( 
            IMFASFProfile * This,
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization);
        
        DECLSPEC_XFGVIRT(IMFASFProfile, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMFASFProfile * This,
            /* [out] */ IMFASFProfile **ppIProfile);
        
        END_INTERFACE
    } IMFASFProfileVtbl;

    interface IMFASFProfile
    {
        CONST_VTBL struct IMFASFProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFProfile_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFASFProfile_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFASFProfile_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFASFProfile_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFASFProfile_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFASFProfile_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFASFProfile_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFASFProfile_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFASFProfile_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFASFProfile_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFASFProfile_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFASFProfile_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFASFProfile_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFASFProfile_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFASFProfile_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFASFProfile_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFASFProfile_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFASFProfile_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFASFProfile_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFASFProfile_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFASFProfile_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFASFProfile_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFASFProfile_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFASFProfile_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFASFProfile_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFASFProfile_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFASFProfile_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFASFProfile_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFASFProfile_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFASFProfile_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFASFProfile_GetStreamCount(This,pcStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcStreams) ) 

#define IMFASFProfile_GetStream(This,dwStreamIndex,pwStreamNumber,ppIStream)	\
    ( (This)->lpVtbl -> GetStream(This,dwStreamIndex,pwStreamNumber,ppIStream) ) 

#define IMFASFProfile_GetStreamByNumber(This,wStreamNumber,ppIStream)	\
    ( (This)->lpVtbl -> GetStreamByNumber(This,wStreamNumber,ppIStream) ) 

#define IMFASFProfile_SetStream(This,pIStream)	\
    ( (This)->lpVtbl -> SetStream(This,pIStream) ) 

#define IMFASFProfile_RemoveStream(This,wStreamNumber)	\
    ( (This)->lpVtbl -> RemoveStream(This,wStreamNumber) ) 

#define IMFASFProfile_CreateStream(This,pIMediaType,ppIStream)	\
    ( (This)->lpVtbl -> CreateStream(This,pIMediaType,ppIStream) ) 

#define IMFASFProfile_GetMutualExclusionCount(This,pcMutexs)	\
    ( (This)->lpVtbl -> GetMutualExclusionCount(This,pcMutexs) ) 

#define IMFASFProfile_GetMutualExclusion(This,dwMutexIndex,ppIMutex)	\
    ( (This)->lpVtbl -> GetMutualExclusion(This,dwMutexIndex,ppIMutex) ) 

#define IMFASFProfile_AddMutualExclusion(This,pIMutex)	\
    ( (This)->lpVtbl -> AddMutualExclusion(This,pIMutex) ) 

#define IMFASFProfile_RemoveMutualExclusion(This,dwMutexIndex)	\
    ( (This)->lpVtbl -> RemoveMutualExclusion(This,dwMutexIndex) ) 

#define IMFASFProfile_CreateMutualExclusion(This,ppIMutex)	\
    ( (This)->lpVtbl -> CreateMutualExclusion(This,ppIMutex) ) 

#define IMFASFProfile_GetStreamPrioritization(This,ppIStreamPrioritization)	\
    ( (This)->lpVtbl -> GetStreamPrioritization(This,ppIStreamPrioritization) ) 

#define IMFASFProfile_AddStreamPrioritization(This,pIStreamPrioritization)	\
    ( (This)->lpVtbl -> AddStreamPrioritization(This,pIStreamPrioritization) ) 

#define IMFASFProfile_RemoveStreamPrioritization(This)	\
    ( (This)->lpVtbl -> RemoveStreamPrioritization(This) ) 

#define IMFASFProfile_CreateStreamPrioritization(This,ppIStreamPrioritization)	\
    ( (This)->lpVtbl -> CreateStreamPrioritization(This,ppIStreamPrioritization) ) 

#define IMFASFProfile_Clone(This,ppIProfile)	\
    ( (This)->lpVtbl -> Clone(This,ppIProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFProfile_INTERFACE_DEFINED__ */


#ifndef __IMFASFStreamConfig_INTERFACE_DEFINED__
#define __IMFASFStreamConfig_INTERFACE_DEFINED__

/* interface IMFASFStreamConfig */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFStreamConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9E8AE8D2-DBBD-4200-9ACA-06E6DF484913")
    IMFASFStreamConfig : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamType( 
            /* [out] */ GUID *pguidStreamType) = 0;
        
        virtual WORD STDMETHODCALLTYPE GetStreamNumber( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamNumber( 
            /* [in] */ WORD wStreamNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaType( 
            /* [out] */ IMFMediaType **ppIMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [in] */ IMFMediaType *pIMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPayloadExtensionCount( 
            /* [out] */ WORD *pcPayloadExtensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPayloadExtension( 
            /* [in] */ WORD wPayloadExtensionNumber,
            /* [out] */ GUID *pguidExtensionSystemID,
            /* [out] */ WORD *pcbExtensionDataSize,
            /* [size_is][optional][out] */ BYTE *pbExtensionSystemInfo,
            /* [optional][out][in] */ DWORD *pcbExtensionSystemInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPayloadExtension( 
            /* [in] */ GUID guidExtensionSystemID,
            /* [in] */ WORD cbExtensionDataSize,
            /* [size_is][in] */ BYTE *pbExtensionSystemInfo,
            /* [in] */ DWORD cbExtensionSystemInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllPayloadExtensions( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IMFASFStreamConfig **ppIStreamConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFStreamConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFStreamConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFASFStreamConfig * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFASFStreamConfig * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFASFStreamConfig * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFASFStreamConfig * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFASFStreamConfig * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, GetStreamType)
        HRESULT ( STDMETHODCALLTYPE *GetStreamType )( 
            IMFASFStreamConfig * This,
            /* [out] */ GUID *pguidStreamType);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, GetStreamNumber)
        WORD ( STDMETHODCALLTYPE *GetStreamNumber )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, SetStreamNumber)
        HRESULT ( STDMETHODCALLTYPE *SetStreamNumber )( 
            IMFASFStreamConfig * This,
            /* [in] */ WORD wStreamNum);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IMFASFStreamConfig * This,
            /* [out] */ IMFMediaType **ppIMediaType);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            IMFASFStreamConfig * This,
            /* [in] */ IMFMediaType *pIMediaType);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, GetPayloadExtensionCount)
        HRESULT ( STDMETHODCALLTYPE *GetPayloadExtensionCount )( 
            IMFASFStreamConfig * This,
            /* [out] */ WORD *pcPayloadExtensions);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, GetPayloadExtension)
        HRESULT ( STDMETHODCALLTYPE *GetPayloadExtension )( 
            IMFASFStreamConfig * This,
            /* [in] */ WORD wPayloadExtensionNumber,
            /* [out] */ GUID *pguidExtensionSystemID,
            /* [out] */ WORD *pcbExtensionDataSize,
            /* [size_is][optional][out] */ BYTE *pbExtensionSystemInfo,
            /* [optional][out][in] */ DWORD *pcbExtensionSystemInfo);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, AddPayloadExtension)
        HRESULT ( STDMETHODCALLTYPE *AddPayloadExtension )( 
            IMFASFStreamConfig * This,
            /* [in] */ GUID guidExtensionSystemID,
            /* [in] */ WORD cbExtensionDataSize,
            /* [size_is][in] */ BYTE *pbExtensionSystemInfo,
            /* [in] */ DWORD cbExtensionSystemInfo);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, RemoveAllPayloadExtensions)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllPayloadExtensions )( 
            IMFASFStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IMFASFStreamConfig, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMFASFStreamConfig * This,
            /* [out] */ IMFASFStreamConfig **ppIStreamConfig);
        
        END_INTERFACE
    } IMFASFStreamConfigVtbl;

    interface IMFASFStreamConfig
    {
        CONST_VTBL struct IMFASFStreamConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFStreamConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFStreamConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFStreamConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFStreamConfig_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFASFStreamConfig_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFASFStreamConfig_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFASFStreamConfig_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFASFStreamConfig_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFASFStreamConfig_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFASFStreamConfig_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFASFStreamConfig_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFASFStreamConfig_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFASFStreamConfig_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFASFStreamConfig_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFASFStreamConfig_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFASFStreamConfig_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFASFStreamConfig_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFASFStreamConfig_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFASFStreamConfig_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFASFStreamConfig_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFASFStreamConfig_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFASFStreamConfig_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFASFStreamConfig_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFASFStreamConfig_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFASFStreamConfig_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFASFStreamConfig_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFASFStreamConfig_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFASFStreamConfig_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFASFStreamConfig_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFASFStreamConfig_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFASFStreamConfig_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFASFStreamConfig_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFASFStreamConfig_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFASFStreamConfig_GetStreamType(This,pguidStreamType)	\
    ( (This)->lpVtbl -> GetStreamType(This,pguidStreamType) ) 

#define IMFASFStreamConfig_GetStreamNumber(This)	\
    ( (This)->lpVtbl -> GetStreamNumber(This) ) 

#define IMFASFStreamConfig_SetStreamNumber(This,wStreamNum)	\
    ( (This)->lpVtbl -> SetStreamNumber(This,wStreamNum) ) 

#define IMFASFStreamConfig_GetMediaType(This,ppIMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,ppIMediaType) ) 

#define IMFASFStreamConfig_SetMediaType(This,pIMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,pIMediaType) ) 

#define IMFASFStreamConfig_GetPayloadExtensionCount(This,pcPayloadExtensions)	\
    ( (This)->lpVtbl -> GetPayloadExtensionCount(This,pcPayloadExtensions) ) 

#define IMFASFStreamConfig_GetPayloadExtension(This,wPayloadExtensionNumber,pguidExtensionSystemID,pcbExtensionDataSize,pbExtensionSystemInfo,pcbExtensionSystemInfo)	\
    ( (This)->lpVtbl -> GetPayloadExtension(This,wPayloadExtensionNumber,pguidExtensionSystemID,pcbExtensionDataSize,pbExtensionSystemInfo,pcbExtensionSystemInfo) ) 

#define IMFASFStreamConfig_AddPayloadExtension(This,guidExtensionSystemID,cbExtensionDataSize,pbExtensionSystemInfo,cbExtensionSystemInfo)	\
    ( (This)->lpVtbl -> AddPayloadExtension(This,guidExtensionSystemID,cbExtensionDataSize,pbExtensionSystemInfo,cbExtensionSystemInfo) ) 

#define IMFASFStreamConfig_RemoveAllPayloadExtensions(This)	\
    ( (This)->lpVtbl -> RemoveAllPayloadExtensions(This) ) 

#define IMFASFStreamConfig_Clone(This,ppIStreamConfig)	\
    ( (This)->lpVtbl -> Clone(This,ppIStreamConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFStreamConfig_INTERFACE_DEFINED__ */


#ifndef __IMFASFMutualExclusion_INTERFACE_DEFINED__
#define __IMFASFMutualExclusion_INTERFACE_DEFINED__

/* interface IMFASFMutualExclusion */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFMutualExclusion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12558291-E399-11D5-BC2A-00B0D0F3F4AB")
    IMFASFMutualExclusion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ GUID *pguidType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetType( 
            /* [in] */ REFGUID guidType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCount( 
            /* [out] */ DWORD *pdwRecordCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamsForRecord( 
            /* [in] */ DWORD dwRecordNumber,
            /* [out] */ WORD *pwStreamNumArray,
            /* [out][in] */ DWORD *pcStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStreamForRecord( 
            /* [in] */ DWORD dwRecordNumber,
            /* [in] */ WORD wStreamNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStreamFromRecord( 
            /* [in] */ DWORD dwRecordNumber,
            /* [in] */ WORD wStreamNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveRecord( 
            /* [in] */ DWORD dwRecordNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRecord( 
            /* [out] */ DWORD *pdwRecordNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IMFASFMutualExclusion **ppIMutex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFMutualExclusionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFMutualExclusion * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFMutualExclusion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFMutualExclusion * This);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            IMFASFMutualExclusion * This,
            /* [out] */ GUID *pguidType);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, SetType)
        HRESULT ( STDMETHODCALLTYPE *SetType )( 
            IMFASFMutualExclusion * This,
            /* [in] */ REFGUID guidType);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, GetRecordCount)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCount )( 
            IMFASFMutualExclusion * This,
            /* [out] */ DWORD *pdwRecordCount);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, GetStreamsForRecord)
        HRESULT ( STDMETHODCALLTYPE *GetStreamsForRecord )( 
            IMFASFMutualExclusion * This,
            /* [in] */ DWORD dwRecordNumber,
            /* [out] */ WORD *pwStreamNumArray,
            /* [out][in] */ DWORD *pcStreams);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, AddStreamForRecord)
        HRESULT ( STDMETHODCALLTYPE *AddStreamForRecord )( 
            IMFASFMutualExclusion * This,
            /* [in] */ DWORD dwRecordNumber,
            /* [in] */ WORD wStreamNumber);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, RemoveStreamFromRecord)
        HRESULT ( STDMETHODCALLTYPE *RemoveStreamFromRecord )( 
            IMFASFMutualExclusion * This,
            /* [in] */ DWORD dwRecordNumber,
            /* [in] */ WORD wStreamNumber);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, RemoveRecord)
        HRESULT ( STDMETHODCALLTYPE *RemoveRecord )( 
            IMFASFMutualExclusion * This,
            /* [in] */ DWORD dwRecordNumber);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, AddRecord)
        HRESULT ( STDMETHODCALLTYPE *AddRecord )( 
            IMFASFMutualExclusion * This,
            /* [out] */ DWORD *pdwRecordNumber);
        
        DECLSPEC_XFGVIRT(IMFASFMutualExclusion, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMFASFMutualExclusion * This,
            /* [out] */ IMFASFMutualExclusion **ppIMutex);
        
        END_INTERFACE
    } IMFASFMutualExclusionVtbl;

    interface IMFASFMutualExclusion
    {
        CONST_VTBL struct IMFASFMutualExclusionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFMutualExclusion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFMutualExclusion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFMutualExclusion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFMutualExclusion_GetType(This,pguidType)	\
    ( (This)->lpVtbl -> GetType(This,pguidType) ) 

#define IMFASFMutualExclusion_SetType(This,guidType)	\
    ( (This)->lpVtbl -> SetType(This,guidType) ) 

#define IMFASFMutualExclusion_GetRecordCount(This,pdwRecordCount)	\
    ( (This)->lpVtbl -> GetRecordCount(This,pdwRecordCount) ) 

#define IMFASFMutualExclusion_GetStreamsForRecord(This,dwRecordNumber,pwStreamNumArray,pcStreams)	\
    ( (This)->lpVtbl -> GetStreamsForRecord(This,dwRecordNumber,pwStreamNumArray,pcStreams) ) 

#define IMFASFMutualExclusion_AddStreamForRecord(This,dwRecordNumber,wStreamNumber)	\
    ( (This)->lpVtbl -> AddStreamForRecord(This,dwRecordNumber,wStreamNumber) ) 

#define IMFASFMutualExclusion_RemoveStreamFromRecord(This,dwRecordNumber,wStreamNumber)	\
    ( (This)->lpVtbl -> RemoveStreamFromRecord(This,dwRecordNumber,wStreamNumber) ) 

#define IMFASFMutualExclusion_RemoveRecord(This,dwRecordNumber)	\
    ( (This)->lpVtbl -> RemoveRecord(This,dwRecordNumber) ) 

#define IMFASFMutualExclusion_AddRecord(This,pdwRecordNumber)	\
    ( (This)->lpVtbl -> AddRecord(This,pdwRecordNumber) ) 

#define IMFASFMutualExclusion_Clone(This,ppIMutex)	\
    ( (This)->lpVtbl -> Clone(This,ppIMutex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFMutualExclusion_INTERFACE_DEFINED__ */


#ifndef __IMFASFStreamPrioritization_INTERFACE_DEFINED__
#define __IMFASFStreamPrioritization_INTERFACE_DEFINED__

/* interface IMFASFStreamPrioritization */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFStreamPrioritization;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("699bdc27-bbaf-49ff-8e38-9c39c9b5e088")
    IMFASFStreamPrioritization : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [out] */ DWORD *pdwStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [in] */ DWORD dwStreamIndex,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ WORD *pwStreamFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStream( 
            /* [in] */ WORD wStreamNumber,
            /* [in] */ WORD wStreamFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStream( 
            /* [in] */ DWORD dwStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFStreamPrioritizationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFStreamPrioritization * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFStreamPrioritization * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFStreamPrioritization * This);
        
        DECLSPEC_XFGVIRT(IMFASFStreamPrioritization, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFASFStreamPrioritization * This,
            /* [out] */ DWORD *pdwStreamCount);
        
        DECLSPEC_XFGVIRT(IMFASFStreamPrioritization, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IMFASFStreamPrioritization * This,
            /* [in] */ DWORD dwStreamIndex,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ WORD *pwStreamFlags);
        
        DECLSPEC_XFGVIRT(IMFASFStreamPrioritization, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFASFStreamPrioritization * This,
            /* [in] */ WORD wStreamNumber,
            /* [in] */ WORD wStreamFlags);
        
        DECLSPEC_XFGVIRT(IMFASFStreamPrioritization, RemoveStream)
        HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            IMFASFStreamPrioritization * This,
            /* [in] */ DWORD dwStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFASFStreamPrioritization, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMFASFStreamPrioritization * This,
            /* [out] */ IMFASFStreamPrioritization **ppIStreamPrioritization);
        
        END_INTERFACE
    } IMFASFStreamPrioritizationVtbl;

    interface IMFASFStreamPrioritization
    {
        CONST_VTBL struct IMFASFStreamPrioritizationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFStreamPrioritization_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFStreamPrioritization_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFStreamPrioritization_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFStreamPrioritization_GetStreamCount(This,pdwStreamCount)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pdwStreamCount) ) 

#define IMFASFStreamPrioritization_GetStream(This,dwStreamIndex,pwStreamNumber,pwStreamFlags)	\
    ( (This)->lpVtbl -> GetStream(This,dwStreamIndex,pwStreamNumber,pwStreamFlags) ) 

#define IMFASFStreamPrioritization_AddStream(This,wStreamNumber,wStreamFlags)	\
    ( (This)->lpVtbl -> AddStream(This,wStreamNumber,wStreamFlags) ) 

#define IMFASFStreamPrioritization_RemoveStream(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> RemoveStream(This,dwStreamIndex) ) 

#define IMFASFStreamPrioritization_Clone(This,ppIStreamPrioritization)	\
    ( (This)->lpVtbl -> Clone(This,ppIStreamPrioritization) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFStreamPrioritization_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0005 */
/* [local] */ 

typedef 
enum MFASF_INDEXERFLAGS
    {
        MFASF_INDEXER_WRITE_NEW_INDEX	= 0x1,
        MFASF_INDEXER_READ_FOR_REVERSEPLAYBACK	= 0x2,
        MFASF_INDEXER_WRITE_FOR_LIVEREAD	= 0x4
    } 	MFASF_INDEXER_FLAGS;

typedef struct _ASF_INDEX_IDENTIFIER
    {
    GUID guidIndexType;
    WORD wStreamNumber;
    } 	ASF_INDEX_IDENTIFIER;

typedef struct _ASF_INDEX_DESCRIPTOR
    {
    ASF_INDEX_IDENTIFIER Identifier;
    WORD cPerEntryBytes;
    WCHAR szDescription[ 32 ];
    DWORD dwInterval;
    } 	ASF_INDEX_DESCRIPTOR;



extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0005_v0_0_s_ifspec;

#ifndef __IMFASFIndexer_INTERFACE_DEFINED__
#define __IMFASFIndexer_INTERFACE_DEFINED__

/* interface IMFASFIndexer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFIndexer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53590F48-DC3B-4297-813F-787761AD7B3E")
    IMFASFIndexer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IMFASFContentInfo *pIContentInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndexPosition( 
            /* [in] */ IMFASFContentInfo *pIContentInfo,
            /* [out] */ QWORD *pcbIndexOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIndexByteStreams( 
            /* [in] */ IMFByteStream **ppIByteStreams,
            /* [in] */ DWORD cByteStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndexByteStreamCount( 
            /* [out] */ DWORD *pcByteStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndexStatus( 
            /* [in] */ ASF_INDEX_IDENTIFIER *pIndexIdentifier,
            /* [out] */ BOOL *pfIsIndexed,
            /* [out] */ BYTE *pbIndexDescriptor,
            /* [out][in] */ DWORD *pcbIndexDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIndexStatus( 
            /* [in] */ BYTE *pbIndexDescriptor,
            /* [in] */ DWORD cbIndexDescriptor,
            /* [in] */ BOOL fGenerateIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSeekPositionForValue( 
            /* [in] */ const PROPVARIANT *pvarValue,
            /* [in] */ ASF_INDEX_IDENTIFIER *pIndexIdentifier,
            /* [out] */ QWORD *pcbOffsetWithinData,
            /* [optional][out] */ MFTIME *phnsApproxTime,
            /* [optional][out] */ DWORD *pdwPayloadNumberOfStreamWithinPacket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateIndexEntries( 
            /* [in] */ IMFSample *pIASFPacketSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitIndex( 
            /* [in] */ IMFASFContentInfo *pIContentInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndexWriteSpace( 
            /* [out] */ QWORD *pcbIndexWriteSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompletedIndex( 
            /* [in] */ IMFMediaBuffer *pIIndexBuffer,
            /* [in] */ QWORD cbOffsetWithinIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFIndexerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFIndexer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFIndexer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFIndexer * This);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IMFASFIndexer * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IMFASFIndexer * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMFASFIndexer * This,
            /* [in] */ IMFASFContentInfo *pIContentInfo);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetIndexPosition)
        HRESULT ( STDMETHODCALLTYPE *GetIndexPosition )( 
            IMFASFIndexer * This,
            /* [in] */ IMFASFContentInfo *pIContentInfo,
            /* [out] */ QWORD *pcbIndexOffset);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, SetIndexByteStreams)
        HRESULT ( STDMETHODCALLTYPE *SetIndexByteStreams )( 
            IMFASFIndexer * This,
            /* [in] */ IMFByteStream **ppIByteStreams,
            /* [in] */ DWORD cByteStreams);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetIndexByteStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetIndexByteStreamCount )( 
            IMFASFIndexer * This,
            /* [out] */ DWORD *pcByteStreams);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetIndexStatus)
        HRESULT ( STDMETHODCALLTYPE *GetIndexStatus )( 
            IMFASFIndexer * This,
            /* [in] */ ASF_INDEX_IDENTIFIER *pIndexIdentifier,
            /* [out] */ BOOL *pfIsIndexed,
            /* [out] */ BYTE *pbIndexDescriptor,
            /* [out][in] */ DWORD *pcbIndexDescriptor);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, SetIndexStatus)
        HRESULT ( STDMETHODCALLTYPE *SetIndexStatus )( 
            IMFASFIndexer * This,
            /* [in] */ BYTE *pbIndexDescriptor,
            /* [in] */ DWORD cbIndexDescriptor,
            /* [in] */ BOOL fGenerateIndex);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetSeekPositionForValue)
        HRESULT ( STDMETHODCALLTYPE *GetSeekPositionForValue )( 
            IMFASFIndexer * This,
            /* [in] */ const PROPVARIANT *pvarValue,
            /* [in] */ ASF_INDEX_IDENTIFIER *pIndexIdentifier,
            /* [out] */ QWORD *pcbOffsetWithinData,
            /* [optional][out] */ MFTIME *phnsApproxTime,
            /* [optional][out] */ DWORD *pdwPayloadNumberOfStreamWithinPacket);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GenerateIndexEntries)
        HRESULT ( STDMETHODCALLTYPE *GenerateIndexEntries )( 
            IMFASFIndexer * This,
            /* [in] */ IMFSample *pIASFPacketSample);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, CommitIndex)
        HRESULT ( STDMETHODCALLTYPE *CommitIndex )( 
            IMFASFIndexer * This,
            /* [in] */ IMFASFContentInfo *pIContentInfo);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetIndexWriteSpace)
        HRESULT ( STDMETHODCALLTYPE *GetIndexWriteSpace )( 
            IMFASFIndexer * This,
            /* [out] */ QWORD *pcbIndexWriteSpace);
        
        DECLSPEC_XFGVIRT(IMFASFIndexer, GetCompletedIndex)
        HRESULT ( STDMETHODCALLTYPE *GetCompletedIndex )( 
            IMFASFIndexer * This,
            /* [in] */ IMFMediaBuffer *pIIndexBuffer,
            /* [in] */ QWORD cbOffsetWithinIndex);
        
        END_INTERFACE
    } IMFASFIndexerVtbl;

    interface IMFASFIndexer
    {
        CONST_VTBL struct IMFASFIndexerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFIndexer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFIndexer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFIndexer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFIndexer_SetFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags) ) 

#define IMFASFIndexer_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IMFASFIndexer_Initialize(This,pIContentInfo)	\
    ( (This)->lpVtbl -> Initialize(This,pIContentInfo) ) 

#define IMFASFIndexer_GetIndexPosition(This,pIContentInfo,pcbIndexOffset)	\
    ( (This)->lpVtbl -> GetIndexPosition(This,pIContentInfo,pcbIndexOffset) ) 

#define IMFASFIndexer_SetIndexByteStreams(This,ppIByteStreams,cByteStreams)	\
    ( (This)->lpVtbl -> SetIndexByteStreams(This,ppIByteStreams,cByteStreams) ) 

#define IMFASFIndexer_GetIndexByteStreamCount(This,pcByteStreams)	\
    ( (This)->lpVtbl -> GetIndexByteStreamCount(This,pcByteStreams) ) 

#define IMFASFIndexer_GetIndexStatus(This,pIndexIdentifier,pfIsIndexed,pbIndexDescriptor,pcbIndexDescriptor)	\
    ( (This)->lpVtbl -> GetIndexStatus(This,pIndexIdentifier,pfIsIndexed,pbIndexDescriptor,pcbIndexDescriptor) ) 

#define IMFASFIndexer_SetIndexStatus(This,pbIndexDescriptor,cbIndexDescriptor,fGenerateIndex)	\
    ( (This)->lpVtbl -> SetIndexStatus(This,pbIndexDescriptor,cbIndexDescriptor,fGenerateIndex) ) 

#define IMFASFIndexer_GetSeekPositionForValue(This,pvarValue,pIndexIdentifier,pcbOffsetWithinData,phnsApproxTime,pdwPayloadNumberOfStreamWithinPacket)	\
    ( (This)->lpVtbl -> GetSeekPositionForValue(This,pvarValue,pIndexIdentifier,pcbOffsetWithinData,phnsApproxTime,pdwPayloadNumberOfStreamWithinPacket) ) 

#define IMFASFIndexer_GenerateIndexEntries(This,pIASFPacketSample)	\
    ( (This)->lpVtbl -> GenerateIndexEntries(This,pIASFPacketSample) ) 

#define IMFASFIndexer_CommitIndex(This,pIContentInfo)	\
    ( (This)->lpVtbl -> CommitIndex(This,pIContentInfo) ) 

#define IMFASFIndexer_GetIndexWriteSpace(This,pcbIndexWriteSpace)	\
    ( (This)->lpVtbl -> GetIndexWriteSpace(This,pcbIndexWriteSpace) ) 

#define IMFASFIndexer_GetCompletedIndex(This,pIIndexBuffer,cbOffsetWithinIndex)	\
    ( (This)->lpVtbl -> GetCompletedIndex(This,pIIndexBuffer,cbOffsetWithinIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFIndexer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0006 */
/* [local] */ 

STDAPI MFCreateASFIndexer( 
        /* out */     IMFASFIndexer         **ppIIndexer);
STDAPI MFCreateASFIndexerByteStream( 
        /* in */      IMFByteStream         *pIContentByteStream,
        /* in */      QWORD                  cbIndexStartOffset,
        /* out */     IMFByteStream        **pIIndexByteStream);


extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0006_v0_0_s_ifspec;

#ifndef __IMFASFSplitter_INTERFACE_DEFINED__
#define __IMFASFSplitter_INTERFACE_DEFINED__

/* interface IMFASFSplitter */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFSplitter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12558295-E399-11D5-BC2A-00B0D0F3F4AB")
    IMFASFSplitter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IMFASFContentInfo *pIContentInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectStreams( 
            /* [in] */ WORD *pwStreamNumbers,
            /* [in] */ WORD wNumStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectedStreams( 
            /* [out] */ WORD *pwStreamNumbers,
            /* [out][in] */ WORD *pwNumStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseData( 
            /* [in] */ IMFMediaBuffer *pIBuffer,
            /* [in] */ DWORD cbBufferOffset,
            /* [in] */ DWORD cbLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextSample( 
            /* [out] */ DWORD *pdwStatusFlags,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ IMFSample **ppISample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastSendTime( 
            /* [out] */ DWORD *pdwLastSendTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFSplitterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFSplitter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFSplitter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFSplitter * This);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMFASFSplitter * This,
            /* [in] */ IMFASFContentInfo *pIContentInfo);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IMFASFSplitter * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IMFASFSplitter * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, SelectStreams)
        HRESULT ( STDMETHODCALLTYPE *SelectStreams )( 
            IMFASFSplitter * This,
            /* [in] */ WORD *pwStreamNumbers,
            /* [in] */ WORD wNumStreams);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, GetSelectedStreams)
        HRESULT ( STDMETHODCALLTYPE *GetSelectedStreams )( 
            IMFASFSplitter * This,
            /* [out] */ WORD *pwStreamNumbers,
            /* [out][in] */ WORD *pwNumStreams);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, ParseData)
        HRESULT ( STDMETHODCALLTYPE *ParseData )( 
            IMFASFSplitter * This,
            /* [in] */ IMFMediaBuffer *pIBuffer,
            /* [in] */ DWORD cbBufferOffset,
            /* [in] */ DWORD cbLength);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, GetNextSample)
        HRESULT ( STDMETHODCALLTYPE *GetNextSample )( 
            IMFASFSplitter * This,
            /* [out] */ DWORD *pdwStatusFlags,
            /* [out] */ WORD *pwStreamNumber,
            /* [out] */ IMFSample **ppISample);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFASFSplitter * This);
        
        DECLSPEC_XFGVIRT(IMFASFSplitter, GetLastSendTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastSendTime )( 
            IMFASFSplitter * This,
            /* [out] */ DWORD *pdwLastSendTime);
        
        END_INTERFACE
    } IMFASFSplitterVtbl;

    interface IMFASFSplitter
    {
        CONST_VTBL struct IMFASFSplitterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFSplitter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFSplitter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFSplitter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFSplitter_Initialize(This,pIContentInfo)	\
    ( (This)->lpVtbl -> Initialize(This,pIContentInfo) ) 

#define IMFASFSplitter_SetFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags) ) 

#define IMFASFSplitter_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IMFASFSplitter_SelectStreams(This,pwStreamNumbers,wNumStreams)	\
    ( (This)->lpVtbl -> SelectStreams(This,pwStreamNumbers,wNumStreams) ) 

#define IMFASFSplitter_GetSelectedStreams(This,pwStreamNumbers,pwNumStreams)	\
    ( (This)->lpVtbl -> GetSelectedStreams(This,pwStreamNumbers,pwNumStreams) ) 

#define IMFASFSplitter_ParseData(This,pIBuffer,cbBufferOffset,cbLength)	\
    ( (This)->lpVtbl -> ParseData(This,pIBuffer,cbBufferOffset,cbLength) ) 

#define IMFASFSplitter_GetNextSample(This,pdwStatusFlags,pwStreamNumber,ppISample)	\
    ( (This)->lpVtbl -> GetNextSample(This,pdwStatusFlags,pwStreamNumber,ppISample) ) 

#define IMFASFSplitter_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IMFASFSplitter_GetLastSendTime(This,pdwLastSendTime)	\
    ( (This)->lpVtbl -> GetLastSendTime(This,pdwLastSendTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFSplitter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0007 */
/* [local] */ 

STDAPI MFCreateASFSplitter( 
        /* out */     IMFASFSplitter        **ppISplitter);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define MFASF_MIN_HEADER_BYTES ( sizeof( GUID ) + sizeof( QWORD ) )
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_FILE_ID, 0x3de649b4, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_CREATION_TIME, 0x3de649b6, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_PACKETS, 0x3de649b7, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_PLAY_DURATION, 0x3de649b8, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_SEND_DURATION, 0x3de649b9, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_PREROLL, 0x3de649ba, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_FLAGS, 0x3de649bb, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_MIN_PACKET_SIZE, 0x3de649bc, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_MAX_PACKET_SIZE, 0x3de649bd, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID(MF_PD_ASF_FILEPROPERTIES_MAX_BITRATE, 0x3de649be, 0xd76d, 0x4e66, 0x9e, 0xc9, 0x78, 0x12, 0xf, 0xb4, 0xc7, 0xe3);
EXTERN_GUID( CLSID_WMDRMSystemID, 0x8948BB22, 0x11BD, 0x4796, 0x93, 0xE3, 0x97, 0x4D, 0x1B, 0x57, 0x56, 0x78 );
EXTERN_GUID(MF_PD_ASF_CONTENTENCRYPTION_TYPE, 0x8520fe3d, 0x277e, 0x46ea, 0x99, 0xe4, 0xe3, 0xa, 0x86, 0xdb, 0x12, 0xbe);
EXTERN_GUID(MF_PD_ASF_CONTENTENCRYPTION_KEYID, 0x8520fe3e, 0x277e, 0x46ea, 0x99, 0xe4, 0xe3, 0xa, 0x86, 0xdb, 0x12, 0xbe);
EXTERN_GUID(MF_PD_ASF_CONTENTENCRYPTION_SECRET_DATA, 0x8520fe3f, 0x277e, 0x46ea, 0x99, 0xe4, 0xe3, 0xa, 0x86, 0xdb, 0x12, 0xbe);
EXTERN_GUID(MF_PD_ASF_CONTENTENCRYPTION_LICENSE_URL, 0x8520fe40, 0x277e, 0x46ea, 0x99, 0xe4, 0xe3, 0xa, 0x86, 0xdb, 0x12, 0xbe);
EXTERN_GUID( MF_PD_ASF_CONTENTENCRYPTIONEX_ENCRYPTION_DATA, 0x62508be5, 0xecdf, 0x4924, 0xa3, 0x59, 0x72, 0xba, 0xb3, 0x39, 0x7b, 0x9d);
EXTERN_GUID( MF_PD_ASF_LANGLIST, 0xf23de43c, 0x9977, 0x460d, 0xa6, 0xec, 0x32, 0x93, 0x7f, 0x16, 0xf, 0x7d);
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_PD_ASF_LANGLIST_LEGACYORDER, 0xf23de43d, 0x9977, 0x460d, 0xa6, 0xec, 0x32, 0x93, 0x7f, 0x16, 0xf, 0x7d);
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID(MF_PD_ASF_MARKER, 0x5134330e, 0x83a6, 0x475e, 0xa9, 0xd5, 0x4f, 0xb8, 0x75, 0xfb, 0x2e, 0x31);
EXTERN_GUID( MF_PD_ASF_SCRIPT, 0xe29cd0d7, 0xd602, 0x4923, 0xa7, 0xfe, 0x73, 0xfd, 0x97, 0xec, 0xc6, 0x50);
EXTERN_GUID(MF_PD_ASF_CODECLIST, 0xe4bb3509, 0xc18d, 0x4df1, 0xbb, 0x99, 0x7a, 0x36, 0xb3, 0xcc, 0x41, 0x19);
EXTERN_GUID( MF_PD_ASF_METADATA_IS_VBR, 0x5fc6947a, 0xef60, 0x445d, 0xb4, 0x49, 0x44, 0x2e, 0xcc, 0x78, 0xb4, 0xc1);
EXTERN_GUID( MF_PD_ASF_METADATA_V8_VBRPEAK, 0x5fc6947b, 0xef60, 0x445d, 0xb4, 0x49, 0x44, 0x2e, 0xcc, 0x78, 0xb4, 0xc1);
EXTERN_GUID( MF_PD_ASF_METADATA_V8_BUFFERAVERAGE, 0x5fc6947c, 0xef60, 0x445d, 0xb4, 0x49, 0x44, 0x2e, 0xcc, 0x78, 0xb4, 0xc1);
EXTERN_GUID( MF_PD_ASF_METADATA_LEAKY_BUCKET_PAIRS, 0x5fc6947d, 0xef60, 0x445d, 0xb4, 0x49, 0x44, 0x2e, 0xcc, 0x78, 0xb4, 0xc1);
EXTERN_GUID(MF_PD_ASF_DATA_START_OFFSET, 0xe7d5b3e7, 0x1f29, 0x45d3, 0x88, 0x22, 0x3e, 0x78, 0xfa, 0xe2, 0x72, 0xed);
EXTERN_GUID(MF_PD_ASF_DATA_LENGTH, 0xe7d5b3e8, 0x1f29, 0x45d3, 0x88, 0x22, 0x3e, 0x78, 0xfa, 0xe2, 0x72, 0xed);
EXTERN_GUID(MF_SD_ASF_EXTSTRMPROP_LANGUAGE_ID_INDEX, 0x48f8a522, 0x305d, 0x422d, 0x85, 0x24, 0x25, 0x2, 0xdd, 0xa3, 0x36, 0x80);
EXTERN_GUID(MF_SD_ASF_EXTSTRMPROP_AVG_DATA_BITRATE, 0x48f8a523, 0x305d, 0x422d, 0x85, 0x24, 0x25, 0x2, 0xdd, 0xa3, 0x36, 0x80);
EXTERN_GUID(MF_SD_ASF_EXTSTRMPROP_AVG_BUFFERSIZE, 0x48f8a524, 0x305d, 0x422d, 0x85, 0x24, 0x25, 0x2, 0xdd, 0xa3, 0x36, 0x80);
EXTERN_GUID(MF_SD_ASF_EXTSTRMPROP_MAX_DATA_BITRATE, 0x48f8a525, 0x305d, 0x422d, 0x85, 0x24, 0x25, 0x2, 0xdd, 0xa3, 0x36, 0x80);
EXTERN_GUID(MF_SD_ASF_EXTSTRMPROP_MAX_BUFFERSIZE, 0x48f8a526, 0x305d, 0x422d, 0x85, 0x24, 0x25, 0x2, 0xdd, 0xa3, 0x36, 0x80);
EXTERN_GUID(MF_SD_ASF_STREAMBITRATES_BITRATE, 0xa8e182ed, 0xafc8, 0x43d0, 0xb0, 0xd1, 0xf6, 0x5b, 0xad, 0x9d, 0xa5, 0x58);
EXTERN_GUID(MF_SD_ASF_METADATA_DEVICE_CONFORMANCE_TEMPLATE, 0x245e929d, 0xc44e, 0x4f7e, 0xbb, 0x3c, 0x77, 0xd4, 0xdf, 0xd2, 0x7f, 0x8a);
EXTERN_GUID(MF_PD_ASF_INFO_HAS_AUDIO, 0x80e62295, 0x2296, 0x4a44, 0xb3, 0x1c, 0xd1, 0x3, 0xc6, 0xfe, 0xd2, 0x3c);
EXTERN_GUID(MF_PD_ASF_INFO_HAS_VIDEO, 0x80e62296, 0x2296, 0x4a44, 0xb3, 0x1c, 0xd1, 0x3, 0xc6, 0xfe, 0xd2, 0x3c);
EXTERN_GUID(MF_PD_ASF_INFO_HAS_NON_AUDIO_VIDEO, 0x80e62297, 0x2296, 0x4a44, 0xb3, 0x1c, 0xd1, 0x3, 0xc6, 0xfe, 0xd2, 0x3c);
EXTERN_GUID(MF_ASFPROFILE_MINPACKETSIZE, 0x22587626, 0x47de, 0x4168, 0x87, 0xf5, 0xb5, 0xaa, 0x9b, 0x12, 0xa8, 0xf0);
EXTERN_GUID(MF_ASFPROFILE_MAXPACKETSIZE, 0x22587627, 0x47de, 0x4168, 0x87, 0xf5, 0xb5, 0xaa, 0x9b, 0x12, 0xa8, 0xf0);
STDAPI MFCreateASFProfile( 
        /* out */     IMFASFProfile         **ppIProfile);
STDAPI MFCreateASFProfileFromPresentationDescriptor( 
        /* in  */     IMFPresentationDescriptor  *pIPD,
        /* out */     IMFASFProfile             **ppIProfile);
STDAPI MFCreatePresentationDescriptorFromASFProfile( 
        /* in  */     IMFASFProfile              *pIProfile,
        /* out */     IMFPresentationDescriptor **ppIPD);
EXTERN_GUID(MF_ASFSTREAMCONFIG_LEAKYBUCKET1, 0xc69b5901, 0xea1a, 0x4c9b, 0xb6, 0x92, 0xe2, 0xa0, 0xd2, 0x9a, 0x8a, 0xdd);
EXTERN_GUID(MF_ASFSTREAMCONFIG_LEAKYBUCKET2, 0xc69b5902, 0xea1a, 0x4c9b, 0xb6, 0x92, 0xe2, 0xa0, 0xd2, 0x9a, 0x8a, 0xdd);
EXTERN_GUID(MFASFSampleExtension_SampleDuration, 0xc6bd9450, 0x867f, 0x4907, 0x83, 0xa3, 0xc7, 0x79, 0x21, 0xb7, 0x33, 0xad);
EXTERN_GUID(MFASFSampleExtension_OutputCleanPoint, 0xf72a3c6f, 0x6eb4, 0x4ebc, 0xb1, 0x92, 0x9, 0xad, 0x97, 0x59, 0xe8, 0x28);
EXTERN_GUID(MFASFSampleExtension_SMPTE, 0x399595ec, 0x8667, 0x4e2d, 0x8f, 0xdb, 0x98, 0x81, 0x4c, 0xe7, 0x6c, 0x1e);
EXTERN_GUID(MFASFSampleExtension_FileName, 0xe165ec0e, 0x19ed, 0x45d7, 0xb4, 0xa7, 0x25, 0xcb, 0xd1, 0xe2, 0x8e, 0x9b);
EXTERN_GUID(MFASFSampleExtension_ContentType, 0xd590dc20, 0x07bc, 0x436c, 0x9c, 0xf7, 0xf3, 0xbb, 0xfb, 0xf1, 0xa4, 0xdc);
EXTERN_GUID(MFASFSampleExtension_PixelAspectRatio, 0x1b1ee554, 0xf9ea, 0x4bc8, 0x82, 0x1a, 0x37, 0x6b, 0x74, 0xe4, 0xc4, 0xb8);
EXTERN_GUID(MFASFSampleExtension_Encryption_SampleID, 0x6698B84E, 0x0AFA, 0x4330, 0xAE, 0xB2, 0x1C, 0x0A, 0x98, 0xD7, 0xA4, 0x4D);
EXTERN_GUID(MFASFSampleExtension_Encryption_KeyID, 0x76376591,  0x795f,  0x4da1, 0x86, 0xed, 0x9d, 0x46, 0xec, 0xa1, 0x09, 0xa9);
EXTERN_GUID(MFASFMutexType_Language, 0x72178C2B, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
EXTERN_GUID(MFASFMutexType_Bitrate, 0x72178C2C, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
EXTERN_GUID(MFASFMutexType_Presentation, 0x72178C2D, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
EXTERN_GUID(MFASFMutexType_Unknown, 0x72178C2E, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
typedef 
enum MFASF_SPLITTERFLAGS
    {
        MFASF_SPLITTER_REVERSE	= 0x1,
        MFASF_SPLITTER_WMDRM	= 0x2
    } 	MFASF_SPLITTERFLAGS;

typedef 
enum ASF_STATUSFLAGS
    {
        ASF_STATUSFLAGS_INCOMPLETE	= 0x1,
        ASF_STATUSFLAGS_NONFATAL_ERROR	= 0x2
    } 	ASF_STATUSFLAGS;

EXTERN_GUID(MFASFSPLITTER_PACKET_BOUNDARY, 0xfe584a05, 0xe8d6, 0x42e3, 0xb1, 0x76, 0xf1, 0x21, 0x17, 0x5, 0xfb, 0x6f);
typedef 
enum MFASF_MULTIPLEXERFLAGS
    {
        MFASF_MULTIPLEXER_AUTOADJUST_BITRATE	= 0x1
    } 	MFASF_MULTIPLEXERFLAGS;

typedef struct ASF_MUX_STATISTICS
    {
    DWORD cFramesWritten;
    DWORD cFramesDropped;
    } 	ASF_MUX_STATISTICS;



extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0007_v0_0_s_ifspec;

#ifndef __IMFASFMultiplexer_INTERFACE_DEFINED__
#define __IMFASFMultiplexer_INTERFACE_DEFINED__

/* interface IMFASFMultiplexer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFMultiplexer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57BDD80A-9B38-4838-B737-C58F670D7D4F")
    IMFASFMultiplexer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IMFASFContentInfo *pIContentInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessSample( 
            /* [in] */ WORD wStreamNumber,
            /* [in] */ IMFSample *pISample,
            /* [in] */ LONGLONG hnsTimestampAdjust) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextPacket( 
            /* [out] */ DWORD *pdwStatusFlags,
            /* [out] */ IMFSample **ppIPacket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( 
            /* [out][in] */ IMFASFContentInfo *pIContentInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatistics( 
            /* [in] */ WORD wStreamNumber,
            /* [out] */ ASF_MUX_STATISTICS *pMuxStats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncTolerance( 
            /* [in] */ DWORD msSyncTolerance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFMultiplexerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFMultiplexer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFMultiplexer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFMultiplexer * This);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMFASFMultiplexer * This,
            /* [in] */ IMFASFContentInfo *pIContentInfo);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IMFASFMultiplexer * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IMFASFMultiplexer * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, ProcessSample)
        HRESULT ( STDMETHODCALLTYPE *ProcessSample )( 
            IMFASFMultiplexer * This,
            /* [in] */ WORD wStreamNumber,
            /* [in] */ IMFSample *pISample,
            /* [in] */ LONGLONG hnsTimestampAdjust);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, GetNextPacket)
        HRESULT ( STDMETHODCALLTYPE *GetNextPacket )( 
            IMFASFMultiplexer * This,
            /* [out] */ DWORD *pdwStatusFlags,
            /* [out] */ IMFSample **ppIPacket);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMFASFMultiplexer * This);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            IMFASFMultiplexer * This,
            /* [out][in] */ IMFASFContentInfo *pIContentInfo);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            IMFASFMultiplexer * This,
            /* [in] */ WORD wStreamNumber,
            /* [out] */ ASF_MUX_STATISTICS *pMuxStats);
        
        DECLSPEC_XFGVIRT(IMFASFMultiplexer, SetSyncTolerance)
        HRESULT ( STDMETHODCALLTYPE *SetSyncTolerance )( 
            IMFASFMultiplexer * This,
            /* [in] */ DWORD msSyncTolerance);
        
        END_INTERFACE
    } IMFASFMultiplexerVtbl;

    interface IMFASFMultiplexer
    {
        CONST_VTBL struct IMFASFMultiplexerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFMultiplexer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFMultiplexer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFMultiplexer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFMultiplexer_Initialize(This,pIContentInfo)	\
    ( (This)->lpVtbl -> Initialize(This,pIContentInfo) ) 

#define IMFASFMultiplexer_SetFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags) ) 

#define IMFASFMultiplexer_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IMFASFMultiplexer_ProcessSample(This,wStreamNumber,pISample,hnsTimestampAdjust)	\
    ( (This)->lpVtbl -> ProcessSample(This,wStreamNumber,pISample,hnsTimestampAdjust) ) 

#define IMFASFMultiplexer_GetNextPacket(This,pdwStatusFlags,ppIPacket)	\
    ( (This)->lpVtbl -> GetNextPacket(This,pdwStatusFlags,ppIPacket) ) 

#define IMFASFMultiplexer_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IMFASFMultiplexer_End(This,pIContentInfo)	\
    ( (This)->lpVtbl -> End(This,pIContentInfo) ) 

#define IMFASFMultiplexer_GetStatistics(This,wStreamNumber,pMuxStats)	\
    ( (This)->lpVtbl -> GetStatistics(This,wStreamNumber,pMuxStats) ) 

#define IMFASFMultiplexer_SetSyncTolerance(This,msSyncTolerance)	\
    ( (This)->lpVtbl -> SetSyncTolerance(This,msSyncTolerance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFMultiplexer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0008 */
/* [local] */ 

STDAPI MFCreateASFMultiplexer( 
        /* out */     IMFASFMultiplexer     **ppIMultiplexer);
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFASFINDEXER_TYPE_TIMECODE, 0x49815231, 0x6bad, 0x44fd, 0x81, 0xa, 0x3f, 0x60, 0x98, 0x4e, 0xc7, 0xfd );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#define MFASFINDEXER_PER_ENTRY_BYTES_DYNAMIC        0xffff
#define MFASFINDEXER_NO_FIXED_INTERVAL              0xffffffff
#define MFASFINDEXER_READ_FOR_REVERSEPLAYBACK_OUTOFDATASEGMENT        0xffffffffffffffff
#define MFASFINDEXER_APPROX_SEEK_TIME_UNKNOWN 0xffffffffffffffff
typedef 
enum MFASF_STREAMSELECTORFLAGS
    {
        MFASF_STREAMSELECTOR_DISABLE_THINNING	= 0x1,
        MFASF_STREAMSELECTOR_USE_AVERAGE_BITRATE	= 0x2
    } 	MFASF_STREAMSELECTOR_FLAGS;

typedef 
enum ASF_SELECTION_STATUS
    {
        ASF_STATUS_NOTSELECTED	= 0,
        ASF_STATUS_CLEANPOINTSONLY	= 1,
        ASF_STATUS_ALLDATAUNITS	= 2
    } 	ASF_SELECTION_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0008_v0_0_s_ifspec;

#ifndef __IMFASFStreamSelector_INTERFACE_DEFINED__
#define __IMFASFStreamSelector_INTERFACE_DEFINED__

/* interface IMFASFStreamSelector */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFASFStreamSelector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d01bad4a-4fa0-4a60-9349-c27e62da9d41")
    IMFASFStreamSelector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [out] */ DWORD *pcStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCount( 
            /* [out] */ DWORD *pcOutputs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamCount( 
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ DWORD *pcStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamNumbers( 
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ WORD *rgwStreamNumbers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputFromStream( 
            /* [in] */ WORD wStreamNum,
            /* [out] */ DWORD *pdwOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputOverride( 
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ ASF_SELECTION_STATUS *pSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputOverride( 
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ ASF_SELECTION_STATUS Selection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputMutexCount( 
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ DWORD *pcMutexes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputMutex( 
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ DWORD dwMutexNum,
            /* [out] */ IUnknown **ppMutex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputMutexSelection( 
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ DWORD dwMutexNum,
            /* [in] */ WORD wSelectedRecord) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBandwidthStepCount( 
            /* [out] */ DWORD *pcStepCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBandwidthStep( 
            /* [in] */ DWORD dwStepNum,
            /* [out] */ DWORD *pdwBitrate,
            /* [out] */ WORD *rgwStreamNumbers,
            /* [out] */ ASF_SELECTION_STATUS *rgSelections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BitrateToStepNumber( 
            /* [in] */ DWORD dwBitrate,
            /* [out] */ DWORD *pdwStepNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamSelectorFlags( 
            /* [in] */ DWORD dwStreamSelectorFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFASFStreamSelectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFASFStreamSelector * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFASFStreamSelector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFASFStreamSelector * This);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFASFStreamSelector * This,
            /* [out] */ DWORD *pcStreams);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputCount)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCount )( 
            IMFASFStreamSelector * This,
            /* [out] */ DWORD *pcOutputs);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamCount )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ DWORD *pcStreams);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputStreamNumbers)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamNumbers )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ WORD *rgwStreamNumbers);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputFromStream)
        HRESULT ( STDMETHODCALLTYPE *GetOutputFromStream )( 
            IMFASFStreamSelector * This,
            /* [in] */ WORD wStreamNum,
            /* [out] */ DWORD *pdwOutput);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputOverride)
        HRESULT ( STDMETHODCALLTYPE *GetOutputOverride )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ ASF_SELECTION_STATUS *pSelection);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, SetOutputOverride)
        HRESULT ( STDMETHODCALLTYPE *SetOutputOverride )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ ASF_SELECTION_STATUS Selection);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputMutexCount)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMutexCount )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [out] */ DWORD *pcMutexes);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetOutputMutex)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMutex )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ DWORD dwMutexNum,
            /* [out] */ IUnknown **ppMutex);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, SetOutputMutexSelection)
        HRESULT ( STDMETHODCALLTYPE *SetOutputMutexSelection )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwOutputNum,
            /* [in] */ DWORD dwMutexNum,
            /* [in] */ WORD wSelectedRecord);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetBandwidthStepCount)
        HRESULT ( STDMETHODCALLTYPE *GetBandwidthStepCount )( 
            IMFASFStreamSelector * This,
            /* [out] */ DWORD *pcStepCount);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, GetBandwidthStep)
        HRESULT ( STDMETHODCALLTYPE *GetBandwidthStep )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwStepNum,
            /* [out] */ DWORD *pdwBitrate,
            /* [out] */ WORD *rgwStreamNumbers,
            /* [out] */ ASF_SELECTION_STATUS *rgSelections);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, BitrateToStepNumber)
        HRESULT ( STDMETHODCALLTYPE *BitrateToStepNumber )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwBitrate,
            /* [out] */ DWORD *pdwStepNum);
        
        DECLSPEC_XFGVIRT(IMFASFStreamSelector, SetStreamSelectorFlags)
        HRESULT ( STDMETHODCALLTYPE *SetStreamSelectorFlags )( 
            IMFASFStreamSelector * This,
            /* [in] */ DWORD dwStreamSelectorFlags);
        
        END_INTERFACE
    } IMFASFStreamSelectorVtbl;

    interface IMFASFStreamSelector
    {
        CONST_VTBL struct IMFASFStreamSelectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFASFStreamSelector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFASFStreamSelector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFASFStreamSelector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFASFStreamSelector_GetStreamCount(This,pcStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcStreams) ) 

#define IMFASFStreamSelector_GetOutputCount(This,pcOutputs)	\
    ( (This)->lpVtbl -> GetOutputCount(This,pcOutputs) ) 

#define IMFASFStreamSelector_GetOutputStreamCount(This,dwOutputNum,pcStreams)	\
    ( (This)->lpVtbl -> GetOutputStreamCount(This,dwOutputNum,pcStreams) ) 

#define IMFASFStreamSelector_GetOutputStreamNumbers(This,dwOutputNum,rgwStreamNumbers)	\
    ( (This)->lpVtbl -> GetOutputStreamNumbers(This,dwOutputNum,rgwStreamNumbers) ) 

#define IMFASFStreamSelector_GetOutputFromStream(This,wStreamNum,pdwOutput)	\
    ( (This)->lpVtbl -> GetOutputFromStream(This,wStreamNum,pdwOutput) ) 

#define IMFASFStreamSelector_GetOutputOverride(This,dwOutputNum,pSelection)	\
    ( (This)->lpVtbl -> GetOutputOverride(This,dwOutputNum,pSelection) ) 

#define IMFASFStreamSelector_SetOutputOverride(This,dwOutputNum,Selection)	\
    ( (This)->lpVtbl -> SetOutputOverride(This,dwOutputNum,Selection) ) 

#define IMFASFStreamSelector_GetOutputMutexCount(This,dwOutputNum,pcMutexes)	\
    ( (This)->lpVtbl -> GetOutputMutexCount(This,dwOutputNum,pcMutexes) ) 

#define IMFASFStreamSelector_GetOutputMutex(This,dwOutputNum,dwMutexNum,ppMutex)	\
    ( (This)->lpVtbl -> GetOutputMutex(This,dwOutputNum,dwMutexNum,ppMutex) ) 

#define IMFASFStreamSelector_SetOutputMutexSelection(This,dwOutputNum,dwMutexNum,wSelectedRecord)	\
    ( (This)->lpVtbl -> SetOutputMutexSelection(This,dwOutputNum,dwMutexNum,wSelectedRecord) ) 

#define IMFASFStreamSelector_GetBandwidthStepCount(This,pcStepCount)	\
    ( (This)->lpVtbl -> GetBandwidthStepCount(This,pcStepCount) ) 

#define IMFASFStreamSelector_GetBandwidthStep(This,dwStepNum,pdwBitrate,rgwStreamNumbers,rgSelections)	\
    ( (This)->lpVtbl -> GetBandwidthStep(This,dwStepNum,pdwBitrate,rgwStreamNumbers,rgSelections) ) 

#define IMFASFStreamSelector_BitrateToStepNumber(This,dwBitrate,pdwStepNum)	\
    ( (This)->lpVtbl -> BitrateToStepNumber(This,dwBitrate,pdwStepNum) ) 

#define IMFASFStreamSelector_SetStreamSelectorFlags(This,dwStreamSelectorFlags)	\
    ( (This)->lpVtbl -> SetStreamSelectorFlags(This,dwStreamSelectorFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFASFStreamSelector_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0009 */
/* [local] */ 

STDAPI MFCreateASFStreamSelector( 
    /* in */ IMFASFProfile *pIASFProfile, 
    /* out */ IMFASFStreamSelector **ppSelector );
STDAPI
MFCreateASFMediaSink(
    IMFByteStream *pIByteStream,
    IMFMediaSink** ppIMediaSink
    );
STDAPI MFCreateASFMediaSinkActivate(
    LPCWSTR pwszFileName,
    IMFASFContentInfo *pContentInfo,
    IMFActivate** ppIActivate
    );
HRESULT STDMETHODCALLTYPE
MFCreateWMVEncoderActivate(
    IMFMediaType * pMediaType, 
    IPropertyStore * pEncodingConfigurationProperties, 
    IMFActivate ** ppActivate 
    );
HRESULT STDMETHODCALLTYPE
MFCreateWMAEncoderActivate(
    IMFMediaType * pMediaType, 
    IPropertyStore * pEncodingConfigurationProperties, 
    IMFActivate ** ppActivate 
    );
STDAPI
MFCreateASFStreamingMediaSink(
    IMFByteStream *pIByteStream,
    IMFMediaSink** ppIMediaSink
    );
STDAPI MFCreateASFStreamingMediaSinkActivate(
    IMFActivate *pByteStreamActivate,
    IMFASFContentInfo *pContentInfo,
    IMFActivate** ppIActivate
    );
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMEDIASINK_BASE_SENDTIME = { { 0xcddcbc82, 0x3411, 0x4119, 0x91, 0x35, 0x84, 0x23, 0xc4, 0x1b, 0x39, 0x57}, 3  }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMEDIASINK_AUTOADJUST_BITRATE = { { 0xcddcbc82, 0x3411, 0x4119, 0x91, 0x35, 0x84, 0x23, 0xc4, 0x1b, 0x39, 0x57}, 4  }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMEDIASINK_DRMACTION = { { 0xa1db6f6c, 0x1d0a, 0x4cb6, 0x82, 0x54, 0xcb, 0x36, 0xbe, 0xed, 0xbc, 0x48}, 5  }; 
typedef 
enum _MFSINK_WMDRMACTION
    {
        MFSINK_WMDRMACTION_UNDEFINED	= 0,
        MFSINK_WMDRMACTION_ENCODE	= 1,
        MFSINK_WMDRMACTION_TRANSCODE	= 2,
        MFSINK_WMDRMACTION_TRANSCRYPT	= 3,
        MFSINK_WMDRMACTION_LAST	= 3
    } 	MFSINK_WMDRMACTION;

EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFSTREAMSINK_CORRECTED_LEAKYBUCKET = { { 0xa2f152fb, 0x8ad9, 0x4a11, 0xb3, 0x45, 0x2c, 0xe2, 0xfa, 0xd8, 0x72, 0x3d}, 1  }; 
// 
// Define WMContainer constants 
// 
#define  MFASF_MAX_STREAM_NUMBER       127
#define  MFASF_INVALID_STREAM_NUMBER  ( MFASF_MAX_STREAM_NUMBER + 1 )
#define  MFASF_PAYLOADEXTENSION_MAX_SIZE        0xff
#define  MFASF_PAYLOADEXTENSION_VARIABLE_SIZE   0xffff
#define  MFASF_DEFAULT_BUFFER_WINDOW_MS 		3000
// 
#if (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0009_v0_0_s_ifspec;

#ifndef __IMFDRMNetHelper_INTERFACE_DEFINED__
#define __IMFDRMNetHelper_INTERFACE_DEFINED__

/* interface IMFDRMNetHelper */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFDRMNetHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D1FF0EA-679A-4190-8D46-7FA69E8C7E15")
    IMFDRMNetHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProcessLicenseRequest( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicenseRequest) BYTE *pLicenseRequest,
            /* [in] */ DWORD cbLicenseRequest,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicenseResponse) BYTE **ppLicenseResponse,
            /* [out] */ __RPC__out DWORD *pcbLicenseResponse,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrKID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChainedLicenseResponse( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicenseResponse) BYTE **ppLicenseResponse,
            /* [out] */ __RPC__out DWORD *pcbLicenseResponse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDRMNetHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFDRMNetHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFDRMNetHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFDRMNetHelper * This);
        
        DECLSPEC_XFGVIRT(IMFDRMNetHelper, ProcessLicenseRequest)
        HRESULT ( STDMETHODCALLTYPE *ProcessLicenseRequest )( 
            __RPC__in IMFDRMNetHelper * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicenseRequest) BYTE *pLicenseRequest,
            /* [in] */ DWORD cbLicenseRequest,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicenseResponse) BYTE **ppLicenseResponse,
            /* [out] */ __RPC__out DWORD *pcbLicenseResponse,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrKID);
        
        DECLSPEC_XFGVIRT(IMFDRMNetHelper, GetChainedLicenseResponse)
        HRESULT ( STDMETHODCALLTYPE *GetChainedLicenseResponse )( 
            __RPC__in IMFDRMNetHelper * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicenseResponse) BYTE **ppLicenseResponse,
            /* [out] */ __RPC__out DWORD *pcbLicenseResponse);
        
        END_INTERFACE
    } IMFDRMNetHelperVtbl;

    interface IMFDRMNetHelper
    {
        CONST_VTBL struct IMFDRMNetHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDRMNetHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDRMNetHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDRMNetHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDRMNetHelper_ProcessLicenseRequest(This,pLicenseRequest,cbLicenseRequest,ppLicenseResponse,pcbLicenseResponse,pbstrKID)	\
    ( (This)->lpVtbl -> ProcessLicenseRequest(This,pLicenseRequest,cbLicenseRequest,ppLicenseResponse,pcbLicenseResponse,pbstrKID) ) 

#define IMFDRMNetHelper_GetChainedLicenseResponse(This,ppLicenseResponse,pcbLicenseResponse)	\
    ( (This)->lpVtbl -> GetChainedLicenseResponse(This,ppLicenseResponse,pcbLicenseResponse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDRMNetHelper_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmcontainer_0000_0010 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmcontainer_0000_0010_v0_0_s_ifspec;

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


