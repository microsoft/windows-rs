/************************************************************************ 
**	D B D A O I N T. H													*
**																		*
************************************************************************* 
** Copyright (C) 1995-1997 by Microsoft Corporation						*
**		   All Rights Reserved					 						*
************************************************************************/ 
/*
 DBDAOINT.H
 OLE DAO Interface.  
This is a part of the Microsoft Data Access Objects SDK library.
See the dao*.hlp files for detailed information regarding the
Microsoft Data Access Objects SDK product.
 
*/
#ifndef _DBDAOINT_H_
#define _DBDAOINT_H_

#ifndef _INC_TCHAR
#include <tchar.h>
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// Forwards
interface _DAODBEngine;
#define DAODBEngine _DAODBEngine
interface DAOError;
interface _DAOCollection;
#define DAOCollection _DAOCollection
interface DAOErrors;
interface DAOProperty;
interface _DAODynaCollection;
#define DAODynaCollection _DAODynaCollection
interface DAOProperties;
interface DAOWorkspace;
interface DAOWorkspaces;
interface DAOConnection;
interface DAOConnections;
interface DAODatabase;
interface DAODatabases;
interface _DAOTableDef;
#define DAOTableDef _DAOTableDef
interface DAOTableDefs;
interface _DAOQueryDef;
#define DAOQueryDef _DAOQueryDef
interface DAOQueryDefs;
interface DAORecordset;
interface DAORecordsets;
interface _DAOField;
#define DAOField _DAOField
interface DAOFields;
interface _DAOIndex;
#define DAOIndex _DAOIndex
interface DAOIndexes;
interface DAOParameter;
interface DAOParameters;
interface _DAOUser;
#define DAOUser _DAOUser
interface DAOUsers;
interface _DAOGroup;
#define DAOGroup _DAOGroup
interface DAOGroups;
interface _DAORelation;
#define DAORelation _DAORelation
interface DAORelations;
interface DAOContainer;
interface DAOContainers;
interface DAODocument;
interface DAODocuments;
interface DAOIndexFields;



typedef enum RecordsetTypeEnum
    {	dbOpenTable	= 1,
	dbOpenDynaset	= 2,
	dbOpenSnapshot	= 4,
	dbOpenForwardOnly	= 8,
	dbOpenDynamic	= 16
    }	RecordsetTypeEnum;


typedef enum EditModeEnum
    {	dbEditNone	= 0,
	dbEditInProgress	= 1,
	dbEditAdd	= 2,
	dbEditChanged	= 4,
	dbEditDeleted	= 8,
	dbEditNew	= 16
    }	EditModeEnum;


typedef enum RecordsetOptionEnum
    {	dbDenyWrite	= 0x1,
	dbDenyRead	= 0x2,
	dbReadOnly	= 0x4,
	dbAppendOnly	= 0x8,
	dbInconsistent	= 0x10,
	dbConsistent	= 0x20,
	dbSQLPassThrough	= 0x40,
	dbFailOnError	= 0x80,
	dbForwardOnly	= 0x100,
	dbSeeChanges	= 0x200,
	dbRunAsync	= 0x400,
	dbExecDirect	= 0x800
    }	RecordsetOptionEnum;


typedef enum LockTypeEnum
    {	dbPessimistic	= 0x2,
	dbOptimistic	= 0x3,
	dbOptimisticValue	= 0x1,
	dbOptimisticBatch	= 0x5
    }	LockTypeEnum;


typedef enum UpdateCriteriaEnum
    {	dbCriteriaKey	= 0x1,
	dbCriteriaModValues	= 0x2,
	dbCriteriaAllCols	= 0x4,
	dbCriteriaTimestamp	= 0x8,
	dbCriteriaDeleteInsert	= 0x10,
	dbCriteriaUpdate	= 0x20
    }	UpdateCriteriaEnum;


typedef enum FieldAttributeEnum
    {	dbFixedField	= 0x1,
	dbVariableField	= 0x2,
	dbAutoIncrField	= 0x10,
	dbUpdatableField	= 0x20,
	dbSystemField	= 0x2000,
	dbHyperlinkField	= 0x8000,
	dbDescending	= 0x1
    }	FieldAttributeEnum;


typedef enum DataTypeEnum
    {	dbBoolean	= 1,
	dbByte	= 2,
	dbInteger	= 3,
	dbLong	= 4,
	dbCurrency	= 5,
	dbSingle	= 6,
	dbDouble	= 7,
	dbDate	= 8,
	dbBinary	= 9,
	dbText	= 10,
	dbLongBinary	= 11,
	dbMemo	= 12,
	dbGUID	= 15,
	dbBigInt	= 16,
	dbVarBinary	= 17,
	dbChar	= 18,
	dbNumeric	= 19,
	dbDecimal	= 20,
	dbFloat	= 21,
	dbTime	= 22,
	dbTimeStamp	= 23
    }	DataTypeEnum;


typedef enum RelationAttributeEnum
    {	dbRelationUnique	= 0x1,
	dbRelationDontEnforce	= 0x2,
	dbRelationInherited	= 0x4,
	dbRelationUpdateCascade	= 0x100,
	dbRelationDeleteCascade	= 0x1000,
	dbRelationLeft	= 0x1000000,
	dbRelationRight	= 0x2000000
    }	RelationAttributeEnum;


typedef enum TableDefAttributeEnum
    {	dbAttachExclusive	= 0x10000,
	dbAttachSavePWD	= 0x20000,
	dbSystemObject	= 0x80000002,
	dbAttachedTable	= 0x40000000,
	dbAttachedODBC	= 0x20000000,
	dbHiddenObject	= 0x1
    }	TableDefAttributeEnum;


typedef enum QueryDefTypeEnum
    {	dbQSelect	= 0,
	dbQProcedure	= 0xe0,
	dbQAction	= 0xf0,
	dbQCrosstab	= 0x10,
	dbQDelete	= 0x20,
	dbQUpdate	= 0x30,
	dbQAppend	= 0x40,
	dbQMakeTable	= 0x50,
	dbQDDL	= 0x60,
	dbQSQLPassThrough	= 0x70,
	dbQSetOperation	= 0x80,
	dbQSPTBulk	= 0x90,
	dbQCompound	= 0xa0
    }	QueryDefTypeEnum;


typedef enum QueryDefStateEnum
    {	dbQPrepare	= 1,
	dbQUnprepare	= 2
    }	QueryDefStateEnum;


typedef enum DatabaseTypeEnum
    {	dbVersion10	= 1,
	dbEncrypt	= 2,
	dbDecrypt	= 4,
	dbVersion11	= 8,
	dbVersion20	= 16,
	dbVersion30	= 32,
	dbVersion40	= 64
    }	DatabaseTypeEnum;


