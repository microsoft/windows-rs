

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

#ifndef __spatialaudiometadata_h__
#define __spatialaudiometadata_h__

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

#ifndef __ISpatialAudioMetadataItems_FWD_DEFINED__
#define __ISpatialAudioMetadataItems_FWD_DEFINED__
typedef interface ISpatialAudioMetadataItems ISpatialAudioMetadataItems;

#endif 	/* __ISpatialAudioMetadataItems_FWD_DEFINED__ */


#ifndef __ISpatialAudioMetadataWriter_FWD_DEFINED__
#define __ISpatialAudioMetadataWriter_FWD_DEFINED__
typedef interface ISpatialAudioMetadataWriter ISpatialAudioMetadataWriter;

#endif 	/* __ISpatialAudioMetadataWriter_FWD_DEFINED__ */


#ifndef __ISpatialAudioMetadataReader_FWD_DEFINED__
#define __ISpatialAudioMetadataReader_FWD_DEFINED__
typedef interface ISpatialAudioMetadataReader ISpatialAudioMetadataReader;

#endif 	/* __ISpatialAudioMetadataReader_FWD_DEFINED__ */


#ifndef __ISpatialAudioMetadataCopier_FWD_DEFINED__
#define __ISpatialAudioMetadataCopier_FWD_DEFINED__
typedef interface ISpatialAudioMetadataCopier ISpatialAudioMetadataCopier;

#endif 	/* __ISpatialAudioMetadataCopier_FWD_DEFINED__ */


#ifndef __ISpatialAudioMetadataItemsBuffer_FWD_DEFINED__
#define __ISpatialAudioMetadataItemsBuffer_FWD_DEFINED__
typedef interface ISpatialAudioMetadataItemsBuffer ISpatialAudioMetadataItemsBuffer;

#endif 	/* __ISpatialAudioMetadataItemsBuffer_FWD_DEFINED__ */


#ifndef __ISpatialAudioMetadataClient_FWD_DEFINED__
#define __ISpatialAudioMetadataClient_FWD_DEFINED__
typedef interface ISpatialAudioMetadataClient ISpatialAudioMetadataClient;

#endif 	/* __ISpatialAudioMetadataClient_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectForMetadataCommands_FWD_DEFINED__
#define __ISpatialAudioObjectForMetadataCommands_FWD_DEFINED__
typedef interface ISpatialAudioObjectForMetadataCommands ISpatialAudioObjectForMetadataCommands;

#endif 	/* __ISpatialAudioObjectForMetadataCommands_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectForMetadataItems_FWD_DEFINED__
#define __ISpatialAudioObjectForMetadataItems_FWD_DEFINED__
typedef interface ISpatialAudioObjectForMetadataItems ISpatialAudioObjectForMetadataItems;

#endif 	/* __ISpatialAudioObjectForMetadataItems_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamForMetadata_FWD_DEFINED__
#define __ISpatialAudioObjectRenderStreamForMetadata_FWD_DEFINED__
typedef interface ISpatialAudioObjectRenderStreamForMetadata ISpatialAudioObjectRenderStreamForMetadata;

#endif 	/* __ISpatialAudioObjectRenderStreamForMetadata_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "hstring.h"
#include "propidl.h"
#include "SpatialAudioClient.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_spatialaudiometadata_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
#define SPATIAL_AUDIO_STANDARD_COMMANDS_START   200     // 200 and above is reserved
#define SPATIAL_AUDIO_POSITION                   (SPATIAL_AUDIO_STANDARD_COMMANDS_START)
#define SPATIAL_AUDIO_POSITION_BYTE_COUNT        (sizeof(float) * 3)     // 3 (XYZ) 32-Bit Floats
typedef 
enum SpatialAudioMetadataWriterOverflowMode
    {
        SpatialAudioMetadataWriterOverflow_Fail	= 0,
        SpatialAudioMetadataWriterOverflow_MergeWithNew	= ( SpatialAudioMetadataWriterOverflow_Fail + 1 ) ,
        SpatialAudioMetadataWriterOverflow_MergeWithLast	= ( SpatialAudioMetadataWriterOverflow_MergeWithNew + 1 ) 
    } 	SpatialAudioMetadataWriterOverflowMode;

