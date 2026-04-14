/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:        adoint_Backcompat.h
//
// Contents:    ADO Interface header
//
// Comments:    This is essentially the file "adoint.h" shipped in Win7 SDK
//-----------------------------------------------------------------------------

#ifndef _ADOINT_H_
#define _ADOINT_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _INC_TCHAR
#include <tchar.h>
#endif

#if (_MSC_VER >= 1100) && defined (__cplusplus)
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif


/* @@MIDL_FILE_HEADING(  ) */
#pragma warning( disable: 4049 )  /* more than 64k source lines */
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
#endif // __RPCNDR_H_VERSION__
#ifndef __ado10_h__
#define __ado10_h__
#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif
/* Forward Declarations */ 
#ifndef ___ADOCollection_FWD_DEFINED__
#define ___ADOCollection_FWD_DEFINED__
typedef interface _ADOCollection _ADOCollection;
#endif 	/* ___ADOCollection_FWD_DEFINED__ */
#ifndef ___ADODynaCollection_FWD_DEFINED__
#define ___ADODynaCollection_FWD_DEFINED__
typedef interface _ADODynaCollection _ADODynaCollection;
#endif 	/* ___ADODynaCollection_FWD_DEFINED__ */
#ifndef ___ADO_FWD_DEFINED__
#define ___ADO_FWD_DEFINED__
typedef interface _ADO _ADO;
#endif 	/* ___ADO_FWD_DEFINED__ */
#ifndef __Error_FWD_DEFINED__
#define __Error_FWD_DEFINED__
typedef interface ADOError Error;
#endif 	/* __Error_FWD_DEFINED__ */
#ifndef __Errors_FWD_DEFINED__
#define __Errors_FWD_DEFINED__
typedef interface ADOErrors Errors;
#endif 	/* __Errors_FWD_DEFINED__ */
#ifndef __Command15_FWD_DEFINED__
#define __Command15_FWD_DEFINED__
typedef interface Command15 Command15;
#endif 	/* __Command15_FWD_DEFINED__ */
#ifndef __Command25_FWD_DEFINED__
#define __Command25_FWD_DEFINED__
typedef interface Command25 Command25;
#endif 	/* __Command25_FWD_DEFINED__ */
#ifndef ___Command_FWD_DEFINED__
#define ___Command_FWD_DEFINED__
typedef interface _ADOCommand _Command;
#endif 	/* ___Command_FWD_DEFINED__ */
#ifndef __ConnectionEventsVt_FWD_DEFINED__
#define __ConnectionEventsVt_FWD_DEFINED__
typedef interface ConnectionEventsVt ConnectionEventsVt;
#endif 	/* __ConnectionEventsVt_FWD_DEFINED__ */
#ifndef __RecordsetEventsVt_FWD_DEFINED__
#define __RecordsetEventsVt_FWD_DEFINED__
typedef interface RecordsetEventsVt RecordsetEventsVt;
#endif 	/* __RecordsetEventsVt_FWD_DEFINED__ */
#ifndef __ConnectionEvents_FWD_DEFINED__
#define __ConnectionEvents_FWD_DEFINED__
typedef interface ConnectionEvents ConnectionEvents;
#endif 	/* __ConnectionEvents_FWD_DEFINED__ */
#ifndef __RecordsetEvents_FWD_DEFINED__
#define __RecordsetEvents_FWD_DEFINED__
typedef interface RecordsetEvents RecordsetEvents;
#endif 	/* __RecordsetEvents_FWD_DEFINED__ */
#ifndef __Connection15_FWD_DEFINED__
#define __Connection15_FWD_DEFINED__
typedef interface Connection15 Connection15;
#endif 	/* __Connection15_FWD_DEFINED__ */
#ifndef ___Connection_FWD_DEFINED__
#define ___Connection_FWD_DEFINED__
typedef interface _ADOConnection _Connection;
#endif 	/* ___Connection_FWD_DEFINED__ */
#ifndef __ADOConnectionConstruction15_FWD_DEFINED__
#define __ADOConnectionConstruction15_FWD_DEFINED__
typedef interface ADOConnectionConstruction15 ADOConnectionConstruction15;
#endif 	/* __ADOConnectionConstruction15_FWD_DEFINED__ */
#ifndef __ADOConnectionConstruction_FWD_DEFINED__
#define __ADOConnectionConstruction_FWD_DEFINED__
typedef interface ADOConnectionConstruction ADOConnectionConstruction;
#endif 	/* __ADOConnectionConstruction_FWD_DEFINED__ */
#ifndef __Connection_FWD_DEFINED__
#define __Connection_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOConnection Connection;
#else
typedef struct ADOConnection Connection;
#endif /* __cplusplus */
#endif 	/* __Connection_FWD_DEFINED__ */
#ifndef ___Record_FWD_DEFINED__
#define ___Record_FWD_DEFINED__
typedef interface _ADORecord _Record;
#endif 	/* ___Record_FWD_DEFINED__ */
#ifndef __Record_FWD_DEFINED__
#define __Record_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADORecord Record;
#else
typedef struct ADORecord Record;
#endif /* __cplusplus */
#endif 	/* __Record_FWD_DEFINED__ */
#ifndef ___Stream_FWD_DEFINED__
#define ___Stream_FWD_DEFINED__
typedef interface _ADOStream _Stream;
#endif 	/* ___Stream_FWD_DEFINED__ */
#ifndef __Stream_FWD_DEFINED__
#define __Stream_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOStream Stream;
#else
typedef struct ADOStream Stream;
#endif /* __cplusplus */
#endif 	/* __Stream_FWD_DEFINED__ */
#ifndef __ADORecordConstruction_FWD_DEFINED__
#define __ADORecordConstruction_FWD_DEFINED__
typedef interface ADORecordConstruction ADORecordConstruction;
#endif 	/* __ADORecordConstruction_FWD_DEFINED__ */
#ifndef __ADOStreamConstruction_FWD_DEFINED__
#define __ADOStreamConstruction_FWD_DEFINED__
typedef interface ADOStreamConstruction ADOStreamConstruction;
#endif 	/* __ADOStreamConstruction_FWD_DEFINED__ */
#ifndef __ADOCommandConstruction_FWD_DEFINED__
#define __ADOCommandConstruction_FWD_DEFINED__
typedef interface ADOCommandConstruction ADOCommandConstruction;
#endif 	/* __ADOCommandConstruction_FWD_DEFINED__ */
#ifndef __Command_FWD_DEFINED__
#define __Command_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOCommand Command;
#else
typedef struct ADOCommand Command;
#endif /* __cplusplus */
#endif 	/* __Command_FWD_DEFINED__ */
#ifndef __Recordset_FWD_DEFINED__
#define __Recordset_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADORecordset Recordset;
#else
typedef struct ADORecordset Recordset;
#endif /* __cplusplus */
#endif 	/* __Recordset_FWD_DEFINED__ */
#ifndef __Recordset15_FWD_DEFINED__
#define __Recordset15_FWD_DEFINED__
typedef interface Recordset15 Recordset15;
#endif 	/* __Recordset15_FWD_DEFINED__ */
#ifndef __Recordset20_FWD_DEFINED__
#define __Recordset20_FWD_DEFINED__
typedef interface Recordset20 Recordset20;
#endif 	/* __Recordset20_FWD_DEFINED__ */
#ifndef __Recordset21_FWD_DEFINED__
#define __Recordset21_FWD_DEFINED__
typedef interface Recordset21 Recordset21;
#endif 	/* __Recordset21_FWD_DEFINED__ */
#ifndef ___Recordset_FWD_DEFINED__
#define ___Recordset_FWD_DEFINED__
typedef interface _ADORecordset _Recordset;
#endif 	/* ___Recordset_FWD_DEFINED__ */
#ifndef __ADORecordsetConstruction_FWD_DEFINED__
#define __ADORecordsetConstruction_FWD_DEFINED__
typedef interface ADORecordsetConstruction ADORecordsetConstruction;
#endif 	/* __ADORecordsetConstruction_FWD_DEFINED__ */
#ifndef __Field15_FWD_DEFINED__
#define __Field15_FWD_DEFINED__
typedef interface Field15 Field15;
#endif 	/* __Field15_FWD_DEFINED__ */
#ifndef __Field20_FWD_DEFINED__
#define __Field20_FWD_DEFINED__
typedef interface Field20 Field20;
#endif 	/* __Field20_FWD_DEFINED__ */
#ifndef __Field_FWD_DEFINED__
#define __Field_FWD_DEFINED__
typedef interface ADOField Field;
#endif 	/* __Field_FWD_DEFINED__ */
#ifndef __Fields15_FWD_DEFINED__
#define __Fields15_FWD_DEFINED__
typedef interface Fields15 Fields15;
#endif 	/* __Fields15_FWD_DEFINED__ */
#ifndef __Fields20_FWD_DEFINED__
#define __Fields20_FWD_DEFINED__
typedef interface Fields20 Fields20;
#endif 	/* __Fields20_FWD_DEFINED__ */
#ifndef __Fields_FWD_DEFINED__
#define __Fields_FWD_DEFINED__
typedef interface ADOFields Fields;
#endif 	/* __Fields_FWD_DEFINED__ */
#ifndef ___Parameter_FWD_DEFINED__
#define ___Parameter_FWD_DEFINED__
typedef interface _ADOParameter _Parameter;
#endif 	/* ___Parameter_FWD_DEFINED__ */
#ifndef __Parameter_FWD_DEFINED__
#define __Parameter_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOParameter Parameter;
#else
typedef struct ADOParameter Parameter;
#endif /* __cplusplus */
#endif 	/* __Parameter_FWD_DEFINED__ */
#ifndef __Parameters_FWD_DEFINED__
#define __Parameters_FWD_DEFINED__
typedef interface ADOParameters Parameters;
#endif 	/* __Parameters_FWD_DEFINED__ */
#ifndef __Property_FWD_DEFINED__
#define __Property_FWD_DEFINED__
typedef interface ADOProperty Property;
#endif 	/* __Property_FWD_DEFINED__ */
#ifndef __Properties_FWD_DEFINED__
#define __Properties_FWD_DEFINED__
typedef interface ADOProperties Properties;
#endif 	/* __Properties_FWD_DEFINED__ */
#ifdef __cplusplus
extern "C"{
#endif 
/* interface __MIDL_itf_ado10_0000_0000 */
/* [local] */ 
// VSTS 15030784: Wv17: Cleanup C4091: onecore\enduser\databaseaccess\src\mdac\shared\ado. See work item for more details on this suppression.
#pragma warning(push) // For C4091
#pragma warning(disable:4091) // '': ignored on left of 'type' when no variable is declared
#if 0
typedef /* [uuid][public] */  DECLSPEC_UUID("54D8B4B9-663B-4a9c-95F6-0E749ABD70F1") __int64 ADO_LONGPTR;
typedef /* [uuid][public] */  DECLSPEC_UUID("54D8B4B9-663B-4a9c-95F6-0E749ABD70F1") long ADO_LONGPTR;
#endif
#ifdef _WIN64
// Number of rows
typedef LONGLONG				ADO_LONGPTR;
#else
// Number of rows
typedef LONG ADO_LONGPTR;
#endif	// _WIN64
extern RPC_IF_HANDLE __MIDL_itf_ado10_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ado10_0000_0000_v0_0_s_ifspec;
#ifndef __ADODB_LIBRARY_DEFINED__
#define __ADODB_LIBRARY_DEFINED__
/* library ADODB */
/* [helpstring][helpfile][version][uuid] */ 
typedef /* [uuid][helpcontext][public] */  DECLSPEC_UUID("0000051B-0000-0010-8000-00AA006D2EA4") 
enum CursorTypeEnum
    {	adOpenUnspecified	= -1,
	adOpenForwardOnly	= 0,
	adOpenKeyset	= 1,
	adOpenDynamic	= 2,
	adOpenStatic	= 3
    } 	CursorTypeEnum;
typedef /* [uuid][helpcontext] */  DECLSPEC_UUID("0000051C-0000-0010-8000-00AA006D2EA4") 
enum CursorOptionEnum
    {	adHoldRecords	= 0x100,
	adMovePrevious	= 0x200,
	adAddNew	= 0x1000400,
	adDelete	= 0x1000800,
	adUpdate	= 0x1008000,
	adBookmark	= 0x2000,
	adApproxPosition	= 0x4000,
	adUpdateBatch	= 0x10000,
	adResync	= 0x20000,
	adNotify	= 0x40000,
	adFind	= 0x80000,
	adSeek	= 0x400000,
	adIndex	= 0x800000
    } 	CursorOptionEnum;
typedef /* [uuid][helpcontext] */  DECLSPEC_UUID("0000051D-0000-0010-8000-00AA006D2EA4") 
enum LockTypeEnum
    {	adLockUnspecified	= -1,
	adLockReadOnly	= 1,
	adLockPessimistic	= 2,
	adLockOptimistic	= 3,
	adLockBatchOptimistic	= 4
    } 	LockTypeEnum;
typedef /* [uuid][helpcontext] */  DECLSPEC_UUID("0000051E-0000-0010-8000-00AA006D2EA4") 
enum ExecuteOptionEnum
    {	adOptionUnspecified	= -1,
	adAsyncExecute	= 0x10,
	adAsyncFetch	= 0x20,
	adAsyncFetchNonBlocking	= 0x40,
	adExecuteNoRecords	= 0x80,
	adExecuteStream	= 0x400,
	adExecuteRecord	= 0x800
    } 	ExecuteOptionEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000541-0000-0010-8000-00AA006D2EA4") 
enum ConnectOptionEnum
    {	adConnectUnspecified	= -1,
	adAsyncConnect	= 0x10
    } 	ConnectOptionEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000532-0000-0010-8000-00AA006D2EA4") 
enum ObjectStateEnum
    {	adStateClosed	= 0,
	adStateOpen	= 0x1,
	adStateConnecting	= 0x2,
	adStateExecuting	= 0x4,
	adStateFetching	= 0x8
    } 	ObjectStateEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000052F-0000-0010-8000-00AA006D2EA4") 
enum CursorLocationEnum
    {	adUseNone	= 1,
	adUseServer	= 2,
	adUseClient	= 3,
	adUseClientBatch	= 3
    } 	CursorLocationEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000051F-0000-0010-8000-00AA006D2EA4") 
enum DataTypeEnum
    {	adEmpty	= 0,
	adTinyInt	= 16,
	adSmallInt	= 2,
	adInteger	= 3,
	adBigInt	= 20,
	adUnsignedTinyInt	= 17,
	adUnsignedSmallInt	= 18,
	adUnsignedInt	= 19,
	adUnsignedBigInt	= 21,
	adSingle	= 4,
	adDouble	= 5,
	adCurrency	= 6,
	adDecimal	= 14,
	adNumeric	= 131,
	adBoolean	= 11,
	adError	= 10,
	adUserDefined	= 132,
	adVariant	= 12,
	adIDispatch	= 9,
	adIUnknown	= 13,
	adGUID	= 72,
	adDate	= 7,
	adDBDate	= 133,
	adDBTime	= 134,
	adDBTimeStamp	= 135,
	adBSTR	= 8,
	adChar	= 129,
	adVarChar	= 200,
	adLongVarChar	= 201,
	adWChar	= 130,
	adVarWChar	= 202,
	adLongVarWChar	= 203,
	adBinary	= 128,
	adVarBinary	= 204,
	adLongVarBinary	= 205,
	adChapter	= 136,
	adFileTime	= 64,
	adPropVariant	= 138,
	adVarNumeric	= 139,
	adArray	= 0x2000
    } 	DataTypeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000525-0000-0010-8000-00AA006D2EA4") 
enum FieldAttributeEnum
    {	adFldUnspecified	= -1,
	adFldMayDefer	= 0x2,
	adFldUpdatable	= 0x4,
	adFldUnknownUpdatable	= 0x8,
	adFldFixed	= 0x10,
	adFldIsNullable	= 0x20,
	adFldMayBeNull	= 0x40,
	adFldLong	= 0x80,
	adFldRowID	= 0x100,
	adFldRowVersion	= 0x200,
	adFldCacheDeferred	= 0x1000,
	adFldIsChapter	= 0x2000,
	adFldNegativeScale	= 0x4000,
	adFldKeyColumn	= 0x8000,
	adFldIsRowURL	= 0x10000,
	adFldIsDefaultStream	= 0x20000,
	adFldIsCollection	= 0x40000
    } 	FieldAttributeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000526-0000-0010-8000-00AA006D2EA4") 
enum EditModeEnum
    {	adEditNone	= 0,
	adEditInProgress	= 0x1,
	adEditAdd	= 0x2,
	adEditDelete	= 0x4
    } 	EditModeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000527-0000-0010-8000-00AA006D2EA4") 
enum RecordStatusEnum
    {	adRecOK	= 0,
	adRecNew	= 0x1,
	adRecModified	= 0x2,
	adRecDeleted	= 0x4,
	adRecUnmodified	= 0x8,
	adRecInvalid	= 0x10,
	adRecMultipleChanges	= 0x40,
	adRecPendingChanges	= 0x80,
	adRecCanceled	= 0x100,
	adRecCantRelease	= 0x400,
	adRecConcurrencyViolation	= 0x800,
	adRecIntegrityViolation	= 0x1000,
	adRecMaxChangesExceeded	= 0x2000,
	adRecObjectOpen	= 0x4000,
	adRecOutOfMemory	= 0x8000,
	adRecPermissionDenied	= 0x10000,
	adRecSchemaViolation	= 0x20000,
	adRecDBDeleted	= 0x40000
    } 	RecordStatusEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000542-0000-0010-8000-00AA006D2EA4") 
enum GetRowsOptionEnum
    {	adGetRowsRest	= -1
    } 	GetRowsOptionEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000528-0000-0010-8000-00AA006D2EA4") 
enum PositionEnum
    {	adPosUnknown	= -1,
	adPosBOF	= -2,
	adPosEOF	= -3
    } 	PositionEnum;
#if 0
typedef /* [uuid][public] */  DECLSPEC_UUID("A56187C5-D690-4037-AE32-A00EDC376AC3") __int64 PositionEnum_Param;
typedef /* [uuid][public] */  DECLSPEC_UUID("A56187C5-D690-4037-AE32-A00EDC376AC3") PositionEnum PositionEnum_Param;
#endif
#ifdef _WIN64
	typedef LONGLONG PositionEnum_Param;
#else
	typedef PositionEnum PositionEnum_Param;
#endif
typedef /* [helpcontext] */ 
enum BookmarkEnum
    {	adBookmarkCurrent	= 0,
	adBookmarkFirst	= 1,
	adBookmarkLast	= 2
    } 	BookmarkEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000540-0000-0010-8000-00AA006D2EA4") 
enum MarshalOptionsEnum
    {	adMarshalAll	= 0,
	adMarshalModifiedOnly	= 1
    } 	MarshalOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000543-0000-0010-8000-00AA006D2EA4") 
enum AffectEnum
    {	adAffectCurrent	= 1,
	adAffectGroup	= 2,
	adAffectAll	= 3,
	adAffectAllChapters	= 4
    } 	AffectEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000544-0000-0010-8000-00AA006D2EA4") 
enum ResyncEnum
    {	adResyncUnderlyingValues	= 1,
	adResyncAllValues	= 2
    } 	ResyncEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000545-0000-0010-8000-00AA006D2EA4") 
enum CompareEnum
    {	adCompareLessThan	= 0,
	adCompareEqual	= 1,
	adCompareGreaterThan	= 2,
	adCompareNotEqual	= 3,
	adCompareNotComparable	= 4
    } 	CompareEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000546-0000-0010-8000-00AA006D2EA4") 
enum FilterGroupEnum
    {	adFilterNone	= 0,
	adFilterPendingRecords	= 1,
	adFilterAffectedRecords	= 2,
	adFilterFetchedRecords	= 3,
	adFilterPredicate	= 4,
	adFilterConflictingRecords	= 5
    } 	FilterGroupEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000547-0000-0010-8000-00AA006D2EA4") 
enum SearchDirectionEnum
    {	adSearchForward	= 1,
	adSearchBackward	= -1
    } 	SearchDirectionEnum;
typedef /* [hidden] */ SearchDirectionEnum SearchDirection;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000548-0000-0010-8000-00AA006D2EA4") 
enum PersistFormatEnum
    {	adPersistADTG	= 0,
	adPersistXML	= 1
    } 	PersistFormatEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000549-0000-0010-8000-00AA006D2EA4") 
enum StringFormatEnum
    {	adClipString	= 2
    } 	StringFormatEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000520-0000-0010-8000-00AA006D2EA4") 
enum ConnectPromptEnum
    {	adPromptAlways	= 1,
	adPromptComplete	= 2,
	adPromptCompleteRequired	= 3,
	adPromptNever	= 4
    } 	ConnectPromptEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000521-0000-0010-8000-00AA006D2EA4") 
enum ConnectModeEnum
    {	adModeUnknown	= 0,
	adModeRead	= 1,
	adModeWrite	= 2,
	adModeReadWrite	= 3,
	adModeShareDenyRead	= 4,
	adModeShareDenyWrite	= 8,
	adModeShareExclusive	= 0xc,
	adModeShareDenyNone	= 0x10,
	adModeRecursive	= 0x400000
    } 	ConnectModeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000570-0000-0010-8000-00AA006D2EA4") 
enum RecordCreateOptionsEnum
    {	adCreateCollection	= 0x2000,
	adCreateStructDoc	= 0x80000000,
	adCreateNonCollection	= 0,
	adOpenIfExists	= 0x2000000,
	adCreateOverwrite	= 0x4000000,
	adFailIfNotExists	= -1
    } 	RecordCreateOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000571-0000-0010-8000-00AA006D2EA4") 
enum RecordOpenOptionsEnum
    {	adOpenRecordUnspecified	= -1,
	adOpenSource	= 0x800000,
	adOpenOutput	= 0x800000,
	adOpenAsync	= 0x1000,
	adDelayFetchStream	= 0x4000,
	adDelayFetchFields	= 0x8000,
	adOpenExecuteCommand	= 0x10000
    } 	RecordOpenOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000523-0000-0010-8000-00AA006D2EA4") 
enum IsolationLevelEnum
    {	adXactUnspecified	= 0xffffffff,
	adXactChaos	= 0x10,
	adXactReadUncommitted	= 0x100,
	adXactBrowse	= 0x100,
	adXactCursorStability	= 0x1000,
	adXactReadCommitted	= 0x1000,
	adXactRepeatableRead	= 0x10000,
	adXactSerializable	= 0x100000,
	adXactIsolated	= 0x100000
    } 	IsolationLevelEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000524-0000-0010-8000-00AA006D2EA4") 
enum XactAttributeEnum
    {	adXactCommitRetaining	= 0x20000,
	adXactAbortRetaining	= 0x40000,
	adXactAsyncPhaseOne	= 0x80000,
	adXactSyncPhaseOne	= 0x100000
    } 	XactAttributeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000529-0000-0010-8000-00AA006D2EA4") 
enum PropertyAttributesEnum
    {	adPropNotSupported	= 0,
	adPropRequired	= 0x1,
	adPropOptional	= 0x2,
	adPropRead	= 0x200,
	adPropWrite	= 0x400
    } 	PropertyAttributesEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000052A-0000-0010-8000-00AA006D2EA4") 
enum ErrorValueEnum
    {	adErrProviderFailed	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbb8),
	adErrInvalidArgument	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbb9),
	adErrOpeningFile	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbba),
	adErrReadFile	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbbb),
	adErrWriteFile	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbbc),
	adErrNoCurrentRecord	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xbcd),
	adErrIllegalOperation	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xc93),
	adErrCantChangeProvider	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xc94),
	adErrInTransaction	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xcae),
	adErrFeatureNotAvailable	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xcb3),
	adErrItemNotFound	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xcc1),
	adErrObjectInCollection	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xd27),
	adErrObjectNotSet	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xd5c),
	adErrDataConversion	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xd5d),
	adErrObjectClosed	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe78),
	adErrObjectOpen	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe79),
	adErrProviderNotFound	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7a),
	adErrBoundToCommand	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7b),
	adErrInvalidParamInfo	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7c),
	adErrInvalidConnection	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7d),
	adErrNotReentrant	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7e),
	adErrStillExecuting	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe7f),
	adErrOperationCancelled	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe80),
	adErrStillConnecting	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe81),
	adErrInvalidTransaction	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe82),
	adErrNotExecuting	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe83),
	adErrUnsafeOperation	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe84),
	adwrnSecurityDialog	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe85),
	adwrnSecurityDialogHeader	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe86),
	adErrIntegrityViolation	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe87),
	adErrPermissionDenied	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe88),
	adErrDataOverflow	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe89),
	adErrSchemaViolation	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8a),
	adErrSignMismatch	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8b),
	adErrCantConvertvalue	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8c),
	adErrCantCreate	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8d),
	adErrColumnNotOnThisRow	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8e),
	adErrURLDoesNotExist	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe8f),
	adErrTreePermissionDenied	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe90),
	adErrInvalidURL	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe91),
	adErrResourceLocked	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe92),
	adErrResourceExists	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe93),
	adErrCannotComplete	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe94),
	adErrVolumeNotFound	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe95),
	adErrOutOfSpace	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe96),
	adErrResourceOutOfScope	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe97),
	adErrUnavailable	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe98),
	adErrURLNamedRowDoesNotExist	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe99),
	adErrDelResOutOfScope	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9a),
	adErrPropInvalidColumn	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9b),
	adErrPropInvalidOption	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9c),
	adErrPropInvalidValue	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9d),
	adErrPropConflicting	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9e),
	adErrPropNotAllSettable	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xe9f),
	adErrPropNotSet	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea0),
	adErrPropNotSettable	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea1),
	adErrPropNotSupported	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea2),
	adErrCatalogNotSet	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea3),
	adErrCantChangeConnection	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea4),
	adErrFieldsUpdateFailed	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea5),
	adErrDenyNotSupported	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea6),
	adErrDenyTypeNotSupported	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea7),
	adErrProviderNotSpecified	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xea9),
	adErrConnectionStringTooLong	= MAKE_HRESULT(SEVERITY_ERROR, FACILITY_CONTROL,  0xeaa)
    } 	ErrorValueEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000052B-0000-0010-8000-00AA006D2EA4") 
