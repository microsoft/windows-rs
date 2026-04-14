/*++ BUILD Version: 0000     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    esent.h

Abstract:

    This module defines the types and constants that are
    exposed through the ESE API.

--*/

#pragma once

#if !defined(_JET_INCLUDED)
#define _JET_INCLUDED

#include <winapifamily.h>

#ifdef  __cplusplus
extern "C" {
#endif

#include <specstrings.h>


// JET_VERSION is similar to WINVER. It allows the most recent header to be used
// against older targets. Supported versions are:
// 0x0500  - Windows 2000
// 0x0501  - Windows XP
// 0x0502  - Windows Server 2003
// 0x0600  - Windows Vista / Windows Server 2008
// 0x0601  - Windows 7 / Windows Server 2008 R2
// 0x0602  - Windows 8 / Windows Server 2012
// 0x0603  - Windows 8.1 / Windows Server 2012 R2
// 0x0A00  - Windows 10


#ifndef JET_VERSION
#  ifdef WINVER
#    define JET_VERSION WINVER
#  else
        // JET_VERSION has not been specified. Assume all functions are available.
#    define JET_VERSION 0x0A00
#  endif
#endif



#if defined(_WIN64)
#pragma pack(push,8)
#else
#pragma pack(push,4)
#endif


#pragma warning(push)
#pragma warning(disable: 4201)      //  nonstandard extension used : nameless struct/union


#define JET_API     __stdcall
#define JET_NODSAPI __stdcall

#ifndef _JET_BASE_TYPES_DEFINED
#define _JET_BASE_TYPES_DEFINED
// Do we define the basic JET integral types from stdint.h, or do we use the
// historical definitions?
#ifdef _JET_API_USE_STDINT

#include <stdint.h>

typedef int8_t                          JET_INT8;
typedef uint8_t                         JET_UINT8;
typedef int16_t                         JET_INT16;
typedef uint16_t                        JET_UINT16;
typedef int32_t                         JET_INT32;
typedef uint32_t                        JET_UINT32;
typedef int64_t                         JET_INT64;
typedef uint64_t                        JET_UINT64;
typedef uint8_t                         JET_BYTE;

#else

// Note the use of "long" rather than "int" for JET_INT32/JET_UINT32.
// The JET_API has historically used the base type "long" for 32bit integral
// types.  While "int" and "long" are both 32bit integral types and coerce
// back and forth and so may be used interchangably, "int *" and "long *" do
// not.  If the base type of the 32bit integral types were to change to
// "int", existing client code could break and require casts for the pointer types.
typedef char               JET_INT8;
typedef unsigned char      JET_UINT8;
typedef short              JET_INT16;
typedef unsigned short     JET_UINT16;
typedef long               JET_INT32;
typedef unsigned long      JET_UINT32;
typedef long long          JET_INT64;
typedef unsigned long long JET_UINT64;
typedef unsigned char      JET_BYTE;

#endif

typedef void               JET_VOID;
typedef void *             JET_PVOID;
typedef const void *       JET_PCVOID;
typedef char               JET_CHAR;
#if !defined(_NATIVE_WCHAR_T_DEFINED)
typedef unsigned short     JET_WCHAR;
#else
typedef wchar_t            JET_WCHAR;
#endif
#endif // _JET_BASE_TYPES_DEFINED
// After this point, use only JET_type style types.  That is, don't use base types like
// long, int, short, etc, use JET_INT32, JET_INT16, etc.


#if defined(_WIN64)
    typedef JET_UINT64 JET_API_PTR;
#else
    typedef JET_UINT32 JET_API_PTR;
#endif

typedef _Return_type_success_( return >= 0 ) JET_INT32 JET_ERR;

typedef JET_UINT32         JET_LCID;
typedef JET_UINT16         JET_LANGID;
typedef JET_UINT16         JET_CP;


typedef JET_API_PTR JET_HANDLE;     // backup file handle
typedef JET_API_PTR JET_INSTANCE;   // Instance Identifier
typedef JET_API_PTR JET_SESID;      // Session Identifier
typedef JET_API_PTR JET_TABLEID;    // Table Identifier
#if ( JET_VERSION >= 0x0501 )
typedef JET_API_PTR JET_LS;         // Local Storage
#endif // JET_VERSION >= 0x0501

typedef JET_UINT32 JET_COLUMNID;    // Column Identifier

typedef struct tagJET_INDEXID
{
    JET_UINT32      cbStruct;
    JET_BYTE        rgbIndexId[sizeof(JET_API_PTR)+sizeof(JET_UINT32)+sizeof(JET_UINT32)];
} JET_INDEXID;

typedef JET_UINT32 JET_DBID;     // Database Identifier
typedef JET_UINT32 JET_OBJTYP;   // Object Type
typedef JET_UINT32 JET_COLTYP;   // Column Type
typedef JET_UINT32 JET_GRBIT;    // Group of Bits

typedef JET_UINT32 JET_SNP;      // Status Notification Process
typedef JET_UINT32 JET_SNT;      // Status Notification Type
typedef double JET_DATESERIAL;   // JET_coltypDateTime format
#if ( JET_VERSION >= 0x0501 )
typedef JET_UINT32 JET_CBTYP;    // Callback Types
#endif // JET_VERSION >= 0x0501

typedef JET_ERR (JET_API * JET_PFNSTATUS)(
    _In_ JET_SESID     sesid,
    _In_ JET_SNP       snp,
    _In_ JET_SNT       snt,
    _In_opt_ JET_PVOID pv );


typedef _Null_terminated_ JET_CHAR *         JET_PSTR;    // ASCII string (char *) null terminated
typedef _Null_terminated_ const JET_CHAR *   JET_PCSTR;   // const ASCII string (char *) null terminated
typedef _Null_terminated_ JET_WCHAR *        JET_PWSTR;   // Unicode string (wchar_t *) null terminated
typedef _Null_terminated_ const JET_WCHAR *  JET_PCWSTR;  // const Unicode string (wchar_t *) null terminated


typedef struct
{
    JET_PSTR        szDatabaseName;
    JET_PSTR        szNewDatabaseName;
} JET_RSTMAP_A;         // restore map

typedef struct
{
    JET_PWSTR       szDatabaseName;
    JET_PWSTR       szNewDatabaseName;
} JET_RSTMAP_W;         // restore map

#ifdef JET_UNICODE
#define JET_RSTMAP JET_RSTMAP_W
#else
#define JET_RSTMAP JET_RSTMAP_A
#endif


//  For edbutil convert and JetConvert() only.

typedef struct tagCONVERT_A
{
    JET_PSTR                szOldDll;
    union
    {
        JET_UINT32          fFlags;
        struct
        {
            JET_UINT32      fSchemaChangesOnly:1;
        };
    };
} JET_CONVERT_A;

typedef struct tagCONVERT_W
{
    JET_PWSTR               szOldDll;
    union
    {
        JET_UINT32          fFlags;
        struct
        {
            JET_UINT32      fSchemaChangesOnly:1;
        };
    };
} JET_CONVERT_W;

#ifdef JET_UNICODE
#define JET_CONVERT JET_CONVERT_W
#else
#define JET_CONVERT JET_CONVERT_A
#endif


//  Configuration Store
//
//  ESE has the ability to use an external config store for ESE database engine and instance
//  settings.
//
//  Using the registry this might look something like setting the JET param to this:
//
//      JET_paramConfigStoreSpec    "reg:HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PopServer(Inst1)"
//
//          where "PopServer(Inst1)" is just an exaple name, you should pick a different name or
//          even a different part of the registry if appropriate.  You are limited however to
//          beginning under: HKEY_LOCAL_MACHINE or HKEY_CURRENT_USER.
//
//      And configuring the registry thusly:
//
//Windows Registry Editor Version 5.00
//
//[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PopServer(Inst1)]
//
//[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PopServer(Inst1)\SysParamDefault]
//"JET_paramDatabasePageSize"="8192"
//"JET_paramEnableFileCache"="1"
//
//  All values should be set as a string version of a decimal of hex values (such as "0x2000").
//


//
//  The security settings / [D]ACL of the registry key provided to JET_paramConfigStoreSpec
//  should be locked down to the same security context as the client application that uses ESE
//  and opens the database, otherwise there is a possible Escalation of Privilege attack.
//

//  The JET_wszConfigStoreReadControl in the registry are registry values under the root
//  registry key passed to JET_paramConfigStoreSpec.

#define JET_wszConfigStoreReadControl                           L"CsReadControl"
#define JET_bitConfigStoreReadControlInhibitRead                 0x1        //  Will stop reading from the registry config store, and pause reading until flag is removed (this will stall some JET initialization APIs).
#define JET_bitConfigStoreReadControlDisableAll                  0x2        //  Simply disables the registry config store from being read or used.
#define JET_bitConfigStoreReadControlDefault                     0x0        //  Use default ESE behavior.


//  The JET_wszConfigStoreRelPathSysParamDefault and JET_wszConfigStoreRelPathSysParamOverride in
//  the registry are registry sub-keys under the root registry key passed to JET_paramConfigStoreSpec.
#define JET_wszConfigStoreRelPathSysParamDefault        L"SysParamDefault"
#define JET_wszConfigStoreRelPathSysParamOverride       L"SysParamOverride"



// Special format specifiers here
#define JET_efvUseEngineDefault             (0x40000001)    //  Instructs the engine to use the maximal default supported Engine Format Version. (default)
#define JET_efvUsePersistedFormat           (0x40000002)    //  Instructs the engine to use the minimal Engine Format Version of all loaded log and DB files.
#define JET_efvAllowHigherPersistedFormat   (0x41000000)    //  Can be combined with a specific EngineFormatVersion but will not fail if persisted files are ahead of the specified EngineFormatVersion.  Will still fail if the persisted version is ahead of what the engine actually can read/understand.

#define JET_efvWindows19H1Rtm                   8920        //  Last pre-efv version, shipped in Windows 10 until 19H1 release.
#define JET_efvWindows10v2004                   9180        //  Efv shipped with Windows "Vibranium", first shipped with Windows 10 version 2004 (and all subsequent Windows 10 releases).
#define JET_efvWindowsServer2022                9360        //  Efv shipped with Windows Server 2022.
#define JET_efvWindows11v21H2                   9400        //  Efv shipped with Windows 11 21H2 release.
#define JET_efvWindows11v22H2                   9480        //  Efv shipped with Windows 11 22H2 release.
#define JET_efvWindows11v23H2                   9600        //  Efv shipped with Windows 11 23H2 release.


//  Online defragmentation (JetDefragment/JetDefragment2) options
#define JET_bitDefragmentBatchStart             0x00000001
#define JET_bitDefragmentBatchStop              0x00000002

#if ( JET_VERSION >= 0x0501 )
#define JET_bitDefragmentAvailSpaceTreesOnly    0x00000040  /* only defrag AvailExt trees */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0601 )
#define JET_bitDefragmentNoPartialMerges        0x00000080  /* don't do partial merges during OLD */

#define JET_bitDefragmentBTree                  0x00000100  /* defrag one B-Tree */


#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0501 )
    /* Callback-function types */

#define JET_cbtypNull                           0x00000000
#define JET_cbtypFinalize                       0x00000001  /* DEPRECATED: a finalizable column has gone to zero */
#define JET_cbtypBeforeInsert                   0x00000002  /* about to insert a record */
#define JET_cbtypAfterInsert                    0x00000004  /* finished inserting a record */
#define JET_cbtypBeforeReplace                  0x00000008  /* about to modify a record */
#define JET_cbtypAfterReplace                   0x00000010  /* finished modifying a record */
#define JET_cbtypBeforeDelete                   0x00000020  /* about to delete a record */
#define JET_cbtypAfterDelete                    0x00000040  /* finished deleting the record */
#define JET_cbtypUserDefinedDefaultValue        0x00000080  /* calculating a user-defined default */
#define JET_cbtypOnlineDefragCompleted          0x00000100  /* a call to JetDefragment2 has completed */
#define JET_cbtypFreeCursorLS                   0x00000200  /* the Local Storage associated with a cursor must be freed */
#define JET_cbtypFreeTableLS                    0x00000400  /* the Local Storage associated with a table must be freed */

    /* Callback-function prototype */

typedef JET_ERR (JET_API * JET_CALLBACK)(
    _In_ JET_SESID        sesid,
    _In_ JET_DBID         dbid,
    _In_ JET_TABLEID      tableid,
    _In_ JET_CBTYP        cbtyp,
    _Inout_opt_ JET_PVOID pvArg1,
    _Inout_opt_ JET_PVOID pvArg2,
    _In_opt_ JET_PVOID    pvContext,
    _In_ JET_API_PTR      ulUnused );
#endif // JET_VERSION >= 0x0501

    /* Status Notification Structures */

typedef struct              // Status Notification Progress
{
    JET_UINT32      cbStruct;   // Size of this structure
    JET_UINT32      cunitDone;  // Number of units of work completed
    JET_UINT32      cunitTotal; // Total number of units of work
} JET_SNPROG;

typedef struct
{
    JET_UINT32              cbStruct;

    JET_UINT32              cbFilesizeLow;          //  file's current size (low DWORD)
    JET_UINT32              cbFilesizeHigh;         //  file's current size (high DWORD)

    JET_UINT32              cbFreeSpaceRequiredLow; //  estimate of free disk space required for in-place upgrade (low DWORD)
    JET_UINT32              cbFreeSpaceRequiredHigh;//  estimate of free disk space required for in-place upgrade (high DWORD)

    JET_UINT32              csecToUpgrade;          //  estimate of time required, in seconds, for upgrade

    union
    {
        JET_UINT32          ulFlags;
        struct
        {
            JET_UINT32      fUpgradable:1;
            JET_UINT32      fAlreadyUpgraded:1;
        };
    };
} JET_DBINFOUPGRADE;

typedef struct
{
    JET_UINT32          cbStruct;
    JET_OBJTYP          objtyp;
    JET_DATESERIAL      dtCreate;   //  Deprecated.
    JET_DATESERIAL      dtUpdate;   //  Deprecated.
    JET_GRBIT           grbit;
    JET_UINT32          flags;
    JET_UINT32          cRecord;
    JET_UINT32          cPage;
} JET_OBJECTINFO;

    /* The following flags appear in the grbit field above */

#define JET_bitTableInfoUpdatable   0x00000001
#define JET_bitTableInfoBookmark    0x00000002
#define JET_bitTableInfoRollback    0x00000004

    /* The following flags occur in the flags field above */

#define JET_bitObjectSystem         0x80000000  // Internal use only
#define JET_bitObjectTableFixedDDL  0x40000000  // Table's DDL is fixed
#define JET_bitObjectTableTemplate  0x20000000  // Table's DDL is inheritable (implies FixedDDL)
#define JET_bitObjectTableDerived   0x10000000  // Table's DDL is inherited from a template table
#if ( JET_VERSION >= 0x0501 )
#define JET_bitObjectTableNoFixedVarColumnsInDerivedTables  0x04000000  //  used in conjunction with JET_bitObjectTableTemplate
                                                                        //    to disallow fixed/var columns in derived tables (so that
                                                                        //    fixed/var columns may be added to the template in the future)
#endif // JET_VERSION >= 0x0501


typedef struct
{
    JET_UINT32      cbStruct;
    JET_TABLEID     tableid;
    JET_UINT32      cRecord;
    JET_COLUMNID    columnidcontainername;
    JET_COLUMNID    columnidobjectname;
    JET_COLUMNID    columnidobjtyp;
    JET_COLUMNID    columniddtCreate;   //  XXX -- to be deleted
    JET_COLUMNID    columniddtUpdate;   //  XXX -- to be deleted
    JET_COLUMNID    columnidgrbit;
    JET_COLUMNID    columnidflags;
    JET_COLUMNID    columnidcRecord;    // Level 2 info
    JET_COLUMNID    columnidcPage;      // Level 2 info
} JET_OBJECTLIST;

#define cObjectInfoCols 9

typedef struct
{
    JET_UINT32      cbStruct;
    JET_TABLEID     tableid;
    JET_UINT32      cRecord;
    JET_COLUMNID    columnidPresentationOrder;
    JET_COLUMNID    columnidcolumnname;
    JET_COLUMNID    columnidcolumnid;
    JET_COLUMNID    columnidcoltyp;
    JET_COLUMNID    columnidCountry;        // specifies the columnid for the country/region field
    JET_COLUMNID    columnidLangid;
    JET_COLUMNID    columnidCp;
    JET_COLUMNID    columnidCollate;
    JET_COLUMNID    columnidcbMax;
    JET_COLUMNID    columnidgrbit;
    JET_COLUMNID    columnidDefault;
    JET_COLUMNID    columnidBaseTableName;
    JET_COLUMNID    columnidBaseColumnName;
    JET_COLUMNID    columnidDefinitionName;
} JET_COLUMNLIST;

#define cColumnInfoCols 14

typedef struct
{
    JET_UINT32      cbStruct;
    JET_COLUMNID    columnid;
    JET_COLTYP      coltyp;
    JET_UINT16      wCountry;           // sepcifies the country/region for the column definition
    JET_LANGID      langid;
    JET_CP          cp;
    JET_UINT16      wCollate;           // Must be 0
    JET_UINT32      cbMax;
    JET_GRBIT       grbit;
} JET_COLUMNDEF;


typedef struct
{
    JET_UINT32      cbStruct;
    JET_COLUMNID    columnid;
    JET_COLTYP      coltyp;
    JET_UINT16      wCountry;           // specifies the columnid for the country/region field
    JET_LANGID      langid;
    JET_CP          cp;
    JET_UINT16      wFiller;            // Must be 0
    JET_UINT32      cbMax;
    JET_GRBIT       grbit;
    JET_CHAR        szBaseTableName[256];
    JET_CHAR        szBaseColumnName[256];
} JET_COLUMNBASE_A;


typedef struct
{
    JET_UINT32      cbStruct;
    JET_COLUMNID    columnid;
    JET_COLTYP      coltyp;
    JET_UINT16      wCountry;           // specifies the columnid for the country/region field
    JET_LANGID      langid;
    JET_CP          cp;
    JET_UINT16      wFiller;            // Must be 0
    JET_UINT32      cbMax;
    JET_GRBIT       grbit;
    JET_WCHAR       szBaseTableName[256];
    JET_WCHAR       szBaseColumnName[256];
} JET_COLUMNBASE_W;


#ifdef JET_UNICODE
#define JET_COLUMNBASE JET_COLUMNBASE_W
#else
#define JET_COLUMNBASE JET_COLUMNBASE_A
#endif


typedef struct
{
    JET_UINT32      cbStruct;
    JET_TABLEID     tableid;
    JET_UINT32      cRecord;
    JET_COLUMNID    columnidindexname;
    JET_COLUMNID    columnidgrbitIndex;
    JET_COLUMNID    columnidcKey;
    JET_COLUMNID    columnidcEntry;
    JET_COLUMNID    columnidcPage;
    JET_COLUMNID    columnidcColumn;
    JET_COLUMNID    columnidiColumn;
    JET_COLUMNID    columnidcolumnid;
    JET_COLUMNID    columnidcoltyp;
    JET_COLUMNID    columnidCountry;        // specifies the columnid for the country/region field
    JET_COLUMNID    columnidLangid;
    JET_COLUMNID    columnidCp;
    JET_COLUMNID    columnidCollate;
    JET_COLUMNID    columnidgrbitColumn;
    JET_COLUMNID    columnidcolumnname;
    JET_COLUMNID    columnidLCMapFlags;
} JET_INDEXLIST;


#define cIndexInfoCols 15

typedef struct tag_JET_COLUMNCREATE_A
{
    JET_UINT32      cbStruct;               // size of this structure (for future expansion)
    JET_PSTR        szColumnName;           // column name
    JET_COLTYP      coltyp;                 // column type
    JET_UINT32      cbMax;                  // the maximum length of this column (only relevant for binary and text columns)
    JET_GRBIT       grbit;                  // column options
    JET_PVOID       pvDefault;              // default value (NULL if none)
    JET_UINT32      cbDefault;              // length of default value
    JET_UINT32      cp;                     // code page (for text columns only).  Note that historically we don't use JET_CP for this,
                                            //     although the value actually is a JET_CP.
    JET_COLUMNID    columnid;               // returned column id
    JET_ERR         err;                    // returned error code
} JET_COLUMNCREATE_A;

typedef struct tag_JET_COLUMNCREATE_W
{
    JET_UINT32      cbStruct;               // size of this structure (for future expansion)
    JET_PWSTR       szColumnName;           // column name
    JET_COLTYP      coltyp;                 // column type
    JET_UINT32      cbMax;                  // the maximum length of this column (only relevant for binary and text columns)
    JET_GRBIT       grbit;                  // column options
    JET_PVOID       pvDefault;              // default value (NULL if none)
    JET_UINT32      cbDefault;              // length of default value
    JET_UINT32      cp;                     // code page (for text columns only).  Note that historically we don't use JET_CP for this,
                                            //     although the value actually is a JET_CP.
    JET_COLUMNID    columnid;               // returned column id
    JET_ERR         err;                    // returned error code
} JET_COLUMNCREATE_W;

#ifdef JET_UNICODE
#define JET_COLUMNCREATE JET_COLUMNCREATE_W
#else
#define JET_COLUMNCREATE JET_COLUMNCREATE_A
#endif

#if ( JET_VERSION >= 0x0501 )
//  This is the information needed to create a column with a user-defined default. It should be passed in using
//  the pvDefault and cbDefault in a JET_COLUMNCREATE structure

typedef struct tag_JET_USERDEFINEDDEFAULT_A
{
    JET_PSTR     szCallback;
    JET_BYTE *   pbUserData;
    JET_UINT32   cbUserData;
    JET_PSTR     szDependantColumns;
} JET_USERDEFINEDDEFAULT_A;

typedef struct tag_JET_USERDEFINEDDEFAULT_W
{
    JET_PWSTR    szCallback;
    JET_BYTE *   pbUserData;
    JET_UINT32   cbUserData;
    JET_PWSTR    szDependantColumns;
} JET_USERDEFINEDDEFAULT_W;

#ifdef JET_UNICODE
#define JET_USERDEFINEDDEFAULT JET_USERDEFINEDDEFAULT_W
#else
#define JET_USERDEFINEDDEFAULT JET_USERDEFINEDDEFAULT_A
#endif

#endif // JET_VERSION >= 0x0501

typedef struct tagJET_CONDITIONALCOLUMN_A
{
    JET_UINT32      cbStruct;               // size of this structure (for future expansion)
    JET_PSTR        szColumnName;           // column that we are conditionally indexed on
    JET_GRBIT       grbit;                  // conditional column options
} JET_CONDITIONALCOLUMN_A;

typedef struct tagJET_CONDITIONALCOLUMN_W
{
    JET_UINT32      cbStruct;               // size of this structure (for future expansion)
    JET_PWSTR       szColumnName;           // column that we are conditionally indexed on
    JET_GRBIT       grbit;                  // conditional column options
} JET_CONDITIONALCOLUMN_W;

#ifdef JET_UNICODE
#define JET_CONDITIONALCOLUMN JET_CONDITIONALCOLUMN_W
#else
#define JET_CONDITIONALCOLUMN JET_CONDITIONALCOLUMN_A
#endif

typedef struct tagJET_UNICODEINDEX
{
    JET_LCID        lcid;
    JET_UINT32      dwMapFlags;
} JET_UNICODEINDEX;

#if ( JET_VERSION >= 0x0602 )
typedef struct tagJET_UNICODEINDEX2
{
    _Field_z_ JET_PWSTR szLocaleName;
    JET_UINT32          dwMapFlags;
} JET_UNICODEINDEX2;
#endif //JET_VERSION >= 0x0602

#if ( JET_VERSION >= 0x0502 )
typedef struct tagJET_TUPLELIMITS
{
    JET_UINT32      chLengthMin;
    JET_UINT32      chLengthMax;
    JET_UINT32      chToIndexMax;
#if ( JET_VERSION >= 0x0600 )
    JET_UINT32      cchIncrement;
    JET_UINT32      ichStart;
#endif // JET_VERSION >= 0x0600
} JET_TUPLELIMITS;
#endif // JET_VERSION >= 0x0502

#if ( JET_VERSION >= 0x0601 )
//  This structure describes some of the hints we can give to a given B-tree, be it a
//  table, index, or the internal long values tree.
typedef struct tagJET_SPACEHINTS
{
    JET_UINT32          cbStruct;           //  size of this structure
    JET_UINT32          ulInitialDensity;   //  density at (append) layout.
    JET_UINT32          cbInitial;          //  initial size (in bytes).

    JET_GRBIT           grbit;              //  Combination of one or more flags from
                                            //      JET_bitSpaceHints* flags
                                            //      JET_bitCreateHints* flags
                                            //      JET_bitRetrieveHints* flags
                                            //      JET_bitUpdateHints* flags
                                            //      JET_bitDeleteHints* flags
    JET_UINT32          ulMaintDensity;     //  density to maintain at.
    JET_UINT32          ulGrowth;           //  percent growth from:
                                            //    last growth or initial size (possibly rounded to nearest native JET allocation size).
    JET_UINT32          cbMinExtent;        //  This overrides ulGrowth if too small.
    JET_UINT32          cbMaxExtent;        //  This caps ulGrowth.
} JET_SPACEHINTS;
#endif // JET_VERSION >= 0x0601


typedef struct tagJET_INDEXCREATE_A
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PSTR                   szIndexName;         // index name
    JET_PSTR                   szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density

    union
    {
        JET_LCID               lcid;                // lcid for the index (if JET_bitIndexUnicode NOT specified)
        JET_UNICODEINDEX *     pidxunicode;         // pointer to JET_UNICODEINDEX struct (if JET_bitIndexUnicode specified)
    };

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
#if ( JET_VERSION >= 0x0502 )
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
#endif // ! JET_VERSION >= 0x0502
    };

    JET_CONDITIONALCOLUMN_A *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
#if ( JET_VERSION >= 0x0600 )
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
#endif // JET_VERSION >= 0x0600
} JET_INDEXCREATE_A;

typedef struct tagJET_INDEXCREATE_W
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PWSTR                  szIndexName;         // index name
    JET_PWSTR                  szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density

    union
    {
        JET_LCID               lcid;                // lcid for the index (if JET_bitIndexUnicode NOT specified)
        JET_UNICODEINDEX *     pidxunicode;         // pointer to JET_UNICODEINDEX struct (if JET_bitIndexUnicode specified)
    };

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
#if ( JET_VERSION >= 0x0502 )
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
#endif // ! JET_VERSION >= 0x0502
    };

    JET_CONDITIONALCOLUMN_W *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
#if ( JET_VERSION >= 0x0600 )
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
#endif // JET_VERSION >= 0x0600
} JET_INDEXCREATE_W;

#ifdef JET_UNICODE
#define JET_INDEXCREATE JET_INDEXCREATE_W
#else
#define JET_INDEXCREATE JET_INDEXCREATE_A
#endif

#if ( JET_VERSION >= 0x0601 )

typedef struct tagJET_INDEXCREATE2_A
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PSTR                   szIndexName;         // index name
    JET_PSTR                   szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density

    union
    {
        JET_LCID               lcid;                // lcid for the index (if JET_bitIndexUnicode NOT specified)
        JET_UNICODEINDEX *     pidxunicode;         // pointer to JET_UNICODEINDEX struct (if JET_bitIndexUnicode specified)
    };

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
    };

    JET_CONDITIONALCOLUMN_A *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
    JET_SPACEHINTS *           pSpacehints;         // space allocation, maintenance, and usage hints
} JET_INDEXCREATE2_A;

typedef struct tagJET_INDEXCREATE2_W
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PWSTR                  szIndexName;         // index name
    JET_PWSTR                  szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density

    union
    {
        JET_LCID               lcid;                // lcid for the index (if JET_bitIndexUnicode NOT specified)
        JET_UNICODEINDEX *     pidxunicode;         // pointer to JET_UNICODEINDEX struct (if JET_bitIndexUnicode specified)
    };

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
    };

    JET_CONDITIONALCOLUMN_W *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
    JET_SPACEHINTS *           pSpacehints;         // space allocation, maintenance, and usage hints
} JET_INDEXCREATE2_W;

#ifdef JET_UNICODE
#define JET_INDEXCREATE2 JET_INDEXCREATE2_W
#else
#define JET_INDEXCREATE2 JET_INDEXCREATE2_A
#endif
#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0602 )

typedef struct tagJET_INDEXCREATE3_A
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PSTR                   szIndexName;         // index name
    JET_PSTR                   szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density
    JET_UNICODEINDEX2 *        pidxunicode;         // pointer to JET_UNICODEINDEX2 struct (if JET_bitIndexUnicode specified)

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
    };

    JET_CONDITIONALCOLUMN_A *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
    JET_SPACEHINTS *           pSpacehints;         // space allocation, maintenance, and usage hints
} JET_INDEXCREATE3_A;

typedef struct tagJET_INDEXCREATE3_W
{
    JET_UINT32                 cbStruct;            // size of this structure (for future expansion)
    JET_PWSTR                  szIndexName;         // index name
    JET_PWSTR                  szKey;               // index key definition
    JET_UINT32                 cbKey;               // size of key definition in szKey
    JET_GRBIT                  grbit;               // index options
    JET_UINT32                 ulDensity;           // index density
    JET_UNICODEINDEX2 *        pidxunicode;         // pointer to JET_UNICODEINDEX2 struct (if JET_bitIndexUnicode specified)

    union
    {
        JET_UINT32             cbVarSegMac;         // maximum length of variable length columns in index key (if JET_bitIndexTupleLimits not specified)
        JET_TUPLELIMITS *      ptuplelimits;        // pointer to JET_TUPLELIMITS struct (if JET_bitIndexTupleLimits specified)
    };

    JET_CONDITIONALCOLUMN_W *  rgconditionalcolumn; // pointer to conditional column structure
    JET_UINT32                 cConditionalColumn;  // number of conditional columns
    JET_ERR                    err;                 // returned error code
    JET_UINT32                 cbKeyMost;           // size of key preserved in index, e.g. without truncation (if JET_bitIndexKeyMost specified)
    JET_SPACEHINTS *           pSpacehints;         // space allocation, maintenance, and usage hints
} JET_INDEXCREATE3_W;

#ifdef JET_UNICODE
#define JET_INDEXCREATE3 JET_INDEXCREATE3_W
#else
#define JET_INDEXCREATE3 JET_INDEXCREATE3_A
#endif
#endif // JET_VERSION >= 0x0602

//
//      Table Creation Structures
//

typedef struct tagJET_TABLECREATE_A
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PSTR              szTableName;          // name of table to create.
    JET_PSTR              szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_A *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE_A *   rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_GRBIT             grbit;
    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes).
} JET_TABLECREATE_A;

typedef struct tagJET_TABLECREATE_W
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PWSTR             szTableName;          // name of table to create.
    JET_PWSTR             szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_W *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE_W *   rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_GRBIT             grbit;
    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes).
} JET_TABLECREATE_W;

#ifdef JET_UNICODE
#define JET_TABLECREATE JET_TABLECREATE_W
#else
#define JET_TABLECREATE JET_TABLECREATE_A
#endif