typedef enum CollatingOrderEnum
    {	dbSortNeutral	= 0x400,
	dbSortArabic	= 0x401,
	dbSortCyrillic	= 0x419,
	dbSortCzech	= 0x405,
	dbSortDutch	= 0x413,
	dbSortGeneral	= 0x409,
	dbSortGreek	= 0x408,
	dbSortHebrew	= 0x40d,
	dbSortHungarian	= 0x40e,
	dbSortIcelandic	= 0x40f,
	dbSortNorwdan	= 0x406,
	dbSortPDXIntl	= 0x409,
	dbSortPDXNor	= 0x406,
	dbSortPDXSwe	= 0x41d,
	dbSortPolish	= 0x415,
	dbSortSpanish	= 0x40a,
	dbSortSwedFin	= 0x41d,
	dbSortTurkish	= 0x41f,
	dbSortJapanese	= 0x411,
	dbSortChineseSimplified	= 0x804,
	dbSortChineseTraditional	= 0x404,
	dbSortKorean	= 0x412,
	dbSortThai	= 0x41e,
	dbSortSlovenian	= 0x424,
	dbSortUndefined	= -1
    }	CollatingOrderEnum;


typedef enum IdleEnum
    {	dbFreeLocks	= 1,
	dbRefreshCache	= 8
    }	IdleEnum;


typedef enum PermissionEnum
    {	dbSecNoAccess	= 0,
	dbSecFullAccess	= 0xfffff,
	dbSecDelete	= 0x10000,
	dbSecReadSec	= 0x20000,
	dbSecWriteSec	= 0x40000,
	dbSecWriteOwner	= 0x80000,
	dbSecDBCreate	= 0x1,
	dbSecDBOpen	= 0x2,
	dbSecDBExclusive	= 0x4,
	dbSecDBAdmin	= 0x8,
	dbSecCreate	= 0x1,
	dbSecReadDef	= 0x4,
	dbSecWriteDef	= 0x1000c,
	dbSecRetrieveData	= 0x14,
	dbSecInsertData	= 0x20,
	dbSecReplaceData	= 0x40,
	dbSecDeleteData	= 0x80
    }	PermissionEnum;


typedef enum SynchronizeTypeEnum
    {	dbRepExportChanges	= 0x1,
	dbRepImportChanges	= 0x2,
	dbRepImpExpChanges	= 0x4,
	dbRepSyncInternet	= 0x10
    }	SynchronizeTypeEnum;


typedef enum ReplicaTypeEnum
    {	dbRepMakeReadOnly	= 0x2,
	dbRepMakePartial	= 0x1
    }	ReplicaTypeEnum;


typedef enum WorkspaceTypeEnum
    {	dbUseODBC	= 1,
	dbUseJet	= 2
    }	WorkspaceTypeEnum;


typedef enum CursorDriverEnum
    {	dbUseDefaultCursor	= -1,
	dbUseODBCCursor	= 1,
	dbUseServerCursor	= 2,
	dbUseClientBatchCursor	= 3,
	dbUseNoCursor	= 4
    }	CursorDriverEnum;


typedef enum DriverPromptEnum
    {	dbDriverPrompt	= 2,
	dbDriverNoPrompt	= 1,
	dbDriverComplete	= 0,
	dbDriverCompleteRequired	= 3
    }	DriverPromptEnum;


typedef enum SetOptionEnum
    {	dbPageTimeout	= 6,
	dbLockRetry	= 57,
	dbMaxBufferSize	= 8,
	dbUserCommitSync	= 58,
	dbImplicitCommitSync	= 59,
	dbExclusiveAsyncDelay	= 60,
	dbSharedAsyncDelay	= 61,
	dbMaxLocksPerFile	= 62,
	dbLockDelay	= 63,
	dbRecycleLVs	= 65,
	dbFlushTransactionTimeout	= 66
    }	SetOptionEnum;


typedef enum ParameterDirectionEnum
    {	dbParamInput	= 1,
	dbParamOutput	= 2,
	dbParamInputOutput	= 3,
	dbParamReturnValue	= 4
    }	ParameterDirectionEnum;


typedef enum UpdateTypeEnum
    {	dbUpdateBatch	= 4,
	dbUpdateRegular	= 1,
	dbUpdateCurrentRecord	= 2
    }	UpdateTypeEnum;


typedef enum RecordStatusEnum
    {	dbRecordUnmodified	= 0,
	dbRecordModified	= 1,
	dbRecordNew	= 2,
	dbRecordDeleted	= 3,
	dbRecordDBDeleted	= 4
    }	RecordStatusEnum;


typedef enum CommitTransOptionsEnum
    {	dbForceOSFlush	= 1
    }	CommitTransOptionsEnum;


typedef enum _DAOSuppHelp
    {	LogMessages	= 0,
	KeepLocal	= 0,
	Replicable	= 0,
	ReplicableBool	= 0,
	V1xNullBehavior	= 0
    }	_DAOSuppHelp;

#define dbLangArabic _T(";LANGID=0x0401;CP=1256;COUNTRY=0")
#define dbLangCzech _T(";LANGID=0x0405;CP=1250;COUNTRY=0")
#define dbLangDutch _T(";LANGID=0x0413;CP=1252;COUNTRY=0")
#define dbLangGeneral _T(";LANGID=0x0409;CP=1252;COUNTRY=0")
#define dbLangGreek _T(";LANGID=0x0408;CP=1253;COUNTRY=0")
#define dbLangHebrew _T(";LANGID=0x040D;CP=1255;COUNTRY=0")
#define dbLangHungarian _T(";LANGID=0x040E;CP=1250;COUNTRY=0")
#define dbLangIcelandic _T(";LANGID=0x040F;CP=1252;COUNTRY=0")
#define dbLangNordic _T(";LANGID=0x041D;CP=1252;COUNTRY=0")
#define dbLangNorwDan _T(";LANGID=0x0414;CP=1252;COUNTRY=0")
#define dbLangPolish _T(";LANGID=0x0415;CP=1250;COUNTRY=0")
#define dbLangCyrillic _T(";LANGID=0x0419;CP=1251;COUNTRY=0")
#define dbLangSpanish _T(";LANGID=0x040A;CP=1252;COUNTRY=0")
#define dbLangSwedFin _T(";LANGID=0x040B;CP=1252;COUNTRY=0")
#define dbLangTurkish _T(";LANGID=0x041F;CP=1254;COUNTRY=0")
#define dbLangJapanese _T(";LANGID=0x0411;CP=932;COUNTRY=0")
#define dbLangChineseSimplified _T(";LANGID=0x0804;CP=936;COUNTRY=0")
#define dbLangChineseTraditional _T(";LANGID=0x0404;CP=950;COUNTRY=0")
#define dbLangKorean _T(";LANGID=0x0412;CP=949;COUNTRY=0")
#define dbLangThai _T(";LANGID=0x041E;CP=874;COUNTRY=0")
#define dbLangSlovenian _T(";LANGID=0x0424;CP=1250;COUNTRY=0")
// Interface: _DAOCollection
#undef INTERFACE
#define INTERFACE _DAOCollection
DECLARE_INTERFACE_(_DAOCollection, IDispatch)
	{
	STDMETHOD(get_Count)						 (THIS_ short FAR* c) PURE;
	STDMETHOD(_NewEnum)							 (THIS_ IUnknown * FAR* ppunk) PURE;
	STDMETHOD(Refresh)							 (THIS) PURE;
	};
 
