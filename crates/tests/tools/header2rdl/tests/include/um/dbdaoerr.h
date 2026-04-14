
/************************************************************************
**	D B D A O E R R . H													*														*
**																		*
**		History 														*
**		------- 														*
**	5-17-95 Added to DAO SDK				 							*
**	7-17-95 Added DBDAOERR macro, removed internal only codes																	*
**	8-30-96 Added replication and ODBC direct errors																	*
**																		*
**	The following #defines map the integer to a descriptive name
**	i.e.  3270 -> E_DAO_VtoPropNotFound									*
**																		*
**																		*
*************************************************************************
** Copyright (C) 1996 by Microsoft Corporation		 					*
**		   All Rights Reserved					 						*
************************************************************************/

#ifndef _DDAOERR_H_
#define _DDAOERR_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define DBDAOERR(x) MAKE_SCODE(SEVERITY_ERROR, FACILITY_CONTROL, x)

#define E_DAO_InternalError					DBDAOERR(3000) //Reserved error (|); there is no message for this error.
#define E_DAO_InvalidParameter				DBDAOERR(3001) //Invalid argument.
#define E_DAO_CantBegin						DBDAOERR(3002) //Couldn't start session.
#define E_DAO_TransTooDeep					DBDAOERR(3003) //Couldn't start transaction; too many transactions already nested.
#define E_DAO_DatabaseNotFound				DBDAOERR(3004) //Couldn't find database '|'.
#define E_DAO_DatabaseInvalidName			DBDAOERR(3005) //'|' isn't a valid database name.
#define E_DAO_DatabaseLocked				DBDAOERR(3006) //Database '|' is exclusively locked.
#define E_DAO_DatabaseOpenError				DBDAOERR(3007) //Can't open library database '|'.
#define E_DAO_TableLocked					DBDAOERR(3008) //Table '|' is exclusively locked.
#define E_DAO_TableInUse					DBDAOERR(3009) //Couldn't lock table '|'; currently in use.
#define E_DAO_TableDuplicate				DBDAOERR(3010) //Table '|' already exists.
#define E_DAO_ObjectNotFound				DBDAOERR(3011) //Couldn't find object '|'.
#define E_DAO_ObjectDuplicate				DBDAOERR(3012) //Object '|' already exists.
#define E_DAO_CannotRename					DBDAOERR(3013) //Couldn't rename installable ISAM file.
#define E_DAO_TooManyOpenTables				DBDAOERR(3014) //Can't open any more tables.
#define E_DAO_IndexNotFound					DBDAOERR(3015) //'|' isn't an index in this table.
#define E_DAO_ColumnDoesNotFit 				DBDAOERR(3016) //Field won't fit in record.
#define E_DAO_ColumnTooBig					DBDAOERR(3017) //The size of a field is too long.
#define E_DAO_ColumnNotFound				DBDAOERR(3018) //Couldn't find field '|'.
#define E_DAO_NoCurrentIndex				DBDAOERR(3019) //Operation invalid without a current index.
#define E_DAO_RecordNoCopy					DBDAOERR(3020) //Update or CancelUpdate without AddNew or Edit.
#define E_DAO_NoCurrentRecord				DBDAOERR(3021) //No current record.
#define E_DAO_KeyDuplicate					DBDAOERR(3022) //Duplicate value in index, primary key, or relationship.  Changes were unsuccessful.
#define E_DAO_AlreadyPrepared				DBDAOERR(3023) //AddNew or Edit already used.
#define E_DAO_FileNotFound					DBDAOERR(3024) //Couldn't find file '|'.
#define E_DAO_TooManyOpenFiles				DBDAOERR(3025) //Can't open any more files.
#define E_DAO_DiskFull						DBDAOERR(3026) //Not enough space on disk.
#define E_DAO_PermissionDenied				DBDAOERR(3027) //Can't update.  Database or object is read-only.
#define E_DAO_CannotOpenSystemDb			DBDAOERR(3028) //Can't start your application. The system database is missing or opened exclusively by another user.
#define E_DAO_InvalidLogon					DBDAOERR(3029) //Not a valid account name or password.
#define E_DAO_InvalidAccountName			DBDAOERR(3030) //'|' isn't a valid account name.
#define E_DAO_InvalidPassword				DBDAOERR(3031) //Not a valid password.
#define E_DAO_InvalidOperation				DBDAOERR(3032) //Can't perform this operation.
#define E_DAO_AccessDenied					DBDAOERR(3033) //No permission for '|'.
#define E_DAO_NotInTransaction				DBDAOERR(3034) //Commit or Rollback without BeginTrans.
#define E_DAO_OutOfMemory					DBDAOERR(3035) //*
#define E_DAO_CantAllocatePage				DBDAOERR(3036) //Database has reached maximum size.
#define E_DAO_NoMoreCursors					DBDAOERR(3037) //Can't open any more tables or queries.
#define E_DAO_OutOfBuffers					DBDAOERR(3038) //*
#define E_DAO_TooManyIndexes				DBDAOERR(3039) //Couldn't create index; too many indexes already defined.
#define E_DAO_ReadVerifyFailure				DBDAOERR(3040) //Disk I/O error during read.
#define E_DAO_FilesysVersion				DBDAOERR(3041) //Can't open a database created with a previous version of your application.
#define E_DAO_NoMoreFiles					DBDAOERR(3042) //Out of MS-DOS file handles.
#define E_DAO_DiskError						DBDAOERR(3043) //Disk or network error.
#define E_DAO_InvalidPath					DBDAOERR(3044) //'|' isn't a valid path.
#define E_DAO_FileShareViolation			DBDAOERR(3045) //Couldn't use '|'; file already in use.
#define E_DAO_FileLockViolation				DBDAOERR(3046) //Couldn't save; currently locked by another user.
#define E_DAO_RecordTooBig					DBDAOERR(3047) //Record is too large.
#define E_DAO_TooManyOpenDatabases			DBDAOERR(3048) //Can't open any more databases.
#define E_DAO_InvalidDatabase				DBDAOERR(3049) //Can't open database '|'.  It may not be a database that your application recognizes, or the file may be corrupt.
#define E_DAO_FileLockingUnavailable		DBDAOERR(3050) //Couldn't lock file.
#define E_DAO_FileAccessDenied				DBDAOERR(3051) //Couldn't open file '|'.
#define E_DAO_SharingBufferExceeded			DBDAOERR(3052) //MS-DOS file sharing lock count exceeded.  You need to increase the number of locks installed with SHARE.EXE.
#define E_DAO_TaskLimitExceeded				DBDAOERR(3053) //Too many client tasks.
#define E_DAO_TooManyLongColumns			DBDAOERR(3054) //Too many Memo or OLE object fields.
#define E_DAO_InvalidFilename				DBDAOERR(3055) //Not a valid file name.
#define E_DAO_AbortSalvage					DBDAOERR(3056) //Couldn't repair this database.
#define E_DAO_LinkNotSupported				DBDAOERR(3057) //Operation not supported on attached, or linked, tables.
#define E_DAO_NullKeyDisallowed				DBDAOERR(3058) //Index or primary key can't contain a null value.
#define E_DAO_OperationCanceled				DBDAOERR(3059) //Operation canceled by user.
#define E_DAO_QueryParmTypeMismatch			DBDAOERR(3060) //Wrong data type for parameter '|'.
#define E_DAO_QueryMissingParmsM			DBDAOERR(3061) //Too few parameters. Expected |.
#define E_DAO_QueryDuplicateAliasM			DBDAOERR(3062) //Duplicate output alias '|'.
#define E_DAO_QueryDuplicateOutputM			DBDAOERR(3063) //Duplicate output destination '|'.
#define E_DAO_QueryIsBulkOp					DBDAOERR(3064) //Can't open action query '|'.
#define E_DAO_QueryIsNotBulkOp				DBDAOERR(3065) //Can't execute a non-action query.
#define E_DAO_QueryNoOutputsM				DBDAOERR(3066) //Query or table must contain at least one output field.
#define E_DAO_QueryNoInputTablesM			DBDAOERR(3067) //Query input must contain at least one table or query.
#define E_DAO_QueryInvalidAlias				DBDAOERR(3068) //Not a valid alias name.
#define E_DAO_QueryInvalidBulkInputM		DBDAOERR(3069) //The action query '|' cannot be used as a row source.
#define E_DAO_QueryUnboundRef				DBDAOERR(3070) //Can't bind name '|'.
#define E_DAO_QueryExprEvaluation			DBDAOERR(3071) //Can't evaluate expression.
#define E_DAO_EvalEBESErr					DBDAOERR(3072) //|
#define E_DAO_QueryNotUpdatable				DBDAOERR(3073) //Operation must use an updatable query.
#define E_DAO_TableRepeatInFromList			DBDAOERR(3074) //Can't repeat table name '|' in FROM clause.
#define E_DAO_QueryExprSyntax				DBDAOERR(3075) //|1 in query expression '|2'.
#define E_DAO_QbeExprSyntax					DBDAOERR(3076) //| in criteria expression.
#define E_DAO_FindExprSyntax				DBDAOERR(3077) //| in expression.
#define E_DAO_InputTableNotFound			DBDAOERR(3078) //Couldn't find input table or query '|'.
#define E_DAO_QueryAmbigRefM				DBDAOERR(3079) //Ambiguous field reference '|'.
#define E_DAO_JoinTableNotInput				DBDAOERR(3080) //Joined table '|' not listed in FROM clause.
#define E_DAO_UnaliasedSelfJoin				DBDAOERR(3081) //Can't join more than one table with the same name (|).
#define E_DAO_ColumnNotInJoinTable			DBDAOERR(3082) //JOIN operation '|' refers to a non-joined table.
#define E_DAO_QueryIsMGB					DBDAOERR(3083) //Can't use internal report query.
#define E_DAO_QueryInsIntoBulkMGB			DBDAOERR(3084) //Can't insert data with action query.
#define E_DAO_ExprUnknownFunctionM			DBDAOERR(3085) //Undefined function '|' in expression.
#define E_DAO_QueryCannotDelete				DBDAOERR(3086) //Couldn't delete from specified tables.
#define E_DAO_QueryTooManyGroupExprs		DBDAOERR(3087) //Too many expressions in GROUP BY clause.
#define E_DAO_QueryTooManyOrderExprs		DBDAOERR(3088) //Too many expressions in ORDER BY clause.
#define E_DAO_QueryTooManyDistExprs			DBDAOERR(3089) //Too many expressions in DISTINCT output.
#define E_DAO_Column2ndSysMaint				DBDAOERR(3090) //Resultant table not allowed to have more than one Counter or Autonumber field.
#define E_DAO_HavingWOGrouping				DBDAOERR(3091) //HAVING clause (|) without grouping or aggregation.
#define E_DAO_HavingOnTransform				DBDAOERR(3092) //Can't use HAVING clause in TRANSFORM statement.
#define E_DAO_OrderVsDistinct				DBDAOERR(3093) //ORDER BY clause (|) conflicts with DISTINCT.
#define E_DAO_OrderVsGroup					DBDAOERR(3094) //ORDER BY clause (|) conflicts with GROUP BY clause.
#define E_DAO_AggregateInArgument			DBDAOERR(3095) //Can't have aggregate function in expression (|).
#define E_DAO_AggregateInWhere				DBDAOERR(3096) //Can't have aggregate function in WHERE clause (|).
#define E_DAO_AggregateInOrderBy			DBDAOERR(3097) //Can't have aggregate function in ORDER BY clause (|).
#define E_DAO_AggregateInGroupBy			DBDAOERR(3098) //Can't have aggregate function in GROUP BY clause (|).
#define E_DAO_AggregateInJoin				DBDAOERR(3099) //Can't have aggregate function in JOIN operation (|).
#define E_DAO_NullInJoinKey					DBDAOERR(3100) //Can't set field '|' in join key to Null.
#define E_DAO_ValueBreaksJoin				DBDAOERR(3101) //There is no record in table '|2' with key matching field(s) '|1'.
#define E_DAO_QueryTreeCycle				DBDAOERR(3102) //Circular reference caused by '|'.
#define E_DAO_OutputAliasCycle				DBDAOERR(3103) //Circular reference caused by alias '|' in query definition's SELECT list.
#define E_DAO_QryDuplicatedFixedSetM		DBDAOERR(3104) //Can't specify Fixed Column Heading '|' in a crosstab query more than once.
#define E_DAO_NoSelectIntoColumnName		DBDAOERR(3105) //Missing destination field name in SELECT INTO statement (|).
#define E_DAO_NoUpdateColumnName			DBDAOERR(3106) //Missing destination field name in UPDATE statement (|).
#define E_DAO_QueryNoInsertPerm				DBDAOERR(3107) //Record(s) can't be added; no Insert Data permission on '|'.
#define E_DAO_QueryNoReplacePerm			DBDAOERR(3108) //Record(s) can't be edited; no Update Data permission on '|'.
#define E_DAO_QueryNoDeletePerm				DBDAOERR(3109) //Record(s) can't be deleted; no Delete Data permission on '|'.
#define E_DAO_QueryNoReadDefPerm			DBDAOERR(3110) //Couldn't read definitions; no Read Design permission for table or query '|'.
#define E_DAO_QueryNoTblCrtPerm				DBDAOERR(3111) //Couldn't create; no Create permission for table or query '|'.
#define E_DAO_QueryNoReadPerm				DBDAOERR(3112) //Record(s) can't be read; no Read Data permission on '|'.
#define E_DAO_QueryColNotUpd				DBDAOERR(3113) //Can't update '|'; field not updatable.
#define E_DAO_QueryLVInDistinct				DBDAOERR(3114) //Can't include Memo or OLE object when you select unique values (|).
#define E_DAO_QueryLVInAggregate			DBDAOERR(3115) //Can't have Memo or OLE object in aggregate argument (|).
#define E_DAO_QueryLVInHaving				DBDAOERR(3116) //Can't have Memo or OLE object in criteria (|) for aggregate function.
#define E_DAO_QueryLVInOrderBy				DBDAOERR(3117) //Can't sort on Memo or OLE object (|).
#define E_DAO_QueryLVInJoin					DBDAOERR(3118) //Can't join on Memo or OLE object (|).
#define E_DAO_QueryLVInGroupBy				DBDAOERR(3119) //Can't group on Memo or OLE object (|).
#define E_DAO_DotStarWithGrouping			DBDAOERR(3120) //Can't group on fields selected with '*' (|).
#define E_DAO_StarWithGrouping				DBDAOERR(3121) //Can't group on fields selected with '*'.
#define E_DAO_IllegalDetailRef				DBDAOERR(3122) //'|' not part of aggregate function or grouping.
#define E_DAO_StarNotAtLevel0				DBDAOERR(3123) //Can't use '*' in crosstab query.
#define E_DAO_QueryInvalidMGBInput			DBDAOERR(3124) //Can't input from internal report query (|).
#define E_DAO_InvalidName					DBDAOERR(3125) //'|' isn't a valid name.
#define E_DAO_QueryBadBracketing			DBDAOERR(3126) //Invalid bracketing of name '|'.
#define E_DAO_InsertIntoUnknownCol			DBDAOERR(3127) //INSERT INTO statement contains unknown field name '|'.
#define E_DAO_QueryNoDeleteTables			DBDAOERR(3128) //Must specify tables to delete from.
#define E_DAO_SQLSyntax						DBDAOERR(3129) //Invalid SQL statement; expected 'DELETE', 'INSERT', 'PROCEDURE', 'SELECT', or 'UPDATE'.
#define E_DAO_SQLDeleteSyntax				DBDAOERR(3130) //Syntax error in DELETE statement.
#define E_DAO_SQLFromSyntax					DBDAOERR(3131) //Syntax error in FROM clause.
#define E_DAO_SQLGroupBySyntax				DBDAOERR(3132) //Syntax error in GROUP BY clause.
#define E_DAO_SQLHavingSyntax				DBDAOERR(3133) //Syntax error in HAVING clause.
#define E_DAO_SQLInsertSyntax				DBDAOERR(3134) //Syntax error in INSERT statement.
#define E_DAO_SQLJoinSyntax					DBDAOERR(3135) //Syntax error in JOIN operation.
#define E_DAO_SQLLevelSyntax				DBDAOERR(3136) //Syntax error in LEVEL clause.
#define E_DAO_SQLMissingSemicolon			DBDAOERR(3137) //Missing semicolon (;) at end of SQL statement.
#define E_DAO_SQLOrderBySyntax				DBDAOERR(3138) //Syntax error in ORDER BY clause.
#define E_DAO_SQLParameterSyntax			DBDAOERR(3139) //Syntax error in PARAMETER clause.
#define E_DAO_SQLProcedureSyntax			DBDAOERR(3140) //Syntax error in PROCEDURE clause.
#define E_DAO_SQLSelectSyntax				DBDAOERR(3141) //Syntax error in SELECT statement.
#define E_DAO_SQLTooManyTokens				DBDAOERR(3142) //Characters found after end of SQL statement.
#define E_DAO_SQLTransformSyntax			DBDAOERR(3143) //Syntax error in TRANSFORM statement.
#define E_DAO_SQLUpdateSyntax				DBDAOERR(3144) //Syntax error in UPDATE statement.
#define E_DAO_SQLWhereSyntax				DBDAOERR(3145) //Syntax error in WHERE clause.
#define E_DAO_RmtSQLCError					DBDAOERR(3146) //ODBC--call failed.
#define E_DAO_RmtDataOverflow				DBDAOERR(3147) //*
#define E_DAO_RmtConnectFailed				DBDAOERR(3148) //*
#define E_DAO_RmtIncorrectSqlcDll			DBDAOERR(3149) //*
#define E_DAO_RmtMissingSqlcDll				DBDAOERR(3150) //*
#define E_DAO_RmtConnectFailedM				DBDAOERR(3151) //ODBC--connection to '|' failed.
#define E_DAO_RmtDrvrVer					DBDAOERR(3152) //*
#define E_DAO_RmtSrvrVer					DBDAOERR(3153) //*
#define E_DAO_RmtMissingOdbcDll				DBDAOERR(3154) //ODBC--couldn't find DLL '|'.
#define E_DAO_RmtInsertFailedM				DBDAOERR(3155) //ODBC--insert failed on attached (linked) table '|'.
#define E_DAO_RmtDeleteFailedM				DBDAOERR(3156) //ODBC--delete failed on attached (linked) table '|'.
#define E_DAO_RmtUpdateFailedM				DBDAOERR(3157) //ODBC--update failed on attached (linked) table '|'.
#define E_DAO_RecordLocked					DBDAOERR(3158) //Couldn't save record; currently locked by another user.
#define E_DAO_InvalidBookmark				DBDAOERR(3159) //Not a valid bookmark.
#define E_DAO_TableNotOpen					DBDAOERR(3160) //Table isn't open.
#define E_DAO_DecryptFail					DBDAOERR(3161) //Couldn't decrypt file.
#define E_DAO_NullInvalid					DBDAOERR(3162) //Null is invalid.
#define E_DAO_InvalidBufferSize				DBDAOERR(3163) //Couldn't perform operation; data too long for field.
#define E_DAO_ColumnNotUpdatable			DBDAOERR(3164) //Field can't be updated.
#define E_DAO_CantMakeINFFile				DBDAOERR(3165) //Couldn't open .INF file.
#define E_DAO_MissingMemoFile				DBDAOERR(3166) //Missing memo file.
#define E_DAO_RecordDeleted					DBDAOERR(3167) //Record is deleted.
#define E_DAO_INFFileError					DBDAOERR(3168) //Invalid .INF file.
#define E_DAO_ExprIllegalType				DBDAOERR(3169) //Illegal type in expression.
#define E_DAO_InstalIsamNotFound			DBDAOERR(3170) //Couldn't find installable ISAM.
#define E_DAO_NoConfigParameters			DBDAOERR(3171) //Couldn't find net path or user name.
#define E_DAO_CantAccessPdoxNetDir			DBDAOERR(3172) //Couldn't open PARADOX.NET.
#define E_DAO_NoMSysAccounts				DBDAOERR(3173) //Couldn't open table 'MSysAccounts' in the system database file.
#define E_DAO_NoMSysGroups					DBDAOERR(3174) //Couldn't open table 'MSysGroups' in the system database file.
#define E_DAO_DateOutOfRange				DBDAOERR(3175) //Date is out of range or is in an invalid format.
#define E_DAO_ImexCantOpenFile				DBDAOERR(3176) //Couldn't open file '|'.
#define E_DAO_ImexBadTableName				DBDAOERR(3177) //Not a valid table name.
#define E_DAO_ImexOutOfMemory				DBDAOERR(3178) //*
#define E_DAO_ImexEndofFile					DBDAOERR(3179) //Encountered unexpected end of file.
#define E_DAO_ImexCantWriteToFile			DBDAOERR(3180) //Couldn't write to file '|'.
#define E_DAO_ImexBadRange					DBDAOERR(3181) //Invalid range.
#define E_DAO_ImexBogusFile					DBDAOERR(3182) //Invalid file format.
#define E_DAO_TempDiskFull					DBDAOERR(3183) //Not enough space on temporary disk.
#define E_DAO_RmtLinkNotFound				DBDAOERR(3184) //Couldn't execute query; couldn't find attached, or linked, table.
#define E_DAO_RmtTooManyColumns				DBDAOERR(3185) //SELECT INTO remote database tried to produce too many fields.
#define E_DAO_ReadConflictM					DBDAOERR(3186) //Couldn't save; currently locked by user '|2' on machine '|1'.
#define E_DAO_CommitConflictM				DBDAOERR(3187) //Couldn't read; currently locked by user '|2' on machine '|1'.
#define E_DAO_SessionWriteConflict			DBDAOERR(3188) //Couldn't update; currently locked by another session on this machine.
#define E_DAO_JetSpecialTableLocked			DBDAOERR(3189) //Table '|1' is exclusively locked by user '|3' on machine '|2'.
#define E_DAO_TooManyColumns				DBDAOERR(3190) //Too many fields defined.
#define E_DAO_ColumnDuplicate				DBDAOERR(3191) //Can't define field more than once.
#define E_DAO_OutputTableNotFound			DBDAOERR(3192) //Couldn't find output table '|'.
#define E_DAO_JetNoUserName					DBDAOERR(3193) //(unknown)
#define E_DAO_JetNoMachineName				DBDAOERR(3194) //(unknown)
#define E_DAO_JetNoColumnName				DBDAOERR(3195) //(expression)
#define E_DAO_DatabaseInUse					DBDAOERR(3196) //Couldn't use '|'; database already in use.
#define E_DAO_DataHasChanged				DBDAOERR(3197) //Data has changed; operation stopped.
#define E_DAO_TooManySessions				DBDAOERR(3198) //Couldn't start session.  Too many sessions already active.
#define E_DAO_ReferenceNotFound				DBDAOERR(3199) //Couldn't find reference.
#define E_DAO_IntegrityViolMasterM			DBDAOERR(3200) //Can't delete or change record.  Since related records exist in table '|', referential integrity rules would be violated.
#define E_DAO_IntegrityViolSlaveM			DBDAOERR(3201) //Can't add or change record.  Referential integrity rules require a related record in table '|'.
#define E_DAO_ReadConflict					DBDAOERR(3202) //Couldn't save; currently locked by another user.
#define E_DAO_AggregatingHigherLevel		DBDAOERR(3203) //Can't specify subquery in expression (|).
#define E_DAO_DatabaseDuplicate				DBDAOERR(3204) //Database already exists.
#define E_DAO_QueryTooManyXvtColumn			DBDAOERR(3205) //Too many crosstab column headers (|).
#define E_DAO_SelfReference					DBDAOERR(3206) //Can't create a relationship between a field and itself.
#define E_DAO_CantUseUnkeyedTable			DBDAOERR(3207) //Operation not supported on Paradox table with no primary key.
#define E_DAO_IllegalDeletedOption			DBDAOERR(3208) //Invalid Deleted entry in the Xbase section of initialization setting.
#define E_DAO_IllegalStatsOption			DBDAOERR(3209) //Invalid Stats entry in the Xbase section of initialization setting.
#define E_DAO_ConnStrTooLong				DBDAOERR(3210) //Connection string too long.
#define E_DAO_TableInUseQM					DBDAOERR(3211) //Couldn't lock table '|'; currently in use.
#define E_DAO_JetSpecialTableInUse			DBDAOERR(3212) //Couldn't lock table '|1'; currently in use by user '|3' on machine '|2'.
#define E_DAO_IllegalDateOption				DBDAOERR(3213) //Invalid Date entry in the Xbase section of initialization setting.
#define E_DAO_IllegalMarkOption				DBDAOERR(3214) //Invalid Mark entry in the Xbase section of initialization setting.
#define E_DAO_BtrieveTooManyTasks			DBDAOERR(3215) //Too many Btrieve tasks.
#define E_DAO_QueryParmNotTableid			DBDAOERR(3216) //Parameter '|' specified where a table name is required.
#define E_DAO_QueryParmNotDatabase			DBDAOERR(3217) //Parameter '|' specified where a database name is required.
#define E_DAO_WriteConflict					DBDAOERR(3218) //Couldn't update; currently locked.
#define E_DAO_IllegalOperation				DBDAOERR(3219) //Invalid operation.
#define E_DAO_WrongCollatingSequence		DBDAOERR(3220) //Incorrect collating sequence.
#define E_DAO_BadConfigParameters			DBDAOERR(3221) //Invalid entries in the Btrieve section of initialization setting.
#define E_DAO_QueryContainsDbParm			DBDAOERR(3222) //Query can't contain a Database parameter.
#define E_DAO_QueryInvalidParmM				DBDAOERR(3223) //'|' isn't a valid parameter name.
#define E_DAO_BtrieveDDCorrupted			DBDAOERR(3224) //Can't read Btrieve data dictionary.
#define E_DAO_BtrieveDeadlock				DBDAOERR(3225) //Encountered record locking deadlock while performing Btrieve operation.
#define E_DAO_BtrieveFailure				DBDAOERR(3226) //Errors encountered while using the Btrieve DLL.
#define E_DAO_IllegalCenturyOption			DBDAOERR(3227) //Invalid Century entry in the Xbase section of initialization setting.
#define E_DAO_IllegalCollatingSeq			DBDAOERR(3228) //Invalid Collating Sequence.
#define E_DAO_NonModifiableKey				DBDAOERR(3229) //Btrieve--can't change field.
#define E_DAO_ObsoleteLockFile				DBDAOERR(3230) //Out-of-date Paradox lock file.
#define E_DAO_RmtColDataTruncated			DBDAOERR(3231) //ODBC--field would be too long; data truncated.
#define E_DAO_RmtCreateTableFailed			DBDAOERR(3232) //ODBC--couldn't create table.
#define E_DAO_RmtOdbcVer					DBDAOERR(3233) //*
#define E_DAO_RmtQueryTimeout				DBDAOERR(3234) //ODBC--remote query timeout expired.
#define E_DAO_RmtTypeIncompat				DBDAOERR(3235) //ODBC--data type not supported on server.
#define E_DAO_RmtUnexpectedNull				DBDAOERR(3236) //*
#define E_DAO_RmtUnexpectedType				DBDAOERR(3237) //*
#define E_DAO_RmtValueOutOfRange			DBDAOERR(3238) //ODBC--data out of range.
#define E_DAO_TooManyActiveUsers			DBDAOERR(3239) //Too many active users.
#define E_DAO_CantStartBtrieve				DBDAOERR(3240) //Btrieve--missing Btrieve engine.
#define E_DAO_OutOfBVResources				DBDAOERR(3241) //Btrieve--out of resources.
#define E_DAO_QueryBadUpwardRefedM			DBDAOERR(3242) //Invalid reference in SELECT statement.
#define E_DAO_ImexNoMatchingColumns			DBDAOERR(3243) //None of the import field names match fields in the appended table.
#define E_DAO_ImexPasswordProtected			DBDAOERR(3244) //Can't import password-protected spreadsheet.
#define E_DAO_ImexUnparsableRecord			DBDAOERR(3245) //Couldn't parse field names from first row of import table.
#define E_DAO_InTransaction					DBDAOERR(3246) //Operation not supported in transactions.
#define E_DAO_RmtLinkOutOfSync				DBDAOERR(3247) //ODBC--linked table definition has changed.
#define E_DAO_IllegalNetworkOption			DBDAOERR(3248) //Invalid NetworkAccess entry in initialization setting.
#define E_DAO_IllegalTimeoutOption			DBDAOERR(3249) //Invalid PageTimeout entry in initialization setting.
#define E_DAO_CantBuildKey					DBDAOERR(3250) //Couldn't build key.
#define E_DAO_FeatureNotAvailable			DBDAOERR(3251) //Operation is not supported for this type of object.
#define E_DAO_IllegalReentrancy				DBDAOERR(3252) //Can't open form whose underlying query contains a user-defined function that attempts to set or get the form's RecordsetClone property.
#define E_DAO_UNUSED						DBDAOERR(3253) //*
#define E_DAO_RmtDenyWriteIsInvalid			DBDAOERR(3254) //ODBC--Can't lock all records.
#define E_DAO_ODBCParmsChanged				DBDAOERR(3255) //*
#define E_DAO_INFIndexNotFound 				DBDAOERR(3256) //Index file not found.
#define E_DAO_SQLOwnerAccessSyntax			DBDAOERR(3257) //Syntax error in WITH OWNERACCESS OPTION declaration.
#define E_DAO_QueryAmbiguousJoins			DBDAOERR(3258) //Query contains ambiguous outer joins.
#define E_DAO_InvalidColumnType				DBDAOERR(3259) //Invalid field data type.
#define E_DAO_WriteConflictM				DBDAOERR(3260) //Couldn't update; currently locked by user '|2' on machine '|1'.
#define E_DAO_TableLockedM					DBDAOERR(3261) //|
#define E_DAO_TableInUseMUQM				DBDAOERR(3262) //|
#define E_DAO_InvalidTableId				DBDAOERR(3263) //Invalid database object.
#define E_DAO_VtoNoFields					DBDAOERR(3264) //No fields defined - cannot append Tabledef or Index.
#define E_DAO_VtoNameNotFound				DBDAOERR(3265) //Item not found in this collection.
#define E_DAO_VtoFieldInCollection			DBDAOERR(3266) //Can't append.  Field is part of a TableDefs collection.
#define E_DAO_VtoNotARecordset				DBDAOERR(3267) //Property can be set only when the field is part of a Recordset object's Fields collection.
#define E_DAO_VtoNoSetObjInDb				DBDAOERR(3268) //Can't set this property once the object is part of a collection.
#define E_DAO_VtoIndexInCollection			DBDAOERR(3269) //Can't append.  Index is part of a TableDefs collection.
#define E_DAO_VtoPropNotFound				DBDAOERR(3270) //Property not found.
#define E_DAO_VtoIllegalValue				DBDAOERR(3271) //Invalid property value.
#define E_DAO_VtoNotArray					DBDAOERR(3272) //Object isn't a collection.
#define E_DAO_VtoNoSuchMethod				DBDAOERR(3273) //Method not applicable for this object.
#define E_DAO_NotExternalFormat				DBDAOERR(3274) //External table isn't in the expected format.
#define E_DAO_UnexpectedEngineReturn		DBDAOERR(3275) //Unexpected error from external database driver (|).
#define E_DAO_InvalidDatabaseId				DBDAOERR(3276) //Invalid database ID.
#define E_DAO_TooManyKeys					DBDAOERR(3277) //Can't have more than 10 fields in an index.
#define E_DAO_NotInitialized				DBDAOERR(3278) //Database engine hasn't been initialized.
#define E_DAO_AlreadyInitialized			DBDAOERR(3279) //Database engine has already been initialized.
#define E_DAO_ColumnInUse					DBDAOERR(3280) //Can't delete a field that is part of an index or is needed by the system.
#define E_DAO_IndexInUse					DBDAOERR(3281) //Can't delete this index.  It is either the current index or is used in a relationship.
#define E_DAO_TableNotEmpty					DBDAOERR(3282) //Can't create field or index in a table that is already defined.
#define E_DAO_IndexHasPrimary				DBDAOERR(3283) //Primary key already exists.
#define E_DAO_IndexDuplicate				DBDAOERR(3284) //Index already exists.
#define E_DAO_IndexInvalidDef				DBDAOERR(3285) //Invalid index definition.
#define E_DAO_WrongMemoFileType				DBDAOERR(3286) //Format of memo file doesn't match specified external database format.
#define E_DAO_ColumnCannotIndex				DBDAOERR(3287) //Can't create index on the given field.
#define E_DAO_IndexHasNoPrimary				DBDAOERR(3288) //Paradox index is not primary.
#define E_DAO_DDLConstraintSyntax			DBDAOERR(3289) //Syntax error in CONSTRAINT clause.
#define E_DAO_DDLCreateTableSyntax			DBDAOERR(3290) //Syntax error in CREATE TABLE statement.
#define E_DAO_DDLCreateIndexSyntax			DBDAOERR(3291) //Syntax error in CREATE INDEX statement.
#define E_DAO_DDLColumnDefSyntax			DBDAOERR(3292) //Syntax error in field definition.
#define E_DAO_DDLAlterTableSyntax			DBDAOERR(3293) //Syntax error in ALTER TABLE statement.
#define E_DAO_DDLDropIndexSyntax			DBDAOERR(3294) //Syntax error in DROP INDEX statement.
#define E_DAO_DDLDropSyntax					DBDAOERR(3295) //Syntax error in DROP TABLE or DROP INDEX.
#define E_DAO_V11NotSupported				DBDAOERR(3296) //Join expression not supported.
#define E_DAO_ImexNothingToImport			DBDAOERR(3297) //Couldn't import table or query.  No records found, or all records contain errors.
#define E_DAO_RmtTableAmbiguous				DBDAOERR(3298) //There are several tables with that name.  Please specify owner in the format 'owner.table'.
#define E_DAO_JetODBCConformanceError		DBDAOERR(3299) //ODBC Specification Conformance Error (|).  This error should be reported to the ODBC driver vendor.
#define E_DAO_IllegalRelationship			DBDAOERR(3300) //Can't create a relationship.
#define E_DAO_DBVerFeatureNotAvailable		DBDAOERR(3301) //Can't perform this operation; features in this version are not available in databases with older formats.
#define E_DAO_RulesLoaded					DBDAOERR(3302) //Can't change a rule while the rules for this table are in use.
#define E_DAO_ColumnInRelationship			DBDAOERR(3303) //Can't delete this field.  It's part of one or more relationships.
#define E_DAO_InvalidPin					DBDAOERR(3304) //You must enter a personal identifier (PID) consisting of at least four and no more than 20 characters and digits.
#define E_DAO_RmtBogusConnStr				DBDAOERR(3305) //Invalid connection string in pass-through query.
#define E_DAO_SingleColumnExpected			DBDAOERR(3306) //At most one field can be returned from a subquery that doesn't use the EXISTS keyword.
#define E_DAO_ColumnCountMismatch			DBDAOERR(3307) //The number of columns in the two selected tables or queries of a union query don't match.
#define E_DAO_InvalidTopArgumentM			DBDAOERR(3308) //Invalid TOP argument in select query.
#define E_DAO_PropertyTooLarge				DBDAOERR(3309) //Property setting can't be larger than 2 KB.
#define E_DAO_JPMInvalidForV1x				DBDAOERR(3310) //This property isn't supported for external data sources or for databases created in a previous version.
#define E_DAO_PropertyExists				DBDAOERR(3311) //Property specified already exists.
#define E_DAO_TLVNativeUserTablesOnly		DBDAOERR(3312) //Validation rules and default values can't be placed on system or attached (linked) tables.
#define E_DAO_TLVInvalidColumn				DBDAOERR(3313) //Can't place this validation expression on this field.
#define E_DAO_TLVNoNullM					DBDAOERR(3314) //Field '|' can't contain a null value.
#define E_DAO_TLVNoBlankM					DBDAOERR(3315) //Field '|' can't be a zero-length string.
#define E_DAO_TLVRuleViolationM				DBDAOERR(3316) //|
#define E_DAO_TLVRuleVioNoMessage			DBDAOERR(3317) //One or more values entered is prohibited by the validation rule '|2' set for '|1'.
#define E_DAO_QueryTopNotAllowedM			DBDAOERR(3318) //Top not allowed in delete queries.
#define E_DAO_SQLUnionSyntax				DBDAOERR(3319) //Syntax error in union query.
#define E_DAO_TLVExprSyntaxM				DBDAOERR(3320) //| in table-level validation expression.
#define E_DAO_NoDbInConnStr					DBDAOERR(3321) //No database specified in connection string or IN clause.
#define E_DAO_QueryBadValueListM			DBDAOERR(3322) //Crosstab query contains one or more invalid fixed column headings.
#define E_DAO_QueryIsNotRowReturning		DBDAOERR(3323) //The query can not be used as a row source.
#define E_DAO_QueryIsDDL					DBDAOERR(3324) //This query is a DDL query and cannot be used as a row source.
#define E_DAO_SPTReturnedNoRecords			DBDAOERR(3325) //Pass-through query with ReturnsRecords property set to True did not return any records.
#define E_DAO_QueryIsSnapshot				DBDAOERR(3326) //This Recordset is not updatable.
#define E_DAO_QueryExprOutput				DBDAOERR(3327) //Field '|' is based on an expression and can't be edited.
#define E_DAO_QueryTableRO					DBDAOERR(3328) //Table '|2' is read-only.
#define E_DAO_QueryRowDeleted				DBDAOERR(3329) //Record in table '|' was deleted by another user.
#define E_DAO_QueryRowLocked				DBDAOERR(3330) //Record in table '|' is locked by another user.
#define E_DAO_QueryFixupChanged				DBDAOERR(3331) //To make changes to this field, first save the record.
#define E_DAO_QueryCantFillIn				DBDAOERR(3332) //Can't enter value into blank field on 'one' side of outer join.
#define E_DAO_QueryWouldOrphan				DBDAOERR(3333) //Records in table '|' would have no record on the 'one' side.
#define E_DAO_V10Format						DBDAOERR(3334) //Can be present only in version 1.0 format.
#define E_DAO_InvalidDelete					DBDAOERR(3335) //DeleteOnly called with non-zero cbData.
#define E_DAO_IllegalIndexDDFOption			DBDAOERR(3336) //Btrieve: Invalid IndexDDF option in initialization setting.
#define E_DAO_IllegalDataCodePage			DBDAOERR(3337) //Invalid DataCodePage option in initialization setting.
#define E_DAO_XtrieveEnvironmentError		DBDAOERR(3338) //Btrieve: Xtrieve options aren't correct in initialization setting.
#define E_DAO_IllegalIndexNumberOption		DBDAOERR(3339) //Btrieve: Invalid IndexDeleteRenumber option in initialization setting.
#define E_DAO_QueryIsCorruptM				DBDAOERR(3340) //Query '|' is corrupt.
#define E_DAO_IncorrectJoinKeyM				DBDAOERR(3341) //Current field must match join key '|' on 'one' side of outer join because it has been updated.
#define E_DAO_QueryLVInSubqueryM			DBDAOERR(3342) //Invalid Memo or OLE object in subquery '|'.
#define E_DAO_InvalidDatabaseM				DBDAOERR(3343) //Unrecognized database format '|'.
#define E_DAO_TLVCouldNotBindRef			DBDAOERR(3344) //Unknown or invalid reference '|1' in validation expression or default value in table '|2'.
#define E_DAO_CouldNotBindRef				DBDAOERR(3345) //Unknown or invalid field reference '|'.
#define E_DAO_QueryWrongNumDestCol			DBDAOERR(3346) //Number of query values and destination fields aren't the same.
#define E_DAO_QueryPKeyNotOutput			DBDAOERR(3347) //Can't add record(s); primary key for table '|' not in recordset.
#define E_DAO_QueryJKeyNotOutput			DBDAOERR(3348) //Can't add record(s); join key of table '|' not in recordset.
#define E_DAO_NumericFieldOverflow			DBDAOERR(3349) //Numeric field overflow.
#define E_DAO_InvalidObject					DBDAOERR(3350) //Object is invalid for operation.
#define E_DAO_OrderVsUnion					DBDAOERR(3351) //ORDER BY expression (|) uses non-output fields.
#define E_DAO_NoInsertColumnNameM			DBDAOERR(3352) //No destination field name in INSERT INTO statement (|).
#define E_DAO_MissingDDFFile				DBDAOERR(3353) //Btrieve: Can't find file FIELD.DDF.
#define E_DAO_SingleRecordExpected			DBDAOERR(3354) //At most one record can be returned by this subquery.
#define E_DAO_DefaultExprSyntax				DBDAOERR(3355) //Syntax error in default value.
#define E_DAO_ExclusiveDBConflict			DBDAOERR(3356) //The database is opened by user '|2' on machine '|1'.
#define E_DAO_QueryIsNotDDL					DBDAOERR(3357) //This query is not a properly formed data-definition query.
#define E_DAO_SysDatabaseOpenError			DBDAOERR(3358) //Can't open Microsoft Jet engine system database.
#define E_DAO_SQLInvalidSPT					DBDAOERR(3359) //Pass-through query must contain at least one character.
#define E_DAO_QueryTooComplex				DBDAOERR(3360) //Query is too complex.
#define E_DAO_SetOpInvalidInSubquery		DBDAOERR(3361) //Unions not allowed in a subquery.
#define E_DAO_RmtMultiRowUpdate				DBDAOERR(3362) //Single-row update/delete affected more than one row of an attached (linked) table.  Unique index contains duplicate values.
#define E_DAO_QueryNoJoinedRecord			DBDAOERR(3363) //Record(s) can't be added; no corresponding record on the 'one' side.
#define E_DAO_QueryLVInSetOp				DBDAOERR(3364) //Can't use Memo or OLE object field '|' in SELECT clause of a union query.
#define E_DAO_VtoInvalidOnRemote			DBDAOERR(3365) //Property value not valid for REMOTE objects.
#define E_DAO_VtoNoFieldsRel				DBDAOERR(3366) //Can't append a relation with no fields defined.
#define E_DAO_VtoObjectInCollection			DBDAOERR(3367) //Can't append.  Object already in collection.
#define E_DAO_DDLDiffNumRelCols				DBDAOERR(3368) //Relationship must be on the same number of fields with the same data types.
#define E_DAO_DDLIndexColNotFound			DBDAOERR(3369) //Can't find field in index definition.
#define E_DAO_DDLPermissionDenied			DBDAOERR(3370) //Can't modify the design of table '|'.  It's in a read-only database.
#define E_DAO_DDLObjectNotFound				DBDAOERR(3371) //Can't find table or constraint.
#define E_DAO_DDLIndexNotFound				DBDAOERR(3372) //No such index '|2' on table '|1'.
#define E_DAO_DDLNoPkeyOnRefdTable			DBDAOERR(3373) //Can't create relationship.  Referenced table '|' doesn't have a primary key.
#define E_DAO_DDLColumnsNotUnique			DBDAOERR(3374) //The specified fields are not uniquely indexed in table '|'.
#define E_DAO_DDLIndexDuplicate				DBDAOERR(3375) //Table '|1' already has an index named '|2'
#define E_DAO_DDLTableNotFound				DBDAOERR(3376) //Table '|' doesn't exist.
#define E_DAO_DDLRelNotFound				DBDAOERR(3377) //No such relationship '|2' on table '|1'.
#define E_DAO_DDLRelDuplicate				DBDAOERR(3378) //There is already a relationship named '|' in the current database.
#define E_DAO_DDLIntegrityViolation			DBDAOERR(3379) //Can't create relationships to enforce referential integrity.  Existing data in table '|2' violates referential integrity rules with related table '|1'.
#define E_DAO_DDLColumnDuplicate			DBDAOERR(3380) //Field '|2' already exists in table '|1'.
#define E_DAO_DDLColumnNotFound				DBDAOERR(3381) //There is no field named '|2' in table '|1'.
#define E_DAO_DDLColumnTooBig				DBDAOERR(3382) //The size of field '|' is too long.
#define E_DAO_DDLColumnInRel				DBDAOERR(3383) //Can't delete field '|'.  It's part of one or more relationships.
#define E_DAO_VtoCantDeleteBuiltIn			DBDAOERR(3384) //Can't delete a built-in property.
#define E_DAO_VtoUDPsDontSupportNull		DBDAOERR(3385) //User-defined properties don't support a Null value.
#define E_DAO_VtoMissingRequiredParm		DBDAOERR(3386) //Property '|' must be set before using this method.
#define E_DAO_JetJetInitInvalidPath			DBDAOERR(3387) //Can't find TEMP directory.
#define E_DAO_TLVExprUnknownFunctionM		DBDAOERR(3388) //Unknown function '|2' in validation expression or default value on '|1'.
#define E_DAO_QueryNotSupported				DBDAOERR(3389) //Query support unavailable.
#define E_DAO_AccountDuplicate				DBDAOERR(3390) //Account name already exists.
#define E_DAO_JetwrnPropCouldNotSave		DBDAOERR(3391) //An error has occurred.  Properties were not saved.
#define E_DAO_RelNoPrimaryIndexM			DBDAOERR(3392) //There is no primary key in table '|'.
#define E_DAO_QueryKeyTooBig				DBDAOERR(3393) //Can't perform join, group, sort, or indexed restriction. A value being searched or sorted on is too long.
#define E_DAO_PropMustBeDDL					DBDAOERR(3394) //Can't save property; property is a schema property.
#define E_DAO_IllegalRIConstraint			DBDAOERR(3395) //Invalid referential integrity constraint.
#define E_DAO_RIViolationMasterCM			DBDAOERR(3396) //Can't perform cascading operation.  Since related records exist in table '|', referential integrity rules would be violated.
#define E_DAO_RIViolationSlaveCM			DBDAOERR(3397) //Can't perform cascading operation.  There must be a related record in table '|'.
#define E_DAO_RIKeyNullDisallowedCM			DBDAOERR(3398) //Can't perform cascading operation.  It would result in a null key in table '|'.
#define E_DAO_RIKeyDuplicateCM				DBDAOERR(3399) //Can't perform cascading operation.  It would result in a duplicate key in table '|'.
#define E_DAO_RIUpdateTwiceCM				DBDAOERR(3400) //Can't perform cascading operation.  It would result in two updates on field '|2' in table '|1'.
#define E_DAO_RITLVNoNullCM					DBDAOERR(3401) //Can't perform cascading operation.  It would cause field '|' to become null, which is not allowed.
#define E_DAO_RITLVNoBlankCM				DBDAOERR(3402) //Can't perform cascading operation.  It would cause field '|' to become a zero-length string, which is not allowed.
#define E_DAO_RITLVRuleViolationCM			DBDAOERR(3403) //Can't perform cascading operation:  '|'
#define E_DAO_RITLVRuleVioCNoMessage		DBDAOERR(3404) //Can't perform cascading operation.  The value entered is prohibited by the validation rule '|2' set for '|1'.
#define E_DAO_TLVRuleEvalEBESErr			DBDAOERR(3405) //Error '|' in validation rule.
#define E_DAO_TLVDefaultEvalEBESErr			DBDAOERR(3406) //Error '|' in default value.
#define E_DAO_BadMSysConf					DBDAOERR(3407) //The server's MSysConf table exists, but is in an incorrect format.  Contact your system administrator.
#define E_DAO_TooManyFindSessions			DBDAOERR(3408) //Too many FastFind Sessions were invoked.
#define E_DAO_InvalidColumnM				DBDAOERR(3409) //Invalid field name '|' in definition of index or relationship.
#define E_DAO_REPReadOnly					DBDAOERR(3410) //*
#define E_DAO_RIInvalidBufferSizeCM			DBDAOERR(3411) //Invalid entry.  Can't perform cascading operation specified in table '|1' because value entered is too big for field '|2'.
#define E_DAO_RIWriteConflictCM				DBDAOERR(3412) //|
#define E_DAO_JetSpecialRIWriteConflictCM	DBDAOERR(3413) //Can't perform cascading update on table '|1' because it is currently in use by user '|3' on machine '|2'.
#define E_DAO_RISessWriteConflictCM			DBDAOERR(3414) //Can't perform cascading update on table '|' because it is currently in use.
#define E_DAO_NoBlank						DBDAOERR(3415) //Zero-length string is valid only in a text or Memo field.
#define E_DAO_FutureError					DBDAOERR(3416) //|
#define E_DAO_QueryInvalidBulkInput			DBDAOERR(3417) //An action query cannot be used as a row source.
#define E_DAO_NetCtrlMismatch				DBDAOERR(3418) //Can't open '|'.  Another user has the table open using a different network control file or locking style.
#define E_DAO_4xTableWith3xLocking			DBDAOERR(3419) //Can't open this Paradox 4.x or Paradox 5.x table because ParadoxNetStyle is set to 3.x in the initialization setting.
#define E_DAO_VtoObjectNotSet				DBDAOERR(3420) //Object is invalid or not set.
#define E_DAO_VtoDataConvError				DBDAOERR(3421) //Data type conversion error.
#define E_DAO_TableNotLocked				DBDAOERR(3422) //Can't modify table structure.  Another user has the table open
#define E_DAO_RmtDriverNotSupported			DBDAOERR(3423) //You cannot use ODBC to attach an external Microsoft Access or ISAM database table to your database
#define E_DAO_InvalidLanguageId				DBDAOERR(3424) //Can't create database; Invalid locale
#define E_DAO_VtoInvalidOpDuringCallback	DBDAOERR(3425) //This method or property is not currently available on this Recordset
#define E_DAO_VtoActionCancelled			DBDAOERR(3426) //The action was cancelled by an associated object
#define E_DAO_VtoOleAutoFailed				DBDAOERR(3427) //Error in DAO automation
#define E_DAO_DatabaseCorrupted_Cmpct		DBDAOERR(3428) //The Jet database engine has encountered a problem in your database.  To correct the problem, you must repair and compact the database
#define E_DAO_IncompatibleIIsam				DBDAOERR(3429) //Incompatible installable ISAM version
#define E_DAO_OLEInitializeFailure			DBDAOERR(3430) //While loading the Excel installable ISAM, OLE was unable to initialize
#define E_DAO_OLENotCompoundFile			DBDAOERR(3431) //This is not an Excel 5 file
#define E_DAO_OLEFailure					DBDAOERR(3432) //Error opening an Excel 5 file
#define E_DAO_IllegalIisamIniOption			DBDAOERR(3433) //Invalid parameter in [Excel ISAM] section of the initialization file
#define E_DAO_TableFull						DBDAOERR(3434) //Can't expand named range
#define E_DAO_TableCantClear				DBDAOERR(3435) //Cannot delete Excel cells
#define E_DAO_CreateFailed					DBDAOERR(3436) //Failure creating file
#define E_DAO_DatabaseFull					DBDAOERR(3437) //Excel spreadsheet is full
#define E_DAO_SpecAndColumnMismatch			DBDAOERR(3438) //File specification and data do not match
#define E_DAO_CantOpenWordMergeFiles		DBDAOERR(3439) //Can't attach or import Word mail merge file
#define E_DAO_FileHasNoColumns				DBDAOERR(3440) //Text file has no columns
#define E_DAO_AmbiguousDelimiters			DBDAOERR(3441) //Text file specification field separator matches decimal seperator or text delimiter
#define E_DAO_FileSpecErrorM				DBDAOERR(3442) //Error in entry |2 of section [|1] in schema.ini
#define E_DAO_NoSpecForFixedFormatM			DBDAOERR(3443) //Can't create fixed width text file without column specification in section [|2] of schema.ini
#define E_DAO_WidthMissInFixedSpecM			DBDAOERR(3444) //Column width required for column |2 in section [|1] of schema.ini
#define E_DAO_VtoWrongDllVersion			DBDAOERR(3445) //Incorrect version of JET DLL found
#define E_DAO_VtoMissingVBA					DBDAOERR(3446) //Could not locate a VBA related file
#define E_DAO_VtoVBAFailed					DBDAOERR(3447) //Failed to initialize VBA
#define E_DAO_VtoOLEFailed					DBDAOERR(3448) //An OLE system function failed
#define E_DAO_InvalidCountry				DBDAOERR(3449) //Missing country code
#define E_DAO_QueryIncompleteRowM			DBDAOERR(3450) //Syntax error in query.  Incomplete query clause
#define E_DAO_QueryParmTypeNotAllowed		DBDAOERR(3451) //Illegal reference in query
#define E_DAO_REPDBNotMaster				DBDAOERR(3452) //You can't make changes to the design of the database at this replica.
#define E_DAO_REPCantRelate					DBDAOERR(3453) //You can't establish or maintain an enforced relationship between a replicated table and a local table.
#define E_DAO_REPNotOwner					DBDAOERR(3454) //*
#define E_DAO_CantMakeReplicable			DBDAOERR(3455) //Can't make the database replicable.
#define E_DAO_CantMakeObjectReplicable		DBDAOERR(3456) //Can't make the |2 object in |1 container replicable.
#define E_DAO_REPCantKeepLocal				DBDAOERR(3457) //You can't set the KeepLocal property for an object that is already replicated.
#define E_DAO_REPCantKeepDBLocal			DBDAOERR(3458) //The KeepLocal property cannot be set on a database; it can be set only on the objects in a database.
#define E_DAO_CantUnreplDatabase			DBDAOERR(3459) //After a database has been replicated, you cannot remove the replication features from the database.
#define E_DAO_ReplConflict					DBDAOERR(3460) //The operation you attempted conflicts with an existing operation involving this member of the replica set.
#define E_DAO_REPSetRepid					DBDAOERR(3461) //The replication property you are attempting to set or delete is read-only and can't be changed.
#define E_DAO_TransportLoadFailure			DBDAOERR(3462) //Failure to load a DLL.
#define E_DAO_TransportLoadFailureM			DBDAOERR(3463) //Can't find the .dll '|2'.
#define E_DAO_TypeMismatchM					DBDAOERR(3464) //Data type mismatch in criteria expression.
#define E_DAO_DiskIOM						DBDAOERR(3465) //The disk drive you are attempting to access is unreadable.
#define E_DAO_FileAccessDeniedM				DBDAOERR(3466) //*
#define E_DAO_InvalidPathM					DBDAOERR(3467) //*
#define E_DAO_TranspAccessDeniedM			DBDAOERR(3468) //Access was denied while accessing dropbox folder '|2'.
#define E_DAO_TransportDiskFullM			DBDAOERR(3469) //The disk for dropbox folder '|2' is full.
#define E_DAO_TransportDiskIOM				DBDAOERR(3470) //Disk failure accessing dropbox folder '|2'.
#define E_DAO_LogWriteFail					DBDAOERR(3471) //Failure to write to the Synchronizer log file.
#define E_DAO_LogDiskFullM					DBDAOERR(3472) //Disk full for path '|1'.
#define E_DAO_LogDiskIOM					DBDAOERR(3473) //Disk failure while accessing log file '|1'.
#define E_DAO_LogFileAccessDeniedM			DBDAOERR(3474) //Can't open the log file '|1' for writing.
#define E_DAO_LogFileShareViolationM		DBDAOERR(3475) //Sharing violation while attempting to open log file '|1' in Deny Write mode.
#define E_DAO_TransportInvalidPathM			DBDAOERR(3476) //Invalid dropbox path '|2'.
#define E_DAO_TranspInvalidAddressM			DBDAOERR(3477) //Dropbox address '|2' is syntactically invalid.
#define E_DAO_RepNotPartial					DBDAOERR(3478) //The replica is not a partial replica.
#define E_DAO_RepPartial					DBDAOERR(3479) //Can't designate a partial replica as the Design Master for the replica set.
#define E_DAO_PARTInvalidRelNameM			DBDAOERR(3480) //The relationship '|' in the partial filter expression is invalid.
#define E_DAO_PARTInvalidTableNameM			DBDAOERR(3481) //The table name '|' in the partial filter expression is invalid.
#define E_DAO_REPInvalidFilter				DBDAOERR(3482) //The filter expression for the partial replica is invalid.
#define E_DAO_TranspInvalidPasswordM		DBDAOERR(3483) //The password supplied for the dropbox folder '|2' is invalid.
#define E_DAO_TransDestInvalidPassword		DBDAOERR(3484) //The password used by the Synchronizer to write to a destination dropbox folder is invalid.
#define E_DAO_REPDBNotRep					DBDAOERR(3485) //The object can't be replicated because the database is not replicated.
#define E_DAO_REPSecondGuid					DBDAOERR(3486) //You can't add a second Replication ID AutoNumber field to a table.
#define E_DAO_REPOnlyBuiltin				DBDAOERR(3487) //The database you are attempting to replicate can't be converted.
#define E_DAO_REPNoSuchRepid				DBDAOERR(3488) //The value specified is not a ReplicaID for any member in the replica set.
#define E_DAO_REPObjectNotRep				DBDAOERR(3489) //The object specified can't be replicated because it is missing a  necessary resource.
#define E_DAO_CantCreateReplica				DBDAOERR(3490) //Can't create a new replica because the '|2' object in '|1' container could not be replicated.
#define E_DAO_MustOpenDbExclusive			DBDAOERR(3491) //The database must be opened in exclusive mode before it can be replicated.
#define E_DAO_CantDoSchemaChange			DBDAOERR(3492) //The synchronization failed because a design change could not be applied to one of the replicas.
#define E_DAO_UnableToSetParam				DBDAOERR(3493) //Can't set the specified Registry parameter for the Synchronizer.
#define E_DAO_UnableToGetParam				DBDAOERR(3494) //Unable to retrieve the specified Registry parameter for the Synchronizer.
#define E_DAO_REPNoSuchSchedule				DBDAOERR(3495) //There are no scheduled synchronizations between the two Synchronizers.
#define E_DAO_REPNoSuchExchange				DBDAOERR(3496) //Replication Manager cannot find the ExchangeID in the MSysExchangeLog table.
#define E_DAO_REPCantSetSchedule			DBDAOERR(3497) //Unable to set a schedule for the Synchronizer.
#define E_DAO_REPCantGetSchedule			DBDAOERR(3498) //*
#define E_DAO_REPCantGetDBPath				DBDAOERR(3499) //Can't retrieve the full path information for a member of the replica set.
#define E_DAO_REPCantSetExchange			DBDAOERR(3500) //You cannot specify two different Synchronizers to manage the same replica.
#define E_DAO_REPNotUpdated					DBDAOERR(3501) //*
#define E_DAO_REPNotManaged					DBDAOERR(3502) //The Design Master or replica is not being managed by a Synchronizer.
#define E_DAO_ValueNotSet					DBDAOERR(3503) //The Synchronizer's Registry has no value set for the key you queried.
#define E_DAO_REPInvalidTID					DBDAOERR(3504) //The Synchronizer ID does not match an existing ID in the MSysTranspAddress table.
#define E_DAO_REPFilterNotFound				DBDAOERR(3505) //You attempted to delete or get information about a partial filter that does not exist in MSysFilters.
#define E_DAO_OpenLog						DBDAOERR(3506) //The Synchronizer is unable to open the Synchronizer log.
#define E_DAO_WriteLog						DBDAOERR(3507) //Failure writing to the Synchronizer log.
#define E_DAO_NoTransport					DBDAOERR(3508) //There is no active transport for the Synchronizer.
#define E_DAO_TransportNotFound				DBDAOERR(3509) //Could not find a valid transport for this Synchronizer.
#define E_DAO_ReplicaAlreadyLocked			DBDAOERR(3510) //The member of the replica set you are attempting to synchronize is currently being used in another synchronization.
#define E_DAO_DBAccess						DBDAOERR(3511) //*
#define E_DAO_TransportReadFailure			DBDAOERR(3512) //Failed to read the dropbox folder.
#define E_DAO_TransportWriteFailure			DBDAOERR(3513) //Failed to write to the dropbox folder.
#define E_DAO_NoExchange					DBDAOERR(3514) //Synchronizer could not find any scheduled or on-demand synchronizations to process.
#define E_DAO_SysClock						DBDAOERR(3515) //The Microsoft Jet database engine could not read the system clock on your computer.
#define E_DAO_NoTransportAddress			DBDAOERR(3516) //Destination synchronizer is not configured to support indirect synchronronization, and the destination replica is unavailable for direct synchronization
#define E_DAO_NoMessage						DBDAOERR(3517) //Synchronizer could not find any messages to process.
#define E_DAO_TransporterNotFound			DBDAOERR(3518) //Could not find Synchronizer in the MSysTranspAddress table.
#define E_DAO_TransportSendFailure			DBDAOERR(3519) //Failed to send a message.
#define E_DAO_ReplicaNotFound				DBDAOERR(3520) //The replica name or ID does not match a currently managed member of the replica set.
#define E_DAO_OutOfSynch					DBDAOERR(3521) //Two members of the replica set cannot be synchronized because there is no common point to start the synchronization.
#define E_DAO_ExchangeNotFound				DBDAOERR(3522) //Synchronizer cannot find the record of a specific synchronization in the MSysExchangeLog table.
#define E_DAO_SchemaNotFound				DBDAOERR(3523) //Synchronizer cannot find a specific version number in the MSysSchChange table.
#define E_DAO_SchemaHistMismatch			DBDAOERR(3524) //The history of design changes in the replica does not match the history in the Design Master.
#define E_DAO_MessageDBAccess				DBDAOERR(3525) //Synchronizer could not access the message database.
#define E_DAO_ObjectAlreadyExists			DBDAOERR(3526) //The name selected for the system object is already in use.
#define E_DAO_ObjectDoesntExist				DBDAOERR(3527) //The Synchronizer or Replication Manager could not find the system object.
#define E_DAO_NoNewData						DBDAOERR(3528) //There is no new data in shared memory for the Synchronizer or Replication Manager to read.
#define E_DAO_PrevDataNotRead				DBDAOERR(3529) //The Synchronizer or Replication Manager found unread data in the shared memory. The existing data will be overwritten.
#define E_DAO_ClientAlreadyExists			DBDAOERR(3530) //The Synchronizer is already serving a client.
#define E_DAO_WaitTimeout					DBDAOERR(3531) //The wait period for an event has timed out.
#define E_DAO_ServerInitialization			DBDAOERR(3532) //Synchronizer could not be initialized.
#define E_DAO_ObjectAbandoned				DBDAOERR(3533) //The system object used by a process still exists after the process has stopped.
#define E_DAO_NoEvent						DBDAOERR(3534) //Synchronizer looked for a system event but did not find one to report to the client.
#define E_DAO_ClientSentTerm				DBDAOERR(3535) //Client has asked the Synchronizer to terminate operation.
#define E_DAO_InvalidMessage				DBDAOERR(3536) //Synchronizer received an invalid message for a member of the replica set that it manages.
#define E_DAO_NoClient						DBDAOERR(3537) //The Synchronizer's client is no longer present and cannot be notified.
#define E_DAO_TooManyTasks					DBDAOERR(3538) //Cannot initialize Synchronizer because there are too many applications running.
#define E_DAO_SysDiskIO						DBDAOERR(3539) //A system error has occurred or your swap file has reached its limit.
#define E_DAO_PageFile						DBDAOERR(3540) //Your swap file has reached its limit or is corrupted.
#define E_DAO_ProcessStillActive			DBDAOERR(3541) //Synchronizer could not be shut down properly and is still active.
#define E_DAO_ProcessAborted				DBDAOERR(3542) //Process stopped when attempting to terminate Synchronizer client.
#define E_DAO_TransporterNotSetup			DBDAOERR(3543) //Synchronizer has not been set up.
#define E_DAO_ServerAlreadyRunning			DBDAOERR(3544) //Synchronizer is already running.
#define E_DAO_DiffReplicaSet				DBDAOERR(3545) //The two replicas you are attempting to synchronize are from different replica sets.
#define E_DAO_BadExchangeType				DBDAOERR(3546) //The type of synchronization you are attempting is not valid.
#define E_DAO_NoReplica						DBDAOERR(3547) //Synchronizer could not find a replica from the correct set to complete the synchronization.
#define E_DAO_GuidMismatch					DBDAOERR(3548) //GUIDs do not match or the requested GUID could not be found.
#define E_DAO_FilenameTooLong				DBDAOERR(3549) //The file name you provided is too long.
#define E_DAO_NoGuidIndex					DBDAOERR(3550) //There is no index on the GUID column.
#define E_DAO_UnableToDeleteParam			DBDAOERR(3551) //Unable to delete the specified Registry parameter for the Synchronizer.
#define E_DAO_ValueTooBig					DBDAOERR(3552) //The size of the Registry parameter exceeds the maximum allowed.
#define E_DAO_REPGuidCreateFailure			DBDAOERR(3553) //The GUID could not be created.
#define E_DAO_REPDBMovedCopied				DBDAOERR(3554) //*
#define E_DAO_REPNoValidNick				DBDAOERR(3555) //All valid nicknames for replicas are already in use.
#define E_DAO_TransportDestInvalidPath		DBDAOERR(3556) //Invalid path for destination dropbox folder.
#define E_DAO_TransDestInvalidAddress		DBDAOERR(3557) //Invalid address for destination dropbox folder.
#define E_DAO_TransportDestDiskIO			DBDAOERR(3558) //Disk I/O error at destination dropbox folder.
#define E_DAO_TransportDestDiskFull			DBDAOERR(3559) //Failure to write because destination disk is full.
#define E_DAO_REPSameReplicaID				DBDAOERR(3560) //The two members of the replica set you are attempting to synchronize have the same ReplicaID.
#define E_DAO_REPBothMasters				DBDAOERR(3561) //The two members of the replica set you are attempting to synchronize are both Design Masters.
#define E_DAO_TransDestAccessDenied			DBDAOERR(3562) //Access denied at destination dropbox folder.
#define E_DAO_TransportSrcAccess			DBDAOERR(3563) //Fatal error accessing a local dropbox folder.
#define E_DAO_TransportSrcFileNotFound		DBDAOERR(3564) //Synchronizer can't find the source file for messages.
#define E_DAO_TransSrcSharingViolation		DBDAOERR(3565) //There is a sharing violation in the source dropbox folder because the message database is open in another application.
#define E_DAO_NetworkIO						DBDAOERR(3566) //Network I/O error.
#define E_DAO_TransportWrongMessage			DBDAOERR(3567) //Message in dropbox folder belongs to the wrong Synchronizer.
#define E_DAO_TransportDeleteFailure		DBDAOERR(3568) //Synchronizer could not delete a file.
#define E_DAO_RepRemoved					DBDAOERR(3569) //This member of the replica set has been logically removed from the set and is no longer available.
#define E_DAO_FiltersChanged				DBDAOERR(3570) //The filters defining a partial replica are out of synch with each other.
#define E_DAO_LimitedUpdate					DBDAOERR(3571) //The attempt  to set a column in a partial replica violated a rule governing partial replicas.
#define E_DAO_TempDiskIO					DBDAOERR(3572) //A disk I/O error occurred while reading or writing to the TEMP directory.
#define E_DAO_DirNotManaged					DBDAOERR(3573) //The directory you queried for a list of replicas is not a managed directory.
#define E_DAO_RepidChanged					DBDAOERR(3574) //The ReplicaID for this member of the replica set was reassigned during a move or copy procedure.
#define E_DAO_DiskFullM						DBDAOERR(3575) //The disk drive you are attempting to write to is full.
#define E_DAO_ShareViolationM				DBDAOERR(3576) //The database you are attempting to open is already in use by another application.
#define E_DAO_UpdateReplCol					DBDAOERR(3577) //Can't update replication system column.
#define E_DAO_GetDbinfoM					DBDAOERR(3578) //Failure to replicate database; can't determine whether the database is open in exclusive mode.
#define E_DAO_MakeRepTablesM				DBDAOERR(3579) //Could not create replication system tables needed to make the database replicable.
#define E_DAO_AddReplicaInfoM				DBDAOERR(3580) //Could not add rows needed to make the database replicable.
#define E_DAO_OpenRepTablesM				DBDAOERR(3581) //Can't open replication system table '|' because the table is already in use.
#define E_DAO_CreateReplicaObjectM			DBDAOERR(3582) //Cannot make a new replica because the |2 object in |1 container could not be made replicable.
#define E_DAO_MakeObjectReplM				DBDAOERR(3583) //Cannot make the |2 object in |1 container replicable.
#define E_DAO_OutOfMemoryM					DBDAOERR(3584) //Insufficient memory to complete operation.
#define E_DAO_RepTooManyColumnsM			DBDAOERR(3585) //Can't replicate the table; the number of columns exceeds the maximum allowed.
#define E_DAO_PARTFilterExprSyntaxM			DBDAOERR(3586) //Syntax error in partial filter expression on table |1.
#define E_DAO_PARTUnknownTokenM				DBDAOERR(3587) //Invalid expression in the ReplicaFilter property.
#define E_DAO_PARTExprEvaluationM			DBDAOERR(3588) //Error when evaluating the partial filter expression.
#define E_DAO_PARTExprUnknownFuncM			DBDAOERR(3589) //The partial filter expression contains an unknown function.
#define E_DAO_LimitedUpdateM				DBDAOERR(3590) //Violates the rules for partial replicas.
#define E_DAO_LogInvalidPathM				DBDAOERR(3591) //Log file path '|1' is invalid.
#define E_DAO_REPPasswdNotAllowed			DBDAOERR(3592) //You can't replicate a password-protected database or set password protection on a replicated database.
#define E_DAO_BadSingleMasterAttrib			DBDAOERR(3593) //You can't change the data master attribute for the replica set.
#define E_DAO_BadMultiMasterAttrib			DBDAOERR(3594) //You can't change the data master attribute for the replica set.  It allows data changes only at the Design Master.
#define E_DAO_REPCantRepair					DBDAOERR(3595) //The system tables in your replica are no longer reliable and the replica should not be used.
#define E_DAO_NoDataIncluded				DBDAOERR(3596) //*
#define E_DAO_SenderNotFound				DBDAOERR(3597) //*
#define E_DAO_CouldnotFindService			DBDAOERR(3598) //*
#define E_DAO_UnableToStartService			DBDAOERR(3599) //*
#define E_DAO_ExprAggIllegalOnGuid			DBDAOERR(3600) //Aggregation expressions cannot use GUIDs.
#define E_DAO_RefreshReplicaList			DBDAOERR(3601) //*
#define E_DAO_MoreWorkNeeded				DBDAOERR(3602) //*
#define E_DAO_SenderTooOld					DBDAOERR(3603) //*
#define E_DAO_RepAccess						DBDAOERR(3604) //*
#define E_DAO_REPDbNotReplicableM			DBDAOERR(3605) //Synchronizing with a non-replicated database is not allowed. The '|' database is not a Design Master or replica.
#define E_DAO_DaemonDied					DBDAOERR(3606) //*
#define E_DAO_REPCantDelete					DBDAOERR(3607) //The replication property you are attempting to delete is read-only and cannot be removed.
#define E_DAO_IndexCantBuild				DBDAOERR(3608) //Record length is too long for an indexed Paradox table.
#define E_DAO_RelNoPrimaryIndex				DBDAOERR(3609) //No unique index found for the referenced field of the primary table.
#define E_DAO_QuerySameSrcDestTableM		DBDAOERR(3610) //Same table '|' referenced as both the source and destination in make-table query.
#define E_DAO_InvalidDDLObject				DBDAOERR(3611) //Can't execute data definition statements on linked data sources.
#define E_DAO_QueryMGBWithSubquery			DBDAOERR(3612) //Multi-level GROUP BY clause is not allowed in a subquery.
#define E_DAO_SQLLinkNotSupported			DBDAOERR(3613) //Can't create a relationship on linked ODBC tables.
#define E_DAO_InvalidFindOnGUID				DBDAOERR(3614) //GUID not allowed in Find method criteria expression.
#define E_DAO_QueryJoinExprInComp			DBDAOERR(3615) //Type mismatch in JOIN expression.
#define E_DAO_UpdateNotAvailable			DBDAOERR(3616) //Updating data in a linked table is not supported by this ISAM.
#define E_DAO_DeleteNotAvailable			DBDAOERR(3617) //Deleting data in a linked table is not supported by this ISAM.
#define E_DAO_ExceptTableCreateFail			DBDAOERR(3618) //Exceptions table could not be created on import/export.
#define E_DAO_ExceptTableWriteFail			DBDAOERR(3619) //Records could not be added to exceptions table.
#define E_DAO_ExcelOLEConnectLost			DBDAOERR(3620) //The connection for viewing your linked Microsoft Excel worksheet was lost.
#define E_DAO_CantChangeDbPwdOnShared		DBDAOERR(3621) //Can't change password on a shared open database.
#define E_DAO_RmtMustCheckTimeStamp			DBDAOERR(3622) //You must use the dbSeeChanges option with OpenRecordset when accessing a SQL Server table that has an IDENTITY column.
#define E_DAO_NotWithBoundFileM				DBDAOERR(3623) //Cannot access the FoxPro 3.0 bound DBF file '|'.
#define E_DAO_CommitConflict				DBDAOERR(3624) //Couldn't read the record; currently locked by another user.
#define E_DAO_NoSuchSpecM					DBDAOERR(3625) //The text file specification '|' does not exist.  You can't import, export, or link using the specification.
#define E_DAO_TooManyIndexesM				DBDAOERR(3626) //The operation failed.  There are too many indexes on table '|'.  Delete some of the indexes on the table and try the operation again.
#define E_DAO_TransExeNotFound				DBDAOERR(3627) //Cannot find the executable file for the Synchronizer (mstran35.exe).
#define E_DAO_RemRepNotManaged				DBDAOERR(3628) //Partner replica is not managed by a Synchronizer.
#define E_DAO_FSDropboxShared				DBDAOERR(3629) //Synchronizer '|1' is also using the same File System dropbox '|2'.
#define E_DAO_FSDropboxSharedM				DBDAOERR(3630) //Synchronizer '|1' is also using the same File System dropbox '|2'.
#define E_DAO_RepInvalidTableInFilter		DBDAOERR(3631) //Invalid Table Name In Filter
#define E_DAO_InetNotEnabled				DBDAOERR(3632) //Internet Transport not enabled on the remote Synchronizer.
#define E_DAO_VtoDllLoadFailed				DBDAOERR(3633) //Can't load DLL:  '|'
#define E_DAO_REPDBIsPartial				DBDAOERR(3634) //Cannot create a replica using a partial replica.
#define E_DAO_CantCreatePartialSys			DBDAOERR(3635) //Cannot create partial replica of a system database.
#define E_DAO_CantPopulateWithErrors		DBDAOERR(3636) //Cannot populate the replica or change the replica's filter because the replica has conflicts or data errors.
#define E_DAO_QueryEmbeddedVarTab			DBDAOERR(3637) //Cannot use the crosstab of a non-fixed column as a subquery.
#define E_DAO_SrcCntrlDB					DBDAOERR(3638) //A Source Controlled database cannot be made replicable.
#define E_DAO_CantCreateSysReplica			DBDAOERR(3639) //Cannot create a replica of a System database.
#define E_DAO_VtoFetchBuffTooSmall			DBDAOERR(3640) //The fetch buffer was too small for the amount of data you requested.
#define E_DAO_VtoEOFDuringFetch				DBDAOERR(3641) //There are fewer records remaining in the recordset than you requested.
#define E_DAO_VtoSilentCancel				DBDAOERR(3642) //A cancel was performed on the operation.
#define E_DAO_VtoRecordDeleted				DBDAOERR(3643) //One of the records in the recordset was deleted by another process.
#define E_DAO_3644							DBDAOERR(3644) //*
#define E_DAO_VtoBadBindInfo				DBDAOERR(3645) //One of the binding parameters is incorrect.
#define E_DAO_VtoRowLenTooSmall				DBDAOERR(3646) //The specified row length is shorter than the sum of the column lengths.
#define E_DAO_VtoColumnMissing				DBDAOERR(3647) //A column requested is not being returned to the recordset.
#define E_DAO_BothPartials					DBDAOERR(3648) //Cannot synchronize a partial replica with another partial replica.
#define E_DAO_InvalidCodePage				DBDAOERR(3649) //The language-specific code page was not specified or could not be found.
#define E_DAO_InetTooSlow					DBDAOERR(3650) //Either the Internet is very slow OR there is some problem in the replication manager setup on the internet server machine.
#define E_DAO_InetInvalidAddress			DBDAOERR(3651) //Invalid internet address.
#define E_DAO_InetLoginFailure				DBDAOERR(3652) //Internet login failure.
#define E_DAO_InetNotSetup					DBDAOERR(3653) //Internet not set up.
#define E_DAO_InetInternalFailure			DBDAOERR(3654) //Internal internet failure.
#define E_DAO_InetServicesUnavailable		DBDAOERR(3655) //The wininet.dll can't be loaded or initialized.
#define E_DAO_PARTExprEvaluation			DBDAOERR(3656) //Error in evaluating a partial expression
#define E_DAO_PARTFilterEvalM				DBDAOERR(3657) //Error in evaluating the boolean filter expression for table '|1'.
#define E_DAO_PARTBinaryNotAllowedM			DBDAOERR(3658) //Binary column '|' cannot be used in a boolean filter.
#define E_DAO_PARTUnenforcedRelM			DBDAOERR(3659) //Relationship '|1' is unenforced. Relationship in a partial filter expression must be enforced.
#define E_DAO_ExchangeFailed				DBDAOERR(3660) //Requested exchange failed because '|1'
#define E_DAO_ExchangeFailedM				DBDAOERR(3661) //Requested exchange failed because '|1'
#define E_DAO_VtoRSNeedsBatchCursorLib		DBDAOERR(3662) //*
#define E_DAO_VtoNeedDiffCursorLibrary		DBDAOERR(3663) //This operation requires a different cursor library.
#define E_DAO_VtoStillConnecting			DBDAOERR(3664) //An asynchronous OpenConnection call is not yet complete, and you cannot yet reference the returned Connection object until it is complete.
#define E_DAO_AccessDeniedRepl				DBDAOERR(3665) //You cannot modify the replication system object '1'
#define E_DAO_AccessDeniedReplM				DBDAOERR(3666) //You cannot modify the replication system object '1'
#define E_DAO_VtoOtherOperBlocking			DBDAOERR(3667) //A different operation is preventing this operation from being executed.
#define E_DAO_VtoNoActiveConnection			DBDAOERR(3668) //Can not perform this operation because there is no active connection.
#define E_DAO_VtoExecCancelled				DBDAOERR(3669) //Execution cancelled.
#define E_DAO_VtoCursorNotValid				DBDAOERR(3670) //Cursor is not valid.
#define E_DAO_VtoCanNotFindTable			DBDAOERR(3671) //Can not find table to update.
#define E_DAO_VtoCanNotFindCursLib			DBDAOERR(3672) //Failed to load RDOCURS.DLL.


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // def _DBDAOERR.H_

