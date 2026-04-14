/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    midles.h

Abstract:

    This module contains definitions needed for encoding/decoding
    support (serializing/deserializing a.k.a. pickling).

--*/

#ifndef __MIDLES_H__
#define __MIDLES_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#include <rpcndr.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 *  Pickling support
 */
typedef enum
{
    MES_ENCODE,
    MES_DECODE,
    MES_ENCODE_NDR64  
} MIDL_ES_CODE;

typedef enum
{
    MES_INCREMENTAL_HANDLE,
    MES_FIXED_BUFFER_HANDLE,
    MES_DYNAMIC_BUFFER_HANDLE
} MIDL_ES_HANDLE_STYLE;


typedef void (__RPC_USER *  MIDL_ES_ALLOC )
                ( IN OUT  void  *   state,
                  OUT     char  **  pbuffer,
                  IN OUT  unsigned int * psize );

typedef void (__RPC_USER *  MIDL_ES_WRITE)
                ( IN OUT  void    * state,
                  IN      char    * buffer,
                  IN      unsigned int  size );

typedef void (__RPC_USER *  MIDL_ES_READ)
                ( IN OUT  void  *   state,
                  OUT     char  **  pbuffer,
                  IN OUT  unsigned int * psize );

typedef handle_t    MIDL_ES_HANDLE;

typedef struct _MIDL_TYPE_PICKLING_INFO
{
    unsigned long   Version;
    unsigned long   Flags;    
    UINT_PTR        Reserved[3];
} MIDL_TYPE_PICKLING_INFO,  * PMIDL_TYPE_PICKLING_INFO;


RPC_STATUS  RPC_ENTRY
MesEncodeIncrementalHandleCreate(
    void          * UserState,
    MIDL_ES_ALLOC   AllocFn,
    MIDL_ES_WRITE   WriteFn,
    handle_t      * pHandle );

RPC_STATUS  RPC_ENTRY
MesDecodeIncrementalHandleCreate(
    void         *  UserState,
    MIDL_ES_READ    ReadFn,
    handle_t     *  pHandle );


RPC_STATUS  RPC_ENTRY
MesIncrementalHandleReset(
    handle_t        Handle,
    void     *      UserState,
    MIDL_ES_ALLOC   AllocFn,
    MIDL_ES_WRITE   WriteFn,
    MIDL_ES_READ    ReadFn,
    MIDL_ES_CODE    Operation );


RPC_STATUS  RPC_ENTRY
MesEncodeFixedBufferHandleCreate(
    _Out_writes_bytes_(BufferSize) char *pBuffer,
    _In_ unsigned long BufferSize,
    _Out_ unsigned long *pEncodedSize,
    _Out_ handle_t *pHandle
    );

RPC_STATUS  RPC_ENTRY
MesEncodeDynBufferHandleCreate(
    _Outptr_result_bytebuffer_(*pEncodedSize) char **pBuffer,
    _Out_ unsigned long *pEncodedSize,
    _Out_ handle_t *pHandle
    );

RPC_STATUS  RPC_ENTRY
MesDecodeBufferHandleCreate(
    _In_reads_bytes_(BufferSize ) char * Buffer,
    _In_ unsigned long   BufferSize,
    _Out_ handle_t  * pHandle
    );


RPC_STATUS  RPC_ENTRY
MesBufferHandleReset(
    _In_ handle_t Handle,
    _In_ unsigned long HandleStyle,
    _In_ MIDL_ES_CODE Operation,
    _In_opt_ _At_(*pBuffer, _Pre_readable_byte_size_(BufferSize)) char **pBuffer,
    _In_ unsigned long BufferSize,
    _Out_opt_ unsigned long *pEncodedSize
    );


RPC_STATUS  RPC_ENTRY
MesHandleFree( handle_t  Handle );

RPC_STATUS  RPC_ENTRY
MesInqProcEncodingId(
    handle_t                    Handle,
    PRPC_SYNTAX_IDENTIFIER      pInterfaceId,
    unsigned long  *   pProcNum );