// Interface: _DAODynaCollection
#undef INTERFACE
#define INTERFACE _DAODynaCollection
DECLARE_INTERFACE_(_DAODynaCollection, _DAOCollection)
	{
	STDMETHOD(Append)							 (THIS_ IDispatch * Object) PURE;
	STDMETHOD(Delete)							 (THIS_ BSTR Name) PURE;
	};
 
// Interface: _DAO
#undef INTERFACE
#define INTERFACE _DAO
DECLARE_INTERFACE_(_DAO, IDispatch)
	{
	STDMETHOD(get_Properties)					 (THIS_ DAOProperties FAR* FAR* ppprops) PURE;
	};
// Interface: _DAODBEngine
#undef INTERFACE
#define INTERFACE _DAODBEngine
DECLARE_INTERFACE_(_DAODBEngine, _DAO)
{
	STDMETHOD( get_Properties )						(
			/* [retval][out] */ DAOProperties __RPC_FAR *__RPC_FAR *ppprops );
	STDMETHOD( get_Version )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_IniPath )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_IniPath )					 ( 
    
 /* [in] */ BSTR path ) PURE;
	STDMETHOD( put_DefaultUser )				 ( 
    
 /* [in] */ BSTR user ) PURE;
	STDMETHOD( put_DefaultPassword )			 ( 
    
 /* [in] */ BSTR pw ) PURE;
	STDMETHOD( get_LoginTimeout )				 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_LoginTimeout )				 ( 
    
 /* [in] */ short Timeout ) PURE;
	STDMETHOD( get_Workspaces )					 ( 
    
 /* [retval][out] */ DAOWorkspaces __RPC_FAR *__RPC_FAR *ppworks ) PURE;
	STDMETHOD( get_Errors )						 ( 
    
 /* [retval][out] */ DAOErrors __RPC_FAR *__RPC_FAR *pperrs ) PURE;
	STDMETHOD( Idle )							 ( 
    
 /* [optional][in] */ VARIANT Action ) PURE;
	STDMETHOD( CompactDatabase )				 ( 
    
 /* [in] */ BSTR SrcName,
 /* [in] */ BSTR DstName,
 /* [optional][in] */ VARIANT DstLocale,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT SrcLocale ) PURE;
	STDMETHOD( RepairDatabase )					 ( 
    
 /* [in] */ BSTR Name ) PURE;
	STDMETHOD( RegisterDatabase )				 ( 
    
 /* [in] */ BSTR Dsn,
 /* [in] */ BSTR Driver,
 /* [in] */ VARIANT_BOOL Silent,
 /* [in] */ BSTR Attributes ) PURE;
	STDMETHOD( _30_CreateWorkspace )			 ( 
    
 /* [in] */ BSTR Name,
 /* [in] */ BSTR UserName,
 /* [in] */ BSTR Password,
 /* [retval][out] */ DAOWorkspace __RPC_FAR *__RPC_FAR *ppwrk ) PURE;
	STDMETHOD( OpenDatabase )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT ReadOnly,
 /* [optional][in] */ VARIANT Connect,
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppdb ) PURE;
	STDMETHOD( CreateDatabase )					 ( 
    
 /* [in] */ BSTR Name,
 /* [in] */ BSTR Locale,
 /* [optional][in] */ VARIANT Option,
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppdb ) PURE;
	STDMETHOD( FreeLocks )						 ( 
   			VOID ) PURE;
	STDMETHOD( BeginTrans )						 ( 
   			VOID ) PURE;
	STDMETHOD( CommitTrans )					 ( 
    
 /* [defaultvalue][in] */ long Option ) PURE;
	STDMETHOD( Rollback )						 ( 
   			VOID ) PURE;
	STDMETHOD( SetDefaultWorkspace )			 ( 
    
 /* [in] */ BSTR Name,
 /* [in] */ BSTR Password ) PURE;
	STDMETHOD( SetDataAccessOption )			 ( 
    
 /* [in] */ short Option,
 /* [in] */ VARIANT Value ) PURE;
	STDMETHOD( ISAMStats )						 ( 
    
 /* [in] */ long StatNum,
 /* [optional][in] */ VARIANT Reset,
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_SystemDB )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_SystemDB )					 ( 
    
 /* [in] */ BSTR SystemDBPath ) PURE;
	STDMETHOD( CreateWorkspace )				 ( 
    
 /* [in] */ BSTR Name,
 /* [in] */ BSTR UserName,
 /* [in] */ BSTR Password,
 /* [optional][in] */ VARIANT UseType,
 /* [retval][out] */ DAOWorkspace __RPC_FAR *__RPC_FAR *ppwrk ) PURE;
	STDMETHOD( OpenConnection )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT ReadOnly,
 /* [optional][in] */ VARIANT Connect,
 /* [retval][out] */ DAOConnection __RPC_FAR *__RPC_FAR *ppconn ) PURE;
	STDMETHOD( get_DefaultType )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *Option ) PURE;
	STDMETHOD( put_DefaultType )				 ( 
    
 /* [in] */ long Option ) PURE;
	STDMETHOD( SetOption )						 ( 
    
 /* [in] */ LONG Option,
 /* [in] */ VARIANT Value ) PURE;
	STDMETHOD( DumpObjects )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( DebugPrint )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	};// end interface;

// Interface: DAOError
#undef INTERFACE
#define INTERFACE DAOError
DECLARE_INTERFACE_(DAOError, IDispatch)
{
	STDMETHOD( get_Number )						 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Source )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Description )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_HelpFile )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_HelpContext )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	};// end interface;

// Interface: DAOErrors
#undef INTERFACE
#define INTERFACE DAOErrors
DECLARE_INTERFACE_(DAOErrors, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOError __RPC_FAR *__RPC_FAR *pperr ) PURE;
	};// end interface;

// Interface: DAOProperty
#undef INTERFACE
#define INTERFACE DAOProperty
DECLARE_INTERFACE_(DAOProperty, _DAO)
{
	STDMETHOD( get_Value )						 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pval ) PURE;
	STDMETHOD( put_Value )						 ( 
    
 /* [in] */ VARIANT val ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ short __RPC_FAR *ptype ) PURE;
	STDMETHOD( put_Type )						 ( 
    
 /* [in] */ short type ) PURE;
	STDMETHOD( get_Inherited )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	};// end interface;

// Interface: DAOProperties
#undef INTERFACE
#define INTERFACE DAOProperties
DECLARE_INTERFACE_(DAOProperties, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *ppprop ) PURE;
	};// end interface;

