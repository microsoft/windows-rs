

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __imapi2_h__
#define __imapi2_h__

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

#ifndef __IDiscMaster2_FWD_DEFINED__
#define __IDiscMaster2_FWD_DEFINED__
typedef interface IDiscMaster2 IDiscMaster2;

#endif 	/* __IDiscMaster2_FWD_DEFINED__ */


#ifndef __DDiscMaster2Events_FWD_DEFINED__
#define __DDiscMaster2Events_FWD_DEFINED__
typedef interface DDiscMaster2Events DDiscMaster2Events;

#endif 	/* __DDiscMaster2Events_FWD_DEFINED__ */


#ifndef __IDiscRecorder2Ex_FWD_DEFINED__
#define __IDiscRecorder2Ex_FWD_DEFINED__
typedef interface IDiscRecorder2Ex IDiscRecorder2Ex;

#endif 	/* __IDiscRecorder2Ex_FWD_DEFINED__ */


#ifndef __IDiscRecorder2_FWD_DEFINED__
#define __IDiscRecorder2_FWD_DEFINED__
typedef interface IDiscRecorder2 IDiscRecorder2;

#endif 	/* __IDiscRecorder2_FWD_DEFINED__ */


#ifndef __IWriteEngine2_FWD_DEFINED__
#define __IWriteEngine2_FWD_DEFINED__
typedef interface IWriteEngine2 IWriteEngine2;

#endif 	/* __IWriteEngine2_FWD_DEFINED__ */


#ifndef __IWriteEngine2EventArgs_FWD_DEFINED__
#define __IWriteEngine2EventArgs_FWD_DEFINED__
typedef interface IWriteEngine2EventArgs IWriteEngine2EventArgs;

#endif 	/* __IWriteEngine2EventArgs_FWD_DEFINED__ */


#ifndef __DWriteEngine2Events_FWD_DEFINED__
#define __DWriteEngine2Events_FWD_DEFINED__
typedef interface DWriteEngine2Events DWriteEngine2Events;

#endif 	/* __DWriteEngine2Events_FWD_DEFINED__ */


#ifndef __IDiscFormat2_FWD_DEFINED__
#define __IDiscFormat2_FWD_DEFINED__
typedef interface IDiscFormat2 IDiscFormat2;

#endif 	/* __IDiscFormat2_FWD_DEFINED__ */


#ifndef __IDiscFormat2Erase_FWD_DEFINED__
#define __IDiscFormat2Erase_FWD_DEFINED__
typedef interface IDiscFormat2Erase IDiscFormat2Erase;

#endif 	/* __IDiscFormat2Erase_FWD_DEFINED__ */


#ifndef __DDiscFormat2EraseEvents_FWD_DEFINED__
#define __DDiscFormat2EraseEvents_FWD_DEFINED__
typedef interface DDiscFormat2EraseEvents DDiscFormat2EraseEvents;

#endif 	/* __DDiscFormat2EraseEvents_FWD_DEFINED__ */


#ifndef __IDiscFormat2Data_FWD_DEFINED__
#define __IDiscFormat2Data_FWD_DEFINED__
typedef interface IDiscFormat2Data IDiscFormat2Data;

#endif 	/* __IDiscFormat2Data_FWD_DEFINED__ */


#ifndef __DDiscFormat2DataEvents_FWD_DEFINED__
#define __DDiscFormat2DataEvents_FWD_DEFINED__
typedef interface DDiscFormat2DataEvents DDiscFormat2DataEvents;

#endif 	/* __DDiscFormat2DataEvents_FWD_DEFINED__ */


#ifndef __IDiscFormat2DataEventArgs_FWD_DEFINED__
#define __IDiscFormat2DataEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2DataEventArgs IDiscFormat2DataEventArgs;

#endif 	/* __IDiscFormat2DataEventArgs_FWD_DEFINED__ */


#ifndef __IDiscFormat2TrackAtOnce_FWD_DEFINED__
#define __IDiscFormat2TrackAtOnce_FWD_DEFINED__
typedef interface IDiscFormat2TrackAtOnce IDiscFormat2TrackAtOnce;

#endif 	/* __IDiscFormat2TrackAtOnce_FWD_DEFINED__ */


#ifndef __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__
#define __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__
typedef interface DDiscFormat2TrackAtOnceEvents DDiscFormat2TrackAtOnceEvents;

#endif 	/* __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__ */


#ifndef __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__
#define __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2TrackAtOnceEventArgs IDiscFormat2TrackAtOnceEventArgs;

#endif 	/* __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__ */


#ifndef __IDiscFormat2RawCD_FWD_DEFINED__
#define __IDiscFormat2RawCD_FWD_DEFINED__
typedef interface IDiscFormat2RawCD IDiscFormat2RawCD;

#endif 	/* __IDiscFormat2RawCD_FWD_DEFINED__ */


#ifndef __DDiscFormat2RawCDEvents_FWD_DEFINED__
#define __DDiscFormat2RawCDEvents_FWD_DEFINED__
typedef interface DDiscFormat2RawCDEvents DDiscFormat2RawCDEvents;

#endif 	/* __DDiscFormat2RawCDEvents_FWD_DEFINED__ */


#ifndef __IDiscFormat2RawCDEventArgs_FWD_DEFINED__
#define __IDiscFormat2RawCDEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2RawCDEventArgs IDiscFormat2RawCDEventArgs;

#endif 	/* __IDiscFormat2RawCDEventArgs_FWD_DEFINED__ */


#ifndef __IBurnVerification_FWD_DEFINED__
#define __IBurnVerification_FWD_DEFINED__
typedef interface IBurnVerification IBurnVerification;

#endif 	/* __IBurnVerification_FWD_DEFINED__ */


#ifndef __IWriteSpeedDescriptor_FWD_DEFINED__
#define __IWriteSpeedDescriptor_FWD_DEFINED__
typedef interface IWriteSpeedDescriptor IWriteSpeedDescriptor;

#endif 	/* __IWriteSpeedDescriptor_FWD_DEFINED__ */


#ifndef __IMultisession_FWD_DEFINED__
#define __IMultisession_FWD_DEFINED__
typedef interface IMultisession IMultisession;

#endif 	/* __IMultisession_FWD_DEFINED__ */


#ifndef __IMultisessionSequential_FWD_DEFINED__
#define __IMultisessionSequential_FWD_DEFINED__
typedef interface IMultisessionSequential IMultisessionSequential;

#endif 	/* __IMultisessionSequential_FWD_DEFINED__ */


#ifndef __IMultisessionSequential2_FWD_DEFINED__
#define __IMultisessionSequential2_FWD_DEFINED__
typedef interface IMultisessionSequential2 IMultisessionSequential2;

#endif 	/* __IMultisessionSequential2_FWD_DEFINED__ */


#ifndef __IMultisessionRandomWrite_FWD_DEFINED__
#define __IMultisessionRandomWrite_FWD_DEFINED__
typedef interface IMultisessionRandomWrite IMultisessionRandomWrite;

#endif 	/* __IMultisessionRandomWrite_FWD_DEFINED__ */


#ifndef __IStreamPseudoRandomBased_FWD_DEFINED__
#define __IStreamPseudoRandomBased_FWD_DEFINED__
typedef interface IStreamPseudoRandomBased IStreamPseudoRandomBased;

#endif 	/* __IStreamPseudoRandomBased_FWD_DEFINED__ */


#ifndef __IStreamConcatenate_FWD_DEFINED__
#define __IStreamConcatenate_FWD_DEFINED__
typedef interface IStreamConcatenate IStreamConcatenate;

#endif 	/* __IStreamConcatenate_FWD_DEFINED__ */


#ifndef __IStreamInterleave_FWD_DEFINED__
#define __IStreamInterleave_FWD_DEFINED__
typedef interface IStreamInterleave IStreamInterleave;

#endif 	/* __IStreamInterleave_FWD_DEFINED__ */


#ifndef __IRawCDImageCreator_FWD_DEFINED__
#define __IRawCDImageCreator_FWD_DEFINED__
typedef interface IRawCDImageCreator IRawCDImageCreator;

#endif 	/* __IRawCDImageCreator_FWD_DEFINED__ */


#ifndef __IRawCDImageTrackInfo_FWD_DEFINED__
#define __IRawCDImageTrackInfo_FWD_DEFINED__
typedef interface IRawCDImageTrackInfo IRawCDImageTrackInfo;

#endif 	/* __IRawCDImageTrackInfo_FWD_DEFINED__ */


#ifndef __IBlockRange_FWD_DEFINED__
#define __IBlockRange_FWD_DEFINED__
typedef interface IBlockRange IBlockRange;

#endif 	/* __IBlockRange_FWD_DEFINED__ */


#ifndef __IBlockRangeList_FWD_DEFINED__
#define __IBlockRangeList_FWD_DEFINED__
typedef interface IBlockRangeList IBlockRangeList;

#endif 	/* __IBlockRangeList_FWD_DEFINED__ */


#ifndef __IWriteEngine2EventArgs_FWD_DEFINED__
#define __IWriteEngine2EventArgs_FWD_DEFINED__
typedef interface IWriteEngine2EventArgs IWriteEngine2EventArgs;

#endif 	/* __IWriteEngine2EventArgs_FWD_DEFINED__ */


#ifndef __IDiscFormat2DataEventArgs_FWD_DEFINED__
#define __IDiscFormat2DataEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2DataEventArgs IDiscFormat2DataEventArgs;

#endif 	/* __IDiscFormat2DataEventArgs_FWD_DEFINED__ */


#ifndef __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__
#define __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2TrackAtOnceEventArgs IDiscFormat2TrackAtOnceEventArgs;

#endif 	/* __IDiscFormat2TrackAtOnceEventArgs_FWD_DEFINED__ */


#ifndef __IDiscFormat2RawCDEventArgs_FWD_DEFINED__
#define __IDiscFormat2RawCDEventArgs_FWD_DEFINED__
typedef interface IDiscFormat2RawCDEventArgs IDiscFormat2RawCDEventArgs;

#endif 	/* __IDiscFormat2RawCDEventArgs_FWD_DEFINED__ */


#ifndef __IWriteSpeedDescriptor_FWD_DEFINED__
#define __IWriteSpeedDescriptor_FWD_DEFINED__
typedef interface IWriteSpeedDescriptor IWriteSpeedDescriptor;

#endif 	/* __IWriteSpeedDescriptor_FWD_DEFINED__ */


#ifndef __DDiscMaster2Events_FWD_DEFINED__
#define __DDiscMaster2Events_FWD_DEFINED__
typedef interface DDiscMaster2Events DDiscMaster2Events;

#endif 	/* __DDiscMaster2Events_FWD_DEFINED__ */


#ifndef __DWriteEngine2Events_FWD_DEFINED__
#define __DWriteEngine2Events_FWD_DEFINED__
typedef interface DWriteEngine2Events DWriteEngine2Events;

#endif 	/* __DWriteEngine2Events_FWD_DEFINED__ */


#ifndef __DDiscFormat2EraseEvents_FWD_DEFINED__
#define __DDiscFormat2EraseEvents_FWD_DEFINED__
typedef interface DDiscFormat2EraseEvents DDiscFormat2EraseEvents;

#endif 	/* __DDiscFormat2EraseEvents_FWD_DEFINED__ */


#ifndef __DDiscFormat2DataEvents_FWD_DEFINED__
#define __DDiscFormat2DataEvents_FWD_DEFINED__
typedef interface DDiscFormat2DataEvents DDiscFormat2DataEvents;

#endif 	/* __DDiscFormat2DataEvents_FWD_DEFINED__ */


#ifndef __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__
#define __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__
typedef interface DDiscFormat2TrackAtOnceEvents DDiscFormat2TrackAtOnceEvents;

#endif 	/* __DDiscFormat2TrackAtOnceEvents_FWD_DEFINED__ */


#ifndef __DDiscFormat2RawCDEvents_FWD_DEFINED__
#define __DDiscFormat2RawCDEvents_FWD_DEFINED__
typedef interface DDiscFormat2RawCDEvents DDiscFormat2RawCDEvents;

#endif 	/* __DDiscFormat2RawCDEvents_FWD_DEFINED__ */


#ifndef __IRawCDImageCreator_FWD_DEFINED__
#define __IRawCDImageCreator_FWD_DEFINED__
typedef interface IRawCDImageCreator IRawCDImageCreator;

#endif 	/* __IRawCDImageCreator_FWD_DEFINED__ */


#ifndef __IRawCDImageTrackInfo_FWD_DEFINED__
#define __IRawCDImageTrackInfo_FWD_DEFINED__
typedef interface IRawCDImageTrackInfo IRawCDImageTrackInfo;

#endif 	/* __IRawCDImageTrackInfo_FWD_DEFINED__ */


#ifndef __IBurnVerification_FWD_DEFINED__
#define __IBurnVerification_FWD_DEFINED__
typedef interface IBurnVerification IBurnVerification;

#endif 	/* __IBurnVerification_FWD_DEFINED__ */


#ifndef __IBlockRange_FWD_DEFINED__
#define __IBlockRange_FWD_DEFINED__
typedef interface IBlockRange IBlockRange;

#endif 	/* __IBlockRange_FWD_DEFINED__ */


#ifndef __IBlockRangeList_FWD_DEFINED__
#define __IBlockRangeList_FWD_DEFINED__
typedef interface IBlockRangeList IBlockRangeList;

#endif 	/* __IBlockRangeList_FWD_DEFINED__ */


#ifndef __MsftDiscMaster2_FWD_DEFINED__
#define __MsftDiscMaster2_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscMaster2 MsftDiscMaster2;
#else
typedef struct MsftDiscMaster2 MsftDiscMaster2;
#endif /* __cplusplus */

#endif 	/* __MsftDiscMaster2_FWD_DEFINED__ */


#ifndef __MsftDiscRecorder2_FWD_DEFINED__
#define __MsftDiscRecorder2_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscRecorder2 MsftDiscRecorder2;
#else
typedef struct MsftDiscRecorder2 MsftDiscRecorder2;
#endif /* __cplusplus */

#endif 	/* __MsftDiscRecorder2_FWD_DEFINED__ */


#ifndef __MsftWriteEngine2_FWD_DEFINED__
#define __MsftWriteEngine2_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftWriteEngine2 MsftWriteEngine2;
#else
typedef struct MsftWriteEngine2 MsftWriteEngine2;
#endif /* __cplusplus */

#endif 	/* __MsftWriteEngine2_FWD_DEFINED__ */


#ifndef __MsftDiscFormat2Erase_FWD_DEFINED__
#define __MsftDiscFormat2Erase_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscFormat2Erase MsftDiscFormat2Erase;
#else
typedef struct MsftDiscFormat2Erase MsftDiscFormat2Erase;
#endif /* __cplusplus */

#endif 	/* __MsftDiscFormat2Erase_FWD_DEFINED__ */


#ifndef __MsftDiscFormat2Data_FWD_DEFINED__
#define __MsftDiscFormat2Data_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscFormat2Data MsftDiscFormat2Data;
#else
typedef struct MsftDiscFormat2Data MsftDiscFormat2Data;
#endif /* __cplusplus */

#endif 	/* __MsftDiscFormat2Data_FWD_DEFINED__ */


#ifndef __MsftDiscFormat2TrackAtOnce_FWD_DEFINED__
#define __MsftDiscFormat2TrackAtOnce_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscFormat2TrackAtOnce MsftDiscFormat2TrackAtOnce;
#else
typedef struct MsftDiscFormat2TrackAtOnce MsftDiscFormat2TrackAtOnce;
#endif /* __cplusplus */

#endif 	/* __MsftDiscFormat2TrackAtOnce_FWD_DEFINED__ */


#ifndef __MsftDiscFormat2RawCD_FWD_DEFINED__
#define __MsftDiscFormat2RawCD_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftDiscFormat2RawCD MsftDiscFormat2RawCD;
#else
typedef struct MsftDiscFormat2RawCD MsftDiscFormat2RawCD;
#endif /* __cplusplus */

#endif 	/* __MsftDiscFormat2RawCD_FWD_DEFINED__ */


#ifndef __MsftStreamZero_FWD_DEFINED__
#define __MsftStreamZero_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftStreamZero MsftStreamZero;
#else
typedef struct MsftStreamZero MsftStreamZero;
#endif /* __cplusplus */

#endif 	/* __MsftStreamZero_FWD_DEFINED__ */


#ifndef __MsftStreamPrng001_FWD_DEFINED__
#define __MsftStreamPrng001_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftStreamPrng001 MsftStreamPrng001;
#else
typedef struct MsftStreamPrng001 MsftStreamPrng001;
#endif /* __cplusplus */

#endif 	/* __MsftStreamPrng001_FWD_DEFINED__ */


#ifndef __MsftStreamConcatenate_FWD_DEFINED__
#define __MsftStreamConcatenate_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftStreamConcatenate MsftStreamConcatenate;
#else
typedef struct MsftStreamConcatenate MsftStreamConcatenate;
#endif /* __cplusplus */

#endif 	/* __MsftStreamConcatenate_FWD_DEFINED__ */


#ifndef __MsftStreamInterleave_FWD_DEFINED__
#define __MsftStreamInterleave_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftStreamInterleave MsftStreamInterleave;
#else
typedef struct MsftStreamInterleave MsftStreamInterleave;
#endif /* __cplusplus */

#endif 	/* __MsftStreamInterleave_FWD_DEFINED__ */


#ifndef __MsftWriteSpeedDescriptor_FWD_DEFINED__
#define __MsftWriteSpeedDescriptor_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftWriteSpeedDescriptor MsftWriteSpeedDescriptor;
#else
typedef struct MsftWriteSpeedDescriptor MsftWriteSpeedDescriptor;
#endif /* __cplusplus */

#endif 	/* __MsftWriteSpeedDescriptor_FWD_DEFINED__ */


#ifndef __MsftMultisessionSequential_FWD_DEFINED__
#define __MsftMultisessionSequential_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftMultisessionSequential MsftMultisessionSequential;
#else
typedef struct MsftMultisessionSequential MsftMultisessionSequential;
#endif /* __cplusplus */

#endif 	/* __MsftMultisessionSequential_FWD_DEFINED__ */


#ifndef __MsftMultisessionRandomWrite_FWD_DEFINED__
#define __MsftMultisessionRandomWrite_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftMultisessionRandomWrite MsftMultisessionRandomWrite;
#else
typedef struct MsftMultisessionRandomWrite MsftMultisessionRandomWrite;
#endif /* __cplusplus */

#endif 	/* __MsftMultisessionRandomWrite_FWD_DEFINED__ */


#ifndef __MsftRawCDImageCreator_FWD_DEFINED__
#define __MsftRawCDImageCreator_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftRawCDImageCreator MsftRawCDImageCreator;
#else
typedef struct MsftRawCDImageCreator MsftRawCDImageCreator;
#endif /* __cplusplus */

#endif 	/* __MsftRawCDImageCreator_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_imapi2_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


























