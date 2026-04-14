/*****************************************************************************\
*                                                                             *
* MsiQuery.h - Interface to running installer for custom actions and tools    *
*                                                                             *
* Version 3.0                                                                 *
*                                                                             *
* NOTES:  All buffers sizes are TCHAR count, null included only on input      *
*         Return argument pointers may be null if not interested in value     *
*         Returned handles of all types must be closed: MsiCloseHandle(h)     *
*         Functions with UINT return type return a system error code          *
*         Designated functions will set or clear the last error record,       *
*         which is then accessible with MsiGetLastErrorRecord. However,       *
*         the following argument errors do not register an error record:      *
*         ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_MORE_DATA.     *
*                                                                             *
* Copyright (c) Microsoft Corporation.  All rights reserved.                  *
*                                                                             *
\*****************************************************************************/

#ifndef _MSIQUERY_H_
#define _MSIQUERY_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "msi.h"  // INSTALLSTATE

#define MSI_NULL_INTEGER 0x80000000  // integer value reserved for null

// MsiOpenDatabase persist predefine values, otherwise output database path is used
#define MSIDBOPEN_READONLY     (LPCTSTR)0  // database open read-only, no persistent changes
#define MSIDBOPEN_TRANSACT     (LPCTSTR)1  // database read/write in transaction mode
#define MSIDBOPEN_DIRECT       (LPCTSTR)2  // database direct read/write without transaction
#define MSIDBOPEN_CREATE       (LPCTSTR)3  // create new database, transact mode read/write
#define MSIDBOPEN_CREATEDIRECT (LPCTSTR)4  // create new database, direct mode read/write
#define MSIDBOPEN_PATCHFILE    32/sizeof(*MSIDBOPEN_READONLY) // add flag to indicate patch file

typedef enum tagMSIDBSTATE
{
	MSIDBSTATE_ERROR    =-1,  // invalid database handle
	MSIDBSTATE_READ     = 0,  // database open read-only, no persistent changes
	MSIDBSTATE_WRITE    = 1,  // database readable and updatable
} MSIDBSTATE;

typedef enum tagMSIMODIFY
{
	MSIMODIFY_SEEK             =-1,  // reposition to current record primary key
	MSIMODIFY_REFRESH          = 0,  // refetch current record data
	MSIMODIFY_INSERT           = 1,  // insert new record, fails if matching key exists
	MSIMODIFY_UPDATE           = 2,  // update existing non-key data of fetched record
	MSIMODIFY_ASSIGN           = 3,  // insert record, replacing any existing record
	MSIMODIFY_REPLACE          = 4,  // update record, delete old if primary key edit
	MSIMODIFY_MERGE            = 5,  // fails if record with duplicate key not identical
	MSIMODIFY_DELETE           = 6,  // remove row referenced by this record from table
	MSIMODIFY_INSERT_TEMPORARY = 7,  // insert a temporary record
	MSIMODIFY_VALIDATE         = 8,  // validate a fetched record
	MSIMODIFY_VALIDATE_NEW     = 9,  // validate a new record
	MSIMODIFY_VALIDATE_FIELD   = 10, // validate field(s) of an incomplete record
	MSIMODIFY_VALIDATE_DELETE  = 11, // validate before deleting record
} MSIMODIFY;

typedef enum tagMSICOLINFO
{
	MSICOLINFO_NAMES = 0,  // return column names
	MSICOLINFO_TYPES = 1,  // return column definitions, datatype code followed by width
} MSICOLINFO;

typedef enum tagMSICONDITION
{
	MSICONDITION_FALSE = 0,  // expression evaluates to False
	MSICONDITION_TRUE  = 1,  // expression evaluates to True
	MSICONDITION_NONE  = 2,  // no expression present
	MSICONDITION_ERROR = 3,  // syntax error in expression
} MSICONDITION;

typedef enum tagMSICOSTTREE
{
	MSICOSTTREE_SELFONLY = 0,
	MSICOSTTREE_CHILDREN = 1,
	MSICOSTTREE_PARENTS  = 2,
	MSICOSTTREE_RESERVED = 3,	// Reserved for future use
} MSICOSTTREE;

typedef enum tagMSIDBERROR
{
	MSIDBERROR_INVALIDARG        = -3, //  invalid argument
	MSIDBERROR_MOREDATA          = -2, //  buffer too small
	MSIDBERROR_FUNCTIONERROR     = -1, //  function error
	MSIDBERROR_NOERROR           = 0,  //  no error
	MSIDBERROR_DUPLICATEKEY      = 1,  //  new record duplicates primary keys of existing record in table
	MSIDBERROR_REQUIRED          = 2,  //  non-nullable column, no null values allowed
	MSIDBERROR_BADLINK           = 3,  //  corresponding record in foreign table not found
	MSIDBERROR_OVERFLOW          = 4,  //  data greater than maximum value allowed
	MSIDBERROR_UNDERFLOW         = 5,  //  data less than minimum value allowed
	MSIDBERROR_NOTINSET          = 6,  //  data not a member of the values permitted in the set
	MSIDBERROR_BADVERSION        = 7,  //  invalid version string
	MSIDBERROR_BADCASE           = 8,  //  invalid case, must be all upper-case or all lower-case
	MSIDBERROR_BADGUID           = 9,  //  invalid GUID
	MSIDBERROR_BADWILDCARD       = 10, //  invalid wildcardfilename or use of wildcards
	MSIDBERROR_BADIDENTIFIER     = 11, //  bad identifier
	MSIDBERROR_BADLANGUAGE       = 12, //  bad language Id(s)
	MSIDBERROR_BADFILENAME       = 13, //  bad filename
	MSIDBERROR_BADPATH           = 14, //  bad path
	MSIDBERROR_BADCONDITION      = 15, //  bad conditional statement
	MSIDBERROR_BADFORMATTED      = 16, //  bad format string
	MSIDBERROR_BADTEMPLATE       = 17, //  bad template string
	MSIDBERROR_BADDEFAULTDIR     = 18, //  bad string in DefaultDir column of Directory table
	MSIDBERROR_BADREGPATH        = 19, //  bad registry path string
	MSIDBERROR_BADCUSTOMSOURCE   = 20, //  bad string in CustomSource column of CustomAction table
	MSIDBERROR_BADPROPERTY       = 21, //  bad property string
	MSIDBERROR_MISSINGDATA       = 22, //  _Validation table missing reference to column
	MSIDBERROR_BADCATEGORY       = 23, //  Category column of _Validation table for column is invalid
	MSIDBERROR_BADKEYTABLE       = 24, //  table in KeyTable column of _Validation table could not be found/loaded
	MSIDBERROR_BADMAXMINVALUES   = 25, //  value in MaxValue column of _Validation table is less than value in MinValue column
	MSIDBERROR_BADCABINET        = 26, //  bad cabinet name
	MSIDBERROR_BADSHORTCUT       = 27, //  bad shortcut target
	MSIDBERROR_STRINGOVERFLOW    = 28, //  string overflow (greater than length allowed in column def)
	MSIDBERROR_BADLOCALIZEATTRIB = 29  //  invalid localization attribute (primary keys cannot be localized)

} MSIDBERROR;