size_t  RPC_ENTRY
NdrMesSimpleTypeAlignSize ( handle_t );

void  RPC_ENTRY
NdrMesSimpleTypeDecode(
    handle_t            Handle,
    void            *   pObject,
    short               Size );

void  RPC_ENTRY
NdrMesSimpleTypeEncode(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    const void           *  pObject,
    short                   Size );


size_t  RPC_ENTRY
NdrMesTypeAlignSize(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    PFORMAT_STRING          pFormatString,
    const void           *  pObject );

void  RPC_ENTRY
NdrMesTypeEncode(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    PFORMAT_STRING          pFormatString,
    const void           *  pObject );

void  RPC_ENTRY
NdrMesTypeDecode(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    PFORMAT_STRING          pFormatString,
    void                 *  pObject );

size_t  RPC_ENTRY
NdrMesTypeAlignSize2(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,
    const MIDL_STUB_DESC *          pStubDesc,
    PFORMAT_STRING                  pFormatString,
    const void           *          pObject );

void  RPC_ENTRY
NdrMesTypeEncode2(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,    
    const MIDL_STUB_DESC *          pStubDesc,
    PFORMAT_STRING                  pFormatString,
    const void           *          pObject );

void  RPC_ENTRY
NdrMesTypeDecode2(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,    
    const MIDL_STUB_DESC *          pStubDesc,
    PFORMAT_STRING                  pFormatString,
    void                 *          pObject );

void RPC_ENTRY
NdrMesTypeFree2(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,
    const MIDL_STUB_DESC          * pStubDesc,
    PFORMAT_STRING                  pFormatString,
    void                 *          pObject );

void  RPC_VAR_ENTRY
NdrMesProcEncodeDecode(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    PFORMAT_STRING          pFormatString,
    ... );

CLIENT_CALL_RETURN  RPC_VAR_ENTRY
NdrMesProcEncodeDecode2(
    handle_t                Handle,
    const MIDL_STUB_DESC *  pStubDesc,
    PFORMAT_STRING          pFormatString,
    ...
    );


// ndr64 entries.
size_t  RPC_ENTRY
NdrMesTypeAlignSize3(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    const unsigned long *          ArrTypeOffset[],
    unsigned long                   nTypeIndex,
    const void          *           pObject );

void  RPC_ENTRY
NdrMesTypeEncode3(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,    
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    const unsigned long *          ArrTypeOffset[],
    unsigned long                   nTypeIndex,
    const void          *           pObject );

void  RPC_ENTRY
NdrMesTypeDecode3(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,    
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    const unsigned long *          ArrTypeOffset[],
    unsigned long                   nTypeIndex,
    void                *           pObject );

void  RPC_ENTRY
NdrMesTypeFree3(
    handle_t                        Handle,
    const MIDL_TYPE_PICKLING_INFO * pPicklingInfo,    
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    const unsigned long *          ArrTypeOffset[],
    unsigned long                   nTypeIndex,
    void                *           pObject );


CLIENT_CALL_RETURN RPC_VAR_ENTRY
NdrMesProcEncodeDecode3(
    handle_t                        Handle,
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    unsigned long                   nProcNum,
    void                           *pReturnValue,    
    ... );

void  RPC_ENTRY
NdrMesSimpleTypeDecodeAll(
    handle_t                        Handle,
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    void                *           pObject,
    short                           Size );

void  RPC_ENTRY
NdrMesSimpleTypeEncodeAll(
    handle_t                        Handle,
    const MIDL_STUBLESS_PROXY_INFO* pProxyInfo,
    const void          *           pObject,
    short                           Size );

size_t  RPC_ENTRY
NdrMesSimpleTypeAlignSizeAll ( handle_t Handle,
                               const MIDL_STUBLESS_PROXY_INFO*  pProxyInfo );


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif /* __MIDLES_H__ */