#define	IMAPI_SECTOR_SIZE	( 2048 )

#define IMAPI2_DEFAULT_COMMAND_TIMEOUT 10
typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_MEDIA_PHYSICAL_TYPE
    {
        IMAPI_MEDIA_TYPE_UNKNOWN	= 0,
        IMAPI_MEDIA_TYPE_CDROM	= 0x1,
        IMAPI_MEDIA_TYPE_CDR	= 0x2,
        IMAPI_MEDIA_TYPE_CDRW	= 0x3,
        IMAPI_MEDIA_TYPE_DVDROM	= 0x4,
        IMAPI_MEDIA_TYPE_DVDRAM	= 0x5,
        IMAPI_MEDIA_TYPE_DVDPLUSR	= 0x6,
        IMAPI_MEDIA_TYPE_DVDPLUSRW	= 0x7,
        IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER	= 0x8,
        IMAPI_MEDIA_TYPE_DVDDASHR	= 0x9,
        IMAPI_MEDIA_TYPE_DVDDASHRW	= 0xa,
        IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER	= 0xb,
        IMAPI_MEDIA_TYPE_DISK	= 0xc,
        IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER	= 0xd,
        IMAPI_MEDIA_TYPE_HDDVDROM	= 0xe,
        IMAPI_MEDIA_TYPE_HDDVDR	= 0xf,
        IMAPI_MEDIA_TYPE_HDDVDRAM	= 0x10,
        IMAPI_MEDIA_TYPE_BDROM	= 0x11,
        IMAPI_MEDIA_TYPE_BDR	= 0x12,
        IMAPI_MEDIA_TYPE_BDRE	= 0x13,
        IMAPI_MEDIA_TYPE_MAX	= 0x13
    } 	IMAPI_MEDIA_PHYSICAL_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_MEDIA_PHYSICAL_TYPE *PIMAPI_MEDIA_PHYSICAL_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_MEDIA_WRITE_PROTECT_STATE
    {
        IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN	= 0x1,
        IMAPI_WRITEPROTECTED_BY_CARTRIDGE	= 0x2,
        IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON	= 0x4,
        IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT	= 0x8,
        IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK	= 0x10,
        IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA	= 0x4000
    } 	IMAPI_MEDIA_WRITE_PROTECT_STATE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_MEDIA_WRITE_PROTECT_STATE *PIMAPI_MEDIA_WRITE_PROTECT_STATE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_READ_TRACK_ADDRESS_TYPE
    {
        IMAPI_READ_TRACK_ADDRESS_TYPE_LBA	= 0,
        IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK	= 1,
        IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION	= 2
    } 	IMAPI_READ_TRACK_ADDRESS_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_READ_TRACK_ADDRESS_TYPE *PIMAPI_READ_TRACK_ADDRESS_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_MODE_PAGE_REQUEST_TYPE
    {
        IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES	= 0,
        IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES	= 1,
        IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES	= 2,
        IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES	= 3
    } 	IMAPI_MODE_PAGE_REQUEST_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_MODE_PAGE_REQUEST_TYPE *PIMAPI_MODE_PAGE_REQUEST_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_MODE_PAGE_TYPE
    {
        IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY	= 0x1,
        IMAPI_MODE_PAGE_TYPE_MRW	= 0x3,
        IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS	= 0x5,
        IMAPI_MODE_PAGE_TYPE_CACHING	= 0x8,
        IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS	= 0x1c,
        IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT	= 0x1d,
        IMAPI_MODE_PAGE_TYPE_POWER_CONDITION	= 0x1a,
        IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES	= 0x2a
    } 	IMAPI_MODE_PAGE_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_MODE_PAGE_TYPE *PIMAPI_MODE_PAGE_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FEATURE_PAGE_TYPE
    {
        IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST	= 0,
        IMAPI_FEATURE_PAGE_TYPE_CORE	= 0x1,
        IMAPI_FEATURE_PAGE_TYPE_MORPHING	= 0x2,
        IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM	= 0x3,
        IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT	= 0x4,
        IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE	= 0x10,
        IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD	= 0x1d,
        IMAPI_FEATURE_PAGE_TYPE_CD_READ	= 0x1e,
        IMAPI_FEATURE_PAGE_TYPE_DVD_READ	= 0x1f,
        IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE	= 0x20,
        IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE	= 0x21,
        IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE	= 0x22,
        IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE	= 0x23,
        IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT	= 0x24,
        IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE	= 0x25,
        IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE	= 0x26,
        IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE	= 0x27,
        IMAPI_FEATURE_PAGE_TYPE_MRW	= 0x28,
        IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING	= 0x29,
        IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW	= 0x2a,
        IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R	= 0x2b,
        IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE	= 0x2c,
        IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE	= 0x2d,
        IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING	= 0x2e,
        IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE	= 0x2f,
        IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ	= 0x30,
        IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE	= 0x31,
        IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE	= 0x32,
        IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING	= 0x33,
        IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT	= 0x37,
        IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE	= 0x38,
        IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER	= 0x3b,
        IMAPI_FEATURE_PAGE_TYPE_BD_READ	= 0x40,
        IMAPI_FEATURE_PAGE_TYPE_BD_WRITE	= 0x41,
        IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ	= 0x50,
        IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE	= 0x51,
        IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT	= 0x100,
        IMAPI_FEATURE_PAGE_TYPE_SMART	= 0x101,
        IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER	= 0x102,
        IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY	= 0x103,
        IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE	= 0x104,
        IMAPI_FEATURE_PAGE_TYPE_TIMEOUT	= 0x105,
        IMAPI_FEATURE_PAGE_TYPE_DVD_CSS	= 0x106,
        IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING	= 0x107,
        IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER	= 0x108,
        IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER	= 0x109,
        IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS	= 0x10a,
        IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM	= 0x10b,
        IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION	= 0x10c,
        IMAPI_FEATURE_PAGE_TYPE_AACS	= 0x10d,
        IMAPI_FEATURE_PAGE_TYPE_VCPS	= 0x110
    } 	IMAPI_FEATURE_PAGE_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FEATURE_PAGE_TYPE *PIMAPI_FEATURE_PAGE_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_PROFILE_TYPE
    {
        IMAPI_PROFILE_TYPE_INVALID	= 0,
        IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK	= 0x1,
        IMAPI_PROFILE_TYPE_REMOVABLE_DISK	= 0x2,
        IMAPI_PROFILE_TYPE_MO_ERASABLE	= 0x3,
        IMAPI_PROFILE_TYPE_MO_WRITE_ONCE	= 0x4,
        IMAPI_PROFILE_TYPE_AS_MO	= 0x5,
        IMAPI_PROFILE_TYPE_CDROM	= 0x8,
        IMAPI_PROFILE_TYPE_CD_RECORDABLE	= 0x9,
        IMAPI_PROFILE_TYPE_CD_REWRITABLE	= 0xa,
        IMAPI_PROFILE_TYPE_DVDROM	= 0x10,
        IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE	= 0x11,
        IMAPI_PROFILE_TYPE_DVD_RAM	= 0x12,
        IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE	= 0x13,
        IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL	= 0x14,
        IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL	= 0x15,
        IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP	= 0x16,
        IMAPI_PROFILE_TYPE_DVD_PLUS_RW	= 0x1a,
        IMAPI_PROFILE_TYPE_DVD_PLUS_R	= 0x1b,
        IMAPI_PROFILE_TYPE_DDCDROM	= 0x20,
        IMAPI_PROFILE_TYPE_DDCD_RECORDABLE	= 0x21,
        IMAPI_PROFILE_TYPE_DDCD_REWRITABLE	= 0x22,
        IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL	= 0x2a,
        IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL	= 0x2b,
        IMAPI_PROFILE_TYPE_BD_ROM	= 0x40,
        IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL	= 0x41,
        IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING	= 0x42,
        IMAPI_PROFILE_TYPE_BD_REWRITABLE	= 0x43,
        IMAPI_PROFILE_TYPE_HD_DVD_ROM	= 0x50,
        IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE	= 0x51,
        IMAPI_PROFILE_TYPE_HD_DVD_RAM	= 0x52,
        IMAPI_PROFILE_TYPE_NON_STANDARD	= 0xffff
    } 	IMAPI_PROFILE_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_PROFILE_TYPE *PIMAPI_PROFILE_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FORMAT2_DATA_WRITE_ACTION
    {
        IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA	= 0,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA	= 0x1,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE	= 0x2,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER	= 0x3,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA	= 0x4,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION	= 0x5,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED	= 0x6,
        IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING	= 0x7
    } 	IMAPI_FORMAT2_DATA_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FORMAT2_DATA_WRITE_ACTION *PIMAPI_FORMAT2_DATA_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FORMAT2_DATA_MEDIA_STATE
    {
        IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN	= 0,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK	= 0xf,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK	= 0xfc00,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY	= 0x1,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE	= 0x1,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK	= 0x2,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE	= 0x4,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION	= 0x8,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED	= 0x400,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED	= 0x800,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION	= 0x1000,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED	= 0x2000,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED	= 0x4000,
        IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA	= 0x8000
    } 	IMAPI_FORMAT2_DATA_MEDIA_STATE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FORMAT2_DATA_MEDIA_STATE *PIMAPI_FORMAT2_DATA_MEDIA_STATE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FORMAT2_TAO_WRITE_ACTION
    {
        IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN	= 0,
        IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING	= 0x1,
        IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING	= 0x2,
        IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING	= 0x3,
        IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING	= 0x4
    } 	IMAPI_FORMAT2_TAO_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FORMAT2_TAO_WRITE_ACTION *PIMAPI_FORMAT2_TAO_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE
    {
        IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY	= 0x1,
        IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED	= 0x2,
        IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW	= 0x3
    } 	IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE *PIMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_FORMAT2_RAW_CD_WRITE_ACTION
    {
        IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN	= 0,
        IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING	= 0x1,
        IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING	= 0x2,
        IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING	= 0x3
    } 	IMAPI_FORMAT2_RAW_CD_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_FORMAT2_RAW_CD_WRITE_ACTION *PIMAPI_FORMAT2_RAW_CD_WRITE_ACTION;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_CD_SECTOR_TYPE
    {
        IMAPI_CD_SECTOR_AUDIO	= 0,
        IMAPI_CD_SECTOR_MODE_ZERO	= 0x1,
        IMAPI_CD_SECTOR_MODE1	= 0x2,
        IMAPI_CD_SECTOR_MODE2FORM0	= 0x3,
        IMAPI_CD_SECTOR_MODE2FORM1	= 0x4,
        IMAPI_CD_SECTOR_MODE2FORM2	= 0x5,
        IMAPI_CD_SECTOR_MODE1RAW	= 0x6,
        IMAPI_CD_SECTOR_MODE2FORM0RAW	= 0x7,
        IMAPI_CD_SECTOR_MODE2FORM1RAW	= 0x8,
        IMAPI_CD_SECTOR_MODE2FORM2RAW	= 0x9
    } 	IMAPI_CD_SECTOR_TYPE;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_CD_SECTOR_TYPE *PIMAPI_CD_SECTOR_TYPE;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_CD_TRACK_DIGITAL_COPY_SETTING
    {
        IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED	= 0,
        IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED	= 0x1,
        IMAPI_CD_TRACK_DIGITAL_COPY_SCMS	= 0x2
    } 	IMAPI_CD_TRACK_DIGITAL_COPY_SETTING;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_CD_TRACK_DIGITAL_COPY_SETTING *PIMAPI_CD_TRACK_DIGITAL_COPY_SETTING;

typedef /* [public][helpstring][v1_enum] */ 
enum _IMAPI_BURN_VERIFICATION_LEVEL
    {
        IMAPI_BURN_VERIFICATION_NONE	= 0,
        IMAPI_BURN_VERIFICATION_QUICK	= 1,
        IMAPI_BURN_VERIFICATION_FULL	= 2
    } 	IMAPI_BURN_VERIFICATION_LEVEL;

typedef /* [public][helpstring][v1_enum] */ enum _IMAPI_BURN_VERIFICATION_LEVEL *PIMAPI_BURN_VERIFICATION_LEVEL;

// begin_wpp config
// CUSTOM_TYPE(IMAPI_MEDIA_PHYSICAL_TYPE,            ItemEnum(_IMAPI_MEDIA_PHYSICAL_TYPE));
// CUSTOM_TYPE(IMAPI_MEDIA_WRITE_PROTECT_STATE,      ItemEnum(_IMAPI_MEDIA_WRITE_PROTECT_STATE));
// CUSTOM_TYPE(IMAPI_READ_TRACK_ADDRESS_TYPE,        ItemEnum(_IMAPI_READ_TRACK_ADDRESS_TYPE));
// CUSTOM_TYPE(IMAPI_MODE_PAGE_REQUEST_TYPE,         ItemEnum(_IMAPI_MODE_PAGE_REQUEST_TYPE));
// CUSTOM_TYPE(IMAPI_MODE_PAGE_TYPE,                 ItemEnum(_IMAPI_MODE_PAGE_TYPE));
// CUSTOM_TYPE(IMAPI_FEATURE_PAGE_TYPE,              ItemEnum(_IMAPI_FEATURE_PAGE_TYPE));
// CUSTOM_TYPE(IMAPI_PROFILE_TYPE,                   ItemEnum(_IMAPI_PROFILE_TYPE));
// CUSTOM_TYPE(IMAPI_FORMAT2_DATA_WRITE_ACTION,      ItemEnum(_IMAPI_FORMAT2_DATA_WRITE_ACTION));
// CUSTOM_TYPE(IMAPI_FORMAT2_DATA_MEDIA_STATE,       ItemEnum(_IMAPI_FORMAT2_DATA_MEDIA_STATE));
// CUSTOM_TYPE(IMAPI_FORMAT2_TAO_WRITE_ACTION,       ItemEnum(_IMAPI_FORMAT2_TAO_WRITE_ACTION));
// CUSTOM_TYPE(IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE, ItemEnum(_IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE));
// CUSTOM_TYPE(IMAPI_FORMAT2_RAW_CD_WRITE_ACTION,    ItemEnum(_IMAPI_FORMAT2_RAW_CD_WRITE_ACTION));
// CUSTOM_TYPE(IMAPI_CD_SECTOR_DATA_TYPE,            ItemEnum(_IMAPI_CD_SECTOR_TYPE));
// CUSTOM_TYPE(IMAPI_CD_TRACK_DIGITAL_COPY_SETTING,  ItemEnum(_IMAPI_CD_TRACK_DIGITAL_COPY_SETTING));
// CUSTOM_TYPE(IMAPI_BURN_VERIFICATION_LEVEL,        ItemEnum(_IMAPI_BURN_VERIFICATION_LEVEL));
// end_wpp


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0000_v0_0_s_ifspec;

#ifndef __IDiscMaster2_INTERFACE_DEFINED__
#define __IDiscMaster2_INTERFACE_DEFINED__

/* interface IDiscMaster2 */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscMaster2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354130-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscMaster2 : public IDispatch
    {
    public:
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumVARIANT **ppunk) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG index,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsSupportedEnvironment( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscMaster2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscMaster2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscMaster2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscMaster2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscMaster2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscMaster2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscMaster2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscMaster2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscMaster2, get__NewEnum)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IDiscMaster2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumVARIANT **ppunk);
        
        DECLSPEC_XFGVIRT(IDiscMaster2, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IDiscMaster2 * This,
            /* [in] */ LONG index,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscMaster2, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IDiscMaster2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscMaster2, get_IsSupportedEnvironment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSupportedEnvironment )( 
            __RPC__in IDiscMaster2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        END_INTERFACE
    } IDiscMaster2Vtbl;

    interface IDiscMaster2
    {
        CONST_VTBL struct IDiscMaster2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscMaster2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscMaster2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscMaster2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscMaster2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscMaster2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscMaster2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscMaster2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscMaster2_get__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppunk) ) 

#define IDiscMaster2_get_Item(This,index,value)	\
    ( (This)->lpVtbl -> get_Item(This,index,value) ) 

#define IDiscMaster2_get_Count(This,value)	\
    ( (This)->lpVtbl -> get_Count(This,value) ) 