// Interface: DAOWorkspace
#undef INTERFACE
#define INTERFACE DAOWorkspace
DECLARE_INTERFACE_(DAOWorkspace, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR Name ) PURE;
	STDMETHOD( get_UserName )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put__30_UserName )				 ( 
    
 /* [in] */ BSTR UserName ) PURE;
	STDMETHOD( put__30_Password )				 ( 
    
 /* [in] */ BSTR Password ) PURE;
	STDMETHOD( get_IsolateODBCTrans )			 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_IsolateODBCTrans )			 ( 
    
 /* [in] */ short s ) PURE;
	STDMETHOD( get_Databases )					 ( 
    
 /* [retval][out] */ DAODatabases __RPC_FAR *__RPC_FAR *ppdbs ) PURE;
	STDMETHOD( get_Users )						 ( 
    
 /* [retval][out] */ DAOUsers __RPC_FAR *__RPC_FAR *ppusrs ) PURE;
	STDMETHOD( get_Groups )						 ( 
    
 /* [retval][out] */ DAOGroups __RPC_FAR *__RPC_FAR *ppgrps ) PURE;
	STDMETHOD( BeginTrans )						 ( 
   			VOID ) PURE;
	STDMETHOD( CommitTrans )					 ( 
    
 /* [defaultvalue][in] */ long Options ) PURE;
	STDMETHOD( Close )							 ( 
   			VOID ) PURE;
	STDMETHOD( Rollback )						 ( 
   			VOID ) PURE;
	STDMETHOD( OpenDatabase )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT ReadOnly,
 /* [optional][in] */ VARIANT Connect,
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppdb ) PURE;
	STDMETHOD( CreateDatabase )					 ( 
    
 /* [in] */ BSTR Name,
 /* [in] */ BSTR Connect,
 /* [optional][in] */ VARIANT Option,
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppdb ) PURE;
	STDMETHOD( CreateUser )						 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT PID,
 /* [optional][in] */ VARIANT Password,
 /* [retval][out] */ DAOUser __RPC_FAR *__RPC_FAR *ppusr ) PURE;
	STDMETHOD( CreateGroup )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT PID,
 /* [retval][out] */ DAOGroup __RPC_FAR *__RPC_FAR *ppgrp ) PURE;
	STDMETHOD( OpenConnection )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT ReadOnly,
 /* [optional][in] */ VARIANT Connect,
 /* [retval][out] */ DAOConnection __RPC_FAR *__RPC_FAR *ppconn ) PURE;
	STDMETHOD( get_LoginTimeout )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pTimeout ) PURE;
	STDMETHOD( put_LoginTimeout )				 ( 
    
 /* [in] */ long Timeout ) PURE;
	STDMETHOD( get_DefaultCursorDriver )		 ( 
    
 /* [retval][out] */ long __RPC_FAR *pCursorType ) PURE;
	STDMETHOD( put_DefaultCursorDriver )		 ( 
    
 /* [in] */ long CursorType ) PURE;
	STDMETHOD( get_hEnv )						 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *phEnv ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *ptype ) PURE;
	STDMETHOD( get_Connections )				 ( 
    
 /* [retval][out] */ DAOConnections __RPC_FAR *__RPC_FAR *ppcns ) PURE;
	};// end interface;

// Interface: DAOWorkspaces
#undef INTERFACE
#define INTERFACE DAOWorkspaces
DECLARE_INTERFACE_(DAOWorkspaces, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOWorkspace __RPC_FAR *__RPC_FAR *ppwrk ) PURE;
	};// end interface;

// Interface: DAOConnection
#undef INTERFACE
#define INTERFACE DAOConnection
DECLARE_INTERFACE_(DAOConnection, IDispatch)
{
	STDMETHOD( QueryInterface )					 ( REFIID riid, LPVOID FAR* ppvObj );
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Connect )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Database )					 ( 
    
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppDb ) PURE;
	STDMETHOD( get_hDbc )						 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *phDbc ) PURE;
	STDMETHOD( get_QueryTimeout )				 ( 
    
 /* [retval][out] */ SHORT __RPC_FAR *pSeconds ) PURE;
	STDMETHOD( put_QueryTimeout )				 ( 
    
 /* [in] */ SHORT Seconds ) PURE;
	STDMETHOD( get_Transactions )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_RecordsAffected )			 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *pRecords ) PURE;
	STDMETHOD( get_StillExecuting )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pStillExec ) PURE;
	STDMETHOD( get_Updatable )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pStillExec ) PURE;
	STDMETHOD( get_QueryDefs )					 ( 
    
 /* [retval][out] */ DAOQueryDefs __RPC_FAR *__RPC_FAR *ppqdfs ) PURE;
	STDMETHOD( get_Recordsets )					 ( 
    
 /* [retval][out] */ DAORecordsets __RPC_FAR *__RPC_FAR *pprsts ) PURE;
	STDMETHOD( Cancel )							 ( 
   			VOID ) PURE;
	STDMETHOD( Close )							 ( 
   			VOID ) PURE;
	STDMETHOD( CreateQueryDef )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT SQLText,
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdf ) PURE;
	STDMETHOD( Execute )						 ( 
    
 /* [in] */ BSTR Query,
 /* [optional][in] */ VARIANT Options ) PURE;
	STDMETHOD( OpenRecordset )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT LockEdit,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	};// end interface;

// Interface: DAOConnections
#undef INTERFACE
#define INTERFACE DAOConnections
DECLARE_INTERFACE_(DAOConnections, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOConnection __RPC_FAR *__RPC_FAR *ppconn ) PURE;
	};// end interface;