typedef 
enum SpatialAudioMetadataCopyMode
    {
        SpatialAudioMetadataCopy_Overwrite	= 0,
        SpatialAudioMetadataCopy_Append	= ( SpatialAudioMetadataCopy_Overwrite + 1 ) ,
        SpatialAudioMetadataCopy_AppendMergeWithLast	= ( SpatialAudioMetadataCopy_Append + 1 ) ,
        SpatialAudioMetadataCopy_AppendMergeWithFirst	= ( SpatialAudioMetadataCopy_AppendMergeWithLast + 1 ) 
    } 	SpatialAudioMetadataCopyMode;


#pragma pack(push, 1)
typedef struct SpatialAudioMetadataItemsInfo
    {
    UINT16 FrameCount;
    UINT16 ItemCount;
    UINT16 MaxItemCount;
    UINT32 MaxValueBufferLength;
    } 	SpatialAudioMetadataItemsInfo;

typedef struct SpatialAudioObjectRenderStreamForMetadataActivationParams
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    GUID MetadataFormatId;
    UINT16 MaxMetadataItemCount;
    const PROPVARIANT *MetadataActivationParams;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    } 	SpatialAudioObjectRenderStreamForMetadataActivationParams;

typedef struct SpatialAudioObjectRenderStreamForMetadataActivationParams2
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    GUID MetadataFormatId;
    UINT32 MaxMetadataItemCount;
    const PROPVARIANT *MetadataActivationParams;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    SPATIAL_AUDIO_STREAM_OPTIONS Options;
    } 	SpatialAudioObjectRenderStreamForMetadataActivationParams2;


#pragma pack(pop)


extern RPC_IF_HANDLE __MIDL_itf_spatialaudiometadata_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudiometadata_0000_0000_v0_0_s_ifspec;