#define IDiscMaster2_get_IsSupportedEnvironment(This,value)	\
    ( (This)->lpVtbl -> get_IsSupportedEnvironment(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscMaster2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0001 */
/* [local] */ 

#define DISPID_DDISCMASTER2EVENTS_DEVICEADDED   0x100
#define DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED 0x101


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0001_v0_0_s_ifspec;

#ifndef __DDiscMaster2Events_INTERFACE_DEFINED__
#define __DDiscMaster2Events_INTERFACE_DEFINED__

/* interface DDiscMaster2Events */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DDiscMaster2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354131-7F64-5B0F-8F00-5D77AFBE261E")
    DDiscMaster2Events : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyDeviceAdded( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR uniqueId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyDeviceRemoved( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR uniqueId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DDiscMaster2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DDiscMaster2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DDiscMaster2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DDiscMaster2Events * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DDiscMaster2Events * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DDiscMaster2Events * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DDiscMaster2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DDiscMaster2Events * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DDiscMaster2Events, NotifyDeviceAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyDeviceAdded )( 
            __RPC__in DDiscMaster2Events * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR uniqueId);
        
        DECLSPEC_XFGVIRT(DDiscMaster2Events, NotifyDeviceRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyDeviceRemoved )( 
            __RPC__in DDiscMaster2Events * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR uniqueId);
        
        END_INTERFACE
    } DDiscMaster2EventsVtbl;

    interface DDiscMaster2Events
    {
        CONST_VTBL struct DDiscMaster2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DDiscMaster2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DDiscMaster2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DDiscMaster2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DDiscMaster2Events_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DDiscMaster2Events_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DDiscMaster2Events_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DDiscMaster2Events_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DDiscMaster2Events_NotifyDeviceAdded(This,object,uniqueId)	\
    ( (This)->lpVtbl -> NotifyDeviceAdded(This,object,uniqueId) ) 

#define DDiscMaster2Events_NotifyDeviceRemoved(This,object,uniqueId)	\
    ( (This)->lpVtbl -> NotifyDeviceRemoved(This,object,uniqueId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DDiscMaster2Events_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0002 */
/* [local] */ 

typedef /* [range] */ __RPC__range(0,0xffff) ULONG ULONG_IMAPI2_DVD_STRUCTURE;

typedef /* [range] */ __RPC__range(0,0xfffffff) ULONG ULONG_IMAPI2_ADAPTER_DESCRIPTOR;

typedef /* [range] */ __RPC__range(0,0xfffffff) ULONG ULONG_IMAPI2_DEVICE_DESCRIPTOR;

typedef /* [range] */ __RPC__range(0,0x10002) ULONG ULONG_IMAPI2_DISC_INFORMATION;

typedef /* [range] */ __RPC__range(0,0x10002) ULONG ULONG_IMAPI2_TRACK_INFORMATION;

typedef /* [range] */ __RPC__range(0,0x100) ULONG ULONG_IMAPI2_FEATURE_PAGE;

typedef /* [range] */ __RPC__range(0,0x101) ULONG ULONG_IMAPI2_MODE_PAGE;

typedef /* [range] */ __RPC__range(0,0x10000) ULONG ULONG_IMAPI2_ALL_FEATURE_PAGES;

typedef /* [range] */ __RPC__range(0,0x3f) ULONG ULONG_IMAPI2_ALL_PROFILES;

typedef /* [range] */ __RPC__range(0,0x7ffb) ULONG ULONG_IMAPI2_ALL_MODE_PAGES;

typedef /* [range] */ __RPC__range(1,0x7fffffff) ULONG ULONG_IMAPI2_NONZERO;

typedef /* [range] */ __RPC__range(0,0x7fffffff) ULONG ULONG_IMAPI2_NOT_NEGATIVE;



extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0002_v0_0_s_ifspec;

#ifndef __IDiscRecorder2Ex_INTERFACE_DEFINED__
#define __IDiscRecorder2Ex_INTERFACE_DEFINED__

/* interface IDiscRecorder2Ex */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IDiscRecorder2Ex;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354132-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscRecorder2Ex : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendCommandNoData( 
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendCommandSendDataToDevice( 
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(BufferSize) BYTE *Buffer,
            /* [in] */ ULONG_IMAPI2_NONZERO BufferSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendCommandGetDataFromDevice( 
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout,
            /* [length_is][size_is][ref][out] */ __RPC__out_ecount_part(BufferSize, *BufferFetched) BYTE *Buffer,
            /* [in] */ ULONG_IMAPI2_NONZERO BufferSize,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_NOT_NEGATIVE *BufferFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReadDvdStructure( 
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG format,
            /* [in] */ ULONG address,
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG layer,
            /* [range][in] */ __RPC__in_range(0,0x3) ULONG agid,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*count) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DVD_STRUCTURE *count) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendDvdStructure( 
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG format,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(count) BYTE *data,
            /* [in] */ ULONG_IMAPI2_DVD_STRUCTURE count) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAdapterDescriptor( 
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ADAPTER_DESCRIPTOR *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDeviceDescriptor( 
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DEVICE_DESCRIPTOR *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDiscInformation( 
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **discInformation,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DISC_INFORMATION *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTrackInformation( 
            /* [in] */ ULONG address,
            /* [in] */ IMAPI_READ_TRACK_ADDRESS_TYPE addressType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **trackInformation,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_TRACK_INFORMATION *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFeaturePage( 
            /* [in] */ IMAPI_FEATURE_PAGE_TYPE requestedFeature,
            /* [in] */ BOOLEAN currentFeatureOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **featureData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_FEATURE_PAGE *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetModePage( 
            /* [in] */ IMAPI_MODE_PAGE_TYPE requestedModePage,
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **modePageData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_MODE_PAGE *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetModePage( 
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(byteSize) BYTE *data,
            /* [in] */ ULONG_IMAPI2_MODE_PAGE byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSupportedFeaturePages( 
            /* [in] */ BOOLEAN currentFeatureOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) IMAPI_FEATURE_PAGE_TYPE **featureData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_FEATURE_PAGES *byteSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSupportedProfiles( 
            /* [in] */ BOOLEAN currentOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*validProfiles) IMAPI_PROFILE_TYPE **profileTypes,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_PROFILES *validProfiles) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSupportedModePages( 
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*validPages) IMAPI_MODE_PAGE_TYPE **modePageTypes,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_MODE_PAGES *validPages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetByteAlignmentMask( 
            /* [retval][ref][out] */ __RPC__out ULONG *value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMaximumNonPageAlignedTransferSize( 
            /* [retval][ref][out] */ __RPC__out ULONG *value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMaximumPageAlignedTransferSize( 
            /* [retval][ref][out] */ __RPC__out ULONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscRecorder2ExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscRecorder2Ex * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscRecorder2Ex * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, SendCommandNoData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendCommandNoData )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, SendCommandSendDataToDevice)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendCommandSendDataToDevice )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(BufferSize) BYTE *Buffer,
            /* [in] */ ULONG_IMAPI2_NONZERO BufferSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, SendCommandGetDataFromDevice)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendCommandGetDataFromDevice )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(CdbSize) BYTE *Cdb,
            /* [range][in] */ __RPC__in_range(6,16) ULONG CdbSize,
            /* [ref][out] */ __RPC__out_ecount_full(18) BYTE SenseBuffer[ 18 ],
            /* [in] */ ULONG Timeout,
            /* [length_is][size_is][ref][out] */ __RPC__out_ecount_part(BufferSize, *BufferFetched) BYTE *Buffer,
            /* [in] */ ULONG_IMAPI2_NONZERO BufferSize,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_NOT_NEGATIVE *BufferFetched);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, ReadDvdStructure)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReadDvdStructure )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG format,
            /* [in] */ ULONG address,
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG layer,
            /* [range][in] */ __RPC__in_range(0,0x3) ULONG agid,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*count) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DVD_STRUCTURE *count);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, SendDvdStructure)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendDvdStructure )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [range][in] */ __RPC__in_range(0,0xff) ULONG format,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(count) BYTE *data,
            /* [in] */ ULONG_IMAPI2_DVD_STRUCTURE count);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetAdapterDescriptor)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAdapterDescriptor )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ADAPTER_DESCRIPTOR *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetDeviceDescriptor)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceDescriptor )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **data,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DEVICE_DESCRIPTOR *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetDiscInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDiscInformation )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **discInformation,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_DISC_INFORMATION *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetTrackInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTrackInformation )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ ULONG address,
            /* [in] */ IMAPI_READ_TRACK_ADDRESS_TYPE addressType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **trackInformation,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_TRACK_INFORMATION *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetFeaturePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFeaturePage )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ IMAPI_FEATURE_PAGE_TYPE requestedFeature,
            /* [in] */ BOOLEAN currentFeatureOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **featureData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_FEATURE_PAGE *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetModePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetModePage )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ IMAPI_MODE_PAGE_TYPE requestedModePage,
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) BYTE **modePageData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_MODE_PAGE *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, SetModePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetModePage )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][ref][in] */ __RPC__in_ecount_full(byteSize) BYTE *data,
            /* [in] */ ULONG_IMAPI2_MODE_PAGE byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetSupportedFeaturePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedFeaturePages )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ BOOLEAN currentFeatureOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*byteSize) IMAPI_FEATURE_PAGE_TYPE **featureData,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_FEATURE_PAGES *byteSize);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetSupportedProfiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedProfiles )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ BOOLEAN currentOnly,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*validProfiles) IMAPI_PROFILE_TYPE **profileTypes,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_PROFILES *validProfiles);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetSupportedModePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedModePages )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [in] */ IMAPI_MODE_PAGE_REQUEST_TYPE requestType,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*validPages) IMAPI_MODE_PAGE_TYPE **modePageTypes,
            /* [ref][out] */ __RPC__out ULONG_IMAPI2_ALL_MODE_PAGES *validPages);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetByteAlignmentMask)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetByteAlignmentMask )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [retval][ref][out] */ __RPC__out ULONG *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetMaximumNonPageAlignedTransferSize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMaximumNonPageAlignedTransferSize )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [retval][ref][out] */ __RPC__out ULONG *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2Ex, GetMaximumPageAlignedTransferSize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMaximumPageAlignedTransferSize )( 
            __RPC__in IDiscRecorder2Ex * This,
            /* [retval][ref][out] */ __RPC__out ULONG *value);
        
        END_INTERFACE
    } IDiscRecorder2ExVtbl;

    interface IDiscRecorder2Ex
    {
        CONST_VTBL struct IDiscRecorder2ExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscRecorder2Ex_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscRecorder2Ex_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscRecorder2Ex_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscRecorder2Ex_SendCommandNoData(This,Cdb,CdbSize,SenseBuffer,Timeout)	\
    ( (This)->lpVtbl -> SendCommandNoData(This,Cdb,CdbSize,SenseBuffer,Timeout) ) 

#define IDiscRecorder2Ex_SendCommandSendDataToDevice(This,Cdb,CdbSize,SenseBuffer,Timeout,Buffer,BufferSize)	\
    ( (This)->lpVtbl -> SendCommandSendDataToDevice(This,Cdb,CdbSize,SenseBuffer,Timeout,Buffer,BufferSize) ) 

#define IDiscRecorder2Ex_SendCommandGetDataFromDevice(This,Cdb,CdbSize,SenseBuffer,Timeout,Buffer,BufferSize,BufferFetched)	\
    ( (This)->lpVtbl -> SendCommandGetDataFromDevice(This,Cdb,CdbSize,SenseBuffer,Timeout,Buffer,BufferSize,BufferFetched) ) 

#define IDiscRecorder2Ex_ReadDvdStructure(This,format,address,layer,agid,data,count)	\
    ( (This)->lpVtbl -> ReadDvdStructure(This,format,address,layer,agid,data,count) ) 

#define IDiscRecorder2Ex_SendDvdStructure(This,format,data,count)	\
    ( (This)->lpVtbl -> SendDvdStructure(This,format,data,count) ) 

#define IDiscRecorder2Ex_GetAdapterDescriptor(This,data,byteSize)	\
    ( (This)->lpVtbl -> GetAdapterDescriptor(This,data,byteSize) ) 

#define IDiscRecorder2Ex_GetDeviceDescriptor(This,data,byteSize)	\
    ( (This)->lpVtbl -> GetDeviceDescriptor(This,data,byteSize) ) 

#define IDiscRecorder2Ex_GetDiscInformation(This,discInformation,byteSize)	\
    ( (This)->lpVtbl -> GetDiscInformation(This,discInformation,byteSize) ) 

#define IDiscRecorder2Ex_GetTrackInformation(This,address,addressType,trackInformation,byteSize)	\
    ( (This)->lpVtbl -> GetTrackInformation(This,address,addressType,trackInformation,byteSize) ) 

#define IDiscRecorder2Ex_GetFeaturePage(This,requestedFeature,currentFeatureOnly,featureData,byteSize)	\
    ( (This)->lpVtbl -> GetFeaturePage(This,requestedFeature,currentFeatureOnly,featureData,byteSize) ) 

#define IDiscRecorder2Ex_GetModePage(This,requestedModePage,requestType,modePageData,byteSize)	\
    ( (This)->lpVtbl -> GetModePage(This,requestedModePage,requestType,modePageData,byteSize) ) 

#define IDiscRecorder2Ex_SetModePage(This,requestType,data,byteSize)	\
    ( (This)->lpVtbl -> SetModePage(This,requestType,data,byteSize) ) 

#define IDiscRecorder2Ex_GetSupportedFeaturePages(This,currentFeatureOnly,featureData,byteSize)	\
    ( (This)->lpVtbl -> GetSupportedFeaturePages(This,currentFeatureOnly,featureData,byteSize) ) 

#define IDiscRecorder2Ex_GetSupportedProfiles(This,currentOnly,profileTypes,validProfiles)	\
    ( (This)->lpVtbl -> GetSupportedProfiles(This,currentOnly,profileTypes,validProfiles) ) 

#define IDiscRecorder2Ex_GetSupportedModePages(This,requestType,modePageTypes,validPages)	\
    ( (This)->lpVtbl -> GetSupportedModePages(This,requestType,modePageTypes,validPages) ) 

#define IDiscRecorder2Ex_GetByteAlignmentMask(This,value)	\
    ( (This)->lpVtbl -> GetByteAlignmentMask(This,value) ) 

#define IDiscRecorder2Ex_GetMaximumNonPageAlignedTransferSize(This,value)	\
    ( (This)->lpVtbl -> GetMaximumNonPageAlignedTransferSize(This,value) ) 

#define IDiscRecorder2Ex_GetMaximumPageAlignedTransferSize(This,value)	\
    ( (This)->lpVtbl -> GetMaximumPageAlignedTransferSize(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscRecorder2Ex_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0003 */
/* [local] */ 

#define DISPID_IDISCRECORDER2_EJECTMEDIA             0x100
#define DISPID_IDISCRECORDER2_CLOSETRAY              0x101
#define DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS 0x102
#define DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS 0x103
#define DISPID_IDISCRECORDER2_DISABLEMCN             0x104
#define DISPID_IDISCRECORDER2_ENABLEMCN              0x105
#define DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER 0x106
#define DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER     DISPID_VALUE
#define DISPID_IDISCRECORDER2_VENDORID               0x201
#define DISPID_IDISCRECORDER2_PRODUCTID              0x202
#define DISPID_IDISCRECORDER2_PRODUCTREVISION        0x203
#define DISPID_IDISCRECORDER2_VOLUMENAME             0x204
#define DISPID_IDISCRECORDER2_VOLUMEPATHNAMES        0x205
#define DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA     0x206
#define DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER     0x207
#define DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES  0x208
#define DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES    0x209
#define DISPID_IDISCRECORDER2_SUPPORTEDPROFILES      0x20A
#define DISPID_IDISCRECORDER2_CURRENTPROFILES        0x20B
#define DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES     0x20C
#define DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER   0x20D


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0003_v0_0_s_ifspec;

#ifndef __IDiscRecorder2_INTERFACE_DEFINED__
#define __IDiscRecorder2_INTERFACE_DEFINED__

/* interface IDiscRecorder2 */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscRecorder2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354133-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscRecorder2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EjectMedia( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseTray( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AcquireExclusiveAccess( 
            /* [in] */ VARIANT_BOOL force,
            /* [in] */ __RPC__in BSTR __MIDL__IDiscRecorder20000) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseExclusiveAccess( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisableMcn( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableMcn( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeDiscRecorder( 
            /* [in] */ __RPC__in BSTR recorderUniqueId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveDiscRecorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VendorId( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductId( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductRevision( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumePathNames( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceCanLoadMedia( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LegacyDeviceNumber( 
            /* [retval][ref][out] */ __RPC__out LONG *legacyDeviceNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedFeaturePages( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentFeaturePages( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedProfiles( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentProfiles( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedModePages( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExclusiveAccessOwner( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscRecorder2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscRecorder2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscRecorder2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscRecorder2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscRecorder2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscRecorder2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, EjectMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EjectMedia )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, CloseTray)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CloseTray )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, AcquireExclusiveAccess)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AcquireExclusiveAccess )( 
            __RPC__in IDiscRecorder2 * This,
            /* [in] */ VARIANT_BOOL force,
            /* [in] */ __RPC__in BSTR __MIDL__IDiscRecorder20000);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, ReleaseExclusiveAccess)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseExclusiveAccess )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, DisableMcn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisableMcn )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, EnableMcn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableMcn )( 
            __RPC__in IDiscRecorder2 * This);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, InitializeDiscRecorder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeDiscRecorder )( 
            __RPC__in IDiscRecorder2 * This,
            /* [in] */ __RPC__in BSTR recorderUniqueId);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_ActiveDiscRecorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveDiscRecorder )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_VendorId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VendorId )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_ProductId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductId )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_ProductRevision)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductRevision )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_VolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeName )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_VolumePathNames)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumePathNames )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_DeviceCanLoadMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceCanLoadMedia )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_LegacyDeviceNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LegacyDeviceNumber )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *legacyDeviceNumber);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_SupportedFeaturePages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedFeaturePages )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_CurrentFeaturePages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentFeaturePages )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_SupportedProfiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedProfiles )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_CurrentProfiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentProfiles )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_SupportedModePages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedModePages )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscRecorder2, get_ExclusiveAccessOwner)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExclusiveAccessOwner )( 
            __RPC__in IDiscRecorder2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        END_INTERFACE
    } IDiscRecorder2Vtbl;

    interface IDiscRecorder2
    {
        CONST_VTBL struct IDiscRecorder2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscRecorder2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscRecorder2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscRecorder2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscRecorder2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscRecorder2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscRecorder2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscRecorder2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscRecorder2_EjectMedia(This)	\
    ( (This)->lpVtbl -> EjectMedia(This) ) 

#define IDiscRecorder2_CloseTray(This)	\
    ( (This)->lpVtbl -> CloseTray(This) ) 

#define IDiscRecorder2_AcquireExclusiveAccess(This,force,__MIDL__IDiscRecorder20000)	\
    ( (This)->lpVtbl -> AcquireExclusiveAccess(This,force,__MIDL__IDiscRecorder20000) ) 

#define IDiscRecorder2_ReleaseExclusiveAccess(This)	\
    ( (This)->lpVtbl -> ReleaseExclusiveAccess(This) ) 

#define IDiscRecorder2_DisableMcn(This)	\
    ( (This)->lpVtbl -> DisableMcn(This) ) 

#define IDiscRecorder2_EnableMcn(This)	\
    ( (This)->lpVtbl -> EnableMcn(This) ) 

#define IDiscRecorder2_InitializeDiscRecorder(This,recorderUniqueId)	\
    ( (This)->lpVtbl -> InitializeDiscRecorder(This,recorderUniqueId) ) 

#define IDiscRecorder2_get_ActiveDiscRecorder(This,value)	\
    ( (This)->lpVtbl -> get_ActiveDiscRecorder(This,value) ) 

#define IDiscRecorder2_get_VendorId(This,value)	\
    ( (This)->lpVtbl -> get_VendorId(This,value) ) 

#define IDiscRecorder2_get_ProductId(This,value)	\
    ( (This)->lpVtbl -> get_ProductId(This,value) ) 

#define IDiscRecorder2_get_ProductRevision(This,value)	\
    ( (This)->lpVtbl -> get_ProductRevision(This,value) ) 

#define IDiscRecorder2_get_VolumeName(This,value)	\
    ( (This)->lpVtbl -> get_VolumeName(This,value) ) 

#define IDiscRecorder2_get_VolumePathNames(This,value)	\
    ( (This)->lpVtbl -> get_VolumePathNames(This,value) ) 

#define IDiscRecorder2_get_DeviceCanLoadMedia(This,value)	\
    ( (This)->lpVtbl -> get_DeviceCanLoadMedia(This,value) ) 

#define IDiscRecorder2_get_LegacyDeviceNumber(This,legacyDeviceNumber)	\
    ( (This)->lpVtbl -> get_LegacyDeviceNumber(This,legacyDeviceNumber) ) 

#define IDiscRecorder2_get_SupportedFeaturePages(This,value)	\
    ( (This)->lpVtbl -> get_SupportedFeaturePages(This,value) ) 

#define IDiscRecorder2_get_CurrentFeaturePages(This,value)	\
    ( (This)->lpVtbl -> get_CurrentFeaturePages(This,value) ) 

#define IDiscRecorder2_get_SupportedProfiles(This,value)	\
    ( (This)->lpVtbl -> get_SupportedProfiles(This,value) ) 

#define IDiscRecorder2_get_CurrentProfiles(This,value)	\
    ( (This)->lpVtbl -> get_CurrentProfiles(This,value) ) 

#define IDiscRecorder2_get_SupportedModePages(This,value)	\
    ( (This)->lpVtbl -> get_SupportedModePages(This,value) ) 

#define IDiscRecorder2_get_ExclusiveAccessOwner(This,value)	\
    ( (This)->lpVtbl -> get_ExclusiveAccessOwner(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscRecorder2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0004 */
/* [local] */ 

#define DISPID_IWRITEENGINE2_WRITESECTION              0x200
#define DISPID_IWRITEENGINE2_CANCELWRITE               0x201
#define DISPID_IWRITEENGINE2_DISCRECORDER              0x100
#define DISPID_IWRITEENGINE2_USESTREAMINGWRITE12       0x101
#define DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND  0x102
#define DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND    0x103
#define DISPID_IWRITEENGINE2_BYTESPERSECTOR            0x104
#define DISPID_IWRITEENGINE2_WRITEINPROGRESS           0x105


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0004_v0_0_s_ifspec;

#ifndef __IWriteEngine2_INTERFACE_DEFINED__
#define __IWriteEngine2_INTERFACE_DEFINED__

/* interface IWriteEngine2 */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IWriteEngine2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354135-7F64-5B0F-8F00-5D77AFBE261E")
    IWriteEngine2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WriteSection( 
            /* [in] */ __RPC__in_opt IStream *data,
            /* [in] */ LONG startingBlockAddress,
            /* [in] */ LONG numberOfBlocks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelWrite( void) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recorder( 
            /* [in] */ __RPC__in_opt IDiscRecorder2Ex *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2Ex **value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseStreamingWrite12( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseStreamingWrite12( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartingSectorsPerSecond( 
            /* [in] */ LONG value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartingSectorsPerSecond( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EndingSectorsPerSecond( 
            /* [in] */ LONG value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EndingSectorsPerSecond( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BytesPerSector( 
            /* [in] */ LONG value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BytesPerSector( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WriteInProgress( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWriteEngine2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWriteEngine2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWriteEngine2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWriteEngine2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWriteEngine2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, WriteSection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WriteSection )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ __RPC__in_opt IStream *data,
            /* [in] */ LONG startingBlockAddress,
            /* [in] */ LONG numberOfBlocks);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, CancelWrite)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelWrite )( 
            __RPC__in IWriteEngine2 * This);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, put_Recorder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recorder )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2Ex *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_Recorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recorder )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2Ex **value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, put_UseStreamingWrite12)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseStreamingWrite12 )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_UseStreamingWrite12)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseStreamingWrite12 )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, put_StartingSectorsPerSecond)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartingSectorsPerSecond )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ LONG value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_StartingSectorsPerSecond)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartingSectorsPerSecond )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, put_EndingSectorsPerSecond)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndingSectorsPerSecond )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ LONG value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_EndingSectorsPerSecond)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndingSectorsPerSecond )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, put_BytesPerSector)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BytesPerSector )( 
            __RPC__in IWriteEngine2 * This,
            /* [in] */ LONG value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_BytesPerSector)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BytesPerSector )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2, get_WriteInProgress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteInProgress )( 
            __RPC__in IWriteEngine2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        END_INTERFACE
    } IWriteEngine2Vtbl;

    interface IWriteEngine2
    {
        CONST_VTBL struct IWriteEngine2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWriteEngine2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWriteEngine2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWriteEngine2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWriteEngine2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWriteEngine2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWriteEngine2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWriteEngine2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWriteEngine2_WriteSection(This,data,startingBlockAddress,numberOfBlocks)	\
    ( (This)->lpVtbl -> WriteSection(This,data,startingBlockAddress,numberOfBlocks) ) 

#define IWriteEngine2_CancelWrite(This)	\
    ( (This)->lpVtbl -> CancelWrite(This) ) 

#define IWriteEngine2_put_Recorder(This,value)	\
    ( (This)->lpVtbl -> put_Recorder(This,value) ) 

#define IWriteEngine2_get_Recorder(This,value)	\
    ( (This)->lpVtbl -> get_Recorder(This,value) ) 

#define IWriteEngine2_put_UseStreamingWrite12(This,value)	\
    ( (This)->lpVtbl -> put_UseStreamingWrite12(This,value) ) 

#define IWriteEngine2_get_UseStreamingWrite12(This,value)	\
    ( (This)->lpVtbl -> get_UseStreamingWrite12(This,value) ) 

#define IWriteEngine2_put_StartingSectorsPerSecond(This,value)	\
    ( (This)->lpVtbl -> put_StartingSectorsPerSecond(This,value) ) 

#define IWriteEngine2_get_StartingSectorsPerSecond(This,value)	\
    ( (This)->lpVtbl -> get_StartingSectorsPerSecond(This,value) ) 

#define IWriteEngine2_put_EndingSectorsPerSecond(This,value)	\
    ( (This)->lpVtbl -> put_EndingSectorsPerSecond(This,value) ) 

#define IWriteEngine2_get_EndingSectorsPerSecond(This,value)	\
    ( (This)->lpVtbl -> get_EndingSectorsPerSecond(This,value) ) 

#define IWriteEngine2_put_BytesPerSector(This,value)	\
    ( (This)->lpVtbl -> put_BytesPerSector(This,value) ) 

#define IWriteEngine2_get_BytesPerSector(This,value)	\
    ( (This)->lpVtbl -> get_BytesPerSector(This,value) ) 

#define IWriteEngine2_get_WriteInProgress(This,value)	\
    ( (This)->lpVtbl -> get_WriteInProgress(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWriteEngine2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0005 */
/* [local] */ 

#define DISPID_IWRITEENGINE2EVENTARGS_STARTLBA             0x100
#define DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT          0x101
#define DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA          0x102
#define DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA       0x103
#define DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER    0x104
#define DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER     0x105
#define DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER    0x106
#define DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER     0x107
#define DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER     0x108


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0005_v0_0_s_ifspec;

#ifndef __IWriteEngine2EventArgs_INTERFACE_DEFINED__
#define __IWriteEngine2EventArgs_INTERFACE_DEFINED__

/* interface IWriteEngine2EventArgs */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IWriteEngine2EventArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354136-7F64-5B0F-8F00-5D77AFBE261E")
    IWriteEngine2EventArgs : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SectorCount( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastReadLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastWrittenLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalSystemBuffer( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsedSystemBuffer( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeSystemBuffer( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWriteEngine2EventArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWriteEngine2EventArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWriteEngine2EventArgs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWriteEngine2EventArgs * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_StartLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartLba )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_SectorCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorCount )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastReadLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastReadLba )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastWrittenLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenLba )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_TotalSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSystemBuffer )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_UsedSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedSystemBuffer )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_FreeSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSystemBuffer )( 
            __RPC__in IWriteEngine2EventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IWriteEngine2EventArgsVtbl;

    interface IWriteEngine2EventArgs
    {
        CONST_VTBL struct IWriteEngine2EventArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWriteEngine2EventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWriteEngine2EventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWriteEngine2EventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWriteEngine2EventArgs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWriteEngine2EventArgs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWriteEngine2EventArgs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWriteEngine2EventArgs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWriteEngine2EventArgs_get_StartLba(This,value)	\
    ( (This)->lpVtbl -> get_StartLba(This,value) ) 

#define IWriteEngine2EventArgs_get_SectorCount(This,value)	\
    ( (This)->lpVtbl -> get_SectorCount(This,value) ) 

#define IWriteEngine2EventArgs_get_LastReadLba(This,value)	\
    ( (This)->lpVtbl -> get_LastReadLba(This,value) ) 

#define IWriteEngine2EventArgs_get_LastWrittenLba(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenLba(This,value) ) 

#define IWriteEngine2EventArgs_get_TotalSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_TotalSystemBuffer(This,value) ) 

#define IWriteEngine2EventArgs_get_UsedSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_UsedSystemBuffer(This,value) ) 

#define IWriteEngine2EventArgs_get_FreeSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_FreeSystemBuffer(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWriteEngine2EventArgs_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0006 */
/* [local] */ 

#define DISPID_DWRITEENGINE2EVENTS_UPDATE 0x100


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0006_v0_0_s_ifspec;

#ifndef __DWriteEngine2Events_INTERFACE_DEFINED__
#define __DWriteEngine2Events_INTERFACE_DEFINED__

/* interface DWriteEngine2Events */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DWriteEngine2Events;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354137-7F64-5B0F-8F00-5D77AFBE261E")
    DWriteEngine2Events : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DWriteEngine2EventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DWriteEngine2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DWriteEngine2Events * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DWriteEngine2Events * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DWriteEngine2Events * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DWriteEngine2Events * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DWriteEngine2Events * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DWriteEngine2Events * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DWriteEngine2Events, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DWriteEngine2Events * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress);
        
        END_INTERFACE
    } DWriteEngine2EventsVtbl;

    interface DWriteEngine2Events
    {
        CONST_VTBL struct DWriteEngine2EventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DWriteEngine2Events_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DWriteEngine2Events_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DWriteEngine2Events_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DWriteEngine2Events_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DWriteEngine2Events_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DWriteEngine2Events_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DWriteEngine2Events_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DWriteEngine2Events_Update(This,object,progress)	\
    ( (This)->lpVtbl -> Update(This,object,progress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DWriteEngine2Events_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0007 */
/* [local] */ 

#define DISPID_IDISCFORMAT2_RECORDERSUPPORTED        0x800
#define DISPID_IDISCFORMAT2_MEDIASUPPORTED           0x801
#define DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK     0x700
#define DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK  0x701
#define DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES      0x702


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0007_v0_0_s_ifspec;

#ifndef __IDiscFormat2_INTERFACE_DEFINED__
#define __IDiscFormat2_INTERFACE_DEFINED__

/* interface IDiscFormat2 */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354152-8F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsRecorderSupported( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsCurrentMediaSupported( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaPhysicallyBlank( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaHeuristicallyBlank( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedMediaTypes( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsRecorderSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsRecorderSupported )( 
            __RPC__in IDiscFormat2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsCurrentMediaSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsCurrentMediaSupported )( 
            __RPC__in IDiscFormat2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaPhysicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaPhysicallyBlank )( 
            __RPC__in IDiscFormat2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaHeuristicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaHeuristicallyBlank )( 
            __RPC__in IDiscFormat2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_SupportedMediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedMediaTypes )( 
            __RPC__in IDiscFormat2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        END_INTERFACE
    } IDiscFormat2Vtbl;

    interface IDiscFormat2
    {
        CONST_VTBL struct IDiscFormat2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2_IsRecorderSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsRecorderSupported(This,recorder,value) ) 

#define IDiscFormat2_IsCurrentMediaSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsCurrentMediaSupported(This,recorder,value) ) 

#define IDiscFormat2_get_MediaPhysicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaPhysicallyBlank(This,value) ) 

#define IDiscFormat2_get_MediaHeuristicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaHeuristicallyBlank(This,value) ) 

#define IDiscFormat2_get_SupportedMediaTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedMediaTypes(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0008 */
/* [local] */ 

#define DISPID_IDISCFORMAT2ERASE_RECORDER            0x100
#define DISPID_IDISCFORMAT2ERASE_FULLERASE           0x101
#define DISPID_IDISCFORMAT2ERASE_MEDIATYPE           0x102
#define DISPID_IDISCFORMAT2ERASE_CLIENTNAME          0x103
#define DISPID_IDISCFORMAT2ERASE_ERASEMEDIA          0x201


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0008_v0_0_s_ifspec;

#ifndef __IDiscFormat2Erase_INTERFACE_DEFINED__
#define __IDiscFormat2Erase_INTERFACE_DEFINED__

/* interface IDiscFormat2Erase */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2Erase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354156-8F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2Erase : public IDiscFormat2
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recorder( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FullErase( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FullErase( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPhysicalMediaType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ClientName( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EraseMedia( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2EraseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2Erase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2Erase * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2Erase * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsRecorderSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsRecorderSupported )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsCurrentMediaSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsCurrentMediaSupported )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaPhysicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaPhysicallyBlank )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaHeuristicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaHeuristicallyBlank )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_SupportedMediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedMediaTypes )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, put_Recorder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recorder )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, get_Recorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recorder )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, put_FullErase)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FullErase )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, get_FullErase)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullErase )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, get_CurrentPhysicalMediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPhysicalMediaType )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, put_ClientName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientName )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, get_ClientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IDiscFormat2Erase * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Erase, EraseMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EraseMedia )( 
            __RPC__in IDiscFormat2Erase * This);
        
        END_INTERFACE
    } IDiscFormat2EraseVtbl;

    interface IDiscFormat2Erase
    {
        CONST_VTBL struct IDiscFormat2EraseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2Erase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2Erase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2Erase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2Erase_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2Erase_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2Erase_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2Erase_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2Erase_IsRecorderSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsRecorderSupported(This,recorder,value) ) 