#if ( JET_VERSION >= 0x0501 )
typedef struct tagJET_TABLECREATE2_A
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PSTR              szTableName;          // name of table to create.
    JET_PSTR              szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_A *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE_A *   rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PSTR              szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE2_A;

typedef struct tagJET_TABLECREATE2_W
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PWSTR             szTableName;          // name of table to create.
    JET_PWSTR             szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_W *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE_W *   rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PWSTR             szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE2_W;

#ifdef JET_UNICODE
#define JET_TABLECREATE2 JET_TABLECREATE2_W
#else
#define JET_TABLECREATE2 JET_TABLECREATE2_A
#endif

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION >= 0x0601 )
typedef struct tagJET_TABLECREATE3_A
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PSTR              szTableName;          // name of table to create.
    JET_PSTR              szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_A *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE2_A *  rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PSTR              szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_SPACEHINTS *      pSeqSpacehints;       // space allocation, maintenance, and usage hints for default sequential index
    JET_SPACEHINTS *      pLVSpacehints;        // space allocation, maintenance, and usage hints for Separated LV tree.
    JET_UINT32            cbSeparateLV;         // heuristic size to separate a intrinsic LV from the primary record

    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE3_A;

typedef struct tagJET_TABLECREATE3_W
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PWSTR             szTableName;          // name of table to create.
    JET_PWSTR             szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_W *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE2_W *  rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PWSTR             szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_SPACEHINTS *      pSeqSpacehints;       // space allocation, maintenance, and usage hints for default sequential index
    JET_SPACEHINTS *      pLVSpacehints;        // space allocation, maintenance, and usage hints for Separated LV tree.
    JET_UINT32            cbSeparateLV;         // heuristic size to separate a intrinsic LV from the primary record
    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE3_W;

#ifdef JET_UNICODE
#define JET_TABLECREATE3 JET_TABLECREATE3_W
#else
#define JET_TABLECREATE3 JET_TABLECREATE3_A
#endif

#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0602 )
typedef struct tagJET_TABLECREATE4_A
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PSTR              szTableName;          // name of table to create.
    JET_PSTR              szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_A *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE3_A *  rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PSTR              szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_SPACEHINTS *      pSeqSpacehints;       // space allocation, maintenance, and usage hints for default sequential index
    JET_SPACEHINTS *      pLVSpacehints;        // space allocation, maintenance, and usage hints for Separated LV tree.
    JET_UINT32            cbSeparateLV;         // heuristic size to separate a intrinsic LV from the primary record

    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE4_A;

typedef struct tagJET_TABLECREATE4_W
{
    JET_UINT32            cbStruct;             // size of this structure (for future expansion)
    JET_PWSTR             szTableName;          // name of table to create.
    JET_PWSTR             szTemplateTableName;  // name of table from which to inherit base DDL
    JET_UINT32            ulPages;              // initial pages to allocate for table.
    JET_UINT32            ulDensity;            // table density.
    JET_COLUMNCREATE_W *  rgcolumncreate;       // array of column creation info
    JET_UINT32            cColumns;             // number of columns to create
    JET_INDEXCREATE3_W *  rgindexcreate;        // array of index creation info
    JET_UINT32            cIndexes;             // number of indexes to create
    JET_PWSTR             szCallback;           // callback to use for this table
    JET_CBTYP             cbtyp;                // when the callback should be called
    JET_GRBIT             grbit;
    JET_SPACEHINTS *      pSeqSpacehints;       // space allocation, maintenance, and usage hints for default sequential index
    JET_SPACEHINTS *      pLVSpacehints;        // space allocation, maintenance, and usage hints for Separated LV tree.
    JET_UINT32            cbSeparateLV;         // heuristic size to separate a intrinsic LV from the primary record

    JET_TABLEID           tableid;              // returned tableid.
    JET_UINT32            cCreated;             // count of objects created (columns+table+indexes+callbacks).
} JET_TABLECREATE4_W;

#ifdef JET_UNICODE
#define JET_TABLECREATE4 JET_TABLECREATE4_W
#else
#define JET_TABLECREATE4 JET_TABLECREATE4_A
#endif

#endif // JET_VERSION >= 0x0602


#if ( JET_VERSION >= 0x0600 )
typedef struct tagJET_OPENTEMPORARYTABLE
{
    JET_UINT32             cbStruct;            // size of this structure (for future expansion)
    const JET_COLUMNDEF *  prgcolumndef;
    JET_UINT32             ccolumn;
    JET_UNICODEINDEX *     pidxunicode;
    JET_GRBIT              grbit;
    JET_COLUMNID *         prgcolumnid;
    JET_UINT32             cbKeyMost;
    JET_UINT32             cbVarSegMac;
    JET_TABLEID            tableid;
} JET_OPENTEMPORARYTABLE;
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0602 )
typedef struct tagJET_OPENTEMPORARYTABLE2
{
    JET_UINT32             cbStruct;            // size of this structure (for future expansion)
    const JET_COLUMNDEF *  prgcolumndef;
    JET_UINT32             ccolumn;
    JET_UNICODEINDEX2 *    pidxunicode;
    JET_GRBIT              grbit;
    JET_COLUMNID *         prgcolumnid;
    JET_UINT32             cbKeyMost;
    JET_UINT32             cbVarSegMac;
    JET_TABLEID            tableid;
} JET_OPENTEMPORARYTABLE2;
#endif // JET_VERSION >= 0x0602

typedef struct
{
    JET_UINT32      cbStruct;
    JET_UINT32      ibLongValue;
    JET_UINT32      itagSequence;
    JET_COLUMNID    columnidNextTagged;
} JET_RETINFO;

typedef struct
{
    JET_UINT32      cbStruct;
    JET_UINT32      ibLongValue;
    JET_UINT32      itagSequence;
} JET_SETINFO;

typedef struct
{
    JET_UINT32      cbStruct;
    JET_UINT32      centriesLT;
    JET_UINT32      centriesInRange;
    JET_UINT32      centriesTotal;
} JET_RECPOS;

// On input to JetGotoPosition, centriesLTDeprecated and centriesTotalDeprecated must be 0.
// On output from JetGetRecordPositon, centriesLTDeprecated and centriesTotalDeprecated
// hold potentially truncated versions of centriesLT and centriesTotal.
typedef struct
{
    JET_UINT32           cbStruct;
    JET_UINT32           centriesLTDeprecated;
    JET_UINT32           centriesInRangeDeprecated;
    JET_UINT32           centriesTotalDeprecated;
    JET_UINT64           centriesLT;
    JET_UINT64           centriesTotal;
} JET_RECPOS2;

typedef struct
{
    JET_UINT32      cbStruct;
    JET_TABLEID     tableid;
    JET_UINT32      cRecord;
    JET_COLUMNID    columnidBookmark;
} JET_RECORDLIST;

typedef struct
{
    JET_UINT32      cbStruct;
    JET_TABLEID     tableid;
    JET_GRBIT       grbit;
} JET_INDEXRANGE;

#if ( JET_VERSION >= 0x0602 )
typedef enum
{
    // can be used for index (JetPrereadIndexRanges) or residual predicate (JetSetCursorFilter)
    JET_relopEquals = 0,
    JET_relopPrefixEquals,

    // can only be used for residual predicate (JetSetCursorFilter)
    JET_relopNotEquals,
    JET_relopLessThanOrEqual,
    JET_relopLessThan,
    JET_relopGreaterThanOrEqual,
    JET_relopGreaterThan,
    JET_relopBitmaskEqualsZero,
    JET_relopBitmaskNotEqualsZero,
} JET_RELOP;

typedef struct
{
    JET_COLUMNID    columnid;   //  columnid of the column
    JET_RELOP       relop;      //  relational operator
    JET_PVOID       pv;         //  pointer to the value to use
    JET_UINT32      cb;         //  size of the value to use
    JET_GRBIT       grbit;      //  optional grbits
} JET_INDEX_COLUMN;

typedef struct
{
    JET_INDEX_COLUMN *  rgStartColumns;
    JET_UINT32          cStartColumns;
    JET_INDEX_COLUMN *  rgEndColumns;
    JET_UINT32          cEndColumns;
} JET_INDEX_RANGE;
#endif  //  JET_VERSION >= 0x0602



#pragma pack(push,1)
#define JET_MAX_COMPUTERNAME_LENGTH 15

typedef struct
{
    JET_INT8    bSeconds;               //  0 - 59
    JET_INT8    bMinutes;               //  0 - 59
    JET_INT8    bHours;                 //  0 - 23
    JET_INT8    bDay;                   //  1 - 31
    JET_INT8    bMonth;                 //  1 - 12
    JET_INT8    bYear;                  //  current year - 1900
    union
    {
        JET_BYTE    bFiller1;
        struct
        {
            JET_BYTE      fTimeIsUTC:1;
            JET_BYTE      bMillisecondsLow:7;
        };
    };
    union
    {
        JET_BYTE    bFiller2;
        struct
        {
            JET_BYTE      fReserved:1;
            JET_BYTE      bMillisecondsHigh:3;
            JET_BYTE      fUnused:4;
        };
    };
} JET_LOGTIME;

#if ( JET_VERSION >= 0x0600 )
// the JET_BKLOGTIME is an extention of JET_LOGTIME to be used
// in the JET_BKINFO structure. They should have the same size for
// compatibility reasons
typedef struct
{
    JET_INT8    bSeconds;               //  0 - 59
    JET_INT8    bMinutes;               //  0 - 59
    JET_INT8    bHours;                 //  0 - 23
    JET_INT8    bDay;                   //  1 - 31
    JET_INT8    bMonth;                 //  1 - 12
    JET_INT8    bYear;                  //  current year - 1900
    union
    {
        JET_BYTE    bFiller1;
        struct
        {
            JET_BYTE      fTimeIsUTC:1;
            JET_BYTE      bMillisecondsLow:7;
        };
    };
    union
    {
        JET_BYTE    bFiller2;
        struct
        {
            JET_BYTE      fOSSnapshot:1;
            JET_BYTE      bMillisecondsHigh:3;
            JET_BYTE      fReserved:4;
        };
    };
} JET_BKLOGTIME;
#endif // JET_VERSION >= 0x0600

typedef struct
{
    JET_UINT16      ib;             // must be the last so that lgpos can
    JET_UINT16      isec;           // index of disksec starting logsec
    JET_INT32       lGeneration;    // generation of logsec
} JET_LGPOS;                    // be casted to TIME.

typedef struct
{
    JET_UINT32      ulRandom;           //  a random number
    JET_LOGTIME     logtimeCreate;      //  time db created, in logtime format
    JET_CHAR        szComputerName[ JET_MAX_COMPUTERNAME_LENGTH + 1 ];  // where db is created
} JET_SIGNATURE;

typedef struct
{
    JET_LGPOS       lgposMark;          //  id for this backup
    union
    {
        JET_LOGTIME     logtimeMark;
#if ( JET_VERSION >= 0x0600 )
        JET_BKLOGTIME   bklogtimeMark;
#endif // JET_VERSION >= 0x0600
    };
    JET_UINT32      genLow;
    JET_UINT32      genHigh;
} JET_BKINFO;

#pragma pack(pop)

typedef struct
{
    JET_UINT32      ulVersion;      //  the major (incompatible) version of DAE from the last engine attach/create.
    JET_UINT32      ulUpdate;       //  used to track incremental database format "update (major)" version from the
                                    //  last attach/create that is a backward-compatible major update.
    JET_SIGNATURE   signDb;         //  (28 bytes) signature of the db (incl. creation time).
    JET_UINT32      dbstate;        //  consistent/inconsistent state

    JET_LGPOS       lgposConsistent;    //  null if in inconsistent state
    JET_LOGTIME     logtimeConsistent;  // null if in inconsistent state

    JET_LOGTIME     logtimeAttach;  //  Last attach time.
    JET_LGPOS       lgposAttach;

    JET_LOGTIME     logtimeDetach;  //  Last detach time.
    JET_LGPOS       lgposDetach;

    JET_SIGNATURE   signLog;        //  (28 bytes) log signature for this attachments

    JET_BKINFO      bkinfoFullPrev; //  Last successful full backup.

    JET_BKINFO      bkinfoIncPrev;  //  Last successful Incremental backup.
                                    //  Reset when bkinfoFullPrev is set
    JET_BKINFO      bkinfoFullCur;  //  current backup. Succeed if a
                                    //  corresponding pat file generated.
    JET_UINT32      fShadowingDisabled;
    JET_UINT32      fUpgradeDb;

    //  NT version information. This is needed to decide if an index need
    //  be recreated due to sort table changes.

    JET_UINT32      dwMajorVersion;     //  OS version info
    JET_UINT32      dwMinorVersion;
    JET_UINT32      dwBuildNumber;
    JET_INT32       lSPNumber;

    JET_UINT32      cbPageSize;         //  database page size (0 = 4k pages)

} JET_DBINFOMISC;

#if ( JET_VERSION >= 0x0600 )
typedef struct
{
    JET_UINT32      ulVersion;      //  the major (incompatible) version of DAE from the last engine attach/create.
    JET_UINT32      ulUpdate;       //  used to track incremental database format "update (major)" version from the
                                    //  last attach/create that is a backward-compatible major update.
    JET_SIGNATURE   signDb;         //  (28 bytes) signature of the db (incl. creation time).
    JET_UINT32      dbstate;        //  consistent/inconsistent state

    JET_LGPOS       lgposConsistent;    //  null if in inconsistent state
    JET_LOGTIME     logtimeConsistent;  // null if in inconsistent state

    JET_LOGTIME     logtimeAttach;  //  Last attach time.
    JET_LGPOS       lgposAttach;

    JET_LOGTIME     logtimeDetach;  //  Last detach time.
    JET_LGPOS       lgposDetach;

    JET_SIGNATURE   signLog;        //  (28 bytes) log signature for this attachments

    JET_BKINFO      bkinfoFullPrev; //  Last successful full backup.

    JET_BKINFO      bkinfoIncPrev;  //  Last successful Incremental backup.
                                    //  Reset when bkinfoFullPrev is set
    JET_BKINFO      bkinfoFullCur;  //  current backup. Succeed if a
                                    //  corresponding pat file generated.
    JET_UINT32      fShadowingDisabled;
    JET_UINT32      fUpgradeDb;

    //  NT version information. This is needed to decide if an index need
    //  be recreated due to sort table changes.

    JET_UINT32      dwMajorVersion;     //  OS version info                            
    JET_UINT32      dwMinorVersion;
    JET_UINT32      dwBuildNumber;
    JET_INT32       lSPNumber;

    JET_UINT32      cbPageSize;         //  database page size (0 = 4k pages)

    // new fields added on top of the above JET_DBINFOMISC
    JET_UINT32      genMinRequired;         //  the minimum log generation required for replaying the logs. Typically the checkpoint generation
    JET_UINT32      genMaxRequired;         //  the maximum log generation required for replaying the logs.
    JET_LOGTIME     logtimeGenMaxCreate;    //  creation time of the genMax log file

    JET_UINT32      ulRepairCount;          //  number of times repair has been called on this database
    JET_LOGTIME     logtimeRepair;          //  the date of the last time that repair was run
    JET_UINT32      ulRepairCountOld;       //  number of times ErrREPAIRAttachForRepair has been called on this database before the last defrag

    JET_UINT32      ulECCFixSuccess;        //  number of times a one bit error was fixed and resulted in a good page
    JET_LOGTIME     logtimeECCFixSuccess;   //  the date of the last time that a one bit error was fixed and resulted in a good page
    JET_UINT32      ulECCFixSuccessOld;     //  number of times a one bit error was fixed and resulted in a good page before last repair

    JET_UINT32      ulECCFixFail;           //  number of times a one bit error was fixed and resulted in a bad page
    JET_LOGTIME     logtimeECCFixFail;      //  the date of the last time that a one bit error was fixed and resulted in a bad page
    JET_UINT32      ulECCFixFailOld;        //  number of times a one bit error was fixed and resulted in a bad page before last repair

    JET_UINT32      ulBadChecksum;          //  number of times a non-correctable ECC/checksum error was found
    JET_LOGTIME     logtimeBadChecksum;     //  the date of the last time that a non-correctable ECC/checksum error was found
    JET_UINT32      ulBadChecksumOld;       //  number of times a non-correctable ECC/checksum error was found before last repair

} JET_DBINFOMISC2;
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0601 )
typedef struct
{
    JET_UINT32      ulVersion;      //  the major (incompatible) version of DAE from the last engine attach/create.
    JET_UINT32      ulUpdate;       //  used to track incremental database format "update (major)" version from the
                                    //  last attach/create that is a backward-compatible major update.
    JET_SIGNATURE   signDb;         //  (28 bytes) signature of the db (incl. creation time).
    JET_UINT32      dbstate;        //  consistent/inconsistent state

    JET_LGPOS       lgposConsistent;    //  null if in inconsistent state
    JET_LOGTIME     logtimeConsistent;  // null if in inconsistent state

    JET_LOGTIME     logtimeAttach;  //  Last attach time.
    JET_LGPOS       lgposAttach;

    JET_LOGTIME     logtimeDetach;  //  Last detach time.
    JET_LGPOS       lgposDetach;

    JET_SIGNATURE   signLog;        //  (28 bytes) log signature for this attachments

    JET_BKINFO      bkinfoFullPrev; //  Last successful full backup.

    JET_BKINFO      bkinfoIncPrev;  //  Last successful Incremental backup.
                                    //  Reset when bkinfoFullPrev is set
    JET_BKINFO      bkinfoFullCur;  //  current backup. Succeed if a
                                    //  corresponding pat file generated.
    JET_UINT32      fShadowingDisabled;
    JET_UINT32      fUpgradeDb;

    //  NT version information. This is needed to decide if an index need
    //  be recreated due to sort table changes.

    JET_UINT32      dwMajorVersion;     //  OS version info                            
    JET_UINT32      dwMinorVersion;
    JET_UINT32      dwBuildNumber;
    JET_INT32       lSPNumber;

    JET_UINT32      cbPageSize;         //  database page size (0 = 4k pages)

    // new fields added on top of the above JET_DBINFOMISC
    JET_UINT32      genMinRequired;         //  the minimum log generation required for replaying the logs. Typically the checkpoint generation
    JET_UINT32      genMaxRequired;         //  the maximum log generation required for replaying the logs.
    JET_LOGTIME     logtimeGenMaxCreate;    //  creation time of the genMax log file

    JET_UINT32      ulRepairCount;          //  number of times repair has been called on this database
    JET_LOGTIME     logtimeRepair;          //  the date of the last time that repair was run
    JET_UINT32      ulRepairCountOld;       //  number of times ErrREPAIRAttachForRepair has been called on this database before the last defrag

    JET_UINT32      ulECCFixSuccess;        //  number of times a one bit error was fixed and resulted in a good page
    JET_LOGTIME     logtimeECCFixSuccess;   //  the date of the last time that a one bit error was fixed and resulted in a good page
    JET_UINT32      ulECCFixSuccessOld;     //  number of times a one bit error was fixed and resulted in a good page before last repair

    JET_UINT32      ulECCFixFail;           //  number of times a one bit error was fixed and resulted in a bad page
    JET_LOGTIME     logtimeECCFixFail;      //  the date of the last time that a one bit error was fixed and resulted in a bad page
    JET_UINT32      ulECCFixFailOld;        //  number of times a one bit error was fixed and resulted in a bad page before last repair

    JET_UINT32      ulBadChecksum;          //  number of times a non-correctable ECC/checksum error was found
    JET_LOGTIME     logtimeBadChecksum;     //  the date of the last time that a non-correctable ECC/checksum error was found
    JET_UINT32      ulBadChecksumOld;       //  number of times a non-correctable ECC/checksum error was found before last repair

    // new fields added on top of the above JET_DBINFOMISC2
    JET_UINT32      genCommitted;           //  the maximum log generation committed to the database. Typically the current log generation

} JET_DBINFOMISC3;

typedef struct
{
    JET_UINT32      ulVersion;      //  the major (incompatible) version of DAE from the last engine attach/create.
    JET_UINT32      ulUpdate;       //  used to track incremental database format "update (major)" version from the
                                    //  last attach/create that is a backward-compatible major update.
    JET_SIGNATURE   signDb;         //  (28 bytes) signature of the db (incl. creation time).
    JET_UINT32      dbstate;        //  consistent/inconsistent state

    JET_LGPOS       lgposConsistent;    //  null if in inconsistent state
    JET_LOGTIME     logtimeConsistent;  // null if in inconsistent state

    JET_LOGTIME     logtimeAttach;  //  Last attach time.
    JET_LGPOS       lgposAttach;

    JET_LOGTIME     logtimeDetach;  //  Last detach time.
    JET_LGPOS       lgposDetach;

    JET_SIGNATURE   signLog;        //  (28 bytes) log signature for this attachments

    JET_BKINFO      bkinfoFullPrev; //  Last successful full backup.

    JET_BKINFO      bkinfoIncPrev;  //  Last successful Incremental backup.
                                    //  Reset when bkinfoFullPrev is set
    JET_BKINFO      bkinfoFullCur;  //  current backup. Succeed if a
                                    //  corresponding pat file generated.
    JET_UINT32      fShadowingDisabled;
    JET_UINT32      fUpgradeDb;

    //  NT version information. This is needed to decide if an index need
    //  be recreated due to sort table changes.

    JET_UINT32      dwMajorVersion;     //  OS version info                            
    JET_UINT32      dwMinorVersion;
    JET_UINT32      dwBuildNumber;
    JET_INT32       lSPNumber;

    JET_UINT32      cbPageSize;         //  database page size (0 = 4k pages)

    // new fields added on top of the above JET_DBINFOMISC
    JET_UINT32      genMinRequired;         //  the minimum log generation required for replaying the logs. Typically the checkpoint generation
    JET_UINT32      genMaxRequired;         //  the maximum log generation required for replaying the logs.
    JET_LOGTIME     logtimeGenMaxCreate;    //  creation time of the genMax log file

    JET_UINT32      ulRepairCount;          //  number of times repair has been called on this database
    JET_LOGTIME     logtimeRepair;          //  the date of the last time that repair was run
    JET_UINT32      ulRepairCountOld;       //  number of times ErrREPAIRAttachForRepair has been called on this database before the last defrag

    JET_UINT32      ulECCFixSuccess;        //  number of times a one bit error was fixed and resulted in a good page
    JET_LOGTIME     logtimeECCFixSuccess;   //  the date of the last time that a one bit error was fixed and resulted in a good page
    JET_UINT32      ulECCFixSuccessOld;     //  number of times a one bit error was fixed and resulted in a good page before last repair

    JET_UINT32      ulECCFixFail;           //  number of times a one bit error was fixed and resulted in a bad page
    JET_LOGTIME     logtimeECCFixFail;      //  the date of the last time that a one bit error was fixed and resulted in a bad page
    JET_UINT32      ulECCFixFailOld;        //  number of times a one bit error was fixed and resulted in a bad page before last repair

    JET_UINT32      ulBadChecksum;          //  number of times a non-correctable ECC/checksum error was found
    JET_LOGTIME     logtimeBadChecksum;     //  the date of the last time that a non-correctable ECC/checksum error was found
    JET_UINT32      ulBadChecksumOld;       //  number of times a non-correctable ECC/checksum error was found before last repair

    // new fields added on top of the above JET_DBINFOMISC2
    JET_UINT32      genCommitted;           //  the maximum log generation committed to the database. Typically the current log generation

    // new fields added on top of the above JET_DBINFOMISC3
    JET_BKINFO  bkinfoCopyPrev;         //  Last successful Copy backup
    JET_BKINFO  bkinfoDiffPrev;         //  Last successful Differential backup, reset when bkinfoFullPrev is set
} JET_DBINFOMISC4;
#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0600 )
//  JET performance counters accumulated by thread
//
struct JET_THREADSTATS
{
    JET_UINT32      cbStruct;           //  size of this struct
    JET_UINT32      cPageReferenced;    //  pages referenced
    JET_UINT32      cPageRead;          //  pages read from disk
    JET_UINT32      cPagePreread;       //  pages preread from disk
    JET_UINT32      cPageDirtied;       //  clean pages modified
    JET_UINT32      cPageRedirtied;     //  dirty pages modified
    JET_UINT32      cLogRecord;         //  log records generated
    JET_UINT32      cbLogRecord;        //  log record bytes generated
};
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0A00 )
//  JET performance counters accumulated by thread
//
struct JET_THREADSTATS2
{
    JET_UINT32          cbStruct;               //  size of this struct
    JET_UINT32          cPageReferenced;        //  pages referenced
    JET_UINT32          cPageRead;              //  pages read from disk
    JET_UINT32          cPagePreread;           //  pages preread from disk
    JET_UINT32          cPageDirtied;           //  clean pages modified
    JET_UINT32          cPageRedirtied;         //  dirty pages modified
    JET_UINT32          cLogRecord;             //  log records generated
    JET_UINT32          cbLogRecord;            //  log record bytes generated
    JET_UINT64          cusecPageCacheMiss;     //  page cache miss latency in microseconds
    JET_UINT32          cPageCacheMiss;         //  page cache misses
};
#endif // JET_VERSION >= 0x0A00

#if ( JET_VERSION >= 0x0A01 )
//  JET performance counters accumulated by thread
//
struct JET_THREADSTATS3
{
    JET_UINT32          cbStruct;                       //  size of this struct
    JET_UINT32          cPageReferenced;                //  pages referenced
    JET_UINT32          cPageRead;                      //  pages read from disk
    JET_UINT32          cPagePreread;                   //  pages preread from disk
    JET_UINT32          cPageDirtied;                   //  clean pages modified
    JET_UINT32          cPageRedirtied;                 //  dirty pages modified
    JET_UINT32          cLogRecord;                     //  log records generated
    JET_UINT32          cbLogRecord;                    //  log record bytes generated
    JET_UINT64          cusecPageCacheMiss;             //  page cache miss latency in microseconds
    JET_UINT32          cPageCacheMiss;                 //  page cache misses
    JET_UINT32          cSeparatedLongValueRead;        //  separated LV reads
    JET_UINT64          cusecLongValuePageCacheMiss;    //  page cache miss latency in microseconds while reading separated LV data
    JET_UINT32          cLongValuePageCacheMiss;        //  page cache misses while reading separated LV data
};
#endif // JET_VERSION >= 0x0A01

#if ( JET_VERSION >= 0x0600 )

typedef struct
{
    JET_UINT32              cbStruct;

    JET_RSTMAP_A *          rgrstmap;
    JET_INT32               crstmap;

    JET_LGPOS               lgposStop;
    JET_LOGTIME             logtimeStop;

    JET_PFNSTATUS           pfnStatus;
} JET_RSTINFO_A;

typedef struct
{
    JET_UINT32              cbStruct;

    JET_RSTMAP_W *          rgrstmap;
    JET_INT32               crstmap;

    JET_LGPOS               lgposStop;
    JET_LOGTIME             logtimeStop;

    JET_PFNSTATUS           pfnStatus;
} JET_RSTINFO_W;

#ifdef JET_UNICODE
#define JET_RSTINFO JET_RSTINFO_W
#else
#define JET_RSTINFO JET_RSTINFO_A
#endif

#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0602 )

// JET_errcatError
//    |
//    |-- JET_errcatOperation
//    |     |-- JET_errcatFatal
//    |     |-- JET_errcatIO                //  bad IO issues, may or may not be transient.
//    |     |-- JET_errcatResource
//    |           |-- JET_errcatMemory      //  out of memory (all variants)
//    |           |-- JET_errcatQuota
//    |           |-- JET_errcatDisk        //  out of disk space (all variants)
//    |-- JET_errcatData
//    |     |-- JET_errcatCorruption
//    |     |-- JET_errcatInconsistent      //  typically caused by user Mishandling
//    |     |-- JET_errcatFragmentation
//    |-- JET_errcatApi
//          |-- JET_errcatUsage
//          |-- JET_errcatState


// A brief description of each error type
//
//  Operation(al) - Errors that can usually happen any time due to uncontrollable
//                  conditions.  Frequently temporary, but not always.
//
//                  Recovery: Probably retry, or eventually inform the operator.
//
//      Fatal -     This sort error happens only when ESE encounters an error condition
//                  so grave, that we can not continue on in a safe (often transactional)
//                  way, and rather than corrupt data we throw errors of this category.
//
//                  Recovery: Restart the instance or process.  If the problem persists
//                  inform the operator.
//
//      IO -        IO errors come from the OS, and are out of ESE's control, this sort
//                  of error is possibly temporary, possibly not.
//
//                  Recovery: Retry.  If not resolved, ask operator about disk issue.
//
//      Resource -  This is a category that indicates one of many potential out-of-resource
//                  conditions.
//
//          Memory  Classic out of memory condition.
//
//                  Recovery: Wait a while and retry, free up memory, or quit.
//
//          Quota   Certain "specialty" resources are in pools of a certain size, making
//                  it easier to detect leaks of these resources.
//
//                  Recovery: Bug fix, generally the application should Assert() on these
//                  conditions so as to detect these issues during development.  However,
//                  in retail code, the best to hope for is to treat like Memory.
//
//          Disk    Out of disk conditions.
//
//                  Recovery: Can retry later in the hope more space is available, or
//                  ask the operator to free some disk space.
//  Data
//
//      Corruption  My hard drive ate my homework.  Classic corruption issues, frequently
//                  permanent without corrective action.
//
//                  Recovery: Restore from backup, perhaps the ese utilities repair
//                  operation (which only salvages what data is left / lossy).  Also
//                  in the case of recovery(JetInit) perhaps recovery can be performed
//                  by allowing data loss.
//
//      Inconsistent This is similar to Corruption in that the database and/or log files
//                  are in a state that is inconsistent and unreconcilable with each
//                  other. Often this is caused by application/administrator mishandling.
//
//                  Recovery: Restore from backup, perhaps the ese utilities repair
//                  operation (which only salvages what data is left / lossy).  Also
//                  in the case of recovery(JetInit) perhaps recovery can be performed
//                  by allowing data loss.
//
//      Fragmentation   This is a class of errors where some persisted internal resource ran
//                  out.
//
//                  Recovery: For database errors, offline defragmentation will rectify
//                  the problem, for the log files _first_ recover all attached databases
//                  to a clean shutdown, and then delete all the log files and checkpoint.
//
//  Api
//
//      Usage       Classic usage error, this means the client code did not pass correct
//                  arguments to the JET API.  This error will likely not go away with
//                  retry.
//
//                  Recovery: Generally speaking client code should Assert() this class
//                  of errors is not returned, so issues can be caught during development.
//                  In retail, the app will probably have little option but to return
//                  the issue up to the operator.
//
//      State       This is the classification for different signals the API could return
//                  describe the state of the database, a classic case is JET_errRecordNotFound
//                  which can be returned by JetSeek() when the record you asked for
//                  was not found.
//
//                  Recovery: Not really relevant, depends greatly on the API.
//