// Interface: DAODatabase
#undef INTERFACE
#define INTERFACE DAODatabase
DECLARE_INTERFACE_(DAODatabase, _DAO)
{
	STDMETHOD( get_CollatingOrder )				 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Connect )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_QueryTimeout )				 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_QueryTimeout )				 ( 
    
 /* [in] */ short Timeout ) PURE;
	STDMETHOD( get_Transactions )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Updatable )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Version )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_RecordsAffected )			 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_TableDefs )					 ( 
    
 /* [retval][out] */ DAOTableDefs __RPC_FAR *__RPC_FAR *pptdfs ) PURE;
	STDMETHOD( get_QueryDefs )					 ( 
    
 /* [retval][out] */ DAOQueryDefs __RPC_FAR *__RPC_FAR *ppqdfs ) PURE;
	STDMETHOD( get_Relations )					 ( 
    
 /* [retval][out] */ DAORelations __RPC_FAR *__RPC_FAR *pprls ) PURE;
	STDMETHOD( get_Containers )					 ( 
    
 /* [retval][out] */ DAOContainers __RPC_FAR *__RPC_FAR *ppctns ) PURE;
	STDMETHOD( get_Recordsets )					 ( 
    
 /* [retval][out] */ DAORecordsets __RPC_FAR *__RPC_FAR *pprsts ) PURE;
	STDMETHOD( Close )							 ( 
   			VOID ) PURE;
	STDMETHOD( Execute )						 ( 
    
 /* [in] */ BSTR Query,
 /* [optional][in] */ VARIANT Options ) PURE;
	STDMETHOD( _30_OpenRecordset )				 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	STDMETHOD( CreateRelation )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Table,
 /* [optional][in] */ VARIANT ForeignTable,
 /* [optional][in] */ VARIANT Attributes,
 /* [retval][out] */ DAORelation __RPC_FAR *__RPC_FAR *pprel ) PURE;
	STDMETHOD( CreateTableDef )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Attributes,
 /* [optional][in] */ VARIANT SourceTablename,
 /* [optional][in] */ VARIANT Connect,
 /* [retval][out] */ DAOTableDef __RPC_FAR *__RPC_FAR *pptdf ) PURE;
	STDMETHOD( BeginTrans )						 ( 
   			VOID ) PURE;
	STDMETHOD( CommitTrans )					 ( 
    
 /* [defaultvalue][in] */ long Options ) PURE;
	STDMETHOD( Rollback )						 ( 
   			VOID ) PURE;
	STDMETHOD( CreateDynaset )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT Inconsistent,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CreateQueryDef )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT SQLText,
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdf ) PURE;
	STDMETHOD( CreateSnapshot )					 ( 
    
 /* [in] */ BSTR Source,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( DeleteQueryDef )					 ( 
    
 /* [in] */ BSTR Name ) PURE;
	STDMETHOD( ExecuteSQL )						 ( 
    
 /* [in] */ BSTR SQL,
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( ListFields )						 ( 
    
 /* [in] */ BSTR Name,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( ListTables )						 ( 
    
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( OpenQueryDef )					 ( 
    
 /* [in] */ BSTR Name,
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdf ) PURE;
	STDMETHOD( OpenTable )						 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( get_ReplicaID )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_DesignMasterID )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_DesignMasterID )				 ( 
    
 /* [in] */ BSTR MasterID ) PURE;
	STDMETHOD( Synchronize )					 ( 
    
 /* [in] */ BSTR DbPathName,
 /* [optional][in] */ VARIANT ExchangeType ) PURE;
	STDMETHOD( MakeReplica )					 ( 
    
 /* [in] */ BSTR PathName,
 /* [in] */ BSTR Description,
 /* [optional][in] */ VARIANT Options ) PURE;
	STDMETHOD( put_Connect )					 ( 
    
 /* [in] */ BSTR ODBCConnnect ) PURE;
	STDMETHOD( NewPassword )					 ( 
    
 /* [in] */ BSTR bstrOld,
 /* [in] */ BSTR bstrNew ) PURE;
	STDMETHOD( OpenRecordset )					 ( 
    
 /* [in] */ BSTR Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT LockEdit,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( get_Connection )					 ( 
    
 /* [retval][out] */ DAOConnection __RPC_FAR *__RPC_FAR *ppCn ) PURE;
	STDMETHOD( PopulatePartial )				 ( 
    
 /* [in] */ BSTR DbPathName ) PURE;
	};// end interface;

// Interface: DAODatabases
#undef INTERFACE
#define INTERFACE DAODatabases
DECLARE_INTERFACE_(DAODatabases, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *ppdb ) PURE;
	};// end interface;

// Interface: _DAOTableDef
#undef INTERFACE
#define INTERFACE _DAOTableDef
DECLARE_INTERFACE_(_DAOTableDef, _DAO)
{
	STDMETHOD( get_Attributes )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Attributes )					 ( 
    
 /* [in] */ long Attributes ) PURE;
	STDMETHOD( get_Connect )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Connect )					 ( 
    
 /* [in] */ BSTR Connection ) PURE;
	STDMETHOD( get_DateCreated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_LastUpdated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR Name ) PURE;
	STDMETHOD( get_SourceTableName )			 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_SourceTableName )			 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Updatable )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_ValidationText )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ValidationText )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_ValidationRule )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ValidationRule )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_RecordCount )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Fields )						 ( 
    
 /* [retval][out] */ DAOFields __RPC_FAR *__RPC_FAR *ppflds ) PURE;
	STDMETHOD( get_Indexes )					 ( 
    
 /* [retval][out] */ DAOIndexes __RPC_FAR *__RPC_FAR *ppidxs ) PURE;
	STDMETHOD( OpenRecordset )					 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( RefreshLink )					 ( 
   			VOID ) PURE;
	STDMETHOD( CreateField )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Size,
 /* [retval][out] */ DAOField __RPC_FAR *__RPC_FAR *ppfld ) PURE;
	STDMETHOD( CreateIndex )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [retval][out] */ DAOIndex __RPC_FAR *__RPC_FAR *ppidx ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	STDMETHOD( get_ConflictTable )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_ReplicaFilter )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pFilter ) PURE;
	STDMETHOD( put_ReplicaFilter )				 ( 
    
 /* [in] */ VARIANT Filter ) PURE;
	};// end interface;

// Interface: DAOTableDefs
#undef INTERFACE
#define INTERFACE DAOTableDefs
DECLARE_INTERFACE_(DAOTableDefs, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOTableDef __RPC_FAR *__RPC_FAR *pptdf ) PURE;
	};// end interface;

// Interface: _DAOQueryDef
#undef INTERFACE
#define INTERFACE _DAOQueryDef
DECLARE_INTERFACE_(_DAOQueryDef, _DAO)
{
	STDMETHOD( get_DateCreated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_LastUpdated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_ODBCTimeout )				 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_ODBCTimeout )				 ( 
    
 /* [in] */ short timeout ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ short __RPC_FAR *pi ) PURE;
	STDMETHOD( get_SQL )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_SQL )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Updatable )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Connect )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Connect )					 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_ReturnsRecords )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_ReturnsRecords )				 ( 
    
 /* [in] */ VARIANT_BOOL f ) PURE;
	STDMETHOD( get_RecordsAffected )			 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Fields )						 ( 
    
 /* [retval][out] */ DAOFields __RPC_FAR *__RPC_FAR *ppflds ) PURE;
	STDMETHOD( get_Parameters )					 ( 
    
 /* [retval][out] */ DAOParameters __RPC_FAR *__RPC_FAR *ppprms ) PURE;
	STDMETHOD( Close )							 ( 
   			VOID ) PURE;
	STDMETHOD( _30_OpenRecordset )				 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( _30__OpenRecordset )				 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( _Copy )							 ( 
    
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdf ) PURE;
	STDMETHOD( Execute )						 ( 
    
 /* [optional][in] */ VARIANT Options ) PURE;
	STDMETHOD( Compare )						 ( 
    
 /* [in] */ DAOQueryDef __RPC_FAR *pQdef,
 /* [in] */ SHORT __RPC_FAR *lps ) PURE;
	STDMETHOD( CreateDynaset )					 ( 
    
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT Inconsistent,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CreateSnapshot )					 ( 
    
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( ListParameters )					 ( 
    
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	STDMETHOD( OpenRecordset )					 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT LockEdit,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( _OpenRecordset )					 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT LockEdit,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( Cancel )							 ( 
   			VOID ) PURE;
	STDMETHOD( get_hStmt )						 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *phStmt ) PURE;
	STDMETHOD( get_MaxRecords )					 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *pMxRecs ) PURE;
	STDMETHOD( put_MaxRecords )					 ( 
    
 /* [in] */ LONG MxRecs ) PURE;
	STDMETHOD( get_StillExecuting )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pStillExec ) PURE;
	STDMETHOD( get_CacheSize )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *lCacheSize ) PURE;
	STDMETHOD( put_CacheSize )					 ( 
    
 /* [in] */ long lCacheSize ) PURE;
	STDMETHOD( get_Prepare )					 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Prepare )					 ( 
    
 /* [in] */ VARIANT f ) PURE;
	};// end interface;