#define IDiscFormat2Erase_IsCurrentMediaSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsCurrentMediaSupported(This,recorder,value) ) 

#define IDiscFormat2Erase_get_MediaPhysicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaPhysicallyBlank(This,value) ) 

#define IDiscFormat2Erase_get_MediaHeuristicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaHeuristicallyBlank(This,value) ) 

#define IDiscFormat2Erase_get_SupportedMediaTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedMediaTypes(This,value) ) 


#define IDiscFormat2Erase_put_Recorder(This,value)	\
    ( (This)->lpVtbl -> put_Recorder(This,value) ) 

#define IDiscFormat2Erase_get_Recorder(This,value)	\
    ( (This)->lpVtbl -> get_Recorder(This,value) ) 

#define IDiscFormat2Erase_put_FullErase(This,value)	\
    ( (This)->lpVtbl -> put_FullErase(This,value) ) 

#define IDiscFormat2Erase_get_FullErase(This,value)	\
    ( (This)->lpVtbl -> get_FullErase(This,value) ) 

#define IDiscFormat2Erase_get_CurrentPhysicalMediaType(This,value)	\
    ( (This)->lpVtbl -> get_CurrentPhysicalMediaType(This,value) ) 

#define IDiscFormat2Erase_put_ClientName(This,value)	\
    ( (This)->lpVtbl -> put_ClientName(This,value) ) 

#define IDiscFormat2Erase_get_ClientName(This,value)	\
    ( (This)->lpVtbl -> get_ClientName(This,value) ) 

#define IDiscFormat2Erase_EraseMedia(This)	\
    ( (This)->lpVtbl -> EraseMedia(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2Erase_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0009 */
/* [local] */ 

#define DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE 0x200


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0009_v0_0_s_ifspec;

#ifndef __DDiscFormat2EraseEvents_INTERFACE_DEFINED__
#define __DDiscFormat2EraseEvents_INTERFACE_DEFINED__

/* interface DDiscFormat2EraseEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DDiscFormat2EraseEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2735413A-7F64-5B0F-8F00-5D77AFBE261E")
    DDiscFormat2EraseEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ LONG elapsedSeconds,
            /* [in] */ LONG estimatedTotalSeconds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DDiscFormat2EraseEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DDiscFormat2EraseEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DDiscFormat2EraseEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DDiscFormat2EraseEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DDiscFormat2EraseEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DDiscFormat2EraseEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DDiscFormat2EraseEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DDiscFormat2EraseEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DDiscFormat2EraseEvents, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DDiscFormat2EraseEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ LONG elapsedSeconds,
            /* [in] */ LONG estimatedTotalSeconds);
        
        END_INTERFACE
    } DDiscFormat2EraseEventsVtbl;

    interface DDiscFormat2EraseEvents
    {
        CONST_VTBL struct DDiscFormat2EraseEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DDiscFormat2EraseEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DDiscFormat2EraseEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DDiscFormat2EraseEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DDiscFormat2EraseEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DDiscFormat2EraseEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DDiscFormat2EraseEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DDiscFormat2EraseEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DDiscFormat2EraseEvents_Update(This,object,elapsedSeconds,estimatedTotalSeconds)	\
    ( (This)->lpVtbl -> Update(This,object,elapsedSeconds,estimatedTotalSeconds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DDiscFormat2EraseEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0010 */
/* [local] */ 

#define DISPID_IDISCFORMAT2DATA_RECORDER                         0x100
#define DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED       0x101
#define DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE            0x104
#define DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS               0x106
#define DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS               0x107
#define DISPID_IDISCFORMAT2DATA_TOTALSECTORS                     0x108
#define DISPID_IDISCFORMAT2DATA_FREESECTORS                      0x109
#define DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS              0x10A
#define DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION     0x10B
#define DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION      0x10C
#define DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED             0x10D
#define DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE      0x10E
#define DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE                 0x10F
#define DISPID_IDISCFORMAT2DATA_CLIENTNAME                       0x110
#define DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED              0x111
#define DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV   0x112
#define DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED                0x113
#define DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV     0x114
#define DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS             0x115
#define DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS   0x116
#define DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE                   0x117
#define DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES           0x118
#define DISPID_IDISCFORMAT2DATA_WRITE                            0x200
#define DISPID_IDISCFORMAT2DATA_CANCELWRITE                      0x201
#define DISPID_IDISCFORMAT2DATA_SETWRITESPEED                    0x202


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0010_v0_0_s_ifspec;

#ifndef __IDiscFormat2Data_INTERFACE_DEFINED__
#define __IDiscFormat2Data_INTERFACE_DEFINED__

/* interface IDiscFormat2Data */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2Data;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354153-9F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2Data : public IDiscFormat2
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recorder( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BufferUnderrunFreeDisabled( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BufferUnderrunFreeDisabled( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PostgapAlreadyInImage( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PostgapAlreadyInImage( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentMediaStatus( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_DATA_MEDIA_STATE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WriteProtectStatus( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_WRITE_PROTECT_STATE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NextWritableAddress( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartAddressOfPreviousSession( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastWrittenAddressOfPreviousSession( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ForceMediaToBeClosed( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForceMediaToBeClosed( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisableConsumerDvdCompatibilityMode( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisableConsumerDvdCompatibilityMode( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPhysicalMediaType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ClientName( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeeds( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeedDescriptors( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ForceOverwrite( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForceOverwrite( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MultisessionInterfaces( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ __RPC__in_opt IStream *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelWrite( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWriteSpeed( 
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2DataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2Data * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2Data * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2Data * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2Data * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsRecorderSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsRecorderSupported )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsCurrentMediaSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsCurrentMediaSupported )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaPhysicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaPhysicallyBlank )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaHeuristicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaHeuristicallyBlank )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_SupportedMediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedMediaTypes )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_Recorder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recorder )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_Recorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recorder )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_PostgapAlreadyInImage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostgapAlreadyInImage )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_PostgapAlreadyInImage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostgapAlreadyInImage )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_CurrentMediaStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentMediaStatus )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_DATA_MEDIA_STATE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_WriteProtectStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteProtectStatus )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_WRITE_PROTECT_STATE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_TotalSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSectorsOnMedia )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_FreeSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSectorsOnMedia )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_NextWritableAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NextWritableAddress )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_StartAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartAddressOfPreviousSession )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_LastWrittenAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenAddressOfPreviousSession )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_ForceMediaToBeClosed)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ForceMediaToBeClosed )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_ForceMediaToBeClosed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForceMediaToBeClosed )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_DisableConsumerDvdCompatibilityMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisableConsumerDvdCompatibilityMode )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_DisableConsumerDvdCompatibilityMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisableConsumerDvdCompatibilityMode )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_CurrentPhysicalMediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPhysicalMediaType )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_ClientName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientName )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_ClientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_RequestedWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedWriteSpeed )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_RequestedRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_CurrentWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentWriteSpeed )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_CurrentRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_SupportedWriteSpeeds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeeds )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_SupportedWriteSpeedDescriptors)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeedDescriptors )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, put_ForceOverwrite)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ForceOverwrite )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_ForceOverwrite)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForceOverwrite )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, get_MultisessionInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultisessionInterfaces )( 
            __RPC__in IDiscFormat2Data * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, Write)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ __RPC__in_opt IStream *data);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, CancelWrite)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelWrite )( 
            __RPC__in IDiscFormat2Data * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2Data, SetWriteSpeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWriteSpeed )( 
            __RPC__in IDiscFormat2Data * This,
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV);
        
        END_INTERFACE
    } IDiscFormat2DataVtbl;

    interface IDiscFormat2Data
    {
        CONST_VTBL struct IDiscFormat2DataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2Data_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2Data_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2Data_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2Data_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2Data_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2Data_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2Data_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2Data_IsRecorderSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsRecorderSupported(This,recorder,value) ) 

