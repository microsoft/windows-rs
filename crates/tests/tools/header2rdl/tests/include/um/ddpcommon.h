

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


#ifndef __ddpcommon_h__
#define __ddpcommon_h__

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

/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ddpcommon_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum _DedupDataPortManagerOption
    {
        DedupDataPortManagerOption_None	= 0,
        DedupDataPortManagerOption_AutoStart	= 0x1,
        DedupDataPortManagerOption_SkipReconciliation	= 0x2
    } 	DedupDataPortManagerOption;

typedef 
enum _DedupDataPortVolumeStatus
    {
        DedupDataPortVolumeStatus_Unknown	= 0,
        DedupDataPortVolumeStatus_NotEnabled	= 1,
        DedupDataPortVolumeStatus_NotAvailable	= 2,
        DedupDataPortVolumeStatus_Initializing	= 3,
        DedupDataPortVolumeStatus_Ready	= 4,
        DedupDataPortVolumeStatus_Maintenance	= 5,
        DedupDataPortVolumeStatus_Shutdown	= 6
    } 	DedupDataPortVolumeStatus;

typedef 
enum _DedupDataPortRequestStatus
    {
        DedupDataPortRequestStatus_Unknown	= 0,
        DedupDataPortRequestStatus_Queued	= 1,
        DedupDataPortRequestStatus_Processing	= 2,
        DedupDataPortRequestStatus_Partial	= 3,
        DedupDataPortRequestStatus_Complete	= 4,
        DedupDataPortRequestStatus_Failed	= 5
    } 	DedupDataPortRequestStatus;

typedef 
enum _DedupChunkFlags
    {
        DedupChunkFlags_None	= 0,
        DedupChunkFlags_Compressed	= 1
    } 	DedupChunkFlags;

typedef struct _DedupHash
    {
    BYTE Hash[ 32 ];
    } 	DedupHash;

typedef struct _DedupChunk
    {
    DedupHash Hash;
    DedupChunkFlags Flags;
    DWORD LogicalSize;
    DWORD DataSize;
    } 	DedupChunk;

typedef struct _DedupStreamEntry
    {
    DedupHash Hash;
    DWORD LogicalSize;
    DWORD64 Offset;
    } 	DedupStreamEntry;

typedef struct _DedupStream
    {
    BSTR Path;
    DWORD64 Offset;
    DWORD64 Length;
    DWORD ChunkCount;
    } 	DedupStream;

typedef 
enum _DedupChunkingAlgorithm
    {
        DedupChunkingAlgorithm_Unknonwn	= 0,
        DedupChunkingAlgorithm_V1	= 1
    } 	DedupChunkingAlgorithm;

typedef 
enum _DedupHashingAlgorithm
    {
        DedupHashingAlgorithm_Unknonwn	= 0,
        DedupHashingAlgorithm_V1	= 1
    } 	DedupHashingAlgorithm;

typedef 
enum _DedupCompressionAlgorithm
    {
        DedupCompressionAlgorithm_Unknonwn	= 0,
        DedupCompressionAlgorithm_Xpress	= 1
    } 	DedupCompressionAlgorithm;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ddpcommon_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpcommon_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