typedef enum
{
    JET_errcatUnknown = 0,  //      unknown, error retrieving err category
    JET_errcatError,        //      top level (no errors should be of this class)
    JET_errcatOperation,
    JET_errcatFatal,
    JET_errcatIO,           //      bad IO issues, may or may not be transient.
    JET_errcatResource,
    JET_errcatMemory,       //      out of memory (all variants)
    JET_errcatQuota,
    JET_errcatDisk,         //      out of disk space (all variants)
    JET_errcatData,
    JET_errcatCorruption,
    JET_errcatInconsistent,
    JET_errcatFragmentation,
    JET_errcatApi,
    JET_errcatUsage,
    JET_errcatState,
    JET_errcatObsolete,
    JET_errcatMax,
} JET_ERRCAT;

// Output structure for JetGetErrorInfoW(). Not all fields may
// be populated by all error levels.
typedef struct
{
    JET_UINT32          cbStruct;
    JET_ERR             errValue;                   //  The error value for the requested info level.
    JET_ERRCAT          errcatMostSpecific;         //  The most specific category of the error.
    JET_BYTE            rgCategoricalHierarchy[8];  //  Hierarchy of error categories. Position 0 is the highest level in the hierarchy, and the rest are JET_errcatUnknown.
    JET_UINT32          lSourceLine;                //  The source file line for the requested info level.
    JET_WCHAR           rgszSourceFile[64];         //  The source file name for the requested info level.
} JET_ERRINFOBASIC_W;

// grbits for JET_PFNDURABLECOMMITCALLBACK
#if ( JET_VERSION >= 0x0A00 )
#define JET_bitDurableCommitCallbackLogUnavailable      0x00000001  // Passed back to durable commit callback to let it know that log is down (and all pending commits will not be flushed to disk)
#endif

// commit-id from JetCommitTransaction2
typedef struct
{
    JET_SIGNATURE   signLog;
    JET_INT32       reserved; // for packing so int64 below is 8-byte aligned on 32-bits despite the pshpack4 above
    JET_INT64       commitId;
} JET_COMMIT_ID;

// assert that commit-id is 8-byte aligned so managed interop works correctly
// static_assert( offsetof( JET_COMMIT_ID, commitId ) % 8 == 0 );

// callback for JET_paramDurableCommitCallback
typedef JET_ERR (JET_API * JET_PFNDURABLECOMMITCALLBACK)(
    _In_ JET_INSTANCE     instance,
    _In_ JET_COMMIT_ID *  pCommitIdSeen,
    _In_ JET_GRBIT        grbit );

#endif // JET_VERSION >= 0x0602

/************************************************************************/
/*************************     JET CONSTANTS     ************************/
/************************************************************************/

#if ( JET_VERSION >= 0x0501 )
#define JET_instanceNil             (~(JET_INSTANCE)0)
#endif // JET_VERSION >= 0x0501
#define JET_sesidNil                (~(JET_SESID)0)
#define JET_tableidNil              (~(JET_TABLEID)0)
#define JET_bitNil                  ((JET_GRBIT)0)

    /* Max size of a bookmark */

#define JET_cbBookmarkMost          256
#if ( JET_VERSION >= 0x0601 )
#define JET_cbBookmarkMostMost      JET_cbKeyMostMost
#endif // JET_VERSION >= 0x0601

    /* Max length of a object/column/index/property name */

#ifndef JET_UNICODE
#define JET_cbNameMost              64
#else
#define JET_cbNameMost              128
#endif

    /* Max length of a "name.name.name..." construct */

#ifndef JET_UNICODE
#define JET_cbFullNameMost          255
#else
#define JET_cbFullNameMost          510
#endif

    /* Max size of long-value (LongBinary or LongText) column chunk */

//  #define JET_cbColumnLVChunkMost     ( JET_cbPage - 82 ) to the following:
//  Get cbPage from GetSystemParameter.
//  changed JET_cbColumnLVChunkMost reference to cbPage - JET_cbColumnLVPageOverhead

#define JET_cbColumnLVPageOverhead      82      // ONLY for small (<=8kiB) page, otherwise, query JET_paramLVChunkSizeMost


    /* Max size of long-value (LongBinary or LongText) column default value */

#define JET_cbLVDefaultValueMost    255

    /* Max size of non-long-value column data */

#define JET_cbColumnMost            255

    /* Max size of long-value column data. */

#define JET_cbLVColumnMost          0x7FFFFFFF

    /* Max size of a sort/index key */

#if ( JET_VERSION >= 0x0601 )
#define JET_cbKeyMostMost               JET_cbKeyMost32KBytePage
#define JET_cbKeyMost32KBytePage        JET_cbKeyMost8KBytePage
#define JET_cbKeyMost16KBytePage        JET_cbKeyMost8KBytePage
#endif // JET_VERSION >= 0x0601
#if ( JET_VERSION >= 0x0600 )
#define JET_cbKeyMost8KBytePage     2000
#define JET_cbKeyMost4KBytePage     1000
#define JET_cbKeyMost2KBytePage     500
#define JET_cbKeyMostMin            255
#endif // JET_VERSION >= 0x0600

#define JET_cbKeyMost               255     //  defunct constant retained for backward compatibility
#define JET_cbLimitKeyMost          256     //  defunct constant retained for backward compatibility
#define JET_cbPrimaryKeyMost        255     //  defunct constant retained for backward compatibility
#define JET_cbSecondaryKeyMost      255     //  defunct constant retained for backward compatibility


    /* Max number of components in a sort/index key */

#if ( JET_VERSION >= 0x0600 )
#define JET_ccolKeyMost             16
#else // !JET_VERSION >= 0x0600
#define JET_ccolKeyMost             12
#endif // !JET_VERSION >= 0x0600

//  maximum number of columns
#if ( JET_VERSION >= 0x0501 )
#define JET_ccolMost                0x0000fee0
#else // !JET_VERSION >= 0x0501
#define JET_ccolMost                0x00007ffe
#endif // !JET_VERSION >= 0x0501
#define JET_ccolFixedMost           0x0000007f
#define JET_ccolVarMost             0x00000080
#define JET_ccolTaggedMost          ( JET_ccolMost - 0x000000ff )

#if ( JET_VERSION >= 0x0501 )
#define JET_EventLoggingDisable     0
#if ( JET_VERSION >= 0x0601 )
#define JET_EventLoggingLevelMin    1
#define JET_EventLoggingLevelLow    25
#define JET_EventLoggingLevelMedium 50
#define JET_EventLoggingLevelHigh   75
#endif // JET_VERSION >= 0x0601
#define JET_EventLoggingLevelMax    100
#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0603 )
// Values for JET_paramEnableIndexChecking.
typedef enum
{
    JET_IndexCheckingOff = 0,
    JET_IndexCheckingOn = 1,
    JET_IndexCheckingDeferToOpenTable = 2,
    JET_IndexCheckingMax = 3,
} JET_INDEXCHECKING;
#endif

// The following values are bit-fields that JET_paramIOPriority can be set to
#if ( JET_VERSION >= 0x0600 )
// Values for JET_paramIOPriority
#define JET_IOPriorityNormal                    0x0       // default
#define JET_IOPriorityLow                       0x1
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0602 )
//  Values for usage with JET_paramConfiguration
//
//  Can set the optimization configs one at a time.
//
#define JET_configDefault                       0x0001  //  Resets ALL parameters to their default value
#define JET_configRemoveQuotas                  0x0002  //  Unrestricts the quota enforcement (by setting to as high as possible) for any ESE handle types where memory is not pre-allocated or used as a cache.
#define JET_configLowDiskFootprint              0x0004  //  Set appropriate parameters to optimize the engine to use a small amount of disk space.  Uses circular logging.
#define JET_configMediumDiskFootprint           0x0008  //  Set appropriate parameters to optimize the engine to use a medium amount of disk space.  Uses circular logging.
#define JET_configLowMemory                     0x0010  //  Set appropriate parameters to optimize the engine to use a small amount of memory/working set at the cost of CPU efficiency and some disk efficiency.
#define JET_configDynamicMediumMemory           0x0020  //  Set appropriate parameters to optimize the engine to use a modest amount of memory/working set at the cost of CPU efficiency, dynamically adjusting for bursts in activity.
#define JET_configLowPower                      0x0040  //  Set appropriate parameters to optimize the engine to attempt to conserve power over keeping everything the most up to date, or memory usage.
#define JET_configSSDProfileIO                  0x0080  //  Set appropriate parameters to optimize the engine to be using the SSD profile IO parameters.
#define JET_configRunSilent                     0x0100  //  Turns off all externally visible signs of the library running (event logs, perfmon, tracing, etc).  NOTE: This makes debugging issues difficult, best if app policy has way to configure this off or on.
#if ( JET_VERSION >= 0x0A00 )
#define JET_configUnthrottledMemory             0x0200  //  Allows ESE to grow to most of memory because this is likely a single purpose server for this machine, or wants to allow our variable memory caches to grow to use most of memory if in use.
#define JET_configHighConcurrencyScaling        0x0400  //  Ensures ESE uses all its high concurrency scaling methods to achieve high levels of performance on multi-CPU systems (SMP, Multi-Core, Hyper-Threading, etc) for server scale applications, at a higher fixed memory overhead.
#endif // JET_VERSION >= 0x0A00

#endif // JET_VERSION >= 0x0602

//  system parameters
//
//  NOTE:  the default values of these parameters used to be documented here.
//  this can no longer be done because we now support multiple sets of default
//  values as set by JET_paramConfiguration
//
//  location parameters
//
#define JET_paramSystemPath                     0   //  path to check point file
#define JET_paramTempPath                       1   //  path to the temporary database
#define JET_paramLogFilePath                    2   //  path to the log file directory
#define JET_paramBaseName                       3   //  base name for all DBMS object names
#define JET_paramEventSource                    4   //  language independent process descriptor string

//  performance parameters
//
#define JET_paramMaxSessions                    5   //  maximum number of sessions
#define JET_paramMaxOpenTables                  6   //  maximum number of open directories
                                                    //      need 1 for each open table index,
                                                    //      plus 1 for each open table with no indexes,
                                                    //      plus 1 for each table with long column data,
                                                    //      plus a few more.
                                                    //      for 4.1, 1/3 for regular table, 2/3 for index
#define JET_paramPreferredMaxOpenTables         7   //  preferred maximum number of open directories
#if ( JET_VERSION >= 0x0600 )
#define JET_paramCachedClosedTables             125 //  number of closed tables to cache the meta-data for
#endif // JET_VERSION >= 0x0600
#define JET_paramMaxCursors                     8   //  maximum number of open cursors
#define JET_paramMaxVerPages                    9   //  maximum version store size in version pages
#define JET_paramPreferredVerPages              63  //  preferred version store size in version pages
#if ( JET_VERSION >= 0x0501 )
#define JET_paramGlobalMinVerPages              81  //  minimum version store size for all instances in version pages
#define JET_paramVersionStoreTaskQueueMax       105 //  maximum number of tasks in the task queue before start dropping the tasks
#endif // JET_VERSION >= 0x0501
#define JET_paramMaxTemporaryTables             10  //  maximum concurrent open temporary table/index creation
#define JET_paramLogFileSize                    11  //  log file size in kBytes
#define JET_paramLogBuffers                     12  //  log buffers in 512 byte units.
#define JET_paramWaitLogFlush                   13  //  log flush wait time in milliseconds
#define JET_paramLogCheckpointPeriod            14  //  checkpoint period in sectors
#define JET_paramLogWaitingUserMax              15  //  maximum sessions waiting log flush
#define JET_paramCommitDefault                  16  //  default grbit for JetCommitTransaction
#define JET_paramCircularLog                    17  //  boolean flag for circular logging
#define JET_paramDbExtensionSize                18  //  database extension size in pages
#define JET_paramPageTempDBMin                  19  //  minimum size temporary database in pages
#define JET_paramPageFragment                   20  //  maximum disk extent considered fragment in pages
#if ( JET_VERSION >= 0x0600 )
#define JET_paramEnableFileCache                126 //  enable the use of the OS file cache for all managed files
#define JET_paramVerPageSize                    128 //  the version store page size
#define JET_paramConfiguration                  129 //  RESETs all parameters to their default for a given configuration
#define JET_paramEnableAdvanced                 130 //  enables the modification of advanced settings
#define JET_paramMaxColtyp                      131 //  maximum coltyp supported by this version of ESE
#endif // JET_VERSION >= 0x0600

//  cache performance parameters
//
#define JET_paramBatchIOBufferMax               22  //  maximum batch I/O buffers in pages
#define JET_paramCacheSize                      41  //  current cache size in pages
#define JET_paramCacheSizeMin                   60  //  minimum cache size in pages
#define JET_paramCacheSizeMax                   23  //  maximum cache size in pages
#define JET_paramCheckpointDepthMax             24  //  maximum checkpoint depth in bytes
#define JET_paramLRUKCorrInterval               25  //  time (usec) under which page accesses are correlated
#define JET_paramLRUKHistoryMax                 26  //  maximum LRUK history records
#define JET_paramLRUKPolicy                     27  //  K-ness of LRUK page eviction algorithm (1...2)
#define JET_paramLRUKTimeout                    28  //  time (sec) after which cached pages are always evictable
#define JET_paramLRUKTrxCorrInterval            29  //  Not Used: time (usec) under which page accesses by the same transaction are correlated
#define JET_paramOutstandingIOMax               30  //  maximum outstanding I/Os
#define JET_paramStartFlushThreshold            31  //  evictable pages at which to start a flush (proportional to CacheSizeMax)
#define JET_paramStopFlushThreshold             32  //  evictable pages at which to stop a flush (proportional to CacheSizeMax)
#if ( JET_VERSION >= 0x0600 )
#define JET_paramEnableViewCache                127 //  enable the use of memory mapped file I/O for database files
#define JET_paramCheckpointIOMax                135 //  maxiumum number of pending flush writes
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0600 )
// TableClass names
#define JET_paramTableClass1Name                137     // name of tableclass1
#define JET_paramTableClass2Name                138     // name of tableclass2
#define JET_paramTableClass3Name                139     // name of tableclass3
#define JET_paramTableClass4Name                140     // name of tableclass4
#define JET_paramTableClass5Name                141     // name of tableclass5
#define JET_paramTableClass6Name                142     // name of tableclass6
#define JET_paramTableClass7Name                143     // name of tableclass7
#define JET_paramTableClass8Name                144     // name of tableclass8
#define JET_paramTableClass9Name                145     // name of tableclass9
#define JET_paramTableClass10Name               146     // name of tableclass10
#define JET_paramTableClass11Name               147     // name of tableclass11
#define JET_paramTableClass12Name               148     // name of tableclass12
#define JET_paramTableClass13Name               149     // name of tableclass13
#define JET_paramTableClass14Name               150     // name of tableclass14
#define JET_paramTableClass15Name               151     // name of tableclass15
#endif // JET_VERSION >= 0x0600


#define JET_paramIOPriority                     152     //  adjust IO priority per instance, anytime. Mainly for background recovery
                                                        //  Doesn't affect pending IOs, just subsequent ones

#define JET_paramRecovery                       34  //  enable recovery via setting the string "On" or "Off"
#define JET_paramEnableOnlineDefrag             35  //  enable online defrag

//  Application specific parameter
//
#define JET_paramCheckFormatWhenOpenFail        44  //  JetInit may return JET_errDatabaseXXXformat instead of database corrupt when it is set
#define JET_paramEnableTempTableVersioning      46  //  Enable versioning of temp tables
#define JET_paramIgnoreLogVersion               47  //  Do not check the log version
#define JET_paramDeleteOldLogs                  48  //  Delete the log files if the version is old, after deleting may make database non-recoverable
#define JET_paramEventSourceKey                 49  //  Event source registration key value
#define JET_paramNoInformationEvent             50  //  Disable logging information event
#if ( JET_VERSION >= 0x0501 )
#define JET_paramEventLoggingLevel              51  //  Set the type of information that goes to event log
#define JET_paramDeleteOutOfRangeLogs           52  //  Delete the log files that are not matching (generation wise) during soft recovery
#define JET_paramAccessDeniedRetryPeriod        53  //  Number of milliseconds to retry when about to fail with AccessDenied
#endif // JET_VERSION >= 0x0501

//  Index-checking parameters
//
//  After Windows 7, it was discovered that JET_paramEnableIndexCleanup had some implementation limitations, reducing its effectiveness.
//  Rather than update it to work with locale names, the functionality is removed altogether.
//
//  Unfortunately JET_paramEnableIndexCleanup can not be ignored altogether. JET_paramEnableIndexChecking defaults to false, so if
//  JET_paramEnableIndexCleanup were to be removed entirely, then by default there were would be no checks for NLS changes!
//
//  The current behavious (when enabled) is to track the language sort versions for the indices, and when the sort version for that
//  particular locale changes, the engine knows which indices are now invalid. For example, if the sort version for only "de-de" changes,
//  then the "de-de" indices are invalid, but the "en-us" indices will be fine.
//
//  Post-Windows 8:
//  JET_paramEnableIndexChecking accepts JET_INDEXCHECKING (which is an enum). The values of '0' and '1' have the same meaning as before,
//  but '2' is JET_IndexCheckingDeferToOpenTable, which means that the NLS up-to-date-ness is NOT checked when the database is attached.
//  It is deferred to JetOpenTable(), which may now fail with JET_errPrimaryIndexCorrupted or JET_errSecondaryIndexCorrupted (which
//  are NOT actual corruptions, but instead reflect an NLS sort change).
//
//  IN SUMMARY:
//  New code should explicitly set both IndexChecking and IndexCleanup to the same value.
//
//
//  OLDER NOTES (up to and including Windows 7)
//
//  Different versions of windows normalize unicode text in different ways. That means indexes built under one version of Windows may
//  not work on other versions. Windows Server 2003 Beta 3 introduced GetNLSVersion() which can be used to determine the version of unicode normalization
//  that the OS currently provides. Indexes built in server 2003 are flagged with the version of unicode normalization that they were
//  built with (older indexes have no version information). Most unicode normalization changes consist of adding new characters -- codepoints
//  which were previously undefined are defined and normalize differently. Thus, if binary data is stored in a unicode column it will normalize
//  differently as new codepoints are defined.
//
//  As of Windows Server 2003 RC1 ESENT tracks unicode index entries that contain undefined codepoints. These can be used to fixup an index when the
//  set of defined unicode characters changes.
//
//  These parameters control what happens when ESENT attaches to a database that was last used under a different build of the OS (the OS version
//  is stamped in the database header).
//
//  If JET_paramEnableIndexChecking is TRUE JetAttachDatabase() will delete indexes if JET_bitDbDeleteCorruptIndexes or return an error if
//  the grbit was not specified and there are indexes which need deletion. If it is set to FALSE then JetAttachDatabase() will succeed, even
//  if there are potentially corrupt indexes.
//
//  If JET_paramEnableIndexCleanup is set, the internal fixup table will be used to fixup index entries. This may not fixup all index corruptions
//  but will be transparent to the application.
//

#define JET_paramEnableIndexChecking            45  //  Enable checking OS version for indexes (false by default).
#if ( JET_VERSION >= 0x0502 )
#define JET_paramEnableIndexCleanup             54  //  Enable cleanup of out-of-date index entries (Windows 2003 through Windows 7); Does NLS version checking (Windows 2003 and later).
#endif // JET_VERSION >= 0x0502


//                                              60  //  JET_paramCacheSizeMin defined above
//                                              63  //  JET_paramPreferredVerPages defined above
#define JET_paramDatabasePageSize               64  //  set database page size
#if ( JET_VERSION >= 0x0501 )
#define JET_paramDisableCallbacks               65  //  turn off callback resolution (for defrag/repair)
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0501 )
#define JET_paramLogFileCreateAsynch            69  //  prepares next log file while logging to the current one to smooth response time
#endif // JET_VERSION >= 0x0501
#define JET_paramErrorToString                  70  //  turns a JET_err into a string (taken from the comment in jet.h)
#if ( JET_VERSION >= 0x0501 )
#define JET_paramZeroDatabaseDuringBackup       71  //  Overwrite deleted records/LVs during backup
#endif // JET_VERSION >= 0x0501
#define JET_paramUnicodeIndexDefault            72  //  default LCMapString() lcid and flags to use for CreateIndex() and unique multi-values check
                                                    //      (pass pointer to JET_UNICODEINDEX structure for plParam and sizeof(JET_UNICODE_INDEX) for cbMax)
#if ( JET_VERSION >= 0x0501 )
#define JET_paramRuntimeCallback                73  //  pointer to runtime-only callback function
#endif // JET_VERSION >= 0x0501
#define JET_paramCleanupMismatchedLogFiles      77  //  instead of erroring out after a successful recovery with JET_errLogFileSizeMismatchDatabasesConsistent, ESE will silently delete the old log files and checkpoint file and continue operations
#if ( JET_VERSION >= 0x0501 )
#define JET_paramRecordUpgradeDirtyLevel        78  //  how aggresively should pages with their record format converted be flushed (0-3)
//                                              81  //  JET_paramGlobalMinVerPages defined above
#define JET_paramOSSnapshotTimeout              82  //  timeout for the freeze period in msec
#endif // JET_VERSION >= 0x0501

#define JET_paramExceptionAction                98  //  what to do with exceptions generated within JET
#define JET_paramEventLogCache                  99  //  number of bytes of eventlog records to cache if service is not available

#if ( JET_VERSION >= 0x0501 )
#define JET_paramCreatePathIfNotExist           100 //  create system/temp/log/log-failover paths if they do not exist
#define JET_paramPageHintCacheSize              101 //  maximum size of the fast page latch hint cache in bytes
#define JET_paramOneDatabasePerSession          102 //  allow just one open user database per session
#define JET_paramMaxInstances                   104 //  maximum number of instances per process
#define JET_paramDisablePerfmon                 107 //  disable perfmon support for this process

#define JET_paramIndexTuplesLengthMin           110 //  for tuple indexes, minimum length of a tuple
#define JET_paramIndexTuplesLengthMax           111 //  for tuple indexes, maximum length of a tuple
#define JET_paramIndexTuplesToIndexMax          112 //  for tuple indexes, maximum number of characters in a given string to index
#endif // JET_VERSION >= 0x0501

// Parameters added in Windows 2003/XP64.
#if ( JET_VERSION >= 0x0502 )
#define JET_paramAlternateDatabaseRecoveryPath  113 //  recovery-only - search for dirty-shutdown databases in specified location only
#endif // JET_VERSION >= 0x0502


// Parameters added in Windows Vista.
#if ( JET_VERSION >= 0x0600 )
#define JET_paramIndexTupleIncrement            132 //  for tuple indexes, offset increment for each succesive tuple
#define JET_paramIndexTupleStart                133 //  for tuple indexes, offset to start tuple indexing
#define JET_paramKeyMost                        134 //  read only maximum settable key length before key trunctation occurs
#define JET_paramLegacyFileNames                136  // Legacy  file name characteristics to preserve ( JET_bitESE98FileNames | JET_bitEightDotThreeSoftCompat )
#define JET_paramEnablePersistedCallbacks       156  //  allow the database engine to resolve and use callbacks persisted in a database
#endif // JET_VERSION >= 0x0600

// Parameters added in Windows 7.
#if ( JET_VERSION >= 0x0601 )
#define JET_paramWaypointLatency                153  // The latency (in logs) behind the tip / highest committed log to defer database page flushes.
#define JET_paramDefragmentSequentialBTrees     160 //  Turn on/off automatic sequential B-tree defragmentation tasks (On by default, but also requires JET_SPACEHINTS flags / JET_bitRetrieveHintTableScan* to trigger on any given tables).
#define JET_paramDefragmentSequentialBTreesDensityCheckFrequency    161 //  Determine how frequently B-tree density is checked
#define JET_paramIOThrottlingTimeQuanta         162 //  Max time (in MS) that the I/O throttling mechanism gives a task to run for it to be considered 'completed'.
#define JET_paramLVChunkSizeMost                163 //  Max LV chunk size supported wrt the chosen page size (R/O)
#define JET_paramMaxCoalesceReadSize            164 //  Max number of bytes that can be grouped for a coalesced read operation.
#define JET_paramMaxCoalesceWriteSize           165 //  Max number of bytes that can be grouped for a coalesced write operation.
#define JET_paramMaxCoalesceReadGapSize         166 //  Max number of bytes that can be gapped for a coalesced read IO operation.
#define JET_paramMaxCoalesceWriteGapSize        167 //  Max number of bytes that can be gapped for a coalesced write IO operation.
#define JET_paramEnableDBScanInRecovery         169 //  Do checksumming of the database during recovery.
#define JET_paramDbScanThrottle                 170 //  throttle (mSec).
#define JET_paramDbScanIntervalMinSec           171 //  Min internal to repeat checksumming (Sec).
#define JET_paramDbScanIntervalMaxSec           172 //  Max internal checksumming must finish (Sec).
#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0602 )

#define JET_paramCachePriority                  177 //  Per-instance property for relative cache priorities (default = 100).
                                                    //
                                                    //  There are three scopes for which cache priority may be assigned:
                                                    //
                                                    //    - Instance: by calling JetSetSystemParameter and setting JET_paramCachePriority for a specific
                                                    //                ESE instance. The cache priority for this scope is always defined (default is 100),
                                                    //                even if the client does not set the priority explicitly using the system parameter.
                                                    //    - Session: by calling JetSetSessionParameter and setting JET_sesparamCachePriority for a specific
                                                    //               ESE session. The cache priority for this scope is undefined by default.
                                                    //    - Database: by calling JetCreateDatabase3 (or above) or JetAttachDatabase3 (or above) and setting
                                                    //                JET_dbparamCachePriority for a specific new or attached database. The cache priority
                                                    //                for this scope is undefined by default.
                                                    //
                                                    //  The way cache priority for those three scopes interact is as follows:
                                                    //    - If only the priority for the instance scope is defined, it is used for all operations related
                                                    //      to that instance.
                                                    //    - If only the priorities for the instance and session scopes are defined, the session scope priority
                                                    //      is used for all operations related to that session.
                                                    //    - If only the priorities for the instance and database scopes are defined, the database scope priority
                                                    //      is used for all operations related to that database.
                                                    //    - If the priorities for all three scopes are defined, the lowest of session and database scope
                                                    //      priorities is used for all operations related to that session/database combination. For everything
                                                    //      else, the rules above apply on a per-database-page basis.

#define JET_paramMaxTransactionSize             178 //  Percentage of version store that can be used by oldest transaction before JET_errVersionStoreOutOfMemory (default = 100).
#define JET_paramPrereadIOMax                   179 //  Maximum number of I/O operations dispatched for a given purpose.
#define JET_paramEnableDBScanSerialization      180 //  Database Maintenance serialization is enabled for databases sharing the same disk.
#define JET_paramHungIOThreshold                181 //  The threshold for what is considered a hung IO that should be acted upon.
#define JET_paramHungIOActions                  182 //  A set of actions to be taken on IOs that appear hung.
#define JET_paramMinDataForXpress               183 //  Smallest amount of data that should be compressed with xpress compression.
#endif // JET_VERSION >= 0x0602

#if ( JET_VERSION >= 0x0603 )
#define JET_paramEnableShrinkDatabase           184 //  Release space back to the OS when deleting data. This may require an OS feature of Sparse Files, and is subject to change.

#endif // JET_VERSION >= 0x0603

// Parameters added in Windows 8.
#if ( JET_VERSION >= 0x0602 )
#define JET_paramProcessFriendlyName            186 //  Friendly name for this instance of the process (e.g. performance counter global instance name, event logs).
#define JET_paramDurableCommitCallback          187 //  callback for when log is flushed
#endif // JET_VERSION >= 0x0602

// Parameters added in Windows 8.1.
#if ( JET_VERSION >= 0x0603 )
#define JET_paramEnableSqm                      188 //  Deprecated / ignored param.
#endif // JET_VERSION >= 0x0603

// Parameters added in Windows 10.
#if ( JET_VERSION >= 0x0A00 )

#define JET_paramConfigStoreSpec                189 //  Custom path that allows the consumer to specify a path (currently from in the registry) from which to pull custom ESE configuration.


#endif // JET_VERSION >= 0x0A00

#define JET_paramEngineFormatVersion            194 //  Engine format version - specifies the maximum format version the engine should allow, ensuring no format features are used beyond this (allowing the DB / logs to be forward compatible).
#if ( JET_VERSION >= 0x0A01 )
#define JET_paramUseFlushForWriteDurability     214 //  This controls whether ESE uses Flush or FUA to make sure a write to disk is durable.

#define JET_paramEnableRBS                      215 //  Turns on revert snapshot. Not an ESE flight as we will let the variant be controlled outside ESE (like HA can enable this when lag is disabled)
#define JET_paramRBSFilePath                    216 //  path to the revert snapshot directory

#define JET_paramPerfmonRefreshInterval         217 //  Interval, in units of msec, used by the Permormance Monitor to refresh values for collection.

#define JET_paramEnableBlockCache               218 //  Indicates that the ESE Block Cache is enabled.  This is sufficient to access files previously attached to the ESE Block Cache but not to attach new files.

#endif // JET_VERSION >= 0x0A01

#define JET_paramTraceFlags                                     223 // Specific flags to include in IO traces indicating various info
#define JET_paramMaxValueInvalid                                232 //  This is not a valid parameter. It can change from release to release!



//  Session parameters
//
//      JET_sesparamBase                    4096    //  All JET_sesparams designed to be distinct from system / JET_params and JET_dbparams for code defense.

#define JET_sesparamCommitDefault           4097    //  Default grbit for JetCommitTransaction
#if ( JET_VERSION >= 0x0A00 )
#define JET_sesparamTransactionLevel        4099    //  Retrieves (read-only, no set) the current number of nested levels of transactions begun.  0 = not in a transaction.
#define JET_sesparamOperationContext        4100    //  a client context that the engine uses to track and trace operations (such as IOs)
#define JET_sesparamCorrelationID           4101    //  an ID that is logged in traces and can be used by clients to correlate ESE actions with their activity
#define JET_sesparamMaxValueInvalid         4111    //  This is not a valid session parameter. It can change from release to release!

typedef struct
{
    JET_UINT32      ulUserID;
    JET_BYTE        nOperationID;
    JET_BYTE        nOperationType;
    JET_BYTE        nClientType;
    JET_BYTE        fFlags;
} JET_OPERATIONCONTEXT;
#endif // JET_VERSION >= 0x0A00

#if ( JET_VERSION >= 0x0600 )

    /* Flags for JET_paramLegacyFileNames */

#define JET_bitESE98FileNames           0x00000001  //  Preserve the .log and .chk extension for compatibility reasons (i.e. Exchange)
#define JET_bitEightDotThreeSoftCompat  0x00000002  //  Preserve the 8.3 naming syntax for as long as possible. (this should not be changed, w/o ensuring there are no log files)
#endif // JET_VERSION >= 0x0600

    /* Flags for JET_paramHungIOActions */

#define JET_bitHungIOEvent                  0x00000001  // Log event when an IO appears to be hung for over the IO threshold.

#if ( JET_VERSION >= 0x0603 )
// Values for JET_paramEnableShrinkDatabase.
#define JET_bitShrinkDatabaseOff            0x0
#define JET_bitShrinkDatabaseOn             0x1     // Uses the file system's Sparse Files feature to release space in the middle of a file.
#define JET_bitShrinkDatabaseRealtime       0x2     // Attempts to reclaim space back to the file system after freeing significant amounts of data (when space is marked as Available to the Root space tree).