#define IDiscFormat2Data_IsCurrentMediaSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsCurrentMediaSupported(This,recorder,value) ) 

#define IDiscFormat2Data_get_MediaPhysicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaPhysicallyBlank(This,value) ) 

#define IDiscFormat2Data_get_MediaHeuristicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaHeuristicallyBlank(This,value) ) 

#define IDiscFormat2Data_get_SupportedMediaTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedMediaTypes(This,value) ) 


#define IDiscFormat2Data_put_Recorder(This,value)	\
    ( (This)->lpVtbl -> put_Recorder(This,value) ) 

#define IDiscFormat2Data_get_Recorder(This,value)	\
    ( (This)->lpVtbl -> get_Recorder(This,value) ) 

#define IDiscFormat2Data_put_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> put_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2Data_get_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> get_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2Data_put_PostgapAlreadyInImage(This,value)	\
    ( (This)->lpVtbl -> put_PostgapAlreadyInImage(This,value) ) 

#define IDiscFormat2Data_get_PostgapAlreadyInImage(This,value)	\
    ( (This)->lpVtbl -> get_PostgapAlreadyInImage(This,value) ) 

#define IDiscFormat2Data_get_CurrentMediaStatus(This,value)	\
    ( (This)->lpVtbl -> get_CurrentMediaStatus(This,value) ) 

#define IDiscFormat2Data_get_WriteProtectStatus(This,value)	\
    ( (This)->lpVtbl -> get_WriteProtectStatus(This,value) ) 

#define IDiscFormat2Data_get_TotalSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_TotalSectorsOnMedia(This,value) ) 

#define IDiscFormat2Data_get_FreeSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_FreeSectorsOnMedia(This,value) ) 

#define IDiscFormat2Data_get_NextWritableAddress(This,value)	\
    ( (This)->lpVtbl -> get_NextWritableAddress(This,value) ) 

#define IDiscFormat2Data_get_StartAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_StartAddressOfPreviousSession(This,value) ) 

#define IDiscFormat2Data_get_LastWrittenAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenAddressOfPreviousSession(This,value) ) 

#define IDiscFormat2Data_put_ForceMediaToBeClosed(This,value)	\
    ( (This)->lpVtbl -> put_ForceMediaToBeClosed(This,value) ) 

#define IDiscFormat2Data_get_ForceMediaToBeClosed(This,value)	\
    ( (This)->lpVtbl -> get_ForceMediaToBeClosed(This,value) ) 

#define IDiscFormat2Data_put_DisableConsumerDvdCompatibilityMode(This,value)	\
    ( (This)->lpVtbl -> put_DisableConsumerDvdCompatibilityMode(This,value) ) 

#define IDiscFormat2Data_get_DisableConsumerDvdCompatibilityMode(This,value)	\
    ( (This)->lpVtbl -> get_DisableConsumerDvdCompatibilityMode(This,value) ) 

#define IDiscFormat2Data_get_CurrentPhysicalMediaType(This,value)	\
    ( (This)->lpVtbl -> get_CurrentPhysicalMediaType(This,value) ) 

#define IDiscFormat2Data_put_ClientName(This,value)	\
    ( (This)->lpVtbl -> put_ClientName(This,value) ) 

#define IDiscFormat2Data_get_ClientName(This,value)	\
    ( (This)->lpVtbl -> get_ClientName(This,value) ) 

#define IDiscFormat2Data_get_RequestedWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_RequestedWriteSpeed(This,value) ) 

#define IDiscFormat2Data_get_RequestedRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_RequestedRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2Data_get_CurrentWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_CurrentWriteSpeed(This,value) ) 

#define IDiscFormat2Data_get_CurrentRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_CurrentRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2Data_get_SupportedWriteSpeeds(This,supportedSpeeds)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeeds(This,supportedSpeeds) ) 

#define IDiscFormat2Data_get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors) ) 

#define IDiscFormat2Data_put_ForceOverwrite(This,value)	\
    ( (This)->lpVtbl -> put_ForceOverwrite(This,value) ) 

#define IDiscFormat2Data_get_ForceOverwrite(This,value)	\
    ( (This)->lpVtbl -> get_ForceOverwrite(This,value) ) 

#define IDiscFormat2Data_get_MultisessionInterfaces(This,value)	\
    ( (This)->lpVtbl -> get_MultisessionInterfaces(This,value) ) 

#define IDiscFormat2Data_Write(This,data)	\
    ( (This)->lpVtbl -> Write(This,data) ) 

#define IDiscFormat2Data_CancelWrite(This)	\
    ( (This)->lpVtbl -> CancelWrite(This) ) 

#define IDiscFormat2Data_SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV)	\
    ( (This)->lpVtbl -> SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2Data_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0011 */
/* [local] */ 

#define DISPID_DDISCFORMAT2DATAEVENTS_UPDATE  0x200


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0011_v0_0_s_ifspec;

#ifndef __DDiscFormat2DataEvents_INTERFACE_DEFINED__
#define __DDiscFormat2DataEvents_INTERFACE_DEFINED__

/* interface DDiscFormat2DataEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DDiscFormat2DataEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2735413C-7F64-5B0F-8F00-5D77AFBE261E")
    DDiscFormat2DataEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DDiscFormat2DataEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DDiscFormat2DataEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DDiscFormat2DataEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DDiscFormat2DataEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DDiscFormat2DataEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DDiscFormat2DataEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DDiscFormat2DataEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DDiscFormat2DataEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DDiscFormat2DataEvents, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DDiscFormat2DataEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress);
        
        END_INTERFACE
    } DDiscFormat2DataEventsVtbl;

    interface DDiscFormat2DataEvents
    {
        CONST_VTBL struct DDiscFormat2DataEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DDiscFormat2DataEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DDiscFormat2DataEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DDiscFormat2DataEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DDiscFormat2DataEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DDiscFormat2DataEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DDiscFormat2DataEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DDiscFormat2DataEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DDiscFormat2DataEvents_Update(This,object,progress)	\
    ( (This)->lpVtbl -> Update(This,object,progress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DDiscFormat2DataEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0012 */
/* [local] */ 

#define DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME            0x300
#define DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME 0x301
#define DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME     0x302
#define DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION          0x303


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0012_v0_0_s_ifspec;

#ifndef __IDiscFormat2DataEventArgs_INTERFACE_DEFINED__
#define __IDiscFormat2DataEventArgs_INTERFACE_DEFINED__

/* interface IDiscFormat2DataEventArgs */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2DataEventArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2735413D-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2DataEventArgs : public IWriteEngine2EventArgs
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ElapsedTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemainingTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentAction( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_DATA_WRITE_ACTION *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2DataEventArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2DataEventArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2DataEventArgs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2DataEventArgs * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_StartLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartLba )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_SectorCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorCount )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastReadLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastReadLba )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastWrittenLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenLba )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_TotalSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSystemBuffer )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_UsedSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedSystemBuffer )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_FreeSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSystemBuffer )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2DataEventArgs, get_ElapsedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ElapsedTime )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2DataEventArgs, get_RemainingTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemainingTime )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2DataEventArgs, get_TotalTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalTime )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2DataEventArgs, get_CurrentAction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentAction )( 
            __RPC__in IDiscFormat2DataEventArgs * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_DATA_WRITE_ACTION *value);
        
        END_INTERFACE
    } IDiscFormat2DataEventArgsVtbl;

    interface IDiscFormat2DataEventArgs
    {
        CONST_VTBL struct IDiscFormat2DataEventArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2DataEventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2DataEventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2DataEventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2DataEventArgs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2DataEventArgs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2DataEventArgs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2DataEventArgs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2DataEventArgs_get_StartLba(This,value)	\
    ( (This)->lpVtbl -> get_StartLba(This,value) ) 

#define IDiscFormat2DataEventArgs_get_SectorCount(This,value)	\
    ( (This)->lpVtbl -> get_SectorCount(This,value) ) 

#define IDiscFormat2DataEventArgs_get_LastReadLba(This,value)	\
    ( (This)->lpVtbl -> get_LastReadLba(This,value) ) 

#define IDiscFormat2DataEventArgs_get_LastWrittenLba(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenLba(This,value) ) 

#define IDiscFormat2DataEventArgs_get_TotalSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_TotalSystemBuffer(This,value) ) 

#define IDiscFormat2DataEventArgs_get_UsedSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_UsedSystemBuffer(This,value) ) 

#define IDiscFormat2DataEventArgs_get_FreeSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_FreeSystemBuffer(This,value) ) 


#define IDiscFormat2DataEventArgs_get_ElapsedTime(This,value)	\
    ( (This)->lpVtbl -> get_ElapsedTime(This,value) ) 

#define IDiscFormat2DataEventArgs_get_RemainingTime(This,value)	\
    ( (This)->lpVtbl -> get_RemainingTime(This,value) ) 

#define IDiscFormat2DataEventArgs_get_TotalTime(This,value)	\
    ( (This)->lpVtbl -> get_TotalTime(This,value) ) 

#define IDiscFormat2DataEventArgs_get_CurrentAction(This,value)	\
    ( (This)->lpVtbl -> get_CurrentAction(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2DataEventArgs_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0013 */
/* [local] */ 

#define DISPID_IDISCFORMAT2TAO_RECORDER                          0x100
#define DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED        0x102
#define DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS            0x103
#define DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA               0x104
#define DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA                0x105
#define DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA                0x106
#define DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA                0x107
#define DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS           0x10A
#define DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE                  0x10B
#define DISPID_IDISCFORMAT2TAO_CLIENTNAME                        0x10E
#define DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED               0x10F
#define DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV    0x110
#define DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED                 0x111
#define DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV      0x112
#define DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS              0x113
#define DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS    0x114
#define DISPID_IDISCFORMAT2TAO_PREPAREMEDIA                      0x200
#define DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK                     0x201
#define DISPID_IDISCFORMAT2TAO_CANCELADDTRACK                    0x202
#define DISPID_IDISCFORMAT2TAO_FINISHMEDIA                       0x203
#define DISPID_IDISCFORMAT2TAO_SETWRITESPEED                     0x204


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0013_v0_0_s_ifspec;

#ifndef __IDiscFormat2TrackAtOnce_INTERFACE_DEFINED__
#define __IDiscFormat2TrackAtOnce_INTERFACE_DEFINED__

/* interface IDiscFormat2TrackAtOnce */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2TrackAtOnce;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354154-8F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2TrackAtOnce : public IDiscFormat2
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PrepareMedia( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddAudioTrack( 
            /* [in] */ __RPC__in_opt IStream *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelAddTrack( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseMedia( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWriteSpeed( 
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recorder( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BufferUnderrunFreeDisabled( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BufferUnderrunFreeDisabled( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfExistingTracks( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsedSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DoNotFinalizeMedia( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DoNotFinalizeMedia( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExpectedTableOfContents( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPhysicalMediaType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ClientName( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeeds( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeedDescriptors( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2TrackAtOnceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2TrackAtOnce * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2TrackAtOnce * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2TrackAtOnce * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsRecorderSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsRecorderSupported )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsCurrentMediaSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsCurrentMediaSupported )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaPhysicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaPhysicallyBlank )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaHeuristicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaHeuristicallyBlank )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_SupportedMediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedMediaTypes )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, PrepareMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PrepareMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, AddAudioTrack)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddAudioTrack )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in_opt IStream *data);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, CancelAddTrack)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelAddTrack )( 
            __RPC__in IDiscFormat2TrackAtOnce * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, ReleaseMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, SetWriteSpeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWriteSpeed )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, put_Recorder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recorder )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_Recorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recorder )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, put_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_NumberOfExistingTracks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfExistingTracks )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_TotalSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSectorsOnMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_FreeSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSectorsOnMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_UsedSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedSectorsOnMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, put_DoNotFinalizeMedia)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DoNotFinalizeMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_DoNotFinalizeMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotFinalizeMedia )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_ExpectedTableOfContents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpectedTableOfContents )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_CurrentPhysicalMediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPhysicalMediaType )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, put_ClientName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientName )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_ClientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_RequestedWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedWriteSpeed )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_RequestedRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_CurrentWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentWriteSpeed )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_CurrentRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_SupportedWriteSpeeds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeeds )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnce, get_SupportedWriteSpeedDescriptors)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeedDescriptors )( 
            __RPC__in IDiscFormat2TrackAtOnce * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors);
        
        END_INTERFACE
    } IDiscFormat2TrackAtOnceVtbl;

    interface IDiscFormat2TrackAtOnce
    {
        CONST_VTBL struct IDiscFormat2TrackAtOnceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2TrackAtOnce_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2TrackAtOnce_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2TrackAtOnce_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2TrackAtOnce_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2TrackAtOnce_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2TrackAtOnce_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2TrackAtOnce_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2TrackAtOnce_IsRecorderSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsRecorderSupported(This,recorder,value) ) 

#define IDiscFormat2TrackAtOnce_IsCurrentMediaSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsCurrentMediaSupported(This,recorder,value) ) 

#define IDiscFormat2TrackAtOnce_get_MediaPhysicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaPhysicallyBlank(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_MediaHeuristicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaHeuristicallyBlank(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_SupportedMediaTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedMediaTypes(This,value) ) 


#define IDiscFormat2TrackAtOnce_PrepareMedia(This)	\
    ( (This)->lpVtbl -> PrepareMedia(This) ) 

#define IDiscFormat2TrackAtOnce_AddAudioTrack(This,data)	\
    ( (This)->lpVtbl -> AddAudioTrack(This,data) ) 

#define IDiscFormat2TrackAtOnce_CancelAddTrack(This)	\
    ( (This)->lpVtbl -> CancelAddTrack(This) ) 

#define IDiscFormat2TrackAtOnce_ReleaseMedia(This)	\
    ( (This)->lpVtbl -> ReleaseMedia(This) ) 

#define IDiscFormat2TrackAtOnce_SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV)	\
    ( (This)->lpVtbl -> SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV) ) 