// Interface: DAOQueryDefs
#undef INTERFACE
#define INTERFACE DAOQueryDefs
DECLARE_INTERFACE_(DAOQueryDefs, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdef ) PURE;
	};// end interface;

// Interface: DAORecordset
#undef INTERFACE
#define INTERFACE DAORecordset
DECLARE_INTERFACE_(DAORecordset, _DAO)
{
	STDMETHOD( GetIDsOfNames )					 (      REFIID riid,      OLECHAR FAR* FAR* rgszNames,      UINT cNames,      LCID lcid,      DISPID FAR* rgdispid );
	STDMETHOD( Invoke )							 (      DISPID dispidMember,      REFIID riid,      LCID lcid,      WORD wFlags,      DISPPARAMS FAR* pdispparams,      VARIANT FAR* pvarResult,      EXCEPINFO FAR* pexcepinfo,      UINT FAR* puArgErr );
	STDMETHOD( get_BOF )						 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Bookmark )					 ( 
    
 /* [retval][out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *ppsach ) PURE;
	STDMETHOD( put_Bookmark )					 ( 
    
 /* [in] */ SAFEARRAY __RPC_FAR * __RPC_FAR *psach ) PURE;
	STDMETHOD( get_Bookmarkable )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_DateCreated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_EOF )						 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Filter )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Filter )						 ( 
    
 /* [in] */ BSTR Filter ) PURE;
	STDMETHOD( get_Index )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Index )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_LastModified )				 ( 
    
 /* [retval][out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *ppsa ) PURE;
	STDMETHOD( get_LastUpdated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_LockEdits )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_LockEdits )					 ( 
    
 /* [in] */ VARIANT_BOOL Lock ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_NoMatch )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Sort )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Sort )						 ( 
    
 /* [in] */ BSTR Sort ) PURE;
	STDMETHOD( get_Transactions )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( get_RecordCount )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Updatable )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Restartable )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_ValidationText )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_ValidationRule )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_CacheStart )					 ( 
    
 /* [retval][out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *ppsa ) PURE;
	STDMETHOD( put_CacheStart )					 ( 
    
 /* [in] */ SAFEARRAY __RPC_FAR * __RPC_FAR *psa ) PURE;
	STDMETHOD( get_CacheSize )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_CacheSize )					 ( 
    
 /* [in] */ long CacheSize ) PURE;
	STDMETHOD( get_PercentPosition )			 ( 
    
 /* [retval][out] */ float __RPC_FAR *pd ) PURE;
	STDMETHOD( put_PercentPosition )			 ( 
    
 /* [in] */ float Position ) PURE;
	STDMETHOD( get_AbsolutePosition )			 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_AbsolutePosition )			 ( 
    
 /* [in] */ long Position ) PURE;
	STDMETHOD( get_EditMode )					 ( 
    
 /* [retval][out] */ short __RPC_FAR *pi ) PURE;
	STDMETHOD( get_ODBCFetchCount )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_ODBCFetchDelay )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Parent )						 ( 
    
 /* [retval][out] */ DAODatabase __RPC_FAR *__RPC_FAR *pdb ) PURE;
	STDMETHOD( get_Fields )						 ( 
    
 /* [retval][out] */ DAOFields __RPC_FAR *__RPC_FAR *ppflds ) PURE;
	STDMETHOD( get_Indexes )					 ( 
    
 /* [retval][out] */ DAOIndexes __RPC_FAR *__RPC_FAR *ppidxs ) PURE;
	STDMETHOD( _30_CancelUpdate )				 ( 
   			VOID ) PURE;
	STDMETHOD( AddNew )							 ( 
   			VOID ) PURE;
	STDMETHOD( Close )							 ( 
   			VOID ) PURE;
	STDMETHOD( OpenRecordset )					 ( 
    
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( Delete )							 ( 
   			VOID ) PURE;
	STDMETHOD( Edit )							 ( 
   			VOID ) PURE;
	STDMETHOD( FindFirst )						 ( 
    
 /* [in] */ BSTR Criteria ) PURE;
	STDMETHOD( FindLast )						 ( 
    
 /* [in] */ BSTR Criteria ) PURE;
	STDMETHOD( FindNext )						 ( 
    
 /* [in] */ BSTR Criteria ) PURE;
	STDMETHOD( FindPrevious )					 ( 
    
 /* [in] */ BSTR Criteria ) PURE;
	STDMETHOD( MoveFirst )						 ( 
   			VOID ) PURE;
	STDMETHOD( _30_MoveLast )					 ( 
   			VOID ) PURE;
	STDMETHOD( MoveNext )						 ( 
   			VOID ) PURE;
	STDMETHOD( MovePrevious )					 ( 
   			VOID ) PURE;
	STDMETHOD( Seek )							 ( 
    
 /* [in] */ BSTR Comparison,
 /* [in] */ VARIANT Key1,
 /* [optional][in] */ VARIANT Key2,
 /* [optional][in] */ VARIANT Key3,
 /* [optional][in] */ VARIANT Key4,
 /* [optional][in] */ VARIANT Key5,
 /* [optional][in] */ VARIANT Key6,
 /* [optional][in] */ VARIANT Key7,
 /* [optional][in] */ VARIANT Key8,
 /* [optional][in] */ VARIANT Key9,
 /* [optional][in] */ VARIANT Key10,
 /* [optional][in] */ VARIANT Key11,
 /* [optional][in] */ VARIANT Key12,
 /* [optional][in] */ VARIANT Key13 ) PURE;
	STDMETHOD( _30_Update )						 ( 
   			VOID ) PURE;
	STDMETHOD( Clone )							 ( 
    
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( Requery )						 ( 
    
 /* [optional][in] */ VARIANT NewQueryDef ) PURE;
	STDMETHOD( Move )							 ( 
    
 /* [in] */ long Rows,
 /* [optional][in] */ VARIANT StartBookmark ) PURE;
	STDMETHOD( FillCache )						 ( 
    
 /* [optional][in] */ VARIANT Rows,
 /* [optional][in] */ VARIANT StartBookmark ) PURE;
	STDMETHOD( CreateDynaset )					 ( 
    
 /* [optional][in] */ VARIANT Options,
 /* [optional][in] */ VARIANT Inconsistent,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CreateSnapshot )					 ( 
    
 /* [optional][in] */ VARIANT Options,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( CopyQueryDef )					 ( 
    
 /* [retval][out] */ DAOQueryDef __RPC_FAR *__RPC_FAR *ppqdf ) PURE;
	STDMETHOD( ListFields )						 ( 
    
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( ListIndexes )					 ( 
    
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	STDMETHOD( GetRows )						 ( 
    
 /* [optional][in] */ VARIANT NumRows,
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_Collect )					 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( put_Collect )					 ( 
    
 /* [in] */ VARIANT Item,
 /* [in] */ VARIANT value ) PURE;
	STDMETHOD( Cancel )							 ( 
   			VOID ) PURE;
	STDMETHOD( NextRecordset )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_hStmt )						 ( 
    
 /* [retval][out] */ LONG __RPC_FAR *phStmt ) PURE;
	STDMETHOD( get_StillExecuting )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pStillExec ) PURE;
	STDMETHOD( get_BatchSize )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_BatchSize )					 ( 
    
 /* [in] */ long BatchSize ) PURE;
	STDMETHOD( get_BatchCollisionCount )		 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_BatchCollisions )			 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_Connection )					 ( 
    
 /* [retval][out] */ DAOConnection __RPC_FAR *__RPC_FAR *ppCn ) PURE;
	STDMETHOD( putref_Connection )				 ( 
    
 /* [in] */ DAOConnection __RPC_FAR *pNewCn ) PURE;
	STDMETHOD( get_RecordStatus )				 ( 
    
 /* [retval][out] */ short __RPC_FAR *pi ) PURE;
	STDMETHOD( get_UpdateOptions )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_UpdateOptions )				 ( 
    
 /* [in] */ long l ) PURE;
	STDMETHOD( CancelUpdate )					 ( 
    
 /* [defaultvalue][in] */ long UpdateType ) PURE;
	STDMETHOD( Update )							 ( 
    
 /* [defaultvalue][in] */ long UpdateType,
 /* [defaultvalue][in] */ VARIANT_BOOL Force ) PURE;
	STDMETHOD( MoveLast )						 ( 
    
 /* [defaultvalue][in] */ long Options ) PURE;
	};// end interface;