// DEPRECATED:
#define JET_bitShrinkDatabaseTrim           0x1     // DEPRECATED: Deprecated value for JET_bitShrinkDatabaseOn; Will be removed!

#endif // JET_VERSION >= 0x0603

    /* Flags for JetInit2, JetInit3 */

#if ( JET_VERSION >= 0x0501 )
#define JET_bitReplayIgnoreMissingDB        0x00000004  //  Ignore missing databases during recovery. This is a very dangerous option and may irrevocably produce an inconsistent database if improperly used. Normal ESE usage does not typically require this dangerous option.
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0600 )
#define JET_bitRecoveryWithoutUndo          0x00000008  //  perform recovery, but halt at the Undo phase
#define JET_bitTruncateLogsAfterRecovery    0x00000010  //  on successful soft recovery, truncate log files
#define JET_bitReplayMissingMapEntryDB      0x00000020  //  missing database map entry default to same location
#define JET_bitLogStreamMustExist           0x00000040  //  transaction logs must exist in the logfile directory (ie. cannot auto-start a new log stream)
#endif // JET_VERSION >= 0x0600
#if ( JET_VERSION >= 0x0601 )
#define JET_bitReplayIgnoreLostLogs         0x00000080  //  ignore logs lost from the end of the log stream
#endif // JET_VERSION >= 0x0601
#if ( JET_VERSION >= 0x0602 )
#define JET_bitKeepDbAttachedAtEndOfRecovery 0x00001000 //  this allows db to remain attached at the end of recovery (for faster transition to running state)
#endif // JET_VERSION >= 0x0602

    /* Flags for JetTerm2 */

#define JET_bitTermComplete             0x00000001
#define JET_bitTermAbrupt               0x00000002
#define JET_bitTermStopBackup           0x00000004
#if ( JET_VERSION >= 0x0601 )
#define JET_bitTermDirty                0x00000008
#endif // JET_VERSION >= 0x0601

    /* Flags for JetIdle */

#define JET_bitIdleFlushBuffers         0x00000001
#define JET_bitIdleCompact              0x00000002
#define JET_bitIdleStatus               0x00000004

    /* Flags for JetEndSession */


    /* Flags for JetAttachDatabase/JetOpenDatabase */

#define JET_bitDbReadOnly               0x00000001
#define JET_bitDbExclusive              0x00000002 /* multiple opens allowed */
#define JET_bitDbDeleteCorruptIndexes   0x00000010 /* delete indexes possibly corrupted by NT version upgrade */
#if ( JET_VERSION >= 0x0502 )
#define JET_bitDbDeleteUnicodeIndexes   0x00000400 /* delete all indexes with unicode columns */
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0501 )
#define JET_bitDbUpgrade                0x00000200 /* */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0601 )
#define JET_bitDbEnableBackgroundMaintenance    0x00000800  /* the database engine will initiate automatic background database maintenance */
#endif
#if ( JET_VERSION >= 0x0602 )
#define JET_bitDbPurgeCacheOnAttach     0x00001000 /* used to ensure any kept alive cache is purged for this DB before attach */
#endif

    /* Flags for JetDetachDatabase2 */

#if ( JET_VERSION >= 0x0501 )
#define JET_bitForceDetach                  0x00000001
#define JET_bitForceCloseAndDetach          (0x00000002 | JET_bitForceDetach)
#endif // JET_VERSION >= 0x0501


    /* Flags for JetCreateDatabase */

#define JET_bitDbRecoveryOff            0x00000008 /* disable logging/recovery for this database */
#define JET_bitDbShadowingOff           0x00000080 /* disable catalog shadowing */
#if ( JET_VERSION >= 0x0501 )
#define JET_bitDbOverwriteExisting      0x00000200 /* overwrite existing database with same name */
#endif // JET_VERSION >= 0x0501

    /* Flags for JetBackup, JetBeginExternalBackup, JetBeginExternalBackupInstance, JetBeginSurrogateBackup */

#define JET_bitBackupIncremental        0x00000001
#define JET_bitBackupAtomic             0x00000004
#if ( JET_VERSION >= 0x0501 )
#define JET_bitBackupSnapshot           0x00000010
#endif // JET_VERSION >= 0x0501

    /* Flags for JetEndExternalBackupInstance2, JetEndSurrogateBackup */

#if ( JET_VERSION >= 0x0501 )
#define JET_bitBackupEndNormal              0x0001
#define JET_bitBackupEndAbort               0x0002
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0600 )
#define JET_bitBackupTruncateDone           0x0100
#endif // JET_VERSION >= 0x0600

    /* Database types */

#define JET_dbidNil         ((JET_DBID) 0xFFFFFFFF)


    /* Flags for JetCreateTableColumnIndex */
#define JET_bitTableCreateFixedDDL          0x00000001  /* DDL is fixed */
#define JET_bitTableCreateTemplateTable     0x00000002  /* DDL is inheritable (implies FixedDDL) */
#if ( JET_VERSION >= 0x0501 )
#define JET_bitTableCreateNoFixedVarColumnsInDerivedTables  0x00000004
                                                        //  used in conjunction with JET_bitTableCreateTemplateTable
                                                        //  to disallow fixed/var columns in derived tables (so that
                                                        //  fixed/var columns may be added to the template in the future)
#endif // JET_VERSION >= 0x0501
#if JET_VERSION >= 0x0A00
#define JET_bitTableCreateImmutableStructure    0x00000008  // Do not write to the input structures. Additionally, do not return any auto-opened tableid.
#endif // JET_VERSION >= 0x0A00


    /* Flags for JetAddColumn, JetGetColumnInfo, JetOpenTempTable */

#define JET_bitColumnFixed              0x00000001
#define JET_bitColumnTagged             0x00000002
#define JET_bitColumnNotNULL            0x00000004
#define JET_bitColumnVersion                0x00000008
#define JET_bitColumnAutoincrement      0x00000010
#define JET_bitColumnUpdatable          0x00000020 /* JetGetColumnInfo only */
#define JET_bitColumnTTKey              0x00000040 /* JetOpenTempTable only */
#define JET_bitColumnTTDescending       0x00000080 /* JetOpenTempTable only */
#define JET_bitColumnMultiValued            0x00000400
#define JET_bitColumnEscrowUpdate       0x00000800 /* escrow updated, supported coltyps are long and longlong */
#define JET_bitColumnUnversioned        0x00001000 /* for add column only - add column unversioned */
#if ( JET_VERSION >= 0x0501 )
#define JET_bitColumnMaybeNull          0x00002000 /* for retrieve column info of outer join where no match from the inner table */
#define JET_bitColumnFinalize           0x00004000 /* DEPRECATED / Not Fully Implemented: use JET_bitColumnDeleteOnZero instead. */
#define JET_bitColumnUserDefinedDefault 0x00008000 /* default value from a user-provided callback */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0502 )
#define JET_bitColumnDeleteOnZero       0x00020000 /* When the escrow-update column reaches a value of zero (after all versions are resolve), the record will be deleted. A common use for a column that can be finalized is to use it as a reference count field, and when the field reaches zero the record gets deleted. A Delete-on-zero column must be an escrow update / JET_bitColumnEscrowUpdate column. JET_bitColumnDeleteOnZero cannot be used with JET_bitColumnFinalize. JET_bitColumnDeleteOnZero cannot be used with user defined default columns. */
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0601 )
#define JET_bitColumnCompressed         0x00080000 /* data in the column can be compressed */
#endif

#if ( JET_VERSION >= 0x0501 )
//  flags for JetDeleteColumn
#define JET_bitDeleteColumnIgnoreTemplateColumns    0x00000001  //  for derived tables, don't bother looking in template columns
#endif // JET_VERSION >= 0x0501


    /* Flags for JetSetCurrentIndex */

#define JET_bitMoveFirst                0x00000000
#define JET_bitNoMove                   0x00000002

    /* Flags for JetMakeKey */

#define JET_bitNewKey                   0x00000001
#define JET_bitStrLimit                 0x00000002
#define JET_bitSubStrLimit              0x00000004
#define JET_bitNormalizedKey            0x00000008
#define JET_bitKeyDataZeroLength        0x00000010
#if ( JET_VERSION >= 0x0501 )
#define JET_bitFullColumnStartLimit     0x00000100
#define JET_bitFullColumnEndLimit       0x00000200
#define JET_bitPartialColumnStartLimit  0x00000400
#define JET_bitPartialColumnEndLimit    0x00000800
#endif // JET_VERSION >= 0x0501

    /* Flags for JetSetIndexRange */

#define JET_bitRangeInclusive           0x00000001
#define JET_bitRangeUpperLimit          0x00000002
#define JET_bitRangeInstantDuration     0x00000004
#define JET_bitRangeRemove              0x00000008

    /* Flags for JetGetLock */

#define JET_bitReadLock                 0x00000001
#define JET_bitWriteLock                0x00000002

    /* Constants for JetMove */

#define JET_MoveFirst                   (0x80000000)
#define JET_MovePrevious                (-1)
#define JET_MoveNext                    (+1)
#define JET_MoveLast                    (0x7fffffff)

    /* Flags for JetMove */

#define JET_bitMoveKeyNE                0x00000001

    /* Flags for JetSeek */

#define JET_bitSeekEQ                   0x00000001
#define JET_bitSeekLT                   0x00000002
#define JET_bitSeekLE                   0x00000004
#define JET_bitSeekGE                   0x00000008
#define JET_bitSeekGT                   0x00000010
#define JET_bitSetIndexRange            0x00000020
#if ( JET_VERSION >= 0x0502 )
#define JET_bitCheckUniqueness          0x00000040  //  to be used with JET_bitSeekEQ only, returns JET_wrnUniqueKey if seek lands on a key which has no dupes
#endif // JET_VERSION >= 0x0502

#if ( JET_VERSION >= 0x0501 )
    //  Flags for JetGotoSecondaryIndexBookmark
#define JET_bitBookmarkPermitVirtualCurrency    0x00000001  //  place cursor on relative position in index if specified bookmark no longer exists
#endif // JET_VERSION >= 0x0501

    /* Flags for JET_CONDITIONALCOLUMN */
#define JET_bitIndexColumnMustBeNull    0x00000001
#define JET_bitIndexColumnMustBeNonNull 0x00000002

    /* Flags for JET_INDEXRANGE */
#define JET_bitRecordInIndex            0x00000001
#define JET_bitRecordNotInIndex         0x00000002

    /* Flags for JetCreateIndex */

#define JET_bitIndexUnique              0x00000001
#define JET_bitIndexPrimary             0x00000002
#define JET_bitIndexDisallowNull        0x00000004
#define JET_bitIndexIgnoreNull          0x00000008
#define JET_bitIndexIgnoreAnyNull       0x00000020
#define JET_bitIndexIgnoreFirstNull     0x00000040
#define JET_bitIndexLazyFlush           0x00000080
#define JET_bitIndexEmpty               0x00000100  // don't attempt to build index, because all entries would evaluate to NULL (MUST also specify JET_bitIgnoreAnyNull)
#define JET_bitIndexUnversioned         0x00000200
#define JET_bitIndexSortNullsHigh       0x00000400  // NULL sorts after data for all columns in the index
#define JET_bitIndexUnicode             0x00000800  // LCID field of JET_INDEXCREATE actually points to a JET_UNICODEINDEX struct to allow user-defined LCMapString() flags
#if ( JET_VERSION >= 0x0501 )
#define JET_bitIndexTuples              0x00001000  // index on substring tuples (text columns only)
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0502 )
#define JET_bitIndexTupleLimits         0x00002000  // cbVarSegMac field of JET_INDEXCREATE actually points to a JET_TUPLELIMITS struct to allow custom tuple index limits (implies JET_bitIndexTuples)
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0600 )
#define JET_bitIndexCrossProduct        0x00004000  // index over multiple multi-valued columns has full cross product
#define JET_bitIndexKeyMost             0x00008000  // custom index key size set instead of default of 255 bytes
#define JET_bitIndexDisallowTruncation  0x00010000  // fail update rather than truncate index keys
#define JET_bitIndexNestedTable         0x00020000  // index over multiple multi-valued columns but only with values of same itagSequence
#endif // JET_VERSION >= 0x0600
#if ( JET_VERSION >= 0x0602 )
#define JET_bitIndexDotNetGuid          0x00040000  // index over GUID column according to .Net GUID sort order
#endif // JET_VERSION >= 0x602
#if ( JET_VERSION >= 0x0A00 )
#define JET_bitIndexImmutableStructure  0x00080000  // Do not write to the input structures during a JetCreateIndexN call.
#endif // JET_VERSION >= 0x0A00

    /* Flags for index key definition */

#define JET_bitKeyAscending             0x00000000
#define JET_bitKeyDescending            0x00000001

    /* Flags for JetOpenTable */

#define JET_bitTableDenyWrite           0x00000001
#define JET_bitTableDenyRead            0x00000002
#define JET_bitTableReadOnly            0x00000004
#define JET_bitTableUpdatable           0x00000008
#define JET_bitTablePermitDDL           0x00000010  /*  override table flagged as FixedDDL (must be used with DenyRead) */
#define JET_bitTableNoCache             0x00000020  /*  don't cache the pages for this table */
#define JET_bitTablePreread             0x00000040  /*  assume the table is probably not in the buffer cache */
#define JET_bitTableOpportuneRead       0x00000080  /*  attempt to opportunely read physically adjacent leaf pages using larger physical IOs */
#define JET_bitTableSequential          0x00008000  /*  assume the table will be scanned sequentially */

#define JET_bitTableClassMask       0x001F0000  /*  table stats class mask */
#define JET_bitTableClassNone       0x00000000  /*  table belongs to no stats class (default) */
#define JET_bitTableClass1          0x00010000  /*  table belongs to stats class 1 */
#define JET_bitTableClass2          0x00020000  /*  table belongs to stats class 2 */
#define JET_bitTableClass3          0x00030000  /*  table belongs to stats class 3 */
#define JET_bitTableClass4          0x00040000  /*  table belongs to stats class 4 */
#define JET_bitTableClass5          0x00050000  /*  table belongs to stats class 5 */
#define JET_bitTableClass6          0x00060000  /*  table belongs to stats class 6 */
#define JET_bitTableClass7          0x00070000  /*  table belongs to stats class 7 */
#define JET_bitTableClass8          0x00080000  /*  table belongs to stats class 8 */
#define JET_bitTableClass9          0x00090000  /*  table belongs to stats class 9 */
#define JET_bitTableClass10         0x000A0000  /*  table belongs to stats class 10 */
#define JET_bitTableClass11         0x000B0000  /*  table belongs to stats class 11 */
#define JET_bitTableClass12         0x000C0000  /*  table belongs to stats class 12 */
#define JET_bitTableClass13         0x000D0000  /*  table belongs to stats class 13 */
#define JET_bitTableClass14         0x000E0000  /*  table belongs to stats class 14 */
#define JET_bitTableClass15         0x000F0000  /*  table belongs to stats class 15 */


#if ( JET_VERSION >= 0x0501 )
#define JET_bitLSReset              0x00000001  /*  reset LS value */
#define JET_bitLSCursor             0x00000002  /*  set/retrieve LS of table cursor */
#define JET_bitLSTable              0x00000004  /*  set/retrieve LS of table */

#define JET_LSNil                   (~(JET_LS)0)
#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0601 )
    /* Flags for JetSetTableSequential and JetPrereadIndexRanges */

#define JET_bitPrereadForward       0x00000001  /*  Hint that the sequential traversal will be in the forward direction */
#define JET_bitPrereadBackward      0x00000002  /*  Hint that the sequential traversal will be in the backward direction */
#if ( JET_VERSION >= 0x0602 )
#define JET_bitPrereadFirstPage     0x00000004  /*  Only first page of long values should be preread */
#define JET_bitPrereadNormalizedKey 0x00000008  /*  Normalized key/bookmark provided instead of column value */
#endif // JET_VERSION >= 0x0602
#endif // JET_VERSION >= 0x0601

    /* Flags for JetOpenTempTable */

#define JET_bitTTIndexed            0x00000001  /* Allow seek */
#define JET_bitTTUnique             0x00000002  /* Remove duplicates */
#define JET_bitTTUpdatable          0x00000004  /* Allow updates */
#define JET_bitTTScrollable         0x00000008  /* Allow backwards scrolling */
#define JET_bitTTSortNullsHigh      0x00000010  /* NULL sorts after data for all columns in the index */
#define JET_bitTTForceMaterialization       0x00000020                      /* Forces temp. table to be materialized into a btree (allows for duplicate detection) */
#if ( JET_VERSION >= 0x0501 )
#define JET_bitTTErrorOnDuplicateInsertion  JET_bitTTForceMaterialization   /* Error always returned when duplicate is inserted (instead of dupe being silently removed) */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0502 )
#define JET_bitTTForwardOnly        0x00000040  /* Prevents temp. table from being materialized into a btree (and enables duplicate keys) */
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0601 )
#define JET_bitTTIntrinsicLVsOnly   0x00000080  //  permit only intrinsic LV's (so materialisation is not required simply because a TT has an LV column)
#endif // JET_VERSION >= 0x0601
#if ( JET_VERSION >= 0x0602 )
#define JET_bitTTDotNetGuid         0x00000100  //  sort all JET_coltypGUID columns according to .Net Guid sort order
#endif // JET_VERSION >= 0x0602
#if ( JET_VERSION >= 0x0A01 )
#define JET_bitTTMaterializeBBT     0x00000200  // The temp table uses the Buffered BTree format when it is materialized
#endif // JET_VERSION >= 0x0A01
// begin_PubEsent


    /* Flags for JetSetColumn */

#define JET_bitSetAppendLV                  0x00000001
#define JET_bitSetOverwriteLV               0x00000004 /* overwrite JET_coltypLong* byte range */
#define JET_bitSetSizeLV                    0x00000008 /* set JET_coltypLong* size */
#define JET_bitSetZeroLength                0x00000020
#define JET_bitSetSeparateLV                0x00000040 /* force LV separation */
#define JET_bitSetUniqueMultiValues         0x00000080 /* prevent duplicate multi-values */
#define JET_bitSetUniqueNormalizedMultiValues   0x00000100 /* prevent duplicate multi-values, normalizing all data before performing comparisons */
#if ( JET_VERSION >= 0x0501 )
#define JET_bitSetRevertToDefaultValue      0x00000200 /* if setting last tagged instance to NULL, revert to default value instead if one exists */
#define JET_bitSetIntrinsicLV               0x00000400 /* store whole LV in record without bursting or return an error */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0601 )
#define JET_bitSetUncompressed              0x00010000 /* don't attempt compression when storing the data */
#define JET_bitSetCompressed                0x00020000 /* attempt compression when storing the data */
#if ( JET_VERSION >= 0x0A01 )
#define JET_bitSetContiguousLV              0x00040000 /* Allocates the long-value across contiguous pages (at potentialy space saving costs) for better IO behavior. Valid only with JET_bitSetSeparateLV. Invalid (or not implemented) with certain long-value operations such as replace, and certain column options such as compression. Use across many varying LVs sizes may cause space fragmentation / allocation issues. */
#endif // JET_VERSION >= 0x0A01
#endif // JET_VERSION >= 0x0601


#if ( JET_VERSION >= 0x0601 )
    /*  Space Hint Flags / JET_SPACEHINTS */

//  Generic
#define JET_bitSpaceHintsUtilizeParentSpace         0x00000001  //  This changes the internal allocation policy to get space hierarchically from a B-Tree's immediate parent.
//  Create
#define JET_bitCreateHintAppendSequential           0x00000002  //  This bit will enable Append split behavior to grow according to the growth dynamics of the table (set by cbMinExtent, ulGrowth, cbMaxExtent).
#define JET_bitCreateHintHotpointSequential         0x00000004  //  This bit will enable Hotpoint split behavior to grow according to the growth dynamics of the table (set by cbMinExtent, ulGrowth, cbMaxExtent).
//  Retrieve
#define JET_bitRetrieveHintReserve1                 0x00000008  //  Reserved and ignored
#define JET_bitRetrieveHintTableScanForward         0x00000010  //  By setting this the client indicates that forward sequential scan is the predominant usage pattern of this table (causing B+ Tree defrag to be auto-triggered to clean it up if fragmented).
#define JET_bitRetrieveHintTableScanBackward        0x00000020  //  By setting this the client indicates that backwards sequential scan is the predominant usage pattern of this table (causing B+ Tree defrag to be auto-triggered to clean it up if fragmented).
#define JET_bitRetrieveHintReserve2                 0x00000040  //  Reserved and ignored
#define JET_bitRetrieveHintReserve3                 0x00000080  //  Reserved and ignored
//  Update
//#define JET_bitUpdateReserved                     0x00000000  //  TBD.
//  Delete
#define JET_bitDeleteHintTableSequential            0x00000100  //  This means that the application expects this table to be cleaned up in-order sequentially (from lowest key to highest key)
#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0A01 )
#endif // JET_VERSION >= 0x0A01


    /*  Set column parameter structure for JetSetColumns */

typedef struct
{
    JET_COLUMNID            columnid;
    JET_PCVOID              pvData;
    JET_UINT32              cbData;
    JET_GRBIT               grbit;
    JET_UINT32              ibLongValue;
    JET_UINT32              itagSequence;
    JET_ERR                 err;
} JET_SETCOLUMN;

#if ( JET_VERSION >= 0x0501 )
typedef struct
{
    JET_UINT32      paramid;
    JET_API_PTR     lParam;
    JET_PCSTR       sz;
    JET_ERR         err;
} JET_SETSYSPARAM_A;

typedef struct
{
    JET_UINT32      paramid;
    JET_API_PTR     lParam;
    JET_PCWSTR      sz;
    JET_ERR         err;
} JET_SETSYSPARAM_W;


#ifdef JET_UNICODE
#define JET_SETSYSPARAM JET_SETSYSPARAM_W
#else
#define JET_SETSYSPARAM JET_SETSYSPARAM_A
#endif

#endif // JET_VERSION >= 0x0501

    /* Options for JetPrepareUpdate */

#define JET_prepInsert                      0
#define JET_prepReplace                     2
#define JET_prepCancel                      3
#define JET_prepReplaceNoLock               4
#define JET_prepInsertCopy                  5
#if ( JET_VERSION >= 0x0501 )
#define JET_prepInsertCopyDeleteOriginal    7   //  used for updating a record in the primary key; avoids the delete/insert process and updates autoinc */
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0603 )
#define JET_prepInsertCopyReplaceOriginal   9   //  used for updating a record in the primary key; avoids the delete/insert process and keeps autoinc */
#endif // JET_VERSION >= 0x0603

#if ( JET_VERSION >= 0x0603 )
// Values for JET_paramEnableSqm
#define JET_sqmDisable                      0   //  Explicitly disable SQM
#define JET_sqmEnable                       1   //  Explicitly enable SQM
#define JET_sqmFromCEIP                     2   //  Enables SQM based on Customer Experience Improvement Program opt-in
#endif // JET_VERSION >= 0x0603

    //  Flags for JetUpdate
#if ( JET_VERSION >= 0x0502 )
#define JET_bitUpdateCheckESE97Compatibility    0x00000001  //  check whether record fits if represented in ESE97 database format
#endif // JET_VERSION >= 0x0502

    /* Flags for JetEscrowUpdate */
#define JET_bitEscrowNoRollback             0x0001

    /* Flags for JetRetrieveColumn */

#define JET_bitRetrieveCopy                 0x00000001
#define JET_bitRetrieveFromIndex            0x00000002
#define JET_bitRetrieveFromPrimaryBookmark  0x00000004
#define JET_bitRetrieveTag                  0x00000008
#define JET_bitRetrieveNull                 0x00000010  /*  for columnid 0 only */
#define JET_bitRetrieveIgnoreDefault        0x00000020  /*  for columnid 0 only */
#if ( JET_VERSION >= 0x0600 )
#define JET_bitRetrieveTuple                0x00000800 /* retrieve tuple fragment from index */
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0602 )
    /* Flags for JET_INDEX_COLUMN */
#define JET_bitZeroLength                   0x00000001
#endif

    /* Retrieve column parameter structure for JetRetrieveColumns */

typedef struct
{
    JET_COLUMNID        columnid;
    JET_PVOID           pvData;
    JET_UINT32          cbData;
    JET_UINT32          cbActual;
    JET_GRBIT           grbit;
    JET_UINT32          ibLongValue;
    JET_UINT32          itagSequence;
    JET_COLUMNID        columnidNextTagged;
    JET_ERR             err;
} JET_RETRIEVECOLUMN;




#if ( JET_VERSION >= 0x0501 )
    /* Flags for JetEnumerateColumns */

#define JET_bitEnumerateCopy                        JET_bitRetrieveCopy
#define JET_bitEnumerateIgnoreDefault               JET_bitRetrieveIgnoreDefault
#define JET_bitEnumeratePresenceOnly                0x00020000
#define JET_bitEnumerateTaggedOnly                  0x00040000
#define JET_bitEnumerateCompressOutput              0x00080000
#if ( JET_VERSION >= 0x0502 )
// Available on Server 2003 SP1
#define JET_bitEnumerateIgnoreUserDefinedDefault    0x00100000
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0601 )
#define JET_bitEnumerateInRecordOnly                0x00200000
#endif // JET_VERSION >= 0x0601

    /* Parameter structures for JetEnumerateColumns */

typedef struct
{
    JET_COLUMNID            columnid;
    JET_UINT32              ctagSequence;
    JET_UINT32 *            rgtagSequence;
} JET_ENUMCOLUMNID;

typedef struct
{
    JET_UINT32              itagSequence;
    JET_ERR                 err;
    JET_UINT32              cbData;
    JET_PVOID               pvData;
} JET_ENUMCOLUMNVALUE;

typedef struct
{
    JET_COLUMNID            columnid;
    JET_ERR                 err;
    union
    {
        struct // err != JET_wrnColumnSingleValue
        {
            JET_UINT32              cEnumColumnValue;
            JET_ENUMCOLUMNVALUE*    rgEnumColumnValue;
        };
        struct // err == JET_wrnColumnSingleValue
        {
            JET_UINT32              cbData;
            JET_PVOID               pvData;
        };
    };
} JET_ENUMCOLUMN;

    /* Realloc callback for JetEnumerateColumns */

typedef JET_PVOID (JET_API * JET_PFNREALLOC)(
    _In_opt_ JET_PVOID  pvContext,
    _In_opt_ JET_PVOID  pv,
    _In_ JET_UINT32     cb );

#endif // JET_VERSION >= 0x0501



#if ( JET_VERSION >= 0x0600 )
    /* Flags for JetGetRecordSize */

#define JET_bitRecordSizeInCopyBuffer           0x00000001  //  use record in copy buffer
#define JET_bitRecordSizeRunningTotal           0x00000002  //  increment totals in output buffer instead of setting them
#define JET_bitRecordSizeLocal                  0x00000004  //  ignore Long Values (and other data otherwise not in the same page as the record)

    /* parameter structures for JetGetRecordSize */

typedef struct
{
    JET_UINT64          cbData;                 //  user data in record
    JET_UINT64          cbLongValueData;        //  user data associated with the record but stored in the long-value tree (NOTE: does NOT count intrinsic long-values)
    JET_UINT64          cbOverhead;             //  record overhead
    JET_UINT64          cbLongValueOverhead;    //  overhead of long-value data (NOTE: does not count intrinsic long-values)
    JET_UINT64          cNonTaggedColumns;      //  total number of fixed/variable columns
    JET_UINT64          cTaggedColumns;         //  total number of tagged columns
    JET_UINT64          cLongValues;            //  total number of values stored in the long-value tree for this record (NOTE: does NOT count intrinsic long-values)
    JET_UINT64          cMultiValues;           //  total number of values beyond the first for each column in the record
} JET_RECSIZE;
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0601 )
typedef struct
{
    JET_UINT64          cbData;                 //  user data in record
    JET_UINT64          cbLongValueData;        //  user data associated with the record but stored in the long-value tree (NOTE: does NOT count intrinsic long-values)
    JET_UINT64          cbOverhead;             //  record overhead
    JET_UINT64          cbLongValueOverhead;    //  overhead of long-value data (NOTE: does not count intrinsic long-values)
    JET_UINT64          cNonTaggedColumns;      //  total number of fixed/variable columns
    JET_UINT64          cTaggedColumns;         //  total number of tagged columns
    JET_UINT64          cLongValues;            //  total number of values stored in the long-value tree for this record (NOTE: does NOT count intrinsic long-values)
    JET_UINT64          cMultiValues;           //  total number of values beyond the first for each column in the record
    JET_UINT64          cCompressedColumns;     //  total number of columns which are compressed
    JET_UINT64          cbDataCompressed;       //  compressed size of user data in record (same as cbData if no intrinsic long-values are compressed)
    JET_UINT64          cbLongValueDataCompressed;  // compressed size of user data in the long-value tree (same as cbLongValue data if no separated long values are compressed)
} JET_RECSIZE2;
#endif // JET_VERSION >= 0x0601

#pragma warning(pop)        //  nonstandard extension used : nameless struct/union


    /* Flags for JetBeginTransaction2 */

#if ( JET_VERSION >= 0x0501 )
#define JET_bitTransactionReadOnly      0x00000001  /* transaction will not modify the database */
#endif // JET_VERSION >= 0x0501

    /* Flags for JetCommitTransaction */

#define JET_bitCommitLazyFlush          0x00000001  /* lazy flush log buffers. */
#define JET_bitWaitLastLevel0Commit     0x00000002  /* wait for last level 0 commit record flushed */
#if ( JET_VERSION >= 0x0502 )
#define JET_bitWaitAllLevel0Commit      0x00000008  /* wait for all level 0 commits to be flushed */
#endif // JET_VERSION >= 0x0502
#if ( JET_VERSION >= 0x0601 )
#define JET_bitForceNewLog              0x00000010
#endif // JET_VERSION >= 0x0601

    /* Flags for JetRollback */

#define JET_bitRollbackAll              0x00000001


#if ( JET_VERSION >= 0x0600 )
    /* Flags for JetOSSnapshot APIs */

    /* Flags for JetOSSnapshotPrepare */
#define JET_bitIncrementalSnapshot      0x00000001  /* bit 0: full (0) or incremental (1) snapshot */
#define JET_bitCopySnapshot             0x00000002  /* bit 1: normal (0) or copy (1) snapshot */
#define JET_bitContinueAfterThaw        0x00000004  /* bit 2: end on thaw (0) or wait for [truncate +] end snapshot */
#if ( JET_VERSION >= 0x0601 )
#define JET_bitExplicitPrepare          0x00000008  /* bit 3: all instaces prepared by default (0) or no instance prepared by default (1) */
#endif // JET_VERSION >= 0x0601

    /* Flags for JetOSSnapshotTruncateLog & JetOSSnapshotTruncateLogInstance */
#define JET_bitAllDatabasesSnapshot     0x00000001  /* bit 0: there are detached dbs in the instance (i.e. can't truncate logs) */

    /* Flags for JetOSSnapshotEnd */