enum ParameterAttributesEnum
    {	adParamSigned	= 0x10,
	adParamNullable	= 0x40,
	adParamLong	= 0x80
    } 	ParameterAttributesEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000052C-0000-0010-8000-00AA006D2EA4") 
enum ParameterDirectionEnum
    {	adParamUnknown	= 0,
	adParamInput	= 0x1,
	adParamOutput	= 0x2,
	adParamInputOutput	= 0x3,
	adParamReturnValue	= 0x4
    } 	ParameterDirectionEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000052E-0000-0010-8000-00AA006D2EA4") 
enum CommandTypeEnum
    {	adCmdUnspecified	= -1,
	adCmdUnknown	= 0x8,
	adCmdText	= 0x1,
	adCmdTable	= 0x2,
	adCmdStoredProc	= 0x4,
	adCmdFile	= 0x100,
	adCmdTableDirect	= 0x200
    } 	CommandTypeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000530-0000-0010-8000-00AA006D2EA4") 
enum EventStatusEnum
    {	adStatusOK	= 0x1,
	adStatusErrorsOccurred	= 0x2,
	adStatusCantDeny	= 0x3,
	adStatusCancel	= 0x4,
	adStatusUnwantedEvent	= 0x5
    } 	EventStatusEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000531-0000-0010-8000-00AA006D2EA4") 
enum EventReasonEnum
    {	adRsnAddNew	= 1,
	adRsnDelete	= 2,
	adRsnUpdate	= 3,
	adRsnUndoUpdate	= 4,
	adRsnUndoAddNew	= 5,
	adRsnUndoDelete	= 6,
	adRsnRequery	= 7,
	adRsnResynch	= 8,
	adRsnClose	= 9,
	adRsnMove	= 10,
	adRsnFirstChange	= 11,
	adRsnMoveFirst	= 12,
	adRsnMoveNext	= 13,
	adRsnMovePrevious	= 14,
	adRsnMoveLast	= 15
    } 	EventReasonEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000533-0000-0010-8000-00AA006D2EA4") 
enum SchemaEnum
    {	adSchemaProviderSpecific	= -1,
	adSchemaAsserts	= 0,
	adSchemaCatalogs	= 1,
	adSchemaCharacterSets	= 2,
	adSchemaCollations	= 3,
	adSchemaColumns	= 4,
	adSchemaCheckConstraints	= 5,
	adSchemaConstraintColumnUsage	= 6,
	adSchemaConstraintTableUsage	= 7,
	adSchemaKeyColumnUsage	= 8,
	adSchemaReferentialContraints	= 9,
	adSchemaReferentialConstraints	= 9,
	adSchemaTableConstraints	= 10,
	adSchemaColumnsDomainUsage	= 11,
	adSchemaIndexes	= 12,
	adSchemaColumnPrivileges	= 13,
	adSchemaTablePrivileges	= 14,
	adSchemaUsagePrivileges	= 15,
	adSchemaProcedures	= 16,
	adSchemaSchemata	= 17,
	adSchemaSQLLanguages	= 18,
	adSchemaStatistics	= 19,
	adSchemaTables	= 20,
	adSchemaTranslations	= 21,
	adSchemaProviderTypes	= 22,
	adSchemaViews	= 23,
	adSchemaViewColumnUsage	= 24,
	adSchemaViewTableUsage	= 25,
	adSchemaProcedureParameters	= 26,
	adSchemaForeignKeys	= 27,
	adSchemaPrimaryKeys	= 28,
	adSchemaProcedureColumns	= 29,
	adSchemaDBInfoKeywords	= 30,
	adSchemaDBInfoLiterals	= 31,
	adSchemaCubes	= 32,
	adSchemaDimensions	= 33,
	adSchemaHierarchies	= 34,
	adSchemaLevels	= 35,
	adSchemaMeasures	= 36,
	adSchemaProperties	= 37,
	adSchemaMembers	= 38,
	adSchemaTrustees	= 39,
	adSchemaFunctions	= 40,
	adSchemaActions	= 41,
	adSchemaCommands	= 42,
	adSchemaSets	= 43
    } 	SchemaEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000057E-0000-0010-8000-00AA006D2EA4") 
enum FieldStatusEnum
    {	adFieldOK	= 0,
	adFieldCantConvertValue	= 2,
	adFieldIsNull	= 3,
	adFieldTruncated	= 4,
	adFieldSignMismatch	= 5,
	adFieldDataOverflow	= 6,
	adFieldCantCreate	= 7,
	adFieldUnavailable	= 8,
	adFieldPermissionDenied	= 9,
	adFieldIntegrityViolation	= 10,
	adFieldSchemaViolation	= 11,
	adFieldBadStatus	= 12,
	adFieldDefault	= 13,
	adFieldIgnore	= 15,
	adFieldDoesNotExist	= 16,
	adFieldInvalidURL	= 17,
	adFieldResourceLocked	= 18,
	adFieldResourceExists	= 19,
	adFieldCannotComplete	= 20,
	adFieldVolumeNotFound	= 21,
	adFieldOutOfSpace	= 22,
	adFieldCannotDeleteSource	= 23,
	adFieldReadOnly	= 24,
	adFieldResourceOutOfScope	= 25,
	adFieldAlreadyExists	= 26,
	adFieldPendingInsert	= 0x10000,
	adFieldPendingDelete	= 0x20000,
	adFieldPendingChange	= 0x40000,
	adFieldPendingUnknown	= 0x80000,
	adFieldPendingUnknownDelete	= 0x100000
    } 	FieldStatusEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000552-0000-0010-8000-00AA006D2EA4") 
enum SeekEnum
    {	adSeekFirstEQ	= 0x1,
	adSeekLastEQ	= 0x2,
	adSeekAfterEQ	= 0x4,
	adSeekAfter	= 0x8,
	adSeekBeforeEQ	= 0x10,
	adSeekBefore	= 0x20
    } 	SeekEnum;
#ifndef _COMMON_ADC_AND_ADO_PROPS_
#define _COMMON_ADC_AND_ADO_PROPS_
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000054A-0000-0010-8000-00AA006D2EA4") 
enum ADCPROP_UPDATECRITERIA_ENUM
    {	adCriteriaKey	= 0,
	adCriteriaAllCols	= 1,
	adCriteriaUpdCols	= 2,
	adCriteriaTimeStamp	= 3
    } 	ADCPROP_UPDATECRITERIA_ENUM;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000054B-0000-0010-8000-00AA006D2EA4") 
enum ADCPROP_ASYNCTHREADPRIORITY_ENUM
    {	adPriorityLowest	= 1,
	adPriorityBelowNormal	= 2,
	adPriorityNormal	= 3,
	adPriorityAboveNormal	= 4,
	adPriorityHighest	= 5
    } 	ADCPROP_ASYNCTHREADPRIORITY_ENUM;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000554-0000-0010-8000-00AA006D2EA4") 
enum ADCPROP_AUTORECALC_ENUM
    {	adRecalcUpFront	= 0,
	adRecalcAlways	= 1
    } 	ADCPROP_AUTORECALC_ENUM;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000553-0000-0010-8000-00AA006D2EA4") 
enum ADCPROP_UPDATERESYNC_ENUM
    {	adResyncNone	= 0,
	adResyncAutoIncrement	= 1,
	adResyncConflicts	= 2,
	adResyncUpdates	= 4,
	adResyncInserts	= 8,
	adResyncAll	= 15
    } 	ADCPROP_UPDATERESYNC_ENUM;
#endif	/* _COMMON_ADC_AND_ADO_PROPS_ */
typedef ADCPROP_UPDATERESYNC_ENUM CEResyncEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000573-0000-0010-8000-00AA006D2EA4") 
enum MoveRecordOptionsEnum
    {	adMoveUnspecified	= -1,
	adMoveOverWrite	= 1,
	adMoveDontUpdateLinks	= 2,
	adMoveAllowEmulation	= 4
    } 	MoveRecordOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000574-0000-0010-8000-00AA006D2EA4") 
enum CopyRecordOptionsEnum
    {	adCopyUnspecified	= -1,
	adCopyOverWrite	= 1,
	adCopyAllowEmulation	= 4,
	adCopyNonRecursive	= 2
    } 	CopyRecordOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000576-0000-0010-8000-00AA006D2EA4") 
enum StreamTypeEnum
    {	adTypeBinary	= 1,
	adTypeText	= 2
    } 	StreamTypeEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("00000577-0000-0010-8000-00AA006D2EA4") 
enum LineSeparatorEnum
    {	adLF	= 10,
	adCR	= 13,
	adCRLF	= -1
    } 	LineSeparatorEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000057A-0000-0010-8000-00AA006D2EA4") 
enum StreamOpenOptionsEnum
    {	adOpenStreamUnspecified	= -1,
	adOpenStreamAsync	= 1,
	adOpenStreamFromRecord	= 4
    } 	StreamOpenOptionsEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000057B-0000-0010-8000-00AA006D2EA4") 
enum StreamWriteEnum
    {	adWriteChar	= 0,
	adWriteLine	= 1,
	stWriteChar	= 0,
	stWriteLine	= 1
    } 	StreamWriteEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000057C-0000-0010-8000-00AA006D2EA4") 
enum SaveOptionsEnum
    {	adSaveCreateNotExist	= 1,
	adSaveCreateOverWrite	= 2
    } 	SaveOptionsEnum;
typedef /* [helpcontext] */ 
enum FieldEnum
    {	adDefaultStream	= -1,
	adRecordURL	= -2
    } 	FieldEnum;
typedef /* [helpcontext] */ 
enum StreamReadEnum
    {	adReadAll	= -1,
	adReadLine	= -2
    } 	StreamReadEnum;
typedef /* [helpcontext][uuid] */  DECLSPEC_UUID("0000057D-0000-0010-8000-00AA006D2EA4") 
enum RecordTypeEnum
    {	adSimpleRecord	= 0,
	adCollectionRecord	= 1,
	adStructDoc	= 2
    } 	RecordTypeEnum;
EXTERN_C const IID LIBID_ADODB;
#ifndef ___ADOCollection_INTERFACE_DEFINED__
#define ___ADOCollection_INTERFACE_DEFINED__
/* interface _ADOCollection */
/* [object][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__ADOCollection;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000512-0000-0010-8000-00AA006D2EA4")
    _ADOCollection : public IDispatch
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *c) = 0;
        
        virtual /* [id][restricted] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual /* [id][helpcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _ADOCollectionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOCollection * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOCollection * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOCollection * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in _ADOCollection * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in _ADOCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in _ADOCollection * This);
        
        END_INTERFACE
    } _ADOCollectionVtbl;
    interface _ADOCollection
    {
        CONST_VTBL struct _ADOCollectionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _ADOCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _ADOCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _ADOCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _ADOCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _ADOCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _ADOCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _ADOCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Collection_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define _ADOCollection__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define _ADOCollection_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___ADOCollection_INTERFACE_DEFINED__ */
#ifndef ___ADODynaCollection_INTERFACE_DEFINED__
#define ___ADODynaCollection_INTERFACE_DEFINED__
/* interface _ADODynaCollection */
/* [object][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__ADODynaCollection;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000513-0000-0010-8000-00AA006D2EA4")
    _ADODynaCollection : public _ADOCollection
    {
    public:
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IDispatch *Object) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Index) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _ADODynaCollectionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADODynaCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADODynaCollection * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADODynaCollection * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADODynaCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADODynaCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADODynaCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADODynaCollection * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in _ADODynaCollection * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in _ADODynaCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in _ADODynaCollection * This);
        
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in _ADODynaCollection * This,
            /* [in] */ __RPC__in_opt IDispatch *Object);
        
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in _ADODynaCollection * This,
            /* [in] */ VARIANT Index);
        
        END_INTERFACE
    } _ADODynaCollectionVtbl;
    interface _ADODynaCollection
    {
        CONST_VTBL struct _ADODynaCollectionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _ADODynaCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _ADODynaCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _ADODynaCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _ADODynaCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _ADODynaCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _ADODynaCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _ADODynaCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _DynaCollection_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define _ADODynaCollection__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define _ADODynaCollection_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define _ADODynaCollection_Append(This,Object)	\
    ( (This)->lpVtbl -> Append(This,Object) ) 
#define _ADODynaCollection_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___ADODynaCollection_INTERFACE_DEFINED__ */
#ifndef ___ADO_INTERFACE_DEFINED__
#define ___ADO_INTERFACE_DEFINED__
/* interface _ADO */
/* [object][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__ADO;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000534-0000-0010-8000-00AA006D2EA4")
    _ADO : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _ADOVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADO * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADO * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADO * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADO * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADO * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADO * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        END_INTERFACE
    } _ADOVtbl;
    interface _ADO
    {
        CONST_VTBL struct _ADOVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _ADO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _ADO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _ADO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _ADO_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _ADO_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _ADO_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _ADO_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _ADO_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___ADO_INTERFACE_DEFINED__ */
#ifndef __Error_INTERFACE_DEFINED__
#define __Error_INTERFACE_DEFINED__
/* interface ADOError */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Error;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000500-0000-0010-8000-00AA006D2EA4")
    ADOError : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Number( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_HelpFile( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_HelpContext( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_SQLState( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_NativeError( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ErrorVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOError * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOError * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOError * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOError * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOError * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Number )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HelpFile )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HelpContext )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SQLState )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NativeError )( 
            __RPC__in struct ADOError * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        END_INTERFACE
    } ErrorVtbl;
    interface Error
    {
        CONST_VTBL struct ErrorVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Error_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Error_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Error_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Error_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Error_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Error_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Error_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Error_get_Number(This,pl)	\
    ( (This)->lpVtbl -> get_Number(This,pl) ) 
#define Error_get_Source(This,pbstr)	\
    ( (This)->lpVtbl -> get_Source(This,pbstr) ) 
#define Error_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 
#define Error_get_HelpFile(This,pbstr)	\
    ( (This)->lpVtbl -> get_HelpFile(This,pbstr) ) 
#define Error_get_HelpContext(This,pl)	\
    ( (This)->lpVtbl -> get_HelpContext(This,pl) ) 
#define Error_get_SQLState(This,pbstr)	\
    ( (This)->lpVtbl -> get_SQLState(This,pbstr) ) 
#define Error_get_NativeError(This,pl)	\
    ( (This)->lpVtbl -> get_NativeError(This,pl) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Error_INTERFACE_DEFINED__ */
#ifndef __Errors_INTERFACE_DEFINED__
#define __Errors_INTERFACE_DEFINED__
/* interface ADOErrors */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Errors;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000501-0000-0010-8000-00AA006D2EA4")
    ADOErrors : public _ADOCollection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt ADOError **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ErrorsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOErrors * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOErrors * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOErrors * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOErrors * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOErrors * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOErrors * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOErrors * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in struct ADOErrors * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in struct ADOErrors * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in struct ADOErrors * This);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in struct ADOErrors * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOError **ppvObject);
        
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in struct ADOErrors * This);
        
        END_INTERFACE
    } ErrorsVtbl;
    interface Errors
    {
        CONST_VTBL struct ErrorsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Errors_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Errors_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Errors_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Errors_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Errors_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Errors_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Errors_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Errors_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Errors__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Errors_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Errors_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#define Errors_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Errors_INTERFACE_DEFINED__ */
#ifndef __Command15_INTERFACE_DEFINED__
#define __Command15_INTERFACE_DEFINED__
/* interface Command15 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Command15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000508-0000-0010-8000-00AA006D2EA4")
    Command15 : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__deref_out_opt _ADOConnection **ppvObject) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt _ADOConnection *pCon) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ VARIANT vConn) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CommandText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CommandText( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CommandTimeout( 
            /* [retval][out] */ __RPC__out LONG *pl) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CommandTimeout( 
            /* [in] */ LONG Timeout) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Prepared( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPrepared) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Prepared( 
            /* [in] */ VARIANT_BOOL fPrepared) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Execute( 
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [optional][in] */ __RPC__in VARIANT *Parameters,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppirs) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CreateParameter( 
            /* [defaultvalue][in] */ __RPC__in BSTR Name,
            /* [defaultvalue][in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ParameterDirectionEnum Direction,
            /* [defaultvalue][in] */ ADO_LONGPTR Size,
            /* [optional][in] */ VARIANT Value,
            /* [retval][out] */ __RPC__deref_out_opt _ADOParameter **ppiprm) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parameters( 
            /* [retval][out] */ __RPC__deref_out_opt ADOParameters **ppvObject) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CommandType( 
            /* [in] */ CommandTypeEnum lCmdType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CommandType( 
            /* [retval][out] */ __RPC__out CommandTypeEnum *plCmdType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Command15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Command15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Command15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Command15 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Command15 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Command15 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Command15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Command15 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOConnection **ppvObject);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in Command15 * This,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pCon);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in Command15 * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandText )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandText )( 
            __RPC__in Command15 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandTimeout )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__out LONG *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandTimeout )( 
            __RPC__in Command15 * This,
            /* [in] */ LONG Timeout);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Prepared )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPrepared);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Prepared )( 
            __RPC__in Command15 * This,
            /* [in] */ VARIANT_BOOL fPrepared);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in Command15 * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [optional][in] */ __RPC__in VARIANT *Parameters,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppirs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateParameter )( 
            __RPC__in Command15 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Name,
            /* [defaultvalue][in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ParameterDirectionEnum Direction,
            /* [defaultvalue][in] */ ADO_LONGPTR Size,
            /* [optional][in] */ VARIANT Value,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOParameter **ppiprm);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parameters )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOParameters **ppvObject);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandType )( 
            __RPC__in Command15 * This,
            /* [in] */ CommandTypeEnum lCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandType )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__out CommandTypeEnum *plCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Command15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in Command15 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        END_INTERFACE
    } Command15Vtbl;
    interface Command15
    {
        CONST_VTBL struct Command15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Command15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Command15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Command15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Command15_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Command15_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Command15_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Command15_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Command15_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Command15_get_ActiveConnection(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppvObject) ) 
#define Command15_putref_ActiveConnection(This,pCon)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pCon) ) 
#define Command15_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define Command15_get_CommandText(This,pbstr)	\
    ( (This)->lpVtbl -> get_CommandText(This,pbstr) ) 
#define Command15_put_CommandText(This,bstr)	\
    ( (This)->lpVtbl -> put_CommandText(This,bstr) ) 
#define Command15_get_CommandTimeout(This,pl)	\
    ( (This)->lpVtbl -> get_CommandTimeout(This,pl) ) 
#define Command15_put_CommandTimeout(This,Timeout)	\
    ( (This)->lpVtbl -> put_CommandTimeout(This,Timeout) ) 
#define Command15_get_Prepared(This,pfPrepared)	\
    ( (This)->lpVtbl -> get_Prepared(This,pfPrepared) ) 
#define Command15_put_Prepared(This,fPrepared)	\
    ( (This)->lpVtbl -> put_Prepared(This,fPrepared) ) 
#define Command15_Execute(This,RecordsAffected,Parameters,Options,ppirs)	\
    ( (This)->lpVtbl -> Execute(This,RecordsAffected,Parameters,Options,ppirs) ) 
#define Command15_CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm)	\
    ( (This)->lpVtbl -> CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm) ) 
#define Command15_get_Parameters(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Parameters(This,ppvObject) ) 
#define Command15_put_CommandType(This,lCmdType)	\
    ( (This)->lpVtbl -> put_CommandType(This,lCmdType) ) 
#define Command15_get_CommandType(This,plCmdType)	\
    ( (This)->lpVtbl -> get_CommandType(This,plCmdType) ) 
#define Command15_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 
#define Command15_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Command15_INTERFACE_DEFINED__ */
#ifndef __Command25_INTERFACE_DEFINED__
#define __Command25_INTERFACE_DEFINED__
/* interface Command25 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Command25;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000054E-0000-0010-8000-00AA006D2EA4")
    Command25 : public Command15
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out LONG *plObjState) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Command25Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Command25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Command25 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Command25 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Command25 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Command25 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Command25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Command25 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOConnection **ppvObject);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in Command25 * This,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pCon);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in Command25 * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandText )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandText )( 
            __RPC__in Command25 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandTimeout )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__out LONG *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandTimeout )( 
            __RPC__in Command25 * This,
            /* [in] */ LONG Timeout);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Prepared )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPrepared);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Prepared )( 
            __RPC__in Command25 * This,
            /* [in] */ VARIANT_BOOL fPrepared);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in Command25 * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [optional][in] */ __RPC__in VARIANT *Parameters,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppirs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateParameter )( 
            __RPC__in Command25 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Name,
            /* [defaultvalue][in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ParameterDirectionEnum Direction,
            /* [defaultvalue][in] */ ADO_LONGPTR Size,
            /* [optional][in] */ VARIANT Value,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOParameter **ppiprm);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parameters )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOParameters **ppvObject);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandType )( 
            __RPC__in Command25 * This,
            /* [in] */ CommandTypeEnum lCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandType )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__out CommandTypeEnum *plCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in Command25 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in Command25 * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in Command25 * This);
        
        END_INTERFACE
    } Command25Vtbl;
    interface Command25
    {
        CONST_VTBL struct Command25Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Command25_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Command25_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Command25_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Command25_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Command25_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Command25_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Command25_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Command25_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Command25_get_ActiveConnection(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppvObject) ) 