#ifndef __ISpatialAudioMetadataItems_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataItems_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataItems */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BCD7C78F-3098-4F22-B547-A2F25A381269")
    ISpatialAudioMetadataItems : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFrameCount( 
            /* [annotation][out] */ 
            _Out_  UINT16 *frameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemCount( 
            /* [annotation][out] */ 
            _Out_  UINT16 *itemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxItemCount( 
            /* [annotation][out] */ 
            _Out_  UINT16 *maxItemCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxValueBufferLength( 
            /* [annotation][out] */ 
            _Out_  UINT32 *maxValueBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [annotation][out] */ 
            _Out_  SpatialAudioMetadataItemsInfo *info) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataItems * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataItems * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItems, GetFrameCount)
        HRESULT ( STDMETHODCALLTYPE *GetFrameCount )( 
            ISpatialAudioMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  UINT16 *frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItems, GetItemCount)
        HRESULT ( STDMETHODCALLTYPE *GetItemCount )( 
            ISpatialAudioMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  UINT16 *itemCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItems, GetMaxItemCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxItemCount )( 
            ISpatialAudioMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  UINT16 *maxItemCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItems, GetMaxValueBufferLength)
        HRESULT ( STDMETHODCALLTYPE *GetMaxValueBufferLength )( 
            ISpatialAudioMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *maxValueBufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItems, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            ISpatialAudioMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  SpatialAudioMetadataItemsInfo *info);
        
        END_INTERFACE
    } ISpatialAudioMetadataItemsVtbl;

    interface ISpatialAudioMetadataItems
    {
        CONST_VTBL struct ISpatialAudioMetadataItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataItems_GetFrameCount(This,frameCount)	\
    ( (This)->lpVtbl -> GetFrameCount(This,frameCount) ) 

#define ISpatialAudioMetadataItems_GetItemCount(This,itemCount)	\
    ( (This)->lpVtbl -> GetItemCount(This,itemCount) ) 

#define ISpatialAudioMetadataItems_GetMaxItemCount(This,maxItemCount)	\
    ( (This)->lpVtbl -> GetMaxItemCount(This,maxItemCount) ) 

#define ISpatialAudioMetadataItems_GetMaxValueBufferLength(This,maxValueBufferLength)	\
    ( (This)->lpVtbl -> GetMaxValueBufferLength(This,maxValueBufferLength) ) 

#define ISpatialAudioMetadataItems_GetInfo(This,info)	\
    ( (This)->lpVtbl -> GetInfo(This,info) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataItems_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioMetadataWriter_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataWriter_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataWriter */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataWriter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1B17CA01-2955-444D-A430-537DC589A844")
    ISpatialAudioMetadataWriter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNextItem( 
            /* [annotation][in] */ 
            _In_  UINT16 frameOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNextItemCommand( 
            /* [annotation][in] */ 
            _In_  BYTE commandID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_opt_(valueBufferLength)  const void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 valueBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataWriterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataWriter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataWriter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataWriter * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataWriter, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            ISpatialAudioMetadataWriter * This,
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataWriter, WriteNextItem)
        HRESULT ( STDMETHODCALLTYPE *WriteNextItem )( 
            ISpatialAudioMetadataWriter * This,
            /* [annotation][in] */ 
            _In_  UINT16 frameOffset);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataWriter, WriteNextItemCommand)
        HRESULT ( STDMETHODCALLTYPE *WriteNextItemCommand )( 
            ISpatialAudioMetadataWriter * This,
            /* [annotation][in] */ 
            _In_  BYTE commandID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_opt_(valueBufferLength)  const void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 valueBufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataWriter, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            ISpatialAudioMetadataWriter * This);
        
        END_INTERFACE
    } ISpatialAudioMetadataWriterVtbl;

    interface ISpatialAudioMetadataWriter
    {
        CONST_VTBL struct ISpatialAudioMetadataWriterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataWriter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataWriter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataWriter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataWriter_Open(This,metadataItems)	\
    ( (This)->lpVtbl -> Open(This,metadataItems) ) 

#define ISpatialAudioMetadataWriter_WriteNextItem(This,frameOffset)	\
    ( (This)->lpVtbl -> WriteNextItem(This,frameOffset) ) 

#define ISpatialAudioMetadataWriter_WriteNextItemCommand(This,commandID,valueBuffer,valueBufferLength)	\
    ( (This)->lpVtbl -> WriteNextItemCommand(This,commandID,valueBuffer,valueBufferLength) ) 

#define ISpatialAudioMetadataWriter_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataWriter_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioMetadataReader_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataReader_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataReader */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B78E86A2-31D9-4C32-94D2-7DF40FC7EBEC")
    ISpatialAudioMetadataReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadNextItem( 
            /* [annotation][out] */ 
            _Out_  UINT8 *commandCount,
            /* [annotation][out] */ 
            _Out_  UINT16 *frameOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadNextItemCommand( 
            /* [annotation][out] */ 
            _Out_  BYTE *commandID,
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(maxValueBufferLength)  void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 maxValueBufferLength,
            /* [annotation][out] */ 
            _Out_  UINT32 *valueBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataReader * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataReader, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            ISpatialAudioMetadataReader * This,
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataReader, ReadNextItem)
        HRESULT ( STDMETHODCALLTYPE *ReadNextItem )( 
            ISpatialAudioMetadataReader * This,
            /* [annotation][out] */ 
            _Out_  UINT8 *commandCount,
            /* [annotation][out] */ 
            _Out_  UINT16 *frameOffset);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataReader, ReadNextItemCommand)
        HRESULT ( STDMETHODCALLTYPE *ReadNextItemCommand )( 
            ISpatialAudioMetadataReader * This,
            /* [annotation][out] */ 
            _Out_  BYTE *commandID,
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(maxValueBufferLength)  void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 maxValueBufferLength,
            /* [annotation][out] */ 
            _Out_  UINT32 *valueBufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataReader, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            ISpatialAudioMetadataReader * This);
        
        END_INTERFACE
    } ISpatialAudioMetadataReaderVtbl;

    interface ISpatialAudioMetadataReader
    {
        CONST_VTBL struct ISpatialAudioMetadataReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataReader_Open(This,metadataItems)	\
    ( (This)->lpVtbl -> Open(This,metadataItems) ) 

#define ISpatialAudioMetadataReader_ReadNextItem(This,commandCount,frameOffset)	\
    ( (This)->lpVtbl -> ReadNextItem(This,commandCount,frameOffset) ) 

#define ISpatialAudioMetadataReader_ReadNextItemCommand(This,commandID,valueBuffer,maxValueBufferLength,valueBufferLength)	\
    ( (This)->lpVtbl -> ReadNextItemCommand(This,commandID,valueBuffer,maxValueBufferLength,valueBufferLength) ) 

#define ISpatialAudioMetadataReader_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataReader_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioMetadataCopier_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataCopier_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataCopier */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataCopier;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D224B233-E251-4FD0-9CA2-D5ECF9A68404")
    ISpatialAudioMetadataCopier : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyMetadataForFrames( 
            /* [annotation][in] */ 
            _In_  UINT16 copyFrameCount,
            /* [annotation][in] */ 
            _In_  SpatialAudioMetadataCopyMode copyMode,
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *dstMetadataItems,
            /* [annotation][out] */ 
            _Out_  UINT16 *itemsCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataCopierVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataCopier * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataCopier * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataCopier * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataCopier, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            ISpatialAudioMetadataCopier * This,
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *metadataItems);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataCopier, CopyMetadataForFrames)
        HRESULT ( STDMETHODCALLTYPE *CopyMetadataForFrames )( 
            ISpatialAudioMetadataCopier * This,
            /* [annotation][in] */ 
            _In_  UINT16 copyFrameCount,
            /* [annotation][in] */ 
            _In_  SpatialAudioMetadataCopyMode copyMode,
            /* [annotation][in] */ 
            _In_  ISpatialAudioMetadataItems *dstMetadataItems,
            /* [annotation][out] */ 
            _Out_  UINT16 *itemsCopied);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataCopier, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            ISpatialAudioMetadataCopier * This);
        
        END_INTERFACE
    } ISpatialAudioMetadataCopierVtbl;

    interface ISpatialAudioMetadataCopier
    {
        CONST_VTBL struct ISpatialAudioMetadataCopierVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataCopier_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataCopier_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataCopier_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataCopier_Open(This,metadataItems)	\
    ( (This)->lpVtbl -> Open(This,metadataItems) ) 

#define ISpatialAudioMetadataCopier_CopyMetadataForFrames(This,copyFrameCount,copyMode,dstMetadataItems,itemsCopied)	\
    ( (This)->lpVtbl -> CopyMetadataForFrames(This,copyFrameCount,copyMode,dstMetadataItems,itemsCopied) ) 

#define ISpatialAudioMetadataCopier_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataCopier_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioMetadataItemsBuffer_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataItemsBuffer_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataItemsBuffer */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataItemsBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42640A16-E1BD-42D9-9FF6-031AB71A2DBA")
    ISpatialAudioMetadataItemsBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AttachToBuffer( 
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(bufferLength)  BYTE *buffer,
            /* [annotation][in] */ 
            _In_  UINT32 bufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AttachToPopulatedBuffer( 
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(bufferLength)  BYTE *buffer,
            /* [annotation][in] */ 
            _In_  UINT32 bufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetachBuffer( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataItemsBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataItemsBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataItemsBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataItemsBuffer * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItemsBuffer, AttachToBuffer)
        HRESULT ( STDMETHODCALLTYPE *AttachToBuffer )( 
            ISpatialAudioMetadataItemsBuffer * This,
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(bufferLength)  BYTE *buffer,
            /* [annotation][in] */ 
            _In_  UINT32 bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItemsBuffer, AttachToPopulatedBuffer)
        HRESULT ( STDMETHODCALLTYPE *AttachToPopulatedBuffer )( 
            ISpatialAudioMetadataItemsBuffer * This,
            /* [annotation][size_is][in] */ 
            _Out_writes_bytes_(bufferLength)  BYTE *buffer,
            /* [annotation][in] */ 
            _In_  UINT32 bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataItemsBuffer, DetachBuffer)
        HRESULT ( STDMETHODCALLTYPE *DetachBuffer )( 
            ISpatialAudioMetadataItemsBuffer * This);
        
        END_INTERFACE
    } ISpatialAudioMetadataItemsBufferVtbl;

    interface ISpatialAudioMetadataItemsBuffer
    {
        CONST_VTBL struct ISpatialAudioMetadataItemsBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataItemsBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataItemsBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataItemsBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataItemsBuffer_AttachToBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> AttachToBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioMetadataItemsBuffer_AttachToPopulatedBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> AttachToPopulatedBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioMetadataItemsBuffer_DetachBuffer(This)	\
    ( (This)->lpVtbl -> DetachBuffer(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataItemsBuffer_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioMetadataClient_INTERFACE_DEFINED__
#define __ISpatialAudioMetadataClient_INTERFACE_DEFINED__

/* interface ISpatialAudioMetadataClient */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioMetadataClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("777D4A3B-F6FF-4A26-85DC-68D7CDEDA1D4")
    ISpatialAudioMetadataClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioMetadataItems( 
            /* [annotation][in] */ 
            _In_  UINT16 maxItemCount,
            /* [annotation][in] */ 
            _In_  UINT16 frameCount,
            /* [annotation][out] */ 
            _Outptr_opt_result_nullonfailure_  ISpatialAudioMetadataItemsBuffer **metadataItemsBuffer,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataItems **metadataItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpatialAudioMetadataItemsBufferLength( 
            /* [annotation][in] */ 
            _In_  UINT16 maxItemCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioMetadataWriter( 
            /* [annotation][in] */ 
            _In_  SpatialAudioMetadataWriterOverflowMode overflowMode,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataWriter **metadataWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioMetadataCopier( 
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataCopier **metadataCopier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioMetadataReader( 
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataReader **metadataReader) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioMetadataClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioMetadataClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioMetadataClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioMetadataClient * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataClient, ActivateSpatialAudioMetadataItems)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioMetadataItems )( 
            ISpatialAudioMetadataClient * This,
            /* [annotation][in] */ 
            _In_  UINT16 maxItemCount,
            /* [annotation][in] */ 
            _In_  UINT16 frameCount,
            /* [annotation][out] */ 
            _Outptr_opt_result_nullonfailure_  ISpatialAudioMetadataItemsBuffer **metadataItemsBuffer,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataItems **metadataItems);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataClient, GetSpatialAudioMetadataItemsBufferLength)
        HRESULT ( STDMETHODCALLTYPE *GetSpatialAudioMetadataItemsBufferLength )( 
            ISpatialAudioMetadataClient * This,
            /* [annotation][in] */ 
            _In_  UINT16 maxItemCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataClient, ActivateSpatialAudioMetadataWriter)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioMetadataWriter )( 
            ISpatialAudioMetadataClient * This,
            /* [annotation][in] */ 
            _In_  SpatialAudioMetadataWriterOverflowMode overflowMode,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataWriter **metadataWriter);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataClient, ActivateSpatialAudioMetadataCopier)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioMetadataCopier )( 
            ISpatialAudioMetadataClient * This,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataCopier **metadataCopier);
        
        DECLSPEC_XFGVIRT(ISpatialAudioMetadataClient, ActivateSpatialAudioMetadataReader)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioMetadataReader )( 
            ISpatialAudioMetadataClient * This,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataReader **metadataReader);
        
        END_INTERFACE
    } ISpatialAudioMetadataClientVtbl;

    interface ISpatialAudioMetadataClient
    {
        CONST_VTBL struct ISpatialAudioMetadataClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioMetadataClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioMetadataClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioMetadataClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioMetadataClient_ActivateSpatialAudioMetadataItems(This,maxItemCount,frameCount,metadataItemsBuffer,metadataItems)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioMetadataItems(This,maxItemCount,frameCount,metadataItemsBuffer,metadataItems) ) 

#define ISpatialAudioMetadataClient_GetSpatialAudioMetadataItemsBufferLength(This,maxItemCount,bufferLength)	\
    ( (This)->lpVtbl -> GetSpatialAudioMetadataItemsBufferLength(This,maxItemCount,bufferLength) ) 

#define ISpatialAudioMetadataClient_ActivateSpatialAudioMetadataWriter(This,overflowMode,metadataWriter)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioMetadataWriter(This,overflowMode,metadataWriter) ) 

#define ISpatialAudioMetadataClient_ActivateSpatialAudioMetadataCopier(This,metadataCopier)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioMetadataCopier(This,metadataCopier) ) 

#define ISpatialAudioMetadataClient_ActivateSpatialAudioMetadataReader(This,metadataReader)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioMetadataReader(This,metadataReader) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioMetadataClient_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectForMetadataCommands_INTERFACE_DEFINED__
#define __ISpatialAudioObjectForMetadataCommands_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectForMetadataCommands */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectForMetadataCommands;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0DF2C94B-F5F9-472D-AF6B-C46E0AC9CD05")
    ISpatialAudioObjectForMetadataCommands : public ISpatialAudioObjectBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WriteNextMetadataCommand( 
            /* [annotation][in] */ 
            _In_  BYTE commandID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_opt_(valueBufferLength)  void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 valueBufferLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectForMetadataCommandsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectForMetadataCommands * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectForMetadataCommands * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [annotation][in] */ 
            _In_  UINT32 frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, IsActive)
        HRESULT ( STDMETHODCALLTYPE *IsActive )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [annotation][out] */ 
            _Out_  BOOL *isActive);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetAudioObjectType)
        HRESULT ( STDMETHODCALLTYPE *GetAudioObjectType )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForMetadataCommands, WriteNextMetadataCommand)
        HRESULT ( STDMETHODCALLTYPE *WriteNextMetadataCommand )( 
            ISpatialAudioObjectForMetadataCommands * This,
            /* [annotation][in] */ 
            _In_  BYTE commandID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_opt_(valueBufferLength)  void *valueBuffer,
            /* [annotation][in] */ 
            _In_  UINT32 valueBufferLength);
        
        END_INTERFACE
    } ISpatialAudioObjectForMetadataCommandsVtbl;

    interface ISpatialAudioObjectForMetadataCommands
    {
        CONST_VTBL struct ISpatialAudioObjectForMetadataCommandsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectForMetadataCommands_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectForMetadataCommands_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectForMetadataCommands_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectForMetadataCommands_GetBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> GetBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioObjectForMetadataCommands_SetEndOfStream(This,frameCount)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,frameCount) ) 