typedef enum tagMSIRUNMODE
{
	MSIRUNMODE_ADMIN           =  0, // admin mode install, else product install
	MSIRUNMODE_ADVERTISE       =  1, // installing advertisements, else installing or updating product
	MSIRUNMODE_MAINTENANCE     =  2, // modifying an existing installation, else new installation
	MSIRUNMODE_ROLLBACKENABLED =  3, // rollback is enabled
	MSIRUNMODE_LOGENABLED      =  4, // log file active, enabled prior to install session
	MSIRUNMODE_OPERATIONS      =  5, // spooling execute operations, else in determination phase
	MSIRUNMODE_REBOOTATEND     =  6, // reboot needed after successful installation (settable)
	MSIRUNMODE_REBOOTNOW       =  7, // reboot needed to continue installation (settable)
	MSIRUNMODE_CABINET         =  8, // installing files from cabinets and files using Media table
	MSIRUNMODE_SOURCESHORTNAMES=  9, // source LongFileNames suppressed via PID_MSISOURCE summary property
	MSIRUNMODE_TARGETSHORTNAMES= 10, // target LongFileNames suppressed via SHORTFILENAMES property
	MSIRUNMODE_RESERVED11      = 11, // future use
	MSIRUNMODE_WINDOWS9X       = 12, // operating systems is Windows9?, else Windows NT
	MSIRUNMODE_ZAWENABLED      = 13, // operating system supports demand installation
	MSIRUNMODE_RESERVED14      = 14, // future use
	MSIRUNMODE_RESERVED15      = 15, // future use
	MSIRUNMODE_SCHEDULED       = 16, // custom action call from install script execution
	MSIRUNMODE_ROLLBACK        = 17, // custom action call from rollback execution script
	MSIRUNMODE_COMMIT          = 18, // custom action call from commit execution script
} MSIRUNMODE;

#define INSTALLMESSAGE_TYPEMASK 0xFF000000L  // mask for type code

// Note: INSTALLMESSAGE_ERROR, INSTALLMESSAGE_WARNING, INSTALLMESSAGE_USER are to or'd
// with a message box style to indicate the buttons to display and return:
// MB_OK,MB_OKCANCEL,MB_ABORTRETRYIGNORE,MB_YESNOCANCEL,MB_YESNO,MB_RETRYCANCEL
// the default button (MB_DEFBUTTON1 is normal default):
// MB_DEFBUTTON1, MB_DEFBUTTON2, MB_DEFBUTTON3
// and optionally an icon style:
// MB_ICONERROR, MB_ICONQUESTION, MB_ICONWARNING, MB_ICONINFORMATION

typedef enum tagMSITRANSFORM_ERROR
{
	MSITRANSFORM_ERROR_ADDEXISTINGROW   =  0x00000001,
	MSITRANSFORM_ERROR_DELMISSINGROW    =  0x00000002,
	MSITRANSFORM_ERROR_ADDEXISTINGTABLE =  0x00000004,
	MSITRANSFORM_ERROR_DELMISSINGTABLE  =  0x00000008,
	MSITRANSFORM_ERROR_UPDATEMISSINGROW =  0x00000010,
	MSITRANSFORM_ERROR_CHANGECODEPAGE   =  0x00000020,
	MSITRANSFORM_ERROR_VIEWTRANSFORM    =  0x00000100,
} MSITRANSFORM_ERROR;

typedef enum tagMSITRANSFORM_VALIDATE
{
	MSITRANSFORM_VALIDATE_LANGUAGE                   = 0x00000001,
	MSITRANSFORM_VALIDATE_PRODUCT                    = 0x00000002,
	MSITRANSFORM_VALIDATE_PLATFORM                   = 0x00000004,
	MSITRANSFORM_VALIDATE_MAJORVERSION               = 0x00000008,
	MSITRANSFORM_VALIDATE_MINORVERSION               = 0x00000010,
	MSITRANSFORM_VALIDATE_UPDATEVERSION              = 0x00000020,
	MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION         = 0x00000040,
	MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION    = 0x00000080,
	MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION        = 0x00000100,
	MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION = 0x00000200,
	MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION      = 0x00000400,
	MSITRANSFORM_VALIDATE_UPGRADECODE                = 0x00000800,
} MSITRANSFORM_VALIDATE;