#define Command25_putref_ActiveConnection(This,pCon)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pCon) ) 
#define Command25_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define Command25_get_CommandText(This,pbstr)	\
    ( (This)->lpVtbl -> get_CommandText(This,pbstr) ) 
#define Command25_put_CommandText(This,bstr)	\
    ( (This)->lpVtbl -> put_CommandText(This,bstr) ) 
#define Command25_get_CommandTimeout(This,pl)	\
    ( (This)->lpVtbl -> get_CommandTimeout(This,pl) ) 
#define Command25_put_CommandTimeout(This,Timeout)	\
    ( (This)->lpVtbl -> put_CommandTimeout(This,Timeout) ) 
#define Command25_get_Prepared(This,pfPrepared)	\
    ( (This)->lpVtbl -> get_Prepared(This,pfPrepared) ) 
#define Command25_put_Prepared(This,fPrepared)	\
    ( (This)->lpVtbl -> put_Prepared(This,fPrepared) ) 
#define Command25_Execute(This,RecordsAffected,Parameters,Options,ppirs)	\
    ( (This)->lpVtbl -> Execute(This,RecordsAffected,Parameters,Options,ppirs) ) 
#define Command25_CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm)	\
    ( (This)->lpVtbl -> CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm) ) 
#define Command25_get_Parameters(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Parameters(This,ppvObject) ) 
#define Command25_put_CommandType(This,lCmdType)	\
    ( (This)->lpVtbl -> put_CommandType(This,lCmdType) ) 
#define Command25_get_CommandType(This,plCmdType)	\
    ( (This)->lpVtbl -> get_CommandType(This,plCmdType) ) 
#define Command25_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 
#define Command25_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 
#define Command25_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define Command25_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Command25_INTERFACE_DEFINED__ */
#ifndef ___Command_INTERFACE_DEFINED__
#define ___Command_INTERFACE_DEFINED__
/* interface _ADOCommand */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__Command;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B08400BD-F9D1-4D02-B856-71D5DBA123E9")
    _ADOCommand : public Command25
    {
    public:
        virtual /* [helpcontext][propputref][id] */ HRESULT __stdcall putref_CommandStream( 
            /* [in] */ __RPC__in_opt IUnknown *pStream) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT __stdcall get_CommandStream( 
            /* [retval][out] */ __RPC__out VARIANT *pvStream) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT __stdcall put_Dialect( 
            /* [in] */ __RPC__in BSTR bstrDialect) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT __stdcall get_Dialect( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDialect) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT __stdcall put_NamedParameters( 
            /* [in] */ VARIANT_BOOL fNamedParameters) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT __stdcall get_NamedParameters( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfNamedParameters) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _CommandVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADOCommand * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADOCommand * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADOCommand * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADOCommand * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOConnection **ppvObject);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pCon);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandText )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandText )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandTimeout )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out LONG *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandTimeout )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ LONG Timeout);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Prepared )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPrepared);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Prepared )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ VARIANT_BOOL fPrepared);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in struct _ADOCommand * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [optional][in] */ __RPC__in VARIANT *Parameters,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppirs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CreateParameter )( 
            __RPC__in struct _ADOCommand * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Name,
            /* [defaultvalue][in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ParameterDirectionEnum Direction,
            /* [defaultvalue][in] */ ADO_LONGPTR Size,
            /* [optional][in] */ VARIANT Value,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOParameter **ppiprm);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parameters )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOParameters **ppvObject);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CommandType )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ CommandTypeEnum lCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CommandType )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out CommandTypeEnum *plCmdType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in struct _ADOCommand * This);
        
        /* [helpcontext][propputref][id] */ HRESULT ( __stdcall *putref_CommandADOStream )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in_opt IUnknown *pStream);
        
        /* [helpcontext][propget][id] */ HRESULT ( __stdcall *get_CommandStream )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out VARIANT *pvStream);
        
        /* [helpcontext][propput][id] */ HRESULT ( __stdcall *put_Dialect )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ __RPC__in BSTR bstrDialect);
        
        /* [helpcontext][propget][id] */ HRESULT ( __stdcall *get_Dialect )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDialect);
        
        /* [helpcontext][propput][id] */ HRESULT ( __stdcall *put_NamedParameters )( 
            __RPC__in struct _ADOCommand * This,
            /* [in] */ VARIANT_BOOL fNamedParameters);
        
        /* [helpcontext][propget][id] */ HRESULT ( __stdcall *get_NamedParameters )( 
            __RPC__in struct _ADOCommand * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfNamedParameters);
        
        END_INTERFACE
    } _CommandVtbl;
    interface _Command
    {
        CONST_VTBL struct _CommandVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Command_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Command_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Command_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Command_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Command_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Command_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Command_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Command_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Command_get_ActiveConnection(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppvObject) ) 
#define _Command_putref_ActiveConnection(This,pCon)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pCon) ) 
#define _Command_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define _Command_get_CommandText(This,pbstr)	\
    ( (This)->lpVtbl -> get_CommandText(This,pbstr) ) 
#define _Command_put_CommandText(This,bstr)	\
    ( (This)->lpVtbl -> put_CommandText(This,bstr) ) 
#define _Command_get_CommandTimeout(This,pl)	\
    ( (This)->lpVtbl -> get_CommandTimeout(This,pl) ) 
#define _Command_put_CommandTimeout(This,Timeout)	\
    ( (This)->lpVtbl -> put_CommandTimeout(This,Timeout) ) 
#define _Command_get_Prepared(This,pfPrepared)	\
    ( (This)->lpVtbl -> get_Prepared(This,pfPrepared) ) 
#define _Command_put_Prepared(This,fPrepared)	\
    ( (This)->lpVtbl -> put_Prepared(This,fPrepared) ) 
#define _Command_Execute(This,RecordsAffected,Parameters,Options,ppirs)	\
    ( (This)->lpVtbl -> Execute(This,RecordsAffected,Parameters,Options,ppirs) ) 
#define _Command_CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm)	\
    ( (This)->lpVtbl -> CreateParameter(This,Name,Type,Direction,Size,Value,ppiprm) ) 
#define _Command_get_Parameters(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Parameters(This,ppvObject) ) 
#define _Command_put_CommandType(This,lCmdType)	\
    ( (This)->lpVtbl -> put_CommandType(This,lCmdType) ) 
#define _Command_get_CommandType(This,plCmdType)	\
    ( (This)->lpVtbl -> get_CommandType(This,plCmdType) ) 
#define _Command_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 
#define _Command_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 
#define _Command_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define _Command_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#define _Command_putref_CommandStream(This,pStream)	\
    ( (This)->lpVtbl -> putref_CommandStream(This,pStream) ) 
#define _Command_get_CommandStream(This,pvStream)	\
    ( (This)->lpVtbl -> get_CommandStream(This,pvStream) ) 
#define _Command_put_Dialect(This,bstrDialect)	\
    ( (This)->lpVtbl -> put_Dialect(This,bstrDialect) ) 
#define _Command_get_Dialect(This,pbstrDialect)	\
    ( (This)->lpVtbl -> get_Dialect(This,pbstrDialect) ) 
#define _Command_put_NamedParameters(This,fNamedParameters)	\
    ( (This)->lpVtbl -> put_NamedParameters(This,fNamedParameters) ) 