#define ISpatialAudioObjectForMetadataCommands_IsActive(This,isActive)	\
    ( (This)->lpVtbl -> IsActive(This,isActive) ) 

#define ISpatialAudioObjectForMetadataCommands_GetAudioObjectType(This,audioObjectType)	\
    ( (This)->lpVtbl -> GetAudioObjectType(This,audioObjectType) ) 


#define ISpatialAudioObjectForMetadataCommands_WriteNextMetadataCommand(This,commandID,valueBuffer,valueBufferLength)	\
    ( (This)->lpVtbl -> WriteNextMetadataCommand(This,commandID,valueBuffer,valueBufferLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectForMetadataCommands_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectForMetadataItems_INTERFACE_DEFINED__
#define __ISpatialAudioObjectForMetadataItems_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectForMetadataItems */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectForMetadataItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDEA49FF-3BC0-4377-8AAD-9FBCFD808566")
    ISpatialAudioObjectForMetadataItems : public ISpatialAudioObjectBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSpatialAudioMetadataItems( 
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataItems **metadataItems) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectForMetadataItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectForMetadataItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectForMetadataItems * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [annotation][in] */ 
            _In_  UINT32 frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, IsActive)
        HRESULT ( STDMETHODCALLTYPE *IsActive )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  BOOL *isActive);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetAudioObjectType)
        HRESULT ( STDMETHODCALLTYPE *GetAudioObjectType )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForMetadataItems, GetSpatialAudioMetadataItems)
        HRESULT ( STDMETHODCALLTYPE *GetSpatialAudioMetadataItems )( 
            ISpatialAudioObjectForMetadataItems * This,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  ISpatialAudioMetadataItems **metadataItems);
        
        END_INTERFACE
    } ISpatialAudioObjectForMetadataItemsVtbl;

    interface ISpatialAudioObjectForMetadataItems
    {
        CONST_VTBL struct ISpatialAudioObjectForMetadataItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectForMetadataItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectForMetadataItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectForMetadataItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectForMetadataItems_GetBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> GetBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioObjectForMetadataItems_SetEndOfStream(This,frameCount)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,frameCount) ) 