// Interface: DAORecordsets
#undef INTERFACE
#define INTERFACE DAORecordsets
DECLARE_INTERFACE_(DAORecordsets, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAORecordset __RPC_FAR *__RPC_FAR *pprst ) PURE;
	};// end interface;

// Interface: _DAOField
#undef INTERFACE
#define INTERFACE _DAOField
DECLARE_INTERFACE_(_DAOField, _DAO)
{
	STDMETHOD( get_CollatingOrder )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_Type )						 ( 
    
 /* [in] */ short Type ) PURE;
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR Name ) PURE;
	STDMETHOD( get_Size )						 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Size )						 ( 
    
 /* [in] */ long Size ) PURE;
	STDMETHOD( get_SourceField )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_SourceTable )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Value )						 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( put_Value )						 ( 
    
 /* [in] */ VARIANT Val ) PURE;
	STDMETHOD( get_Attributes )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Attributes )					 ( 
    
 /* [in] */ long Attr ) PURE;
	STDMETHOD( get_OrdinalPosition )			 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_OrdinalPosition )			 ( 
    
 /* [in] */ short Pos ) PURE;
	STDMETHOD( get_ValidationText )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ValidationText )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_ValidateOnSet )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_ValidateOnSet )				 ( 
    
 /* [in] */ VARIANT_BOOL Validate ) PURE;
	STDMETHOD( get_ValidationRule )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ValidationRule )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_DefaultValue )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( put_DefaultValue )				 ( 
    
 /* [in] */ VARIANT var ) PURE;
	STDMETHOD( get_Required )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Required )					 ( 
    
 /* [in] */ VARIANT_BOOL fReq ) PURE;
	STDMETHOD( get_AllowZeroLength )			 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_AllowZeroLength )			 ( 
    
 /* [in] */ VARIANT_BOOL fAllow ) PURE;
	STDMETHOD( get_DataUpdatable )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_ForeignName )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ForeignName )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( AppendChunk )					 ( 
    
 /* [in] */ VARIANT Val ) PURE;
	STDMETHOD( GetChunk )						 ( 
    
 /* [in] */ long Offset,
 /* [in] */ long Bytes,
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( _30_FieldSize )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	STDMETHOD( get_CollectionIndex )			 ( 
    
 /* [retval][out] */ short __RPC_FAR *i ) PURE;
	STDMETHOD( get_OriginalValue )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_VisibleValue )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_FieldSize )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	};// end interface;

// Interface: DAOFields
#undef INTERFACE
#define INTERFACE DAOFields
DECLARE_INTERFACE_(DAOFields, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOField __RPC_FAR *__RPC_FAR *ppfld ) PURE;
	};// end interface;

// Interface: _DAOIndex
#undef INTERFACE
#define INTERFACE _DAOIndex
DECLARE_INTERFACE_(_DAOIndex, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Foreign )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( get_Unique )						 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Unique )						 ( 
    
 /* [in] */ VARIANT_BOOL fUnique ) PURE;
	STDMETHOD( get_Clustered )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Clustered )					 ( 
    
 /* [in] */ VARIANT_BOOL fClustered ) PURE;
	STDMETHOD( get_Required )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Required )					 ( 
    
 /* [in] */ VARIANT_BOOL fRequired ) PURE;
	STDMETHOD( get_IgnoreNulls )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_IgnoreNulls )				 ( 
    
 /* [in] */ VARIANT_BOOL fIgnoreNulls ) PURE;
	STDMETHOD( get_Primary )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Primary )					 ( 
    
 /* [in] */ VARIANT_BOOL fPrimary ) PURE;
	STDMETHOD( get_DistinctCount )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( get_Fields )						 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pv ) PURE;
	STDMETHOD( put_Fields )						 ( 
    
 /* [in] */ VARIANT v ) PURE;
	STDMETHOD( CreateField )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Size,
 /* [retval][out] */ DAOField __RPC_FAR *__RPC_FAR *ppfld ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	};// end interface;

// Interface: DAOIndexes
#undef INTERFACE
#define INTERFACE DAOIndexes
DECLARE_INTERFACE_(DAOIndexes, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOIndex __RPC_FAR *__RPC_FAR *ppidx ) PURE;
	};// end interface;