#define _Command_get_NamedParameters(This,pfNamedParameters)	\
    ( (This)->lpVtbl -> get_NamedParameters(This,pfNamedParameters) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Command_INTERFACE_DEFINED__ */
#ifndef __ConnectionEventsVt_INTERFACE_DEFINED__
#define __ConnectionEventsVt_INTERFACE_DEFINED__
/* interface ConnectionEventsVt */
/* [object][uuid][hidden] */ 
EXTERN_C const IID IID_ConnectionEventsVt;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000402-0000-0010-8000-00AA006D2EA4")
    ConnectionEventsVt : public IUnknown
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE InfoMessage( 
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE BeginTransComplete( 
            /* [in] */ LONG TransactionLevel,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CommitTransComplete( 
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE RollbackTransComplete( 
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillExecute( 
            /* [out][in] */ __RPC__deref_inout_opt BSTR *Source,
            /* [out][in] */ __RPC__inout CursorTypeEnum *CursorType,
            /* [out][in] */ __RPC__inout LockTypeEnum *LockType,
            /* [out][in] */ __RPC__inout long *Options,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOCommand *pCommand,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE ExecuteComplete( 
            /* [in] */ LONG RecordsAffected,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOCommand *pCommand,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillConnect( 
            /* [out][in] */ __RPC__deref_inout_opt BSTR *ConnectionString,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *UserID,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *Password,
            /* [out][in] */ __RPC__inout long *Options,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE ConnectComplete( 
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADOConnection *pConnection) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ConnectionEventsVtVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ConnectionEventsVt * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ConnectionEventsVt * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *InfoMessage )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *BeginTransComplete )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ LONG TransactionLevel,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CommitTransComplete )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackTransComplete )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillExecute )( 
            __RPC__in ConnectionEventsVt * This,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *Source,
            /* [out][in] */ __RPC__inout CursorTypeEnum *CursorType,
            /* [out][in] */ __RPC__inout LockTypeEnum *LockType,
            /* [out][in] */ __RPC__inout long *Options,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOCommand *pCommand,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteComplete )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ LONG RecordsAffected,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOCommand *pCommand,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillConnect )( 
            __RPC__in ConnectionEventsVt * This,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *ConnectionString,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *UserID,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *Password,
            /* [out][in] */ __RPC__inout long *Options,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectComplete )( 
            __RPC__in ConnectionEventsVt * This,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in ConnectionEventsVt * This,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADOConnection *pConnection);
        
        END_INTERFACE
    } ConnectionEventsVtVtbl;
    interface ConnectionEventsVt
    {
        CONST_VTBL struct ConnectionEventsVtVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ConnectionEventsVt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ConnectionEventsVt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ConnectionEventsVt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ConnectionEventsVt_InfoMessage(This,pError,adStatus,pConnection)	\
    ( (This)->lpVtbl -> InfoMessage(This,pError,adStatus,pConnection) ) 
#define ConnectionEventsVt_BeginTransComplete(This,TransactionLevel,pError,adStatus,pConnection)	\
    ( (This)->lpVtbl -> BeginTransComplete(This,TransactionLevel,pError,adStatus,pConnection) ) 
#define ConnectionEventsVt_CommitTransComplete(This,pError,adStatus,pConnection)	\
    ( (This)->lpVtbl -> CommitTransComplete(This,pError,adStatus,pConnection) ) 
#define ConnectionEventsVt_RollbackTransComplete(This,pError,adStatus,pConnection)	\
    ( (This)->lpVtbl -> RollbackTransComplete(This,pError,adStatus,pConnection) ) 
#define ConnectionEventsVt_WillExecute(This,Source,CursorType,LockType,Options,adStatus,pCommand,pRecordset,pConnection)	\
    ( (This)->lpVtbl -> WillExecute(This,Source,CursorType,LockType,Options,adStatus,pCommand,pRecordset,pConnection) ) 
#define ConnectionEventsVt_ExecuteComplete(This,RecordsAffected,pError,adStatus,pCommand,pRecordset,pConnection)	\
    ( (This)->lpVtbl -> ExecuteComplete(This,RecordsAffected,pError,adStatus,pCommand,pRecordset,pConnection) ) 
#define ConnectionEventsVt_WillConnect(This,ConnectionString,UserID,Password,Options,adStatus,pConnection)	\
    ( (This)->lpVtbl -> WillConnect(This,ConnectionString,UserID,Password,Options,adStatus,pConnection) ) 
#define ConnectionEventsVt_ConnectComplete(This,pError,adStatus,pConnection)	\
    ( (This)->lpVtbl -> ConnectComplete(This,pError,adStatus,pConnection) ) 
#define ConnectionEventsVt_Disconnect(This,adStatus,pConnection)	\
    ( (This)->lpVtbl -> Disconnect(This,adStatus,pConnection) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ConnectionEventsVt_INTERFACE_DEFINED__ */
#ifndef __RecordsetEventsVt_INTERFACE_DEFINED__
#define __RecordsetEventsVt_INTERFACE_DEFINED__
/* interface RecordsetEventsVt */
/* [object][uuid][hidden] */ 
EXTERN_C const IID IID_RecordsetEventsVt;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000403-0000-0010-8000-00AA006D2EA4")
    RecordsetEventsVt : public IUnknown
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillChangeField( 
            /* [in] */ LONG cFields,
            /* [in] */ VARIANT Fields,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE FieldChangeComplete( 
            /* [in] */ LONG cFields,
            /* [in] */ VARIANT Fields,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillChangeRecord( 
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ LONG cRecords,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE RecordChangeComplete( 
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ LONG cRecords,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillChangeRecordset( 
            /* [in] */ EventReasonEnum adReason,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE RecordsetChangeComplete( 
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WillMove( 
            /* [in] */ EventReasonEnum adReason,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MoveComplete( 
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE EndOfRecordset( 
            /* [out][in] */ __RPC__inout VARIANT_BOOL *fMoreData,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE FetchProgress( 
            /* [in] */ long Progress,
            /* [in] */ long MaxProgress,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE FetchComplete( 
            /* [in] */ __RPC__in_opt ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt _ADORecordset *pRecordset) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct RecordsetEventsVtVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in RecordsetEventsVt * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in RecordsetEventsVt * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillChangeADOField )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ LONG cFields,
            /* [in] */ VARIANT Fields,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *FieldChangeComplete )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ LONG cFields,
            /* [in] */ VARIANT Fields,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillChangeADORecord )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ LONG cRecords,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *RecordChangeComplete )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ LONG cRecords,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillChangeADORecordset )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *RecordsetChangeComplete )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WillMove )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveComplete )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ EventReasonEnum adReason,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *EndOfADORecordset )( 
            __RPC__in RecordsetEventsVt * This,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *fMoreData,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *FetchProgress )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ long Progress,
            /* [in] */ long MaxProgress,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *FetchComplete )( 
            __RPC__in RecordsetEventsVt * This,
            /* [in] */ __RPC__in_opt struct ADOError *pError,
            /* [out][in] */ __RPC__inout EventStatusEnum *adStatus,
            /* [in] */ __RPC__in_opt struct _ADORecordset *pRecordset);
        
        END_INTERFACE
    } RecordsetEventsVtVtbl;
    interface RecordsetEventsVt
    {
        CONST_VTBL struct RecordsetEventsVtVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define RecordsetEventsVt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define RecordsetEventsVt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define RecordsetEventsVt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define RecordsetEventsVt_WillChangeField(This,cFields,Fields,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> WillChangeField(This,cFields,Fields,adStatus,pRecordset) ) 
#define RecordsetEventsVt_FieldChangeComplete(This,cFields,Fields,pError,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> FieldChangeComplete(This,cFields,Fields,pError,adStatus,pRecordset) ) 
#define RecordsetEventsVt_WillChangeRecord(This,adReason,cRecords,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> WillChangeRecord(This,adReason,cRecords,adStatus,pRecordset) ) 
#define RecordsetEventsVt_RecordChangeComplete(This,adReason,cRecords,pError,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> RecordChangeComplete(This,adReason,cRecords,pError,adStatus,pRecordset) ) 
#define RecordsetEventsVt_WillChangeRecordset(This,adReason,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> WillChangeRecordset(This,adReason,adStatus,pRecordset) ) 
#define RecordsetEventsVt_RecordsetChangeComplete(This,adReason,pError,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> RecordsetChangeComplete(This,adReason,pError,adStatus,pRecordset) ) 
#define RecordsetEventsVt_WillMove(This,adReason,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> WillMove(This,adReason,adStatus,pRecordset) ) 
#define RecordsetEventsVt_MoveComplete(This,adReason,pError,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> MoveComplete(This,adReason,pError,adStatus,pRecordset) ) 
#define RecordsetEventsVt_EndOfRecordset(This,fMoreData,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> EndOfRecordset(This,fMoreData,adStatus,pRecordset) ) 
#define RecordsetEventsVt_FetchProgress(This,Progress,MaxProgress,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> FetchProgress(This,Progress,MaxProgress,adStatus,pRecordset) ) 
#define RecordsetEventsVt_FetchComplete(This,pError,adStatus,pRecordset)	\
    ( (This)->lpVtbl -> FetchComplete(This,pError,adStatus,pRecordset) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __RecordsetEventsVt_INTERFACE_DEFINED__ */
#ifndef __ConnectionEvents_DISPINTERFACE_DEFINED__
#define __ConnectionEvents_DISPINTERFACE_DEFINED__
/* dispinterface ConnectionEvents */
/* [uuid] */ 
EXTERN_C const IID DIID_ConnectionEvents;
#if defined(__cplusplus) && !defined(CINTERFACE)
    MIDL_INTERFACE("00000400-0000-0010-8000-00AA006D2EA4")
    ConnectionEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */
    typedef struct ConnectionEventsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ConnectionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ConnectionEvents * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ConnectionEvents * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ConnectionEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ConnectionEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ConnectionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ConnectionEvents * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        END_INTERFACE
    } ConnectionEventsVtbl;
    interface ConnectionEvents
    {
        CONST_VTBL struct ConnectionEventsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ConnectionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ConnectionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ConnectionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ConnectionEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define ConnectionEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define ConnectionEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define ConnectionEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ConnectionEvents_DISPINTERFACE_DEFINED__ */
#ifndef __RecordsetEvents_DISPINTERFACE_DEFINED__
#define __RecordsetEvents_DISPINTERFACE_DEFINED__
/* dispinterface RecordsetEvents */
/* [uuid] */ 
EXTERN_C const IID DIID_RecordsetEvents;
#if defined(__cplusplus) && !defined(CINTERFACE)
    MIDL_INTERFACE("00000266-0000-0010-8000-00AA006D2EA4")
    RecordsetEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */
    typedef struct RecordsetEventsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in RecordsetEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in RecordsetEvents * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in RecordsetEvents * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in RecordsetEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in RecordsetEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in RecordsetEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            RecordsetEvents * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        END_INTERFACE
    } RecordsetEventsVtbl;
    interface RecordsetEvents
    {
        CONST_VTBL struct RecordsetEventsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define RecordsetEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define RecordsetEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define RecordsetEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define RecordsetEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define RecordsetEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define RecordsetEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define RecordsetEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __RecordsetEvents_DISPINTERFACE_DEFINED__ */
#ifndef __Connection15_INTERFACE_DEFINED__
#define __Connection15_INTERFACE_DEFINED__
/* interface Connection15 */
/* [object][helpcontext][uuid][hidden][dual] */ 
EXTERN_C const IID IID_Connection15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000515-0000-0010-8000-00AA006D2EA4")
    Connection15 : public _ADO
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectionString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ConnectionString( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommandTimeout( 
            /* [retval][out] */ __RPC__out LONG *plTimeout) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_CommandTimeout( 
            /* [in] */ LONG lTimeout) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectionTimeout( 
            /* [retval][out] */ __RPC__out LONG *plTimeout) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ConnectionTimeout( 
            /* [in] */ LONG lTimeout) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Execute( 
            /* [in] */ __RPC__in BSTR CommandText,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppiRset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE BeginTrans( 
            /* [retval][out] */ __RPC__out long *TransactionLevel) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CommitTrans( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE RollbackTrans( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [defaultvalue][in] */ __RPC__in BSTR ConnectionString = NULL,
            /* [defaultvalue][in] */ __RPC__in BSTR UserID = NULL,
            /* [defaultvalue][in] */ __RPC__in BSTR Password = NULL,
            /* [defaultvalue][in] */ long Options = adOptionUnspecified) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Errors( 
            /* [retval][out] */ __RPC__deref_out_opt ADOErrors **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultDatabase( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultDatabase( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsolationLevel( 
            /* [retval][out] */ __RPC__out IsolationLevelEnum *Level) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_IsolationLevel( 
            /* [in] */ IsolationLevelEnum Level) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out long *plAttr) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ long lAttr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CursorLocation( 
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CursorLocation( 
            /* [in] */ CursorLocationEnum lCursorLoc) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out ConnectModeEnum *plMode) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ ConnectModeEnum lMode) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Provider( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Provider( 
            /* [in] */ __RPC__in BSTR Provider) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out LONG *plObjState) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE OpenSchema( 
            /* [in] */ SchemaEnum Schema,
            /* [optional][in] */ VARIANT Restrictions,
            /* [optional][in] */ VARIANT SchemaID,
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **pprset) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Connection15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Connection15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Connection15 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Connection15 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Connection15 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Connection15 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionString )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectionString )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommandTimeout )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out LONG *plTimeout);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CommandTimeout )( 
            __RPC__in Connection15 * This,
            /* [in] */ LONG lTimeout);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionTimeout )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out LONG *plTimeout);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectionTimeout )( 
            __RPC__in Connection15 * This,
            /* [in] */ LONG lTimeout);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in Connection15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in BSTR CommandText,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *BeginTrans )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out long *TransactionLevel);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CommitTrans )( 
            __RPC__in Connection15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackTrans )( 
            __RPC__in Connection15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in Connection15 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR ConnectionString,
            /* [defaultvalue][in] */ __RPC__in BSTR UserID,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ long Options);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Errors )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOErrors **ppvObject);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultDatabase )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultDatabase )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsolationLevel )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out IsolationLevelEnum *Level);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsolationLevel )( 
            __RPC__in Connection15 * This,
            /* [in] */ IsolationLevelEnum Level);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out long *plAttr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in Connection15 * This,
            /* [in] */ long lAttr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in Connection15 * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out ConnectModeEnum *plMode);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in Connection15 * This,
            /* [in] */ ConnectModeEnum lMode);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Provider )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Provider )( 
            __RPC__in Connection15 * This,
            /* [in] */ __RPC__in BSTR Provider);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in Connection15 * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *OpenSchema )( 
            __RPC__in Connection15 * This,
            /* [in] */ SchemaEnum Schema,
            /* [optional][in] */ VARIANT Restrictions,
            /* [optional][in] */ VARIANT SchemaID,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **pprset);
        
        END_INTERFACE
    } Connection15Vtbl;
    interface Connection15
    {
        CONST_VTBL struct Connection15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Connection15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Connection15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Connection15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Connection15_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Connection15_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Connection15_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Connection15_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Connection15_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Connection15_get_ConnectionString(This,pbstr)	\
    ( (This)->lpVtbl -> get_ConnectionString(This,pbstr) ) 
#define Connection15_put_ConnectionString(This,bstr)	\
    ( (This)->lpVtbl -> put_ConnectionString(This,bstr) ) 
#define Connection15_get_CommandTimeout(This,plTimeout)	\
    ( (This)->lpVtbl -> get_CommandTimeout(This,plTimeout) ) 
#define Connection15_put_CommandTimeout(This,lTimeout)	\
    ( (This)->lpVtbl -> put_CommandTimeout(This,lTimeout) ) 
#define Connection15_get_ConnectionTimeout(This,plTimeout)	\
    ( (This)->lpVtbl -> get_ConnectionTimeout(This,plTimeout) ) 
#define Connection15_put_ConnectionTimeout(This,lTimeout)	\
    ( (This)->lpVtbl -> put_ConnectionTimeout(This,lTimeout) ) 
#define Connection15_get_Version(This,pbstr)	\
    ( (This)->lpVtbl -> get_Version(This,pbstr) ) 
#define Connection15_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define Connection15_Execute(This,CommandText,RecordsAffected,Options,ppiRset)	\
    ( (This)->lpVtbl -> Execute(This,CommandText,RecordsAffected,Options,ppiRset) ) 
#define Connection15_BeginTrans(This,TransactionLevel)	\
    ( (This)->lpVtbl -> BeginTrans(This,TransactionLevel) ) 
#define Connection15_CommitTrans(This)	\
    ( (This)->lpVtbl -> CommitTrans(This) ) 
#define Connection15_RollbackTrans(This)	\
    ( (This)->lpVtbl -> RollbackTrans(This) ) 
#define Connection15_Open(This,ConnectionString,UserID,Password,Options)	\
    ( (This)->lpVtbl -> Open(This,ConnectionString,UserID,Password,Options) ) 
#define Connection15_get_Errors(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Errors(This,ppvObject) ) 
#define Connection15_get_DefaultDatabase(This,pbstr)	\
    ( (This)->lpVtbl -> get_DefaultDatabase(This,pbstr) ) 
#define Connection15_put_DefaultDatabase(This,bstr)	\
    ( (This)->lpVtbl -> put_DefaultDatabase(This,bstr) ) 
#define Connection15_get_IsolationLevel(This,Level)	\
    ( (This)->lpVtbl -> get_IsolationLevel(This,Level) ) 
#define Connection15_put_IsolationLevel(This,Level)	\
    ( (This)->lpVtbl -> put_IsolationLevel(This,Level) ) 
#define Connection15_get_Attributes(This,plAttr)	\
    ( (This)->lpVtbl -> get_Attributes(This,plAttr) ) 
#define Connection15_put_Attributes(This,lAttr)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttr) ) 
#define Connection15_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define Connection15_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define Connection15_get_Mode(This,plMode)	\
    ( (This)->lpVtbl -> get_Mode(This,plMode) ) 
#define Connection15_put_Mode(This,lMode)	\
    ( (This)->lpVtbl -> put_Mode(This,lMode) ) 
#define Connection15_get_Provider(This,pbstr)	\
    ( (This)->lpVtbl -> get_Provider(This,pbstr) ) 
#define Connection15_put_Provider(This,Provider)	\
    ( (This)->lpVtbl -> put_Provider(This,Provider) ) 
#define Connection15_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define Connection15_OpenSchema(This,Schema,Restrictions,SchemaID,pprset)	\
    ( (This)->lpVtbl -> OpenSchema(This,Schema,Restrictions,SchemaID,pprset) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Connection15_INTERFACE_DEFINED__ */
#ifndef ___Connection_INTERFACE_DEFINED__
#define ___Connection_INTERFACE_DEFINED__
/* interface _ADOConnection */
/* [object][helpcontext][uuid][dual] */ 
EXTERN_C const IID IID__Connection;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000550-0000-0010-8000-00AA006D2EA4")
    _ADOConnection : public Connection15
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _ConnectionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADOConnection * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADOConnection * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADOConnection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADOConnection * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionString )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectionString )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommandTimeout )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out LONG *plTimeout);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CommandTimeout )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ LONG lTimeout);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionTimeout )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out LONG *plTimeout);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectionTimeout )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ LONG lTimeout);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in struct _ADOConnection * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in BSTR CommandText,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [defaultvalue][in] */ long Options,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *BeginTrans )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out long *TransactionLevel);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CommitTrans )( 
            __RPC__in struct _ADOConnection * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackTrans )( 
            __RPC__in struct _ADOConnection * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in struct _ADOConnection * This,
            /* [defaultvalue][in] */ __RPC__in BSTR ConnectionString,
            /* [defaultvalue][in] */ __RPC__in BSTR UserID,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ long Options);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Errors )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOErrors **ppvObject);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultDatabase )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultDatabase )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsolationLevel )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out IsolationLevelEnum *Level);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsolationLevel )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ IsolationLevelEnum Level);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out long *plAttr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ long lAttr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out ConnectModeEnum *plMode);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ ConnectModeEnum lMode);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Provider )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Provider )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ __RPC__in BSTR Provider);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in struct _ADOConnection * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *OpenSchema )( 
            __RPC__in struct _ADOConnection * This,
            /* [in] */ SchemaEnum Schema,
            /* [optional][in] */ VARIANT Restrictions,
            /* [optional][in] */ VARIANT SchemaID,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **pprset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in struct _ADOConnection * This);
        
        END_INTERFACE
    } _ConnectionVtbl;
    interface _Connection
    {
        CONST_VTBL struct _ConnectionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Connection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Connection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Connection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Connection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Connection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Connection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Connection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Connection_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Connection_get_ConnectionString(This,pbstr)	\
    ( (This)->lpVtbl -> get_ConnectionString(This,pbstr) ) 
#define _Connection_put_ConnectionString(This,bstr)	\
    ( (This)->lpVtbl -> put_ConnectionString(This,bstr) ) 
#define _Connection_get_CommandTimeout(This,plTimeout)	\
    ( (This)->lpVtbl -> get_CommandTimeout(This,plTimeout) ) 
#define _Connection_put_CommandTimeout(This,lTimeout)	\
    ( (This)->lpVtbl -> put_CommandTimeout(This,lTimeout) ) 
#define _Connection_get_ConnectionTimeout(This,plTimeout)	\
    ( (This)->lpVtbl -> get_ConnectionTimeout(This,plTimeout) ) 
#define _Connection_put_ConnectionTimeout(This,lTimeout)	\
    ( (This)->lpVtbl -> put_ConnectionTimeout(This,lTimeout) ) 
#define _Connection_get_Version(This,pbstr)	\
    ( (This)->lpVtbl -> get_Version(This,pbstr) ) 
#define _Connection_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define _Connection_Execute(This,CommandText,RecordsAffected,Options,ppiRset)	\
    ( (This)->lpVtbl -> Execute(This,CommandText,RecordsAffected,Options,ppiRset) ) 
#define _Connection_BeginTrans(This,TransactionLevel)	\
    ( (This)->lpVtbl -> BeginTrans(This,TransactionLevel) ) 
#define _Connection_CommitTrans(This)	\
    ( (This)->lpVtbl -> CommitTrans(This) ) 
#define _Connection_RollbackTrans(This)	\
    ( (This)->lpVtbl -> RollbackTrans(This) ) 
#define _Connection_Open(This,ConnectionString,UserID,Password,Options)	\
    ( (This)->lpVtbl -> Open(This,ConnectionString,UserID,Password,Options) ) 
#define _Connection_get_Errors(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Errors(This,ppvObject) ) 
#define _Connection_get_DefaultDatabase(This,pbstr)	\
    ( (This)->lpVtbl -> get_DefaultDatabase(This,pbstr) ) 
#define _Connection_put_DefaultDatabase(This,bstr)	\
    ( (This)->lpVtbl -> put_DefaultDatabase(This,bstr) ) 
#define _Connection_get_IsolationLevel(This,Level)	\
    ( (This)->lpVtbl -> get_IsolationLevel(This,Level) ) 
#define _Connection_put_IsolationLevel(This,Level)	\
    ( (This)->lpVtbl -> put_IsolationLevel(This,Level) ) 
#define _Connection_get_Attributes(This,plAttr)	\
    ( (This)->lpVtbl -> get_Attributes(This,plAttr) ) 
#define _Connection_put_Attributes(This,lAttr)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttr) ) 
#define _Connection_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define _Connection_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define _Connection_get_Mode(This,plMode)	\
    ( (This)->lpVtbl -> get_Mode(This,plMode) ) 
#define _Connection_put_Mode(This,lMode)	\
    ( (This)->lpVtbl -> put_Mode(This,lMode) ) 
#define _Connection_get_Provider(This,pbstr)	\
    ( (This)->lpVtbl -> get_Provider(This,pbstr) ) 
#define _Connection_put_Provider(This,Provider)	\
    ( (This)->lpVtbl -> put_Provider(This,Provider) ) 
#define _Connection_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define _Connection_OpenSchema(This,Schema,Restrictions,SchemaID,pprset)	\
    ( (This)->lpVtbl -> OpenSchema(This,Schema,Restrictions,SchemaID,pprset) ) 
#define _Connection_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Connection_INTERFACE_DEFINED__ */
#ifndef __ADOConnectionConstruction15_INTERFACE_DEFINED__
#define __ADOConnectionConstruction15_INTERFACE_DEFINED__
/* interface ADOConnectionConstruction15 */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADOConnectionConstruction15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000516-0000-0010-8000-00AA006D2EA4")
    ADOConnectionConstruction15 : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DSO( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppDSO) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WrapDSOandSession( 
            /* [in] */ __RPC__in_opt IUnknown *pDSO,
            /* [in] */ __RPC__in_opt IUnknown *pSession) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ADOConnectionConstruction15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOConnectionConstruction15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOConnectionConstruction15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOConnectionConstruction15 * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DSO )( 
            __RPC__in ADOConnectionConstruction15 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppDSO);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in ADOConnectionConstruction15 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppSession);
        
        HRESULT ( STDMETHODCALLTYPE *WrapDSOandSession )( 
            __RPC__in ADOConnectionConstruction15 * This,
            /* [in] */ __RPC__in_opt IUnknown *pDSO,
            /* [in] */ __RPC__in_opt IUnknown *pSession);
        
        END_INTERFACE
    } ADOConnectionConstruction15Vtbl;
    interface ADOConnectionConstruction15
    {
        CONST_VTBL struct ADOConnectionConstruction15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADOConnectionConstruction15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADOConnectionConstruction15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADOConnectionConstruction15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADOConnectionConstruction15_get_DSO(This,ppDSO)	\
    ( (This)->lpVtbl -> get_DSO(This,ppDSO) ) 
#define ADOConnectionConstruction15_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 
#define ADOConnectionConstruction15_WrapDSOandSession(This,pDSO,pSession)	\
    ( (This)->lpVtbl -> WrapDSOandSession(This,pDSO,pSession) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADOConnectionConstruction15_INTERFACE_DEFINED__ */
#ifndef __ADOConnectionConstruction_INTERFACE_DEFINED__
#define __ADOConnectionConstruction_INTERFACE_DEFINED__
/* interface ADOConnectionConstruction */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADOConnectionConstruction;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000551-0000-0010-8000-00AA006D2EA4")
    ADOConnectionConstruction : public ADOConnectionConstruction15
    {
    public:
    };
    
#else 	/* C style interface */
    typedef struct ADOConnectionConstructionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOConnectionConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOConnectionConstruction * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOConnectionConstruction * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DSO )( 
            __RPC__in ADOConnectionConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppDSO);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in ADOConnectionConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppSession);
        
        HRESULT ( STDMETHODCALLTYPE *WrapDSOandSession )( 
            __RPC__in ADOConnectionConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pDSO,
            /* [in] */ __RPC__in_opt IUnknown *pSession);
        
        END_INTERFACE
    } ADOConnectionConstructionVtbl;
    interface ADOConnectionConstruction
    {
        CONST_VTBL struct ADOConnectionConstructionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADOConnectionConstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADOConnectionConstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADOConnectionConstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADOConnectionConstruction_get_DSO(This,ppDSO)	\
    ( (This)->lpVtbl -> get_DSO(This,ppDSO) ) 
#define ADOConnectionConstruction_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 
#define ADOConnectionConstruction_WrapDSOandSession(This,pDSO,pSession)	\
    ( (This)->lpVtbl -> WrapDSOandSession(This,pDSO,pSession) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADOConnectionConstruction_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Connection;
#ifdef __cplusplus
Connection;
#endif
#ifndef ___Record_INTERFACE_DEFINED__
#define ___Record_INTERFACE_DEFINED__
/* interface _ADORecord */
/* [object][uuid][helpcontext][hidden][dual] */ 
EXTERN_C const IID IID__Record;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000562-0000-0010-8000-00AA006D2EA4")
    _ADORecord : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ __RPC__in BSTR bstrConn) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt _ADOConnection *Con) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out ObjectStateEnum *pState) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Source( 
            /* [in] */ __RPC__in BSTR Source) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Source( 
            /* [in] */ __RPC__in_opt IDispatch *Source) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out ConnectModeEnum *pMode) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ ConnectModeEnum Mode) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ParentURL( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrParentURL) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MoveRecord( 
            /* [defaultvalue][in] */ __RPC__in BSTR Source,
            /* [defaultvalue][in] */ __RPC__in BSTR Destination,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ MoveRecordOptionsEnum Options,
            /* [defaultvalue][in] */ VARIANT_BOOL Async,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNewURL) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CopyRecord( 
            /* [defaultvalue][in] */ __RPC__in BSTR Source,
            /* [defaultvalue][in] */ __RPC__in BSTR Destination,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ CopyRecordOptionsEnum Options,
            /* [defaultvalue][in] */ VARIANT_BOOL Async,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNewURL) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE DeleteRecord( 
            /* [defaultvalue][in] */ __RPC__in BSTR Source = NULL,
            /* [defaultvalue][in] */ VARIANT_BOOL Async = 0) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ ConnectModeEnum Mode = adModeUnknown,
            /* [defaultvalue][in] */ RecordCreateOptionsEnum CreateOptions = adFailIfNotExists,
            /* [defaultvalue][in] */ RecordOpenOptionsEnum Options = adOpenRecordUnspecified,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName = NULL,
            /* [defaultvalue][in] */ __RPC__in BSTR Password = NULL) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Fields( 
            /* [retval][out] */ __RPC__deref_out_opt ADOFields **ppFlds) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_RecordType( 
            /* [retval][out] */ __RPC__out RecordTypeEnum *pType) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetChildren( 
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppRSet) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _RecordVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADORecord * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADORecord * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADORecord * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADORecord * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in_opt struct _ADOConnection *Con);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__out ObjectStateEnum *pState);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in BSTR Source);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ __RPC__in_opt IDispatch *Source);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__out ConnectModeEnum *pMode);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in struct _ADORecord * This,
            /* [in] */ ConnectModeEnum Mode);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ParentURL )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrParentURL);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveADORecord )( 
            __RPC__in struct _ADORecord * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Source,
            /* [defaultvalue][in] */ __RPC__in BSTR Destination,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ MoveRecordOptionsEnum Options,
            /* [defaultvalue][in] */ VARIANT_BOOL Async,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNewURL);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CopyADORecord )( 
            __RPC__in struct _ADORecord * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Source,
            /* [defaultvalue][in] */ __RPC__in BSTR Destination,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password,
            /* [defaultvalue][in] */ CopyRecordOptionsEnum Options,
            /* [defaultvalue][in] */ VARIANT_BOOL Async,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNewURL);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteADORecord )( 
            __RPC__in struct _ADORecord * This,
            /* [defaultvalue][in] */ __RPC__in BSTR Source,
            /* [defaultvalue][in] */ VARIANT_BOOL Async);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in struct _ADORecord * This,
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ ConnectModeEnum Mode,
            /* [defaultvalue][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [defaultvalue][in] */ RecordOpenOptionsEnum Options,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in struct _ADORecord * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOFields **ppFlds);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecordType )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__out RecordTypeEnum *pType);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetChildren )( 
            __RPC__in struct _ADORecord * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppRSet);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in struct _ADORecord * This);
        
        END_INTERFACE
    } _RecordVtbl;
    interface _Record
    {
        CONST_VTBL struct _RecordVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Record_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Record_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Record_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Record_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Record_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Record_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Record_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Record_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Record_get_ActiveConnection(This,pvar)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pvar) ) 
#define _Record_put_ActiveConnection(This,bstrConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,bstrConn) ) 
#define _Record_putref_ActiveConnection(This,Con)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,Con) ) 
#define _Record_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 
#define _Record_get_Source(This,pvar)	\
    ( (This)->lpVtbl -> get_Source(This,pvar) ) 
#define _Record_put_Source(This,Source)	\
    ( (This)->lpVtbl -> put_Source(This,Source) ) 
#define _Record_putref_Source(This,Source)	\
    ( (This)->lpVtbl -> putref_Source(This,Source) ) 
#define _Record_get_Mode(This,pMode)	\
    ( (This)->lpVtbl -> get_Mode(This,pMode) ) 
#define _Record_put_Mode(This,Mode)	\
    ( (This)->lpVtbl -> put_Mode(This,Mode) ) 
#define _Record_get_ParentURL(This,pbstrParentURL)	\
    ( (This)->lpVtbl -> get_ParentURL(This,pbstrParentURL) ) 
#define _Record_MoveRecord(This,Source,Destination,UserName,Password,Options,Async,pbstrNewURL)	\
    ( (This)->lpVtbl -> MoveRecord(This,Source,Destination,UserName,Password,Options,Async,pbstrNewURL) ) 
#define _Record_CopyRecord(This,Source,Destination,UserName,Password,Options,Async,pbstrNewURL)	\
    ( (This)->lpVtbl -> CopyRecord(This,Source,Destination,UserName,Password,Options,Async,pbstrNewURL) ) 
#define _Record_DeleteRecord(This,Source,Async)	\
    ( (This)->lpVtbl -> DeleteRecord(This,Source,Async) ) 
#define _Record_Open(This,Source,ActiveConnection,Mode,CreateOptions,Options,UserName,Password)	\
    ( (This)->lpVtbl -> Open(This,Source,ActiveConnection,Mode,CreateOptions,Options,UserName,Password) ) 
#define _Record_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define _Record_get_Fields(This,ppFlds)	\
    ( (This)->lpVtbl -> get_Fields(This,ppFlds) ) 
#define _Record_get_RecordType(This,pType)	\
    ( (This)->lpVtbl -> get_RecordType(This,pType) ) 
#define _Record_GetChildren(This,ppRSet)	\
    ( (This)->lpVtbl -> GetChildren(This,ppRSet) ) 