#define ISpatialAudioObjectForMetadataItems_IsActive(This,isActive)	\
    ( (This)->lpVtbl -> IsActive(This,isActive) ) 

#define ISpatialAudioObjectForMetadataItems_GetAudioObjectType(This,audioObjectType)	\
    ( (This)->lpVtbl -> GetAudioObjectType(This,audioObjectType) ) 


#define ISpatialAudioObjectForMetadataItems_GetSpatialAudioMetadataItems(This,metadataItems)	\
    ( (This)->lpVtbl -> GetSpatialAudioMetadataItems(This,metadataItems) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectForMetadataItems_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamForMetadata_INTERFACE_DEFINED__
#define __ISpatialAudioObjectRenderStreamForMetadata_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectRenderStreamForMetadata */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectRenderStreamForMetadata;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BBC9C907-48D5-4A2E-A0C7-F7F0D67C1FB1")
    ISpatialAudioObjectRenderStreamForMetadata : public ISpatialAudioObjectRenderStreamBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioObjectForMetadataCommands( 
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForMetadataCommands **audioObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioObjectForMetadataItems( 
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForMetadataItems **audioObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectRenderStreamForMetadataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetAvailableDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableDynamicObjectCount )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **service);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, BeginUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *availableDynamicObjectCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, EndUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *EndUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamForMetadata * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamForMetadata, ActivateSpatialAudioObjectForMetadataCommands)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioObjectForMetadataCommands )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForMetadataCommands **audioObject);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamForMetadata, ActivateSpatialAudioObjectForMetadataItems)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioObjectForMetadataItems )( 
            ISpatialAudioObjectRenderStreamForMetadata * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForMetadataItems **audioObject);
        
        END_INTERFACE
    } ISpatialAudioObjectRenderStreamForMetadataVtbl;

    interface ISpatialAudioObjectRenderStreamForMetadata
    {
        CONST_VTBL struct ISpatialAudioObjectRenderStreamForMetadataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectRenderStreamForMetadata_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectRenderStreamForMetadata_GetAvailableDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetAvailableDynamicObjectCount(This,value) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_GetService(This,riid,service)	\
    ( (This)->lpVtbl -> GetService(This,riid,service) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_EndUpdatingAudioObjects(This)	\
    ( (This)->lpVtbl -> EndUpdatingAudioObjects(This) ) 


#define ISpatialAudioObjectRenderStreamForMetadata_ActivateSpatialAudioObjectForMetadataCommands(This,type,audioObject)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioObjectForMetadataCommands(This,type,audioObject) ) 

#define ISpatialAudioObjectRenderStreamForMetadata_ActivateSpatialAudioObjectForMetadataItems(This,type,audioObject)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioObjectForMetadataItems(This,type,audioObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectRenderStreamForMetadata_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spatialaudiometadata_0000_0009 */
/* [local] */ 

#define SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND                   AUDCLNT_ERR(0x0200)
#define SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED              AUDCLNT_ERR(0x0201)
#define SPTLAUD_MD_CLNT_E_INVALID_ARGS                        AUDCLNT_ERR(0x0202)
#define SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND           AUDCLNT_ERR(0x0203)
#define SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE         AUDCLNT_ERR(0x0204)
#define SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS                       AUDCLNT_ERR(0x0205)
#define SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS                    AUDCLNT_ERR(0x0206)
#define SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED             AUDCLNT_ERR(0x0207)
#define SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED                 AUDCLNT_ERR(0x0208)
#define SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE             AUDCLNT_ERR(0x0209)
#define SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND                      AUDCLNT_ERR(0x0210)
#define SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW                  AUDCLNT_ERR(0x0211)
#define SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN                       AUDCLNT_ERR(0x0212)
#define SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN                  AUDCLNT_ERR(0x0213)
#define SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER       AUDCLNT_ERR(0x0214)
#define SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER       AUDCLNT_ERR(0x0215)
#define SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED                  AUDCLNT_ERR(0x0216)
#define SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS                       AUDCLNT_ERR(0x0217)
#define SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE            AUDCLNT_ERR(0x0218)
#define SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS             AUDCLNT_ERR(0x0219)
#define SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN               AUDCLNT_ERR(0x0220)
#define SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN                    AUDCLNT_ERR(0x0221)
#define SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN             AUDCLNT_ERR(0x0222)
#define SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH                     AUDCLNT_ERR(0x0223)
#define SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED               AUDCLNT_ERR(0x0224)
#define SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING            AUDCLNT_ERR(0x0225)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */


extern RPC_IF_HANDLE __MIDL_itf_spatialaudiometadata_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudiometadata_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