#define JET_bitAbortSnapshot            0x00000001  /* snapshot process failed */
#endif // JET_VERSION >= 0x0600
    /* Info parameter for JetGetDatabaseInfo and JetGetDatabaseFileInfo */

#define JET_DbInfoFilename          0
#define JET_DbInfoConnect           1
#define JET_DbInfoCountry           2   //  retrieves the default country/region
#if ( JET_VERSION >= 0x0501 )
#define JET_DbInfoLCID              3
#endif // JET_VERSION >= 0x0501
#define JET_DbInfoLangid            3       // OBSOLETE: use JET_DbInfoLCID instead
#define JET_DbInfoCp                4
#define JET_DbInfoCollate           5
#define JET_DbInfoOptions           6
#define JET_DbInfoTransactions      7
#define JET_DbInfoVersion           8
#define JET_DbInfoIsam              9
#define JET_DbInfoFilesize          10
#define JET_DbInfoSpaceOwned        11
#define JET_DbInfoSpaceAvailable    12
#define JET_DbInfoUpgrade           13
#define JET_DbInfoMisc              14
#if ( JET_VERSION >= 0x0501 )
#define JET_DbInfoDBInUse           15
#define JET_DbInfoPageSize          17
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0600 )
#define JET_DbInfoFileType          19
#if ( JET_VERSION >= 0x603 )
#define JET_DbInfoFilesizeOnDisk    21
#endif

#endif // JET_VERSION >= 0x0600

    /* Dbstates from JetGetDatabaseFileInfo */

#define JET_dbstateJustCreated                  1
#define JET_dbstateDirtyShutdown                2
#define JET_dbstateCleanShutdown                3
#define JET_dbstateBeingConverted               4
#if ( JET_VERSION >= 0x0501 )
#define JET_dbstateForceDetach                  5
#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0600 )

    //  supported file types (returned from JetGetDatabaseFileInfo with JET_DbInfoFileType)

#define JET_filetypeUnknown                 0
#define JET_filetypeDatabase                1
#define JET_filetypeLog                     3
#define JET_filetypeCheckpoint              4
#define JET_filetypeTempDatabase            5
#define JET_filetypeFlushMap                7

#endif // JET_VERSION >= 0x0600

    /* Column data types */

#define JET_coltypNil               0
#define JET_coltypBit               1   /* True, False, or NULL */
#define JET_coltypUnsignedByte      2   /* 1-byte integer, unsigned */
#define JET_coltypShort             3   /* 2-byte integer, signed */
#define JET_coltypLong              4   /* 4-byte integer, signed */
#define JET_coltypCurrency          5   /* 8 byte integer, signed */
#define JET_coltypIEEESingle        6   /* 4-byte IEEE single precision */
#define JET_coltypIEEEDouble        7   /* 8-byte IEEE double precision */
#define JET_coltypDateTime          8   /* Integral date, fractional time */
#define JET_coltypBinary            9   /* Binary data, < 255 bytes */
#define JET_coltypText              10  /* ANSI text, case insensitive, < 255 bytes */
#define JET_coltypLongBinary        11  /* Binary data, long value */
#define JET_coltypLongText          12  /* ANSI text, long value */

// Pre XP
#if ( JET_VERSION < 0x0501 )
#define JET_coltypMax               13  /* the number of column types  */
                                        /* used for validity tests and */
                                        /* array declarations.         */
#endif // JET_VERSION < 0x0501

// Windows XP
#if ( JET_VERSION >= 0x0501 )
#define JET_coltypSLV               13  /* SLV's. Obsolete. */

#if ( JET_VERSION < 0x0600 )
#define JET_coltypMax               14  /* the number of column types  */
                                        /* used for validity tests and */
                                        /* array declarations.         */
#endif // JET_VERSION == 0x0501

#endif // JET_VERSION >= 0x0501

// Windows Vista to Windows 8.1
#if ( JET_VERSION >= 0x0600 )
#define JET_coltypUnsignedLong      14  /* 4-byte unsigned integer */
#define JET_coltypLongLong          15  /* 8-byte signed integer */
#define JET_coltypGUID              16  /* 16-byte globally unique identifier */
#define JET_coltypUnsignedShort     17  /* 2-byte unsigned integer */

#if ( JET_VERSION >= 0x0600 && JET_VERSION <= 0x0603 )
#define JET_coltypMax               18  /* the number of column types  */
                                        /* used for validity tests and */
                                        /* array declarations.         */
#endif // ( JET_VERSION >= 0x0600 && JET_VERSION <= 0x0603 )

#endif // JET_VERSION >= 0x0600

// Windows 10
#if ( JET_VERSION >= 0x0A00 )
#define JET_coltypUnsignedLongLong  18  /* 8-byte unsigned integer     */
#define JET_coltypMax               19  /* the number of column types  */
                                        /* used for validity tests and */
                                        /* array declarations.         */
#endif // JET_VERSION >= 0x0A00


    /* Info levels for JetGetObjectInfo */

#define JET_ObjInfo                 0U
#define JET_ObjInfoListNoStats      1U
#define JET_ObjInfoList             2U
#define JET_ObjInfoSysTabCursor     3U
#define JET_ObjInfoListACM          4U /* Blocked by JetGetObjectInfo */
#define JET_ObjInfoNoStats          5U
#define JET_ObjInfoSysTabReadOnly   6U
#define JET_ObjInfoRulesLoaded      7U
#define JET_ObjInfoMax              8U

    /* Info levels for JetGetTableInfo/JetSetTableInfo */

#define JET_TblInfo                    0U
#define JET_TblInfoName                1U
#define JET_TblInfoDbid                2U
#define JET_TblInfoMostMany            3U
#define JET_TblInfoRvt                 4U
#define JET_TblInfoOLC                 5U
#define JET_TblInfoResetOLC            6U
#define JET_TblInfoSpaceUsage          7U
#define JET_TblInfoDumpTable           8U
#define JET_TblInfoSpaceAlloc          9U
#define JET_TblInfoSpaceOwned         10U         // OwnExt for primary, 2ndary indices, and LV
#define JET_TblInfoSpaceAvailable     11U         // AvailExt for primary, 2ndary indices, and LV
#define JET_TblInfoTemplateTableName  12U
#if ( JET_VERSION >= 0x0A01 )
#define JET_TblInfoSpaceOwnedLV       17U         // OwnExt for LV
#define JET_TblInfoSpaceAvailableLV   18U         // AvailExt for LV
#endif

    /* Info levels for JetGetIndexInfo and JetGetTableIndexInfo */

#define JET_IdxInfo                 0U
#define JET_IdxInfoList             1U
#define JET_IdxInfoSysTabCursor     2U      //  OBSOLETE and unused.
#define JET_IdxInfoOLC              3U      //  OBSOLETE and unused.
#define JET_IdxInfoResetOLC         4U      //  OBSOLETE and unused.
#define JET_IdxInfoSpaceAlloc       5U
#if ( JET_VERSION >= 0x0501 )
#define JET_IdxInfoLCID             6U
#endif // JET_VERSION >= 0x0501
#define JET_IdxInfoLangid           6U      //  OBSOLETE: use JET_IdxInfoLCID instead
#define JET_IdxInfoCount            7U
#define JET_IdxInfoVarSegMac        8U
#define JET_IdxInfoIndexId          9U
#if ( JET_VERSION >= 0x0600 )
#define JET_IdxInfoKeyMost          10U
#endif // JET_VERSION >= 0x0600
#if ( JET_VERSION >= 0x0601 )
#define JET_IdxInfoCreateIndex      11U     //  return a JET_INDEXCREATE structure suitable for use by JetCreateIndex2()
#define JET_IdxInfoCreateIndex2     12U     //  return a JET_INDEXCREATE2 structure suitable for use by JetCreateIndex3()
#endif // JET_VERSION >= 0x0601
#if ( JET_VERSION >= 0x0602 )
#define JET_IdxInfoCreateIndex3     13U     //  return a JET_INDEXCREATE3 structure suitable for use by JetCreateIndex4()
#define JET_IdxInfoLocaleName       14U     //  Returns the locale name, which can be a wide string of up to LOCALE_NAME_MAX_LENGTH (including null).
#endif // JET_VERSION >= 0x0602
#if ( JET_VERSION >= 0x0A01 )
#define JET_IdxInfoSpaceOwned       18U    // Space owned exclusively by this index (unlike tables, ignores space from 2ndary indices, even
                                           //     when inquiring about primary indices
#define JET_IdxInfoSpaceAvailable   19U    // Space available exclusively for this index
#endif


    /* Info levels for JetGetColumnInfo and JetGetTableColumnInfo */

#define JET_ColInfo                 0U
#define JET_ColInfoList             1U
#define JET_ColInfoSysTabCursor     3U
#define JET_ColInfoBase             4U
#define JET_ColInfoListCompact      5U      //  INTERNAL USE ONLY
#if ( JET_VERSION >= 0x0501 )
#define JET_ColInfoByColid          6U
#define JET_ColInfoListSortColumnid 7U      //  OBSOLETE: use grbit instead
#endif // JET_VERSION >= 0x0501
#if ( JET_VERSION >= 0x0600 )
#define JET_ColInfoBaseByColid      8U
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0600 )

        // Grbits for JET_GetColumnInfo and JetGetTableColumnInfo (OR together with the info level)
#define JET_ColInfoGrbitNonDerivedColumnsOnly   0x80000000  //  for lists, only return non-derived columns (if the table is derived from a template)
#define JET_ColInfoGrbitMinimalInfo             0x40000000  //  for lists, only return the column name and columnid of each column
#define JET_ColInfoGrbitSortByColumnid          0x20000000  //  for lists, sort returned column list by columnid (default is to sort list by column name)

#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0600 )

    /* Info levels for JetGetInstanceMiscInfo, which is very different than JetGetInstanceInfo, as that retrieves a list of all instances */

#define JET_InstanceMiscInfoLogSignature    0U

#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0A01 )

#define JET_InstanceMiscInfoRBS             2U // Retrieve revert snapshot info for the instance.

#endif // JET_VERSION >= 0x0A01



    /* Engine Object Types */

#define JET_objtypNil               0
#define JET_objtypTable             1

    /* Compact Options */

#define JET_bitCompactStats             0x00000020  /* Dump off-line compaction stats (only when progress meter also specified) */
#define JET_bitCompactRepair            0x00000040  /* Don't preread and ignore duplicate keys */

    /* Status Notification Processes */

#define JET_snpRepair                   2
#define JET_snpCompact                  4
#define JET_snpRestore                  8
#define JET_snpBackup                   9
#define JET_snpUpgrade                  10
#if ( JET_VERSION >= 0x0501 )
#define JET_snpScrub                    11
#define JET_snpUpgradeRecordFormat      12
#endif // JET_VERSION >= 0x0501


    /* Status Notification Types */

//  Generic Status Notification Types
#define JET_sntBegin            5   /* callback for beginning of operation */
#define JET_sntRequirements     7   /* callback for returning operation requirements */
#define JET_sntProgress         0   /* callback for progress */
#define JET_sntComplete         6   /* callback for completion of operation */
#define JET_sntFail             3   /* callback for failure during progress */


    /* Exception action / JET_paramExceptionAction */

#define JET_ExceptionMsgBox     0x0001      /* Display message box on exception */
#define JET_ExceptionNone       0x0002      /* Do nothing on exceptions */
#define JET_ExceptionFailFast   0x0004      /* Use the Windows RaiseFailFastException API to force a crash */


#if ( JET_VERSION >= 0x0501 )
    //  Online defragmentation options
#define JET_OnlineDefragDisable         0x0000      //  disable online defrag
#define JET_OnlineDefragAllOBSOLETE     0x0001      //  enable online defrag for everything (must be 1 for backward compatibility)
#define JET_OnlineDefragDatabases       0x0002      //  enable online defrag of databases
#define JET_OnlineDefragSpaceTrees      0x0004      //  enable online defrag of space trees
#define JET_OnlineDefragAll             0xffff      //  enable online defrag for everything

#endif // JET_VERSION >= 0x0501



#if ( JET_VERSION >= 0x0602 )

// Info levels for JetGetErrorInfo:
#define JET_ErrorInfoSpecificErr        1U          //  Info about the specific error passed in pvContext.

// grbits for JetGetErrorInfoW:
// None yet.

// grbits for JetResizeDatabase:
#define JET_bitResizeDatabaseOnlyGrow               0x00000001  // Only grow the database. If the resize call would shrink the database, do nothing.
#endif // JET_VERSION >= 0x0602

#if ( JET_VERSION >= 0x0603 )
#define JET_bitResizeDatabaseOnlyShrink             0x00000002  // Only shrink the database. If the resize call would grow the database, do nothing. The file may end up smaller than requested.

#endif // JET_VERSION >= 0x0603

#if ( JET_VERSION >= 0x0602 )
#define JET_bitStopServiceAll                       0x00000000  //  Stops all ESE services for the specified instance.
#define JET_bitStopServiceBackgroundUserTasks       0x00000002  //  Stops restartable client specificed background maintenance tasks (B+ Tree Defrag for example).
#define JET_bitStopServiceQuiesceCaches             0x00000004  //  Quiesces all dirty caches to disk. Asynchronous. Cancellable.

// Warning: This bit can only be used to resume StopServiceBackgroundUserTasks and JET_bitStopServiceQuiesceCaches, if you
// previously called with JET_bitStopServiceAll, attempting to use JET_bitStopServiceResume will fail.
#define JET_bitStopServiceResume                    0x80000000  //  Resumes previously issued StopService operations, i.e. "restarts service".  Can be combined with above grbits to Resume specific services, or with JET_bitStopServiceAll to Resume all previously stopped services.
#endif // JET_VERSION >= 0x0602


/**********************************************************************/
/***********************     ERROR CODES     **************************/
/**********************************************************************/

/* The Error codes are not versioned with WINVER. */

/* SUCCESS */

#define JET_errSuccess                       0    /* Successful Operation */

/* ERRORS */

#define JET_wrnNyi                          -1    /* Function Not Yet Implemented */

/*  SYSTEM errors
/**/
#define JET_errRfsFailure                   -100  /* Resource Failure Simulator failure */
#define JET_errRfsNotArmed                  -101  /* Resource Failure Simulator not initialized */
#define JET_errFileClose                    -102  /* Could not close file */
#define JET_errOutOfThreads                 -103  /* Could not start thread */
#define JET_errTooManyIO                    -105  /* System busy due to too many IOs */
#define JET_errTaskDropped                  -106  /* A requested async task could not be executed */
#define JET_errInternalError                -107  /* Fatal internal error */
#define JET_errDisabledFunctionality        -112  /* You are running MinESE, that does not have all features compiled in.  This functionality is only supported in a full version of ESE. */
#define JET_errUnloadableOSFunctionality    -113  /* The desired OS functionality could not be located and loaded / linked. */

//  BUFFER MANAGER errors
//
#define JET_errDatabaseBufferDependenciesCorrupted  -255    /* Buffer dependencies improperly set. Recovery failure */

/*  DIRECTORY MANAGER errors
/**/
#define JET_wrnRemainingVersions             321  /* The version store is still active */
#define JET_errPreviousVersion              -322  /* Version already existed. Recovery failure */
#define JET_errPageBoundary                 -323  /* Reached Page Boundary */
#define JET_errKeyBoundary                  -324  /* Reached Key Boundary */
#define JET_errBadPageLink                  -327  /* Database corrupted */
#define JET_errBadBookmark                  -328  /* Bookmark has no corresponding address in database */
#define JET_errNTSystemCallFailed           -334  // A call to the operating system failed
#define JET_errBadParentPageLink            -338  // Database corrupted
#define JET_errSPAvailExtCacheOutOfSync     -340  // AvailExt cache doesn't match btree
#define JET_errSPAvailExtCorrupted          -341  // AvailExt space tree is corrupt
#define JET_errSPAvailExtCacheOutOfMemory   -342  // Out of memory allocating an AvailExt cache node
#define JET_errSPOwnExtCorrupted            -343  // OwnExt space tree is corrupt
#define JET_errDbTimeCorrupted              -344  // Dbtime on current page is greater than global database dbtime
#define JET_wrnUniqueKey                     345  // seek on non-unique index yielded a unique key
#define JET_errKeyTruncated                 -346  // key truncated on index that disallows key truncation
#define JET_errDatabaseLeakInSpace          -348  // Some database pages have become unreachable even from the avail tree, only an offline defragmentation can return the lost space.
#define JET_errBadEmptyPage                 -351  // Database corrupted. Searching an unexpectedly empty page.
#define wrnBTNotVisibleRejected              352  /* Current entry is not visible because it has been rejected by a move filter */
#define wrnBTNotVisibleAccumulated           353  /* Current entry is not visible because it is being accumulated by a move filter */
#define JET_errBadLineCount                 -354  /* Number of lines on the page is too few compared to the line being operated on */
#define JET_errPageTagCorrupted             -357  // A tag / line on page is logically corrupted, offset or size is bad, or tag count on page is bad.
#define JET_errNodeCorrupted                -358  // A node or prefix node is logically corrupted, the key suffix size is larger than the node or line's size.
#define JET_errBBTNodeCorrupted             -364  /* A property of the BBT node is logically corrupted. Or the BBT node isn't valid. */
#define JET_errBBTBuffCorrupted             -365  /* A BBT buff is logically corrupted. The nodes are out of sequence or the BBT header is corrupt. */
#define JET_wrnSeparateLongValue             406  /* Column is a separated long-value */
#define JET_wrnRecordFoundGreater           JET_wrnSeekNotEqual
#define JET_wrnRecordFoundLess              JET_wrnSeekNotEqual
#define JET_errColumnIllegalNull            JET_errNullInvalid
#define JET_errKeyTooBig                    -408  /* Key is too large */
#define JET_errCannotSeparateIntrinsicLV    -416    // illegal attempt to separate an LV which must be intrinsic
#define JET_errSeparatedLongValue           -421 /* Operation not supported on separated long-value */
#define JET_errMustBeSeparateLongValue      -423  /* Can only preread long value columns that can be separate, e.g. not size constrained so that they are fixed or variable columns */
#define JET_errInvalidPreread               -424  /* Cannot preread long values when current index secondary */

/*  LOGGING/RECOVERY errors
/**/
#define JET_errInvalidLoggedOperation       -500  /* Logged operation cannot be redone */
#define JET_errLogFileCorrupt               -501  /* Log file is corrupt */
#define JET_errNoBackupDirectory            -503  /* No backup directory given */
#define JET_errBackupDirectoryNotEmpty      -504  /* The backup directory is not empty */
#define JET_errBackupInProgress             -505  /* Backup is active already */
#define JET_errRestoreInProgress            -506  /* Restore in progress */
#define JET_errMissingPreviousLogFile       -509  /* Missing the log file for check point */
#define JET_errLogWriteFail                 -510  /* Failure writing to log file */
#define JET_errLogDisabledDueToRecoveryFailure  -511 /* Try to log something after recovery failed */
#define JET_errCannotLogDuringRecoveryRedo      -512    /* Try to log something during recovery redo */
#define JET_errLogGenerationMismatch        -513  /* Name of logfile does not match internal generation number */
#define JET_errBadLogVersion                -514  /* Version of log file is not compatible with Jet version */
#define JET_errInvalidLogSequence           -515  /* Timestamp in next log does not match expected */
#define JET_errLoggingDisabled              -516  /* Log is not active */
#define JET_errLogBufferTooSmall            -517  /* An operation generated a log record which was too large to fit in the log buffer or in a single log file */
#define JET_errLogSequenceEnd               -519  /* Maximum log file number exceeded */
#define JET_errNoBackup                     -520  /* No backup in progress */
#define JET_errInvalidBackupSequence        -521  /* Backup call out of sequence */
#define JET_errBackupNotAllowedYet          -523  /* Cannot do backup now */
#define JET_errDeleteBackupFileFail         -524  /* Could not delete backup file */
#define JET_errMakeBackupDirectoryFail      -525  /* Could not make backup temp directory */
#define JET_errInvalidBackup                -526  /* Cannot perform incremental backup when circular logging enabled */
#define JET_errRecoveredWithErrors          -527  /* Restored with errors */
#define JET_errMissingLogFile               -528  /* Current log file missing */
#define JET_errLogDiskFull                  -529  /* Log disk full */
#define JET_errBadLogSignature              -530  /* Bad signature for a log file */
#define JET_errBadDbSignature               -531  /* Bad signature for a db file */
#define JET_errBadCheckpointSignature       -532  /* Bad signature for a checkpoint file */
#define JET_errCheckpointCorrupt            -533  /* Checkpoint file not found or corrupt */
#define JET_errMissingPatchPage             -534  /* Patch file page not found during recovery */
#define JET_errBadPatchPage                 -535  /* Patch file page is not valid */
#define JET_errRedoAbruptEnded              -536  /* Redo abruptly ended due to sudden failure in reading logs from log file */
#define JET_errPatchFileMissing             -538  /* Hard restore detected that patch file is missing from backup set */
#define JET_errDatabaseLogSetMismatch       -539  /* Database does not belong with the current set of log files */
#define JET_errDatabaseStreamingFileMismatch    -540 /* Database and streaming file do not match each other */
#define JET_errLogFileSizeMismatch          -541  /* actual log file size does not match JET_paramLogFileSize */
#define JET_errCheckpointFileNotFound       -542  /* Could not locate checkpoint file */
#define JET_errRequiredLogFilesMissing      -543  /* The required log files for recovery is missing. */
#define JET_errSoftRecoveryOnBackupDatabase -544  /* Soft recovery is intended on a backup database. Restore should be used instead */
#define JET_errLogFileSizeMismatchDatabasesConsistent   -545  /* databases have been recovered, but the log file size used during recovery does not match JET_paramLogFileSize */
#define JET_errLogSectorSizeMismatch        -546  /* the log file sector size does not match the current volume's sector size */
#define JET_errLogSectorSizeMismatchDatabasesConsistent -547  /* databases have been recovered, but the log file sector size (used during recovery) does not match the current volume's sector size */
#define JET_errLogSequenceEndDatabasesConsistent        -548 /* databases have been recovered, but all possible log generations in the current sequence are used; delete all log files and the checkpoint file and backup the databases before continuing */

#define JET_errStreamingDataNotLogged       -549  /* Illegal attempt to replay a streaming file operation where the data wasn't logged. Probably caused by an attempt to roll-forward with circular logging enabled */

#define JET_errDatabaseDirtyShutdown        -550  /* Database was not shutdown cleanly. Recovery must first be run to properly complete database operations for the previous shutdown. */
#define JET_errDatabaseInconsistent         JET_errDatabaseDirtyShutdown    /* OBSOLETE */
#define JET_errConsistentTimeMismatch       -551  /* Database last consistent time unmatched */
#define JET_errDatabasePatchFileMismatch    -552  /* Patch file is not generated from this backup */
#define JET_errEndingRestoreLogTooLow       -553  /* The starting log number too low for the restore */
#define JET_errStartingRestoreLogTooHigh    -554  /* The starting log number too high for the restore */
#define JET_errGivenLogFileHasBadSignature  -555  /* Restore log file has bad signature */
#define JET_errGivenLogFileIsNotContiguous  -556  /* Restore log file is not contiguous */
#define JET_errMissingRestoreLogFiles       -557  /* Some restore log files are missing */
#define JET_wrnExistingLogFileHasBadSignature   558  /* Existing log file has bad signature */
#define JET_wrnExistingLogFileIsNotContiguous   559  /* Existing log file is not contiguous */
#define JET_errMissingFullBackup            -560  /* The database missed a previous full backup before incremental backup */
#define JET_errBadBackupDatabaseSize        -561  /* The backup database size is not in 4k */
#define JET_errDatabaseAlreadyUpgraded      -562  /* Attempted to upgrade a database that is already current */
#define JET_errDatabaseIncompleteUpgrade    -563  /* Attempted to use a database which was only partially converted to the current format -- must restore from backup */
#define JET_wrnSkipThisRecord                564  /* INTERNAL ERROR */
#define JET_errMissingCurrentLogFiles       -565  /* Some current log files are missing for continuous restore */

#define JET_errDbTimeTooOld                     -566  /* dbtime on page smaller than dbtimeBefore in record */
#define JET_errDbTimeTooNew                     -567  /* dbtime on page in advance of the dbtimeBefore and below dbtimeAfter in record */
#define JET_errMissingFileToBackup              -569  /* Some log or patch files are missing during backup */

#define JET_errLogTornWriteDuringHardRestore    -570    /* torn-write was detected in a backup set during hard restore */
#define JET_errLogTornWriteDuringHardRecovery   -571    /* torn-write was detected during hard recovery (log was not part of a backup set) */
#define JET_errLogCorruptDuringHardRestore      -573    /* corruption was detected in a backup set during hard restore */
#define JET_errLogCorruptDuringHardRecovery     -574    /* corruption was detected during hard recovery (log was not part of a backup set) */

#define JET_errMustDisableLoggingForDbUpgrade   -575    /* Cannot have logging enabled while attempting to upgrade db */

#define JET_errBadRestoreTargetInstance         -577    /* TargetInstance specified for restore is not found or log files don't match */
#define JET_wrnTargetInstanceRunning             578    /* TargetInstance specified for restore is running */

#define JET_errRecoveredWithoutUndo             -579    /* Soft recovery successfully replayed all operations, but the Undo phase of recovery was skipped */

#define JET_errDatabasesNotFromSameSnapshot     -580    /* Databases to be restored are not from the same shadow copy backup */
#define JET_errSoftRecoveryOnSnapshot           -581    /* Soft recovery on a database from a shadow copy backup set */
#define JET_errCommittedLogFilesMissing         -582    /* One or more logs that were committed to this database, are missing.  These log files are required to maintain durable ACID semantics, but not required to maintain consistency if the JET_bitReplayIgnoreLostLogs bit is specified during recovery. */
#define JET_errSectorSizeNotSupported           -583    /* The physical sector size reported by the disk subsystem, is unsupported by ESE for a specific file type. */
#define JET_errRecoveredWithoutUndoDatabasesConsistent  -584    /* Soft recovery successfully replayed all operations and intended to skip the Undo phase of recovery, but the Undo phase was not required */
#define JET_wrnCommittedLogFilesLost            585     /* One or more logs that were committed to this database, were not recovered.  The database is still clean/consistent, as though the lost log's transactions were committed lazily (and lost). */
#define JET_errCommittedLogFileCorrupt          -586    /* One or more logs were found to be corrupt during recovery.  These log files are required to maintain durable ACID semantics, but not required to maintain consistency if the JET_bitIgnoreLostLogs bit and JET_paramDeleteOutOfRangeLogs is specified during recovery. */
#define JET_wrnCommittedLogFilesRemoved         587     /* One or more logs that were committed to this database, were no recovered.  The database is still clean/consistent, as though the corrupted log's transactions were committed lazily (and lost). */
#define JET_wrnFinishWithUndo                   588     /* Signal used by clients to indicate JetInit() finished with undo */
#define JET_errLogSequenceChecksumMismatch      -590    /* The previous log's accumulated segment checksum doesn't match the next log */

#define JET_wrnDatabaseRepaired                  595    /* Database corruption has been repaired */
#define JET_errPageInitializedMismatch          -596    /* Database divergence mismatch. Page was uninitialized on remote node, but initialized on local node. */


#define JET_errUnicodeTranslationBufferTooSmall -601    /* Unicode translation buffer too small */
#define JET_errUnicodeTranslationFail           -602    /* Unicode normalization failed */
#define JET_errUnicodeNormalizationNotSupported -603    /* OS does not provide support for Unicode normalisation (and no normalisation callback was specified) */
#define JET_errUnicodeLanguageValidationFailure -604    /* Can not validate the language */

#define JET_errExistingLogFileHasBadSignature   -610    /* Existing log file has bad signature */
#define JET_errExistingLogFileIsNotContiguous   -611    /* Existing log file is not contiguous */

#define JET_errLogReadVerifyFailure         -612  /* Checksum error in log file during backup */

#define JET_errCheckpointDepthTooDeep       -614    //  too many outstanding generations between checkpoint and current generation

#define JET_errRestoreOfNonBackupDatabase   -615    //  hard recovery attempted on a database that wasn't a backup database
#define JET_errLogFileNotCopied             -616    //  log truncation attempted but not all required logs were copied
#define JET_errTransactionTooLong           -618    //  Too many outstanding generations between JetBeginTransaction and current generation.

#define JET_errEngineFormatVersionNoLongerSupportedTooLow           -619 /* The specified JET_ENGINEFORMATVERSION value is too low to be supported by this version of ESE. */
#define JET_errEngineFormatVersionNotYetImplementedTooHigh          -620 /* The specified JET_ENGINEFORMATVERSION value is too high, higher than this version of ESE knows about. */
#define JET_errEngineFormatVersionParamTooLowForRequestedFeature    -621 /* Thrown by a format feature (not at JetSetSystemParameter) if the client requests a feature that requires a version higher than that set for the JET_paramEngineFormatVersion. */
#define JET_errEngineFormatVersionSpecifiedTooLowForLogVersion                      -622 /* The specified JET_ENGINEFORMATVERSION is set too low for this log stream, the log files have already been upgraded to a higher version.  A higher JET_ENGINEFORMATVERSION value must be set in the param. */
#define JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion                 -623 /* The specified JET_ENGINEFORMATVERSION is set too low for this database file, the database file has already been upgraded to a higher version.  A higher JET_ENGINEFORMATVERSION value must be set in the param. */
#define JET_errDbTimeBeyondMaxRequired      -625  /* dbtime on page greater than or equal to dbtimeAfter in record, but record is outside required range for the database */
#define JET_errLogOperationInconsistentWithDatabase -626 /* Log record in the log is inconsistent with the current state of the database and cannot be applied */
#define JET_errInsertKeyOutOfOrder          -627  /* The insert attempted was not placed in correct key order.  Possibly indicates transient memory issues. */
#define JET_errBackupAbortByServer          -801  /* Backup was aborted by server by calling JetTerm with JET_bitTermStopBackup or by calling JetStopBackup */

#define JET_errInvalidGrbit                 -900  /* Invalid flags parameter */