#define _Record_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Record_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Record;
#ifdef __cplusplus
Record;
#endif
#ifndef ___Stream_INTERFACE_DEFINED__
#define ___Stream_INTERFACE_DEFINED__
/* interface _ADOStream */
/* [object][helpcontext][uuid][hidden][dual] */ 
EXTERN_C const IID IID__Stream;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000565-0000-0010-8000-00AA006D2EA4")
    _ADOStream : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pSize) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_EOS( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEOS) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Position( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pPos) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Position( 
            /* [in] */ ADO_LONGPTR Position) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out StreamTypeEnum *pType) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ StreamTypeEnum Type) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_LineSeparator( 
            /* [retval][out] */ __RPC__out LineSeparatorEnum *pLS) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_LineSeparator( 
            /* [in] */ LineSeparatorEnum LineSeparator) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out ObjectStateEnum *pState) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out ConnectModeEnum *pMode) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ ConnectModeEnum Mode) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Charset( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCharset) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Charset( 
            /* [in] */ __RPC__in BSTR Charset) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Read( 
            /* [defaultvalue][in] */ long NumBytes,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [optional][in] */ VARIANT Source,
            /* [defaultvalue][in] */ ConnectModeEnum Mode = adModeUnknown,
            /* [defaultvalue][in] */ StreamOpenOptionsEnum Options = adOpenStreamUnspecified,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName = NULL,
            /* [defaultvalue][in] */ __RPC__in BSTR Password = NULL) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SkipLine( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ VARIANT Buffer) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetEOS( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CopyTo( 
            /* [in] */ __RPC__in_opt _ADOStream *DestStream,
            /* [defaultvalue][in] */ ADO_LONGPTR CharNumber = -1) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SaveToFile( 
            /* [in] */ __RPC__in BSTR FileName,
            /* [defaultvalue][in] */ SaveOptionsEnum Options = adSaveCreateNotExist) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE LoadFromFile( 
            /* [in] */ __RPC__in BSTR FileName) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE ReadText( 
            /* [defaultvalue][in] */ long NumChars,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE WriteText( 
            /* [in] */ __RPC__in BSTR Data,
            /* [defaultvalue][in] */ StreamWriteEnum Options = adWriteChar) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _StreamVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADOStream * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADOStream * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADOStream * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADOStream * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pSize);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EOS )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEOS);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Position )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pPos);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Position )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ ADO_LONGPTR Position);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out StreamTypeEnum *pType);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ StreamTypeEnum Type);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineSeparator )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out LineSeparatorEnum *pLS);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LineSeparator )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ LineSeparatorEnum LineSeparator);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out ObjectStateEnum *pState);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__out ConnectModeEnum *pMode);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ ConnectModeEnum Mode);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Charset )( 
            __RPC__in struct _ADOStream * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCharset);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Charset )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in BSTR Charset);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in struct _ADOStream * This,
            /* [defaultvalue][in] */ long NumBytes,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in struct _ADOStream * This,
            /* [optional][in] */ VARIANT Source,
            /* [defaultvalue][in] */ ConnectModeEnum Mode,
            /* [defaultvalue][in] */ StreamOpenOptionsEnum Options,
            /* [defaultvalue][in] */ __RPC__in BSTR UserName,
            /* [defaultvalue][in] */ __RPC__in BSTR Password);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in struct _ADOStream * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SkipLine )( 
            __RPC__in struct _ADOStream * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ VARIANT Buffer);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetEOS )( 
            __RPC__in struct _ADOStream * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in_opt struct _ADOStream *DestStream,
            /* [defaultvalue][in] */ ADO_LONGPTR CharNumber);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in struct _ADOStream * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SaveToFile )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in BSTR FileName,
            /* [defaultvalue][in] */ SaveOptionsEnum Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *LoadFromFile )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in BSTR FileName);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *ReadText )( 
            __RPC__in struct _ADOStream * This,
            /* [defaultvalue][in] */ long NumChars,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *WriteText )( 
            __RPC__in struct _ADOStream * This,
            /* [in] */ __RPC__in BSTR Data,
            /* [defaultvalue][in] */ StreamWriteEnum Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in struct _ADOStream * This);
        
        END_INTERFACE
    } _StreamVtbl;
    interface _Stream
    {
        CONST_VTBL struct _StreamVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Stream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Stream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Stream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Stream_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Stream_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Stream_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Stream_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Stream_get_Size(This,pSize)	\
    ( (This)->lpVtbl -> get_Size(This,pSize) ) 
#define _Stream_get_EOS(This,pEOS)	\
    ( (This)->lpVtbl -> get_EOS(This,pEOS) ) 
#define _Stream_get_Position(This,pPos)	\
    ( (This)->lpVtbl -> get_Position(This,pPos) ) 
#define _Stream_put_Position(This,Position)	\
    ( (This)->lpVtbl -> put_Position(This,Position) ) 
#define _Stream_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 
#define _Stream_put_Type(This,Type)	\
    ( (This)->lpVtbl -> put_Type(This,Type) ) 
#define _Stream_get_LineSeparator(This,pLS)	\
    ( (This)->lpVtbl -> get_LineSeparator(This,pLS) ) 
#define _Stream_put_LineSeparator(This,LineSeparator)	\
    ( (This)->lpVtbl -> put_LineSeparator(This,LineSeparator) ) 
#define _Stream_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 
#define _Stream_get_Mode(This,pMode)	\
    ( (This)->lpVtbl -> get_Mode(This,pMode) ) 
#define _Stream_put_Mode(This,Mode)	\
    ( (This)->lpVtbl -> put_Mode(This,Mode) ) 
#define _Stream_get_Charset(This,pbstrCharset)	\
    ( (This)->lpVtbl -> get_Charset(This,pbstrCharset) ) 
#define _Stream_put_Charset(This,Charset)	\
    ( (This)->lpVtbl -> put_Charset(This,Charset) ) 
#define _Stream_Read(This,NumBytes,pVal)	\
    ( (This)->lpVtbl -> Read(This,NumBytes,pVal) ) 
#define _Stream_Open(This,Source,Mode,Options,UserName,Password)	\
    ( (This)->lpVtbl -> Open(This,Source,Mode,Options,UserName,Password) ) 
#define _Stream_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define _Stream_SkipLine(This)	\
    ( (This)->lpVtbl -> SkipLine(This) ) 
#define _Stream_Write(This,Buffer)	\
    ( (This)->lpVtbl -> Write(This,Buffer) ) 
#define _Stream_SetEOS(This)	\
    ( (This)->lpVtbl -> SetEOS(This) ) 
#define _Stream_CopyTo(This,DestStream,CharNumber)	\
    ( (This)->lpVtbl -> CopyTo(This,DestStream,CharNumber) ) 
#define _Stream_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 
#define _Stream_SaveToFile(This,FileName,Options)	\
    ( (This)->lpVtbl -> SaveToFile(This,FileName,Options) ) 
#define _Stream_LoadFromFile(This,FileName)	\
    ( (This)->lpVtbl -> LoadFromFile(This,FileName) ) 
#define _Stream_ReadText(This,NumChars,pbstr)	\
    ( (This)->lpVtbl -> ReadText(This,NumChars,pbstr) ) 
#define _Stream_WriteText(This,Data,Options)	\
    ( (This)->lpVtbl -> WriteText(This,Data,Options) ) 
#define _Stream_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Stream_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Stream;
#ifdef __cplusplus
Stream;
#endif
#ifndef __ADORecordConstruction_INTERFACE_DEFINED__
#define __ADORecordConstruction_INTERFACE_DEFINED__
/* interface ADORecordConstruction */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADORecordConstruction;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000567-0000-0010-8000-00AA006D2EA4")
    ADORecordConstruction : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Row( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Row( 
            /* [in] */ __RPC__in_opt IUnknown *pRow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ParentRow( 
            /* [in] */ __RPC__in_opt IUnknown *pRow) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ADORecordConstructionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADORecordConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADORecordConstruction * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADORecordConstruction * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADORecordConstruction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADORecordConstruction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADORecordConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADORecordConstruction * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Row )( 
            __RPC__in ADORecordConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRow);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Row )( 
            __RPC__in ADORecordConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pRow);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ParentRow )( 
            __RPC__in ADORecordConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pRow);
        
        END_INTERFACE
    } ADORecordConstructionVtbl;
    interface ADORecordConstruction
    {
        CONST_VTBL struct ADORecordConstructionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADORecordConstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADORecordConstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADORecordConstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADORecordConstruction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define ADORecordConstruction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define ADORecordConstruction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define ADORecordConstruction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define ADORecordConstruction_get_Row(This,ppRow)	\
    ( (This)->lpVtbl -> get_Row(This,ppRow) ) 
#define ADORecordConstruction_put_Row(This,pRow)	\
    ( (This)->lpVtbl -> put_Row(This,pRow) ) 
#define ADORecordConstruction_put_ParentRow(This,pRow)	\
    ( (This)->lpVtbl -> put_ParentRow(This,pRow) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADORecordConstruction_INTERFACE_DEFINED__ */
#ifndef __ADOStreamConstruction_INTERFACE_DEFINED__
#define __ADOStreamConstruction_INTERFACE_DEFINED__
/* interface ADOStreamConstruction */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADOStreamConstruction;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000568-0000-0010-8000-00AA006D2EA4")
    ADOStreamConstruction : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Stream( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppStm) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Stream( 
            /* [in] */ __RPC__in_opt IUnknown *pStm) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ADOStreamConstructionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOStreamConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOStreamConstruction * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOStreamConstruction * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOStreamConstruction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOStreamConstruction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOStreamConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOStreamConstruction * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Stream )( 
            __RPC__in ADOStreamConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppStm);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Stream )( 
            __RPC__in ADOStreamConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pStm);
        
        END_INTERFACE
    } ADOStreamConstructionVtbl;
    interface ADOStreamConstruction
    {
        CONST_VTBL struct ADOStreamConstructionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADOStreamConstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADOStreamConstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADOStreamConstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADOStreamConstruction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define ADOStreamConstruction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define ADOStreamConstruction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define ADOStreamConstruction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define ADOStreamConstruction_get_Stream(This,ppStm)	\
    ( (This)->lpVtbl -> get_Stream(This,ppStm) ) 
#define ADOStreamConstruction_put_Stream(This,pStm)	\
    ( (This)->lpVtbl -> put_Stream(This,pStm) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADOStreamConstruction_INTERFACE_DEFINED__ */
#ifndef __ADOCommandConstruction_INTERFACE_DEFINED__
#define __ADOCommandConstruction_INTERFACE_DEFINED__
/* interface ADOCommandConstruction */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADOCommandConstruction;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000517-0000-0010-8000-00AA006D2EA4")
    ADOCommandConstruction : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OLEDBCommand( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppOLEDBCommand) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_OLEDBCommand( 
            /* [in] */ __RPC__in_opt IUnknown *pOLEDBCommand) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ADOCommandConstructionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOCommandConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOCommandConstruction * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOCommandConstruction * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OLEDBCommand )( 
            __RPC__in ADOCommandConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppOLEDBCommand);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_OLEDBCommand )( 
            __RPC__in ADOCommandConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pOLEDBCommand);
        
        END_INTERFACE
    } ADOCommandConstructionVtbl;
    interface ADOCommandConstruction
    {
        CONST_VTBL struct ADOCommandConstructionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADOCommandConstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADOCommandConstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADOCommandConstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADOCommandConstruction_get_OLEDBCommand(This,ppOLEDBCommand)	\
    ( (This)->lpVtbl -> get_OLEDBCommand(This,ppOLEDBCommand) ) 
#define ADOCommandConstruction_put_OLEDBCommand(This,pOLEDBCommand)	\
    ( (This)->lpVtbl -> put_OLEDBCommand(This,pOLEDBCommand) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADOCommandConstruction_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Command;
#ifdef __cplusplus
Command;
#endif
EXTERN_C const CLSID CLSID_Recordset;
#ifdef __cplusplus
Recordset;
#endif
#ifndef __Recordset15_INTERFACE_DEFINED__
#define __Recordset15_INTERFACE_DEFINED__
/* interface Recordset15 */
/* [object][helpcontext][uuid][nonextensible][hidden][dual] */ 
EXTERN_C const IID IID_Recordset15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000050E-0000-0010-8000-00AA006D2EA4")
    Recordset15 : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_AbsolutePosition( 
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_AbsolutePosition( 
            /* [in] */ PositionEnum_Param Position) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pconn) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ VARIANT vConn) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_BOF( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Bookmark( 
            /* [retval][out] */ __RPC__out VARIANT *pvBookmark) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Bookmark( 
            /* [in] */ VARIANT vBookmark) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CacheSize( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CacheSize( 
            /* [in] */ long CacheSize) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CursorType( 
            /* [retval][out] */ __RPC__out CursorTypeEnum *plCursorType) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CursorType( 
            /* [in] */ CursorTypeEnum lCursorType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_EOF( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Fields( 
            /* [retval][out] */ __RPC__deref_out_opt ADOFields **ppvObject) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_LockType( 
            /* [retval][out] */ __RPC__out LockTypeEnum *plLockType) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_LockType( 
            /* [in] */ LockTypeEnum lLockType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_MaxRecords( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plMaxRecords) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaxRecords( 
            /* [in] */ ADO_LONGPTR lMaxRecords) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_RecordCount( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Source( 
            /* [in] */ __RPC__in_opt IDispatch *pcmd) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Source( 
            /* [in] */ __RPC__in BSTR bstrConn) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__out VARIANT *pvSource) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE AddNew( 
            /* [optional][in] */ VARIANT FieldList,
            /* [optional][in] */ VARIANT Values) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CancelUpdate( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [defaultvalue][in] */ AffectEnum AffectRecords = adAffectCurrent) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetRows( 
            /* [defaultvalue][in] */ long Rows,
            /* [optional][in] */ VARIANT Start,
            /* [optional][in] */ VARIANT Fields,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ ADO_LONGPTR NumRecords,
            /* [optional][in] */ VARIANT Start) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MoveNext( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MovePrevious( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MoveFirst( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE MoveLast( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ CursorTypeEnum CursorType = adOpenUnspecified,
            /* [defaultvalue][in] */ LockTypeEnum LockType = adLockUnspecified,
            /* [defaultvalue][in] */ LONG Options = adCmdUnspecified) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Requery( 
            /* [defaultvalue][in] */ LONG Options = adOptionUnspecified) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE _xResync( 
            /* [defaultvalue][in] */ AffectEnum AffectRecords = adAffectAll) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [optional][in] */ VARIANT Fields,
            /* [optional][in] */ VARIANT Values) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_AbsolutePage( 
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_AbsolutePage( 
            /* [in] */ PositionEnum_Param Page) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_EditMode( 
            /* [retval][out] */ __RPC__out EditModeEnum *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Filter( 
            /* [retval][out] */ __RPC__out VARIANT *Criteria) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Filter( 
            /* [in] */ VARIANT Criteria) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_PageCount( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_PageSize( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_PageSize( 
            /* [in] */ long PageSize) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Sort( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Criteria) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Sort( 
            /* [in] */ __RPC__in BSTR Criteria) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out LONG *plObjState) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE _xClone( 
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppvObject) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE UpdateBatch( 
            /* [defaultvalue][in] */ AffectEnum AffectRecords = adAffectAll) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CancelBatch( 
            /* [defaultvalue][in] */ AffectEnum AffectRecords = adAffectAll) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_CursorLocation( 
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_CursorLocation( 
            /* [in] */ CursorLocationEnum lCursorLoc) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE NextRecordset( 
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppiRs) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Supports( 
            /* [in] */ CursorOptionEnum CursorOptions,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_Collect( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_Collect( 
            /* [in] */ VARIANT Index,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_MarshalOptions( 
            /* [retval][out] */ __RPC__out MarshalOptionsEnum *peMarshal) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_MarshalOptions( 
            /* [in] */ MarshalOptionsEnum eMarshal) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Find( 
            /* [in] */ __RPC__in BSTR Criteria,
            /* [defaultvalue][in] */ ADO_LONGPTR SkipRecords,
            /* [defaultvalue][in] */ SearchDirectionEnum SearchDirection,
            /* [optional][in] */ VARIANT Start) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Recordset15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Recordset15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Recordset15 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Recordset15 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Recordset15 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Recordset15 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePosition )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePosition )( 
            __RPC__in Recordset15 * This,
            /* [in] */ PositionEnum_Param Position);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in Recordset15 * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BOF )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bookmark )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBookmark);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bookmark )( 
            __RPC__in Recordset15 * This,
            /* [in] */ VARIANT vBookmark);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CacheSize )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CacheSize )( 
            __RPC__in Recordset15 * This,
            /* [in] */ long CacheSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorType )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out CursorTypeEnum *plCursorType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorType )( 
            __RPC__in Recordset15 * This,
            /* [in] */ CursorTypeEnum lCursorType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EOF )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOFields **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LockType )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out LockTypeEnum *plLockType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LockType )( 
            __RPC__in Recordset15 * This,
            /* [in] */ LockTypeEnum lLockType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxRecords )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plMaxRecords);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxRecords )( 
            __RPC__in Recordset15 * This,
            /* [in] */ ADO_LONGPTR lMaxRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecordCount )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in_opt IDispatch *pcmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvSource);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AddNew )( 
            __RPC__in Recordset15 * This,
            /* [optional][in] */ VARIANT FieldList,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelUpdate )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetRows )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ long Rows,
            /* [optional][in] */ VARIANT Start,
            /* [optional][in] */ VARIANT Fields,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in Recordset15 * This,
            /* [in] */ ADO_LONGPTR NumRecords,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveLast )( 
            __RPC__in Recordset15 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in Recordset15 * This,
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ CursorTypeEnum CursorType,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Requery )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xResync )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in Recordset15 * This,
            /* [optional][in] */ VARIANT Fields,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePage )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePage )( 
            __RPC__in Recordset15 * This,
            /* [in] */ PositionEnum_Param Page);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EditMode )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out EditModeEnum *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out VARIANT *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in Recordset15 * This,
            /* [in] */ VARIANT Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageSize )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PageSize )( 
            __RPC__in Recordset15 * This,
            /* [in] */ long PageSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Sort )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Sort )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in BSTR Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xClone )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateBatch )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelBatch )( 
            __RPC__in Recordset15 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in Recordset15 * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *NextADORecordset )( 
            __RPC__in Recordset15 * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Supports )( 
            __RPC__in Recordset15 * This,
            /* [in] */ CursorOptionEnum CursorOptions,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Collect )( 
            __RPC__in Recordset15 * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Collect )( 
            __RPC__in Recordset15 * This,
            /* [in] */ VARIANT Index,
            /* [in] */ VARIANT value);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MarshalOptions )( 
            __RPC__in Recordset15 * This,
            /* [retval][out] */ __RPC__out MarshalOptionsEnum *peMarshal);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MarshalOptions )( 
            __RPC__in Recordset15 * This,
            /* [in] */ MarshalOptionsEnum eMarshal);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in Recordset15 * This,
            /* [in] */ __RPC__in BSTR Criteria,
            /* [defaultvalue][in] */ ADO_LONGPTR SkipRecords,
            /* [defaultvalue][in] */ SearchDirectionEnum SearchDirection,
            /* [optional][in] */ VARIANT Start);
        
        END_INTERFACE
    } Recordset15Vtbl;
    interface Recordset15
    {
        CONST_VTBL struct Recordset15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Recordset15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Recordset15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Recordset15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Recordset15_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Recordset15_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Recordset15_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Recordset15_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Recordset15_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Recordset15_get_AbsolutePosition(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePosition(This,pl) ) 
#define Recordset15_put_AbsolutePosition(This,Position)	\
    ( (This)->lpVtbl -> put_AbsolutePosition(This,Position) ) 
#define Recordset15_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 
#define Recordset15_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define Recordset15_get_ActiveConnection(This,pvar)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pvar) ) 
#define Recordset15_get_BOF(This,pb)	\
    ( (This)->lpVtbl -> get_BOF(This,pb) ) 
#define Recordset15_get_Bookmark(This,pvBookmark)	\
    ( (This)->lpVtbl -> get_Bookmark(This,pvBookmark) ) 
#define Recordset15_put_Bookmark(This,vBookmark)	\
    ( (This)->lpVtbl -> put_Bookmark(This,vBookmark) ) 
#define Recordset15_get_CacheSize(This,pl)	\
    ( (This)->lpVtbl -> get_CacheSize(This,pl) ) 
#define Recordset15_put_CacheSize(This,CacheSize)	\
    ( (This)->lpVtbl -> put_CacheSize(This,CacheSize) ) 
#define Recordset15_get_CursorType(This,plCursorType)	\
    ( (This)->lpVtbl -> get_CursorType(This,plCursorType) ) 
#define Recordset15_put_CursorType(This,lCursorType)	\
    ( (This)->lpVtbl -> put_CursorType(This,lCursorType) ) 
#define Recordset15_get_EOF(This,pb)	\
    ( (This)->lpVtbl -> get_EOF(This,pb) ) 
#define Recordset15_get_Fields(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Fields(This,ppvObject) ) 
#define Recordset15_get_LockType(This,plLockType)	\
    ( (This)->lpVtbl -> get_LockType(This,plLockType) ) 
#define Recordset15_put_LockType(This,lLockType)	\
    ( (This)->lpVtbl -> put_LockType(This,lLockType) ) 
#define Recordset15_get_MaxRecords(This,plMaxRecords)	\
    ( (This)->lpVtbl -> get_MaxRecords(This,plMaxRecords) ) 
#define Recordset15_put_MaxRecords(This,lMaxRecords)	\
    ( (This)->lpVtbl -> put_MaxRecords(This,lMaxRecords) ) 
#define Recordset15_get_RecordCount(This,pl)	\
    ( (This)->lpVtbl -> get_RecordCount(This,pl) ) 
#define Recordset15_putref_Source(This,pcmd)	\
    ( (This)->lpVtbl -> putref_Source(This,pcmd) ) 
#define Recordset15_put_Source(This,bstrConn)	\
    ( (This)->lpVtbl -> put_Source(This,bstrConn) ) 
#define Recordset15_get_Source(This,pvSource)	\
    ( (This)->lpVtbl -> get_Source(This,pvSource) ) 
#define Recordset15_AddNew(This,FieldList,Values)	\
    ( (This)->lpVtbl -> AddNew(This,FieldList,Values) ) 
#define Recordset15_CancelUpdate(This)	\
    ( (This)->lpVtbl -> CancelUpdate(This) ) 
#define Recordset15_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define Recordset15_Delete(This,AffectRecords)	\
    ( (This)->lpVtbl -> Delete(This,AffectRecords) ) 
#define Recordset15_GetRows(This,Rows,Start,Fields,pvar)	\
    ( (This)->lpVtbl -> GetRows(This,Rows,Start,Fields,pvar) ) 
#define Recordset15_Move(This,NumRecords,Start)	\
    ( (This)->lpVtbl -> Move(This,NumRecords,Start) ) 
#define Recordset15_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 
#define Recordset15_MovePrevious(This)	\
    ( (This)->lpVtbl -> MovePrevious(This) ) 
#define Recordset15_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 
#define Recordset15_MoveLast(This)	\
    ( (This)->lpVtbl -> MoveLast(This) ) 
#define Recordset15_Open(This,Source,ActiveConnection,CursorType,LockType,Options)	\
    ( (This)->lpVtbl -> Open(This,Source,ActiveConnection,CursorType,LockType,Options) ) 
#define Recordset15_Requery(This,Options)	\
    ( (This)->lpVtbl -> Requery(This,Options) ) 
#define Recordset15__xResync(This,AffectRecords)	\
    ( (This)->lpVtbl -> _xResync(This,AffectRecords) ) 
#define Recordset15_Update(This,Fields,Values)	\
    ( (This)->lpVtbl -> Update(This,Fields,Values) ) 
#define Recordset15_get_AbsolutePage(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePage(This,pl) ) 
#define Recordset15_put_AbsolutePage(This,Page)	\
    ( (This)->lpVtbl -> put_AbsolutePage(This,Page) ) 
#define Recordset15_get_EditMode(This,pl)	\
    ( (This)->lpVtbl -> get_EditMode(This,pl) ) 
#define Recordset15_get_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> get_Filter(This,Criteria) ) 
#define Recordset15_put_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> put_Filter(This,Criteria) ) 
#define Recordset15_get_PageCount(This,pl)	\
    ( (This)->lpVtbl -> get_PageCount(This,pl) ) 
#define Recordset15_get_PageSize(This,pl)	\
    ( (This)->lpVtbl -> get_PageSize(This,pl) ) 
#define Recordset15_put_PageSize(This,PageSize)	\
    ( (This)->lpVtbl -> put_PageSize(This,PageSize) ) 
#define Recordset15_get_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> get_Sort(This,Criteria) ) 
#define Recordset15_put_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> put_Sort(This,Criteria) ) 
#define Recordset15_get_Status(This,pl)	\
    ( (This)->lpVtbl -> get_Status(This,pl) ) 
#define Recordset15_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define Recordset15__xClone(This,ppvObject)	\
    ( (This)->lpVtbl -> _xClone(This,ppvObject) ) 
#define Recordset15_UpdateBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> UpdateBatch(This,AffectRecords) ) 
#define Recordset15_CancelBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> CancelBatch(This,AffectRecords) ) 
#define Recordset15_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define Recordset15_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define Recordset15_NextRecordset(This,RecordsAffected,ppiRs)	\
    ( (This)->lpVtbl -> NextRecordset(This,RecordsAffected,ppiRs) ) 
#define Recordset15_Supports(This,CursorOptions,pb)	\
    ( (This)->lpVtbl -> Supports(This,CursorOptions,pb) ) 
#define Recordset15_get_Collect(This,Index,pvar)	\
    ( (This)->lpVtbl -> get_Collect(This,Index,pvar) ) 
#define Recordset15_put_Collect(This,Index,value)	\
    ( (This)->lpVtbl -> put_Collect(This,Index,value) ) 
#define Recordset15_get_MarshalOptions(This,peMarshal)	\
    ( (This)->lpVtbl -> get_MarshalOptions(This,peMarshal) ) 
#define Recordset15_put_MarshalOptions(This,eMarshal)	\
    ( (This)->lpVtbl -> put_MarshalOptions(This,eMarshal) ) 
#define Recordset15_Find(This,Criteria,SkipRecords,SearchDirection,Start)	\
    ( (This)->lpVtbl -> Find(This,Criteria,SkipRecords,SearchDirection,Start) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Recordset15_INTERFACE_DEFINED__ */
#ifndef __Recordset20_INTERFACE_DEFINED__
#define __Recordset20_INTERFACE_DEFINED__
/* interface Recordset20 */
/* [object][helpcontext][uuid][nonextensible][hidden][dual] */ 
EXTERN_C const IID IID_Recordset20;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000054F-0000-0010-8000-00AA006D2EA4")
    Recordset20 : public Recordset15
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_DataSource( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkDataSource) = 0;
        
        virtual /* [helpcontext][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_DataSource( 
            /* [in] */ __RPC__in_opt IUnknown *punkDataSource) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE _xSave( 
            /* [defaultvalue][in] */ __RPC__in BSTR FileName = NULL,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat = adPersistADTG) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveCommand( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCmd) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_StayInSync( 
            /* [in] */ VARIANT_BOOL bStayInSync) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_StayInSync( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbStayInSync) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetString( 
            /* [defaultvalue][in] */ StringFormatEnum StringFormat,
            /* [defaultvalue][in] */ long NumRows,
            /* [defaultvalue][in] */ __RPC__in BSTR ColumnDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR RowDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR NullExpr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetString) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_DataMember( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDataMember) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_DataMember( 
            /* [in] */ __RPC__in BSTR bstrDataMember) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CompareBookmarks( 
            /* [in] */ VARIANT Bookmark1,
            /* [in] */ VARIANT Bookmark2,
            /* [retval][out] */ __RPC__out CompareEnum *pCompare) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [retval][out] */ __RPC__deref_out_opt _ADORecordset **ppvObject) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Resync( 
            /* [defaultvalue][in] */ AffectEnum AffectRecords = adAffectAll,
            /* [defaultvalue][in] */ ResyncEnum ResyncValues = adResyncAllValues) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Recordset20Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Recordset20 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Recordset20 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Recordset20 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Recordset20 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Recordset20 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePosition )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePosition )( 
            __RPC__in Recordset20 * This,
            /* [in] */ PositionEnum_Param Position);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BOF )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bookmark )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBookmark);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bookmark )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT vBookmark);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CacheSize )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CacheSize )( 
            __RPC__in Recordset20 * This,
            /* [in] */ long CacheSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorType )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out CursorTypeEnum *plCursorType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorType )( 
            __RPC__in Recordset20 * This,
            /* [in] */ CursorTypeEnum lCursorType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EOF )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOFields **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LockType )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out LockTypeEnum *plLockType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LockType )( 
            __RPC__in Recordset20 * This,
            /* [in] */ LockTypeEnum lLockType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxRecords )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plMaxRecords);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxRecords )( 
            __RPC__in Recordset20 * This,
            /* [in] */ ADO_LONGPTR lMaxRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecordCount )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in_opt IDispatch *pcmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvSource);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AddNew )( 
            __RPC__in Recordset20 * This,
            /* [optional][in] */ VARIANT FieldList,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelUpdate )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetRows )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ long Rows,
            /* [optional][in] */ VARIANT Start,
            /* [optional][in] */ VARIANT Fields,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in Recordset20 * This,
            /* [in] */ ADO_LONGPTR NumRecords,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveLast )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in Recordset20 * This,
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ CursorTypeEnum CursorType,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Requery )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xResync )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in Recordset20 * This,
            /* [optional][in] */ VARIANT Fields,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePage )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePage )( 
            __RPC__in Recordset20 * This,
            /* [in] */ PositionEnum_Param Page);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EditMode )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out EditModeEnum *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageSize )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PageSize )( 
            __RPC__in Recordset20 * This,
            /* [in] */ long PageSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Sort )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Sort )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in BSTR Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xClone )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateBatch )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelBatch )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in Recordset20 * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *NextADORecordset )( 
            __RPC__in Recordset20 * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Supports )( 
            __RPC__in Recordset20 * This,
            /* [in] */ CursorOptionEnum CursorOptions,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Collect )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Collect )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT Index,
            /* [in] */ VARIANT value);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MarshalOptions )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out MarshalOptionsEnum *peMarshal);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MarshalOptions )( 
            __RPC__in Recordset20 * This,
            /* [in] */ MarshalOptionsEnum eMarshal);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in BSTR Criteria,
            /* [defaultvalue][in] */ ADO_LONGPTR SkipRecords,
            /* [defaultvalue][in] */ SearchDirectionEnum SearchDirection,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in Recordset20 * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSource )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkDataSource);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DataSource )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in_opt IUnknown *punkDataSource);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xSave )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR FileName,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveCommand )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StayInSync )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT_BOOL bStayInSync);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StayInSync )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbStayInSync);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ StringFormatEnum StringFormat,
            /* [defaultvalue][in] */ long NumRows,
            /* [defaultvalue][in] */ __RPC__in BSTR ColumnDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR RowDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR NullExpr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetString);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataMember )( 
            __RPC__in Recordset20 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDataMember);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataMember )( 
            __RPC__in Recordset20 * This,
            /* [in] */ __RPC__in BSTR bstrDataMember);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CompareBookmarks )( 
            __RPC__in Recordset20 * This,
            /* [in] */ VARIANT Bookmark1,
            /* [in] */ VARIANT Bookmark2,
            /* [retval][out] */ __RPC__out CompareEnum *pCompare);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Resync )( 
            __RPC__in Recordset20 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords,
            /* [defaultvalue][in] */ ResyncEnum ResyncValues);
        
        END_INTERFACE
    } Recordset20Vtbl;
    interface Recordset20
    {
        CONST_VTBL struct Recordset20Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Recordset20_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Recordset20_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Recordset20_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Recordset20_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Recordset20_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Recordset20_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Recordset20_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Recordset20_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Recordset20_get_AbsolutePosition(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePosition(This,pl) ) 
#define Recordset20_put_AbsolutePosition(This,Position)	\
    ( (This)->lpVtbl -> put_AbsolutePosition(This,Position) ) 
#define Recordset20_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 
#define Recordset20_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define Recordset20_get_ActiveConnection(This,pvar)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pvar) ) 
#define Recordset20_get_BOF(This,pb)	\
    ( (This)->lpVtbl -> get_BOF(This,pb) ) 
#define Recordset20_get_Bookmark(This,pvBookmark)	\
    ( (This)->lpVtbl -> get_Bookmark(This,pvBookmark) ) 
#define Recordset20_put_Bookmark(This,vBookmark)	\
    ( (This)->lpVtbl -> put_Bookmark(This,vBookmark) ) 
#define Recordset20_get_CacheSize(This,pl)	\
    ( (This)->lpVtbl -> get_CacheSize(This,pl) ) 
#define Recordset20_put_CacheSize(This,CacheSize)	\
    ( (This)->lpVtbl -> put_CacheSize(This,CacheSize) ) 
#define Recordset20_get_CursorType(This,plCursorType)	\
    ( (This)->lpVtbl -> get_CursorType(This,plCursorType) ) 
#define Recordset20_put_CursorType(This,lCursorType)	\
    ( (This)->lpVtbl -> put_CursorType(This,lCursorType) ) 
#define Recordset20_get_EOF(This,pb)	\
    ( (This)->lpVtbl -> get_EOF(This,pb) ) 
#define Recordset20_get_Fields(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Fields(This,ppvObject) ) 
#define Recordset20_get_LockType(This,plLockType)	\
    ( (This)->lpVtbl -> get_LockType(This,plLockType) ) 
#define Recordset20_put_LockType(This,lLockType)	\
    ( (This)->lpVtbl -> put_LockType(This,lLockType) ) 
#define Recordset20_get_MaxRecords(This,plMaxRecords)	\
    ( (This)->lpVtbl -> get_MaxRecords(This,plMaxRecords) ) 
#define Recordset20_put_MaxRecords(This,lMaxRecords)	\
    ( (This)->lpVtbl -> put_MaxRecords(This,lMaxRecords) ) 
#define Recordset20_get_RecordCount(This,pl)	\
    ( (This)->lpVtbl -> get_RecordCount(This,pl) ) 
#define Recordset20_putref_Source(This,pcmd)	\
    ( (This)->lpVtbl -> putref_Source(This,pcmd) ) 
#define Recordset20_put_Source(This,bstrConn)	\
    ( (This)->lpVtbl -> put_Source(This,bstrConn) ) 
#define Recordset20_get_Source(This,pvSource)	\
    ( (This)->lpVtbl -> get_Source(This,pvSource) ) 
#define Recordset20_AddNew(This,FieldList,Values)	\
    ( (This)->lpVtbl -> AddNew(This,FieldList,Values) ) 
#define Recordset20_CancelUpdate(This)	\
    ( (This)->lpVtbl -> CancelUpdate(This) ) 
#define Recordset20_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define Recordset20_Delete(This,AffectRecords)	\
    ( (This)->lpVtbl -> Delete(This,AffectRecords) ) 
#define Recordset20_GetRows(This,Rows,Start,Fields,pvar)	\
    ( (This)->lpVtbl -> GetRows(This,Rows,Start,Fields,pvar) ) 
#define Recordset20_Move(This,NumRecords,Start)	\
    ( (This)->lpVtbl -> Move(This,NumRecords,Start) ) 
#define Recordset20_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 
#define Recordset20_MovePrevious(This)	\
    ( (This)->lpVtbl -> MovePrevious(This) ) 
#define Recordset20_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 
#define Recordset20_MoveLast(This)	\
    ( (This)->lpVtbl -> MoveLast(This) ) 
#define Recordset20_Open(This,Source,ActiveConnection,CursorType,LockType,Options)	\
    ( (This)->lpVtbl -> Open(This,Source,ActiveConnection,CursorType,LockType,Options) ) 
#define Recordset20_Requery(This,Options)	\
    ( (This)->lpVtbl -> Requery(This,Options) ) 
#define Recordset20__xResync(This,AffectRecords)	\
    ( (This)->lpVtbl -> _xResync(This,AffectRecords) ) 
#define Recordset20_Update(This,Fields,Values)	\
    ( (This)->lpVtbl -> Update(This,Fields,Values) ) 
#define Recordset20_get_AbsolutePage(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePage(This,pl) ) 
#define Recordset20_put_AbsolutePage(This,Page)	\
    ( (This)->lpVtbl -> put_AbsolutePage(This,Page) ) 
#define Recordset20_get_EditMode(This,pl)	\
    ( (This)->lpVtbl -> get_EditMode(This,pl) ) 
#define Recordset20_get_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> get_Filter(This,Criteria) ) 
#define Recordset20_put_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> put_Filter(This,Criteria) ) 
#define Recordset20_get_PageCount(This,pl)	\
    ( (This)->lpVtbl -> get_PageCount(This,pl) ) 
#define Recordset20_get_PageSize(This,pl)	\
    ( (This)->lpVtbl -> get_PageSize(This,pl) ) 
#define Recordset20_put_PageSize(This,PageSize)	\
    ( (This)->lpVtbl -> put_PageSize(This,PageSize) ) 
#define Recordset20_get_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> get_Sort(This,Criteria) ) 
#define Recordset20_put_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> put_Sort(This,Criteria) ) 
#define Recordset20_get_Status(This,pl)	\
    ( (This)->lpVtbl -> get_Status(This,pl) ) 
#define Recordset20_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define Recordset20__xClone(This,ppvObject)	\
    ( (This)->lpVtbl -> _xClone(This,ppvObject) ) 
#define Recordset20_UpdateBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> UpdateBatch(This,AffectRecords) ) 
#define Recordset20_CancelBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> CancelBatch(This,AffectRecords) ) 
#define Recordset20_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define Recordset20_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define Recordset20_NextRecordset(This,RecordsAffected,ppiRs)	\
    ( (This)->lpVtbl -> NextRecordset(This,RecordsAffected,ppiRs) ) 
#define Recordset20_Supports(This,CursorOptions,pb)	\
    ( (This)->lpVtbl -> Supports(This,CursorOptions,pb) ) 
#define Recordset20_get_Collect(This,Index,pvar)	\
    ( (This)->lpVtbl -> get_Collect(This,Index,pvar) ) 
#define Recordset20_put_Collect(This,Index,value)	\
    ( (This)->lpVtbl -> put_Collect(This,Index,value) ) 
#define Recordset20_get_MarshalOptions(This,peMarshal)	\
    ( (This)->lpVtbl -> get_MarshalOptions(This,peMarshal) ) 
#define Recordset20_put_MarshalOptions(This,eMarshal)	\
    ( (This)->lpVtbl -> put_MarshalOptions(This,eMarshal) ) 
#define Recordset20_Find(This,Criteria,SkipRecords,SearchDirection,Start)	\
    ( (This)->lpVtbl -> Find(This,Criteria,SkipRecords,SearchDirection,Start) ) 
#define Recordset20_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#define Recordset20_get_DataSource(This,ppunkDataSource)	\
    ( (This)->lpVtbl -> get_DataSource(This,ppunkDataSource) ) 
#define Recordset20_putref_DataSource(This,punkDataSource)	\
    ( (This)->lpVtbl -> putref_DataSource(This,punkDataSource) ) 
#define Recordset20__xSave(This,FileName,PersistFormat)	\
    ( (This)->lpVtbl -> _xSave(This,FileName,PersistFormat) ) 
#define Recordset20_get_ActiveCommand(This,ppCmd)	\
    ( (This)->lpVtbl -> get_ActiveCommand(This,ppCmd) ) 
#define Recordset20_put_StayInSync(This,bStayInSync)	\
    ( (This)->lpVtbl -> put_StayInSync(This,bStayInSync) ) 
#define Recordset20_get_StayInSync(This,pbStayInSync)	\
    ( (This)->lpVtbl -> get_StayInSync(This,pbStayInSync) ) 
#define Recordset20_GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString)	\
    ( (This)->lpVtbl -> GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString) ) 