// Interface: DAOParameter
#undef INTERFACE
#define INTERFACE DAOParameter
DECLARE_INTERFACE_(DAOParameter, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Value )						 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( put_Value )						 ( 
    
 /* [in] */ VARIANT val ) PURE;
	STDMETHOD( get_Type )						 ( 
    
 /* [retval][out] */ short __RPC_FAR *ps ) PURE;
	STDMETHOD( put_Type )						 ( 
    
 /* [in] */ short s ) PURE;
	STDMETHOD( get_Direction )					 ( 
    
 /* [retval][out] */ short __RPC_FAR *pOption ) PURE;
	STDMETHOD( put_Direction )					 ( 
    
 /* [in] */ short Option ) PURE;
	};// end interface;

// Interface: DAOParameters
#undef INTERFACE
#define INTERFACE DAOParameters
DECLARE_INTERFACE_(DAOParameters, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOParameter __RPC_FAR *__RPC_FAR *ppprm ) PURE;
	};// end interface;

// Interface: _DAOUser
#undef INTERFACE
#define INTERFACE _DAOUser
DECLARE_INTERFACE_(_DAOUser, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( put_PID )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( put_Password )					 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Groups )						 ( 
    
 /* [retval][out] */ DAOGroups __RPC_FAR *__RPC_FAR *ppgrps ) PURE;
	STDMETHOD( NewPassword )					 ( 
    
 /* [in] */ BSTR bstrOld,
 /* [in] */ BSTR bstrNew ) PURE;
	STDMETHOD( CreateGroup )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT PID,
 /* [retval][out] */ DAOGroup __RPC_FAR *__RPC_FAR *ppgrp ) PURE;
	};// end interface;

// Interface: DAOUsers
#undef INTERFACE
#define INTERFACE DAOUsers
DECLARE_INTERFACE_(DAOUsers, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOUser __RPC_FAR *__RPC_FAR *ppusr ) PURE;
	};// end interface;

// Interface: _DAOGroup
#undef INTERFACE
#define INTERFACE _DAOGroup
DECLARE_INTERFACE_(_DAOGroup, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( put_PID )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Users )						 ( 
    
 /* [retval][out] */ DAOUsers __RPC_FAR *__RPC_FAR *ppusrs ) PURE;
	STDMETHOD( CreateUser )						 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT PID,
 /* [optional][in] */ VARIANT Password,
 /* [retval][out] */ DAOUser __RPC_FAR *__RPC_FAR *ppusr ) PURE;
	};// end interface;

// Interface: DAOGroups
#undef INTERFACE
#define INTERFACE DAOGroups
DECLARE_INTERFACE_(DAOGroups, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOGroup __RPC_FAR *__RPC_FAR *ppgrp ) PURE;
	};// end interface;

// Interface: _DAORelation
#undef INTERFACE
#define INTERFACE _DAORelation
DECLARE_INTERFACE_(_DAORelation, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Name )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Table )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Table )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_ForeignTable )				 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_ForeignTable )				 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Attributes )					 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Attributes )					 ( 
    
 /* [in] */ long attr ) PURE;
	STDMETHOD( get_Fields )						 ( 
    
 /* [retval][out] */ DAOFields __RPC_FAR *__RPC_FAR *ppflds ) PURE;
	STDMETHOD( CreateField )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Size,
 /* [retval][out] */ DAOField __RPC_FAR *__RPC_FAR *ppfld ) PURE;
	STDMETHOD( get_PartialReplica )				 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pfPartialReplica ) PURE;
	STDMETHOD( put_PartialReplica )				 ( 
    
 /* [in] */ VARIANT_BOOL fPartialReplica ) PURE;
	};// end interface;

// Interface: DAORelations
#undef INTERFACE
#define INTERFACE DAORelations
DECLARE_INTERFACE_(DAORelations, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAORelation __RPC_FAR *__RPC_FAR *pprel ) PURE;
	};// end interface;

// Interface: DAOContainer
#undef INTERFACE
#define INTERFACE DAOContainer
DECLARE_INTERFACE_(DAOContainer, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Owner )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Owner )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_UserName )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_UserName )					 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Permissions )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Permissions )				 ( 
    
 /* [in] */ long permissions ) PURE;
	STDMETHOD( get_Inherit )					 ( 
    
 /* [retval][out] */ VARIANT_BOOL __RPC_FAR *pb ) PURE;
	STDMETHOD( put_Inherit )					 ( 
    
 /* [in] */ VARIANT_BOOL fInherit ) PURE;
	STDMETHOD( get_Documents )					 ( 
    
 /* [retval][out] */ DAODocuments __RPC_FAR *__RPC_FAR *ppdocs ) PURE;
	STDMETHOD( get_AllPermissions )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	};// end interface;

// Interface: DAOContainers
#undef INTERFACE
#define INTERFACE DAOContainers
DECLARE_INTERFACE_(DAOContainers, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAOContainer __RPC_FAR *__RPC_FAR *ppctn ) PURE;
	};// end interface;

// Interface: DAODocument
#undef INTERFACE
#define INTERFACE DAODocument
DECLARE_INTERFACE_(DAODocument, _DAO)
{
	STDMETHOD( get_Name )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_Owner )						 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_Owner )						 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Container )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( get_UserName )					 ( 
    
 /* [retval][out] */ BSTR __RPC_FAR *pbstr ) PURE;
	STDMETHOD( put_UserName )					 ( 
    
 /* [in] */ BSTR bstr ) PURE;
	STDMETHOD( get_Permissions )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( put_Permissions )				 ( 
    
 /* [in] */ long permissions ) PURE;
	STDMETHOD( get_DateCreated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_LastUpdated )				 ( 
    
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	STDMETHOD( get_AllPermissions )				 ( 
    
 /* [retval][out] */ long __RPC_FAR *pl ) PURE;
	STDMETHOD( CreateProperty )					 ( 
    
 /* [optional][in] */ VARIANT Name,
 /* [optional][in] */ VARIANT Type,
 /* [optional][in] */ VARIANT Value,
 /* [optional][in] */ VARIANT DDL,
 /* [retval][out] */ DAOProperty __RPC_FAR *__RPC_FAR *pprp ) PURE;
	};// end interface;

// Interface: DAODocuments
#undef INTERFACE
#define INTERFACE DAODocuments
DECLARE_INTERFACE_(DAODocuments, _DAOCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [in] */ VARIANT Item,
 /* [retval][out] */ DAODocument __RPC_FAR *__RPC_FAR *ppdoc ) PURE;
	};// end interface;

// Interface: DAOIndexFields
#undef INTERFACE
#define INTERFACE DAOIndexFields
DECLARE_INTERFACE_(DAOIndexFields, _DAODynaCollection)
{
	STDMETHOD( get_Item )						 ( 
    
 /* [optional][in] */ VARIANT Item,
 /* [retval][out] */ VARIANT __RPC_FAR *pvar ) PURE;
	};// end interface;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _DBDAOINT_H_