#define JET_errTermInProgress               -1000 /* Termination in progress */
#define JET_errFeatureNotAvailable          -1001 /* API not supported */
#define JET_errInvalidName                  -1002 /* Invalid name */
#define JET_errInvalidParameter             -1003 /* Invalid API parameter */
#define JET_wrnColumnNull                    1004 /* Column is NULL-valued */
#define JET_wrnBufferTruncated               1006 /* Buffer too small for data */
#define JET_wrnDatabaseAttached              1007 /* Database is already attached */
#define JET_errDatabaseFileReadOnly         -1008 /* Tried to attach a read-only database file for read/write operations */
#define JET_wrnSortOverflow                  1009 /* Sort does not fit in memory */
#define JET_errInvalidDatabaseId            -1010 /* Invalid database id */
#define JET_errOutOfMemory                  -1011 /* Out of Memory */
#define JET_errOutOfDatabaseSpace           -1012 /* Maximum database size reached */
#define JET_errOutOfCursors                 -1013 /* Out of table cursors */
#define JET_errOutOfBuffers                 -1014 /* Out of database page buffers */
#define JET_errTooManyIndexes               -1015 /* Too many indexes */
#define JET_errTooManyKeys                  -1016 /* Too many columns in an index */
#define JET_errRecordDeleted                -1017 /* Record has been deleted */
#define JET_errReadVerifyFailure            -1018 /* Checksum error on a database page */
#define JET_errPageNotInitialized           -1019 /* Blank database page */
#define JET_errOutOfFileHandles             -1020 /* Out of file handles */
#define JET_errDiskReadVerificationFailure  -1021 /* The OS returned ERROR_CRC from file IO */
#define JET_errDiskIO                       -1022 /* Disk IO error */
#define JET_errInvalidPath                  -1023 /* Invalid file path */
#define JET_errInvalidSystemPath            -1024 /* Invalid system path */
#define JET_errInvalidLogDirectory          -1025 /* Invalid log directory */
#define JET_errRecordTooBig                 -1026 /* Record larger than maximum size */
#define JET_errTooManyOpenDatabases         -1027 /* Too many open databases */
#define JET_errInvalidDatabase              -1028 /* Not a database file */
#define JET_errNotInitialized               -1029 /* Database engine not initialized */
#define JET_errAlreadyInitialized           -1030 /* Database engine already initialized */
#define JET_errInitInProgress               -1031 /* Database engine is being initialized */
#define JET_errFileAccessDenied             -1032 /* Cannot access file, the file is locked or in use */
#define JET_errBufferTooSmall               -1038 /* Buffer is too small */
#define JET_wrnSeekNotEqual                  1039 /* Exact match not found during seek */
#define JET_errTooManyColumns               -1040 /* Too many columns defined */
#define JET_errContainerNotEmpty            -1043 /* Container is not empty */
#define JET_errInvalidFilename              -1044 /* Filename is invalid */
#define JET_errInvalidBookmark              -1045 /* Invalid bookmark */
#define JET_errColumnInUse                  -1046 /* Column used in an index */
#define JET_errInvalidBufferSize            -1047 /* Data buffer doesn't match column size */
#define JET_errColumnNotUpdatable           -1048 /* Cannot set column value */
#define JET_errIndexInUse                   -1051 /* Index is in use */
#define JET_errLinkNotSupported             -1052 /* Link support unavailable */
#define JET_errNullKeyDisallowed            -1053 /* Null keys are disallowed on index */
#define JET_errNotInTransaction             -1054 /* Operation must be within a transaction */
#define JET_wrnNoErrorInfo                   1055 /* No extended error information */
#define JET_errMustRollback                 -1057 /* Transaction must rollback because failure of unversioned update */
#define JET_wrnNoIdleActivity                1058 /* No idle activity occurred */
#define JET_errTooManyActiveUsers           -1059 /* Too many active database users */
#define JET_errInvalidCountry               -1061 /* Invalid or unknown country/region code */
#define JET_errInvalidLanguageId            -1062 /* Invalid or unknown language id */
#define JET_errInvalidCodePage              -1063 /* Invalid or unknown code page */
#define JET_errInvalidLCMapStringFlags      -1064 /* Invalid flags for LCMapString() */
#define JET_errVersionStoreEntryTooBig      -1065 /* Attempted to create a version store entry (RCE) larger than a version bucket */
#define JET_errVersionStoreOutOfMemoryAndCleanupTimedOut    -1066 /* Version store out of memory (and cleanup attempt failed to complete) */
#define JET_wrnNoWriteLock                   1067 /* No write lock at transaction level 0 */
#define JET_wrnColumnSetNull                 1068 /* Column set to NULL-value */
#define JET_errVersionStoreOutOfMemory      -1069 /* Version store out of memory (cleanup already attempted) */
#define JET_errCannotIndex                  -1071 /* Cannot index escrow column */
#define JET_errRecordNotDeleted             -1072 /* Record has not been deleted */
#define JET_errTooManyMempoolEntries        -1073 /* Too many mempool entries requested */
#define JET_errOutOfObjectIDs               -1074 /* Out of btree ObjectIDs (perform offline defrag to reclaim freed/unused ObjectIds) */
#define JET_errOutOfLongValueIDs            -1075 /* Long-value ID counter has reached maximum value. (perform offline defrag to reclaim free/unused LongValueIDs) */
#define JET_errOutOfAutoincrementValues     -1076 /* Auto-increment counter has reached maximum value (offline defrag WILL NOT be able to reclaim free/unused Auto-increment values). */
#define JET_errOutOfDbtimeValues            -1077 /* Dbtime counter has reached maximum value (perform offline defrag to reclaim free/unused Dbtime values) */
#define JET_errOutOfSequentialIndexValues   -1078 /* Sequential index counter has reached maximum value (perform offline defrag to reclaim free/unused SequentialIndex values) */

#define JET_errRunningInOneInstanceMode     -1080 /* Multi-instance call with single-instance mode enabled */
#define JET_errRunningInMultiInstanceMode   -1081 /* Single-instance call with multi-instance mode enabled */
#define JET_errSystemParamsAlreadySet       -1082 /* Global system parameters have already been set */

#define JET_errSystemPathInUse              -1083 /* System path already used by another database instance */
#define JET_errLogFilePathInUse             -1084 /* Logfile path already used by another database instance */
#define JET_errTempPathInUse                -1085 /* Temp path already used by another database instance */
#define JET_errInstanceNameInUse            -1086 /* Instance Name already in use */
#define JET_errSystemParameterConflict      -1087 /* Global system parameters have already been set, but to a conflicting or disagreeable state to the specified values. */

#define JET_errInstanceUnavailable          -1090 /* This instance cannot be used because it encountered a fatal error */
#define JET_errDatabaseUnavailable          -1091 /* This database cannot be used because it encountered a fatal error */
#define JET_errInstanceUnavailableDueToFatalLogDiskFull -1092 /* This instance cannot be used because it encountered a log-disk-full error performing an operation (likely transaction rollback) that could not tolerate failure */
#define JET_errInvalidSesparamId            -1093 /* This JET_sesparam* identifier is not known to the ESE engine. */

#define JET_errTooManyRecords               -1094 /* There are too many records to enumerate, switch to an API that handles 64-bit numbers */

#define JET_errInvalidDbparamId             -1095 /* This JET_dbparam* identifier is not known to the ESE engine. */

#define JET_errOutOfSessions                -1101 /* Out of sessions */
#define JET_errWriteConflict                -1102 /* Write lock failed due to outstanding write lock */
#define JET_errTransTooDeep                 -1103 /* Transactions nested too deeply */
#define JET_errInvalidSesid                 -1104 /* Invalid session handle */
#define JET_errWriteConflictPrimaryIndex    -1105 /* Update attempted on uncommitted primary index */
#define JET_errInTransaction                -1108 /* Operation not allowed within a transaction */
#define JET_errRollbackRequired             -1109 /* Must rollback current transaction -- cannot commit or begin a new one */
#define JET_errTransReadOnly                -1110 /* Read-only transaction tried to modify the database */
#define JET_errSessionWriteConflict         -1111 /* Attempt to replace the same record by two different cursors in the same session */

#define JET_errRecordTooBigForBackwardCompatibility             -1112 /* record would be too big if represented in a database format from a previous version of Jet */
#define JET_errCannotMaterializeForwardOnlySort                 -1113 /* The temp table could not be created due to parameters that conflict with JET_bitTTForwardOnly */

#define JET_errSesidTableIdMismatch         -1114 /* This session handle can't be used with this table id */
#define JET_errInvalidInstance              -1115 /* Invalid instance handle */
#define JET_errDirtyShutdown                -1116 /* The instance was shutdown successfully but all the attached databases were left in a dirty state by request via JET_bitTermDirty */
// unused -1117
#define JET_errReadPgnoVerifyFailure        -1118 /* The database page read from disk had the wrong page number. */
#define JET_errReadLostFlushVerifyFailure   -1119 /* The database page read from disk had a previous write not represented on the page. */
#define JET_errFileSystemCorruption             -1121 /* File system operation failed with an error indicating the file system is corrupt. */
#define JET_wrnShrinkNotPossible                1122 /* Database file could not be shrunk because there is not enough internal free space available or there is unmovable data present. */
#define JET_errRecoveryVerifyFailure            -1123 /* One or more database pages read from disk during recovery do not match the expected state. */

#define JET_errFilteredMoveNotSupported         -1124 /* Attempted to provide a filter to JetSetCursorFilter() in an unsupported scenario. */


#define JET_errDatabaseDuplicate            -1201 /* Database already exists */
#define JET_errDatabaseInUse                -1202 /* Database in use */
#define JET_errDatabaseNotFound             -1203 /* No such database */
#define JET_errDatabaseInvalidName          -1204 /* Invalid database name */
#define JET_errDatabaseInvalidPages         -1205 /* Invalid number of pages */
#define JET_errDatabaseCorrupted            -1206 /* Non database file or corrupted db */
#define JET_errDatabaseLocked               -1207 /* Database exclusively locked */
#define JET_errCannotDisableVersioning      -1208 /* Cannot disable versioning for this database */
#define JET_errInvalidDatabaseVersion       -1209 /* Database engine is incompatible with database */

#define JET_errDatabase200Format            -1210 /* The database is in an older (200) format */
#define JET_errDatabase400Format            -1211 /* The database is in an older (400) format */
#define JET_errDatabase500Format            -1212 /* The database is in an older (500) format */

#define JET_errPageSizeMismatch             -1213 /* The database page size does not match the engine */
#define JET_errTooManyInstances             -1214 /* Cannot start any more database instances */
#define JET_errDatabaseSharingViolation     -1215 /* A different database instance is using this database */
#define JET_errAttachedDatabaseMismatch     -1216 /* An outstanding database attachment has been detected at the start or end of recovery, but database is missing or does not match attachment info */
#define JET_errDatabaseInvalidPath          -1217 /* Specified path to database file is illegal */
#define JET_errDatabaseIdInUse              -1218 /* A database is being assigned an id already in use */
#define JET_errForceDetachNotAllowed        -1219 /* Force Detach allowed only after normal detach errored out */
#define JET_errCatalogCorrupted             -1220 /* Corruption detected in catalog */
#define JET_errPartiallyAttachedDB          -1221 /* Database is partially attached. Cannot complete attach operation */
#define JET_errDatabaseSignInUse            -1222 /* Database with same signature in use */

#define JET_errDatabaseCorruptedNoRepair    -1224 /* Corrupted db but repair not allowed */
#define JET_errInvalidCreateDbVersion       -1225 /* recovery tried to replay a database creation, but the database was originally created with an incompatible (likely older) version of the database engine */


#define JET_errDatabaseNotReady             -1230 /* Recovery on this database has not yet completed enough to permit access. */
#define JET_errDatabaseAttachedForRecovery  -1231 /* Database is attached but only for recovery.  It must be explicitly attached before it can be opened. */
#define JET_errTransactionsNotReadyDuringRecovery -1232  /* Recovery has not seen any Begin0/Commit0 records and so does not know what trxBegin0 to assign to this transaction */


#define JET_wrnTableEmpty                    1301 /* Opened an empty table */
#define JET_errTableLocked                  -1302 /* Table is exclusively locked */
#define JET_errTableDuplicate               -1303 /* Table already exists */
#define JET_errTableInUse                   -1304 /* Table is in use, cannot lock */
#define JET_errObjectNotFound               -1305 /* No such table or object */
#define JET_errDensityInvalid               -1307 /* Bad file/index density */
#define JET_errTableNotEmpty                -1308 /* Table is not empty */
#define JET_errInvalidTableId               -1310 /* Invalid table id */
#define JET_errTooManyOpenTables            -1311 /* Cannot open any more tables (cleanup already attempted) */
#define JET_errIllegalOperation             -1312 /* Oper. not supported on table */
#define JET_errTooManyOpenTablesAndCleanupTimedOut  -1313 /* Cannot open any more tables (cleanup attempt failed to complete) */
#define JET_errObjectDuplicate              -1314 /* Table or object name in use */
#define JET_errInvalidObject                -1316 /* Object is invalid for operation */
#define JET_errCannotDeleteTempTable        -1317 /* Use CloseTable instead of DeleteTable to delete temp table */
#define JET_errCannotDeleteSystemTable      -1318 /* Illegal attempt to delete a system table */
#define JET_errCannotDeleteTemplateTable    -1319 /* Illegal attempt to delete a template table */
#define JET_errExclusiveTableLockRequired   -1322 /* Must have exclusive lock on table. */
#define JET_errFixedDDL                     -1323 /* DDL operations prohibited on this table */
#define JET_errFixedInheritedDDL            -1324 /* On a derived table, DDL operations are prohibited on inherited portion of DDL */
#define JET_errCannotNestDDL                -1325 /* Nesting of hierarchical DDL is not currently supported. */
#define JET_errDDLNotInheritable            -1326 /* Tried to inherit DDL from a table not marked as a template table. */
#define JET_wrnTableInUseBySystem            1327 /* System cleanup has a cursor open on the table */
#define JET_errInvalidSettings              -1328 /* System parameters were set improperly */
#define JET_errClientRequestToStopJetService            -1329   /* Client has requested stop service */
#define JET_errCannotAddFixedVarColumnToDerivedTable    -1330   /* Template table was created with NoFixedVarColumnsInDerivedTables */

/*  DDL errors
/**/
// Note: Some DDL errors have snuck into other categories.
#define JET_errIndexCantBuild               -1401 /* Index build failed */
#define JET_errIndexHasPrimary              -1402 /* Primary index already defined */
#define JET_errIndexDuplicate               -1403 /* Index is already defined */
#define JET_errIndexNotFound                -1404 /* No such index */
#define JET_errIndexMustStay                -1405 /* Cannot delete clustered index */
#define JET_errIndexInvalidDef              -1406 /* Illegal index definition */
#define JET_errInvalidCreateIndex           -1409 /* Invalid create index description */
#define JET_errTooManyOpenIndexes           -1410 /* Out of index description blocks */
#define JET_errMultiValuedIndexViolation    -1411 /* Non-unique inter-record index keys generated for a multivalued index */
#define JET_errIndexBuildCorrupted          -1412 /* Failed to build a secondary index that properly reflects primary index */
#define JET_errPrimaryIndexCorrupted        -1413 /* Primary index is corrupt. The database must be defragmented or the table deleted. */
#define JET_errSecondaryIndexCorrupted      -1414 /* Secondary index is corrupt. The database must be defragmented or the affected index must be deleted. If the corrupt index is over Unicode text, a likely cause is a sort-order change. */
#define JET_wrnCorruptIndexDeleted           1415 /* Out of date index removed */
#define JET_errInvalidIndexId               -1416 /* Illegal index id */
#define JET_wrnPrimaryIndexOutOfDate         1417 /* The Primary index is created with an incompatible OS sort version. The table can not be safely modified. */
#define JET_wrnSecondaryIndexOutOfDate       1418 /* One or more Secondary index is created with an incompatible OS sort version. Any index over Unicode text should be deleted. */

#define JET_errIndexTuplesSecondaryIndexOnly        -1430   //  tuple index can only be on a secondary index
#define JET_errIndexTuplesTooManyColumns            -1431   //  tuple index may only have eleven columns in the index
#define JET_errIndexTuplesOneColumnOnly             JET_errIndexTuplesTooManyColumns    /* OBSOLETE */
#define JET_errIndexTuplesNonUniqueOnly             -1432   //  tuple index must be a non-unique index
#define JET_errIndexTuplesTextBinaryColumnsOnly     -1433   //  tuple index must be on a text/binary column
#define JET_errIndexTuplesTextColumnsOnly           JET_errIndexTuplesTextBinaryColumnsOnly     /* OBSOLETE */
#define JET_errIndexTuplesVarSegMacNotAllowed       -1434   //  tuple index does not allow setting cbVarSegMac
#define JET_errIndexTuplesInvalidLimits             -1435   //  invalid min/max tuple length or max characters to index specified
#define JET_errIndexTuplesCannotRetrieveFromIndex   -1436   //  cannot call RetrieveColumn() with RetrieveFromIndex on a tuple index
#define JET_errIndexTuplesKeyTooSmall               -1437   //  specified key does not meet minimum tuple length
#define JET_errInvalidLVChunkSize                   -1438   //  Specified LV chunk size is not supported
#define JET_errColumnCannotBeEncrypted              -1439   //  Only JET_coltypLongText and JET_coltypLongBinary columns without default values can be encrypted
#define JET_errCannotIndexOnEncryptedColumn         -1440   //  Cannot index encrypted column

/*  DML errors
/**/
// Note: Some DML errors have snuck into other categories.
// Note: Some DDL errors have inappropriately snuck in here.
#define JET_errColumnLong                   -1501 /* Column value is long */
#define JET_errColumnNoChunk                -1502 /* No such chunk in long value */
#define JET_errColumnDoesNotFit             -1503 /* Field will not fit in record */
#define JET_errNullInvalid                  -1504 /* Null not valid */
#define JET_errColumnIndexed                -1505 /* Column indexed, cannot delete */
#define JET_errColumnTooBig                 -1506 /* Field length is greater than maximum */
#define JET_errColumnNotFound               -1507 /* No such column */
#define JET_errColumnDuplicate              -1508 /* Field is already defined */
#define JET_errMultiValuedColumnMustBeTagged    -1509 /* Attempted to create a multi-valued column, but column was not Tagged */
#define JET_errColumnRedundant              -1510 /* Second autoincrement or version column */
#define JET_errInvalidColumnType            -1511 /* Invalid column data type */
#define JET_wrnColumnMaxTruncated            1512 /* Max length too big, truncated */
#define JET_errTaggedNotNULL                -1514 /* No non-NULL tagged columns */
#define JET_errNoCurrentIndex               -1515 /* Invalid w/o a current index */
#define JET_errKeyIsMade                    -1516 /* The key is completely made */
#define JET_errBadColumnId                  -1517 /* Column Id Incorrect */
#define JET_errBadItagSequence              -1518 /* Bad itagSequence for tagged column */
#define JET_errColumnInRelationship         -1519 /* Cannot delete, column participates in relationship */
#define JET_wrnCopyLongValue                 1520 /* Single instance column bursted */
#define JET_errCannotBeTagged               -1521 /* AutoIncrement and Version cannot be tagged */
#define JET_errDefaultValueTooBig           -1524 /* Default value exceeds maximum size */
#define JET_errMultiValuedDuplicate         -1525 /* Duplicate detected on a unique multi-valued column */
#define JET_errLVCorrupted                  -1526 /* Corruption encountered in long-value tree */
#define JET_errMultiValuedDuplicateAfterTruncation  -1528 /* Duplicate detected on a unique multi-valued column after data was normalized, and normalizing truncated the data before comparison */
#define JET_errDerivedColumnCorruption      -1529 /* Invalid column in derived table */
#define JET_errInvalidPlaceholderColumn     -1530 /* Tried to convert column to a primary index placeholder, but column doesn't meet necessary criteria */
#define JET_wrnColumnSkipped                 1531 /* Column value(s) not returned because the corresponding column id or itagSequence requested for enumeration was null */
#define JET_wrnColumnNotLocal                1532 /* Column value(s) not returned because they could not be reconstructed from the data at hand */
#define JET_wrnColumnMoreTags                1533 /* Column values exist that were not requested for enumeration */
#define JET_wrnColumnTruncated               1534 /* Column value truncated at the requested size limit during enumeration */
#define JET_wrnColumnPresent                 1535 /* Column values exist but were not returned by request */
#define JET_wrnColumnSingleValue             1536 /* Column value returned in JET_COLUMNENUM as a result of JET_bitEnumerateCompressOutput */
#define JET_wrnColumnDefault                 1537 /* Column value(s) not returned because they were set to their default value(s) and JET_bitEnumerateIgnoreDefault was specified */
#define JET_errColumnCannotBeCompressed     -1538 /* Only JET_coltypLongText and JET_coltypLongBinary columns can be compressed */
#define JET_wrnColumnNotInRecord             1539 /* Column value(s) not returned because they could not be reconstructed from the data in the record */
#define JET_errColumnNoEncryptionKey        -1540 /* Cannot retrieve/set encrypted column without an encryption key */
#define JET_wrnColumnReference               1541 /* Column value returned as a reference because it could not be reconstructed from the data in the record */

#define JET_errRecordNotFound               -1601 /* The key was not found */
#define JET_errRecordNoCopy                 -1602 /* No working buffer */
#define JET_errNoCurrentRecord              -1603 /* Currency not on a record */
#define JET_errRecordPrimaryChanged         -1604 /* Primary key may not change */
#define JET_errKeyDuplicate                 -1605 /* Illegal duplicate key */
#define JET_errAlreadyPrepared              -1607 /* Attempted to update record when record update was already in progress */
#define JET_errKeyNotMade                   -1608 /* No call to JetMakeKey */
#define JET_errUpdateNotPrepared            -1609 /* No call to JetPrepareUpdate */
#define JET_wrnDataHasChanged                1610 /* Data has changed */
#define JET_errDataHasChanged               -1611 /* Data has changed, operation aborted */
#define JET_wrnKeyChanged                    1618 /* Moved to new key */
#define JET_errLanguageNotSupported         -1619 /* Windows installation does not support language */
#define JET_errDecompressionFailed          -1620 /* Internal error: data could not be decompressed */
#define JET_errUpdateMustVersion            -1621 /* No version updates only for uncommitted tables */
#define JET_errDecryptionFailed             -1622 /* Data could not be decrypted */
#define JET_errEncryptionBadItag            -1623 /* Cannot encrypt tagged columns with itag>1 */
#define JET_errSetAutoIncrementTooHigh      -1624  /* The auto-increment value that the user tried to set explicitly is too high . */
#define JET_errAutoIncrementNotSet          -1625  /* The user must have explicitly set the auto-increment column for this table. */

/*  Sort Table errors
/**/
#define JET_errTooManySorts                 -1701 /* Too many sort processes */
#define JET_errInvalidOnSort                -1702 /* Invalid operation on Sort */

/*  Other errors
/**/
#define JET_errTempFileOpenError            -1803 /* Temp file could not be opened */
#define JET_errTooManyAttachedDatabases     -1805 /* Too many open databases */
#define JET_errDiskFull                     -1808 /* No space left on disk */
#define JET_errPermissionDenied             -1809 /* Permission denied */
#define JET_errFileNotFound                 -1811 /* File not found */
#define JET_errFileInvalidType              -1812 /* Invalid file type */
#define JET_wrnFileOpenReadOnly              1813 /* Database file is read only */
#define JET_errFileAlreadyExists            -1814 /* File already exists */


#define JET_errAfterInitialization          -1850 /* Cannot Restore after init. */
#define JET_errLogCorrupted                 -1852 /* Logs could not be interpreted */


#define JET_errInvalidOperation             -1906 /* Invalid operation */
#define JET_errAccessDenied                 -1907 /* Access denied */
#define JET_wrnIdleFull                      1908 /* Idle registry full */
#define JET_errTooManySplits                -1909 /* Infinite split */
#define JET_errSessionSharingViolation      -1910 /* Multiple threads are using the same session */
#define JET_errEntryPointNotFound           -1911 /* An entry point in a DLL we require could not be found */
#define JET_errSessionContextAlreadySet     -1912 /* Specified session already has a session context set */
#define JET_errSessionContextNotSetByThisThread -1913 /* Tried to reset session context, but current thread did not originally set the session context */
#define JET_errSessionInUse                 -1914 /* Tried to terminate session in use */
#define JET_errRecordFormatConversionFailed -1915 /* Internal error during dynamic record format conversion */
#define JET_errOneDatabasePerSession        -1916 /* Just one open user database per session is allowed (JET_paramOneDatabasePerSession) */
#define JET_errRollbackError                -1917 /* error during rollback */
#define JET_errFlushMapVersionUnsupported   -1918 /* The version of the persisted flush map is not supported by this version of the engine. */
#define JET_errFlushMapDatabaseMismatch     -1919 /* The persisted flush map and the database do not match. */
#define JET_errFlushMapUnrecoverable        -1920 /* The persisted flush map cannot be reconstructed. */


#define JET_wrnDefragAlreadyRunning          2000 /* Online defrag already running on specified database */
#define JET_wrnDefragNotRunning              2001 /* Online defrag not running on specified database */
#define JET_errDatabaseAlreadyRunningMaintenance -2004  /* The operation did not complete successfully because the database is already running maintenance on specified database */


#define JET_wrnCallbackNotRegistered         2100 /* Unregistered a non-existent callback function */
#define JET_errCallbackFailed               -2101 /* A callback failed */
#define JET_errCallbackNotResolved          -2102 /* A callback function could not be found */

#define JET_errSpaceHintsInvalid            -2103 /* An element of the JET space hints structure was not correct or actionable. */

#define JET_errOSSnapshotInvalidSequence    -2401 /* OS Shadow copy API used in an invalid sequence */
#define JET_errOSSnapshotTimeOut            -2402 /* OS Shadow copy ended with time-out */
#define JET_errOSSnapshotNotAllowed         -2403 /* OS Shadow copy not allowed (backup or recovery in progress) */
#define JET_errOSSnapshotInvalidSnapId      -2404 /* invalid JET_OSSNAPID */


/** KVP ERRORS
 **/

#define JET_errLSCallbackNotSpecified       -3000 /* Attempted to use Local Storage without a callback function being specified */
#define JET_errLSAlreadySet                 -3001 /* Attempted to set Local Storage for an object which already had it set */
#define JET_errLSNotSet                     -3002 /* Attempted to retrieve Local Storage from an object which didn't have it set */

/** FILE and DISK ERRORS
 **/
//JET_errFileAccessDenied                   -1032
//JET_errFileNotFound                       -1811
//JET_errInvalidFilename                    -1044
#define JET_errFileIOSparse                 -4000 /* an I/O was issued to a location that was sparse */
#define JET_errFileIOBeyondEOF              -4001 /* a read was issued to a location beyond EOF (writes will expand the file) */
#define JET_errFileIOAbort                  -4002 /* instructs the JET_ABORTRETRYFAILCALLBACK caller to abort the specified I/O */
#define JET_errFileIORetry                  -4003 /* instructs the JET_ABORTRETRYFAILCALLBACK caller to retry the specified I/O */
#define JET_errFileIOFail                   -4004 /* instructs the JET_ABORTRETRYFAILCALLBACK caller to fail the specified I/O */
#define JET_errFileCompressed               -4005 /* read/write access is not supported on compressed files */

/** CLIENT RESERVED ERROR SPACE.
    An unused errors/warnings section.  JET will never generate values in this space.  Clients may use this space
        without conflicting with ESE.  Note that the warnings are reserved as well as the errors.  That is, the
        range from -10,000 to -11,999 is reserved as well as the range from 10,000 to 11,999.
 **/
#define JET_errClientSpaceBegin             -10000 /* Begin of the error space reserved for JET client use */
#define JET_errClientSpaceEnd               -11999 /* End of the error space reserved for JET client use */

/**********************************************************************/
/***********************     PROTOTYPES      **************************/
/**********************************************************************/

#if !defined(_JET_NOPROTOTYPES)

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetInit(
    _Inout_opt_ JET_INSTANCE *  pinstance );


#if ( JET_VERSION >= 0x0501 )
JET_ERR JET_API
JetInit2(
    _Inout_opt_ JET_INSTANCE *  pinstance,
    _In_ JET_GRBIT              grbit );