#define Recordset20_get_DataMember(This,pbstrDataMember)	\
    ( (This)->lpVtbl -> get_DataMember(This,pbstrDataMember) ) 
#define Recordset20_put_DataMember(This,bstrDataMember)	\
    ( (This)->lpVtbl -> put_DataMember(This,bstrDataMember) ) 
#define Recordset20_CompareBookmarks(This,Bookmark1,Bookmark2,pCompare)	\
    ( (This)->lpVtbl -> CompareBookmarks(This,Bookmark1,Bookmark2,pCompare) ) 
#define Recordset20_Clone(This,LockType,ppvObject)	\
    ( (This)->lpVtbl -> Clone(This,LockType,ppvObject) ) 
#define Recordset20_Resync(This,AffectRecords,ResyncValues)	\
    ( (This)->lpVtbl -> Resync(This,AffectRecords,ResyncValues) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Recordset20_INTERFACE_DEFINED__ */
#ifndef __Recordset21_INTERFACE_DEFINED__
#define __Recordset21_INTERFACE_DEFINED__
/* interface Recordset21 */
/* [object][helpcontext][uuid][nonextensible][hidden][dual] */ 
EXTERN_C const IID IID_Recordset21;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000555-0000-0010-8000-00AA006D2EA4")
    Recordset21 : public Recordset20
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ VARIANT KeyValues,
            /* [defaultvalue][in] */ SeekEnum SeekOption = adSeekFirstEQ) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Index( 
            /* [in] */ __RPC__in BSTR Index) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Index( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrIndex) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Recordset21Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Recordset21 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Recordset21 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Recordset21 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Recordset21 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Recordset21 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePosition )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePosition )( 
            __RPC__in Recordset21 * This,
            /* [in] */ PositionEnum_Param Position);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BOF )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bookmark )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBookmark);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bookmark )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT vBookmark);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CacheSize )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CacheSize )( 
            __RPC__in Recordset21 * This,
            /* [in] */ long CacheSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorType )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out CursorTypeEnum *plCursorType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorType )( 
            __RPC__in Recordset21 * This,
            /* [in] */ CursorTypeEnum lCursorType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EOF )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOFields **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LockType )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out LockTypeEnum *plLockType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LockType )( 
            __RPC__in Recordset21 * This,
            /* [in] */ LockTypeEnum lLockType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxRecords )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plMaxRecords);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxRecords )( 
            __RPC__in Recordset21 * This,
            /* [in] */ ADO_LONGPTR lMaxRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecordCount )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in_opt IDispatch *pcmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvSource);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AddNew )( 
            __RPC__in Recordset21 * This,
            /* [optional][in] */ VARIANT FieldList,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelUpdate )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetRows )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ long Rows,
            /* [optional][in] */ VARIANT Start,
            /* [optional][in] */ VARIANT Fields,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in Recordset21 * This,
            /* [in] */ ADO_LONGPTR NumRecords,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveLast )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in Recordset21 * This,
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ CursorTypeEnum CursorType,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Requery )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xResync )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in Recordset21 * This,
            /* [optional][in] */ VARIANT Fields,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePage )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePage )( 
            __RPC__in Recordset21 * This,
            /* [in] */ PositionEnum_Param Page);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EditMode )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out EditModeEnum *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageSize )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PageSize )( 
            __RPC__in Recordset21 * This,
            /* [in] */ long PageSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Sort )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Sort )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in BSTR Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xClone )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateBatch )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelBatch )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in Recordset21 * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *NextADORecordset )( 
            __RPC__in Recordset21 * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Supports )( 
            __RPC__in Recordset21 * This,
            /* [in] */ CursorOptionEnum CursorOptions,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Collect )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Collect )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT Index,
            /* [in] */ VARIANT value);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MarshalOptions )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out MarshalOptionsEnum *peMarshal);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MarshalOptions )( 
            __RPC__in Recordset21 * This,
            /* [in] */ MarshalOptionsEnum eMarshal);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in BSTR Criteria,
            /* [defaultvalue][in] */ ADO_LONGPTR SkipRecords,
            /* [defaultvalue][in] */ SearchDirectionEnum SearchDirection,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in Recordset21 * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSource )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkDataSource);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DataSource )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in_opt IUnknown *punkDataSource);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xSave )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR FileName,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveCommand )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StayInSync )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT_BOOL bStayInSync);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StayInSync )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbStayInSync);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ StringFormatEnum StringFormat,
            /* [defaultvalue][in] */ long NumRows,
            /* [defaultvalue][in] */ __RPC__in BSTR ColumnDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR RowDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR NullExpr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetString);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataMember )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDataMember);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataMember )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in BSTR bstrDataMember);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CompareBookmarks )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT Bookmark1,
            /* [in] */ VARIANT Bookmark2,
            /* [retval][out] */ __RPC__out CompareEnum *pCompare);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Resync )( 
            __RPC__in Recordset21 * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords,
            /* [defaultvalue][in] */ ResyncEnum ResyncValues);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in Recordset21 * This,
            /* [in] */ VARIANT KeyValues,
            /* [defaultvalue][in] */ SeekEnum SeekOption);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in Recordset21 * This,
            /* [in] */ __RPC__in BSTR Index);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in Recordset21 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrIndex);
        
        END_INTERFACE
    } Recordset21Vtbl;
    interface Recordset21
    {
        CONST_VTBL struct Recordset21Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Recordset21_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Recordset21_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Recordset21_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Recordset21_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Recordset21_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Recordset21_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Recordset21_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Recordset21_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Recordset21_get_AbsolutePosition(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePosition(This,pl) ) 
#define Recordset21_put_AbsolutePosition(This,Position)	\
    ( (This)->lpVtbl -> put_AbsolutePosition(This,Position) ) 
#define Recordset21_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 
#define Recordset21_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define Recordset21_get_ActiveConnection(This,pvar)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pvar) ) 
#define Recordset21_get_BOF(This,pb)	\
    ( (This)->lpVtbl -> get_BOF(This,pb) ) 
#define Recordset21_get_Bookmark(This,pvBookmark)	\
    ( (This)->lpVtbl -> get_Bookmark(This,pvBookmark) ) 
#define Recordset21_put_Bookmark(This,vBookmark)	\
    ( (This)->lpVtbl -> put_Bookmark(This,vBookmark) ) 
#define Recordset21_get_CacheSize(This,pl)	\
    ( (This)->lpVtbl -> get_CacheSize(This,pl) ) 
#define Recordset21_put_CacheSize(This,CacheSize)	\
    ( (This)->lpVtbl -> put_CacheSize(This,CacheSize) ) 
#define Recordset21_get_CursorType(This,plCursorType)	\
    ( (This)->lpVtbl -> get_CursorType(This,plCursorType) ) 
#define Recordset21_put_CursorType(This,lCursorType)	\
    ( (This)->lpVtbl -> put_CursorType(This,lCursorType) ) 
#define Recordset21_get_EOF(This,pb)	\
    ( (This)->lpVtbl -> get_EOF(This,pb) ) 
#define Recordset21_get_Fields(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Fields(This,ppvObject) ) 
#define Recordset21_get_LockType(This,plLockType)	\
    ( (This)->lpVtbl -> get_LockType(This,plLockType) ) 
#define Recordset21_put_LockType(This,lLockType)	\
    ( (This)->lpVtbl -> put_LockType(This,lLockType) ) 
#define Recordset21_get_MaxRecords(This,plMaxRecords)	\
    ( (This)->lpVtbl -> get_MaxRecords(This,plMaxRecords) ) 
#define Recordset21_put_MaxRecords(This,lMaxRecords)	\
    ( (This)->lpVtbl -> put_MaxRecords(This,lMaxRecords) ) 
#define Recordset21_get_RecordCount(This,pl)	\
    ( (This)->lpVtbl -> get_RecordCount(This,pl) ) 
#define Recordset21_putref_Source(This,pcmd)	\
    ( (This)->lpVtbl -> putref_Source(This,pcmd) ) 
#define Recordset21_put_Source(This,bstrConn)	\
    ( (This)->lpVtbl -> put_Source(This,bstrConn) ) 
#define Recordset21_get_Source(This,pvSource)	\
    ( (This)->lpVtbl -> get_Source(This,pvSource) ) 
#define Recordset21_AddNew(This,FieldList,Values)	\
    ( (This)->lpVtbl -> AddNew(This,FieldList,Values) ) 
#define Recordset21_CancelUpdate(This)	\
    ( (This)->lpVtbl -> CancelUpdate(This) ) 
#define Recordset21_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define Recordset21_Delete(This,AffectRecords)	\
    ( (This)->lpVtbl -> Delete(This,AffectRecords) ) 
#define Recordset21_GetRows(This,Rows,Start,Fields,pvar)	\
    ( (This)->lpVtbl -> GetRows(This,Rows,Start,Fields,pvar) ) 
#define Recordset21_Move(This,NumRecords,Start)	\
    ( (This)->lpVtbl -> Move(This,NumRecords,Start) ) 
#define Recordset21_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 
#define Recordset21_MovePrevious(This)	\
    ( (This)->lpVtbl -> MovePrevious(This) ) 
#define Recordset21_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 
#define Recordset21_MoveLast(This)	\
    ( (This)->lpVtbl -> MoveLast(This) ) 
#define Recordset21_Open(This,Source,ActiveConnection,CursorType,LockType,Options)	\
    ( (This)->lpVtbl -> Open(This,Source,ActiveConnection,CursorType,LockType,Options) ) 
#define Recordset21_Requery(This,Options)	\
    ( (This)->lpVtbl -> Requery(This,Options) ) 
#define Recordset21__xResync(This,AffectRecords)	\
    ( (This)->lpVtbl -> _xResync(This,AffectRecords) ) 
#define Recordset21_Update(This,Fields,Values)	\
    ( (This)->lpVtbl -> Update(This,Fields,Values) ) 
#define Recordset21_get_AbsolutePage(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePage(This,pl) ) 
#define Recordset21_put_AbsolutePage(This,Page)	\
    ( (This)->lpVtbl -> put_AbsolutePage(This,Page) ) 
#define Recordset21_get_EditMode(This,pl)	\
    ( (This)->lpVtbl -> get_EditMode(This,pl) ) 
#define Recordset21_get_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> get_Filter(This,Criteria) ) 
#define Recordset21_put_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> put_Filter(This,Criteria) ) 
#define Recordset21_get_PageCount(This,pl)	\
    ( (This)->lpVtbl -> get_PageCount(This,pl) ) 
#define Recordset21_get_PageSize(This,pl)	\
    ( (This)->lpVtbl -> get_PageSize(This,pl) ) 
#define Recordset21_put_PageSize(This,PageSize)	\
    ( (This)->lpVtbl -> put_PageSize(This,PageSize) ) 
#define Recordset21_get_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> get_Sort(This,Criteria) ) 
#define Recordset21_put_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> put_Sort(This,Criteria) ) 
#define Recordset21_get_Status(This,pl)	\
    ( (This)->lpVtbl -> get_Status(This,pl) ) 
#define Recordset21_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define Recordset21__xClone(This,ppvObject)	\
    ( (This)->lpVtbl -> _xClone(This,ppvObject) ) 
#define Recordset21_UpdateBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> UpdateBatch(This,AffectRecords) ) 
#define Recordset21_CancelBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> CancelBatch(This,AffectRecords) ) 
#define Recordset21_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define Recordset21_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define Recordset21_NextRecordset(This,RecordsAffected,ppiRs)	\
    ( (This)->lpVtbl -> NextRecordset(This,RecordsAffected,ppiRs) ) 
#define Recordset21_Supports(This,CursorOptions,pb)	\
    ( (This)->lpVtbl -> Supports(This,CursorOptions,pb) ) 
#define Recordset21_get_Collect(This,Index,pvar)	\
    ( (This)->lpVtbl -> get_Collect(This,Index,pvar) ) 
#define Recordset21_put_Collect(This,Index,value)	\
    ( (This)->lpVtbl -> put_Collect(This,Index,value) ) 
#define Recordset21_get_MarshalOptions(This,peMarshal)	\
    ( (This)->lpVtbl -> get_MarshalOptions(This,peMarshal) ) 
#define Recordset21_put_MarshalOptions(This,eMarshal)	\
    ( (This)->lpVtbl -> put_MarshalOptions(This,eMarshal) ) 
#define Recordset21_Find(This,Criteria,SkipRecords,SearchDirection,Start)	\
    ( (This)->lpVtbl -> Find(This,Criteria,SkipRecords,SearchDirection,Start) ) 
#define Recordset21_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#define Recordset21_get_DataSource(This,ppunkDataSource)	\
    ( (This)->lpVtbl -> get_DataSource(This,ppunkDataSource) ) 
#define Recordset21_putref_DataSource(This,punkDataSource)	\
    ( (This)->lpVtbl -> putref_DataSource(This,punkDataSource) ) 
#define Recordset21__xSave(This,FileName,PersistFormat)	\
    ( (This)->lpVtbl -> _xSave(This,FileName,PersistFormat) ) 
#define Recordset21_get_ActiveCommand(This,ppCmd)	\
    ( (This)->lpVtbl -> get_ActiveCommand(This,ppCmd) ) 
#define Recordset21_put_StayInSync(This,bStayInSync)	\
    ( (This)->lpVtbl -> put_StayInSync(This,bStayInSync) ) 
#define Recordset21_get_StayInSync(This,pbStayInSync)	\
    ( (This)->lpVtbl -> get_StayInSync(This,pbStayInSync) ) 
#define Recordset21_GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString)	\
    ( (This)->lpVtbl -> GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString) ) 