#ifdef __cplusplus
extern "C" {
#endif

// --------------------------------------------------------------------------
// Installer database access functions
// --------------------------------------------------------------------------

// Prepare a database query, creating a view object
// Returns ERROR_SUCCESS if successful, and the view handle is returned,
// else ERROR_INVALID_HANDLE, ERROR_INVALID_HANDLE_STATE, ERROR_BAD_QUERY_SYNTAX, ERROR_GEN_FAILURE
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseOpenViewA(MSIHANDLE hDatabase,
	LPCSTR     szQuery,            // SQL query to be prepared
	MSIHANDLE*  phView);            // returned view if TRUE
UINT WINAPI MsiDatabaseOpenViewW(MSIHANDLE hDatabase,
	LPCWSTR     szQuery,            // SQL query to be prepared
	MSIHANDLE*  phView);            // returned view if TRUE
#ifdef UNICODE
#define MsiDatabaseOpenView  MsiDatabaseOpenViewW
#else
#define MsiDatabaseOpenView  MsiDatabaseOpenViewA
#endif // !UNICODE

// Returns the MSIDBERROR enum and name of the column corresponding to the error
// Similar to a GetLastError function, but for the view. NOT the same as MsiGetLastErrorRecord
// Returns errors of MsiViewModify.

MSIDBERROR WINAPI MsiViewGetErrorA(MSIHANDLE hView,
	_Out_writes_opt_(*pcchBuf)  LPSTR szColumnNameBuffer,   // buffer to hold column name 
	_Inout_opt_                 LPDWORD pcchBuf);             // size of buffer
MSIDBERROR WINAPI MsiViewGetErrorW(MSIHANDLE hView,
	_Out_writes_opt_(*pcchBuf)  LPWSTR szColumnNameBuffer,   // buffer to hold column name 
	_Inout_opt_                 LPDWORD pcchBuf);             // size of buffer
#ifdef UNICODE
#define MsiViewGetError  MsiViewGetErrorW
#else
#define MsiViewGetError  MsiViewGetErrorA
#endif // !UNICODE

// Exectute the view query, supplying parameters as required
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_HANDLE_STATE, ERROR_GEN_FAILURE
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiViewExecute(MSIHANDLE hView,
	MSIHANDLE hRecord);             // optional parameter record, or 0 if none

// Fetch the next sequential record from the view
// Result is ERROR_SUCCESS if a row is found, and its handle is returned
// else ERROR_NO_MORE_ITEMS if no records remain, and a null handle is returned
// else result is error: ERROR_INVALID_HANDLE_STATE, ERROR_INVALID_HANDLE, ERROR_GEN_FAILURE

UINT WINAPI MsiViewFetch(MSIHANDLE hView,
	MSIHANDLE  *phRecord);          // returned data record if fetch succeeds

// Modify a database record, parameters must match types in query columns
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_HANDLE_STATE, ERROR_GEN_FAILURE, ERROR_ACCESS_DENIED
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiViewModify(MSIHANDLE hView,
	MSIMODIFY eModifyMode,         // modify action to perform
	MSIHANDLE hRecord);            // record obtained from fetch, or new record

// Return the column names or specifications for the current view
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, or ERROR_INVALID_HANDLE_STATE

UINT WINAPI MsiViewGetColumnInfo(MSIHANDLE hView,
	MSICOLINFO eColumnInfo,        // retrieve columns names or definitions
	MSIHANDLE *phRecord);          // returned data record containing all names or definitions

// Release the result set for an executed view, to allow re-execution
// Only needs to be called if not all records have been fetched
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_HANDLE_STATE

UINT WINAPI MsiViewClose(MSIHANDLE hView);

// Return a record containing the names of all primary key columns for a given table
// Returns an MSIHANDLE for a record containing the name of each column.
// The field count of the record corresponds to the number of primary key columns.
// Field [0] of the record contains the table name.
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_TABLE

UINT WINAPI MsiDatabaseGetPrimaryKeysA(MSIHANDLE hDatabase,
	LPCSTR    szTableName,       // the name of a specific table <case-sensitive>
	MSIHANDLE  *phRecord);         // returned record if ERROR_SUCCESS
UINT WINAPI MsiDatabaseGetPrimaryKeysW(MSIHANDLE hDatabase,
	LPCWSTR    szTableName,       // the name of a specific table <case-sensitive>
	MSIHANDLE  *phRecord);         // returned record if ERROR_SUCCESS
#ifdef UNICODE
#define MsiDatabaseGetPrimaryKeys  MsiDatabaseGetPrimaryKeysW
#else
#define MsiDatabaseGetPrimaryKeys  MsiDatabaseGetPrimaryKeysA
#endif // !UNICODE

// Return an enum defining the state of the table (temporary, unknown, or persistent).
// Returns MSICONDITION_ERROR, MSICONDITION_FALSE, MSICONDITION_TRUE, MSICONDITION_NONE

MSICONDITION WINAPI MsiDatabaseIsTablePersistentA(MSIHANDLE hDatabase,
	LPCSTR szTableName);         // the name of a specific table
MSICONDITION WINAPI MsiDatabaseIsTablePersistentW(MSIHANDLE hDatabase,
	LPCWSTR szTableName);         // the name of a specific table
#ifdef UNICODE
#define MsiDatabaseIsTablePersistent  MsiDatabaseIsTablePersistentW
#else
#define MsiDatabaseIsTablePersistent  MsiDatabaseIsTablePersistentA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Summary information stream management functions
// --------------------------------------------------------------------------

// Integer Property IDs:    1, 14, 15, 16, 19 
// DateTime Property IDs:   10, 11, 12, 13
// Text Property IDs:       2, 3, 4, 5, 6, 7, 8, 9, 18
// Unsupported Propery IDs: 0 (PID_DICTIONARY), 17 (PID_THUMBNAIL)

// Obtain a handle for the _SummaryInformation stream for an MSI database     
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiGetSummaryInformationA(MSIHANDLE hDatabase, // 0 if not open
	LPCSTR  szDatabasePath,  // path to database, 0 if database handle supplied
	UINT     uiUpdateCount,    // maximium number of updated values, 0 to open read-only
	MSIHANDLE *phSummaryInfo); // returned handle to summary information data
UINT WINAPI MsiGetSummaryInformationW(MSIHANDLE hDatabase, // 0 if not open
	LPCWSTR  szDatabasePath,  // path to database, 0 if database handle supplied
	UINT     uiUpdateCount,    // maximium number of updated values, 0 to open read-only
	MSIHANDLE *phSummaryInfo); // returned handle to summary information data
#ifdef UNICODE
#define MsiGetSummaryInformation  MsiGetSummaryInformationW
#else
#define MsiGetSummaryInformation  MsiGetSummaryInformationA
#endif // !UNICODE

// Obtain the number of existing properties in the SummaryInformation stream

UINT WINAPI MsiSummaryInfoGetPropertyCount(MSIHANDLE hSummaryInfo,
	PUINT puiPropertyCount); // pointer to location to return total property count

// Set a single summary information property
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_UNKNOWN_PROPERTY

UINT WINAPI MsiSummaryInfoSetPropertyA(MSIHANDLE hSummaryInfo,
	UINT     uiProperty,     // property ID, one of allowed values for summary information
	UINT     uiDataType,     // VT_I4, VT_LPSTR, VT_FILETIME, or VT_EMPTY
	INT      iValue,         // integer value, used only if integer property
	FILETIME *pftValue,      // pointer to filetime value, used only if datetime property
	LPCSTR szValue);       // text value, used only if string property
UINT WINAPI MsiSummaryInfoSetPropertyW(MSIHANDLE hSummaryInfo,
	UINT     uiProperty,     // property ID, one of allowed values for summary information
	UINT     uiDataType,     // VT_I4, VT_LPSTR, VT_FILETIME, or VT_EMPTY
	INT      iValue,         // integer value, used only if integer property
	FILETIME *pftValue,      // pointer to filetime value, used only if datetime property
	LPCWSTR szValue);       // text value, used only if string property
#ifdef UNICODE
#define MsiSummaryInfoSetProperty  MsiSummaryInfoSetPropertyW
#else
#define MsiSummaryInfoSetProperty  MsiSummaryInfoSetPropertyA
#endif // !UNICODE

// Get a single property from the summary information
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_UNKNOWN_PROPERTY

UINT WINAPI MsiSummaryInfoGetPropertyA(MSIHANDLE hSummaryInfo,
	UINT     uiProperty,     // property ID, one of allowed values for summary information
	_Out_ PUINT     puiDataType,   // returned type: VT_I4, VT_LPSTR, VT_FILETIME, VT_EMPTY
	_Out_ LPINT     piValue,       // returned integer property data
	_Out_opt_                        FILETIME  *pftValue,      // returned datetime property data
	_Out_writes_opt_(*pcchValueBuf)  LPSTR   szValueBuf,     // buffer to return string property data
	_Inout_opt_                      LPDWORD   pcchValueBuf);  // in/out buffer character count
UINT WINAPI MsiSummaryInfoGetPropertyW(MSIHANDLE hSummaryInfo,
	UINT     uiProperty,     // property ID, one of allowed values for summary information
	_Out_ PUINT     puiDataType,   // returned type: VT_I4, VT_LPSTR, VT_FILETIME, VT_EMPTY
	_Out_ LPINT     piValue,       // returned integer property data
	_Out_opt_                        FILETIME  *pftValue,      // returned datetime property data
	_Out_writes_opt_(*pcchValueBuf)  LPWSTR   szValueBuf,     // buffer to return string property data
	_Inout_opt_                      LPDWORD   pcchValueBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiSummaryInfoGetProperty  MsiSummaryInfoGetPropertyW
#else
#define MsiSummaryInfoGetProperty  MsiSummaryInfoGetPropertyA
#endif // !UNICODE

// Write back changed information to summary information stream

UINT WINAPI MsiSummaryInfoPersist(MSIHANDLE hSummaryInfo);

// --------------------------------------------------------------------------
// Installer database management functions - not used by custom actions
// --------------------------------------------------------------------------

// Open an installer database, specifying the persistance mode, which is a pointer.
// Predefined persist values are reserved pointer values, requiring pointer arithmetic.
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiOpenDatabaseA(
	LPCSTR      szDatabasePath,  // path to database, 0 to create temporary database
	LPCSTR      szPersist,       // output database path or one of predefined values
	MSIHANDLE*   phDatabase);     // location to return database handle
UINT WINAPI MsiOpenDatabaseW(
	LPCWSTR      szDatabasePath,  // path to database, 0 to create temporary database
	LPCWSTR      szPersist,       // output database path or one of predefined values
	MSIHANDLE*   phDatabase);     // location to return database handle
#ifdef UNICODE
#define MsiOpenDatabase  MsiOpenDatabaseW
#else
#define MsiOpenDatabase  MsiOpenDatabaseA
#endif // !UNICODE

// Import an MSI text archive table into an open database
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseImportA(MSIHANDLE hDatabase,
	LPCSTR   szFolderPath,     // folder containing archive files
	LPCSTR   szFileName);      // table archive file to be imported
UINT WINAPI MsiDatabaseImportW(MSIHANDLE hDatabase,
	LPCWSTR   szFolderPath,     // folder containing archive files
	LPCWSTR   szFileName);      // table archive file to be imported
#ifdef UNICODE
#define MsiDatabaseImport  MsiDatabaseImportW
#else
#define MsiDatabaseImport  MsiDatabaseImportA
#endif // !UNICODE

// Export an MSI table from an open database to a text archive file
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseExportA(MSIHANDLE hDatabase,
	LPCSTR   szTableName,      // name of table in database <case-sensitive>
	LPCSTR   szFolderPath,     // folder containing archive files
	LPCSTR   szFileName);      // name of exported table archive file
UINT WINAPI MsiDatabaseExportW(MSIHANDLE hDatabase,
	LPCWSTR   szTableName,      // name of table in database <case-sensitive>
	LPCWSTR   szFolderPath,     // folder containing archive files
	LPCWSTR   szFileName);      // name of exported table archive file
#ifdef UNICODE
#define MsiDatabaseExport  MsiDatabaseExportW
#else
#define MsiDatabaseExport  MsiDatabaseExportA
#endif // !UNICODE

// Merge two database together, allowing duplicate rows
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseMergeA(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseMerge,    // database to be merged into hDatabase
	LPCSTR   szTableName);      // name of non-persistent table to receive errors
UINT WINAPI MsiDatabaseMergeW(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseMerge,    // database to be merged into hDatabase
	LPCWSTR   szTableName);      // name of non-persistent table to receive errors
#ifdef UNICODE
#define MsiDatabaseMerge  MsiDatabaseMergeW
#else
#define MsiDatabaseMerge  MsiDatabaseMergeA
#endif // !UNICODE

// Generate a transform file of differences between two databases
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseGenerateTransformA(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseReference, // base database to reference changes
	LPCSTR   szTransformFile,   // name of generated transform file
	int       iReserved1,         // reserved argument, not used
	int       iReserved2);        // reserved argument, not used
UINT WINAPI MsiDatabaseGenerateTransformW(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseReference, // base database to reference changes
	LPCWSTR   szTransformFile,   // name of generated transform file
	int       iReserved1,         // reserved argument, not used
	int       iReserved2);        // reserved argument, not used
#ifdef UNICODE
#define MsiDatabaseGenerateTransform  MsiDatabaseGenerateTransformW
#else
#define MsiDatabaseGenerateTransform  MsiDatabaseGenerateTransformA
#endif // !UNICODE

// Apply a transform file containing database difference
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseApplyTransformA(MSIHANDLE hDatabase,
	LPCSTR   szTransformFile,    // name of transform file
	int       iErrorConditions);   // errors to suppress, bits from MSITRANSFORM_ERROR
UINT WINAPI MsiDatabaseApplyTransformW(MSIHANDLE hDatabase,
	LPCWSTR   szTransformFile,    // name of transform file
	int       iErrorConditions);   // errors to suppress, bits from MSITRANSFORM_ERROR
#ifdef UNICODE
#define MsiDatabaseApplyTransform  MsiDatabaseApplyTransformW
#else
#define MsiDatabaseApplyTransform  MsiDatabaseApplyTransformA
#endif // !UNICODE

// Create summary information of existing transform to include validation and error conditions
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiCreateTransformSummaryInfoA(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseReference, // base database to reference changes
	LPCSTR   szTransformFile,    // name of generated transform file
	int       iErrorConditions,    // errors to suppress when applied, from MSITRANSFORM_ERROR
	int       iValidation);        // properties validated when applied, MSITRANSFORM_VALIDATE
UINT WINAPI MsiCreateTransformSummaryInfoW(MSIHANDLE hDatabase,
	MSIHANDLE hDatabaseReference, // base database to reference changes
	LPCWSTR   szTransformFile,    // name of generated transform file
	int       iErrorConditions,    // errors to suppress when applied, from MSITRANSFORM_ERROR
	int       iValidation);        // properties validated when applied, MSITRANSFORM_VALIDATE
#ifdef UNICODE
#define MsiCreateTransformSummaryInfo  MsiCreateTransformSummaryInfoW
#else
#define MsiCreateTransformSummaryInfo  MsiCreateTransformSummaryInfoA
#endif // !UNICODE

// Write out all persistent table data, ignored if database opened read-only
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiDatabaseCommit(MSIHANDLE hDatabase);

// Return the update state of a database

MSIDBSTATE WINAPI MsiGetDatabaseState(MSIHANDLE hDatabase);

// --------------------------------------------------------------------------
// Record object functions
// --------------------------------------------------------------------------

// Create a new record object with the requested number of fields
// Field 0, not included in count, is used for format strings and op codes
// All fields are initialized to null
// Returns a handle to the created record, or 0 if memory could not be allocated

MSIHANDLE WINAPI MsiCreateRecord(
	UINT cParams);                   // the number of data fields

// Report whether a record field is NULL
// Returns TRUE if the field is null or does not exist
// Returns FALSE if the field contains data, or the handle is invalid

BOOL WINAPI MsiRecordIsNull(MSIHANDLE hRecord,
	UINT iField);

// Return the length of a record field
// Returns 0 if field is NULL or non-existent
// Returns sizeof(int) if integer data
// Returns character count if string data (not counting null terminator)
// Returns bytes count if stream data

UINT WINAPI MsiRecordDataSize(MSIHANDLE hRecord,
	UINT iField);

// Set a record field to an integer value
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_FIELD

UINT WINAPI MsiRecordSetInteger(MSIHANDLE hRecord,
	UINT iField,
	int iValue);

// Copy a string into the designated field
// A null string pointer and an empty string both set the field to null
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_INVALID_FIELD

UINT WINAPI MsiRecordSetStringA(MSIHANDLE hRecord,
	UINT iField,
	LPCSTR      szValue);
UINT WINAPI MsiRecordSetStringW(MSIHANDLE hRecord,
	UINT iField,
	LPCWSTR      szValue);
#ifdef UNICODE
#define MsiRecordSetString  MsiRecordSetStringW
#else
#define MsiRecordSetString  MsiRecordSetStringA
#endif // !UNICODE

// Return the integer value from a record field
// Returns the value MSI_NULL_INTEGER if the field is null
// or if the field is a string that cannot be converted to an integer

int WINAPI MsiRecordGetInteger(MSIHANDLE hRecord,
	UINT iField);

// Return the string value of a record field
// Integer fields will be converted to a string
// Null and non-existent fields will report a value of 0
// Fields containing stream data will return ERROR_INVALID_DATATYPE
// Returns ERROR_SUCCESS, ERROR_MORE_DATA, 
//         ERROR_INVALID_HANDLE, ERROR_INVALID_FIELD, ERROR_BAD_ARGUMENTS

UINT WINAPI MsiRecordGetStringA(MSIHANDLE hRecord,
	                                UINT iField,
	_Out_writes_opt_(*pcchValueBuf) LPSTR  szValueBuf,      // buffer for returned value
	_Inout_opt_                     LPDWORD  pcchValueBuf);   // in/out buffer character count
UINT WINAPI MsiRecordGetStringW(MSIHANDLE hRecord,
	                                UINT iField,
	_Out_writes_opt_(*pcchValueBuf) LPWSTR  szValueBuf,      // buffer for returned value
	_Inout_opt_                     LPDWORD  pcchValueBuf);   // in/out buffer character count
#ifdef UNICODE
#define MsiRecordGetString  MsiRecordGetStringW
#else
#define MsiRecordGetString  MsiRecordGetStringA
#endif // !UNICODE

// Returns the number of fields allocated in the record
// Does not count field 0, used for formatting and op codes

UINT WINAPI MsiRecordGetFieldCount(MSIHANDLE hRecord);

// Set a record stream field from a file
// The contents of the specified file will be read into a stream object
// The stream will be persisted if the record is inserted into the database
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiRecordSetStreamA(MSIHANDLE hRecord,
	UINT iField,
	LPCSTR      szFilePath);   // path to file containing stream data
UINT WINAPI MsiRecordSetStreamW(MSIHANDLE hRecord,
	UINT iField,
	LPCWSTR      szFilePath);   // path to file containing stream data
#ifdef UNICODE
#define MsiRecordSetStream  MsiRecordSetStreamW
#else
#define MsiRecordSetStream  MsiRecordSetStreamA
#endif // !UNICODE

// Read bytes from a record stream field into a buffer
// Must set the in/out argument to the requested byte count to read
// The number of bytes transferred is returned through the argument
// If no more bytes are available, ERROR_SUCCESS is still returned

UINT WINAPI MsiRecordReadStream(MSIHANDLE hRecord,
	                               UINT iField,
	_Out_writes_bytes_opt_(*pcbDataBuf)  char    *szDataBuf,   // buffer to receive bytes from stream
	_Inout_                        LPDWORD pcbDataBuf);  // in/out buffer byte count

// Clears all data fields in a record to NULL

UINT WINAPI MsiRecordClearData(MSIHANDLE hRecord);

// --------------------------------------------------------------------------
// Functions to access a running installation, called from custom actions
// The install handle is the single argument passed to custom actions
// --------------------------------------------------------------------------

// Return a handle to the database currently in use by this installer instance

MSIHANDLE WINAPI MsiGetActiveDatabase(MSIHANDLE hInstall); // returns handle to database, 0 if none active

// Set the value for an installer property
// If the property is not defined, it will be created
// If the value is null or an empty string, the property will be removed
// Returns ERROR_SUCCESS, ERROR_INVALID_HANDLE, ERROR_BAD_ARGUMENTS

UINT WINAPI MsiSetPropertyA(MSIHANDLE hInstall,
	LPCSTR   szName,       // property identifier, case-sensitive
	LPCSTR   szValue);     // property value, null to undefine property
UINT WINAPI MsiSetPropertyW(MSIHANDLE hInstall,
	LPCWSTR   szName,       // property identifier, case-sensitive
	LPCWSTR   szValue);     // property value, null to undefine property
#ifdef UNICODE
#define MsiSetProperty  MsiSetPropertyW
#else
#define MsiSetProperty  MsiSetPropertyA
#endif // !UNICODE

// Get the value for an installer property
// If the property is not defined, it is equivalent to a 0-length value, not error
// Returns ERROR_SUCCESS, ERROR_MORE_DATA, ERROR_INVALID_HANDLE, ERROR_BAD_ARGUMENTS

UINT  WINAPI MsiGetPropertyA(MSIHANDLE hInstall,
	                                 LPCSTR szName,           // property identifier, case-sensitive
	_Out_writes_opt_(*pcchValueBuf)  LPSTR  szValueBuf,       // buffer for returned property value
	_Inout_opt_                      LPDWORD  pcchValueBuf);    // in/out buffer character count
UINT  WINAPI MsiGetPropertyW(MSIHANDLE hInstall,
	                                 LPCWSTR szName,           // property identifier, case-sensitive
	_Out_writes_opt_(*pcchValueBuf)  LPWSTR  szValueBuf,       // buffer for returned property value
	_Inout_opt_                      LPDWORD  pcchValueBuf);    // in/out buffer character count
#ifdef UNICODE
#define MsiGetProperty  MsiGetPropertyW
#else
#define MsiGetProperty  MsiGetPropertyA
#endif // !UNICODE

// Return the numeric language for the currently running install
// Returns 0 if an install not running

LANGID WINAPI MsiGetLanguage(MSIHANDLE hInstall);

// Return one of the boolean internal installer states
// Returns FALSE if the handle is not active or if the mode is not implemented

BOOL WINAPI MsiGetMode(MSIHANDLE hInstall,
	MSIRUNMODE eRunMode);   // particular mode for which the state is returned

// Set an internal install session boolean mode - Note: most modes are read-only
// Returns ERROR_SUCCESS if the mode can be set to the desired state
// Returns ERROR_ACCESS_DENIED if the mode is not settable
// Returns ERROR_INVALID_HANDLE if the handle is not an active install session

UINT WINAPI MsiSetMode(MSIHANDLE hInstall,
	MSIRUNMODE eRunMode,    // particular mode for which state is to be set
	BOOL fState);           // new state for bit flag

// Format record data using a format string containing field markers and/or properties
// Record field 0 must contain the format string
// Other fields must contain data that may be referenced by the format string.

UINT WINAPI MsiFormatRecordA(MSIHANDLE hInstall, // non-zero for property expansion
	                                  MSIHANDLE hRecord,         // handle to record, field 0 contains format string
	_Out_writes_opt_(*pcchResultBuf)  LPSTR    szResultBuf,    // buffer to return formatted string
	_Inout_opt_                       LPDWORD    pcchResultBuf); // in/out buffer character count
UINT WINAPI MsiFormatRecordW(MSIHANDLE hInstall, // non-zero for property expansion
	                                  MSIHANDLE hRecord,         // handle to record, field 0 contains format string
	_Out_writes_opt_(*pcchResultBuf)  LPWSTR    szResultBuf,    // buffer to return formatted string
	_Inout_opt_                       LPDWORD    pcchResultBuf); // in/out buffer character count
#ifdef UNICODE
#define MsiFormatRecord  MsiFormatRecordW
#else
#define MsiFormatRecord  MsiFormatRecordA
#endif // !UNICODE

// Execute another action, either built-in, custom, or UI wizard
// Returns ERROR_FUNCTION_NOT_CALLED if action not found
// Returns ERROR_SUCCESS if action completed succesfully
// Returns ERROR_INSTALL_USEREXIT if user cancelled during action
// Returns ERROR_INSTALL_FAILURE if action failed
// Returns ERROR_INSTALL_SUSPEND if user suspended installation
// Returns ERROR_MORE_DATA if action wishes to skip remaining actions
// Returns ERROR_INVALID_HANDLE_STATE if install session not active
// Returns ERROR_INVALID_DATA if failure calling custom action
// Returns ERROR_INVALID_HANDLE or ERROR_INVALID_PARAMETER if arguments invalid

UINT WINAPI MsiDoActionA(MSIHANDLE hInstall,
	LPCSTR szAction);     // name of action to call, case-sensitive
UINT WINAPI MsiDoActionW(MSIHANDLE hInstall,
	LPCWSTR szAction);     // name of action to call, case-sensitive
#ifdef UNICODE
#define MsiDoAction  MsiDoActionW
#else
#define MsiDoAction  MsiDoActionA
#endif // !UNICODE

// Execute another action sequence, as descibed in the specified table
// Returns the same error codes as MsiDoAction

UINT WINAPI MsiSequenceA(MSIHANDLE hInstall,
	LPCSTR szTable,       // name of table containing action sequence
	INT iSequenceMode);     // for future use, must be 0 in MSI 1.0
UINT WINAPI MsiSequenceW(MSIHANDLE hInstall,
	LPCWSTR szTable,       // name of table containing action sequence
	INT iSequenceMode);     // for future use, must be 0 in MSI 1.0
#ifdef UNICODE
#define MsiSequence  MsiSequenceW
#else
#define MsiSequence  MsiSequenceA
#endif // !UNICODE

// Send an error record to the installer for processing.
// If field 0 (template) is not set, field 1 must be set to the error code,
//   corresponding the the error message in the Error database table,
//   and the message will be formatted using the template from the Error table
//   before passing it to the UI handler for display.
// Returns Win32 button codes: IDOK IDCANCEL IDABORT IDRETRY IDIGNORE IDYES IDNO
//   or 0 if no action taken, or -1 if invalid argument or handle

int WINAPI MsiProcessMessage(MSIHANDLE hInstall,
	INSTALLMESSAGE eMessageType, // type of message
	MSIHANDLE hRecord);          // record containing message format and data

// Evaluate a conditional expression containing property names and values

MSICONDITION WINAPI MsiEvaluateConditionA(MSIHANDLE hInstall,
	LPCSTR  szCondition);
MSICONDITION WINAPI MsiEvaluateConditionW(MSIHANDLE hInstall,
	LPCWSTR  szCondition);
#ifdef UNICODE
#define MsiEvaluateCondition  MsiEvaluateConditionW
#else
#define MsiEvaluateCondition  MsiEvaluateConditionA
#endif // !UNICODE

// Get the installed state and requested action state of a feature
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiGetFeatureStateA(MSIHANDLE hInstall,
	LPCSTR     szFeature,     // feature name within product
	INSTALLSTATE *piInstalled,  // returned current install state
	INSTALLSTATE *piAction);    // action taken during install session
UINT WINAPI MsiGetFeatureStateW(MSIHANDLE hInstall,
	LPCWSTR     szFeature,     // feature name within product
	INSTALLSTATE *piInstalled,  // returned current install state
	INSTALLSTATE *piAction);    // action taken during install session
#ifdef UNICODE
#define MsiGetFeatureState  MsiGetFeatureStateW
#else
#define MsiGetFeatureState  MsiGetFeatureStateA
#endif // !UNICODE

// Request a feature to be set to a specified state
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiSetFeatureStateA(MSIHANDLE hInstall,
	LPCSTR     szFeature,     // feature name within product
	INSTALLSTATE iState);       // requested state for feature
UINT WINAPI MsiSetFeatureStateW(MSIHANDLE hInstall,
	LPCWSTR     szFeature,     // feature name within product
	INSTALLSTATE iState);       // requested state for feature
#ifdef UNICODE
#define MsiSetFeatureState  MsiSetFeatureStateW
#else
#define MsiSetFeatureState  MsiSetFeatureStateA
#endif // !UNICODE

#if (_WIN32_MSI >=  110)

// Set the attribute bits of a specified feature at runtime.
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiSetFeatureAttributesA(MSIHANDLE hInstall,
	LPCSTR     szFeature,     // feature name within product
	DWORD dwAttributes);        // attributes bits to set for this feature
UINT WINAPI MsiSetFeatureAttributesW(MSIHANDLE hInstall,
	LPCWSTR     szFeature,     // feature name within product
	DWORD dwAttributes);        // attributes bits to set for this feature
#ifdef UNICODE
#define MsiSetFeatureAttributes  MsiSetFeatureAttributesW
#else
#define MsiSetFeatureAttributes  MsiSetFeatureAttributesA
#endif // !UNICODE

#endif //(_WIN32_MSI >=  110)

// Get the installed state and requested action state of a component
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiGetComponentStateA(MSIHANDLE hInstall,
	LPCSTR     szComponent,   // component name within product
	INSTALLSTATE *piInstalled,  // returned current install state
	INSTALLSTATE *piAction);    // action taken during install session
UINT WINAPI MsiGetComponentStateW(MSIHANDLE hInstall,
	LPCWSTR     szComponent,   // component name within product
	INSTALLSTATE *piInstalled,  // returned current install state
	INSTALLSTATE *piAction);    // action taken during install session
#ifdef UNICODE
#define MsiGetComponentState  MsiGetComponentStateW
#else
#define MsiGetComponentState  MsiGetComponentStateA
#endif // !UNICODE

// Request a component to be set to a specified state
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiSetComponentStateA(MSIHANDLE hInstall,
	LPCSTR     szComponent,   // component name within product
	INSTALLSTATE iState);       // requested state for component
UINT WINAPI MsiSetComponentStateW(MSIHANDLE hInstall,
	LPCWSTR     szComponent,   // component name within product
	INSTALLSTATE iState);       // requested state for component
#ifdef UNICODE
#define MsiSetComponentState  MsiSetComponentStateW
#else
#define MsiSetComponentState  MsiSetComponentStateA
#endif // !UNICODE

// Return the disk cost for a feature and related features
// Can specify either current feature state or proposed state
// Can specify extent of related features to cost
// Note that adding costs for several features may produce an
// excessively large cost due to shared components and parents.
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT  WINAPI MsiGetFeatureCostA(MSIHANDLE hInstall,
	LPCSTR      szFeature,      // name of feature
	MSICOSTTREE  iCostTree,     // portion of tree to cost
	INSTALLSTATE iState,        // requested state, or INSTALLSTATE_UNKNOWN
	LPINT         piCost);      // returned cost, in units of 512 bytes
UINT  WINAPI MsiGetFeatureCostW(MSIHANDLE hInstall,
	LPCWSTR      szFeature,      // name of feature
	MSICOSTTREE  iCostTree,     // portion of tree to cost
	INSTALLSTATE iState,        // requested state, or INSTALLSTATE_UNKNOWN
	LPINT         piCost);      // returned cost, in units of 512 bytes
#ifdef UNICODE
#define MsiGetFeatureCost  MsiGetFeatureCostW
#else
#define MsiGetFeatureCost  MsiGetFeatureCostA
#endif // !UNICODE

#if (_WIN32_MSI >= 150)

// Enumerates the costs and temporary costs per drives for
// szComponent. If szComponent is set to NULL, it enumerates
// the above costs for the engine, per drives.
//
// The enumeration is 0-based, i.e. it returns the data for
// the first drive when called w/ dwIndex set to 0.
//
// Can specify either current feature state or proposed state.
//
// Execution of this function sets the error record, accessible
// via MsiGetLastErrorRecord.

UINT WINAPI MsiEnumComponentCostsA(MSIHANDLE hInstall,
	LPCSTR      szComponent,     // name of component
	DWORD        dwIndex,         // 0-based index into the list of drives
	INSTALLSTATE iState,          // requested state, or INSTALLSTATE_UNKNOWN
	_Out_writes_(*pcchDriveBuf)  LPSTR       szDriveBuf,     // buffer for returned value
	_Inout_                      LPDWORD       pcchDriveBuf,   // in/out buffer character count
	_Out_                        LPINT         piCost,         // returned cost, in units of 512 bytes
	_Out_                        LPINT         piTempCost);    // returned temporary cost, in units of 512 bytes
UINT WINAPI MsiEnumComponentCostsW(MSIHANDLE hInstall,
	LPCWSTR      szComponent,     // name of component
	DWORD        dwIndex,         // 0-based index into the list of drives
	INSTALLSTATE iState,          // requested state, or INSTALLSTATE_UNKNOWN
	_Out_writes_(*pcchDriveBuf)  LPWSTR       szDriveBuf,     // buffer for returned value
	_Inout_                      LPDWORD       pcchDriveBuf,   // in/out buffer character count
	_Out_                        LPINT         piCost,         // returned cost, in units of 512 bytes
	_Out_                        LPINT         piTempCost);    // returned temporary cost, in units of 512 bytes
#ifdef UNICODE
#define MsiEnumComponentCosts  MsiEnumComponentCostsW
#else
#define MsiEnumComponentCosts  MsiEnumComponentCostsA
#endif // !UNICODE

#endif // (_WIN32_MSI >= 150)

// Set the install level for a full product installation (not a feature request)
// Setting the value to 0 initialized components and features to the default level
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT  WINAPI MsiSetInstallLevel(MSIHANDLE hInstall,
	int iInstallLevel);

// Get the valid install states for a feature, represented by bit flags
// For each valid install state, a bit is set of value: (1 << INSTALLSTATE)
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT  WINAPI MsiGetFeatureValidStatesA(MSIHANDLE hInstall,
	LPCSTR szFeature,
	LPDWORD  lpInstallStates);
UINT  WINAPI MsiGetFeatureValidStatesW(MSIHANDLE hInstall,
	LPCWSTR szFeature,
	LPDWORD  lpInstallStates);
#ifdef UNICODE
#define MsiGetFeatureValidStates  MsiGetFeatureValidStatesW
#else
#define MsiGetFeatureValidStates  MsiGetFeatureValidStatesA
#endif // !UNICODE

// Return the full source path for a folder in the Directory table
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiGetSourcePathA(MSIHANDLE hInstall,
	                                LPCSTR     szFolder,      // folder identifier, primary key into Directory table
	_Out_writes_opt_(*pcchPathBuf)  LPSTR      szPathBuf,     // buffer to return full path
	_Inout_opt_                     LPDWORD      pcchPathBuf);  // in/out buffer character count
UINT WINAPI MsiGetSourcePathW(MSIHANDLE hInstall,
	                                LPCWSTR     szFolder,      // folder identifier, primary key into Directory table
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR      szPathBuf,     // buffer to return full path
	_Inout_opt_                     LPDWORD      pcchPathBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetSourcePath  MsiGetSourcePathW
#else
#define MsiGetSourcePath  MsiGetSourcePathA
#endif // !UNICODE

// Return the full target path for a folder in the Directory table
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiGetTargetPathA(MSIHANDLE hInstall,
	                                LPCSTR     szFolder,      // folder identifier, primary key into Directory table
	_Out_writes_opt_(*pcchPathBuf)  LPSTR      szPathBuf,     // buffer to return full path
	_Inout_opt_                     LPDWORD      pcchPathBuf);  // in/out buffer character count
UINT WINAPI MsiGetTargetPathW(MSIHANDLE hInstall,
	                                LPCWSTR     szFolder,      // folder identifier, primary key into Directory table
	_Out_writes_opt_(*pcchPathBuf)  LPWSTR      szPathBuf,     // buffer to return full path
	_Inout_opt_                     LPDWORD      pcchPathBuf);  // in/out buffer character count
#ifdef UNICODE
#define MsiGetTargetPath  MsiGetTargetPathW
#else
#define MsiGetTargetPath  MsiGetTargetPathA
#endif // !UNICODE

// Set the full target path for a folder in the Directory table
// Execution of this function sets the error record, accessible via MsiGetLastErrorRecord

UINT WINAPI MsiSetTargetPathA(MSIHANDLE hInstall,
	LPCSTR     szFolder,       // folder identifier, primary key into Directory table
	LPCSTR     szFolderPath);  // full path for folder, ending in directory separator
UINT WINAPI MsiSetTargetPathW(MSIHANDLE hInstall,
	LPCWSTR     szFolder,       // folder identifier, primary key into Directory table
	LPCWSTR     szFolderPath);  // full path for folder, ending in directory separator
#ifdef UNICODE
#define MsiSetTargetPath  MsiSetTargetPathW
#else
#define MsiSetTargetPath  MsiSetTargetPathA
#endif // !UNICODE

// Check to see if sufficent disk space is present for the current installation
// Returns ERROR_SUCCESS, ERROR_DISK_FULL, ERROR_INVALID_HANDLE_STATE, or ERROR_INVALID_HANDLE

UINT WINAPI MsiVerifyDiskSpace(MSIHANDLE hInstall);

// --------------------------------------------------------------------------
// Functions for rendering UI dialogs from the database representations.
// Purpose is for product development, not for use during installation.
// --------------------------------------------------------------------------

// Enable UI in preview mode to facilitate authoring of UI dialogs.
// The preview mode will end when the handle is closed.

UINT WINAPI MsiEnableUIPreview(MSIHANDLE hDatabase,
	MSIHANDLE* phPreview);       // returned handle for UI preview capability

// Display any UI dialog as modeless and inactive.
// Supplying a null name will remove any current dialog.

UINT WINAPI MsiPreviewDialogA(MSIHANDLE hPreview,
	LPCSTR szDialogName);      // dialog to display, Dialog table key
UINT WINAPI MsiPreviewDialogW(MSIHANDLE hPreview,
	LPCWSTR szDialogName);      // dialog to display, Dialog table key
#ifdef UNICODE
#define MsiPreviewDialog  MsiPreviewDialogW
#else
#define MsiPreviewDialog  MsiPreviewDialogA
#endif // !UNICODE

// Display a billboard within a host control in the displayed dialog.
// Supplying a null billboard name will remove any billboard displayed.

UINT WINAPI MsiPreviewBillboardA(MSIHANDLE hPreview,
	LPCSTR szControlName,      // name of control that accepts billboards
	LPCSTR szBillboard);       // name of billboard to display
UINT WINAPI MsiPreviewBillboardW(MSIHANDLE hPreview,
	LPCWSTR szControlName,      // name of control that accepts billboards
	LPCWSTR szBillboard);       // name of billboard to display
#ifdef UNICODE
#define MsiPreviewBillboard  MsiPreviewBillboardW
#else
#define MsiPreviewBillboard  MsiPreviewBillboardA
#endif // !UNICODE

// --------------------------------------------------------------------------
// Error handling not associated with any particular object
// --------------------------------------------------------------------------

// Return a record handle to the last function that generated an error record
// Only specified functions will set the error record, or clear it if success
// Field 1 of the record will contain the internal MSI error code
// Other fields will contain data specific to the particular error
// The error record is released internally after this function is executed

MSIHANDLE WINAPI MsiGetLastErrorRecord();  // returns 0 if no cached record

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSIQUERY_H_