#endif // JET_VERSION >= 0x0501

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )
#if ( JET_VERSION < 0x0600 )
#define JetInit3A JetInit3
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetInit3A(
    _Inout_opt_ JET_INSTANCE *  pinstance,
    _In_opt_ JET_RSTINFO_A *    prstInfo,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetInit3W(
    _Inout_opt_ JET_INSTANCE *  pinstance,
    _In_opt_ JET_RSTINFO_W *    prstInfo,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetInit3 JetInit3W
#else
#define JetInit3 JetInit3A
#endif
#endif


#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetCreateInstanceA JetCreateInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateInstanceA(
    _Out_ JET_INSTANCE *    pinstance,
    _In_opt_ JET_PCSTR      szInstanceName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateInstanceW(
    _Out_ JET_INSTANCE *    pinstance,
    _In_opt_ JET_PCWSTR     szInstanceName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateInstance JetCreateInstanceW
#else
#define JetCreateInstance JetCreateInstanceA
#endif
#endif

#if ( JET_VERSION < 0x0600 )
#define JetCreateInstance2A JetCreateInstance2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateInstance2A(
    _Out_ JET_INSTANCE *    pinstance,
    _In_opt_ JET_PCSTR      szInstanceName,
    _In_opt_ JET_PCSTR      szDisplayName,
    _In_ JET_GRBIT          grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateInstance2W(
    _Out_ JET_INSTANCE *    pinstance,
    _In_opt_ JET_PCWSTR     szInstanceName,
    _In_opt_ JET_PCWSTR     szDisplayName,
    _In_ JET_GRBIT          grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateInstance2 JetCreateInstance2W
#else
#define JetCreateInstance2 JetCreateInstance2A
#endif
#endif

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetInstanceMiscInfo(
    _In_ JET_INSTANCE                     instance,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0600

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetTerm(
    _In_ JET_INSTANCE   instance );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetTerm2(
    _In_ JET_INSTANCE   instance,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetStopService();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetStopServiceInstance(
    _In_ JET_INSTANCE   instance );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0602 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetStopServiceInstance2(
    _In_ JET_INSTANCE       instance,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0602

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetStopBackup();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetStopBackupInstance(
    _In_ JET_INSTANCE   instance );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION < 0x0600 )
#define JetSetSystemParameterA JetSetSystemParameter
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetSystemParameterA(
    _Inout_opt_ JET_INSTANCE *  pinstance,
    _In_opt_ JET_SESID          sesid,
    _In_ JET_UINT32             paramid,
    _In_opt_ JET_API_PTR        lParam,
    _In_opt_ JET_PCSTR          szParam );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetSystemParameterW(
    _Inout_opt_ JET_INSTANCE *  pinstance,
    _In_opt_ JET_SESID          sesid,
    _In_ JET_UINT32             paramid,
    _In_opt_ JET_API_PTR        lParam,
    _In_opt_ JET_PCWSTR         szParam );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetSystemParameter JetSetSystemParameterW
#else
#define JetSetSystemParameter JetSetSystemParameterA
#endif
#endif

#if ( JET_VERSION < 0x0600 )
#define JetGetSystemParameterA JetGetSystemParameter
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetSystemParameterA(
    _In_ JET_INSTANCE                   instance,
    _In_opt_ JET_SESID                  sesid,
    _In_ JET_UINT32                     paramid,
    _Out_opt_ JET_API_PTR *             plParam,
    _Out_writes_bytes_opt_( cbMax ) JET_PSTR    szParam,
    _In_ JET_UINT32                     cbMax );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetSystemParameterW(
    _In_ JET_INSTANCE                   instance,
    _In_opt_ JET_SESID                  sesid,
    _In_ JET_UINT32                     paramid,
    _Out_opt_ JET_API_PTR *             plParam,
    _Out_writes_bytes_opt_( cbMax ) JET_PWSTR   szParam,
    _In_ JET_UINT32                     cbMax );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetSystemParameter JetGetSystemParameterW
#else
#define JetGetSystemParameter JetGetSystemParameterA
#endif
#endif


#if ( JET_VERSION >= 0x0501 )

#if ( JET_VERSION < 0x0600 )
#define JetEnableMultiInstanceA JetEnableMultiInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEnableMultiInstanceA(
    _In_reads_opt_( csetsysparam ) JET_SETSYSPARAM_A *  psetsysparam,
    _In_ JET_UINT32                                     csetsysparam,
    _Out_opt_ JET_UINT32 *                              pcsetsucceed );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEnableMultiInstanceW(
    _In_reads_opt_( csetsysparam ) JET_SETSYSPARAM_W *  psetsysparam,
    _In_ JET_UINT32                                     csetsysparam,
    _Out_opt_ JET_UINT32 *                              pcsetsucceed );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetEnableMultiInstance JetEnableMultiInstanceW
#else
#define JetEnableMultiInstance JetEnableMultiInstanceA
#endif
#endif

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetThreadStats(
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION < 0x0600 )
#define JetBeginSessionA JetBeginSession
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetBeginSessionA(
    _In_ JET_INSTANCE   instance,
    _Out_ JET_SESID *   psesid,
    _In_opt_ JET_PCSTR  szUserName,
    _In_opt_ JET_PCSTR  szPassword );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetBeginSessionW(
    _In_ JET_INSTANCE   instance,
    _Out_ JET_SESID *   psesid,
    _In_opt_ JET_PCWSTR szUserName,
    _In_opt_ JET_PCWSTR szPassword );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetBeginSession JetBeginSessionW
#else
#define JetBeginSession JetBeginSessionA
#endif
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDupSession(
    _In_ JET_SESID      sesid,
    _Out_ JET_SESID *   psesid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEndSession(
    _In_ JET_SESID  sesid,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion


#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetVersion(
    _In_ JET_SESID          sesid,
    _Out_ JET_UINT32 *      pwVersion );

JET_ERR JET_API
JetIdle(
    _In_ JET_SESID  sesid,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion


#if ( JET_VERSION < 0x0600 )
#define JetCreateDatabaseA JetCreateDatabase
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateDatabaseA(
    _In_ JET_SESID      sesid,
    _In_ JET_PCSTR      szFilename,
    _In_opt_ JET_PCSTR  szConnect,
    _Out_ JET_DBID *    pdbid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateDatabaseW(
    _In_ JET_SESID      sesid,
    _In_ JET_PCWSTR     szFilename,
    _In_opt_ JET_PCWSTR szConnect,
    _Out_ JET_DBID *    pdbid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateDatabase JetCreateDatabaseW
#else
#define JetCreateDatabase JetCreateDatabaseA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetCreateDatabase2A JetCreateDatabase2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateDatabase2A(
    _In_ JET_SESID              sesid,
    _In_ JET_PCSTR              szFilename,
    _In_ const JET_UINT32       cpgDatabaseSizeMax,
    _Out_ JET_DBID *            pdbid,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetCreateDatabase2W(
    _In_ JET_SESID              sesid,
    _In_ JET_PCWSTR             szFilename,
    _In_ const JET_UINT32       cpgDatabaseSizeMax,
    _Out_ JET_DBID *            pdbid,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateDatabase2 JetCreateDatabase2W
#else
#define JetCreateDatabase2 JetCreateDatabase2A
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetAttachDatabaseA JetAttachDatabase
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetAttachDatabaseA(
    _In_ JET_SESID  sesid,
    _In_ JET_PCSTR  szFilename,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetAttachDatabaseW(
    _In_ JET_SESID  sesid,
    _In_ JET_PCWSTR szFilename,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetAttachDatabase JetAttachDatabaseW
#else
#define JetAttachDatabase JetAttachDatabaseA
#endif
#endif

#if ( JET_VERSION < 0x0600 )
#define JetAttachDatabase2A JetAttachDatabase2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetAttachDatabase2A(
    _In_ JET_SESID              sesid,
    _In_ JET_PCSTR              szFilename,
    _In_ const JET_UINT32       cpgDatabaseSizeMax,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetAttachDatabase2W(
    _In_ JET_SESID              sesid,
    _In_ JET_PCWSTR             szFilename,
    _In_ const JET_UINT32       cpgDatabaseSizeMax,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetAttachDatabase2 JetAttachDatabase2W
#else
#define JetAttachDatabase2 JetAttachDatabase2A
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetDetachDatabaseA JetDetachDatabase
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDetachDatabaseA(
    _In_ JET_SESID  sesid,
    _In_opt_ JET_PCSTR  szFilename );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDetachDatabaseW(
    _In_ JET_SESID  sesid,
    _In_opt_ JET_PCWSTR szFilename );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDetachDatabase JetDetachDatabaseW
#else
#define JetDetachDatabase JetDetachDatabaseA
#endif
#endif

#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetDetachDatabase2A JetDetachDatabase2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDetachDatabase2A(
    _In_ JET_SESID  sesid,
    _In_opt_ JET_PCSTR  szFilename,
    _In_ JET_GRBIT  grbit);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDetachDatabase2W(
    _In_ JET_SESID  sesid,
    _In_opt_ JET_PCWSTR szFilename,
    _In_ JET_GRBIT  grbit);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDetachDatabase2 JetDetachDatabase2W
#else
#define JetDetachDatabase2 JetDetachDatabase2A
#endif
#endif

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetGetObjectInfoA JetGetObjectInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetObjectInfoA(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_OBJTYP                       objtyp,
    _In_opt_ JET_PCSTR                    szContainerName,
    _In_opt_ JET_PCSTR                    szObjectName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetObjectInfoW(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_OBJTYP                       objtyp,
    _In_opt_ JET_PCWSTR                   szContainerName,
    _In_opt_ JET_PCWSTR                   szObjectName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetObjectInfo JetGetObjectInfoW
#else
#define JetGetObjectInfo JetGetObjectInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetGetTableInfoA JetGetTableInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTableInfoA(
    _In_ JET_SESID                        sesid,
    _In_ JET_TABLEID                      tableid,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTableInfoW(
    _In_ JET_SESID                        sesid,
    _In_ JET_TABLEID                      tableid,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetTableInfo JetGetTableInfoW
#else
#define JetGetTableInfo JetGetTableInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetCreateTableA JetCreateTable
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableA(
    _In_ JET_SESID       sesid,
    _In_ JET_DBID        dbid,
    _In_ JET_PCSTR       szTableName,
    _In_ JET_UINT32      lPages,
    _In_ JET_UINT32      lDensity,
    _Out_ JET_TABLEID *  ptableid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableW(
    _In_ JET_SESID       sesid,
    _In_ JET_DBID        dbid,
    _In_ JET_PCWSTR      szTableName,
    _In_ JET_UINT32      lPages,
    _In_ JET_UINT32      lDensity,
    _Out_ JET_TABLEID *  ptableid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateTable JetCreateTableW
#else
#define JetCreateTable JetCreateTableA
#endif
#endif

#if ( JET_VERSION < 0x0600 )
#define JetCreateTableColumnIndexA JetCreateTableColumnIndex
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndexA(
    _In_ JET_SESID               sesid,
    _In_ JET_DBID                dbid,
    _Inout_ JET_TABLECREATE_A *  ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndexW(
    _In_ JET_SESID               sesid,
    _In_ JET_DBID                dbid,
    _Inout_ JET_TABLECREATE_W *  ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateTableColumnIndex JetCreateTableColumnIndexW
#else
#define JetCreateTableColumnIndex JetCreateTableColumnIndexA
#endif
#endif


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetCreateTableColumnIndex2A JetCreateTableColumnIndex2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndex2A(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE2_A *    ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndex2W(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE2_W *    ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateTableColumnIndex2 JetCreateTableColumnIndex2W
#else
#define JetCreateTableColumnIndex2 JetCreateTableColumnIndex2A
#endif
#endif // JET_VERSION >= 0x0600
#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0601 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndex3A(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE3_A *    ptablecreate );

JET_ERR JET_API
JetCreateTableColumnIndex3W(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE3_W *    ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateTableColumnIndex3 JetCreateTableColumnIndex3W
#else
#define JetCreateTableColumnIndex3 JetCreateTableColumnIndex3A
#endif
#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0602 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndex4A(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE4_A *    ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateTableColumnIndex4W(
    _In_ JET_SESID                  sesid,
    _In_ JET_DBID                   dbid,
    _Inout_ JET_TABLECREATE4_W *    ptablecreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateTableColumnIndex4 JetCreateTableColumnIndex4W
#else
#define JetCreateTableColumnIndex4 JetCreateTableColumnIndex4A
#endif
#endif // JET_VERSION >= 0x0602


#if ( JET_VERSION < 0x0600 )
#define JetDeleteTableA JetDeleteTable
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteTableA(
    _In_ JET_SESID  sesid,
    _In_ JET_DBID   dbid,
    _In_ JET_PCSTR  szTableName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteTableW(
    _In_ JET_SESID  sesid,
    _In_ JET_DBID   dbid,
    _In_ JET_PCWSTR szTableName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDeleteTable JetDeleteTableW
#else
#define JetDeleteTable JetDeleteTableA
#endif
#endif
#if ( JET_VERSION < 0x0600 )
#define JetRenameTableA JetRenameTable
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRenameTableA(
    _In_ JET_SESID  sesid,
    _In_ JET_DBID   dbid,
    _In_ JET_PCSTR  szName,
    _In_ JET_PCSTR  szNameNew );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetRenameTableW(
    _In_ JET_SESID  sesid,
    _In_ JET_DBID   dbid,
    _In_ JET_PCWSTR szName,
    _In_ JET_PCWSTR szNameNew );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetRenameTable JetRenameTableW
#else
#define JetRenameTable JetRenameTableA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetGetTableColumnInfoA JetGetTableColumnInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTableColumnInfoA(
    _In_ JET_SESID                        sesid,
    _In_ JET_TABLEID                      tableid,
    _In_opt_ JET_PCSTR                    szColumnName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetGetTableColumnInfoW(
    _In_ JET_SESID                        sesid,
    _In_ JET_TABLEID                      tableid,
    _In_opt_ JET_PCWSTR                   szColumnName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetTableColumnInfo JetGetTableColumnInfoW
#else
#define JetGetTableColumnInfo JetGetTableColumnInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetGetColumnInfoA JetGetColumnInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetColumnInfoA(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_PCSTR                        szTableName,
    _In_opt_ JET_PCSTR                    pColumnNameOrId,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetGetColumnInfoW(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_PCWSTR                       szTableName,
    _In_opt_ JET_PCWSTR                   pwColumnNameOrId,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetColumnInfo JetGetColumnInfoW
#else
#define JetGetColumnInfo JetGetColumnInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetAddColumnA JetAddColumn
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetAddColumnA(
    _In_ JET_SESID                               sesid,
    _In_ JET_TABLEID                             tableid,
    _In_ JET_PCSTR                               szColumnName,
    _In_ const JET_COLUMNDEF *                   pcolumndef,
    _In_reads_bytes_opt_( cbDefault ) JET_PCVOID pvDefault,
    _In_ JET_UINT32                              cbDefault,
    _Out_opt_ JET_COLUMNID *                     pcolumnid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetAddColumnW(
    _In_ JET_SESID                               sesid,
    _In_ JET_TABLEID                             tableid,
    _In_ JET_PCWSTR                              szColumnName,
    _In_ const JET_COLUMNDEF *                   pcolumndef,
    _In_reads_bytes_opt_( cbDefault ) JET_PCVOID pvDefault,
    _In_ JET_UINT32                              cbDefault,
    _Out_opt_ JET_COLUMNID *                     pcolumnid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetAddColumn JetAddColumnW
#else
#define JetAddColumn JetAddColumnA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetDeleteColumnA JetDeleteColumn
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteColumnA(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCSTR      szColumnName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteColumnW(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCWSTR     szColumnName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDeleteColumn JetDeleteColumnW
#else
#define JetDeleteColumn JetDeleteColumnA
#endif
#endif


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetDeleteColumn2A JetDeleteColumn2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteColumn2A(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _In_ JET_PCSTR          szColumnName,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteColumn2W(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _In_ JET_PCWSTR         szColumnName,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDeleteColumn2 JetDeleteColumn2W
#else
#define JetDeleteColumn2 JetDeleteColumn2A
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetRenameColumnA JetRenameColumn
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRenameColumnA(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCSTR      szName,
    _In_ JET_PCSTR      szNameNew,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRenameColumnW(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCWSTR     szName,
    _In_ JET_PCWSTR     szNameNew,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetRenameColumn JetRenameColumnW
#else
#define JetRenameColumn JetRenameColumnA
#endif
#endif


#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetSetColumnDefaultValueA JetSetColumnDefaultValue
#endif


#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetColumnDefaultValueA(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_PCSTR                        szTableName,
    _In_ JET_PCSTR                        szColumnName,
    _In_reads_bytes_( cbData ) JET_PCVOID pvData,
    _In_ const JET_UINT32                 cbData,
    _In_ const JET_GRBIT                  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetColumnDefaultValueW(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _In_ JET_PCWSTR                       szTableName,
    _In_ JET_PCWSTR                       szColumnName,
    _In_reads_bytes_( cbData ) JET_PCVOID pvData,
    _In_ const JET_UINT32                 cbData,
    _In_ const JET_GRBIT                  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetColumnDefaultValue JetSetColumnDefaultValueW
#else
#define JetSetColumnDefaultValue JetSetColumnDefaultValueA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetGetTableIndexInfoA JetGetTableIndexInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTableIndexInfoA(
    _In_ JET_SESID                           sesid,
    _In_ JET_TABLEID                         tableid,
    _In_opt_ JET_PCSTR                       szIndexName,
    _Out_writes_bytes_( cbResult ) JET_PVOID pvResult,
    _In_ JET_UINT32                          cbResult,
    _In_ JET_UINT32                          InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTableIndexInfoW(
    _In_ JET_SESID                           sesid,
    _In_ JET_TABLEID                         tableid,
    _In_opt_ JET_PCWSTR                      szIndexName,
    _Out_writes_bytes_( cbResult ) JET_PVOID pvResult,
    _In_ JET_UINT32                          cbResult,
    _In_ JET_UINT32                          InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetTableIndexInfo JetGetTableIndexInfoW
#else
#define JetGetTableIndexInfo JetGetTableIndexInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetGetIndexInfoA JetGetIndexInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetIndexInfoA(
    _In_ JET_SESID                           sesid,
    _In_ JET_DBID                            dbid,
    _In_ JET_PCSTR                           szTableName,
    _In_opt_ JET_PCSTR                       szIndexName,
    _Out_writes_bytes_( cbResult ) JET_PVOID pvResult,
    _In_ JET_UINT32                          cbResult,
    _In_ JET_UINT32                          InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetIndexInfoW(
    _In_ JET_SESID                           sesid,
    _In_ JET_DBID                            dbid,
    _In_ JET_PCWSTR                          szTableName,
    _In_opt_ JET_PCWSTR                      szIndexName,
    _Out_writes_bytes_( cbResult ) JET_PVOID pvResult,
    _In_ JET_UINT32                          cbResult,
    _In_ JET_UINT32                          InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetIndexInfo JetGetIndexInfoW
#else
#define JetGetIndexInfo JetGetIndexInfoA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetCreateIndexA JetCreateIndex
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndexA(
    _In_ JET_SESID                      sesid,
    _In_ JET_TABLEID                    tableid,
    _In_ JET_PCSTR                      szIndexName,
    _In_ JET_GRBIT                      grbit,
    _In_reads_bytes_( cbKey ) JET_PCSTR szKey,
    _In_ JET_UINT32                     cbKey,
    _In_ JET_UINT32                     lDensity );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndexW(
    _In_ JET_SESID                       sesid,
    _In_ JET_TABLEID                     tableid,
    _In_ JET_PCWSTR                      szIndexName,
    _In_ JET_GRBIT                       grbit,
    _In_reads_bytes_( cbKey ) JET_PCWSTR szKey,
    _In_ JET_UINT32                      cbKey,
    _In_ JET_UINT32                      lDensity );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateIndex JetCreateIndexW
#else
#define JetCreateIndex JetCreateIndexA
#endif
#endif


#if ( JET_VERSION < 0x0600 )
#define JetCreateIndex2A JetCreateIndex2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndex2A(
    _In_ JET_SESID                                  sesid,
    _In_ JET_TABLEID                                tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE_A *  pindexcreate,
    _In_ JET_UINT32                                 cIndexCreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndex2W(
    _In_ JET_SESID                                  sesid,
    _In_ JET_TABLEID                                tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE_W *  pindexcreate,
    _In_ JET_UINT32                                 cIndexCreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateIndex2 JetCreateIndex2W
#else
#define JetCreateIndex2 JetCreateIndex2A
#endif
#endif

#if ( JET_VERSION >= 0x0601 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndex3A(
    _In_ JET_SESID                                   sesid,
    _In_ JET_TABLEID                                 tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE2_A *  pindexcreate,
    _In_ JET_UINT32                                  cIndexCreate );

JET_ERR JET_API
JetCreateIndex3W(
    _In_ JET_SESID                                   sesid,
    _In_ JET_TABLEID                                 tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE2_W *  pindexcreate,
    _In_ JET_UINT32                                  cIndexCreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateIndex3 JetCreateIndex3W
#else
#define JetCreateIndex3 JetCreateIndex3A
#endif

#endif

#if ( JET_VERSION >= 0x0602 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndex4A(
    _In_ JET_SESID                                   sesid,
    _In_ JET_TABLEID                                 tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE3_A *  pindexcreate,
    _In_ JET_UINT32                                  cIndexCreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCreateIndex4W(
    _In_ JET_SESID                                   sesid,
    _In_ JET_TABLEID                                 tableid,
    _In_reads_( cIndexCreate ) JET_INDEXCREATE3_W *  pindexcreate,
    _In_ JET_UINT32                                  cIndexCreate );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCreateIndex4 JetCreateIndex4W
#else
#define JetCreateIndex4 JetCreateIndex4A
#endif

#endif

#if ( JET_VERSION < 0x0600 )
#define JetDeleteIndexA JetDeleteIndex
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteIndexA(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCSTR      szIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDeleteIndexW(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_PCWSTR     szIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDeleteIndex JetDeleteIndexW
#else
#define JetDeleteIndex JetDeleteIndexA
#endif
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBeginTransaction(
    _In_ JET_SESID  sesid );

JET_ERR JET_API
JetBeginTransaction2(
    _In_ JET_SESID  sesid,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0602 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBeginTransaction3(
    _In_ JET_SESID      sesid,
    _In_ JET_INT64      trxid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0602


#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCommitTransaction(
    _In_ JET_SESID  sesid,
    _In_ JET_GRBIT  grbit );

#if ( JET_VERSION >= 0x0602 )
JET_ERR JET_API
JetCommitTransaction2(
    _In_ JET_SESID              sesid,
    _In_ JET_GRBIT              grbit,
    _In_ JET_UINT32             cmsecDurableCommit,
    _Out_opt_ JET_COMMIT_ID *   pCommitId );
#endif // JET_VERSION >= 0x0602

JET_ERR JET_API
JetRollback(
    _In_ JET_SESID  sesid,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION < 0x0600 )
#define JetGetDatabaseInfoA JetGetDatabaseInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetGetDatabaseInfoA(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetDatabaseInfoW(
    _In_ JET_SESID                        sesid,
    _In_ JET_DBID                         dbid,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetDatabaseInfo JetGetDatabaseInfoW
#else
#define JetGetDatabaseInfo JetGetDatabaseInfoA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetGetDatabaseFileInfoA JetGetDatabaseFileInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetDatabaseFileInfoA(
    _In_ JET_PCSTR                        szDatabaseName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetDatabaseFileInfoW(
    _In_ JET_PCWSTR                       szDatabaseName,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetDatabaseFileInfo JetGetDatabaseFileInfoW
#else
#define JetGetDatabaseFileInfo JetGetDatabaseFileInfoA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetOpenDatabaseA JetOpenDatabase
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenDatabaseA(
    _In_ JET_SESID      sesid,
    _In_ JET_PCSTR      szFilename,
    _In_opt_ JET_PCSTR  szConnect,
    _Out_ JET_DBID *    pdbid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenDatabaseW(
    _In_ JET_SESID      sesid,
    _In_ JET_PCWSTR     szFilename,
    _In_opt_ JET_PCWSTR szConnect,
    _Out_ JET_DBID *    pdbid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetOpenDatabase JetOpenDatabaseW
#else
#define JetOpenDatabase JetOpenDatabaseA
#endif
#endif // JET_VERSION >= 0x0600

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCloseDatabase(
    _In_ JET_SESID  sesid,
    _In_ JET_DBID   dbid,
    _In_ JET_GRBIT  grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION < 0x0600 )
#define JetOpenTableA JetOpenTable
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenTableA(
    _In_ JET_SESID                                  sesid,
    _In_ JET_DBID                                   dbid,
    _In_ JET_PCSTR                                  szTableName,
    _In_reads_bytes_opt_( cbParameters ) JET_PCVOID pvParameters,
    _In_ JET_UINT32                                 cbParameters,
    _In_ JET_GRBIT                                  grbit,
    _Out_ JET_TABLEID *                             ptableid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenTableW(
    _In_ JET_SESID                                  sesid,
    _In_ JET_DBID                                   dbid,
    _In_ JET_PCWSTR                                 szTableName,
    _In_reads_bytes_opt_( cbParameters ) JET_PCVOID pvParameters,
    _In_ JET_UINT32                                 cbParameters,
    _In_ JET_GRBIT                                  grbit,
    _Out_ JET_TABLEID *                             ptableid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetOpenTable JetOpenTableW
#else
#define JetOpenTable JetOpenTableA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0501 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetTableSequential(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_GRBIT      grbit );

JET_ERR JET_API
JetResetTableSequential(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCloseTable(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid );

JET_ERR JET_API
JetDelete(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetUpdate(
    _In_ JET_SESID                                                sesid,
    _In_ JET_TABLEID                                              tableid,
    _Out_writes_bytes_to_opt_( cbBookmark, *pcbActual ) JET_PVOID pvBookmark,
    _In_ JET_UINT32                                               cbBookmark,
    _Out_opt_ JET_UINT32 *                                        pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0502 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetUpdate2(
    _In_ JET_SESID                                                sesid,
    _In_ JET_TABLEID                                              tableid,
    _Out_writes_bytes_to_opt_( cbBookmark, *pcbActual ) JET_PVOID pvBookmark,
    _In_ JET_UINT32                                               cbBookmark,
    _Out_opt_ JET_UINT32 *                                        pcbActual,
    _In_ const JET_GRBIT                                          grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0502

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEscrowUpdate(
    _In_ JET_SESID                                                 sesid,
    _In_ JET_TABLEID                                               tableid,
    _In_ JET_COLUMNID                                              columnid,
    _In_reads_bytes_( cbMax ) JET_PVOID                            pv,
    _In_ JET_UINT32                                                cbMax,
    _Out_writes_bytes_to_opt_( cbOldMax, *pcbOldActual ) JET_PVOID pvOld,
    _In_ JET_UINT32                                                cbOldMax,
    _Out_opt_ JET_UINT32 *                                         pcbOldActual,
    _In_ JET_GRBIT                                                 grbit );

JET_ERR JET_API
JetRetrieveColumn(
    _In_ JET_SESID                                                           sesid,
    _In_ JET_TABLEID                                                         tableid,
    _In_ JET_COLUMNID                                                        columnid,
    _Out_writes_bytes_to_opt_( cbData, min( cbData, *pcbActual ) ) JET_PVOID pvData,
    _In_ JET_UINT32                                                          cbData,
    _Out_opt_ JET_UINT32 *                                                   pcbActual,
    _In_ JET_GRBIT                                                           grbit,
    _Inout_opt_ JET_RETINFO *                                                pretinfo );

JET_ERR JET_API
JetRetrieveColumns(
    _In_ JET_SESID                                               sesid,
    _In_ JET_TABLEID                                             tableid,
    _Inout_updates_opt_( cretrievecolumn ) JET_RETRIEVECOLUMN *  pretrievecolumn,
    _In_ JET_UINT32                                              cretrievecolumn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEnumerateColumns(
    _In_ JET_SESID                                             sesid,
    _In_ JET_TABLEID                                           tableid,
    _In_ JET_UINT32                                            cEnumColumnId,
    _In_reads_opt_( cEnumColumnId ) JET_ENUMCOLUMNID *         rgEnumColumnId,
    _Out_ JET_UINT32 *                                         pcEnumColumn,
    _Outptr_result_buffer_( *pcEnumColumn ) JET_ENUMCOLUMN **  prgEnumColumn,
    _In_ JET_PFNREALLOC                                        pfnRealloc,
    _In_opt_ JET_PVOID                                         pvReallocContext,
    _In_ JET_UINT32                                            cbDataMost,
    _In_ JET_GRBIT                                             grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501


#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

#if ( JET_VERSION >= 0x0600 )
JET_ERR JET_API
JetGetRecordSize(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _Inout_ JET_RECSIZE *   precsize,
    _In_ const JET_GRBIT    grbit );

#endif // JET_VERSION >= 0x0600

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0601 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetRecordSize2(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _Inout_ JET_RECSIZE2 *  precsize,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0601


#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetColumn(
    _In_ JET_SESID                            sesid,
    _In_ JET_TABLEID                          tableid,
    _In_ JET_COLUMNID                         columnid,
    _In_reads_bytes_opt_( cbData ) JET_PCVOID pvData,
    _In_ JET_UINT32                           cbData,
    _In_ JET_GRBIT                            grbit,
    _In_opt_ JET_SETINFO *                    psetinfo );

JET_ERR JET_API
JetSetColumns(
    _In_ JET_SESID                                  sesid,
    _In_ JET_TABLEID                                tableid,
    _In_reads_opt_( csetcolumn ) JET_SETCOLUMN *    psetcolumn,
    _In_ JET_UINT32                                 csetcolumn );

JET_ERR JET_API
JetPrepareUpdate(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_UINT32     prep );

JET_ERR JET_API
JetGetRecordPosition(
    _In_ JET_SESID                               sesid,
    _In_ JET_TABLEID                             tableid,
    _Out_writes_bytes_( cbRecpos ) JET_RECPOS *  precpos,
    _In_ JET_UINT32                              cbRecpos );

JET_ERR JET_API
JetGotoPosition(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_RECPOS *   precpos );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetCursorInfo(
    _In_ JET_SESID                        sesid,
    _In_ JET_TABLEID                      tableid,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel );

JET_ERR JET_API
JetDupCursor(
    _In_ JET_SESID       sesid,
    _In_ JET_TABLEID     tableid,
    _Out_ JET_TABLEID *  ptableid,
    _In_ JET_GRBIT       grbit );

#if ( JET_VERSION < 0x0600 )
#define JetGetCurrentIndexA JetGetCurrentIndex
#endif

JET_ERR JET_API
JetGetCurrentIndexA(
    _In_ JET_SESID                          sesid,
    _In_ JET_TABLEID                        tableid,
    _Out_writes_bytes_( cbIndexName ) JET_PSTR  szIndexName,
    _In_ JET_UINT32                         cbIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetCurrentIndexW(
    _In_ JET_SESID                          sesid,
    _In_ JET_TABLEID                        tableid,
    _Out_writes_bytes_( cbIndexName ) JET_PWSTR szIndexName,
    _In_ JET_UINT32                         cbIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetCurrentIndex JetGetCurrentIndexW
#else
#define JetGetCurrentIndex JetGetCurrentIndexA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetSetCurrentIndexA JetSetCurrentIndex
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndexA(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCSTR  szIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndexW(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCWSTR szIndexName );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetCurrentIndex JetSetCurrentIndexW
#else
#define JetSetCurrentIndex JetSetCurrentIndexA
#endif
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION < 0x0600 )
#define JetSetCurrentIndex2A JetSetCurrentIndex2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex2A(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCSTR  szIndexName,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex2W(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCWSTR szIndexName,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetCurrentIndex2 JetSetCurrentIndex2W
#else
#define JetSetCurrentIndex2 JetSetCurrentIndex2A
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetSetCurrentIndex3A JetSetCurrentIndex3
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex3A(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCSTR  szIndexName,
    _In_ JET_GRBIT      grbit,
    _In_ JET_UINT32     itagSequence );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex3W(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_opt_ JET_PCWSTR szIndexName,
    _In_ JET_GRBIT      grbit,
    _In_ JET_UINT32     itagSequence );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetCurrentIndex3 JetSetCurrentIndex3W
#else
#define JetSetCurrentIndex3 JetSetCurrentIndex3A
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetSetCurrentIndex4A JetSetCurrentIndex4
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex4A(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _In_opt_ JET_PCSTR      szIndexName,
    _In_opt_ JET_INDEXID *  pindexid,
    _In_ JET_GRBIT          grbit,
    _In_ JET_UINT32         itagSequence );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetCurrentIndex4W(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _In_opt_ JET_PCWSTR     szIndexName,
    _In_opt_ JET_INDEXID *  pindexid,
    _In_ JET_GRBIT          grbit,
    _In_ JET_UINT32         itagSequence );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetCurrentIndex4 JetSetCurrentIndex4W
#else
#define JetSetCurrentIndex4 JetSetCurrentIndex4A
#endif
#endif // JET_VERSION >= 0x0600


#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetMove(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_INT32      cRow,
    _In_ JET_GRBIT      grbit );

#if ( JET_VERSION >= 0x0602 )
JET_ERR JET_API
JetSetCursorFilter(
    _In_ JET_SESID                                   sesid,
    _In_ JET_TABLEID                                 tableid,
    _In_reads_( cColumnFilters ) JET_INDEX_COLUMN *  rgColumnFilters,
    _In_ JET_UINT32                                  cColumnFilters,
    _In_ JET_GRBIT                                   grbit );
#endif  //  JET_VERSION >= 0x0602

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLock(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetMakeKey(
    _In_ JET_SESID                            sesid,
    _In_ JET_TABLEID                          tableid,
    _In_reads_bytes_opt_( cbData ) JET_PCVOID pvData,
    _In_ JET_UINT32                           cbData,
    _In_ JET_GRBIT                            grbit );

JET_ERR JET_API
JetSeek(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0601 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

// FOOBY
JET_ERR JET_API
JetPrereadKeys(
    _In_ JET_SESID                                      sesid,
    _In_ JET_TABLEID                                    tableid,
    _In_reads_(ckeys) JET_PCVOID *                      rgpvKeys,
    _In_reads_(ckeys) const JET_UINT32 *                rgcbKeys,
    _In_ JET_INT32                                      ckeys,
    _Out_opt_ JET_INT32 *                               pckeysPreread,
    _In_ JET_GRBIT                                      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0601

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

#if ( JET_VERSION >= 0x0602 )

JET_ERR JET_API
JetPrereadIndexRanges(
    _In_ JET_SESID                                              sesid,
    _In_ JET_TABLEID                                            tableid,
    _In_reads_(cIndexRanges) const JET_INDEX_RANGE * const      rgIndexRanges,
    _In_ const JET_UINT32                                       cIndexRanges,
    _Out_opt_ JET_UINT32 * const                                pcRangesPreread,
    _In_reads_(ccolumnidPreread) const JET_COLUMNID * const     rgcolumnidPreread,
    _In_ const JET_UINT32                                       ccolumnidPreread,
    _In_ JET_GRBIT                                              grbit ); // JET_bitPrereadForward, JET_bitPrereadBackward

#endif // JET_VERSION >= 0x0602

JET_ERR JET_API
JetGetBookmark(
    _In_ JET_SESID                                           sesid,
    _In_ JET_TABLEID                                         tableid,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pvBookmark,
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetSecondaryIndexBookmark(
    _In_ JET_SESID                                                                         sesid,
    _In_ JET_TABLEID                                                                       tableid,
    _Out_writes_bytes_to_opt_( cbSecondaryKeyMax, *pcbSecondaryKeyActual ) JET_PVOID       pvSecondaryKey,
    _In_ JET_UINT32                                                                        cbSecondaryKeyMax,
    _Out_opt_ JET_UINT32 *                                                                 pcbSecondaryKeyActual,
    _Out_writes_bytes_to_opt_( cbPrimaryBookmarkMax, *pcbPrimaryBookmarkActual ) JET_PVOID pvPrimaryBookmark,
    _In_ JET_UINT32                                                                        cbPrimaryBookmarkMax,
    _Out_opt_ JET_UINT32 *                                                                 pcbPrimaryBookmarkActual,
    _In_ const JET_GRBIT                                                                   grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetCompactA JetCompact
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCompactA(
    _In_ JET_SESID              sesid,
    _In_ JET_PCSTR              szDatabaseSrc,
    _In_ JET_PCSTR              szDatabaseDest,
    _In_ JET_PFNSTATUS          pfnStatus,
    _In_opt_ JET_CONVERT_A *    pconvert,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCompactW(
    _In_ JET_SESID              sesid,
    _In_ JET_PCWSTR             szDatabaseSrc,
    _In_ JET_PCWSTR             szDatabaseDest,
    _In_ JET_PFNSTATUS          pfnStatus,
    _In_opt_ JET_CONVERT_W *    pconvert,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetCompact JetCompactW
#else
#define JetCompact JetCompactA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetDefragmentA JetDefragment
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragmentA(
    _In_ JET_SESID              sesid,
    _In_ JET_DBID               dbid,
    _In_opt_ JET_PCSTR          szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragmentW(
    _In_ JET_SESID              sesid,
    _In_ JET_DBID               dbid,
    _In_opt_ JET_PCWSTR         szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDefragment JetDefragmentW
#else
#define JetDefragment JetDefragmentA
#endif
#endif


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetDefragment2A JetDefragment2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragment2A(
    _In_ JET_SESID              sesid,
    _In_ JET_DBID               dbid,
    _In_opt_ JET_PCSTR          szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_CALLBACK           callback,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragment2W(
    _In_ JET_SESID              sesid,
    _In_ JET_DBID               dbid,
    _In_opt_ JET_PCWSTR         szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_CALLBACK           callback,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDefragment2 JetDefragment2W
#else
#define JetDefragment2 JetDefragment2A
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetDefragment3A JetDefragment3
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragment3A(
    _In_ JET_SESID              sesid,
    _In_ JET_PCSTR              szDatabaseName,
    _In_opt_ JET_PCSTR          szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_CALLBACK           callback,
    _In_ JET_PVOID              pvContext,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetDefragment3W(
    _In_ JET_SESID              sesid,
    _In_ JET_PCWSTR             szDatabaseName,
    _In_opt_ JET_PCWSTR         szTableName,
    _Inout_opt_ JET_UINT32 *    pcPasses,
    _Inout_opt_ JET_UINT32 *    pcSeconds,
    _In_ JET_CALLBACK           callback,
    _In_ JET_PVOID              pvContext,
    _In_ JET_GRBIT              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetDefragment3 JetDefragment3W
#else
#define JetDefragment3 JetDefragment3A
#endif
#endif // JET_VERSION >= 0x0600

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION < 0x0600 )
#define JetSetDatabaseSizeA JetSetDatabaseSize
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetDatabaseSizeA(
    _In_ JET_SESID          sesid,
    _In_ JET_PCSTR          szDatabaseName,
    _In_ JET_UINT32         cpg,
    _Out_ JET_UINT32 *      pcpgReal );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetDatabaseSizeW(
    _In_ JET_SESID          sesid,
    _In_ JET_PCWSTR         szDatabaseName,
    _In_ JET_UINT32         cpg,
    _Out_ JET_UINT32 *      pcpgReal );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetSetDatabaseSize JetSetDatabaseSizeW
#else
#define JetSetDatabaseSize JetSetDatabaseSizeA
#endif
#endif // JET_VERSION >= 0x0600

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGrowDatabase(
    _In_ JET_SESID          sesid,
    _In_ JET_DBID           dbid,
    _In_ JET_UINT32         cpg,
    _In_ JET_UINT32 *       pcpgReal );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

#if ( JET_VERSION >= 0x0602 )
JET_ERR JET_API
JetResizeDatabase(
    _In_  JET_SESID         sesid,
    _In_  JET_DBID          dbid,
    _In_  JET_UINT32        cpgTarget,
    _Out_ JET_UINT32 *      pcpgActual,
    _In_  const JET_GRBIT   grbit );
#endif // JET_VERSION >= 0x0602

JET_ERR JET_API
JetSetSessionContext(
    _In_ JET_SESID      sesid,
    _In_ JET_API_PTR    ulContext );

JET_ERR JET_API
JetResetSessionContext(
    _In_ JET_SESID      sesid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion


#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGotoBookmark(
    _In_ JET_SESID                           sesid,
    _In_ JET_TABLEID                         tableid,
    _In_reads_bytes_( cbBookmark ) JET_PVOID pvBookmark,
    _In_ JET_UINT32                          cbBookmark );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGotoSecondaryIndexBookmark(
    _In_ JET_SESID                                      sesid,
    _In_ JET_TABLEID                                    tableid,
    _In_reads_bytes_( cbSecondaryKey ) JET_PVOID        pvSecondaryKey,
    _In_ JET_UINT32                                     cbSecondaryKey,
    _In_reads_bytes_opt_( cbPrimaryBookmark ) JET_PVOID pvPrimaryBookmark,
    _In_ JET_UINT32                                     cbPrimaryBookmark,
    _In_ const JET_GRBIT                                grbit );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetIntersectIndexes(
    _In_ JET_SESID                              sesid,
    _In_reads_( cindexrange ) JET_INDEXRANGE *  rgindexrange,
    _In_ JET_UINT32                             cindexrange,
    _Inout_ JET_RECORDLIST *                    precordlist,
    _In_ JET_GRBIT                              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetComputeStats(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid );

JET_ERR JET_API
JetOpenTempTable(
    _In_ JET_SESID                                  sesid,
    _In_reads_( ccolumn ) const JET_COLUMNDEF *     prgcolumndef,
    _In_ JET_UINT32                                 ccolumn,
    _In_ JET_GRBIT                                  grbit,
    _Out_ JET_TABLEID *                             ptableid,
    _Out_writes_( ccolumn ) JET_COLUMNID *          prgcolumnid );

JET_ERR JET_API
JetOpenTempTable2(
    _In_ JET_SESID                                  sesid,
    _In_reads_( ccolumn ) const JET_COLUMNDEF *     prgcolumndef,
    _In_ JET_UINT32                                 ccolumn,
    _In_ JET_LCID                                   lcid,
    _In_ JET_GRBIT                                  grbit,
    _Out_ JET_TABLEID *                             ptableid,
    _Out_writes_( ccolumn ) JET_COLUMNID *          prgcolumnid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenTempTable3(
    _In_ JET_SESID                                  sesid,
    _In_reads_( ccolumn ) const JET_COLUMNDEF *     prgcolumndef,
    _In_ JET_UINT32                                 ccolumn,
    _In_opt_ JET_UNICODEINDEX *                     pidxunicode,
    _In_ JET_GRBIT                                  grbit,
    _Out_ JET_TABLEID *                             ptableid,
    _Out_writes_( ccolumn ) JET_COLUMNID *          prgcolumnid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenTemporaryTable(
    _In_ JET_SESID                  sesid,
    _In_ JET_OPENTEMPORARYTABLE *   popentemporarytable );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0602 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenTemporaryTable2(
    _In_ JET_SESID                  sesid,
    _In_ JET_OPENTEMPORARYTABLE2 *  popentemporarytable );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0602


#if ( JET_VERSION < 0x0600 )
#define JetBackupA JetBackup
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBackupA(
    _In_ JET_PCSTR      szBackupPath,
    _In_ JET_GRBIT      grbit,
    _In_opt_ JET_PFNSTATUS  pfnStatus );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBackupW(
    _In_ JET_PCWSTR     szBackupPath,
    _In_ JET_GRBIT      grbit,
    _In_opt_ JET_PFNSTATUS  pfnStatus );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetBackup JetBackupW
#else
#define JetBackup JetBackupA
#endif
#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetBackupInstanceA JetBackupInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBackupInstanceA(
    _In_ JET_INSTANCE   instance,
    _In_ JET_PCSTR      szBackupPath,
    _In_ JET_GRBIT      grbit,
    _In_opt_ JET_PFNSTATUS  pfnStatus );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetBackupInstanceW(
    _In_ JET_INSTANCE   instance,
    _In_ JET_PCWSTR     szBackupPath,
    _In_ JET_GRBIT      grbit,
    _In_opt_ JET_PFNSTATUS  pfnStatus );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetBackupInstance JetBackupInstanceW
#else
#define JetBackupInstance JetBackupInstanceA
#endif
#endif // JET_VERSION >= 0x0600

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetRestoreA JetRestore
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestoreA(
    _In_ JET_PCSTR      szSource,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestoreW(
    _In_ JET_PCWSTR     szSource,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetRestore JetRestoreW
#else
#define JetRestore JetRestoreA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetRestore2A JetRestore2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestore2A(
    _In_ JET_PCSTR      sz,
    _In_opt_ JET_PCSTR  szDest,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestore2W(
    _In_ JET_PCWSTR     sz,
    _In_opt_ JET_PCWSTR szDest,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetRestore2 JetRestore2W
#else
#define JetRestore2 JetRestore2A
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetRestoreInstanceA JetRestoreInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestoreInstanceA(
    _In_ JET_INSTANCE   instance,
    _In_ JET_PCSTR      sz,
    _In_opt_ JET_PCSTR  szDest,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRestoreInstanceW(
    _In_ JET_INSTANCE   instance,
    _In_ JET_PCWSTR     sz,
    _In_opt_ JET_PCWSTR szDest,
    _In_opt_ JET_PFNSTATUS  pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetRestoreInstance JetRestoreInstanceW
#else
#define JetRestoreInstance JetRestoreInstanceA
#endif
#endif // JET_VERSION >= 0x0600

#endif // JET_VERSION >= 0x0501

#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetSetIndexRange(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableidSrc,
    _In_ JET_GRBIT      grbit );

JET_ERR JET_API
JetIndexRecordCount(
    _In_ JET_SESID          sesid,
    _In_ JET_TABLEID        tableid,
    _Out_ JET_UINT32 *      pcrec,
    _In_ JET_UINT32         crecMax );

#if ( JET_VERSION >= 0x0A01 )

JET_ERR JET_API
JetIndexRecordCount2(
    _In_ JET_SESID              sesid,
    _In_ JET_TABLEID            tableid,
    _Out_ JET_UINT64 *          pcrec,
    _In_ JET_UINT64             crecMax );

#endif // JET_VERSION >= 0x0A01

JET_ERR JET_API
JetRetrieveKey(
    _In_ JET_SESID                                           sesid,
    _In_ JET_TABLEID                                         tableid,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pvKey,
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual,
    _In_ JET_GRBIT                                           grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetBeginExternalBackup(
    _In_ JET_GRBIT grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetBeginExternalBackupInstance(
    _In_ JET_INSTANCE instance,
    _In_ JET_GRBIT grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION < 0x0600 )
#define JetGetAttachInfoA JetGetAttachInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetAttachInfoA(
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR szzDatabases,
#endif
    _In_ JET_UINT32                                     cbMax,
    _Out_opt_ JET_UINT32 *                              pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetAttachInfoW(
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR    wszzDatabases,
    _In_ JET_UINT32                                         cbMax,
    _Out_opt_ JET_UINT32 *                                  pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetAttachInfo JetGetAttachInfoW
#else
#define JetGetAttachInfo JetGetAttachInfoA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetGetAttachInfoInstanceA JetGetAttachInfoInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetAttachInfoInstanceA(
    _In_ JET_INSTANCE                                        instance,
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR  szzDatabases,
#endif
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetAttachInfoInstanceW(
    _In_ JET_INSTANCE                                       instance,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR    szzDatabases,
    _In_ JET_UINT32                                         cbMax,
    _Out_opt_ JET_UINT32 *                                  pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetAttachInfoInstance JetGetAttachInfoInstanceW
#else
#define JetGetAttachInfoInstance JetGetAttachInfoInstanceA
#endif
#endif // JET_VERSION >= 0x0600

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetOpenFileA JetOpenFile
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenFileA(
    _In_ JET_PCSTR          szFileName,
    _Out_ JET_HANDLE *      phfFile,
    _Out_ JET_UINT32 *      pulFileSizeLow,
    _Out_ JET_UINT32 *      pulFileSizeHigh );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenFileW(
    _In_ JET_PCWSTR         szFileName,
    _Out_ JET_HANDLE *      phfFile,
    _Out_ JET_UINT32 *      pulFileSizeLow,
    _Out_ JET_UINT32 *      pulFileSizeHigh );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetOpenFile JetOpenFileW
#else
#define JetOpenFile JetOpenFileA
#endif
#endif


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetOpenFileInstanceA JetOpenFileInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenFileInstanceA(
    _In_ JET_INSTANCE       instance,
    _In_ JET_PCSTR          szFileName,
    _Out_ JET_HANDLE *      phfFile,
    _Out_ JET_UINT32 *      pulFileSizeLow,
    _Out_ JET_UINT32 *      pulFileSizeHigh );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOpenFileInstanceW(
    _In_ JET_INSTANCE       instance,
    _In_ JET_PCWSTR         szFileName,
    _Out_ JET_HANDLE *      phfFile,
    _Out_ JET_UINT32 *      pulFileSizeLow,
    _Out_ JET_UINT32 *      pulFileSizeHigh );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetOpenFileInstance JetOpenFileInstanceW
#else
#define JetOpenFileInstance JetOpenFileInstanceA
#endif
#endif // JET_VERSION >= 0x0600

#endif // JET_VERSION >= 0x0501


#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetReadFile(
    _In_ JET_HANDLE                                   hfFile,
    _Out_writes_bytes_to_( cb, *pcbActual ) JET_PVOID pv,
    _In_ JET_UINT32                                   cb,
    _Out_opt_ JET_UINT32 *                            pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetReadFileInstance(
    _In_ JET_INSTANCE                                 instance,
    _In_ JET_HANDLE                                   hfFile,
    _Out_writes_bytes_to_( cb, *pcbActual ) JET_PVOID pv,
    _In_ JET_UINT32                                   cb,
    _Out_opt_ JET_UINT32 *                            pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCloseFile(
    _In_ JET_HANDLE     hfFile );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetCloseFileInstance(
    _In_ JET_INSTANCE   instance,
    _In_ JET_HANDLE     hfFile );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION < 0x0600 )
#define JetGetLogInfoA JetGetLogInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoA(
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR  szzLogs,
#endif
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoW(
        _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR    szzLogs,
        _In_ JET_UINT32                                         cbMax,
        _Out_opt_ JET_UINT32 *                                  pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetLogInfo JetGetLogInfoW
#else
#define JetGetLogInfo JetGetLogInfoA
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION >= 0x0501 )
#if ( JET_VERSION < 0x0600 )
#define JetGetLogInfoInstanceA JetGetLogInfoInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoInstanceA(
    _In_ JET_INSTANCE                                        instance,
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR  szzLogs,
#endif
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoInstanceW(
    _In_ JET_INSTANCE                                         instance,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR  wszzLogs,
    _In_ JET_UINT32                                           cbMax,
    _Out_opt_ JET_UINT32 *                                    pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetLogInfoInstance JetGetLogInfoInstanceW
#else
#define JetGetLogInfoInstance JetGetLogInfoInstanceA
#endif
#endif // JET_VERSION >= 0x0600


#define JET_BASE_NAME_LENGTH    3
typedef struct
{
    JET_UINT32      cbSize;
    JET_UINT32      ulGenLow;
    JET_UINT32      ulGenHigh;
    JET_CHAR        szBaseName[ JET_BASE_NAME_LENGTH + 1 ];
} JET_LOGINFO_A;

typedef struct
{
    JET_UINT32      cbSize;
    JET_UINT32      ulGenLow;
    JET_UINT32      ulGenHigh;
    JET_WCHAR       szBaseName[ JET_BASE_NAME_LENGTH + 1 ];
} JET_LOGINFO_W;

#ifdef JET_UNICODE
#define JET_LOGINFO JET_LOGINFO_W
#else
#define JET_LOGINFO JET_LOGINFO_A
#endif

#if ( JET_VERSION < 0x0600 )
#define JetGetLogInfoInstance2A JetGetLogInfoInstance2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoInstance2A(
    _In_ JET_INSTANCE                                        instance,
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR  szzLogs,
#endif
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual,
    _Inout_opt_ JET_LOGINFO_A *                              pLogInfo );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetLogInfoInstance2W(
    _In_ JET_INSTANCE                                         instance,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR  wszzLogs,
    _In_ JET_UINT32                                           cbMax,
    _Out_opt_ JET_UINT32 *                                    pcbActual,
    _Inout_opt_ JET_LOGINFO_W *                               pLogInfo );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetLogInfoInstance2 JetGetLogInfoInstance2W
#else
#define JetGetLogInfoInstance2 JetGetLogInfoInstance2A
#endif
#endif // JET_VERSION >= 0x0600


#if ( JET_VERSION < 0x0600 )
#define JetGetTruncateLogInfoInstanceA JetGetTruncateLogInfoInstance
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTruncateLogInfoInstanceA(
    _In_ JET_INSTANCE                                        instance,
#if ( JET_VERSION < 0x0600 )
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PVOID pv,
#else
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PSTR  szzLogs,
#endif
    _In_ JET_UINT32                                          cbMax,
    _Out_opt_ JET_UINT32 *                                   pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetTruncateLogInfoInstanceW(
    _In_ JET_INSTANCE                                         instance,
    _Out_writes_bytes_to_opt_( cbMax, *pcbActual ) JET_PWSTR  wszzLogs,
    _In_ JET_UINT32                                           cbMax,
    _Out_opt_ JET_UINT32 *                                    pcbActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetTruncateLogInfoInstance JetGetTruncateLogInfoInstanceW
#else
#define JetGetTruncateLogInfoInstance JetGetTruncateLogInfoInstanceA
#endif
#endif // JET_VERSION >= 0x0600


#endif // JET_VERSION >= 0x0501

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetTruncateLog( JET_VOID );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetTruncateLogInstance(
    _In_ JET_INSTANCE   instance );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetEndExternalBackup( JET_VOID );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0501 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetEndExternalBackupInstance(
    _In_ JET_INSTANCE   instance );

JET_ERR JET_API
JetEndExternalBackupInstance2(
    _In_ JET_INSTANCE   instance,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501


#if ( JET_VERSION < 0x0600 )
#define JetExternalRestoreA JetExternalRestore
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetExternalRestoreA(
    _In_ JET_PSTR                                   szCheckpointFilePath,
    _In_ JET_PSTR                                   szLogPath,
    _In_reads_opt_( crstfilemap ) JET_RSTMAP_A *    rgrstmap,
    _In_ JET_INT32                                  crstfilemap,
    _In_ JET_PSTR                                   szBackupLogPath,
    _In_ JET_INT32                                  genLow,
    _In_ JET_INT32                                  genHigh,
    _In_ JET_PFNSTATUS                              pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetExternalRestoreW(
    _In_ JET_PWSTR                                  szCheckpointFilePath,
    _In_ JET_PWSTR                                  szLogPath,
    _In_reads_opt_( crstfilemap ) JET_RSTMAP_W *    rgrstmap,
    _In_ JET_INT32                                  crstfilemap,
    _In_ JET_PWSTR                                  szBackupLogPath,
    _In_ JET_INT32                                  genLow,
    _In_ JET_INT32                                  genHigh,
    _In_ JET_PFNSTATUS                              pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetExternalRestore JetExternalRestoreW
#else
#define JetExternalRestore JetExternalRestoreA
#endif
#endif // JET_VERSION >= 0x0600


#if JET_VERSION >= 0x0501
#if ( JET_VERSION < 0x0600 )
#define JetExternalRestore2A JetExternalRestore2
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetExternalRestore2A(
    _In_ JET_PSTR                                   szCheckpointFilePath,
    _In_ JET_PSTR                                   szLogPath,
    _In_reads_opt_( crstfilemap ) JET_RSTMAP_A *    rgrstmap,
    _In_ JET_INT32                                  crstfilemap,
    _In_ JET_PSTR                                   szBackupLogPath,
    _Inout_ JET_LOGINFO_A *                         pLogInfo,
    _In_opt_ JET_PSTR                               szTargetInstanceName,
    _In_opt_ JET_PSTR                               szTargetInstanceLogPath,
    _In_opt_ JET_PSTR                               szTargetInstanceCheckpointPath,
    _In_ JET_PFNSTATUS                              pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetExternalRestore2W(
    _In_ JET_PWSTR                                  szCheckpointFilePath,
    _In_ JET_PWSTR                                  szLogPath,
    _In_reads_opt_( crstfilemap ) JET_RSTMAP_W *    rgrstmap,
    _In_ JET_INT32                                  crstfilemap,
    _In_ JET_PWSTR                                  szBackupLogPath,
    _Inout_ JET_LOGINFO_W *                         pLogInfo,
    _In_opt_ JET_PWSTR                              szTargetInstanceName,
    _In_opt_ JET_PWSTR                              szTargetInstanceLogPath,
    _In_opt_ JET_PWSTR                              szTargetInstanceCheckpointPath,
    _In_ JET_PFNSTATUS                              pfn );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetExternalRestore2 JetExternalRestore2W
#else
#define JetExternalRestore2 JetExternalRestore2A
#endif
#endif // JET_VERSION >= 0x0600


#pragma region Application Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetRegisterCallback(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_CBTYP      cbtyp,
    _In_ JET_CALLBACK   pCallback,
    _In_opt_ JET_PVOID  pvContext,
    _In_ JET_HANDLE *   phCallbackId );

JET_ERR JET_API
JetUnregisterCallback(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_CBTYP      cbtyp,
    _In_ JET_HANDLE     hCallbackId );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

typedef struct _JET_INSTANCE_INFO_A
{
    JET_INSTANCE        hInstanceId;
    JET_PSTR            szInstanceName;

    JET_API_PTR         cDatabases;
    JET_PSTR *          szDatabaseFileName;
    JET_PSTR *          szDatabaseDisplayName;
    JET_PSTR *          szDatabaseSLVFileName_Obsolete;
} JET_INSTANCE_INFO_A;

typedef struct _JET_INSTANCE_INFO_W
{
    JET_INSTANCE        hInstanceId;
    JET_PWSTR           szInstanceName;

    JET_API_PTR         cDatabases;
    JET_PWSTR *         szDatabaseFileName;
    JET_PWSTR *         szDatabaseDisplayName;
    JET_PWSTR *         szDatabaseSLVFileName_Obsolete;
} JET_INSTANCE_INFO_W;

#ifdef JET_UNICODE
#define JET_INSTANCE_INFO JET_INSTANCE_INFO_W
#else
#define JET_INSTANCE_INFO JET_INSTANCE_INFO_A
#endif

#if ( JET_VERSION < 0x0600 )
#define JetGetInstanceInfoA JetGetInstanceInfo
#endif

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetInstanceInfoA(
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_A **  paInstanceInfo );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetGetInstanceInfoW(
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_W **  paInstanceInfo );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#ifdef JET_UNICODE
#define JetGetInstanceInfo JetGetInstanceInfoW
#else
#define JetGetInstanceInfo JetGetInstanceInfoA
#endif
#endif // JET_VERSION >= 0x0600

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetFreeBuffer(
    _Pre_notnull_ JET_CHAR *  pbBuf );

JET_ERR JET_API
JetSetLS(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _In_ JET_LS         ls,
    _In_ JET_GRBIT      grbit );

JET_ERR JET_API
JetGetLS(
    _In_ JET_SESID      sesid,
    _In_ JET_TABLEID    tableid,
    _Out_ JET_LS *      pls,
    _In_ JET_GRBIT      grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion


#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

typedef JET_API_PTR JET_OSSNAPID;   // Snapshot Session Identifier

JET_ERR JET_API
JetOSSnapshotPrepare(
    _Out_ JET_OSSNAPID *    psnapId,
    _In_ const JET_GRBIT    grbit );
#if ( JET_VERSION >= 0x0600 )
JET_ERR JET_API
JetOSSnapshotPrepareInstance(
    _In_ JET_OSSNAPID       snapId,
    _In_ JET_INSTANCE       instance,
    _In_ const JET_GRBIT    grbit );

#endif // JET_VERSION >= 0x0600

#if ( JET_VERSION < 0x0600 )
#define JetOSSnapshotFreezeA JetOSSnapshotFreeze
#endif

JET_ERR JET_API
JetOSSnapshotFreezeA(
    _In_ const JET_OSSNAPID                                           snapId,
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_A **  paInstanceInfo,
    _In_ const JET_GRBIT                                              grbit );

#if ( JET_VERSION >= 0x0600 )

JET_ERR JET_API
JetOSSnapshotFreezeW(
    _In_ const JET_OSSNAPID                                           snapId,
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_W **  paInstanceInfo,
    _In_ const JET_GRBIT                                              grbit );

#ifdef JET_UNICODE
#define JetOSSnapshotFreeze JetOSSnapshotFreezeW
#else
#define JetOSSnapshotFreeze JetOSSnapshotFreezeA
#endif
#endif // JET_VERSION >= 0x0600

JET_ERR JET_API
JetOSSnapshotThaw(
    _In_ const JET_OSSNAPID snapId,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0501

#if ( JET_VERSION >= 0x0502 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOSSnapshotAbort(
    _In_ const JET_OSSNAPID snapId,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0502

#if ( JET_VERSION >= 0x0600 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOSSnapshotTruncateLog(
    _In_ const JET_OSSNAPID snapId,
    _In_ const JET_GRBIT    grbit );

JET_ERR JET_API
JetOSSnapshotTruncateLogInstance(
    _In_ const JET_OSSNAPID snapId,
    _In_ JET_INSTANCE       instance,
    _In_ const JET_GRBIT    grbit );

#if ( JET_VERSION < 0x0600 )
#define JetOSSnapshotGetFreezeInfoA JetOSSnapshotGetFreezeInfo
#endif

JET_ERR JET_API
JetOSSnapshotGetFreezeInfoA(
    _In_ const JET_OSSNAPID                                           snapId,
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_A **  paInstanceInfo,
    _In_ const JET_GRBIT                                              grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetOSSnapshotGetFreezeInfoW(
    _In_ const JET_OSSNAPID                                           snapId,
    _Out_ JET_UINT32 *                                                pcInstanceInfo,
    _Outptr_result_buffer_( *pcInstanceInfo ) JET_INSTANCE_INFO_W **  paInstanceInfo,
    _In_ const JET_GRBIT                                              grbit );

#ifdef JET_UNICODE
#define JetOSSnapshotGetFreezeInfo JetOSSnapshotGetFreezeInfoW
#else
#define JetOSSnapshotGetFreezeInfo JetOSSnapshotGetFreezeInfoA
#endif

JET_ERR JET_API
JetOSSnapshotEnd(
    _In_ const JET_OSSNAPID snapId,
    _In_ const JET_GRBIT    grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0600



#if ( JET_VERSION >= 0x0601 )

//  Options for JetConfigureProcessForCrashDump

#define JET_bitDumpMinimum                      0x00000001
//  dump minimum includes cache minimum
#define JET_bitDumpMaximum                      0x00000002
//  dump maximum includes dump minimum
//  dump maximum includes cache maximum
#define JET_bitDumpCacheMinimum                 0x00000004
//  cache minimum includes pages that are latched
//  cache minimum includes pages that are used for memory
//  cache minimum includes pages that are flagged with errors
#define JET_bitDumpCacheMaximum                 0x00000008
//  cache maximum includes cache minimum
//  cache maximum includes the entire cache image
#define JET_bitDumpCacheIncludeDirtyPages       0x00000010
//  dump includes pages that are modified
#define JET_bitDumpCacheIncludeCachedPages      0x00000020
//  dump includes pages that contain valid data
#define JET_bitDumpCacheIncludeCorruptedPages   0x00000040
//  dump includes pages that are corrupted (expensive to compute)
#define JET_bitDumpCacheNoDecommit              0x00000080
//  do not decommit any pages not intending to include in crash dump



#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API
JetConfigureProcessForCrashDump(
    _In_ const JET_GRBIT grbit );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion

#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0601 )


#endif // JET_VERSION >= 0x0601

#if ( JET_VERSION >= 0x0602 )

#pragma region Desktop Family or Esent Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT)

JET_ERR JET_API JetGetErrorInfoW(
    _In_opt_ JET_PVOID                    pvContext,
    _Out_writes_bytes_( cbMax ) JET_PVOID pvResult,
    _In_ JET_UINT32                       cbMax,
    _In_ JET_UINT32                       InfoLevel,
    _In_ JET_GRBIT                        grbit );

#ifdef JET_UNICODE
#define JetGetErrorInfo JetGetErrorInfoW
#else
#define JetGetErrorInfo JetGetErrorInfoA_DoesNotExist_OnlyUnicodeVersionOfThisAPI_UseExcplicit_JetGetErrorInfoW_Instead
#endif

JET_ERR JET_API
JetSetSessionParameter(
    _In_opt_ JET_SESID                                          sesid,
    _In_ JET_UINT32                                             sesparamid,
    _In_reads_bytes_opt_( cbParam ) JET_PVOID                   pvParam,
    _In_ JET_UINT32                                             cbParam );

JET_ERR JET_API
JetGetSessionParameter(
    _In_opt_ JET_SESID                                          sesid,
    _In_ JET_UINT32                                             sesparamid,
    _Out_cap_post_count_(cbParamMax, *pcbParamActual) JET_PVOID pvParam,
    _In_ JET_UINT32                                             cbParamMax,
    _Out_opt_ JET_UINT32 *                                      pcbParamActual );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PKG_ESENT) */
#pragma endregion


#endif // JET_VERSION >= 0x0602


#ifdef  __cplusplus
} // extern "C" - Note: from the beginning of the #if !defined(_JET_NOPROTOTYPES) section.
#endif

#endif  /* _JET_NOPROTOTYPES */

#pragma pack(pop)

#ifdef  __cplusplus
} // extern "C"
#endif

#endif  /* _JET_INCLUDED */