#define Recordset21_get_DataMember(This,pbstrDataMember)	\
    ( (This)->lpVtbl -> get_DataMember(This,pbstrDataMember) ) 
#define Recordset21_put_DataMember(This,bstrDataMember)	\
    ( (This)->lpVtbl -> put_DataMember(This,bstrDataMember) ) 
#define Recordset21_CompareBookmarks(This,Bookmark1,Bookmark2,pCompare)	\
    ( (This)->lpVtbl -> CompareBookmarks(This,Bookmark1,Bookmark2,pCompare) ) 
#define Recordset21_Clone(This,LockType,ppvObject)	\
    ( (This)->lpVtbl -> Clone(This,LockType,ppvObject) ) 
#define Recordset21_Resync(This,AffectRecords,ResyncValues)	\
    ( (This)->lpVtbl -> Resync(This,AffectRecords,ResyncValues) ) 
#define Recordset21_Seek(This,KeyValues,SeekOption)	\
    ( (This)->lpVtbl -> Seek(This,KeyValues,SeekOption) ) 
#define Recordset21_put_Index(This,Index)	\
    ( (This)->lpVtbl -> put_Index(This,Index) ) 
#define Recordset21_get_Index(This,pbstrIndex)	\
    ( (This)->lpVtbl -> get_Index(This,pbstrIndex) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Recordset21_INTERFACE_DEFINED__ */
#ifndef ___Recordset_INTERFACE_DEFINED__
#define ___Recordset_INTERFACE_DEFINED__
/* interface _ADORecordset */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__Recordset;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000556-0000-0010-8000-00AA006D2EA4")
    _ADORecordset : public Recordset21
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [optional][in] */ VARIANT Destination,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat = adPersistADTG) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _RecordsetVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADORecordset * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADORecordset * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADORecordset * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADORecordset * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePosition )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePosition )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ PositionEnum_Param Position);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveADOConnection )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT vConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BOF )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Bookmark )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBookmark);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Bookmark )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT vBookmark);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CacheSize )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CacheSize )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ long CacheSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorType )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out CursorTypeEnum *plCursorType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorType )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ CursorTypeEnum lCursorType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EOF )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOFields **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LockType )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out LockTypeEnum *plLockType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LockType )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ LockTypeEnum lLockType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxRecords )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plMaxRecords);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxRecords )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ ADO_LONGPTR lMaxRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecordCount )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in_opt IDispatch *pcmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT *pvSource);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AddNew )( 
            __RPC__in struct _ADORecordset * This,
            /* [optional][in] */ VARIANT FieldList,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelUpdate )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetRows )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ long Rows,
            /* [optional][in] */ VARIANT Start,
            /* [optional][in] */ VARIANT Fields,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ ADO_LONGPTR NumRecords,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *MoveLast )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in struct _ADORecordset * This,
            /* [optional][in] */ VARIANT Source,
            /* [optional][in] */ VARIANT ActiveConnection,
            /* [defaultvalue][in] */ CursorTypeEnum CursorType,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Requery )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ LONG Options);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xResync )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in struct _ADORecordset * This,
            /* [optional][in] */ VARIANT Fields,
            /* [optional][in] */ VARIANT Values);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AbsolutePage )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out PositionEnum_Param *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AbsolutePage )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ PositionEnum_Param Page);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EditMode )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out EditModeEnum *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PageSize )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PageSize )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ long PageSize);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Sort )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Criteria);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Sort )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in BSTR Criteria);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out LONG *plObjState);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xClone )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateBatch )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelBatch )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CursorLocation )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out CursorLocationEnum *plCursorLoc);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CursorLocation )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ CursorLocationEnum lCursorLoc);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *NextADORecordset )( 
            __RPC__in struct _ADORecordset * This,
            /* [optional][out] */ __RPC__out VARIANT *RecordsAffected,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppiRs);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Supports )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ CursorOptionEnum CursorOptions,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Collect )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Collect )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT Index,
            /* [in] */ VARIANT value);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MarshalOptions )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out MarshalOptionsEnum *peMarshal);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MarshalOptions )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ MarshalOptionsEnum eMarshal);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in BSTR Criteria,
            /* [defaultvalue][in] */ ADO_LONGPTR SkipRecords,
            /* [defaultvalue][in] */ SearchDirectionEnum SearchDirection,
            /* [optional][in] */ VARIANT Start);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in struct _ADORecordset * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSource )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkDataSource);
        
        /* [helpcontext][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DataSource )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in_opt IUnknown *punkDataSource);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_xSave )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ __RPC__in BSTR FileName,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveCommand )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCmd);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StayInSync )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT_BOOL bStayInSync);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StayInSync )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbStayInSync);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ StringFormatEnum StringFormat,
            /* [defaultvalue][in] */ long NumRows,
            /* [defaultvalue][in] */ __RPC__in BSTR ColumnDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR RowDelimeter,
            /* [defaultvalue][in] */ __RPC__in BSTR NullExpr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetString);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataMember )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDataMember);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataMember )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in BSTR bstrDataMember);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CompareBookmarks )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT Bookmark1,
            /* [in] */ VARIANT Bookmark2,
            /* [retval][out] */ __RPC__out CompareEnum *pCompare);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ LockTypeEnum LockType,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADORecordset **ppvObject);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Resync )( 
            __RPC__in struct _ADORecordset * This,
            /* [defaultvalue][in] */ AffectEnum AffectRecords,
            /* [defaultvalue][in] */ ResyncEnum ResyncValues);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ VARIANT KeyValues,
            /* [defaultvalue][in] */ SeekEnum SeekOption);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in struct _ADORecordset * This,
            /* [in] */ __RPC__in BSTR Index);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in struct _ADORecordset * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrIndex);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in struct _ADORecordset * This,
            /* [optional][in] */ VARIANT Destination,
            /* [defaultvalue][in] */ PersistFormatEnum PersistFormat);
        
        END_INTERFACE
    } _RecordsetVtbl;
    interface _Recordset
    {
        CONST_VTBL struct _RecordsetVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Recordset_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Recordset_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Recordset_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Recordset_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Recordset_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Recordset_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Recordset_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Recordset_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Recordset_get_AbsolutePosition(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePosition(This,pl) ) 
#define _Recordset_put_AbsolutePosition(This,Position)	\
    ( (This)->lpVtbl -> put_AbsolutePosition(This,Position) ) 
#define _Recordset_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 
#define _Recordset_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 
#define _Recordset_get_ActiveConnection(This,pvar)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pvar) ) 
#define _Recordset_get_BOF(This,pb)	\
    ( (This)->lpVtbl -> get_BOF(This,pb) ) 
#define _Recordset_get_Bookmark(This,pvBookmark)	\
    ( (This)->lpVtbl -> get_Bookmark(This,pvBookmark) ) 
#define _Recordset_put_Bookmark(This,vBookmark)	\
    ( (This)->lpVtbl -> put_Bookmark(This,vBookmark) ) 
#define _Recordset_get_CacheSize(This,pl)	\
    ( (This)->lpVtbl -> get_CacheSize(This,pl) ) 
#define _Recordset_put_CacheSize(This,CacheSize)	\
    ( (This)->lpVtbl -> put_CacheSize(This,CacheSize) ) 
#define _Recordset_get_CursorType(This,plCursorType)	\
    ( (This)->lpVtbl -> get_CursorType(This,plCursorType) ) 
#define _Recordset_put_CursorType(This,lCursorType)	\
    ( (This)->lpVtbl -> put_CursorType(This,lCursorType) ) 
#define _Recordset_get_EOF(This,pb)	\
    ( (This)->lpVtbl -> get_EOF(This,pb) ) 
#define _Recordset_get_Fields(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Fields(This,ppvObject) ) 
#define _Recordset_get_LockType(This,plLockType)	\
    ( (This)->lpVtbl -> get_LockType(This,plLockType) ) 
#define _Recordset_put_LockType(This,lLockType)	\
    ( (This)->lpVtbl -> put_LockType(This,lLockType) ) 
#define _Recordset_get_MaxRecords(This,plMaxRecords)	\
    ( (This)->lpVtbl -> get_MaxRecords(This,plMaxRecords) ) 
#define _Recordset_put_MaxRecords(This,lMaxRecords)	\
    ( (This)->lpVtbl -> put_MaxRecords(This,lMaxRecords) ) 
#define _Recordset_get_RecordCount(This,pl)	\
    ( (This)->lpVtbl -> get_RecordCount(This,pl) ) 
#define _Recordset_putref_Source(This,pcmd)	\
    ( (This)->lpVtbl -> putref_Source(This,pcmd) ) 
#define _Recordset_put_Source(This,bstrConn)	\
    ( (This)->lpVtbl -> put_Source(This,bstrConn) ) 
#define _Recordset_get_Source(This,pvSource)	\
    ( (This)->lpVtbl -> get_Source(This,pvSource) ) 
#define _Recordset_AddNew(This,FieldList,Values)	\
    ( (This)->lpVtbl -> AddNew(This,FieldList,Values) ) 
#define _Recordset_CancelUpdate(This)	\
    ( (This)->lpVtbl -> CancelUpdate(This) ) 
#define _Recordset_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 
#define _Recordset_Delete(This,AffectRecords)	\
    ( (This)->lpVtbl -> Delete(This,AffectRecords) ) 
#define _Recordset_GetRows(This,Rows,Start,Fields,pvar)	\
    ( (This)->lpVtbl -> GetRows(This,Rows,Start,Fields,pvar) ) 
#define _Recordset_Move(This,NumRecords,Start)	\
    ( (This)->lpVtbl -> Move(This,NumRecords,Start) ) 
#define _Recordset_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 
#define _Recordset_MovePrevious(This)	\
    ( (This)->lpVtbl -> MovePrevious(This) ) 
#define _Recordset_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 
#define _Recordset_MoveLast(This)	\
    ( (This)->lpVtbl -> MoveLast(This) ) 
#define _Recordset_Open(This,Source,ActiveConnection,CursorType,LockType,Options)	\
    ( (This)->lpVtbl -> Open(This,Source,ActiveConnection,CursorType,LockType,Options) ) 
#define _Recordset_Requery(This,Options)	\
    ( (This)->lpVtbl -> Requery(This,Options) ) 
#define _Recordset__xResync(This,AffectRecords)	\
    ( (This)->lpVtbl -> _xResync(This,AffectRecords) ) 
#define _Recordset_Update(This,Fields,Values)	\
    ( (This)->lpVtbl -> Update(This,Fields,Values) ) 
#define _Recordset_get_AbsolutePage(This,pl)	\
    ( (This)->lpVtbl -> get_AbsolutePage(This,pl) ) 
#define _Recordset_put_AbsolutePage(This,Page)	\
    ( (This)->lpVtbl -> put_AbsolutePage(This,Page) ) 
#define _Recordset_get_EditMode(This,pl)	\
    ( (This)->lpVtbl -> get_EditMode(This,pl) ) 
#define _Recordset_get_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> get_Filter(This,Criteria) ) 
#define _Recordset_put_Filter(This,Criteria)	\
    ( (This)->lpVtbl -> put_Filter(This,Criteria) ) 
#define _Recordset_get_PageCount(This,pl)	\
    ( (This)->lpVtbl -> get_PageCount(This,pl) ) 
#define _Recordset_get_PageSize(This,pl)	\
    ( (This)->lpVtbl -> get_PageSize(This,pl) ) 
#define _Recordset_put_PageSize(This,PageSize)	\
    ( (This)->lpVtbl -> put_PageSize(This,PageSize) ) 
#define _Recordset_get_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> get_Sort(This,Criteria) ) 
#define _Recordset_put_Sort(This,Criteria)	\
    ( (This)->lpVtbl -> put_Sort(This,Criteria) ) 
#define _Recordset_get_Status(This,pl)	\
    ( (This)->lpVtbl -> get_Status(This,pl) ) 
#define _Recordset_get_State(This,plObjState)	\
    ( (This)->lpVtbl -> get_State(This,plObjState) ) 
#define _Recordset__xClone(This,ppvObject)	\
    ( (This)->lpVtbl -> _xClone(This,ppvObject) ) 
#define _Recordset_UpdateBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> UpdateBatch(This,AffectRecords) ) 
#define _Recordset_CancelBatch(This,AffectRecords)	\
    ( (This)->lpVtbl -> CancelBatch(This,AffectRecords) ) 
#define _Recordset_get_CursorLocation(This,plCursorLoc)	\
    ( (This)->lpVtbl -> get_CursorLocation(This,plCursorLoc) ) 
#define _Recordset_put_CursorLocation(This,lCursorLoc)	\
    ( (This)->lpVtbl -> put_CursorLocation(This,lCursorLoc) ) 
#define _Recordset_NextRecordset(This,RecordsAffected,ppiRs)	\
    ( (This)->lpVtbl -> NextRecordset(This,RecordsAffected,ppiRs) ) 
#define _Recordset_Supports(This,CursorOptions,pb)	\
    ( (This)->lpVtbl -> Supports(This,CursorOptions,pb) ) 
#define _Recordset_get_Collect(This,Index,pvar)	\
    ( (This)->lpVtbl -> get_Collect(This,Index,pvar) ) 
#define _Recordset_put_Collect(This,Index,value)	\
    ( (This)->lpVtbl -> put_Collect(This,Index,value) ) 
#define _Recordset_get_MarshalOptions(This,peMarshal)	\
    ( (This)->lpVtbl -> get_MarshalOptions(This,peMarshal) ) 
#define _Recordset_put_MarshalOptions(This,eMarshal)	\
    ( (This)->lpVtbl -> put_MarshalOptions(This,eMarshal) ) 
#define _Recordset_Find(This,Criteria,SkipRecords,SearchDirection,Start)	\
    ( (This)->lpVtbl -> Find(This,Criteria,SkipRecords,SearchDirection,Start) ) 
#define _Recordset_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 
#define _Recordset_get_DataSource(This,ppunkDataSource)	\
    ( (This)->lpVtbl -> get_DataSource(This,ppunkDataSource) ) 
#define _Recordset_putref_DataSource(This,punkDataSource)	\
    ( (This)->lpVtbl -> putref_DataSource(This,punkDataSource) ) 
#define _Recordset__xSave(This,FileName,PersistFormat)	\
    ( (This)->lpVtbl -> _xSave(This,FileName,PersistFormat) ) 
#define _Recordset_get_ActiveCommand(This,ppCmd)	\
    ( (This)->lpVtbl -> get_ActiveCommand(This,ppCmd) ) 
#define _Recordset_put_StayInSync(This,bStayInSync)	\
    ( (This)->lpVtbl -> put_StayInSync(This,bStayInSync) ) 
#define _Recordset_get_StayInSync(This,pbStayInSync)	\
    ( (This)->lpVtbl -> get_StayInSync(This,pbStayInSync) ) 
#define _Recordset_GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString)	\
    ( (This)->lpVtbl -> GetString(This,StringFormat,NumRows,ColumnDelimeter,RowDelimeter,NullExpr,pRetString) ) 
#define _Recordset_get_DataMember(This,pbstrDataMember)	\
    ( (This)->lpVtbl -> get_DataMember(This,pbstrDataMember) ) 
#define _Recordset_put_DataMember(This,bstrDataMember)	\
    ( (This)->lpVtbl -> put_DataMember(This,bstrDataMember) ) 
#define _Recordset_CompareBookmarks(This,Bookmark1,Bookmark2,pCompare)	\
    ( (This)->lpVtbl -> CompareBookmarks(This,Bookmark1,Bookmark2,pCompare) ) 
#define _Recordset_Clone(This,LockType,ppvObject)	\
    ( (This)->lpVtbl -> Clone(This,LockType,ppvObject) ) 
#define _Recordset_Resync(This,AffectRecords,ResyncValues)	\
    ( (This)->lpVtbl -> Resync(This,AffectRecords,ResyncValues) ) 
#define _Recordset_Seek(This,KeyValues,SeekOption)	\
    ( (This)->lpVtbl -> Seek(This,KeyValues,SeekOption) ) 
#define _Recordset_put_Index(This,Index)	\
    ( (This)->lpVtbl -> put_Index(This,Index) ) 
#define _Recordset_get_Index(This,pbstrIndex)	\
    ( (This)->lpVtbl -> get_Index(This,pbstrIndex) ) 
#define _Recordset_Save(This,Destination,PersistFormat)	\
    ( (This)->lpVtbl -> Save(This,Destination,PersistFormat) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Recordset_INTERFACE_DEFINED__ */
#ifndef __ADORecordsetConstruction_INTERFACE_DEFINED__
#define __ADORecordsetConstruction_INTERFACE_DEFINED__
/* interface ADORecordsetConstruction */
/* [object][uuid][restricted] */ 
EXTERN_C const IID IID_ADORecordsetConstruction;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000283-0000-0010-8000-00AA006D2EA4")
    ADORecordsetConstruction : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Rowset( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRowset) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Rowset( 
            /* [in] */ __RPC__in_opt IUnknown *pRowset) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Chapter( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plChapter) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Chapter( 
            /* [in] */ ADO_LONGPTR lChapter) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RowPosition( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRowPos) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RowPosition( 
            /* [in] */ __RPC__in_opt IUnknown *pRowPos) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ADORecordsetConstructionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADORecordsetConstruction * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADORecordsetConstruction * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADORecordsetConstruction * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Rowset )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRowset);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Rowset )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pRowset);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Chapter )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *plChapter);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Chapter )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ ADO_LONGPTR lChapter);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RowPosition )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppRowPos);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RowPosition )( 
            __RPC__in ADORecordsetConstruction * This,
            /* [in] */ __RPC__in_opt IUnknown *pRowPos);
        
        END_INTERFACE
    } ADORecordsetConstructionVtbl;
    interface ADORecordsetConstruction
    {
        CONST_VTBL struct ADORecordsetConstructionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define ADORecordsetConstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define ADORecordsetConstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define ADORecordsetConstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define ADORecordsetConstruction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define ADORecordsetConstruction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define ADORecordsetConstruction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define ADORecordsetConstruction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define ADORecordsetConstruction_get_Rowset(This,ppRowset)	\
    ( (This)->lpVtbl -> get_Rowset(This,ppRowset) ) 
#define ADORecordsetConstruction_put_Rowset(This,pRowset)	\
    ( (This)->lpVtbl -> put_Rowset(This,pRowset) ) 
#define ADORecordsetConstruction_get_Chapter(This,plChapter)	\
    ( (This)->lpVtbl -> get_Chapter(This,plChapter) ) 
#define ADORecordsetConstruction_put_Chapter(This,lChapter)	\
    ( (This)->lpVtbl -> put_Chapter(This,lChapter) ) 
#define ADORecordsetConstruction_get_RowPosition(This,ppRowPos)	\
    ( (This)->lpVtbl -> get_RowPosition(This,ppRowPos) ) 
#define ADORecordsetConstruction_put_RowPosition(This,pRowPos)	\
    ( (This)->lpVtbl -> put_RowPosition(This,pRowPos) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __ADORecordsetConstruction_INTERFACE_DEFINED__ */
#ifndef __Field15_INTERFACE_DEFINED__
#define __Field15_INTERFACE_DEFINED__
/* interface Field15 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Field15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000505-0000-0010-8000-00AA006D2EA4")
    Field15 : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActualSize( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_DefinedSize( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *pDataType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT Val) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Precision( 
            /* [retval][out] */ __RPC__out BYTE *pbPrecision) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_NumericScale( 
            /* [retval][out] */ __RPC__out BYTE *pbNumericScale) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE AppendChunk( 
            /* [in] */ VARIANT Data) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetChunk( 
            /* [in] */ long Length,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_OriginalValue( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_UnderlyingValue( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Field15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Field15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Field15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Field15 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Field15 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Field15 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Field15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Field15 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActualSize )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DefinedSize )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *pDataType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in Field15 * This,
            /* [in] */ VARIANT Val);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Precision )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out BYTE *pbPrecision);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NumericScale )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out BYTE *pbNumericScale);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AppendChunk )( 
            __RPC__in Field15 * This,
            /* [in] */ VARIANT Data);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetChunk )( 
            __RPC__in Field15 * This,
            /* [in] */ long Length,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalValue )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UnderlyingValue )( 
            __RPC__in Field15 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        END_INTERFACE
    } Field15Vtbl;
    interface Field15
    {
        CONST_VTBL struct Field15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Field15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Field15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Field15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Field15_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Field15_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Field15_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Field15_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Field15_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Field15_get_ActualSize(This,pl)	\
    ( (This)->lpVtbl -> get_ActualSize(This,pl) ) 
#define Field15_get_Attributes(This,pl)	\
    ( (This)->lpVtbl -> get_Attributes(This,pl) ) 
#define Field15_get_DefinedSize(This,pl)	\
    ( (This)->lpVtbl -> get_DefinedSize(This,pl) ) 
#define Field15_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define Field15_get_Type(This,pDataType)	\
    ( (This)->lpVtbl -> get_Type(This,pDataType) ) 
#define Field15_get_Value(This,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,pvar) ) 
#define Field15_put_Value(This,Val)	\
    ( (This)->lpVtbl -> put_Value(This,Val) ) 
#define Field15_get_Precision(This,pbPrecision)	\
    ( (This)->lpVtbl -> get_Precision(This,pbPrecision) ) 
#define Field15_get_NumericScale(This,pbNumericScale)	\
    ( (This)->lpVtbl -> get_NumericScale(This,pbNumericScale) ) 
#define Field15_AppendChunk(This,Data)	\
    ( (This)->lpVtbl -> AppendChunk(This,Data) ) 
#define Field15_GetChunk(This,Length,pvar)	\
    ( (This)->lpVtbl -> GetChunk(This,Length,pvar) ) 
#define Field15_get_OriginalValue(This,pvar)	\
    ( (This)->lpVtbl -> get_OriginalValue(This,pvar) ) 
#define Field15_get_UnderlyingValue(This,pvar)	\
    ( (This)->lpVtbl -> get_UnderlyingValue(This,pvar) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Field15_INTERFACE_DEFINED__ */
#ifndef __Field20_INTERFACE_DEFINED__
#define __Field20_INTERFACE_DEFINED__
/* interface Field20 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Field20;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000054C-0000-0010-8000-00AA006D2EA4")
    Field20 : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActualSize( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_DefinedSize( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *pDataType) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT Val) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Precision( 
            /* [retval][out] */ __RPC__out BYTE *pbPrecision) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_NumericScale( 
            /* [retval][out] */ __RPC__out BYTE *pbNumericScale) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE AppendChunk( 
            /* [in] */ VARIANT Data) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetChunk( 
            /* [in] */ long Length,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_OriginalValue( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_UnderlyingValue( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DataFormat( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppiDF) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_DataFormat( 
            /* [in] */ __RPC__in_opt IUnknown *piDF) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Precision( 
            /* [in] */ BYTE bPrecision) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_NumericScale( 
            /* [in] */ BYTE bScale) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ DataTypeEnum DataType) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_DefinedSize( 
            /* [in] */ ADO_LONGPTR lSize) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ long lAttributes) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Field20Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Field20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Field20 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Field20 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Field20 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Field20 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Field20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Field20 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActualSize )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DefinedSize )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *pDataType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in Field20 * This,
            /* [in] */ VARIANT Val);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Precision )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out BYTE *pbPrecision);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NumericScale )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out BYTE *pbNumericScale);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AppendChunk )( 
            __RPC__in Field20 * This,
            /* [in] */ VARIANT Data);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetChunk )( 
            __RPC__in Field20 * This,
            /* [in] */ long Length,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalValue )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UnderlyingValue )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataFormat )( 
            __RPC__in Field20 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppiDF);
        
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DataFormat )( 
            __RPC__in Field20 * This,
            /* [in] */ __RPC__in_opt IUnknown *piDF);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Precision )( 
            __RPC__in Field20 * This,
            /* [in] */ BYTE bPrecision);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NumericScale )( 
            __RPC__in Field20 * This,
            /* [in] */ BYTE bScale);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in Field20 * This,
            /* [in] */ DataTypeEnum DataType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DefinedSize )( 
            __RPC__in Field20 * This,
            /* [in] */ ADO_LONGPTR lSize);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in Field20 * This,
            /* [in] */ long lAttributes);
        
        END_INTERFACE
    } Field20Vtbl;
    interface Field20
    {
        CONST_VTBL struct Field20Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Field20_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Field20_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Field20_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Field20_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Field20_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Field20_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Field20_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Field20_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Field20_get_ActualSize(This,pl)	\
    ( (This)->lpVtbl -> get_ActualSize(This,pl) ) 
#define Field20_get_Attributes(This,pl)	\
    ( (This)->lpVtbl -> get_Attributes(This,pl) ) 
#define Field20_get_DefinedSize(This,pl)	\
    ( (This)->lpVtbl -> get_DefinedSize(This,pl) ) 
#define Field20_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define Field20_get_Type(This,pDataType)	\
    ( (This)->lpVtbl -> get_Type(This,pDataType) ) 
#define Field20_get_Value(This,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,pvar) ) 
#define Field20_put_Value(This,Val)	\
    ( (This)->lpVtbl -> put_Value(This,Val) ) 
#define Field20_get_Precision(This,pbPrecision)	\
    ( (This)->lpVtbl -> get_Precision(This,pbPrecision) ) 
#define Field20_get_NumericScale(This,pbNumericScale)	\
    ( (This)->lpVtbl -> get_NumericScale(This,pbNumericScale) ) 
#define Field20_AppendChunk(This,Data)	\
    ( (This)->lpVtbl -> AppendChunk(This,Data) ) 
#define Field20_GetChunk(This,Length,pvar)	\
    ( (This)->lpVtbl -> GetChunk(This,Length,pvar) ) 
#define Field20_get_OriginalValue(This,pvar)	\
    ( (This)->lpVtbl -> get_OriginalValue(This,pvar) ) 
#define Field20_get_UnderlyingValue(This,pvar)	\
    ( (This)->lpVtbl -> get_UnderlyingValue(This,pvar) ) 
#define Field20_get_DataFormat(This,ppiDF)	\
    ( (This)->lpVtbl -> get_DataFormat(This,ppiDF) ) 
#define Field20_putref_DataFormat(This,piDF)	\
    ( (This)->lpVtbl -> putref_DataFormat(This,piDF) ) 
#define Field20_put_Precision(This,bPrecision)	\
    ( (This)->lpVtbl -> put_Precision(This,bPrecision) ) 
#define Field20_put_NumericScale(This,bScale)	\
    ( (This)->lpVtbl -> put_NumericScale(This,bScale) ) 
#define Field20_put_Type(This,DataType)	\
    ( (This)->lpVtbl -> put_Type(This,DataType) ) 
#define Field20_put_DefinedSize(This,lSize)	\
    ( (This)->lpVtbl -> put_DefinedSize(This,lSize) ) 
#define Field20_put_Attributes(This,lAttributes)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttributes) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Field20_INTERFACE_DEFINED__ */
#ifndef __Field_INTERFACE_DEFINED__
#define __Field_INTERFACE_DEFINED__
/* interface ADOField */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Field;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000569-0000-0010-8000-00AA006D2EA4")
    ADOField : public Field20
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out long *pFStatus) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct FieldVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOField * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOField * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOField * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOField * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOField * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOField * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOField * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActualSize )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DefinedSize )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *pDataType);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in struct ADOField * This,
            /* [in] */ VARIANT Val);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Precision )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out BYTE *pbPrecision);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NumericScale )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out BYTE *pbNumericScale);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AppendChunk )( 
            __RPC__in struct ADOField * This,
            /* [in] */ VARIANT Data);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetChunk )( 
            __RPC__in struct ADOField * This,
            /* [in] */ long Length,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalValue )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UnderlyingValue )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataFormat )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppiDF);
        
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DataFormat )( 
            __RPC__in struct ADOField * This,
            /* [in] */ __RPC__in_opt IUnknown *piDF);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Precision )( 
            __RPC__in struct ADOField * This,
            /* [in] */ BYTE bPrecision);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NumericScale )( 
            __RPC__in struct ADOField * This,
            /* [in] */ BYTE bScale);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in struct ADOField * This,
            /* [in] */ DataTypeEnum DataType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DefinedSize )( 
            __RPC__in struct ADOField * This,
            /* [in] */ ADO_LONGPTR lSize);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in struct ADOField * This,
            /* [in] */ long lAttributes);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in struct ADOField * This,
            /* [retval][out] */ __RPC__out long *pFStatus);
        
        END_INTERFACE
    } FieldVtbl;
    interface Field
    {
        CONST_VTBL struct FieldVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Field_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Field_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Field_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Field_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Field_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Field_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Field_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Field_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define Field_get_ActualSize(This,pl)	\
    ( (This)->lpVtbl -> get_ActualSize(This,pl) ) 
#define Field_get_Attributes(This,pl)	\
    ( (This)->lpVtbl -> get_Attributes(This,pl) ) 
#define Field_get_DefinedSize(This,pl)	\
    ( (This)->lpVtbl -> get_DefinedSize(This,pl) ) 
#define Field_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define Field_get_Type(This,pDataType)	\
    ( (This)->lpVtbl -> get_Type(This,pDataType) ) 
#define Field_get_Value(This,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,pvar) ) 
#define Field_put_Value(This,Val)	\
    ( (This)->lpVtbl -> put_Value(This,Val) ) 
#define Field_get_Precision(This,pbPrecision)	\
    ( (This)->lpVtbl -> get_Precision(This,pbPrecision) ) 
#define Field_get_NumericScale(This,pbNumericScale)	\
    ( (This)->lpVtbl -> get_NumericScale(This,pbNumericScale) ) 
#define Field_AppendChunk(This,Data)	\
    ( (This)->lpVtbl -> AppendChunk(This,Data) ) 
#define Field_GetChunk(This,Length,pvar)	\
    ( (This)->lpVtbl -> GetChunk(This,Length,pvar) ) 
#define Field_get_OriginalValue(This,pvar)	\
    ( (This)->lpVtbl -> get_OriginalValue(This,pvar) ) 
#define Field_get_UnderlyingValue(This,pvar)	\
    ( (This)->lpVtbl -> get_UnderlyingValue(This,pvar) ) 
#define Field_get_DataFormat(This,ppiDF)	\
    ( (This)->lpVtbl -> get_DataFormat(This,ppiDF) ) 
#define Field_putref_DataFormat(This,piDF)	\
    ( (This)->lpVtbl -> putref_DataFormat(This,piDF) ) 
#define Field_put_Precision(This,bPrecision)	\
    ( (This)->lpVtbl -> put_Precision(This,bPrecision) ) 
#define Field_put_NumericScale(This,bScale)	\
    ( (This)->lpVtbl -> put_NumericScale(This,bScale) ) 
#define Field_put_Type(This,DataType)	\
    ( (This)->lpVtbl -> put_Type(This,DataType) ) 
#define Field_put_DefinedSize(This,lSize)	\
    ( (This)->lpVtbl -> put_DefinedSize(This,lSize) ) 
#define Field_put_Attributes(This,lAttributes)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttributes) ) 
#define Field_get_Status(This,pFStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pFStatus) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Field_INTERFACE_DEFINED__ */
#ifndef __Fields15_INTERFACE_DEFINED__
#define __Fields15_INTERFACE_DEFINED__
/* interface Fields15 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Fields15;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000506-0000-0010-8000-00AA006D2EA4")
    Fields15 : public _ADOCollection
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt ADOField **ppvObject) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Fields15Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Fields15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Fields15 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Fields15 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Fields15 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Fields15 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Fields15 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Fields15 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Fields15 * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Fields15 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Fields15 * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Fields15 * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOField **ppvObject);
        
        END_INTERFACE
    } Fields15Vtbl;
    interface Fields15
    {
        CONST_VTBL struct Fields15Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Fields15_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Fields15_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Fields15_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Fields15_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Fields15_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Fields15_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Fields15_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Fields15_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Fields15__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Fields15_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Fields15_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Fields15_INTERFACE_DEFINED__ */
#ifndef __Fields20_INTERFACE_DEFINED__
#define __Fields20_INTERFACE_DEFINED__
/* interface Fields20 */
/* [object][helpcontext][uuid][hidden][nonextensible][dual] */ 
EXTERN_C const IID IID_Fields20;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000054D-0000-0010-8000-00AA006D2EA4")
    Fields20 : public Fields15
    {
    public:
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE _Append( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ADO_LONGPTR DefinedSize = 0,
            /* [defaultvalue][in] */ FieldAttributeEnum Attrib = adFldUnspecified) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Index) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct Fields20Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Fields20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Fields20 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Fields20 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Fields20 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Fields20 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Fields20 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Fields20 * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Fields20 * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Fields20 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Fields20 * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Fields20 * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOField **ppvObject);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_Append )( 
            __RPC__in Fields20 * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ADO_LONGPTR DefinedSize,
            /* [defaultvalue][in] */ FieldAttributeEnum Attrib);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in Fields20 * This,
            /* [in] */ VARIANT Index);
        
        END_INTERFACE
    } Fields20Vtbl;
    interface Fields20
    {
        CONST_VTBL struct Fields20Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Fields20_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Fields20_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Fields20_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Fields20_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Fields20_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Fields20_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Fields20_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Fields20_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Fields20__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Fields20_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Fields20_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#define Fields20__Append(This,Name,Type,DefinedSize,Attrib)	\
    ( (This)->lpVtbl -> _Append(This,Name,Type,DefinedSize,Attrib) ) 
#define Fields20_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Fields20_INTERFACE_DEFINED__ */
#ifndef __Fields_INTERFACE_DEFINED__
#define __Fields_INTERFACE_DEFINED__
/* interface ADOFields */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Fields;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000564-0000-0010-8000-00AA006D2EA4")
    ADOFields : public Fields20
    {
    public:
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ADO_LONGPTR DefinedSize,
            /* [defaultvalue][in] */ FieldAttributeEnum Attrib,
            /* [optional][in] */ VARIANT FieldValue) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Resync( 
            /* [defaultvalue][in] */ ResyncEnum ResyncValues = adResyncAllValues) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE CancelUpdate( void) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct FieldsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOFields * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOFields * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOFields * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOFields * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in struct ADOFields * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in struct ADOFields * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in struct ADOFields * This);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOField **ppvObject);
        
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *_Append )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ADO_LONGPTR DefinedSize,
            /* [defaultvalue][in] */ FieldAttributeEnum Attrib);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ VARIANT Index);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in struct ADOFields * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ ADO_LONGPTR DefinedSize,
            /* [defaultvalue][in] */ FieldAttributeEnum Attrib,
            /* [optional][in] */ VARIANT FieldValue);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in struct ADOFields * This);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Resync )( 
            __RPC__in struct ADOFields * This,
            /* [defaultvalue][in] */ ResyncEnum ResyncValues);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *CancelUpdate )( 
            __RPC__in struct ADOFields * This);
        
        END_INTERFACE
    } FieldsVtbl;
    interface Fields
    {
        CONST_VTBL struct FieldsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Fields_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Fields_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Fields_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Fields_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Fields_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Fields_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Fields_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Fields_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Fields__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Fields_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Fields_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#define Fields__Append(This,Name,Type,DefinedSize,Attrib)	\
    ( (This)->lpVtbl -> _Append(This,Name,Type,DefinedSize,Attrib) ) 
#define Fields_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 
#define Fields_Append(This,Name,Type,DefinedSize,Attrib,FieldValue)	\
    ( (This)->lpVtbl -> Append(This,Name,Type,DefinedSize,Attrib,FieldValue) ) 
#define Fields_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 
#define Fields_Resync(This,ResyncValues)	\
    ( (This)->lpVtbl -> Resync(This,ResyncValues) ) 
#define Fields_CancelUpdate(This)	\
    ( (This)->lpVtbl -> CancelUpdate(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Fields_INTERFACE_DEFINED__ */
#ifndef ___Parameter_INTERFACE_DEFINED__
#define ___Parameter_INTERFACE_DEFINED__
/* interface _ADOParameter */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__Parameter;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000050C-0000-0010-8000-00AA006D2EA4")
    _ADOParameter : public _ADO
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT val) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *psDataType) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ DataTypeEnum sDataType) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Direction( 
            /* [in] */ ParameterDirectionEnum lParmDirection) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out ParameterDirectionEnum *plParmDirection) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Precision( 
            /* [in] */ BYTE bPrecision) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Precision( 
            /* [retval][out] */ __RPC__out BYTE *pbPrecision) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_NumericScale( 
            /* [in] */ BYTE bScale) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_NumericScale( 
            /* [retval][out] */ __RPC__out BYTE *pbScale) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Size( 
            /* [in] */ ADO_LONGPTR l) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE AppendChunk( 
            /* [in] */ VARIANT Val) = 0;
        
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out LONG *plParmAttribs) = 0;
        
        virtual /* [helpcontext][propput][id] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ LONG lParmAttribs) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct _ParameterVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct _ADOParameter * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct _ADOParameter * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct _ADOParameter * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct _ADOParameter * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperties **ppvObject);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ VARIANT val);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *psDataType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ DataTypeEnum sDataType);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Direction )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ ParameterDirectionEnum lParmDirection);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out ParameterDirectionEnum *plParmDirection);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Precision )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ BYTE bPrecision);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Precision )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out BYTE *pbPrecision);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NumericScale )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ BYTE bScale);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NumericScale )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out BYTE *pbScale);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Size )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ ADO_LONGPTR l);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out ADO_LONGPTR *pl);
        
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *AppendChunk )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ VARIANT Val);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in struct _ADOParameter * This,
            /* [retval][out] */ __RPC__out LONG *plParmAttribs);
        
        /* [helpcontext][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in struct _ADOParameter * This,
            /* [in] */ LONG lParmAttribs);
        
        END_INTERFACE
    } _ParameterVtbl;
    interface _Parameter
    {
        CONST_VTBL struct _ParameterVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Parameter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Parameter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Parameter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Parameter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Parameter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Parameter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Parameter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Parameter_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Parameter_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define _Parameter_put_Name(This,bstr)	\
    ( (This)->lpVtbl -> put_Name(This,bstr) ) 
#define _Parameter_get_Value(This,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,pvar) ) 
#define _Parameter_put_Value(This,val)	\
    ( (This)->lpVtbl -> put_Value(This,val) ) 
#define _Parameter_get_Type(This,psDataType)	\
    ( (This)->lpVtbl -> get_Type(This,psDataType) ) 
#define _Parameter_put_Type(This,sDataType)	\
    ( (This)->lpVtbl -> put_Type(This,sDataType) ) 
#define _Parameter_put_Direction(This,lParmDirection)	\
    ( (This)->lpVtbl -> put_Direction(This,lParmDirection) ) 
#define _Parameter_get_Direction(This,plParmDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,plParmDirection) ) 
#define _Parameter_put_Precision(This,bPrecision)	\
    ( (This)->lpVtbl -> put_Precision(This,bPrecision) ) 
#define _Parameter_get_Precision(This,pbPrecision)	\
    ( (This)->lpVtbl -> get_Precision(This,pbPrecision) ) 
#define _Parameter_put_NumericScale(This,bScale)	\
    ( (This)->lpVtbl -> put_NumericScale(This,bScale) ) 
#define _Parameter_get_NumericScale(This,pbScale)	\
    ( (This)->lpVtbl -> get_NumericScale(This,pbScale) ) 
#define _Parameter_put_Size(This,l)	\
    ( (This)->lpVtbl -> put_Size(This,l) ) 
#define _Parameter_get_Size(This,pl)	\
    ( (This)->lpVtbl -> get_Size(This,pl) ) 
#define _Parameter_AppendChunk(This,Val)	\
    ( (This)->lpVtbl -> AppendChunk(This,Val) ) 
#define _Parameter_get_Attributes(This,plParmAttribs)	\
    ( (This)->lpVtbl -> get_Attributes(This,plParmAttribs) ) 
#define _Parameter_put_Attributes(This,lParmAttribs)	\
    ( (This)->lpVtbl -> put_Attributes(This,lParmAttribs) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Parameter_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Parameter;
#ifdef __cplusplus
Parameter;
#endif
#ifndef __Parameters_INTERFACE_DEFINED__
#define __Parameters_INTERFACE_DEFINED__
/* interface ADOParameters */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Parameters;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000050D-0000-0010-8000-00AA006D2EA4")
    ADOParameters : public _ADODynaCollection
    {
    public:
        virtual /* [helpcontext][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt _ADOParameter **ppvObject) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct ParametersVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOParameters * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOParameters * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOParameters * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOParameters * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in struct ADOParameters * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in struct ADOParameters * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in struct ADOParameters * This);
        
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ __RPC__in_opt IDispatch *Object);
        
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ VARIANT Index);
        
        /* [helpcontext][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in struct ADOParameters * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct _ADOParameter **ppvObject);
        
        END_INTERFACE
    } ParametersVtbl;
    interface Parameters
    {
        CONST_VTBL struct ParametersVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Parameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Parameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Parameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Parameters_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Parameters_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Parameters_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Parameters_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Parameters_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Parameters__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Parameters_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Parameters_Append(This,Object)	\
    ( (This)->lpVtbl -> Append(This,Object) ) 
#define Parameters_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 
#define Parameters_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Parameters_INTERFACE_DEFINED__ */
#ifndef __Property_INTERFACE_DEFINED__
#define __Property_INTERFACE_DEFINED__
/* interface ADOProperty */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Property;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000503-0000-0010-8000-00AA006D2EA4")
    ADOProperty : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pval) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT val) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *ptype) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out long *plAttributes) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ long lAttributes) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct PropertyVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOProperty * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOProperty * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOProperty * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in struct ADOProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pval);
        
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in struct ADOProperty * This,
            /* [in] */ VARIANT val);
        
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in struct ADOProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in struct ADOProperty * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *ptype);
        
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in struct ADOProperty * This,
            /* [retval][out] */ __RPC__out long *plAttributes);
        
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in struct ADOProperty * This,
            /* [in] */ long lAttributes);
        
        END_INTERFACE
    } PropertyVtbl;
    interface Property
    {
        CONST_VTBL struct PropertyVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Property_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Property_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Property_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Property_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Property_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Property_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Property_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Property_get_Value(This,pval)	\
    ( (This)->lpVtbl -> get_Value(This,pval) ) 
#define Property_put_Value(This,val)	\
    ( (This)->lpVtbl -> put_Value(This,val) ) 
#define Property_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define Property_get_Type(This,ptype)	\
    ( (This)->lpVtbl -> get_Type(This,ptype) ) 
#define Property_get_Attributes(This,plAttributes)	\
    ( (This)->lpVtbl -> get_Attributes(This,plAttributes) ) 
#define Property_put_Attributes(This,lAttributes)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttributes) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Property_INTERFACE_DEFINED__ */
#ifndef __Properties_INTERFACE_DEFINED__
#define __Properties_INTERFACE_DEFINED__
/* interface ADOProperties */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Properties;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000504-0000-0010-8000-00AA006D2EA4")
    ADOProperties : public _ADOCollection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperty **ppvObject) = 0;
        
    };
    