#define IDiscFormat2TrackAtOnce_put_Recorder(This,value)	\
    ( (This)->lpVtbl -> put_Recorder(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_Recorder(This,value)	\
    ( (This)->lpVtbl -> get_Recorder(This,value) ) 

#define IDiscFormat2TrackAtOnce_put_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> put_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> get_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_NumberOfExistingTracks(This,value)	\
    ( (This)->lpVtbl -> get_NumberOfExistingTracks(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_TotalSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_TotalSectorsOnMedia(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_FreeSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_FreeSectorsOnMedia(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_UsedSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_UsedSectorsOnMedia(This,value) ) 

#define IDiscFormat2TrackAtOnce_put_DoNotFinalizeMedia(This,value)	\
    ( (This)->lpVtbl -> put_DoNotFinalizeMedia(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_DoNotFinalizeMedia(This,value)	\
    ( (This)->lpVtbl -> get_DoNotFinalizeMedia(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_ExpectedTableOfContents(This,value)	\
    ( (This)->lpVtbl -> get_ExpectedTableOfContents(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_CurrentPhysicalMediaType(This,value)	\
    ( (This)->lpVtbl -> get_CurrentPhysicalMediaType(This,value) ) 

#define IDiscFormat2TrackAtOnce_put_ClientName(This,value)	\
    ( (This)->lpVtbl -> put_ClientName(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_ClientName(This,value)	\
    ( (This)->lpVtbl -> get_ClientName(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_RequestedWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_RequestedWriteSpeed(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_RequestedRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_RequestedRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_CurrentWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_CurrentWriteSpeed(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_CurrentRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_CurrentRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2TrackAtOnce_get_SupportedWriteSpeeds(This,supportedSpeeds)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeeds(This,supportedSpeeds) ) 

#define IDiscFormat2TrackAtOnce_get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2TrackAtOnce_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0014 */
/* [local] */ 

#define DISPID_DDISCFORMAT2TAOEVENTS_UPDATE  0x200


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0014_v0_0_s_ifspec;

#ifndef __DDiscFormat2TrackAtOnceEvents_INTERFACE_DEFINED__
#define __DDiscFormat2TrackAtOnceEvents_INTERFACE_DEFINED__

/* interface DDiscFormat2TrackAtOnceEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DDiscFormat2TrackAtOnceEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2735413F-7F64-5B0F-8F00-5D77AFBE261E")
    DDiscFormat2TrackAtOnceEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DDiscFormat2TrackAtOnceEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DDiscFormat2TrackAtOnceEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DDiscFormat2TrackAtOnceEvents, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DDiscFormat2TrackAtOnceEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress);
        
        END_INTERFACE
    } DDiscFormat2TrackAtOnceEventsVtbl;

    interface DDiscFormat2TrackAtOnceEvents
    {
        CONST_VTBL struct DDiscFormat2TrackAtOnceEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DDiscFormat2TrackAtOnceEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DDiscFormat2TrackAtOnceEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DDiscFormat2TrackAtOnceEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DDiscFormat2TrackAtOnceEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DDiscFormat2TrackAtOnceEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DDiscFormat2TrackAtOnceEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DDiscFormat2TrackAtOnceEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DDiscFormat2TrackAtOnceEvents_Update(This,object,progress)	\
    ( (This)->lpVtbl -> Update(This,object,progress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DDiscFormat2TrackAtOnceEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0015 */
/* [local] */ 

#define DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER     0x300
#define DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION          0x301
#define DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME            0x302
#define DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME 0x303
#define DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME     0x304


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0015_v0_0_s_ifspec;

#ifndef __IDiscFormat2TrackAtOnceEventArgs_INTERFACE_DEFINED__
#define __IDiscFormat2TrackAtOnceEventArgs_INTERFACE_DEFINED__

/* interface IDiscFormat2TrackAtOnceEventArgs */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2TrackAtOnceEventArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354140-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2TrackAtOnceEventArgs : public IWriteEngine2EventArgs
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentTrackNumber( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentAction( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_TAO_WRITE_ACTION *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ElapsedTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemainingTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2TrackAtOnceEventArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2TrackAtOnceEventArgs * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_StartLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartLba )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_SectorCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorCount )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastReadLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastReadLba )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastWrittenLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenLba )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_TotalSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSystemBuffer )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_UsedSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedSystemBuffer )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_FreeSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSystemBuffer )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnceEventArgs, get_CurrentTrackNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentTrackNumber )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnceEventArgs, get_CurrentAction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentAction )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_TAO_WRITE_ACTION *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnceEventArgs, get_ElapsedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ElapsedTime )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2TrackAtOnceEventArgs, get_RemainingTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemainingTime )( 
            __RPC__in IDiscFormat2TrackAtOnceEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IDiscFormat2TrackAtOnceEventArgsVtbl;

    interface IDiscFormat2TrackAtOnceEventArgs
    {
        CONST_VTBL struct IDiscFormat2TrackAtOnceEventArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2TrackAtOnceEventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2TrackAtOnceEventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2TrackAtOnceEventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2TrackAtOnceEventArgs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2TrackAtOnceEventArgs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2TrackAtOnceEventArgs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2TrackAtOnceEventArgs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2TrackAtOnceEventArgs_get_StartLba(This,value)	\
    ( (This)->lpVtbl -> get_StartLba(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_SectorCount(This,value)	\
    ( (This)->lpVtbl -> get_SectorCount(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_LastReadLba(This,value)	\
    ( (This)->lpVtbl -> get_LastReadLba(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_LastWrittenLba(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenLba(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_TotalSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_TotalSystemBuffer(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_UsedSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_UsedSystemBuffer(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_FreeSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_FreeSystemBuffer(This,value) ) 


#define IDiscFormat2TrackAtOnceEventArgs_get_CurrentTrackNumber(This,value)	\
    ( (This)->lpVtbl -> get_CurrentTrackNumber(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_CurrentAction(This,value)	\
    ( (This)->lpVtbl -> get_CurrentAction(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_ElapsedTime(This,value)	\
    ( (This)->lpVtbl -> get_ElapsedTime(This,value) ) 

#define IDiscFormat2TrackAtOnceEventArgs_get_RemainingTime(This,value)	\
    ( (This)->lpVtbl -> get_RemainingTime(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2TrackAtOnceEventArgs_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0016 */
/* [local] */ 

#define DISPID_IDISCFORMAT2RAWCD_RECORDER                        0x100
#define DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED      0x102
#define DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION              0x103
#define DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT      0x104
#define DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE                0x105
#define DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES        0x108
#define DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE         0x109
#define DISPID_IDISCFORMAT2RAWCD_CLIENTNAME                      0x10A
#define DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED             0x10B
#define DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV  0x10C
#define DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED               0x10D
#define DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV    0x10E
#define DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS            0x10F
#define DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS  0x110
#define DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA                    0x200
#define DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA                      0x201
#define DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION        0x202
#define DISPID_IDISCFORMAT2RAWCD_CANCELWRITE                     0x203
#define DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA                    0x204
#define DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED                   0x205


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0016_v0_0_s_ifspec;

#ifndef __IDiscFormat2RawCD_INTERFACE_DEFINED__
#define __IDiscFormat2RawCD_INTERFACE_DEFINED__

/* interface IDiscFormat2RawCD */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2RawCD;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354155-8F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2RawCD : public IDiscFormat2
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PrepareMedia( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WriteMedia( 
            /* [in] */ __RPC__in_opt IStream *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WriteMedia2( 
            /* [in] */ __RPC__in_opt IStream *data,
            /* [in] */ LONG streamLeadInSectors) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelWrite( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseMedia( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWriteSpeed( 
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recorder( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BufferUnderrunFreeDisabled( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BufferUnderrunFreeDisabled( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartOfNextSession( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastPossibleStartOfLeadout( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPhysicalMediaType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedSectorTypes( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RequestedSectorType( 
            /* [in] */ IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedSectorType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ClientName( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestedRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentWriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentRotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeeds( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedWriteSpeedDescriptors( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2RawCDVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2RawCD * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2RawCD * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2RawCD * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsRecorderSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsRecorderSupported )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, IsCurrentMediaSupported)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsCurrentMediaSupported )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *recorder,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaPhysicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaPhysicallyBlank )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_MediaHeuristicallyBlank)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaHeuristicallyBlank )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2, get_SupportedMediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedMediaTypes )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, PrepareMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PrepareMedia )( 
            __RPC__in IDiscFormat2RawCD * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, WriteMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WriteMedia )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in_opt IStream *data);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, WriteMedia2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WriteMedia2 )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in_opt IStream *data,
            /* [in] */ LONG streamLeadInSectors);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, CancelWrite)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelWrite )( 
            __RPC__in IDiscFormat2RawCD * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, ReleaseMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseMedia )( 
            __RPC__in IDiscFormat2RawCD * This);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, SetWriteSpeed)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWriteSpeed )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ LONG RequestedSectorsPerSecond,
            /* [in] */ VARIANT_BOOL RotationTypeIsPureCAV);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, put_Recorder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recorder )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_Recorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recorder )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, put_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_BufferUnderrunFreeDisabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BufferUnderrunFreeDisabled )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_StartOfNextSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartOfNextSession )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_LastPossibleStartOfLeadout)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastPossibleStartOfLeadout )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_CurrentPhysicalMediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPhysicalMediaType )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_SupportedSectorTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedSectorTypes )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, put_RequestedSectorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestedSectorType )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_RequestedSectorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedSectorType )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, put_ClientName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientName )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_ClientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_RequestedWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedWriteSpeed )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_RequestedRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestedRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_CurrentWriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentWriteSpeed )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_CurrentRotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentRotationTypeIsPureCAV )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_SupportedWriteSpeeds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeeds )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeeds);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCD, get_SupportedWriteSpeedDescriptors)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedWriteSpeedDescriptors )( 
            __RPC__in IDiscFormat2RawCD * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *supportedSpeedDescriptors);
        
        END_INTERFACE
    } IDiscFormat2RawCDVtbl;

    interface IDiscFormat2RawCD
    {
        CONST_VTBL struct IDiscFormat2RawCDVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2RawCD_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2RawCD_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2RawCD_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2RawCD_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2RawCD_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2RawCD_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2RawCD_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2RawCD_IsRecorderSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsRecorderSupported(This,recorder,value) ) 

#define IDiscFormat2RawCD_IsCurrentMediaSupported(This,recorder,value)	\
    ( (This)->lpVtbl -> IsCurrentMediaSupported(This,recorder,value) ) 

#define IDiscFormat2RawCD_get_MediaPhysicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaPhysicallyBlank(This,value) ) 

#define IDiscFormat2RawCD_get_MediaHeuristicallyBlank(This,value)	\
    ( (This)->lpVtbl -> get_MediaHeuristicallyBlank(This,value) ) 

#define IDiscFormat2RawCD_get_SupportedMediaTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedMediaTypes(This,value) ) 


#define IDiscFormat2RawCD_PrepareMedia(This)	\
    ( (This)->lpVtbl -> PrepareMedia(This) ) 

#define IDiscFormat2RawCD_WriteMedia(This,data)	\
    ( (This)->lpVtbl -> WriteMedia(This,data) ) 

#define IDiscFormat2RawCD_WriteMedia2(This,data,streamLeadInSectors)	\
    ( (This)->lpVtbl -> WriteMedia2(This,data,streamLeadInSectors) ) 

#define IDiscFormat2RawCD_CancelWrite(This)	\
    ( (This)->lpVtbl -> CancelWrite(This) ) 

#define IDiscFormat2RawCD_ReleaseMedia(This)	\
    ( (This)->lpVtbl -> ReleaseMedia(This) ) 

#define IDiscFormat2RawCD_SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV)	\
    ( (This)->lpVtbl -> SetWriteSpeed(This,RequestedSectorsPerSecond,RotationTypeIsPureCAV) ) 

#define IDiscFormat2RawCD_put_Recorder(This,value)	\
    ( (This)->lpVtbl -> put_Recorder(This,value) ) 

#define IDiscFormat2RawCD_get_Recorder(This,value)	\
    ( (This)->lpVtbl -> get_Recorder(This,value) ) 

#define IDiscFormat2RawCD_put_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> put_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2RawCD_get_BufferUnderrunFreeDisabled(This,value)	\
    ( (This)->lpVtbl -> get_BufferUnderrunFreeDisabled(This,value) ) 

#define IDiscFormat2RawCD_get_StartOfNextSession(This,value)	\
    ( (This)->lpVtbl -> get_StartOfNextSession(This,value) ) 

#define IDiscFormat2RawCD_get_LastPossibleStartOfLeadout(This,value)	\
    ( (This)->lpVtbl -> get_LastPossibleStartOfLeadout(This,value) ) 

#define IDiscFormat2RawCD_get_CurrentPhysicalMediaType(This,value)	\
    ( (This)->lpVtbl -> get_CurrentPhysicalMediaType(This,value) ) 

#define IDiscFormat2RawCD_get_SupportedSectorTypes(This,value)	\
    ( (This)->lpVtbl -> get_SupportedSectorTypes(This,value) ) 

#define IDiscFormat2RawCD_put_RequestedSectorType(This,value)	\
    ( (This)->lpVtbl -> put_RequestedSectorType(This,value) ) 

#define IDiscFormat2RawCD_get_RequestedSectorType(This,value)	\
    ( (This)->lpVtbl -> get_RequestedSectorType(This,value) ) 

#define IDiscFormat2RawCD_put_ClientName(This,value)	\
    ( (This)->lpVtbl -> put_ClientName(This,value) ) 

#define IDiscFormat2RawCD_get_ClientName(This,value)	\
    ( (This)->lpVtbl -> get_ClientName(This,value) ) 

#define IDiscFormat2RawCD_get_RequestedWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_RequestedWriteSpeed(This,value) ) 

#define IDiscFormat2RawCD_get_RequestedRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_RequestedRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2RawCD_get_CurrentWriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_CurrentWriteSpeed(This,value) ) 

#define IDiscFormat2RawCD_get_CurrentRotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_CurrentRotationTypeIsPureCAV(This,value) ) 

#define IDiscFormat2RawCD_get_SupportedWriteSpeeds(This,supportedSpeeds)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeeds(This,supportedSpeeds) ) 

#define IDiscFormat2RawCD_get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors)	\
    ( (This)->lpVtbl -> get_SupportedWriteSpeedDescriptors(This,supportedSpeedDescriptors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2RawCD_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0017 */
/* [local] */ 

#define DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE  0x200


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0017_v0_0_s_ifspec;

#ifndef __DDiscFormat2RawCDEvents_INTERFACE_DEFINED__
#define __DDiscFormat2RawCDEvents_INTERFACE_DEFINED__

/* interface DDiscFormat2RawCDEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DDiscFormat2RawCDEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354142-7F64-5B0F-8F00-5D77AFBE261E")
    DDiscFormat2RawCDEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DDiscFormat2RawCDEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DDiscFormat2RawCDEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DDiscFormat2RawCDEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DDiscFormat2RawCDEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DDiscFormat2RawCDEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DDiscFormat2RawCDEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DDiscFormat2RawCDEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DDiscFormat2RawCDEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(DDiscFormat2RawCDEvents, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DDiscFormat2RawCDEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in_opt IDispatch *progress);
        
        END_INTERFACE
    } DDiscFormat2RawCDEventsVtbl;

    interface DDiscFormat2RawCDEvents
    {
        CONST_VTBL struct DDiscFormat2RawCDEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DDiscFormat2RawCDEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DDiscFormat2RawCDEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DDiscFormat2RawCDEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DDiscFormat2RawCDEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DDiscFormat2RawCDEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DDiscFormat2RawCDEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DDiscFormat2RawCDEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DDiscFormat2RawCDEvents_Update(This,object,progress)	\
    ( (This)->lpVtbl -> Update(This,object,progress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DDiscFormat2RawCDEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0018 */
/* [local] */ 

#define DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER     0x300
#define DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION          0x301
#define DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME            0x300
#define DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME 0x301
#define DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME     0x302


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0018_v0_0_s_ifspec;

#ifndef __IDiscFormat2RawCDEventArgs_INTERFACE_DEFINED__
#define __IDiscFormat2RawCDEventArgs_INTERFACE_DEFINED__

/* interface IDiscFormat2RawCDEventArgs */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IDiscFormat2RawCDEventArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354143-7F64-5B0F-8F00-5D77AFBE261E")
    IDiscFormat2RawCDEventArgs : public IWriteEngine2EventArgs
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentAction( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_WRITE_ACTION *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ElapsedTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemainingTime( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiscFormat2RawCDEventArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDiscFormat2RawCDEventArgs * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_StartLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartLba )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_SectorCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorCount )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastReadLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastReadLba )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_LastWrittenLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenLba )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_TotalSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSystemBuffer )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_UsedSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedSystemBuffer )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IWriteEngine2EventArgs, get_FreeSystemBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSystemBuffer )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCDEventArgs, get_CurrentAction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentAction )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_WRITE_ACTION *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCDEventArgs, get_ElapsedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ElapsedTime )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IDiscFormat2RawCDEventArgs, get_RemainingTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemainingTime )( 
            __RPC__in IDiscFormat2RawCDEventArgs * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IDiscFormat2RawCDEventArgsVtbl;

    interface IDiscFormat2RawCDEventArgs
    {
        CONST_VTBL struct IDiscFormat2RawCDEventArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiscFormat2RawCDEventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiscFormat2RawCDEventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiscFormat2RawCDEventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiscFormat2RawCDEventArgs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDiscFormat2RawCDEventArgs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDiscFormat2RawCDEventArgs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDiscFormat2RawCDEventArgs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDiscFormat2RawCDEventArgs_get_StartLba(This,value)	\
    ( (This)->lpVtbl -> get_StartLba(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_SectorCount(This,value)	\
    ( (This)->lpVtbl -> get_SectorCount(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_LastReadLba(This,value)	\
    ( (This)->lpVtbl -> get_LastReadLba(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_LastWrittenLba(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenLba(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_TotalSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_TotalSystemBuffer(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_UsedSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_UsedSystemBuffer(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_FreeSystemBuffer(This,value)	\
    ( (This)->lpVtbl -> get_FreeSystemBuffer(This,value) ) 


#define IDiscFormat2RawCDEventArgs_get_CurrentAction(This,value)	\
    ( (This)->lpVtbl -> get_CurrentAction(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_ElapsedTime(This,value)	\
    ( (This)->lpVtbl -> get_ElapsedTime(This,value) ) 

#define IDiscFormat2RawCDEventArgs_get_RemainingTime(This,value)	\
    ( (This)->lpVtbl -> get_RemainingTime(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiscFormat2RawCDEventArgs_INTERFACE_DEFINED__ */


#ifndef __IBurnVerification_INTERFACE_DEFINED__
#define __IBurnVerification_INTERFACE_DEFINED__

/* interface IBurnVerification */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IBurnVerification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2FFD834-958B-426d-8470-2A13879C6A91")
    IBurnVerification : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BurnVerificationLevel( 
            /* [in] */ IMAPI_BURN_VERIFICATION_LEVEL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BurnVerificationLevel( 
            /* [retval][ref][out] */ __RPC__out IMAPI_BURN_VERIFICATION_LEVEL *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBurnVerificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBurnVerification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBurnVerification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBurnVerification * This);
        
        DECLSPEC_XFGVIRT(IBurnVerification, put_BurnVerificationLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BurnVerificationLevel )( 
            __RPC__in IBurnVerification * This,
            /* [in] */ IMAPI_BURN_VERIFICATION_LEVEL value);
        
        DECLSPEC_XFGVIRT(IBurnVerification, get_BurnVerificationLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BurnVerificationLevel )( 
            __RPC__in IBurnVerification * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_BURN_VERIFICATION_LEVEL *value);
        
        END_INTERFACE
    } IBurnVerificationVtbl;

    interface IBurnVerification
    {
        CONST_VTBL struct IBurnVerificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBurnVerification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBurnVerification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBurnVerification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBurnVerification_put_BurnVerificationLevel(This,value)	\
    ( (This)->lpVtbl -> put_BurnVerificationLevel(This,value) ) 

#define IBurnVerification_get_BurnVerificationLevel(This,value)	\
    ( (This)->lpVtbl -> get_BurnVerificationLevel(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBurnVerification_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0020 */
/* [local] */ 

#define IMAPI_SECTORS_PER_SECOND_AT_1X_CD      75
#define IMAPI_SECTORS_PER_SECOND_AT_1X_DVD     680
#define IMAPI_SECTORS_PER_SECOND_AT_1X_BD      2195
#define IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD  4568


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0020_v0_0_s_ifspec;

#ifndef __IWriteSpeedDescriptor_INTERFACE_DEFINED__
#define __IWriteSpeedDescriptor_INTERFACE_DEFINED__

/* interface IWriteSpeedDescriptor */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IWriteSpeedDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354144-7F64-5B0F-8F00-5D77AFBE261E")
    IWriteSpeedDescriptor : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RotationTypeIsPureCAV( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WriteSpeed( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWriteSpeedDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWriteSpeedDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWriteSpeedDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWriteSpeedDescriptor * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWriteSpeedDescriptor, get_MediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_MEDIA_PHYSICAL_TYPE *value);
        
        DECLSPEC_XFGVIRT(IWriteSpeedDescriptor, get_RotationTypeIsPureCAV)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RotationTypeIsPureCAV )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IWriteSpeedDescriptor, get_WriteSpeed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteSpeed )( 
            __RPC__in IWriteSpeedDescriptor * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IWriteSpeedDescriptorVtbl;

    interface IWriteSpeedDescriptor
    {
        CONST_VTBL struct IWriteSpeedDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWriteSpeedDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWriteSpeedDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWriteSpeedDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWriteSpeedDescriptor_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWriteSpeedDescriptor_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWriteSpeedDescriptor_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWriteSpeedDescriptor_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWriteSpeedDescriptor_get_MediaType(This,value)	\
    ( (This)->lpVtbl -> get_MediaType(This,value) ) 

#define IWriteSpeedDescriptor_get_RotationTypeIsPureCAV(This,value)	\
    ( (This)->lpVtbl -> get_RotationTypeIsPureCAV(This,value) ) 

#define IWriteSpeedDescriptor_get_WriteSpeed(This,value)	\
    ( (This)->lpVtbl -> get_WriteSpeed(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWriteSpeedDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0021 */
/* [local] */ 

#define DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA 0x100
#define DISPID_IMULTISESSION_INUSE                   0x101
#define DISPID_IMULTISESSION_IMPORTRECORDER          0x102


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0021_v0_0_s_ifspec;

#ifndef __IMultisession_INTERFACE_DEFINED__
#define __IMultisession_INTERFACE_DEFINED__

/* interface IMultisession */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IMultisession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354150-7F64-5B0F-8F00-5D77AFBE261E")
    IMultisession : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsSupportedOnCurrentMediaState( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InUse( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InUse( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImportRecorder( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultisessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultisession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultisession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultisession * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMultisession * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMultisession * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMultisession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMultisession * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMultisession, get_IsSupportedOnCurrentMediaState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSupportedOnCurrentMediaState )( 
            __RPC__in IMultisession * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, put_InUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InUse )( 
            __RPC__in IMultisession * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_InUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InUse )( 
            __RPC__in IMultisession * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_ImportRecorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportRecorder )( 
            __RPC__in IMultisession * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        END_INTERFACE
    } IMultisessionVtbl;

    interface IMultisession
    {
        CONST_VTBL struct IMultisessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultisession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultisession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultisession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultisession_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMultisession_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMultisession_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMultisession_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMultisession_get_IsSupportedOnCurrentMediaState(This,value)	\
    ( (This)->lpVtbl -> get_IsSupportedOnCurrentMediaState(This,value) ) 

#define IMultisession_put_InUse(This,value)	\
    ( (This)->lpVtbl -> put_InUse(This,value) ) 

#define IMultisession_get_InUse(This,value)	\
    ( (This)->lpVtbl -> get_InUse(This,value) ) 

#define IMultisession_get_ImportRecorder(This,value)	\
    ( (This)->lpVtbl -> get_ImportRecorder(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultisession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0022 */
/* [local] */ 

#define DISPID_IMULTISESSION_FIRSTDATASESSION             0x200
#define DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION 0x201
#define DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION  0x202
#define DISPID_IMULTISESSION_NEXTWRITABLEADDRESS          0x203
#define DISPID_IMULTISESSION_FREESECTORS                  0x204
#define DISPID_IMULTISESSION_WRITEUNITSIZE                0x205


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0022_v0_0_s_ifspec;

#ifndef __IMultisessionSequential_INTERFACE_DEFINED__
#define __IMultisessionSequential_INTERFACE_DEFINED__

/* interface IMultisessionSequential */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IMultisessionSequential;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354151-7F64-5B0F-8F00-5D77AFBE261E")
    IMultisessionSequential : public IMultisession
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsFirstDataSession( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartAddressOfPreviousSession( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastWrittenAddressOfPreviousSession( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NextWritableAddress( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultisessionSequentialVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultisessionSequential * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultisessionSequential * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultisessionSequential * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMultisessionSequential * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMultisessionSequential * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMultisessionSequential * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMultisessionSequential * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMultisession, get_IsSupportedOnCurrentMediaState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSupportedOnCurrentMediaState )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, put_InUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InUse )( 
            __RPC__in IMultisessionSequential * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_InUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InUse )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_ImportRecorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportRecorder )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_IsFirstDataSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstDataSession )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_StartAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartAddressOfPreviousSession )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_LastWrittenAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenAddressOfPreviousSession )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_NextWritableAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NextWritableAddress )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_FreeSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSectorsOnMedia )( 
            __RPC__in IMultisessionSequential * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IMultisessionSequentialVtbl;

    interface IMultisessionSequential
    {
        CONST_VTBL struct IMultisessionSequentialVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultisessionSequential_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultisessionSequential_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultisessionSequential_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultisessionSequential_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMultisessionSequential_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMultisessionSequential_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMultisessionSequential_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMultisessionSequential_get_IsSupportedOnCurrentMediaState(This,value)	\
    ( (This)->lpVtbl -> get_IsSupportedOnCurrentMediaState(This,value) ) 

#define IMultisessionSequential_put_InUse(This,value)	\
    ( (This)->lpVtbl -> put_InUse(This,value) ) 

#define IMultisessionSequential_get_InUse(This,value)	\
    ( (This)->lpVtbl -> get_InUse(This,value) ) 

#define IMultisessionSequential_get_ImportRecorder(This,value)	\
    ( (This)->lpVtbl -> get_ImportRecorder(This,value) ) 


#define IMultisessionSequential_get_IsFirstDataSession(This,value)	\
    ( (This)->lpVtbl -> get_IsFirstDataSession(This,value) ) 

#define IMultisessionSequential_get_StartAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_StartAddressOfPreviousSession(This,value) ) 

#define IMultisessionSequential_get_LastWrittenAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenAddressOfPreviousSession(This,value) ) 

#define IMultisessionSequential_get_NextWritableAddress(This,value)	\
    ( (This)->lpVtbl -> get_NextWritableAddress(This,value) ) 

#define IMultisessionSequential_get_FreeSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_FreeSectorsOnMedia(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultisessionSequential_INTERFACE_DEFINED__ */


#ifndef __IMultisessionSequential2_INTERFACE_DEFINED__
#define __IMultisessionSequential2_INTERFACE_DEFINED__

/* interface IMultisessionSequential2 */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IMultisessionSequential2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B507CA22-2204-11DD-966A-001AA01BBC58")
    IMultisessionSequential2 : public IMultisessionSequential
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WriteUnitSize( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultisessionSequential2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultisessionSequential2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultisessionSequential2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMultisessionSequential2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMultisession, get_IsSupportedOnCurrentMediaState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSupportedOnCurrentMediaState )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, put_InUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InUse )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_InUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InUse )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_ImportRecorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportRecorder )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_IsFirstDataSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstDataSession )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_StartAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartAddressOfPreviousSession )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_LastWrittenAddressOfPreviousSession)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenAddressOfPreviousSession )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_NextWritableAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NextWritableAddress )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential, get_FreeSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSectorsOnMedia )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionSequential2, get_WriteUnitSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteUnitSize )( 
            __RPC__in IMultisessionSequential2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IMultisessionSequential2Vtbl;

    interface IMultisessionSequential2
    {
        CONST_VTBL struct IMultisessionSequential2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultisessionSequential2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultisessionSequential2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultisessionSequential2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultisessionSequential2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMultisessionSequential2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMultisessionSequential2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMultisessionSequential2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMultisessionSequential2_get_IsSupportedOnCurrentMediaState(This,value)	\
    ( (This)->lpVtbl -> get_IsSupportedOnCurrentMediaState(This,value) ) 

#define IMultisessionSequential2_put_InUse(This,value)	\
    ( (This)->lpVtbl -> put_InUse(This,value) ) 

#define IMultisessionSequential2_get_InUse(This,value)	\
    ( (This)->lpVtbl -> get_InUse(This,value) ) 

#define IMultisessionSequential2_get_ImportRecorder(This,value)	\
    ( (This)->lpVtbl -> get_ImportRecorder(This,value) ) 


#define IMultisessionSequential2_get_IsFirstDataSession(This,value)	\
    ( (This)->lpVtbl -> get_IsFirstDataSession(This,value) ) 

#define IMultisessionSequential2_get_StartAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_StartAddressOfPreviousSession(This,value) ) 

#define IMultisessionSequential2_get_LastWrittenAddressOfPreviousSession(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenAddressOfPreviousSession(This,value) ) 

#define IMultisessionSequential2_get_NextWritableAddress(This,value)	\
    ( (This)->lpVtbl -> get_NextWritableAddress(This,value) ) 

#define IMultisessionSequential2_get_FreeSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_FreeSectorsOnMedia(This,value) ) 


#define IMultisessionSequential2_get_WriteUnitSize(This,value)	\
    ( (This)->lpVtbl -> get_WriteUnitSize(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultisessionSequential2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0024 */
/* [local] */ 

#define DISPID_IMULTISESSION_LASTWRITTENADDRESS           0x206
#define DISPID_IMULTISESSION_SECTORSONMEDIA               0x207


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0024_v0_0_s_ifspec;

#ifndef __IMultisessionRandomWrite_INTERFACE_DEFINED__
#define __IMultisessionRandomWrite_INTERFACE_DEFINED__

/* interface IMultisessionRandomWrite */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IMultisessionRandomWrite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B507CA23-2204-11DD-966A-001AA01BBC58")
    IMultisessionRandomWrite : public IMultisession
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WriteUnitSize( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastWrittenAddress( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalSectorsOnMedia( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultisessionRandomWriteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultisessionRandomWrite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultisessionRandomWrite * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMultisessionRandomWrite * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMultisession, get_IsSupportedOnCurrentMediaState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSupportedOnCurrentMediaState )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, put_InUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InUse )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_InUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InUse )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IMultisession, get_ImportRecorder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportRecorder )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IDiscRecorder2 **value);
        
        DECLSPEC_XFGVIRT(IMultisessionRandomWrite, get_WriteUnitSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteUnitSize )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionRandomWrite, get_LastWrittenAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastWrittenAddress )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IMultisessionRandomWrite, get_TotalSectorsOnMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSectorsOnMedia )( 
            __RPC__in IMultisessionRandomWrite * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IMultisessionRandomWriteVtbl;

    interface IMultisessionRandomWrite
    {
        CONST_VTBL struct IMultisessionRandomWriteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultisessionRandomWrite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultisessionRandomWrite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultisessionRandomWrite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultisessionRandomWrite_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMultisessionRandomWrite_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMultisessionRandomWrite_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMultisessionRandomWrite_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMultisessionRandomWrite_get_IsSupportedOnCurrentMediaState(This,value)	\
    ( (This)->lpVtbl -> get_IsSupportedOnCurrentMediaState(This,value) ) 

#define IMultisessionRandomWrite_put_InUse(This,value)	\
    ( (This)->lpVtbl -> put_InUse(This,value) ) 

#define IMultisessionRandomWrite_get_InUse(This,value)	\
    ( (This)->lpVtbl -> get_InUse(This,value) ) 

#define IMultisessionRandomWrite_get_ImportRecorder(This,value)	\
    ( (This)->lpVtbl -> get_ImportRecorder(This,value) ) 


#define IMultisessionRandomWrite_get_WriteUnitSize(This,value)	\
    ( (This)->lpVtbl -> get_WriteUnitSize(This,value) ) 

#define IMultisessionRandomWrite_get_LastWrittenAddress(This,value)	\
    ( (This)->lpVtbl -> get_LastWrittenAddress(This,value) ) 

#define IMultisessionRandomWrite_get_TotalSectorsOnMedia(This,value)	\
    ( (This)->lpVtbl -> get_TotalSectorsOnMedia(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultisessionRandomWrite_INTERFACE_DEFINED__ */


#ifndef __IStreamPseudoRandomBased_INTERFACE_DEFINED__
#define __IStreamPseudoRandomBased_INTERFACE_DEFINED__

/* interface IStreamPseudoRandomBased */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IStreamPseudoRandomBased;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354145-7F64-5B0F-8F00-5D77AFBE261E")
    IStreamPseudoRandomBased : public IStream
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE put_Seed( 
            /* [in] */ ULONG value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_Seed( 
            /* [out] */ __RPC__out ULONG *value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE put_ExtendedSeed( 
            /* [size_is][in] */ __RPC__in_ecount_full(eCount) ULONG *values,
            /* [in] */ ULONG eCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_ExtendedSeed( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*eCount) ULONG **values,
            /* [out] */ __RPC__out ULONG *eCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamPseudoRandomBasedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStreamPseudoRandomBased * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStreamPseudoRandomBased * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IStreamPseudoRandomBased * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IStreamPseudoRandomBased * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IStreamPseudoRandomBased * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStreamPseudoRandomBased * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IStreamPseudoRandomBased * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStreamPseudoRandomBased, put_Seed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *put_Seed )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [in] */ ULONG value);
        
        DECLSPEC_XFGVIRT(IStreamPseudoRandomBased, get_Seed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_Seed )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [out] */ __RPC__out ULONG *value);
        
        DECLSPEC_XFGVIRT(IStreamPseudoRandomBased, put_ExtendedSeed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *put_ExtendedSeed )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [size_is][in] */ __RPC__in_ecount_full(eCount) ULONG *values,
            /* [in] */ ULONG eCount);
        
        DECLSPEC_XFGVIRT(IStreamPseudoRandomBased, get_ExtendedSeed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedSeed )( 
            __RPC__in IStreamPseudoRandomBased * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*eCount) ULONG **values,
            /* [out] */ __RPC__out ULONG *eCount);
        
        END_INTERFACE
    } IStreamPseudoRandomBasedVtbl;

    interface IStreamPseudoRandomBased
    {
        CONST_VTBL struct IStreamPseudoRandomBasedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamPseudoRandomBased_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamPseudoRandomBased_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamPseudoRandomBased_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamPseudoRandomBased_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IStreamPseudoRandomBased_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IStreamPseudoRandomBased_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IStreamPseudoRandomBased_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IStreamPseudoRandomBased_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IStreamPseudoRandomBased_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStreamPseudoRandomBased_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStreamPseudoRandomBased_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamPseudoRandomBased_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamPseudoRandomBased_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IStreamPseudoRandomBased_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IStreamPseudoRandomBased_put_Seed(This,value)	\
    ( (This)->lpVtbl -> put_Seed(This,value) ) 

#define IStreamPseudoRandomBased_get_Seed(This,value)	\
    ( (This)->lpVtbl -> get_Seed(This,value) ) 

#define IStreamPseudoRandomBased_put_ExtendedSeed(This,values,eCount)	\
    ( (This)->lpVtbl -> put_ExtendedSeed(This,values,eCount) ) 

#define IStreamPseudoRandomBased_get_ExtendedSeed(This,values,eCount)	\
    ( (This)->lpVtbl -> get_ExtendedSeed(This,values,eCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamPseudoRandomBased_INTERFACE_DEFINED__ */


#ifndef __IStreamConcatenate_INTERFACE_DEFINED__
#define __IStreamConcatenate_INTERFACE_DEFINED__

/* interface IStreamConcatenate */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IStreamConcatenate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354146-7F64-5B0F-8F00-5D77AFBE261E")
    IStreamConcatenate : public IStream
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IStream *stream1,
            /* [in] */ __RPC__in_opt IStream *stream2) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize2( 
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [in] */ ULONG streamCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IStream *stream) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Append2( 
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [in] */ ULONG streamCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamConcatenateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStreamConcatenate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStreamConcatenate * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IStreamConcatenate * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IStreamConcatenate * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IStreamConcatenate * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStreamConcatenate * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IStreamConcatenate * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IStreamConcatenate * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IStreamConcatenate * This,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStreamConcatenate, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ __RPC__in_opt IStream *stream1,
            /* [in] */ __RPC__in_opt IStream *stream2);
        
        DECLSPEC_XFGVIRT(IStreamConcatenate, Initialize2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize2 )( 
            __RPC__in IStreamConcatenate * This,
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [in] */ ULONG streamCount);
        
        DECLSPEC_XFGVIRT(IStreamConcatenate, Append)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IStreamConcatenate * This,
            /* [in] */ __RPC__in_opt IStream *stream);
        
        DECLSPEC_XFGVIRT(IStreamConcatenate, Append2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Append2 )( 
            __RPC__in IStreamConcatenate * This,
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [in] */ ULONG streamCount);
        
        END_INTERFACE
    } IStreamConcatenateVtbl;

    interface IStreamConcatenate
    {
        CONST_VTBL struct IStreamConcatenateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamConcatenate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamConcatenate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamConcatenate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamConcatenate_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IStreamConcatenate_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IStreamConcatenate_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IStreamConcatenate_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IStreamConcatenate_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IStreamConcatenate_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStreamConcatenate_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStreamConcatenate_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamConcatenate_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamConcatenate_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IStreamConcatenate_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IStreamConcatenate_Initialize(This,stream1,stream2)	\
    ( (This)->lpVtbl -> Initialize(This,stream1,stream2) ) 

#define IStreamConcatenate_Initialize2(This,streams,streamCount)	\
    ( (This)->lpVtbl -> Initialize2(This,streams,streamCount) ) 

#define IStreamConcatenate_Append(This,stream)	\
    ( (This)->lpVtbl -> Append(This,stream) ) 

#define IStreamConcatenate_Append2(This,streams,streamCount)	\
    ( (This)->lpVtbl -> Append2(This,streams,streamCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamConcatenate_INTERFACE_DEFINED__ */


#ifndef __IStreamInterleave_INTERFACE_DEFINED__
#define __IStreamInterleave_INTERFACE_DEFINED__

/* interface IStreamInterleave */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IStreamInterleave;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27354147-7F64-5B0F-8F00-5D77AFBE261E")
    IStreamInterleave : public IStream
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) ULONG *interleaveSizes,
            /* [range][in] */ __RPC__in_range(1,0x7fffffff) ULONG streamCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamInterleaveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStreamInterleave * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStreamInterleave * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStreamInterleave * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IStreamInterleave * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IStreamInterleave * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IStreamInterleave * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IStreamInterleave * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStreamInterleave * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IStreamInterleave * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IStreamInterleave * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in IStreamInterleave * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in IStreamInterleave * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IStreamInterleave * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IStreamInterleave * This,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStreamInterleave, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IStreamInterleave * This,
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) IStream **streams,
            /* [size_is][in] */ __RPC__in_ecount_full(streamCount) ULONG *interleaveSizes,
            /* [range][in] */ __RPC__in_range(1,0x7fffffff) ULONG streamCount);
        
        END_INTERFACE
    } IStreamInterleaveVtbl;

    interface IStreamInterleave
    {
        CONST_VTBL struct IStreamInterleaveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamInterleave_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamInterleave_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamInterleave_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamInterleave_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IStreamInterleave_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IStreamInterleave_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IStreamInterleave_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IStreamInterleave_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IStreamInterleave_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStreamInterleave_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStreamInterleave_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamInterleave_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IStreamInterleave_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IStreamInterleave_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IStreamInterleave_Initialize(This,streams,interleaveSizes,streamCount)	\
    ( (This)->lpVtbl -> Initialize(This,streams,interleaveSizes,streamCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamInterleave_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0028 */
/* [local] */ 

#define DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE        0x200
#define DISPID_IRAWCDIMAGECREATOR_ADDTRACK                 0x201
#define DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP         0x202
#define DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR    0x203
#define DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE       0x100
#define DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT           0x101
#define DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT      0x102
#define DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO      0x103
#define DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER       0x104
#define DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER      0x105
#define DISPID_IRAWCDIMAGECREATOR_TRACKINFO                0x106
#define DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS   0x107
#define DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC        0x108
#define DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS  0x109


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0028_v0_0_s_ifspec;

#ifndef __IRawCDImageCreator_INTERFACE_DEFINED__
#define __IRawCDImageCreator_INTERFACE_DEFINED__

/* interface IRawCDImageCreator */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IRawCDImageCreator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25983550-9D65-49CE-B335-40630D901227")
    IRawCDImageCreator : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateResultImage( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **resultStream) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddTrack( 
            /* [in] */ IMAPI_CD_SECTOR_TYPE dataType,
            /* [in] */ __RPC__in_opt IStream *data,
            /* [retval][out] */ __RPC__out LONG *trackIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddSpecialPregap( 
            /* [in] */ __RPC__in_opt IStream *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddSubcodeRWGenerator( 
            /* [in] */ __RPC__in_opt IStream *subcode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ResultingImageType( 
            /* [in] */ IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ResultingImageType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartOfLeadout( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartOfLeadoutLimit( 
            /* [in] */ LONG value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartOfLeadoutLimit( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisableGaplessAudio( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisableGaplessAudio( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaCatalogNumber( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaCatalogNumber( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartingTrackNumber( 
            /* [range][in] */ __RPC__in_range(1,99) LONG value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartingTrackNumber( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrackInfo( 
            /* [in] */ LONG trackIndex,
            /* [retval][ref][out] */ __RPC__deref_out_opt IRawCDImageTrackInfo **value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfExistingTracks( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastUsedUserSectorInImage( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExpectedTableOfContents( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawCDImageCreatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawCDImageCreator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawCDImageCreator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRawCDImageCreator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRawCDImageCreator * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, CreateResultImage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateResultImage )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **resultStream);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, AddTrack)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTrack )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ IMAPI_CD_SECTOR_TYPE dataType,
            /* [in] */ __RPC__in_opt IStream *data,
            /* [retval][out] */ __RPC__out LONG *trackIndex);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, AddSpecialPregap)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddSpecialPregap )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ __RPC__in_opt IStream *data);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, AddSubcodeRWGenerator)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddSubcodeRWGenerator )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ __RPC__in_opt IStream *subcode);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, put_ResultingImageType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ResultingImageType )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_ResultingImageType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ResultingImageType )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_StartOfLeadout)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartOfLeadout )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, put_StartOfLeadoutLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartOfLeadoutLimit )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ LONG value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_StartOfLeadoutLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartOfLeadoutLimit )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, put_DisableGaplessAudio)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisableGaplessAudio )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_DisableGaplessAudio)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisableGaplessAudio )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, put_MediaCatalogNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaCatalogNumber )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_MediaCatalogNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaCatalogNumber )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, put_StartingTrackNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartingTrackNumber )( 
            __RPC__in IRawCDImageCreator * This,
            /* [range][in] */ __RPC__in_range(1,99) LONG value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_StartingTrackNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartingTrackNumber )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_TrackInfo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrackInfo )( 
            __RPC__in IRawCDImageCreator * This,
            /* [in] */ LONG trackIndex,
            /* [retval][ref][out] */ __RPC__deref_out_opt IRawCDImageTrackInfo **value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_NumberOfExistingTracks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfExistingTracks )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_LastUsedUserSectorInImage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastUsedUserSectorInImage )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageCreator, get_ExpectedTableOfContents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpectedTableOfContents )( 
            __RPC__in IRawCDImageCreator * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        END_INTERFACE
    } IRawCDImageCreatorVtbl;

    interface IRawCDImageCreator
    {
        CONST_VTBL struct IRawCDImageCreatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawCDImageCreator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawCDImageCreator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawCDImageCreator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawCDImageCreator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRawCDImageCreator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRawCDImageCreator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRawCDImageCreator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRawCDImageCreator_CreateResultImage(This,resultStream)	\
    ( (This)->lpVtbl -> CreateResultImage(This,resultStream) ) 

#define IRawCDImageCreator_AddTrack(This,dataType,data,trackIndex)	\
    ( (This)->lpVtbl -> AddTrack(This,dataType,data,trackIndex) ) 

#define IRawCDImageCreator_AddSpecialPregap(This,data)	\
    ( (This)->lpVtbl -> AddSpecialPregap(This,data) ) 

#define IRawCDImageCreator_AddSubcodeRWGenerator(This,subcode)	\
    ( (This)->lpVtbl -> AddSubcodeRWGenerator(This,subcode) ) 

#define IRawCDImageCreator_put_ResultingImageType(This,value)	\
    ( (This)->lpVtbl -> put_ResultingImageType(This,value) ) 

#define IRawCDImageCreator_get_ResultingImageType(This,value)	\
    ( (This)->lpVtbl -> get_ResultingImageType(This,value) ) 

#define IRawCDImageCreator_get_StartOfLeadout(This,value)	\
    ( (This)->lpVtbl -> get_StartOfLeadout(This,value) ) 

#define IRawCDImageCreator_put_StartOfLeadoutLimit(This,value)	\
    ( (This)->lpVtbl -> put_StartOfLeadoutLimit(This,value) ) 

#define IRawCDImageCreator_get_StartOfLeadoutLimit(This,value)	\
    ( (This)->lpVtbl -> get_StartOfLeadoutLimit(This,value) ) 

#define IRawCDImageCreator_put_DisableGaplessAudio(This,value)	\
    ( (This)->lpVtbl -> put_DisableGaplessAudio(This,value) ) 

#define IRawCDImageCreator_get_DisableGaplessAudio(This,value)	\
    ( (This)->lpVtbl -> get_DisableGaplessAudio(This,value) ) 

#define IRawCDImageCreator_put_MediaCatalogNumber(This,value)	\
    ( (This)->lpVtbl -> put_MediaCatalogNumber(This,value) ) 

#define IRawCDImageCreator_get_MediaCatalogNumber(This,value)	\
    ( (This)->lpVtbl -> get_MediaCatalogNumber(This,value) ) 

#define IRawCDImageCreator_put_StartingTrackNumber(This,value)	\
    ( (This)->lpVtbl -> put_StartingTrackNumber(This,value) ) 

#define IRawCDImageCreator_get_StartingTrackNumber(This,value)	\
    ( (This)->lpVtbl -> get_StartingTrackNumber(This,value) ) 

#define IRawCDImageCreator_get_TrackInfo(This,trackIndex,value)	\
    ( (This)->lpVtbl -> get_TrackInfo(This,trackIndex,value) ) 

#define IRawCDImageCreator_get_NumberOfExistingTracks(This,value)	\
    ( (This)->lpVtbl -> get_NumberOfExistingTracks(This,value) ) 

#define IRawCDImageCreator_get_LastUsedUserSectorInImage(This,value)	\
    ( (This)->lpVtbl -> get_LastUsedUserSectorInImage(This,value) ) 

#define IRawCDImageCreator_get_ExpectedTableOfContents(This,value)	\
    ( (This)->lpVtbl -> get_ExpectedTableOfContents(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawCDImageCreator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0029 */
/* [local] */ 

#define DISPID_IRAWCDTRACKINFO_STARTINGLBA              0x100
#define DISPID_IRAWCDTRACKINFO_SECTORCOUNT              0x101
#define DISPID_IRAWCDTRACKINFO_TRACKNUMBER              0x102
#define DISPID_IRAWCDTRACKINFO_SECTORTYPE               0x103
#define DISPID_IRAWCDTRACKINFO_ISRC                     0x104
#define DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING  0x105
#define DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS      0x106


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0029_v0_0_s_ifspec;

#ifndef __IRawCDImageTrackInfo_INTERFACE_DEFINED__
#define __IRawCDImageTrackInfo_INTERFACE_DEFINED__

/* interface IRawCDImageTrackInfo */
/* [helpstring][unique][uuid][dual][nonextensible][object] */ 


EXTERN_C const IID IID_IRawCDImageTrackInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25983551-9D65-49CE-B335-40630D901227")
    IRawCDImageTrackInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartingLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SectorCount( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrackNumber( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SectorType( 
            /* [retval][ref][out] */ __RPC__out IMAPI_CD_SECTOR_TYPE *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ISRC( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ISRC( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DigitalAudioCopySetting( 
            /* [retval][ref][out] */ __RPC__out IMAPI_CD_TRACK_DIGITAL_COPY_SETTING *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DigitalAudioCopySetting( 
            /* [in] */ IMAPI_CD_TRACK_DIGITAL_COPY_SETTING value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AudioHasPreemphasis( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AudioHasPreemphasis( 
            /* [in] */ VARIANT_BOOL value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrackIndexes( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddTrackIndex( 
            /* [range][in] */ __RPC__in_range(0,0x7fffffff) LONG lbaOffset) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearTrackIndex( 
            /* [range][in] */ __RPC__in_range(0,0x7fffffff) LONG lbaOffset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawCDImageTrackInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawCDImageTrackInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawCDImageTrackInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRawCDImageTrackInfo * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_StartingLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartingLba )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_SectorCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorCount )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_TrackNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrackNumber )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_SectorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SectorType )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_CD_SECTOR_TYPE *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_ISRC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISRC )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, put_ISRC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ISRC )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_DigitalAudioCopySetting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DigitalAudioCopySetting )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out IMAPI_CD_TRACK_DIGITAL_COPY_SETTING *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, put_DigitalAudioCopySetting)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DigitalAudioCopySetting )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ IMAPI_CD_TRACK_DIGITAL_COPY_SETTING value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_AudioHasPreemphasis)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AudioHasPreemphasis )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, put_AudioHasPreemphasis)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AudioHasPreemphasis )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [in] */ VARIANT_BOOL value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, get_TrackIndexes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrackIndexes )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, AddTrackIndex)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTrackIndex )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [range][in] */ __RPC__in_range(0,0x7fffffff) LONG lbaOffset);
        
        DECLSPEC_XFGVIRT(IRawCDImageTrackInfo, ClearTrackIndex)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearTrackIndex )( 
            __RPC__in IRawCDImageTrackInfo * This,
            /* [range][in] */ __RPC__in_range(0,0x7fffffff) LONG lbaOffset);
        
        END_INTERFACE
    } IRawCDImageTrackInfoVtbl;

    interface IRawCDImageTrackInfo
    {
        CONST_VTBL struct IRawCDImageTrackInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawCDImageTrackInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawCDImageTrackInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawCDImageTrackInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawCDImageTrackInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRawCDImageTrackInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRawCDImageTrackInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRawCDImageTrackInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRawCDImageTrackInfo_get_StartingLba(This,value)	\
    ( (This)->lpVtbl -> get_StartingLba(This,value) ) 

#define IRawCDImageTrackInfo_get_SectorCount(This,value)	\
    ( (This)->lpVtbl -> get_SectorCount(This,value) ) 

#define IRawCDImageTrackInfo_get_TrackNumber(This,value)	\
    ( (This)->lpVtbl -> get_TrackNumber(This,value) ) 

#define IRawCDImageTrackInfo_get_SectorType(This,value)	\
    ( (This)->lpVtbl -> get_SectorType(This,value) ) 

#define IRawCDImageTrackInfo_get_ISRC(This,value)	\
    ( (This)->lpVtbl -> get_ISRC(This,value) ) 

#define IRawCDImageTrackInfo_put_ISRC(This,value)	\
    ( (This)->lpVtbl -> put_ISRC(This,value) ) 

#define IRawCDImageTrackInfo_get_DigitalAudioCopySetting(This,value)	\
    ( (This)->lpVtbl -> get_DigitalAudioCopySetting(This,value) ) 

#define IRawCDImageTrackInfo_put_DigitalAudioCopySetting(This,value)	\
    ( (This)->lpVtbl -> put_DigitalAudioCopySetting(This,value) ) 

#define IRawCDImageTrackInfo_get_AudioHasPreemphasis(This,value)	\
    ( (This)->lpVtbl -> get_AudioHasPreemphasis(This,value) ) 

#define IRawCDImageTrackInfo_put_AudioHasPreemphasis(This,value)	\
    ( (This)->lpVtbl -> put_AudioHasPreemphasis(This,value) ) 

#define IRawCDImageTrackInfo_get_TrackIndexes(This,value)	\
    ( (This)->lpVtbl -> get_TrackIndexes(This,value) ) 

#define IRawCDImageTrackInfo_AddTrackIndex(This,lbaOffset)	\
    ( (This)->lpVtbl -> AddTrackIndex(This,lbaOffset) ) 

#define IRawCDImageTrackInfo_ClearTrackIndex(This,lbaOffset)	\
    ( (This)->lpVtbl -> ClearTrackIndex(This,lbaOffset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawCDImageTrackInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0030 */
/* [local] */ 

#define DISPID_IBLOCKRANGE_STARTLBA                       0x100
#define DISPID_IBLOCKRANGE_ENDLBA                         0x101


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0030_v0_0_s_ifspec;

#ifndef __IBlockRange_INTERFACE_DEFINED__
#define __IBlockRange_INTERFACE_DEFINED__

/* interface IBlockRange */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IBlockRange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B507CA25-2204-11DD-966A-001AA01BBC58")
    IBlockRange : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EndLba( 
            /* [retval][ref][out] */ __RPC__out LONG *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBlockRangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBlockRange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBlockRange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBlockRange * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBlockRange * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBlockRange * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBlockRange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBlockRange * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IBlockRange, get_StartLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartLba )( 
            __RPC__in IBlockRange * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        DECLSPEC_XFGVIRT(IBlockRange, get_EndLba)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndLba )( 
            __RPC__in IBlockRange * This,
            /* [retval][ref][out] */ __RPC__out LONG *value);
        
        END_INTERFACE
    } IBlockRangeVtbl;

    interface IBlockRange
    {
        CONST_VTBL struct IBlockRangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBlockRange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBlockRange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBlockRange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBlockRange_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBlockRange_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBlockRange_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBlockRange_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBlockRange_get_StartLba(This,value)	\
    ( (This)->lpVtbl -> get_StartLba(This,value) ) 

#define IBlockRange_get_EndLba(This,value)	\
    ( (This)->lpVtbl -> get_EndLba(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBlockRange_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0031 */
/* [local] */ 

#define DISPID_IBLOCKRANGELIST_BLOCKRANGES                0x100


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0031_v0_0_s_ifspec;

#ifndef __IBlockRangeList_INTERFACE_DEFINED__
#define __IBlockRangeList_INTERFACE_DEFINED__

/* interface IBlockRangeList */
/* [helpstring][unique][uuid][dual][object] */ 


EXTERN_C const IID IID_IBlockRangeList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B507CA26-2204-11DD-966A-001AA01BBC58")
    IBlockRangeList : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockRanges( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBlockRangeListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBlockRangeList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBlockRangeList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBlockRangeList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBlockRangeList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBlockRangeList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBlockRangeList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBlockRangeList * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IBlockRangeList, get_BlockRanges)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockRanges )( 
            __RPC__in IBlockRangeList * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *value);
        
        END_INTERFACE
    } IBlockRangeListVtbl;

    interface IBlockRangeList
    {
        CONST_VTBL struct IBlockRangeListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBlockRangeList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBlockRangeList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBlockRangeList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBlockRangeList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBlockRangeList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBlockRangeList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBlockRangeList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBlockRangeList_get_BlockRanges(This,value)	\
    ( (This)->lpVtbl -> get_BlockRanges(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBlockRangeList_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2_0000_0032 */
/* [local] */ 


//
// IMAPIv2 version information for TYPELib loading
//
#define IMAPILib2_MajorVersion 1
#define IMAPILib2_MinorVersion 0
#define LIBID_IMAPILib2 LIBID_IMAPI2



extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0032_v0_0_s_ifspec;


#ifndef __IMAPI2_LIBRARY_DEFINED__
#define __IMAPI2_LIBRARY_DEFINED__

/* library IMAPI2 */
/* [helpstring][version][uuid] */ 

































EXTERN_C const IID LIBID_IMAPI2;

EXTERN_C const CLSID CLSID_MsftDiscMaster2;

#ifdef __cplusplus

class DECLSPEC_UUID("2735412E-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscMaster2;
#endif

EXTERN_C const CLSID CLSID_MsftDiscRecorder2;

#ifdef __cplusplus

class DECLSPEC_UUID("2735412D-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscRecorder2;
#endif

EXTERN_C const CLSID CLSID_MsftWriteEngine2;

#ifdef __cplusplus

class DECLSPEC_UUID("2735412C-7F64-5B0F-8F00-5D77AFBE261E")
MsftWriteEngine2;
#endif

EXTERN_C const CLSID CLSID_MsftDiscFormat2Erase;

#ifdef __cplusplus

class DECLSPEC_UUID("2735412B-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscFormat2Erase;
#endif

EXTERN_C const CLSID CLSID_MsftDiscFormat2Data;

#ifdef __cplusplus

class DECLSPEC_UUID("2735412A-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscFormat2Data;
#endif

EXTERN_C const CLSID CLSID_MsftDiscFormat2TrackAtOnce;

#ifdef __cplusplus

class DECLSPEC_UUID("27354129-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscFormat2TrackAtOnce;
#endif

EXTERN_C const CLSID CLSID_MsftDiscFormat2RawCD;

#ifdef __cplusplus

class DECLSPEC_UUID("27354128-7F64-5B0F-8F00-5D77AFBE261E")
MsftDiscFormat2RawCD;
#endif

EXTERN_C const CLSID CLSID_MsftStreamZero;

#ifdef __cplusplus

class DECLSPEC_UUID("27354127-7F64-5B0F-8F00-5D77AFBE261E")
MsftStreamZero;
#endif

EXTERN_C const CLSID CLSID_MsftStreamPrng001;

#ifdef __cplusplus

class DECLSPEC_UUID("27354126-7F64-5B0F-8F00-5D77AFBE261E")
MsftStreamPrng001;
#endif

EXTERN_C const CLSID CLSID_MsftStreamConcatenate;

#ifdef __cplusplus

class DECLSPEC_UUID("27354125-7F64-5B0F-8F00-5D77AFBE261E")
MsftStreamConcatenate;
#endif

EXTERN_C const CLSID CLSID_MsftStreamInterleave;

#ifdef __cplusplus

class DECLSPEC_UUID("27354124-7F64-5B0F-8F00-5D77AFBE261E")
MsftStreamInterleave;
#endif

EXTERN_C const CLSID CLSID_MsftWriteSpeedDescriptor;

#ifdef __cplusplus

class DECLSPEC_UUID("27354123-7F64-5B0F-8F00-5D77AFBE261E")
MsftWriteSpeedDescriptor;
#endif

EXTERN_C const CLSID CLSID_MsftMultisessionSequential;

#ifdef __cplusplus

class DECLSPEC_UUID("27354122-7F64-5B0F-8F00-5D77AFBE261E")
MsftMultisessionSequential;
#endif

EXTERN_C const CLSID CLSID_MsftMultisessionRandomWrite;

#ifdef __cplusplus

class DECLSPEC_UUID("B507CA24-2204-11DD-966A-001AA01BBC58")
MsftMultisessionRandomWrite;
#endif

EXTERN_C const CLSID CLSID_MsftRawCDImageCreator;

#ifdef __cplusplus

class DECLSPEC_UUID("25983561-9D65-49CE-B335-40630D901227")
MsftRawCDImageCreator;
#endif
#endif /* __IMAPI2_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_imapi2_0000_0033 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2_0000_0033_v0_0_s_ifspec;

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

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