#else 	/* C style interface */
    typedef struct PropertiesVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in struct ADOProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in struct ADOProperties * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in struct ADOProperties * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in struct ADOProperties * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in struct ADOProperties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in struct ADOProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            struct ADOProperties * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in struct ADOProperties * This,
            /* [retval][out] */ __RPC__out long *c);
        
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in struct ADOProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        /* [id][helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in struct ADOProperties * This);
        
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in struct ADOProperties * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt struct ADOProperty **ppvObject);
        
        END_INTERFACE
    } PropertiesVtbl;
    interface Properties
    {
        CONST_VTBL struct PropertiesVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Properties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Properties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Properties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Properties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Properties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Properties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Properties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Properties_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Properties__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Properties_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Properties_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Properties_INTERFACE_DEFINED__ */
#endif /* __ADODB_LIBRARY_DEFINED__ */
/* interface __MIDL_itf_ado10_0001_0035 */
/* [local] */ 
#pragma warning(pop) // For C4091
extern RPC_IF_HANDLE __MIDL_itf_ado10_0001_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ado10_0001_0035_v0_0_s_ifspec;
/* Additional Prototypes for ALL interfaces */
/* end of Additional Prototypes */
#ifdef __cplusplus
}
#endif
#endif
#define ADOCommand _ADOCommand
#define ADORecordset _ADORecordset
#define ADOTransaction _ADOTransaction
#define ADOParameter _ADOParameter
#define ADOConnection _ADOConnection
#define ADOCollection _ADOCollection
#define ADODynaCollection _ADODynaCollection
#define ADORecord _ADORecord
#define ADORecField _ADORecField
#define ADOStream _ADOStream



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _ADOINT_H_
